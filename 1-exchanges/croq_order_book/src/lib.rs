use scrypto::prelude::*;

// This utility function create badges which are used to identify user and offers
// currently using a fungible badge, but will move to a non fungible one once unit-testing become easier
fn create_badge(name: &str) -> Bucket {
    ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
        .metadata("name", name)
        .flags(BURNABLE | FREELY_BURNABLE)
        .initial_supply_fungible(1)
}

blueprint! {
    struct CroqOrderBook {
        token_def: ResourceDef, // token to be buy or sell in this exchange
        cash_def: ResourceDef, // currency used to buy or sell in this exchange
        bid_list: Vec<(Address, Address, Decimal, Option<Vault>)>, // list of bid offers: offer badge address, user badge address, price, Token Vault
        ask_list: Vec<(Address, Address, Decimal, Option<Vault>)>, // list of ask offers: offer badge address, user badge address, price, Cash Vault
        user_vaults: HashMap<Address, (Vault, Vault)>, // this map hold the ressources that need to be collected by the users: user badge address, Cash Vault, Token Vault
        dead_vault: Vec<Vault>, // just a vector of dead vault because we can't delete empty vault currenlty
    }

    impl CroqOrderBook {
      
        // instantiation of the component with provided token address and cash, and empty structures
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
        
        // this helper method add tokens to the vault of tokens that the user can collect
        fn add_token_to_user(&mut self, badge: Address, token: Bucket) {
            // first we remove the vaults from te user_vautls hash map, the remove is needed to grab the ownership of the variable
            let mut vaults = self.user_vaults.remove(&badge);
            if vaults.is_none() {
                // if the user doesn't exist in our map, we create vaults for him
                vaults = Some((Vault::new(self.cash_def.clone()), Vault::with_bucket(token)));
            } else {
                // if the user exist take the second vaults and add the tokens to it
                vaults.as_mut().unwrap().1.put(token);
            }
            // put vaults in the user_vaults hashmap with the user badge as the key
            self.user_vaults.insert(badge, vaults.unwrap());
        }
        
        // this helper method add cash to the vault of cash that the user can collect
        // the logic is identical to add_token_to_user
        fn add_cash_to_user(&mut self, badge: Address, cash: Bucket) {
            let mut vaults = self.user_vaults.remove(&badge);
            if vaults.is_none() {
                vaults = Some((Vault::with_bucket(cash), Vault::new(self.token_def.clone())));
            } else {
                vaults.as_mut().unwrap().0.put(cash);
            }
            self.user_vaults.insert(badge, vaults.unwrap());
        }
        
        // this public method allow the caller to get a badge, the badge will be used to identify him and allow him to collect the money he has made on the offers which has been completed
        pub fn register(&self) -> Bucket {
            create_badge("user")
        }
        
        // this public method allow the user to add a bid offer (buy)
        pub fn push_bid(
            &mut self,
            user_badge: BucketRef,
            price: Decimal,
            mut cash: Bucket,
        ) -> Vec<Bucket> {
            assert!(cash.resource_def() == self.cash_def, "wrong cash type");
            assert!(price > Decimal::zero(), "negative or zero price");
            
            // prepare a new bucket to return tokens to the user if the order is executed immediately
            let mut ret_token_bucket = Bucket::new(self.token_def.clone());
            // the ask_list is sorted in decreasing order of price, so we check if the last one has a selling price which is superior or equal to the amount our user is ready to buy
            while !self.ask_list.is_empty() && self.ask_list[self.ask_list.len() - 1].2 <= price {
                // if we enter in the loop it mean that our order will be at least partially filled
                // we remove the most interesting offer from the list
                let mut offer = self.ask_list.pop().unwrap();
                let seller_badge = offer.1;
                let ask_price = offer.2;
                // we compute the amount of token which can be buyed at this price by our buyer
                let offer_qty = cash.amount() / ask_price;
                let vault = offer.3.as_mut().unwrap();                
                if offer_qty < vault.amount() {
                    // if there is more tokens in the offer vault than needed
                    
                    // we add the money to the vault of the seller
                    self.add_cash_to_user(seller_badge, cash);
                    // we take what we need
                    ret_token_bucket.put(vault.take(offer_qty));
                    // we put back the offer in the list
                    self.ask_list.push(offer);
                    // and we return the tokens to the buyer
                    return vec![ret_token_bucket];
                } else if offer_qty == vault.amount() {
                    // if there is the exact amount of token we want on the vault
                    
                    // we add the money to the vault of the seller
                    self.add_cash_to_user(seller_badge, cash);
                    // we take what we need (all)
                    ret_token_bucket.put(vault.take_all());
                    // we push the now empty vault in the list of dead_vault
                    self.dead_vault.push(offer.3.take().unwrap());
                    // and we return the tokens to the buyer
                    return vec![ret_token_bucket];
                }
                // if there is not enough token, we arrive in this branch
                let qty = vault.amount();
                // we compute the cost of the whole offer
                let cost = qty * ask_price;
                // we add the money to the vault of the seller
                self.add_cash_to_user(seller_badge, cash.take(cost));
                // we add all the tokens to the bucket which will be returned
                ret_token_bucket.put(vault.take_all());
                // we push the now empty vault in the list of dead_vault
                self.dead_vault.push(offer.3.take().unwrap());
            }
            if cash.amount() == Decimal::zero() {
                return vec![ret_token_bucket];
            }
            // if we arrive here, the offer has not been completelly fullfilled
            // we create a badge for our buyer
            let badge = create_badge("bid offer");
            // we add the offer to the bid_list
            self.bid_list.push((
                badge.resource_address(),
                user_badge.resource_address(),
                price,
                Some(Vault::with_bucket(cash)),
            ));
            // we sort the list
            self.bid_list.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
            // and we return the tokens (if it has been partially filled) and offer badge
            vec![ret_token_bucket, badge]
        }
                
        // this public method allow the user to add a ask offer (sell)
        // logic is similar to the push_bid method, so no comments
        pub fn push_ask(
            &mut self,
            user_badge: BucketRef,
            price: Decimal,
            mut token: Bucket,
        ) -> Vec<Bucket> {
            assert!(token.resource_def() == self.token_def, "wrong token type");
            assert!(price > Decimal::zero(), "negative or zero price");
            let mut ret_cash_bucket = Bucket::new(self.cash_def.clone());
            while !self.bid_list.is_empty() && self.bid_list[self.bid_list.len() - 1].2 >= price {
                let mut offer = self.bid_list.pop().unwrap();
                let buyer_badge = offer.1;
                let bid_price = offer.2;
                let offer_cost = token.amount() * bid_price;
                let vault = offer.3.as_mut().unwrap();
                if offer_cost < vault.amount() {
                    self.add_token_to_user(buyer_badge, token);
                    ret_cash_bucket.put(vault.take(offer_cost));
                    self.bid_list.push(offer);
                    return vec![ret_cash_bucket];
                } else if offer_cost == vault.amount() {
                    self.add_token_to_user(buyer_badge, token);
                    ret_cash_bucket.put(vault.take_all());
                    self.dead_vault.push(offer.3.take().unwrap());
                    return vec![ret_cash_bucket];
                }
                let cash = vault.amount();
                let qty = cash / bid_price;
                self.add_token_to_user(buyer_badge, token.take(qty));
                ret_cash_bucket.put(vault.take_all());
                self.dead_vault.push(offer.3.take().unwrap());
            }
            if token.amount() == Decimal::zero() {
                return vec![ret_cash_bucket];
            }
            let badge = create_badge("ask offer");
            self.ask_list.push((
                badge.resource_address(),
                user_badge.resource_address(),
                price,
                Some(Vault::with_bucket(token)),
            ));
            self.ask_list.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
            vec![ret_cash_bucket, badge]
        }
        
        // this method allow the user to cancel an offer
        pub fn cancel(&mut self, offer_badge: Bucket) -> (Bucket, Bucket) {
            let mut cash_bucket = Bucket::new(self.cash_def.clone());
            let mut token_bucket = Bucket::new(self.token_def.clone());
            // we are looking for the offer_badge inside the bid_list
            // a lot of the complexity come from the inability to discard empty vaults
            // the filter allow us to keep only the offer which have the offer_badge
            self.bid_list
                .iter_mut()
                .filter(|offer| offer.0 == offer_badge.resource_address())
                .for_each(|mut offer| {
                    // for each offer which have the offer badge we take the vault
                    let vault_option = offer.3.take();
                    let mut vault = vault_option.unwrap();
                    // we empty the vault in the bucket
                    cash_bucket.put(vault.take_all());
                    // push the now empty vault in the dead_vault list
                    self.dead_vault.push(vault);
                    // set the offer price to zero (an invalid value)
                    offer.2 = Decimal::zero();
                });
            // remove from the list all the offer with an invalid price
            self.bid_list.retain(|offer| offer.2 > Decimal::zero());
            // then we're doing the same for the ask_list
            self.ask_list
                .iter_mut()
                .filter(|offer| offer.0 == offer_badge.resource_address())
                .for_each(|mut offer| {
                    let vault_option = offer.3.take();
                    let mut vault = vault_option.unwrap();
                    token_bucket.put(vault.take_all());
                    self.dead_vault.push(vault);
                    offer.2 = Decimal::zero();
                });
            self.ask_list.retain(|offer| offer.2 > Decimal::zero());
            // the badge is now useless, we can burn it
            offer_badge.burn();
            // and we return to our caller the tokens and cash stored in the cancelled offer
            (cash_bucket, token_bucket)
        }
        
        // this method is called by the user to collect the cash and tokens which has been generated by the successful fullfillement of offers
        pub fn withdraw(&mut self, user_badge: BucketRef) -> Vec<Bucket> {
            let addr = user_badge.resource_address();
            let mut ret = Vec::<Bucket>::new();
            // remove the user vaults associated with this user badge from the hashmap user_vaults
            let opt_vaults = self.user_vaults.remove(&addr);
            if opt_vaults.is_some() {
                // if there is vaults, empty them and insert them back
                let mut vaults = opt_vaults.unwrap();
                ret.push(vaults.0.take_all());
                ret.push(vaults.1.take_all());
                self.user_vaults.insert(addr, vaults);
            }
            ret
        }
        
        // this method is used by a user who want to know he content of his vaults to check if he need to call withdraw
        pub fn user_vault_content(&self, user_badge: BucketRef) -> (Decimal, Decimal) {
            let addr = user_badge.resource_address();
            // get the vaults associated with this user badge from the hashmap user_vaults
            let opt_vaults = self.user_vaults.get(&addr);
            if opt_vaults.is_none() {
                // if the user doesn't exist, return (0,0)
                return (Decimal::zero(), Decimal::zero());
            } else {
                // if the user exist return the amount of token and cash contained in the vaults
                let vaults = opt_vaults.unwrap();
                return (vaults.0.amount(), vaults.1.amount());
            }
        }
        
        // this method is used to monitor the current state of the auction, we can easily build a UI around it to display the order book
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
