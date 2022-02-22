#![allow(dead_code, unused_variables, unused_imports)]

use radix_engine::ledger::*;
use radix_engine::model::*;
use radix_engine::transaction::*;
use radix_engine::engine::*;
use scrypto::prelude::*;
use sbor::any::*;

fn get_vault_info<'a, L: SubstateStore>(ledger: &'a L, component: &Address, id: &Vid) -> (Address, Decimal) {
    let vault = ledger.get_vault(component, id).unwrap();
    let amount = vault.amount();
    let resource_def_address = vault.resource_address();
    (resource_def_address, amount)
}

fn get_lazymap_info<'a, L: SubstateStore>(ledger: &'a L, component: &Address, id: &Mid) -> Vec<(Address, Decimal)> {
    let lazy_map = ledger.get_lazy_map(component, id).unwrap();
    lazy_map.map().iter().flat_map(|(_,data)| {
      let validated_data = validate_data(data).unwrap();
      validated_data.vaults.iter().map(|vid| get_vault_info(ledger, component, vid)).collect::<Vec<(Address, Decimal)>>()
    }).collect()
}

fn get_account_vaults<'a, L: SubstateStore>(ledger: &'a L, account: Address) -> HashMap<Address, Decimal> {
    let component = ledger.get_component(account).unwrap();
    let state = component.state();
    let validated_data = validate_data(state).unwrap();
    validated_data.lazy_maps.iter()
        .flat_map(|mid| get_lazymap_info(ledger, &account, &mid))
        .collect()
}

fn create_token<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    account: Address,
    nb: Decimal,
    key: EcdsaPublicKey,
) -> Address {
    let receipt = executor
        .run(
            TransactionBuilder::new(executor)
                .new_token_fixed(HashMap::new(), nb)
                .call_method_with_all_resources(account, "deposit_batch")
                .build(vec![key])
                .unwrap()
        )
        .unwrap();
    return receipt.resource_def(0).unwrap();
}

fn transfer_token<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    account_from: Address,
    account_to: Address,
    nb_token: Decimal,
    token: Address,
    key: EcdsaPublicKey,
) {
  let receipt = executor
      .run(
          TransactionBuilder::new(executor)
              .withdraw_from_account( 
                  &Resource::Fungible {amount: nb_token, resource_address: token},
                  account_from
              )
              .call_method_with_all_resources(account_to, "deposit_batch")
              .build(vec![key])
              .unwrap()
      )
      .unwrap();
  assert!(receipt.result.is_ok());
}

fn create_exchange<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    package: Address,
    account: Address,
    token: Address,
    key: EcdsaPublicKey,
) -> Address {
    let receipt = executor
        .run(
            TransactionBuilder::new(executor)
                .call_function(
                    package,
                    "CroqOrderBook",
                    "instantiate",
                    vec![
                      format!("{}", token),
                      format!("{}", RADIX_TOKEN)
                    ],
                    Some(account),
                )
                .call_method_with_all_resources(account, "deposit_batch")
                .build(vec![key])
                .unwrap()
        )
        .unwrap();
    println!("{:?}\n", receipt);
    receipt.component(0).unwrap()
}

fn create_actor<'a, L: SubstateStore>(executor: &mut TransactionExecutor<'a, L>) -> (EcdsaPublicKey, Address) {
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    (key, account)
}

fn fuzzy_vault_check(before: Decimal, after: Decimal, gain: &str) -> bool {
    let diff = after - (before + Decimal::from_str(gain).unwrap());
    let eps = Decimal::from_str("0.0000000000000001").unwrap();
    return diff < eps && diff > -eps;
}

struct TestEnv<'a, L: SubstateStore> {
    executor: TransactionExecutor<'a, L>,
    token: Address,
    instance: Address,
    initial_wallets: HashMap<Address, HashMap<Address, Decimal>>,
}

impl<'a, L: SubstateStore> TestEnv<'a, L> {
    fn new(
        ledger: &'a mut L
    ) -> (
        TestEnv<'a, L>,
        Vec<(EcdsaPublicKey, Address)>, // actors: key, account
    ) {
        let mut executor = TransactionExecutor::new(ledger, false);
        let actors : Vec<(EcdsaPublicKey, Address)> = (0..5).map(|_| create_actor(&mut executor)).collect();
        let package = executor.publish_package(include_code!("croq_order_book")).unwrap();
        let market_hand = create_actor(&mut executor);
        let token = create_token(&mut executor, market_hand.1, 10000000.into(), market_hand.0);
        actors
            .iter()
            .for_each(|(_, account)| 
              transfer_token(&mut executor, market_hand.1, *account, 100000.into(), token, market_hand.0)
            );
            
        let mut initial_wallets: HashMap<Address, HashMap<Address, Decimal>> = actors
            .iter()
            .map(|(_, account)| (*account, get_account_vaults(executor.ledger(), *account)))
            .collect();
            
        let instance =
            create_exchange(&mut executor, package, market_hand.1, token, market_hand.0);
        (
            TestEnv {
                executor,
                token,
                instance,
                initial_wallets,
            },
            actors,
        )
    }
    
