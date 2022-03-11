use radix_engine::engine::*;
use radix_engine::ledger::*;
use radix_engine::model::*;
use radix_engine::transaction::*;
use sbor::any::*;
use scrypto::prelude::*;

fn create_account<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
) -> (EcdsaPublicKey, Address) {
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    (key, account)
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
                .unwrap(),
        )
        .unwrap();
    return receipt.resource_def(0).unwrap();
}

fn get_vault_info<'a, L: SubstateStore>(
    ledger: &'a L,
    component: &Address,
    id: &Vid,
) -> (Address, Decimal) {
    let vault = ledger.get_vault(component, id).unwrap();
    let amount = vault.amount();
    let resource_def_address = vault.resource_address();
    (resource_def_address, amount)
}

fn get_lazymap_info<'a, L: SubstateStore>(
    ledger: &'a L,
    component: &Address,
    id: &Mid,
) -> Vec<(Address, Decimal)> {
    let lazy_map = ledger.get_lazy_map(component, id).unwrap();
    lazy_map
        .map()
        .iter()
        .flat_map(|(_, data)| {
            let validated_data = validate_data(data).unwrap();
            validated_data
                .vaults
                .iter()
                .map(|vid| get_vault_info(ledger, component, vid))
                .collect::<Vec<(Address, Decimal)>>()
        })
        .collect()
}

fn get_account_vaults<'a, L: SubstateStore>(
    ledger: &'a L,
    account: Address,
) -> HashMap<Address, Decimal> {
    let component = ledger.get_component(account).unwrap();
    let state = component.state();
    let validated_data = validate_data(state).unwrap();
    validated_data
        .lazy_maps
        .iter()
        .flat_map(|mid| get_lazymap_info(ledger, &account, &mid))
        .collect()
}

fn create_market<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    package: Address,
    account: Address,
    quote_token: Address,
    base_token: Address,
    key: EcdsaPublicKey,
) -> Address {
    let receipt = executor
        .run(
            TransactionBuilder::new(executor)
                .call_function(
                    package,
                    "Market",
                    "instantiate_market",
                    vec![
                        format!("{}", quote_token),
                        format!("{}", base_token),
                        "TestMarket".to_string(),
                    ],
                    Some(account),
                )
                .call_method_with_all_resources(account, "deposit_batch")
                .build(vec![key])
                .unwrap(),
        )
        .unwrap();
    println!("{:?}\n", receipt);
    receipt.component(0).unwrap()
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
                    &Resource::Fungible {
                        amount: nb_token,
                        resource_address: token,
                    },
                    account_from,
                )
                .call_method_with_all_resources(account_to, "deposit_batch")
                .build(vec![key])
                .unwrap(),
        )
        .unwrap();
    assert!(receipt.result.is_ok());
}

struct Trader {
    key: EcdsaPublicKey,
    address: Address,
    access_badge_address: Address,
    quote_token: Address,
    base_token: Address,
}

fn init<'a, L: SubstateStore>(
    ledger: &'a mut L,
) -> (TransactionExecutor<'a, L>, Address, Vec<Trader>) {
    let mut executor = TransactionExecutor::new(ledger, false);
    let package = executor
        .publish_package(include_code!("orderbook"))
        .unwrap();
    let market_hand = create_account(&mut executor);
    let quote_token = create_token(
        &mut executor,
        market_hand.1,
        From::<u32>::from(10000000),
        market_hand.0,
    );
    let base_token = create_token(
        &mut executor,
        market_hand.1,
        From::<u32>::from(10000000),
        market_hand.0,
    );

    let instance = create_market(
        &mut executor,
        package,
        market_hand.1,
        quote_token,
        base_token,
        market_hand.0,
    );

    let traders: Vec<Trader> = (0..2)
        .map(|_| {
            let (key, address) = create_account(&mut executor);
            //println!("create_account key:{:?}, address:{:?}\n", key, address);
            transfer_token(
                &mut executor,
                market_hand.1,
                address,
                From::<u32>::from(100000),
                quote_token,
                market_hand.0,
            );
            transfer_token(
                &mut executor,
                market_hand.1,
                address,
                From::<u32>::from(100000),
                base_token,
                market_hand.0,
            );
            let access_badge_address =
                create_trader_openorders(&mut executor, instance, key, address);
            Trader {
                key,
                address,
                access_badge_address,
                quote_token,
                base_token,
            }
        })
        .collect();
    (executor, instance, traders)
}

