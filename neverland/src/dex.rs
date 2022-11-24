use sbor::*;
use scrypto::prelude::*;

blueprint! {
    struct TokenDex {        
        // XRD vault.
        collected_xrd: Vault,         
        // Token Hashmap of vaults.                                   
        token_vaults: HashMap<ResourceAddress, Vault>,      
        
        // Token Hashmap with name, symbol, price.              
        token_map: HashMap<ResourceAddress, (String, String, Decimal)>,        
        // metaBadge Hashmap with entry fee level, MetaToken amount & token address.  
        badge_map: HashMap<ResourceAddress, (Decimal, Decimal, ResourceAddress)>,         
        // Token Hashmap with accrued fee, MetaToken amount & address.
        meta_map: HashMap<ResourceAddress, (Decimal, Decimal, ResourceAddress)>,         
        // MetaToken Hashmap with MetaToken resource adresses. 
        meta: HashMap<ResourceAddress, MetaToken>,  
        
        // Badge to mint and burn MetaTokens.                      
        minter_badge: Vault,         
        // Owner badge to determine protocol fee and collect accrued XRD fee.                                     
        owner_badge: ResourceAddress,      
        
        // Protocol XRD fee variable.                           
        xrd_fee: Decimal,   
        // Amount of accrued XRD protocol fee withdrawed by protocol owner.                                      
        xrd_claimed: Decimal,                                     
        // Protocol fee variable.
        fee: Decimal                                              
    }

    impl TokenDex {
        pub fn new(fee: Decimal) -> (ComponentAddress,Bucket) {
            let minter_badge : Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " MinterBadge ")
                .initial_supply(1);

            let badge_bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " OwnerBadge ")
                .initial_supply(1);

            let access_rules = AccessRules::new()
                .method("claim_xrd_fee", rule!(require(badge_bucket.resource_address())))
                .method("set_fee", rule!(require(badge_bucket.resource_address())))
                .default(rule!(allow_all));

            let mut token_dex: TokenDexComponent = Self {
                collected_xrd: Vault::new(RADIX_TOKEN),
                token_vaults: HashMap::new(),
                token_map: HashMap::new(),
                badge_map: HashMap::new(),
                meta_map: HashMap::new(),
                meta: HashMap::new(),
                minter_badge: Vault::with_bucket(minter_badge),
                owner_badge: badge_bucket.resource_address(),
                xrd_fee: Decimal::zero(),
                xrd_claimed: Decimal::zero(),
                fee
            }
            .instantiate();
            token_dex.add_access_check(access_rules);
            
            (token_dex.globalize(),badge_bucket)
        }

            // Create a metaBadge and populate a hashmap to associate a determinated entry fee level
            // to a token stoke event.
            fn add_meta_badge(
                &mut self, 
                symbol: String, 
                token_addr: ResourceAddress, 
                entry_fee: Decimal, 
                meta_amnt: Decimal
            ) -> Bucket {
                let meta_badge_res_def: ResourceAddress = ResourceBuilder::new_fungible()
                    .metadata("symbol", format!(" mBadge{}", symbol.clone()))
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply();
                
                self.badge_map.insert(meta_badge_res_def,(entry_fee, meta_amnt, token_addr));

                self.minter_badge.authorize(|| { borrow_resource_manager!(meta_badge_res_def).mint(1) })
            }

            // Populate a hashmap relating a determinated metaBadge address to a MetaToken amount
            // with the purpose of guarantee a fair Protocol's accrued fee distribution.
            fn meta_amounts(
                &mut self, 
                meta_badge: Bucket, 
                meta_amount: Decimal, 
                flag: Decimal
            ) -> Bucket {
                let meta_badge_addr: ResourceAddress = meta_badge.resource_address().clone();
                
                let (a,mut m_token_amnt,c) = *self.badge_map.get(&meta_badge_addr).unwrap();
                let entry_fee = a;
                let token_addr = c;               
                
                // Determine if caller is a "stock/restock" method or an "unstock" one and udate
                // consequentely MetaToken amount, burn metaBadge if all liquidity is removed. 
                if flag == dec!("0") {
                    m_token_amnt = m_token_amnt+meta_amount;
                    self.badge_map.insert(meta_badge_addr,(entry_fee,m_token_amnt,token_addr));
                    meta_badge
                } else if flag == dec!("1") {
                    assert!( meta_amount <= m_token_amnt," Let's check passed amount ");
                    m_token_amnt = m_token_amnt-meta_amount;
                  
                    if m_token_amnt == dec!("0") {
                        self.badge_map.remove(&meta_badge_addr);
                        TokenDex::badge_burn(self, meta_badge);
                        Bucket::new(RADIX_TOKEN)
                    } else { 
                        self.badge_map.insert(meta_badge_addr,(entry_fee,m_token_amnt,token_addr));
                        meta_badge 
                    }
                } else { 
                    std::process::abort() 
                }
            }

            // Create a MetaToken resource relative to a kind of token provided to protocol by end 
            // users and populate related hashmaps.
            fn add_meta_token(
                &mut self, 
                name: String, 
                symbol: String, 
                address: ResourceAddress
            ) -> ResourceAddress {
                assert!(!self.meta.contains_key(&address)," token already exist ");

                let  meta_res_def: ResourceAddress = ResourceBuilder::new_fungible()
                    .metadata("name", format!(" m{}", name.clone()))
                    .metadata("symbol", format!(" m{}", symbol.clone()))
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply();

                self.meta.insert(address.clone(),MetaToken::new(name, symbol, address, meta_res_def));
                
                match self.meta_map.get_mut(&address.clone()) {
                    Some((_a,_b,c)) => *c = meta_res_def,
                    None => std::process::abort()                  
                };
               
                meta_res_def
            }

            // Mint a MetaToken amount relative to an amount of token provided to protocol and 
            // update total MetaToken minted amount.
            fn meta_mint(&mut self, token_amnt: Decimal, token_address: ResourceAddress) -> Bucket {             
                match self.meta_map.get_mut(&token_address.clone()) {
                    Some((_a,minted_amnt,_c)) => *minted_amnt = *minted_amnt+token_amnt,
                    None => std::process::abort()                  
                };

                let m_token = self.meta.get_mut(&token_address).unwrap();
               
                self.minter_badge.authorize(|| { borrow_resource_manager!(m_token.meta_res_def).mint(token_amnt) })
            }                                   

            // Burn a MetaToken amount relative to amount of token claimed by end user 
            // via "unstock_token" function.
            fn meta_burn(&mut self, meta_token: Bucket) {                
                self.minter_badge.authorize(|| {meta_token.burn()});
            }

            // Burn metaBadge once all relative MetaToken has been claimed by end user 
            // via "unstock_token" function.
            fn badge_burn(&mut self, meta_badge: Bucket) {
                self.minter_badge.authorize(|| {meta_badge.burn()});
            }

            // Retrieve price of specific token type.
            fn tokenprice(&mut self, token_addr: ResourceAddress) -> Decimal {
                let price: Decimal;
                
                match self.token_map.get(&token_addr) {
                    Some((_a,_b,c)) => price = *c,
                    None => { 
                        info!(" token not in stock! ");
                        std::process::abort()
                    }
                };
                
                price
            }

            // Put token to sell in vault whenever end user specify an exact number of token to sell 
            // using swap functions.
            fn tokenput_pri(
                &mut self, 
                prc_in: Decimal, 
                prc_out: Decimal, 
                addr_in: ResourceAddress, 
                token: Bucket
            ) -> (Decimal,Decimal) {
                let token_addr = token.resource_address();
                let mut token_amnt = token.amount();   
                           
                let nmbr = token.amount()*prc_in/prc_out;
                
                // Adjust token amount adding protocol fee if needed by identified calling method.
                if prc_in == dec!("0") { 
                    token_amnt = TokenDex::adjust_fee(self, token_amnt); 
                }
                
                // Call a method to calculate token output amount.
                let amount = TokenDex::token_sum(self, token_amnt, token_addr, addr_in, 1);

                // Put token in vault.
                let v = self.token_vaults.get_mut(&token_addr).unwrap();

                v.put(token);

                (nmbr,amount)
            }

            // Put token to sell in vault whenever the number of token to sell is determined by 
            // protocol.
            fn tokenput_sec(
                &mut self, 
                amnt: Decimal, 
                addr: ResourceAddress, 
                prc_in: Decimal, 
                prc_out: Decimal, 
                mut token: Bucket
            ) -> Bucket {
                let token_addr = token.resource_address();
                let amount = token.amount();                
                let amnt_in: Decimal;
                
                // Determine relative math formula once identified calling method.
                if prc_in == dec!("0") {
                    let amount_in = TokenDex::token_sum(self, amnt, addr, token_addr, 2); 
                    amnt_in = amount_in-amount_in*self.fee/100;
                } else {
                    amnt_in = amnt*prc_in/prc_out;
                }           

                assert!( amnt_in <= amount, "Not enough input amount ");

                // Put token in vault.
                let v = self.token_vaults.get_mut(&token_addr).unwrap();

                v.put(token.take(amnt_in));

                token
            }

            // Take buyed token from token vault and increment total accrued fee in relative hashmap.
            fn tokentake(&mut self, token_out_nbr: Decimal, token_out_addr: ResourceAddress) -> Bucket {
                match self.meta_map.get_mut(&token_out_addr.clone()) {
                    Some((amnt_fee,_b,_c)) => *amnt_fee = *amnt_fee+token_out_nbr*self.fee/100,
                    None => std::process::abort()                  
                };

                match self.token_vaults.get_mut(&token_out_addr) {
                    Some(vault) => vault.take(token_out_nbr-(token_out_nbr*self.fee/100)),
                    None => { info!("token not in stock! ");
                              std::process::abort()
                    }
                }
            }

            // Calculate new token price.
            fn price_mod(
                &mut self, 
                amount: Decimal, 
                address: ResourceAddress, 
                price: Decimal, 
                flag: i32
            ) -> Decimal {

                // Retrieve token reserve amount.
                let total_token = self.token_vaults.get(&address).unwrap().amount();
            
                // Determine relative math formula once identified calling method. 
                match flag {
                    1 => total_token*price/(total_token-amount),         
                    2 => (total_token*price + amount*price)/total_token, 
                    3 => (total_token*price - amount*price)/total_token,
                    _ => total_token*price/(total_token+amount)
                }
            }

            // Calculate token output amount.
            fn token_sum(
                &mut self, 
                amnt_pri: Decimal, 
                addr_pri: ResourceAddress, 
                addr_sec: ResourceAddress, 
                flag: i32
            )-> Decimal {
                let mut amount: Decimal = amnt_pri;

                // Retrieve token prices.
                let price_in: Decimal = TokenDex::tokenprice(self, addr_pri);
                let price_out: Decimal = TokenDex::tokenprice(self, addr_sec);

                let token_out_amnt = amnt_pri*price_in/price_out;

                // Determine relative math formula once identified calling method.
                match flag {
                    1 => {
                        let tot_amnt_out = self.token_vaults.get(&addr_sec).unwrap().amount();
                        let price_new = price_out*tot_amnt_out/(tot_amnt_out+token_out_amnt);
                        price_new*token_out_amnt/price_out
                    },
                    2 => {
                        let tot_amnt_out = self.token_vaults.get(&addr_pri).unwrap().amount();
                        tot_amnt_out*token_out_amnt/(tot_amnt_out-amnt_pri)
                    },
                    _ => {
                        let mut price_new: Decimal = 1.into();
                        
                        if flag == 3 {      
                            amount = amnt_pri/price_in;
                            price_new = TokenDex::price_mod(self, amount, addr_pri, price_in, 2);
                        } else if flag == 0 {
                            price_new = TokenDex::price_mod(self, amount, addr_pri, price_in, 1);
                        }

                        let (name,symbol,_price) = self.token_map.get(&addr_pri).unwrap();
                        let name_str = name.to_string();
                        let symbol_str = symbol.to_string();
                        self.token_map.insert(addr_pri,(name_str,symbol_str,price_new));
                        amnt_pri/price_new
                    }
                }
            }

            // Adjust buying exact token amount neutralizing protocol fee incidence on final amount.
            fn adjust_fee(&mut self, amount_in: Decimal ) -> Decimal {
                amount_in*dec!("100")/(dec!("100")-self.fee)
            }

            fn info_amounts(ttl: Decimal, af: Decimal, ef: Decimal, df: Decimal, nr: Decimal) {
                info!(" total_minted {} ",ttl);
                info!(" accrued_fee {} ",af);
                info!(" entry_fee {} ",ef);
                info!(" delta_fee {} ",df);
                info!(" token_out_nbr {} ",nr);
            }

            // Set protocol fee function whom only protocol owner can succesfully call.
        pub fn set_fee(&mut self, prtcl_fee: Decimal) {
            assert!(prtcl_fee >= dec!("0") && prtcl_fee <= dec!("1")," Let's pass a fee in thousandths! ");            
            self.fee = prtcl_fee;
            info!(" Protocol fee set to {}% ", self.fee);
        }

            // Claim accrued XRD fee function whom only protocol owner can succesfully call.
        pub fn claim_xrd_fee(&mut self) -> Bucket {
            info!(" Fee value {} XRD ", self.xrd_fee);
            let xrd_output: Bucket = self.collected_xrd.take(self.xrd_fee);
            self.xrd_fee = dec!("0");
            self.xrd_claimed += xrd_output.amount();
            info!(" Protocol fee claimed {} XRD ", self.xrd_claimed);
            
            xrd_output
        }

            // Stock token function callable by an end user wishing to supply unpresent liquidity to 
            // protocol.
        pub fn stock_token(&mut self, token: Bucket, new_price: Decimal) -> (Bucket,Bucket) {
            let token_addr = token.resource_address();
            let token_amnt = token.amount();
            let token_res_def = token.resource_address();

            let name = borrow_resource_manager!(token_res_def).metadata()["name"].clone();
            let symbol = borrow_resource_manager!(token_res_def).metadata()["symbol"].clone();

            // Verify token to stock isn't XRD, amount is greater than zero, this token hasn't  
            // been stocked yet.
            assert!( token_addr != RADIX_TOKEN," Cannot stock XRD as token ");
            assert!(new_price > dec!("0"), "new price must be a positive value");
            
            if self.token_map.contains_key(&token_addr) {
                info!(" token already in Vault. Please use restock_token function ");
                std::process::abort()
            }
            info!(" Added {} {} token, {} symbol @{}XRD price ", token_amnt, name, symbol ,new_price);
            
            // Put token in vault.
            let v = self.token_vaults.entry(token_addr).or_insert(Vault::new(token_addr));            
            v.put(token);
            
            let none = dec!("0");
            
            // Insert token address as well as other metadata in relative hashmaps.
            self.token_map.insert(token_addr,(name.clone(),symbol.clone(),new_price));
            self.meta_map.insert(token_addr,(none,none,token_addr));
            
            // Mint MetaToken, metaBadge and insert relative hashmaps values. 
            self.add_meta_token(name.clone(), symbol.clone(), token_addr);
            let meta_token = self.meta_mint(token_amnt, token_addr);           
            let meta_amount = meta_token.amount();
            let meta_badge = self.add_meta_badge(symbol, token_addr, none, meta_amount);
            self.badge_map.insert(meta_badge.resource_address().clone(),(none,none,token_addr));
            let output_badge = self.meta_amounts(meta_badge, meta_amount, none);
            
            (meta_token,output_badge)
        }

            // Restock token function callable by an end user wishing to supply present liquidity to 
            // protocol.
        pub fn restock_token(&mut self, token: Bucket) -> (Bucket,Bucket) {
            let token_addr = token.resource_address();            
            assert!( token_addr != RADIX_TOKEN," Cannot stock XRD as token ");
            
            let amnt = token.amount();

            // Verify token to restock already exists within Protocol.
            match self.token_map.get(&token_addr) {
                Some((a,b,c)) => 
                    info!(" Adding {} {} token, {} symbol, @{} $XRD price ", amnt, a.to_string(), b.to_string(), c),
                _ => { 
                    info!(" Found no token in Vault. Please use stock_token function ");
                    std::process::abort()
                }
            }
            
            // Put token in vault.
            let v = self.token_vaults.get_mut(&token_addr).unwrap();

            v.put(token);
            
            // Mint relative MetaToken tokens.    
            let meta_token: Bucket = self.meta_mint(amnt, token_addr);

            // Retrieve entry fee level.
            let (accrued_fee,_b,_c) = self.meta_map.get(&token_addr).unwrap();
            info!(" entry_fee {} ",accrued_fee);
            
            // Mint a metaBadge and update relative hashmaps values.
            let entry_fee = *accrued_fee;            
            let (_name,symbol,_price) = self.token_map.get(&token_addr).unwrap();
            let symbol_str = symbol.to_string();
            let meta_amount = meta_token.amount();
            let meta_badge = self.add_meta_badge(symbol_str, token_addr, entry_fee, meta_amount);
            
            self.badge_map.insert(meta_badge.resource_address()
                .clone(),(entry_fee,dec!("0"),token_addr));
            
            let output_badge = self.meta_amounts(meta_badge, meta_amount, dec!("0"));
            
            (meta_token,output_badge)
        }

            // Unstock token function callable by an end user wishing to withdraw owned token 
            // liquidity amount from protocol.
        pub fn unstock_token(
            &mut self, 
            token_addr: ResourceAddress, 
            meta_token: Bucket, 
            meta_badge: Bucket
        ) -> (Bucket,Bucket,Bucket) {

            // Verify supplied resources correspondances. 
            let badge_amnt: Decimal = meta_badge.amount();
            assert!( badge_amnt >= dec!("1"), " Please supply meta badge ");
            
            let (accrued_fee,total_minted,meta_address) = self.meta_map.get(&token_addr).unwrap();
            assert!(meta_address == &meta_token.resource_address()," Mismatch between token & MetaToken! ");
            
            let (entry_fee,_b,token_address) = 
                self.badge_map.get(&meta_badge.resource_address()).unwrap();
            assert!(token_address == &token_addr," MetaBadge address unrecognized! ");
            
            let meta_token_amnt: Decimal = meta_token.amount();
            let token_out_nbr: Decimal;
            let token_bucket: Bucket;
            let xrd_out: Bucket;
            let delta_fee = *accrued_fee-(*entry_fee);
            
            // Determine output token amount.
            token_out_nbr = meta_token_amnt+delta_fee*meta_token_amnt/(*total_minted);

            // Display amounts
            TokenDex::info_amounts(*total_minted,*accrued_fee,*entry_fee,delta_fee,token_out_nbr);

            // Retrieve token reserve amount.
            let total_token = self.token_vaults.get(&token_addr).unwrap().amount();
            
            // Check if there's enough token to reimburse user otherwise reimburse the difference
            // in XRD.
            if token_out_nbr <= total_token {
                token_bucket = match self.token_vaults.get_mut(&token_addr) {
                    Some(vault) => vault.take(token_out_nbr),
                    None => {
                        info!("token not in stock !");
                        std::process::abort()
                    }
                };
                let zero = Decimal::zero();
                xrd_out = self.collected_xrd.take(zero);
            } else {  let delta_token = token_out_nbr-total_token;
                token_bucket = match self.token_vaults.get_mut(&token_addr) {
                    Some(vault) => vault.take(total_token),
                    None => {
                        info!("token not in stock !");
                        std::process::abort()
                    }
                };
                let price_in: Decimal = TokenDex::tokenprice(self, token_addr);
                let xrd_amnt = delta_token*price_in;
                assert!( xrd_amnt <= self.collected_xrd.amount(), " Not enough XRD in Vault ");
                xrd_out = self.collected_xrd.take(xrd_amnt);
            }

            // Burn relative MetaToken, verify user's remaining token liquidity is greater then zero
            // otherwise burn metaBadge, update relative hashmaps values. 
            let meta_token_amnt = meta_token.amount().clone();
            self.meta_burn(meta_token);
            let output_badge = self.meta_amounts(meta_badge, meta_token_amnt, dec!("1"));
            
            (token_bucket,xrd_out,output_badge)
        }

            // Retrieve liquidity provider position providing a relative metaBadge as reference.
        pub fn stock_position(&mut self, meta_badge: Proof) {
            let meta_badge: ValidatedProof = meta_badge.unsafe_skip_proof_validation();
            assert!( meta_badge.amount() == dec!("1"), " Please provide your own metaBadge as reference ");
            
            match self.badge_map.get(&meta_badge.resource_address()) {
                Some((a,b,c)) => { 
                    let (accrued_fee,total_minted,_meta_address) = self.meta_map.get(&c).unwrap();
                    let delta_fee = *accrued_fee-(*a);
                    let token_out_nbr = *b+delta_fee*(*b)/(*total_minted);

                    // Display amounts
                    TokenDex::info_amounts(*total_minted,*accrued_fee,*a,delta_fee,token_out_nbr);
                },
                None => {
                    info!(" No badge's correspondence! ");
                    std::process::abort()
                }
            }
        }

            // Get price, name, symbol of a determinated token giving his resource address.
        pub fn get_price(&self, token_addr: ResourceAddress) {
            assert!( token_addr != RADIX_TOKEN, " XRD is priceless ");
           
            match self.token_map.get(&token_addr) {
                Some((a,b,c)) => 
                    info!(" Resource address: {}, Name: {}, Symbol: {}, Price: @{} $XRD ",token_addr ,a,b,c),
                None => info!("Could not find token in stock !")
            }
        }

            // Get reserve amount of a determinated token giving his resource address.
        pub fn get_reserve(&self, token_addr: ResourceAddress) {
            match self.token_map.get(&token_addr) {
                Some((a,_b,_c)) => { 
                    let total_token = self.token_vaults.get(&token_addr).unwrap().amount();
                    info!(" {} token reserve amount is {} ", a, total_token);
                },
                None => {
                    info!(" Could not find token in stock !");
                    std::process::abort()
                }
            }
        }

            // Get protocol's tokens menu.
        pub fn menu(&self){
            for (addr_name,(str_name,str_sym,price)) in self.token_map.iter() {
                info!(" At address {} we've got {} ({}) token @ {} $XRD each ", addr_name, str_name, str_sym, price);
            }
        }

            // Get token sell amount. Use with function "buy_exact_xrd_sell_token" (bexsc)
        pub fn get_token_sell_amount_bexsc(
            &mut self, 
            token_addr: ResourceAddress, 
            xrd_amnt: Decimal
        ) -> Decimal {
            let xrd_amount = self.adjust_fee(xrd_amnt);
            let price = self.tokenprice(token_addr);            
            let new_price = self.price_mod(xrd_amount/price, token_addr, price, 3);
    
            xrd_amount/new_price
        }

            // Get XRD buy amount. Use with function "buy_xrd_sell_exact_token" (bxsec)
        pub fn get_xrd_buy_amount_bxsec(
            &mut self, 
            token_addr: ResourceAddress, 
            token_amnt: Decimal
        ) -> Decimal {
            let price = self.tokenprice(token_addr);
            let new_price = self.price_mod(token_amnt, token_addr, price, 0);
            
            (token_amnt*new_price)-(token_amnt*new_price)*self.fee/100
        }

            // Get token buy amount. Use with function "buy_token_sell_exact_xrd" (bcsex)
        pub fn get_token_buy_amount_bcsex(
            &mut self, 
            token_addr: ResourceAddress, 
            xrd_amnt: Decimal
        ) -> Decimal {
            let price = TokenDex::tokenprice(self, token_addr);
            let new_price = self.price_mod(xrd_amnt/price, token_addr, price, 2);
            
            (xrd_amnt/new_price)-(xrd_amnt/new_price)*self.fee/100
        }
            
            // Get XRD sell amount. Use with function "buy_exact_token_sell_xrd" (becsx)
        pub fn get_xrd_sell_amount_becsx(
            &mut self, 
            token_addr: ResourceAddress, 
            token_amnt: Decimal
        ) -> Decimal {
            let token_amount = self.adjust_fee(token_amnt);
            let price = self.tokenprice(token_addr);
            let new_price = self.price_mod(token_amount, token_addr, price, 1);
            
            token_amount*new_price
        }

            // Get token sell amount. Use with function "buy_exact_token_sell_token" (becsc)
        pub fn get_token_sell_amount_becsc(
            &mut self, 
            amnt_in: Decimal, 
            addr_in: ResourceAddress, 
            addr_out: ResourceAddress 
        ) -> Decimal {
            let amount_in = self.adjust_fee(amnt_in);
            let amount = self.token_sum(amount_in, addr_in, addr_out, 2);

            amount-amount*self.fee/100
        }

            // Get token buy amount. Use with function "buy_token_sell_exact_token"(bcsec)
        pub fn get_token_buy_amount_bcsec(
            &mut self, 
            addr_in: ResourceAddress, 
            amnt_out: Decimal, 
            addr_out: ResourceAddress
        ) -> Decimal {
            let amount_out = self.adjust_fee(amnt_out);
            let amount = self.token_sum(amount_out, addr_out, addr_in, 1);
            
            amount-amount*self.fee/100
        }

            // Obtain a minimum token amount in exchange of an exact XRD amount. 
            // Function swap exact XRD for token.
        pub fn buy_token_sell_exact_xrd(
            &mut self, 
            min_in: Decimal, 
            addr_in: ResourceAddress, 
            xrd_out: Bucket
        ) -> Bucket {
            let xrd_amnt = xrd_out.amount();
            self.collected_xrd.put(xrd_out);           
            let amount_in = self.token_sum(xrd_amnt, addr_in, addr_in, 3);
            assert!( amount_in >= min_in, "Not enough tokens output amount");
            
            self.tokentake(amount_in, addr_in)
        }

            // Obtain a minimum token amount in exchange of an exact token amount. 
            // Function swap exact token for token.
        pub fn buy_token_sell_exact_token(
            &mut self, 
            min_in: Decimal, 
            addr_in: ResourceAddress, 
            token_out: Bucket
        ) -> Bucket {
            let addr_out = token_out.resource_address();            
            assert!(addr_in != addr_out," Same token's address detect! ");
            let (_nmbr,amount_in) = self.tokenput_pri(dec!("0"), dec!("1"), addr_in, token_out);
            assert!( amount_in >= min_in, "Not enough tokens output amount");
            
            self.tokentake(amount_in, addr_in)
        }

            // Obtain a minimum XRD amount in exchange of an exact token amount. 
            // Function swap exact token for XRD.
        pub fn buy_xrd_sell_exact_token(&mut self, xrd_min: Decimal, token_out: Bucket) -> Bucket {
            let addr = token_out.resource_address();
            let price_out = self.tokenprice(token_out.resource_address());
            assert!( token_out.amount()*price_out <= self.collected_xrd.amount(), "Not enough XRD in Vault");
            let new_price = self.price_mod(token_out.amount(), addr, price_out, 0);
            let (name,symbol,_price) = self.token_map.get(&addr).unwrap();
            self.token_map.insert(addr,(name.to_string(),symbol.to_string(),new_price));
            let (nmbr,_amount_in) = self.tokenput_pri(new_price*new_price, new_price, addr, token_out);
            assert!( nmbr >= xrd_min , "Not enough xrd output amount");
            self.xrd_fee = self.xrd_fee+nmbr*self.fee/100;

            self.collected_xrd.take(*&(nmbr-nmbr*self.fee/100))
        }

            // Obtain an exact token amount in exchange of a maximum XRD amount. 
            // Function swap XRD for exact token.
        pub fn buy_exact_token_sell_xrd(
            &mut self, 
            nbr_in: Decimal, 
            addr_in: ResourceAddress, 
            mut xrd_out: Bucket
        ) -> (Bucket,Bucket) {
            let amnt_in = self.adjust_fee(nbr_in);
            let mut xrd_amnt = self.token_sum(amnt_in, addr_in, addr_in, 0);
            xrd_amnt = amnt_in*amnt_in/xrd_amnt;
            assert!( xrd_amnt <=  xrd_out.amount(), " Not enough XRD input");
            self.collected_xrd.put(xrd_out.take(xrd_amnt));
            
            (self.tokentake(amnt_in, addr_in),xrd_out)
        }

            // Obtain an exact token amount in exchange of a maximum token amount. 
            // Function swap token for exact token.
        pub fn buy_exact_token_sell_token(
            &mut self,            
            amnt_in: Decimal, 
            addr_in: ResourceAddress, 
            token_out: Bucket
        ) -> (Bucket,Bucket) {
            let addr_out = token_out.resource_address();    
            assert!(addr_in != addr_out," Same token's address detect! ");
            let amount_in = self.adjust_fee(amnt_in);
            
            (
                self.tokenput_sec(amount_in, addr_in, dec!("0"), dec!("1"), token_out),
                self.tokentake(amount_in, addr_in)
            )
        }
        
            // Obtain an exact XRD amount in exchange of a maximum token amount. 
            // Function swap token for exact XRD.
        pub fn buy_exact_xrd_sell_token(
            &mut self, 
            xrd_in: Decimal, 
            token_out: Bucket
        ) -> (Bucket,Bucket) {
            let addr = token_out.resource_address();
            let xrd_input = self.adjust_fee(xrd_in);
            assert!(xrd_in <= self.collected_xrd.amount(), "Not enough XRD in Vault");
            let price_out: Decimal = self.tokenprice(token_out.resource_address());
            let new_price = self.price_mod(xrd_input/price_out, addr, price_out, 3);
            
            match self.token_map.get_mut(&addr) {
                Some((_a,_b,price)) => *price = new_price,
                None => std::process::abort()                  
            };

            self.xrd_fee = self.xrd_fee+xrd_input*self.fee/100;

            (
                self.tokenput_sec(xrd_input, addr, dec!("1"), new_price, token_out),
                self.collected_xrd.take(*&(xrd_input-xrd_input*self.fee/100))
            )
        }

            // Request a flashswap performing a call to an external Component address.
        pub fn flashswap(
            &mut self, 
            amnt_in: Decimal, 
            addr_in: ResourceAddress, 
            bckt_addr: ResourceAddress, 
            ext_addr: ComponentAddress, 
            method: String
        ) -> Bucket {                
            let amount_in = self.adjust_fee(amnt_in);
            
            let token_bucket: Bucket;
            let token_output: Bucket;

            let price_in: Decimal;
            let price_out: Decimal;
            
            // Conditional code block to verify nature of resources to borrow, their existance 
            // within Protocol's vaults, collect them and retrieve their prices in case of 
            // affermative response.
            if  addr_in == bckt_addr {    
                price_in = Decimal::one();
                price_out = Decimal::one();

                if addr_in == RADIX_TOKEN {
                    token_bucket = self.collected_xrd.take(*&(amount_in-amount_in*self.fee/100));
                    self.xrd_fee = self.xrd_fee+amount_in*self.fee/100;
                } else { 
                    token_bucket = self.tokentake(amount_in, addr_in); 
                }

            } else if addr_in == RADIX_TOKEN && addr_in != bckt_addr {
                token_bucket = self.collected_xrd.take(*&(amount_in-amount_in*self.fee/100));
                self.xrd_fee = self.xrd_fee+amount_in*self.fee/100;
                price_in = Decimal::one();
                price_out = self.tokenprice(bckt_addr);
            } else if addr_in != bckt_addr && bckt_addr == RADIX_TOKEN {
                token_bucket = self.tokentake(amount_in, addr_in);
                price_in = self.tokenprice(addr_in);
                price_out = Decimal::one();
            } else if bckt_addr != RADIX_TOKEN {
                token_bucket = self.tokentake(amount_in, addr_in);
                price_in = self.tokenprice(addr_in);
                price_out = self.tokenprice(bckt_addr);
            } else { 
                info!(" Check out addresses! "); 
                std::process::abort(); 
            }
            
            // Encode arguments to call external Component within borrowed resources and call him. 
            let args = args![scrypto_encode(&token_bucket),scrypto_encode(&bckt_addr)];
            let token_return = borrow_component!(ext_addr).call::<Bucket>(&method.to_string(), args);

            // Calculate Protocol's repay amount to refurbish.
            let amount = token_return.amount();
            let nmbr = (amnt_in+amnt_in*self.fee/100)*price_in/price_out;

            // Check if transaction has been profitable or abort it.
            if (amount-nmbr) < dec!("0") {
               info!(" Sorry mate, ain't nothin' to scrape! ");
               std::process::abort();
            }
            
            // Check nature of returned resources and collect them consequentely.
            if bckt_addr != RADIX_TOKEN {   
                let v = self.token_vaults.get_mut(&bckt_addr).unwrap();
                v.put(token_return);
                token_output = v.take(amount-nmbr);    
            } else { 
                self.collected_xrd.put(token_return);
                token_output = self.collected_xrd.take(amount-nmbr); 
            }

            token_output
        }
    }
}

// Build a structure and implement it to populate a meta hashmap and relate MetaToken resource with 
// respective token resource.
#[derive(Debug, Clone, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub struct MetaToken {
    token_name: String,
    token_symbol: String,
    token_address: ResourceAddress,
    meta_res_def: ResourceAddress,
}

impl MetaToken {
    pub fn new(
        token_name: String,
        token_symbol: String,
        token_address: ResourceAddress,
        meta_res_def: ResourceAddress,
    ) -> Self {
        Self {
            token_name,
            token_symbol,
            token_address,
            meta_res_def,
        }
    }
}