    fn register_no_check(&mut self, actor: (EcdsaPublicKey, Address)) -> Receipt {
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_method(
                        self.instance,
                        "register",
                        vec![],
                        Some(actor.1),
                    )
                    .call_method_with_all_resources(actor.1, "deposit_batch")
                    .build(vec![actor.0])
                    .unwrap()
            )
            .unwrap();
        println!("{:?}\n", receipt);
        receipt
    }
    
    fn register(&mut self, actor: (EcdsaPublicKey, Address)) -> Address {
        let receipt = self.register_no_check(actor);
        assert!(receipt.result.is_ok());
        return receipt.resource_def(0).unwrap();
    }
    
    fn push_bid_no_check(&mut self, user_badge: Address, price: &str, amount_token: &str, actor: (EcdsaPublicKey, Address)) -> Receipt {
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_method(
                        self.instance,
                        "push_bid",
                        vec![
                            format!("{},{}", 1, user_badge),
                            format!("{}", price),
                            format!("{},{}", amount_token, self.token)
                        ],
                        Some(actor.1),
                    )
                    .call_method_with_all_resources(actor.1, "deposit_batch")
                    .build(vec![actor.0])
                    .unwrap()
            )
            .unwrap();
        println!("{:?}\n", receipt);
        receipt
    }
    fn push_bid(&mut self, user_badge: Address, price: &str, amount_token: &str, actor: (EcdsaPublicKey, Address)) -> Address {
        let receipt = self.push_bid_no_check(user_badge, price, amount_token, actor);
        assert!(receipt.result.is_ok());
        return receipt.resource_def(0).unwrap();
    }
    fn push_bid_no_badge(&mut self, user_badge: Address, price: &str, amount_token: &str, actor: (EcdsaPublicKey, Address)) {
        let receipt = self.push_bid_no_check(user_badge, price, amount_token, actor);
        assert!(receipt.result.is_ok());
        assert!(receipt.resource_def(0).is_none());
    }
    fn push_bid_should_fail(&mut self, user_badge: Address, price: &str, amount_token: &str, actor: (EcdsaPublicKey, Address)) {
        let receipt = self.push_bid_no_check(user_badge,price, amount_token, actor);
        assert!(!receipt.result.is_ok());
    }
    
    fn push_ask_no_check(&mut self, user_badge: Address, price: &str, amount_cash: &str, actor: (EcdsaPublicKey, Address)) -> Receipt {
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_method(
                        self.instance,
                        "push_ask",
                        vec![
                            format!("{},{}", 1, user_badge),
                            format!("{}", price),
                            format!("{},{}", amount_cash, RADIX_TOKEN)
                        ],
                        Some(actor.1),
                    )
                    .call_method_with_all_resources(actor.1, "deposit_batch")
                    .build(vec![actor.0])
                    .unwrap()
            )
            .unwrap();
        println!("{:?}\n", receipt);
        receipt
    }
    fn push_ask(&mut self, user_badge: Address, price: &str, amount_cash: &str, actor: (EcdsaPublicKey, Address)) -> Address {
        let receipt = self.push_ask_no_check(user_badge, price, amount_cash, actor);
        assert!(receipt.result.is_ok());
        return receipt.resource_def(0).unwrap();
    }
    fn push_ask_no_badge(&mut self, user_badge: Address, price: &str, amount_cash: &str, actor: (EcdsaPublicKey, Address)) {
        let receipt = self.push_ask_no_check(user_badge, price, amount_cash, actor);
        assert!(receipt.result.is_ok());
        assert!(receipt.resource_def(0).is_none());
    }
    fn push_ask_should_fail(&mut self, user_badge: Address, price: &str, amount_cash: &str, actor: (EcdsaPublicKey, Address)) {
        let receipt = self.push_ask_no_check(user_badge,price, amount_cash, actor);
        assert!(!receipt.result.is_ok());
    }
    
    fn cancel_no_check(&mut self, badge: Address, actor: (EcdsaPublicKey, Address)) -> Receipt {
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_method(
                        self.instance,
                        "cancel",
                        vec![
                            format!("{},{}", 1, badge)
                        ],
                        Some(actor.1),
                    )
                    .call_method_with_all_resources(actor.1, "deposit_batch")
                    .build(vec![actor.0])
                    .unwrap()
            )
            .unwrap();
        println!("{:?}\n", receipt);
        receipt
    }
    
    fn cancel(&mut self, badge: Address, actor: (EcdsaPublicKey, Address)) {
        let receipt = self.cancel_no_check(badge, actor);
        assert!(receipt.result.is_ok());
    }
    
    fn cancel_should_fail(&mut self, badge: Address, actor: (EcdsaPublicKey, Address)) {
        let receipt = self.cancel_no_check(badge, actor);
        assert!(!receipt.result.is_ok());
    }
    
    fn withdraw_no_check(&mut self, badge: Address, actor: (EcdsaPublicKey, Address)) -> Receipt {
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_method(
                        self.instance,
                        "withdraw",
                        vec![
                            format!("{},{}", 1, badge)
                        ],
                        Some(actor.1),
                    )
                    .call_method_with_all_resources(actor.1, "deposit_batch")
                    .build(vec![actor.0])
                    .unwrap()
            )
            .unwrap();
        println!("{:?}\n", receipt);
        receipt
    }
    
    fn withdraw(&mut self, badge: Address, actor: (EcdsaPublicKey, Address)) {
        let receipt = self.withdraw_no_check(badge, actor);
        assert!(receipt.result.is_ok());
    }
    
    fn withdraw_should_fail(&mut self, badge: Address, actor: (EcdsaPublicKey, Address)) {
        let receipt = self.withdraw_no_check(badge, actor);
        assert!(!receipt.result.is_ok());
    }
    
    fn user_vault_content(&mut self, badge: Address, actor: (EcdsaPublicKey, Address)) -> (Decimal, Decimal) {
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_method(
                        self.instance,
                        "user_vault_content",
                        vec![
                            format!("{},{}", 1, badge)
                        ],
                        Some(actor.1),
                    )
                    .call_method_with_all_resources(actor.1, "deposit_batch")
                    .build(vec![actor.0])
                    .unwrap()
            )
            .unwrap();
        println!("{:?}\n", receipt);
        assert!(receipt.result.is_ok());
        let (cash, token) = match &receipt.outputs[receipt.outputs.len()-2].dom {
          Value::Tuple(elements) => {
            (match &elements[0] {
              Value::Custom(kind, data) => Decimal::try_from(data.as_slice()).unwrap(),
              _ => Decimal::zero()
            },match &elements[1] {
              Value::Custom(kind, data) => Decimal::try_from(data.as_slice()).unwrap(),
              _ => Decimal::zero()
            })
          },
          _ => (Decimal::zero(), Decimal::zero())
        };
        (cash, token)
    }
    
    fn monitor(&mut self, actor: (EcdsaPublicKey, Address)) {
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_method(
                        self.instance,
                        "monitor",
                        vec![],
                        Some(actor.1),
                    )
                    .build(vec![actor.0])
                    .unwrap()
            )
            .unwrap();
        println!("{:?}\n", receipt);
        assert!(receipt.result.is_ok());
    }
    
    fn check_wallet(&mut self, addr: Address, tkn_gain: &str, ccy_gain: &str, txt: &str) {
        let wallet_before = &self.initial_wallets[&addr];
        let wallet_after = get_account_vaults(self.executor.ledger(), addr);
        let before_ccy = *wallet_before.get(&RADIX_TOKEN).unwrap_or(&Decimal::zero());
        let after_ccy = *wallet_after.get(&RADIX_TOKEN).unwrap_or(&Decimal::zero());
        let before_tkn = *wallet_before.get(&self.token).unwrap_or(&Decimal::zero());
        let after_tkn = *wallet_after.get(&self.token).unwrap_or(&Decimal::zero());

        println!(
          "wallet check, what we see is (ccy:{:?} tkn:{:?} => ccy:{:?} tkn:{:?}), the expected gain  is (ccy:{:?} tkn:{:?})\n",
          before_ccy, before_tkn, after_ccy, after_tkn, ccy_gain, tkn_gain
        );

        let ok = fuzzy_vault_check(before_ccy, after_ccy, ccy_gain)
            && fuzzy_vault_check(before_tkn, after_tkn, tkn_gain);
        assert!(ok, "{}", txt);
    }
}