fn create_trader_openorders<'a, L: radix_engine::ledger::SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    instance: Address,
    trader_key: EcdsaPublicKey,
    trader_addresse: Address,
) -> Address {
    let receipt = executor
        .run(
            TransactionBuilder::new(executor)
                .call_method(instance, "create_openorders", vec![], Some(trader_addresse))
                .call_method_with_all_resources(trader_addresse, "deposit_batch")
                .build(vec![trader_key])
                .unwrap(),
        )
        .unwrap();
    assert!(receipt.result.is_ok());
    //get new badge from account data.
    //I don't really understand why it doesn't return from the Tx.
    let data = get_account_vaults(executor.ledger(), trader_addresse);
    let badge = data
        .into_iter()
        .filter_map(|(address, amount)| (amount == Decimal::one()).then(|| address))
        .next();
    assert!(badge.is_some());
    badge.unwrap()
}

fn push_bid_order<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    instance: Address,
    price: usize,
    amount_base: usize,
    amount_token: usize,
    order_type: u8,
    trader: &Trader,
) {
    let receipt = executor
        .run(
            TransactionBuilder::new(executor)
                .call_method(
                    instance,
                    "buy_order",
                    vec![
                        format!("{}", price),
                        format!("{}", amount_base),
                        format!("{}", order_type),
                        format!("{},{}", amount_token, trader.quote_token),
                        format!("{},{}", 1, trader.access_badge_address),
                    ],
                    Some(trader.address),
                )
                .call_method_with_all_resources(trader.address, "deposit_batch")
                .build(vec![trader.key])
                .unwrap(),
        )
        .unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());
}

fn push_ask_order<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    instance: Address,
    price: usize,
    amount_base: usize,
    order_type: u8,
    trader: &Trader,
) {
    let receipt = executor
        .run(
            TransactionBuilder::new(executor)
                .call_method(
                    instance,
                    "sell_order",
                    vec![
                        format!("{}", price),
                        format!("{}", amount_base),
                        format!("{}", order_type),
                        format!("{},{}", amount_base, trader.base_token),
                        format!("{},{}", 1, trader.access_badge_address),
                    ],
                    Some(trader.address),
                )
                .call_method_with_all_resources(trader.address, "deposit_batch")
                .build(vec![trader.key])
                .unwrap(),
        )
        .unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());
}

fn push_withdraw<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    instance: Address,
    trader: &Trader,
) {
    let receipt = executor
        .run(
            TransactionBuilder::new(executor)
                .call_method(
                    instance,
                    "withdraw",
                    vec![format!("{},{}", 1, trader.access_badge_address)],
                    Some(trader.address),
                )
                .call_method_with_all_resources(trader.address, "deposit_batch")
                .build(vec![trader.key])
                .unwrap(),
        )
        .unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());
}

fn check_wallet<'a, L: SubstateStore>(
    executor: &mut TransactionExecutor<'a, L>,
    index: u8,
    trader: &Trader,
) {
    let wallet = get_account_vaults(executor.ledger(), trader.address);
    let quote = *wallet.get(&trader.quote_token).unwrap_or(&Decimal::zero());
    let base = *wallet.get(&trader.base_token).unwrap_or(&Decimal::zero());

    println!("WALLET:{} quote:{:?} base:{:?}", index, quote, base);
}

#[test]
fn test_trade1() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (mut executor, instance, traders) = init(&mut ledger);
    check_wallet(&mut executor, 0, &traders[0]);
    check_wallet(&mut executor, 1, &traders[1]);
    push_ask_order(&mut executor, instance, 20, 10, 0, &traders[1]);
    check_wallet(&mut executor, 1, &traders[1]);
    push_ask_order(&mut executor, instance, 19, 10, 0, &traders[1]);
    check_wallet(&mut executor, 0, &traders[0]);
    check_wallet(&mut executor, 1, &traders[1]);
    push_bid_order(&mut executor, instance, 20, 20, 400, 0, &traders[0]);
    check_wallet(&mut executor, 0, &traders[0]);
    check_wallet(&mut executor, 1, &traders[1]);
    push_withdraw(&mut executor, instance, &traders[0]);
    push_withdraw(&mut executor, instance, &traders[1]);
    check_wallet(&mut executor, 0, &traders[0]);
    check_wallet(&mut executor, 1, &traders[1]);
    push_ask_order(&mut executor, instance, 20, 10, 0, &traders[1]);
    check_wallet(&mut executor, 1, &traders[1]);
    push_bid_order(&mut executor, instance, 20, 20, 400, 0, &traders[0]);
    check_wallet(&mut executor, 0, &traders[0]);
    check_wallet(&mut executor, 1, &traders[1]);
    push_withdraw(&mut executor, instance, &traders[0]);
    push_withdraw(&mut executor, instance, &traders[1]);
    check_wallet(&mut executor, 0, &traders[0]);
    check_wallet(&mut executor, 1, &traders[1]);
    //    push_bid_order(&mut executor, instance, 1, 30, 40, &traders[1]);
    //    push_bid_order(&mut executor, instance, 1, 30, 40, &traders[1]);
}
