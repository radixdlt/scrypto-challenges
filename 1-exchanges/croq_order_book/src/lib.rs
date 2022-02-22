use scrypto::prelude::*;

fn create_badge(name: &str) -> Bucket {
    ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
        .metadata("name", name)
        .flags(BURNABLE | FREELY_BURNABLE)
        .initial_supply_fungible(1)
}

blueprint! {
    struct CroqOrderBook {
        token_def: ResourceDef,
        cash_def: ResourceDef,
        bid_list: Vec<(Address, Address, Decimal, Option<Vault>)>, // Token
        ask_list: Vec<(Address, Address, Decimal, Option<Vault>)>, // Cash
        user_vaults: HashMap<Address, (Vault, Vault)>,             // Cash, Token
        dead_vault: Vec<Vault>,
    }

    impl CroqOrderBook {
        pub fn instantiate(token: Address, cash: Address) -> Component {
            Self {
                token_def: ResourceDef::from(token),
                cash_def: ResourceDef::from(cash),
                bid_list: Vec::new(),
                ask_list: Vec::new(),
                user_vaults: HashMap::new(),
                dead_vault: Vec::new(),
            }
            .instantiate()
        }
        fn add_token_to_user(&mut self, badge: Address, token: Bucket) {
            let mut vaults = self.user_vaults.remove(&badge);
            if vaults.is_none() {
                vaults = Some((Vault::new(self.cash_def.clone()), Vault::with_bucket(token)));
            } else {
                vaults.as_mut().unwrap().1.put(token);
            }
            self.user_vaults.insert(badge, vaults.unwrap());
        }
        fn add_cash_to_user(&mut self, badge: Address, cash: Bucket) {
            let mut vaults = self.user_vaults.remove(&badge);
            if vaults.is_none() {
                vaults = Some((Vault::with_bucket(cash), Vault::new(self.token_def.clone())));
            } else {
                vaults.as_mut().unwrap().0.put(cash);
            }
            self.user_vaults.insert(badge, vaults.unwrap());
        }
        pub fn register(&self) -> Bucket {
            create_badge("user")
        }
        pub fn push_bid(
            &mut self,
            user_badge: BucketRef,
            price: Decimal,
            mut token: Bucket,
        ) -> Vec<Bucket> {
            assert!(token.resource_def() == self.token_def, "wrong token type");
            assert!(price > Decimal::zero(), "negative or zero price");
            let mut ret_cash_bucket = Bucket::new(self.cash_def.clone());
            while !self.ask_list.is_empty() && self.ask_list[self.ask_list.len() - 1].2 >= price {
                let mut offer = self.ask_list.pop().unwrap();
                let buyer_badge = offer.1;
                let ask_price = offer.2;
                let bid_cost = token.amount() * ask_price;
                let vault = offer.3.as_mut().unwrap();
                if bid_cost < vault.amount() {
                    self.add_token_to_user(buyer_badge, token);
                    ret_cash_bucket.put(vault.take(bid_cost));
                    self.ask_list.push(offer);
                    return vec![ret_cash_bucket];
                } else if bid_cost == vault.amount() {
                    self.add_token_to_user(buyer_badge, token);
                    ret_cash_bucket.put(vault.take(bid_cost));
                    self.dead_vault.push(offer.3.take().unwrap());
                    return vec![ret_cash_bucket];
                }
                let cash = vault.amount();
                let qty = cash / ask_price;
                self.add_token_to_user(buyer_badge, token.take(qty));
                ret_cash_bucket.put(vault.take_all());
                self.dead_vault.push(offer.3.take().unwrap());
            }
            if token.amount() == Decimal::zero() {
                return vec![ret_cash_bucket];
            }
            let badge = create_badge("bid offer");
            self.bid_list.push((
                badge.resource_address(),
                user_badge.resource_address(),
                price,
                Some(Vault::with_bucket(token)),
            ));
            self.bid_list.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
            vec![ret_cash_bucket, badge]
        }
        pub fn push_ask(
            &mut self,
            user_badge: BucketRef,
            price: Decimal,
            mut cash: Bucket,
        ) -> Vec<Bucket> {
            assert!(cash.resource_def() == self.cash_def, "wrong cash type");
            assert!(price > Decimal::zero(), "negative or zero price");
            let mut ret_token_bucket = Bucket::new(self.token_def.clone());
            while !self.bid_list.is_empty() && self.bid_list[self.bid_list.len() - 1].2 <= price {
                let mut offer = self.bid_list.pop().unwrap();
                let seller_badge = offer.1;
                let bid_price = offer.2;
                let ask_qty = cash.amount() / bid_price;
                let vault = offer.3.as_mut().unwrap();
                info!(
                    "ask_qty: {:?} vault.amount(): {:?}",
                    ask_qty,
                    vault.amount()
                );
                if ask_qty < vault.amount() {
                    self.add_cash_to_user(seller_badge, cash);
                    ret_token_bucket.put(vault.take(ask_qty));
                    self.bid_list.push(offer);
                    return vec![ret_token_bucket];
                } else if ask_qty == vault.amount() {
                    self.add_cash_to_user(seller_badge, cash);
                    ret_token_bucket.put(vault.take(ask_qty));
                    self.dead_vault.push(offer.3.take().unwrap());
                    return vec![ret_token_bucket];
                }
                let qty = vault.amount();
                let cost = qty * bid_price;
                self.add_cash_to_user(seller_badge, cash.take(cost));
                ret_token_bucket.put(vault.take_all());
                self.dead_vault.push(offer.3.take().unwrap());
            }
            if cash.amount() == Decimal::zero() {
                return vec![ret_token_bucket];
            }
            let badge = create_badge("ask offer");
            self.ask_list.push((
                badge.resource_address(),
                user_badge.resource_address(),
                price,
                Some(Vault::with_bucket(cash)),
            ));
            self.ask_list.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
            vec![ret_token_bucket, badge]
        }
        pub fn cancel(&mut self, offer_badge: Bucket) -> (Bucket, Bucket) {
            let mut cash_bucket = Bucket::new(self.cash_def.clone());
            let mut token_bucket = Bucket::new(self.token_def.clone());
            self.bid_list
                .iter_mut()
                .filter(|offer| offer.0 == offer_badge.resource_address())
                .for_each(|mut offer| {
                    let vault_option = offer.3.take();
                    let mut vault = vault_option.unwrap();
                    token_bucket.put(vault.take_all());
                    self.dead_vault.push(vault);
                    offer.2 = Decimal::zero();
                });
            self.bid_list.retain(|offer| offer.2 > Decimal::zero());
            self.ask_list
                .iter_mut()
                .filter(|offer| offer.0 == offer_badge.resource_address())
                .for_each(|mut offer| {
                    let vault_option = offer.3.take();
                    let mut vault = vault_option.unwrap();
                    cash_bucket.put(vault.take_all());
                    self.dead_vault.push(vault);
                    offer.2 = Decimal::zero();
                });
            self.ask_list.retain(|offer| offer.2 > Decimal::zero());
            offer_badge.burn();
            (cash_bucket, token_bucket)
        }
        pub fn withdraw(&mut self, user_badge: BucketRef) -> Vec<Bucket> {
            let addr = user_badge.resource_address();
            let mut ret = Vec::<Bucket>::new();
            let opt_vaults = self.user_vaults.remove(&addr);
            if opt_vaults.is_some() {
                let mut vaults = opt_vaults.unwrap();
                ret.push(vaults.0.take_all());
                ret.push(vaults.1.take_all());
                self.user_vaults.insert(addr, vaults);
            }
            ret
        }
        pub fn user_vault_content(&self, user_badge: BucketRef) -> (Decimal, Decimal) {
            let addr = user_badge.resource_address();
            let opt_vaults = self.user_vaults.get(&addr);
            if opt_vaults.is_none() {
                return (Decimal::zero(), Decimal::zero());
            } else {
                let vaults = opt_vaults.unwrap();
                return (vaults.0.amount(), vaults.1.amount());
            }
        }
        pub fn monitor(&self) {
            info!("token addr: {:?}", self.token_def.address());
            info!("cash addr: {:?}", self.cash_def.address());

            info!("**bid**");
            info!("floor price, quantity of tokens");
            self.bid_list.iter().for_each(|item| {
                info!(
                    "{:?}, {:?}",
                    item.2,
                    item.3.as_ref().map_or(Decimal::zero(), |v| v.amount())
                );
            });

            info!("**ask**");
            info!("ceilling price, amount of cash");
            self.ask_list.iter().for_each(|item| {
                info!(
                    "{:?}, {:?}",
                    item.2,
                    item.3.as_ref().map_or(Decimal::zero(), |v| v.amount())
                );
            });
        }
    }
}