#[test]
fn test_create() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
}
#[test]
fn test_register() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  env.register(actors[0]);
}
#[test]
fn test_push_bid_empty() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge = env.register(actors[0]);
  env.push_bid(user_badge, "5", "10", actors[0]);
  env.check_wallet(actors[0].1, "-10", "0", "unexpected wallet content");
}
#[test]
fn test_push_ask_empty() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge = env.register(actors[0]);
  env.push_ask(user_badge, "5", "50", actors[0]);
  env.check_wallet(actors[0].1, "0", "-50", "unexpected wallet content");
}
#[test]
fn test_cancel() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge = env.register(actors[0]);
  let badge = env.push_bid(user_badge, "5", "10", actors[0]);
  env.cancel(badge, actors[0]);
  env.check_wallet(actors[0].1, "0", "0", "unexpected wallet content");
}
#[test]
fn test_withdraw_nothing() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  let user_badge = env.register(actors[0]);
  env.withdraw(user_badge, actors[0]);
  env.check_wallet(actors[0].1, "0", "0", "unexpected wallet content");
}

#[test]
fn test_bid_then_ask_immediate_in_full() {

  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  env.push_bid(user_badge0, "5", "10", actors[0]);
  env.push_ask_no_badge(user_badge1, "5", "50", actors[1]);
  env.withdraw(user_badge0, actors[0]);
  env.check_wallet(actors[0].1, "-10", "50", "unexpected wallet content");
  env.check_wallet(actors[1].1, "10", "-50", "unexpected wallet content");
}

#[test]
fn test_ask_then_bid_immediate_in_full() {

  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  env.push_ask(user_badge0, "5", "50", actors[0]);
  env.push_bid_no_badge(user_badge1, "5", "10", actors[1]);
  env.withdraw(user_badge0, actors[0]);
  env.check_wallet(actors[0].1, "10", "-50", "unexpected wallet content");
  env.check_wallet(actors[1].1, "-10", "50", "unexpected wallet content");
}

#[test]
fn test_2x_bid_then_ask_immediate_in_full() {

  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  env.push_bid(user_badge0, "5", "20", actors[0]);
  env.push_ask_no_badge(user_badge1, "5", "50", actors[1]);
  env.withdraw(user_badge0, actors[0]);
  env.check_wallet(actors[0].1, "-20", "50", "unexpected wallet content");
  env.check_wallet(actors[1].1, "10", "-50", "unexpected wallet content");
}

#[test]
fn test_2x_ask_then_bid_immediate_in_full() {

  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  env.push_ask(user_badge0, "5", "100", actors[0]);
  env.push_bid_no_badge(user_badge1, "5", "10", actors[1]);
  env.withdraw(user_badge0, actors[0]);
  env.check_wallet(actors[0].1, "10", "-100", "unexpected wallet content");
  env.check_wallet(actors[1].1, "-10", "50", "unexpected wallet content");
}

#[test]
fn test_bid_then_2x_ask_partial_filled() {

  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  env.push_bid(user_badge0, "5", "10", actors[0]);
  let offer_badge = env.push_ask(user_badge1, "5", "100", actors[1]);
  env.withdraw(user_badge0, actors[0]);
  env.check_wallet(actors[0].1, "-10", "50", "unexpected wallet content");
  env.check_wallet(actors[1].1, "10", "-100", "unexpected wallet content");
  env.cancel(offer_badge, actors[1]);
  env.check_wallet(actors[1].1, "10", "-50", "unexpected wallet content");
}

#[test]
fn test_ask_then_2x_bid_partial_filled() {

  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  env.push_ask(user_badge0, "5", "50", actors[0]);
  let offer_badge = env.push_bid(user_badge1, "5", "20", actors[1]);
  env.withdraw(user_badge0, actors[0]);
  env.check_wallet(actors[0].1, "10", "-50", "unexpected wallet content");
  env.check_wallet(actors[1].1, "-20", "50", "unexpected wallet content");
  env.cancel(offer_badge, actors[1]);
  env.check_wallet(actors[1].1, "-10", "50", "unexpected wallet content");
}

#[test]
fn test_no_touching() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  let user_badge2 = env.register(actors[2]);
  let user_badge3 = env.register(actors[3]);
  let user_badge4 = env.register(actors[4]);
  env.push_ask(user_badge0, "3", "100", actors[0]);
  env.push_ask(user_badge1, "4", "25", actors[1]);
  env.push_bid(user_badge2, "5", "20", actors[2]);
  env.push_bid(user_badge3, "5", "20", actors[3]);
  env.push_bid(user_badge4, "6", "200", actors[4]);
  env.withdraw(user_badge0, actors[0]);
  env.withdraw(user_badge1, actors[1]);
  env.withdraw(user_badge2, actors[2]);
  env.withdraw(user_badge3, actors[3]);
  env.withdraw(user_badge4, actors[4]);
  env.check_wallet(actors[0].1, "0", "-100", "unexpected wallet content");
  env.check_wallet(actors[1].1, "0", "-25", "unexpected wallet content");
  env.check_wallet(actors[2].1, "-20", "0", "unexpected wallet content");
  env.check_wallet(actors[3].1, "-20", "0", "unexpected wallet content");
  env.check_wallet(actors[4].1, "-200", "0", "unexpected wallet content");
}


#[test]
fn test_sorting_ask() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  let user_badge2 = env.register(actors[2]);
  let user_badge3 = env.register(actors[3]);
  let user_badge4 = env.register(actors[4]);
  env.push_ask(user_badge0, "5", "100", actors[0]);
  env.push_ask(user_badge1, "3", "25", actors[1]);
  env.push_ask(user_badge2, "4", "20", actors[2]);
  env.push_ask(user_badge3, "2", "20", actors[3]);
  env.push_bid_no_badge(user_badge4, "4", "10", actors[4]); // Should hit immediately the @5 ask offer
  env.withdraw(user_badge0, actors[0]);
  env.withdraw(user_badge1, actors[1]);
  env.withdraw(user_badge2, actors[2]);
  env.withdraw(user_badge3, actors[3]);
  env.withdraw(user_badge4, actors[4]);
  env.check_wallet(actors[0].1, "10", "-100", "unexpected wallet content");
  env.check_wallet(actors[1].1, "0", "-25", "unexpected wallet content");
  env.check_wallet(actors[2].1, "0", "-20", "unexpected wallet content");
  env.check_wallet(actors[3].1, "0", "-20", "unexpected wallet content");
  env.check_wallet(actors[4].1, "-10", "50", "unexpected wallet content");
  
  env.push_bid(user_badge4, "4", "20", actors[4]); // will deplete @5 and @4 then stay as a bid at @4 with 5 tokens
  env.withdraw(user_badge0, actors[0]);
  env.withdraw(user_badge1, actors[1]);
  env.withdraw(user_badge2, actors[2]);
  env.withdraw(user_badge3, actors[3]);
  env.withdraw(user_badge4, actors[4]);
  env.check_wallet(actors[0].1, "20", "-100", "unexpected wallet content");
  env.check_wallet(actors[1].1, "0", "-25", "unexpected wallet content");
  env.check_wallet(actors[2].1, "5", "-20", "unexpected wallet content");
  env.check_wallet(actors[3].1, "0", "-20", "unexpected wallet content");
  env.check_wallet(actors[4].1, "-30", "120", "unexpected wallet content");
  
  env.push_ask_no_badge(user_badge0, "4", "20", actors[0]); // take user4 leftover
  env.withdraw(user_badge0, actors[0]);
  env.withdraw(user_badge1, actors[1]);
  env.withdraw(user_badge2, actors[2]);
  env.withdraw(user_badge3, actors[3]);
  env.withdraw(user_badge4, actors[4]);
  env.check_wallet(actors[0].1, "25", "-120", "unexpected wallet content");
  env.check_wallet(actors[1].1, "0", "-25", "unexpected wallet content");
  env.check_wallet(actors[2].1, "5", "-20", "unexpected wallet content");
  env.check_wallet(actors[3].1, "0", "-20", "unexpected wallet content");
  env.check_wallet(actors[4].1, "-30", "140", "unexpected wallet content");
}

#[test]
fn test_monitoring() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  let user_badge2 = env.register(actors[2]);
  let user_badge3 = env.register(actors[3]);
  let user_badge4 = env.register(actors[4]);
  env.push_ask(user_badge0, "3", "100", actors[0]);
  env.push_ask(user_badge1, "4", "25", actors[1]);
  env.push_bid(user_badge2, "5", "20", actors[2]);
  env.push_bid(user_badge3, "5", "20", actors[3]);
  env.push_bid(user_badge4, "6", "200", actors[4]);
  
  env.monitor(actors[0]);
}

#[test]
fn test_user_vault_content() {
  let mut ledger = InMemorySubstateStore::with_bootstrap();
  let (mut env, actors) = TestEnv::new(&mut ledger);
  
  let user_badge0 = env.register(actors[0]);
  let user_badge1 = env.register(actors[1]);
  env.push_ask(user_badge0, "5", "50", actors[0]);
  env.push_bid_no_badge(user_badge1, "5", "10", actors[1]);
  
  let (cash, token) = env.user_vault_content(user_badge0, actors[0]);
  assert!(cash == dec!(0), "no cash should be present");
  assert!(token == dec!(10), "10 tokens should be present");
}