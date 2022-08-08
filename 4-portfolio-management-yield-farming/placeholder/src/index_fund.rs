use scrypto::prelude::*;
use radex::radex::*;
use degenfi::degenfi::*;
use crate::price_oracle::*;

blueprint! {
    struct IndexFund {
        fund_admin_address: ResourceAddress,
        fund_name: String,
        fund_ticker: String,
        fund_vaults: HashMap<ResourceAddress, Vault>,
        token_weights: HashMap<ResourceAddress, Decimal>,
        // Mints fund tokens
        fund_token_admin_vault: Vault,
        fund_token_address: ResourceAddress,
        starting_share_price: Decimal,
        price_oracle_address: ComponentAddress,
        radex_address: Option<ComponentAddress>,
        degenfi_address: Option<ComponentAddress>,
        degenfi_vaults: HashMap<ResourceAddress, Vault>,
        borrow_vaults: HashMap<ResourceAddress, Vault>,
        loan_vault: Option<Vault>,
        fee_to_pool: Decimal,
        fee_vault: Vault,
    }

    impl IndexFund {
        
        pub fn new(
            fund_name: String,
            fund_ticker: String,
            fee_to_pool: Decimal,
            starting_share_price: Decimal,
            tokens: HashMap<ResourceAddress, Decimal>,
            price_oracle_address: ComponentAddress,
        ) -> (ComponentAddress, Bucket)
        {
            assert!(
                (fee_to_pool >= Decimal::zero()) & (fee_to_pool <= dec!("100")), 
                "[Fund Creation]: Fee must be between 0 and 100"
            );

            assert!(
                starting_share_price > Decimal::zero(), 
                "[Fund Creation]: Starting share price must be greater than zero"
            );

            let fund_admin: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("{} Admin Badge", fund_name))
                .metadata("symbol", "FO")
                .metadata("description", "Badge that represents admin authority of the fund.")
                .initial_supply(1);
            
            let fund_token_admin: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("{} Token Admin Badge", fund_name))
                .metadata("symbol", "FAB")
                .metadata("description", format!("Admin badge to mint/burn {} tokens", fund_ticker))
                .initial_supply(1);

            let fund_token_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", format!("{} Tokens", fund_name))
                .metadata("symbol", format!("{}", fund_ticker))
                .metadata("description", "Tokens that represent ownerhip of the fund.")
                .mintable(rule!(require(fund_token_admin.resource_address())), LOCKED)
                .burnable(rule!(require(fund_token_admin.resource_address())), LOCKED)
                .no_initial_supply();
            
            let vault_amount = tokens.iter();
            let mut fund_vaults: HashMap<ResourceAddress, Vault> = HashMap::new();
            let mut token_weights: HashMap<ResourceAddress, Decimal> = HashMap::new();

            let mut cumulative_token_weight: Decimal = Decimal::zero();
            for (token_address, token_weight) in vault_amount {
                assert_ne!(
                    borrow_resource_manager!(*token_address).resource_type(), ResourceType::NonFungible,
                    "[Fund Creation]: Assets must be fungible."
                );
                cumulative_token_weight += *token_weight;
                fund_vaults.insert(*token_address, Vault::new(*token_address));
                token_weights.insert(*token_address, *token_weight);
            };

            assert_eq!(cumulative_token_weight, Decimal::one(), 
                "[Fund Creation]: The total weighting of collections of tokens must equal to 100%.",
            );

            let index_fund: ComponentAddress = Self {
                fund_admin_address: fund_admin.resource_address(),
                fund_name: fund_name,
                fund_ticker: fund_ticker.clone(),
                fund_vaults: fund_vaults,
                starting_share_price: starting_share_price,
                token_weights: token_weights,
                fund_token_admin_vault: Vault::with_bucket(fund_token_admin),
                fund_token_address: fund_token_address,
                price_oracle_address: price_oracle_address,
                radex_address: None,
                degenfi_address: None,
                degenfi_vaults: HashMap::new(),
                borrow_vaults: HashMap::new(),
                loan_vault: None,
                fee_to_pool: fee_to_pool,
                fee_vault: Vault::new(fund_token_address),
            }
            .instantiate()
            .globalize();

            return (index_fund, fund_admin);
        }

        /// Gets the resource addresses of the tokens in this liquidity pool and returns them as a `Vec<ResourceAddress>`.
        /// 
        /// # Returns:
        /// 
        /// `Vec<ResourceAddress>` - A vector of the resource addresses of the tokens in this liquidity pool.
        pub fn addresses(&self) -> Vec<ResourceAddress> {
            return self.degenfi_vaults.keys().cloned().collect::<Vec<ResourceAddress>>();
        }

        pub fn integrate_dex(
            &mut self, 
            radex_address: ComponentAddress
        )
        {
            self.radex_address = Some(radex_address);
        }

        pub fn integrate_lending(
            &mut self, 
            degenfi_address: ComponentAddress
        )
        {

            self.degenfi_address = Some(degenfi_address);
        }

        pub fn add_token(
            &mut self,
            token: HashMap<ResourceAddress, Decimal>
        )
        {
            let tokens = token.iter();
            let mut cumulative_token_weight: Decimal = Decimal::one();
            for (token_address, token_weight) in tokens{
                cumulative_token_weight += *token_weight;
                self.token_weights.entry(*token_address).and_modify(|e| { *e = *token_weight }).or_insert(*token_weight);
                self.fund_vaults.entry(*token_address).or_insert(Vault::new(*token_address));
            }

            assert_eq!(cumulative_token_weight, Decimal::one(), 
                "[{:?} Fund]: The total weighting of collections of tokens must equal to 100%.",
                self.fund_name
            );
        }

        /// Ensure that the remove tokens don't have a position in liquidity pool.
        pub fn remove_token(
            &mut self,
            token: HashMap<ResourceAddress, Decimal>,
            mut token_to_remove: Vec<ResourceAddress>, 
        )
        {
            // * REMOVES TOKENS * //
            let remove_len = token_to_remove.len();
            let mut counter = 0;
            while counter < remove_len {
                let token_address: Option<ResourceAddress> = token_to_remove.pop(); 
                match token_address {
                    Some(token) => {
                        self.token_weights.remove_entry(&token);
                    }
                    None => {}
                }
                counter += 1;
            }

            // * REBALANCES TOKEN WEIGHTS * //
            let tokens = token.iter();
            let mut cumulative_token_weight: Decimal = Decimal::one();
            for (token_address, token_weight) in tokens{
                cumulative_token_weight += *token_weight;
                self.token_weights.entry(*token_address).and_modify(|e| { *e = *token_weight });
            }

            assert_eq!(cumulative_token_weight, Decimal::one(), 
                "[{:?} Fund]: The total weighting of collections of tokens must equal to 100%.",
                self.fund_name
            );
        }

        pub fn buy(
            &mut self,
            xrd: Bucket
        )  -> Bucket
        {
            assert_eq!(xrd.resource_address(), RADIX_TOKEN,
                "[Fund Locker]: You can only purchase Fund Tokens with XRD."
            );

            let output_token: ResourceAddress = self.fund_token_address; 
            let radex: RaDEX = self.radex_address.unwrap().into();
            let return_bucket: Bucket = radex.swap(xrd, output_token);

            return_bucket
        }

        pub fn sell(
            &mut self,
            fund_token: Bucket
        ) -> Bucket
        {
            assert_eq!(fund_token.resource_address(), self.fund_token_address,
                "[Fund Locker]: You can only sell Fund Tokens."
            );

            let output_token = RADIX_TOKEN;
            let radex: RaDEX = self.radex_address.unwrap().into();
            let return_bucket: Bucket = radex.swap(fund_token, output_token);

            return_bucket   
        }

        pub fn issue_tokens(
            &mut self,
            mut tokens: Vec<Bucket>,
        ) -> Bucket
        {
            // Retrieves how many bucket of tokens are being passed.
            let number_of_tokens = tokens.len();

            // * CALCULATE THE TOTAL AMOUNT OF TOKENS AND CUMULATIVE VALUE OF TOKENS * //
            let mut cumulative_token_amount: Decimal = Decimal::zero();
            let mut cumulative_value: Decimal = Decimal::zero();
            let tokens_iter = tokens.iter();
            let price_oracle: PriceOracle = self.price_oracle_address.into();
            for bucket in tokens_iter {
                let token_amount: Decimal = bucket.amount();
                let price: Decimal = price_oracle.get_price(bucket.resource_address());
                let token_value: Decimal = token_amount * price; 
                cumulative_token_amount += token_amount;
                cumulative_value += token_value;
            }
            info!("Amount of tokens: {:?}", cumulative_token_amount);
            info!("Value of tokens: {:?}", cumulative_value);
            
            let mut amount_to_mint: Decimal = Decimal::zero();

            let mut counter = 0;
            while counter < number_of_tokens {
                // Retrieves each bucket of tokens.
                let token_buckets: Option<Bucket> = tokens.pop();

                match token_buckets {
                    Some(token) => { // If a bucket exists...

                        assert_ne!(
                            borrow_resource_manager!(token.resource_address()).resource_type(), ResourceType::NonFungible,
                            "[Fund Locker]: Assets must be fungible."
                        );

                        let token_address: ResourceAddress = token.resource_address();

                        // * MINTS FUND TOKENS * //
                        // Takes the weight of one of the collateral and multiplies against the total value of the tokens
                        // deposited. The total weight of each collateral should equal to 100%.
                        let token_weight: Decimal = *self.token_weights.get(&token_address).unwrap();
                        let fund_tokens_to_mint: Decimal = cumulative_value * token_weight;

                        info!("Token weight: {:?}", token_weight);
                        
                        amount_to_mint += fund_tokens_to_mint.round(0, RoundingMode::TowardsPositiveInfinity);

                        assert_eq!(self.fund_vaults.contains_key(&token_address), true,
                            "[Fund Locker]: This token does not belong to this fund."
                        );

                        let fund_vault: &mut Vault = self.fund_vaults.get_mut(&token_address).unwrap();

                        fund_vault.put(token);

                    }
                    None => {
                        
                        info!("[Fund Locker]: All tokens deposited!");

                    }
                }

                counter += 1;

                info!("[Fund Locker]: Fund tokens issues: {:?}", amount_to_mint);

                }

            if counter == number_of_tokens {
                let fund_token = self.fund_token_admin_vault.authorize(|| 
                    borrow_resource_manager!(self.fund_token_address).mint(amount_to_mint)
                );

                let price_oracle: PriceOracle = self.price_oracle_address.into();
                price_oracle.set_price(self.fund_token_address, Decimal::one());

                info!("[Fund Locker]: Amount of Fund Tokens issued: {:?}", amount_to_mint);

                fund_token
            } else {
                let empty_bucket = self.fund_token_admin_vault.take(0);

                empty_bucket
            }
        }

        pub fn redeem(
            &mut self,
            fund_token: Bucket,
        ) -> Vec<Bucket>
        {
            assert_eq!(fund_token.resource_address(), self.fund_token_address,
                "[Fund Locker]: You may only redeem fund tokens that belongs to this index."
            );

            // * VALUES THE TOTAL AMOUNT OF FUND TOKENS PASSED. * //
            let token_amount: Decimal = fund_token.amount();
            let price_oracle: PriceOracle = self.price_oracle_address.into();
            let token_price: Decimal = price_oracle.get_price(fund_token.resource_address());
            let token_value: Decimal = token_price * token_amount;

            // * TAKES THE NUMBER OF COLLATERAL IN THE FUND TO BE LOOPED OVER * //
            let number_of_tokens: usize = self.token_weights.len();
            let mut token_addresses = self.fund_vaults.keys().cloned().collect::<Vec<ResourceAddress>>();
            let mut counter = 0;

            let mut return_collateral: Vec<Bucket> = Vec::new();

            // * LOOPS OVER EACH COLLATERAL OF THE FUND TO GET THE WEIGHT OF EACH COLLATERAL * //
            while counter < number_of_tokens {

                let collateral_token: Option<ResourceAddress> = token_addresses.pop();

                match collateral_token {
                    Some(token) => { 

                        // * RETRIEVES EACH COLLATERAL WEIGHT AND MULTIPLY AGAINST THE TOTAL VALUE OF THE TOKEN FUND * //
                        let collateral_weight: Decimal = *self.token_weights.get(&token).unwrap();
                        let collateral_price = price_oracle.get_price(token);
                        let collateral_value = collateral_price * collateral_weight;
                        let collateral_amount = token_value * collateral_value;

                        // * PUSHES THE BUCKET OF EACH COLLATERAL TO BE RETURNED TO THE INVESTOR * //
                        let collateral_bucket: Bucket = self.fund_vaults.get_mut(&token).unwrap().take(collateral_amount);
                        return_collateral.push(collateral_bucket);


                        info!("[Redeem]: {:?} of {:?}", token, collateral_amount);

                    }
                    None => {}
                }
                counter += 1;
                info!("[Redeem]: Counter: {:?}", counter);
            }
            let bucket_amount = return_collateral.len();
            info!("[Redeem]: Bucket: {:?}", bucket_amount);

            self.fund_token_admin_vault.authorize(|| fund_token.burn());

            return_collateral

        }

        /// Allows Fund Managers to supply liquidity into a RaDEX AMM pool and earn protocol fees.
        /// 
        /// This method is used to provide Fund Managers additional tools to formulate investment strategies
        /// for Fund Managers. Currently, Fund Managers are only allowed to supply liquidity to an existing 
        /// AMM Pool. The reasoning for this is that while this protocol aims to provide Fund Managers with 
        /// a suite of tools to exercise flexibility in managing their fund, Fund Managers should exercise 
        /// some prudence by ideally supplying liquidity to established AMM Pools.
        /// 
        /// This method performs a few checks before liquidity can be supplied:
        /// 
        /// ** Check 1:** - Checks that the badge presented is authorized to manage the fund.
        /// 
        /// ** Check 2:** - Checks that the fund contains the tokens requested to supply liquidity.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `token1_address` (ResourceAddress) - The ResourceAddress of the first requested token
        /// to be used to supply liquidity.
        /// * `token1_amount` (Decimal) - The amount of Token 1 requested to supply liquidity.
        /// * `token2_address` (ResourceAddress) - The ResourceAddress of the second requested token
        /// to be used to supply liquidity.
        /// * `token2_amount` (Decimal) - The amount of Token 2 requested to supply liquidity.
        pub fn add_liquidity(
            &mut self,
            fund_admin: Proof,
            token1_address: ResourceAddress,
            token1_amount: Decimal,
            token2_address: ResourceAddress,
            token2_amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Trading feature has not been integrated. You must first integrate RaDEX",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&token1_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );            

            assert_eq!(self.fund_vaults.contains_key(&token2_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let token1: Bucket = self.withdraw(token1_address, token1_amount);
            let token2: Bucket = self.withdraw(token2_address, token2_amount);

            let radex: RaDEX = self.radex_address.unwrap().into();

            let (option_bucket1, option_bucket2, tracking_tokens): 
            (Option<Bucket>, Option<Bucket>, Bucket) = radex.add_liquidity(token1, token2);

            // Retrieves the corresponding tracking token address. If the key exist, the value is
            // returned and the bucket of tokens is deposited into the vault. If not, a vault is 
            // created with the bucket of tokens. 
            self.fund_vaults.entry(tracking_tokens.resource_address())
            .and_modify(|e| {e.put(tracking_tokens) } )
            .or_insert(Vault::with_bucket(tracking_tokens));
        }

        /// Allows Fund Manager to exit out of their liquidity pool position. 
        /// 
        /// This method is used to allow Fund Managers to remove liquidity out of an AMM Pool
        /// and collect the fees owed to the fund. 
        /// 
        /// This method performs a few check before liquidity is removed:
        /// 
        /// ** Check 1:** - Checks that the badge presented is authorized to manage the fund.
        /// 
        /// ** Check 2:** - Checks that the fund contains the tokens requested to supply liquidity.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `tracking_tokens_address` (ResourceAddress) - The ResourceAddress of the LP Tokens.
        /// * `tracking_tokens_amount` (Decimal) - The amount of tracking tokens to redeem.
        pub fn remove_liquidity(
            &mut self,
            fund_admin: Proof,
            tracking_tokens_address: ResourceAddress,
            tracking_tokens_amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Trading feature has not been integrated. You must first integrate RaDEX",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&tracking_tokens_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );     

            let tracking_tokens: Bucket = self.withdraw(tracking_tokens_address, tracking_tokens_amount);

            let radex: RaDEX = self.radex_address.unwrap().into();

            let (token1, token2): (Bucket, Bucket) = radex.remove_liquidity(tracking_tokens);

            self.fund_vaults.get_mut(&token1.resource_address()).unwrap().put(token1);
            self.fund_vaults.get_mut(&token2.resource_address()).unwrap().put(token2);

        }

        fn withdraw(
            &mut self,
            token_address: ResourceAddress,
            amount: Decimal
        ) -> Bucket 
        {
            // Performing the checks to ensure tha the withdraw can actually go through
            // self.assert_belongs_to_pool(resource_address, String::from("Withdraw"));
            
            // Getting the vault of that resource and checking if there is enough liquidity to perform the withdraw.
            let fund_vault: &mut Vault = self.fund_vaults.get_mut(&token_address).unwrap();
            assert!(
                fund_vault.amount() >= amount,
                "[Withdraw]: Not enough liquidity available for the withdraw."
            );

            return fund_vault.take(amount);
        }

        pub fn swap(
            &mut self,
            fund_admin: Proof,
            token_address: ResourceAddress,
            amount: Decimal,
            output_token: ResourceAddress,
        ) -> Bucket
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Trading feature has not been integrated. You must first integrate RaDEX",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&token_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let input_bucket: Bucket = self.withdraw(token_address, amount);
            let radex: RaDEX = self.radex_address.unwrap().into();
            let return_bucket = radex.swap(input_bucket, output_token);

            return_bucket
        }

        pub fn register_degenfi_user(
            &mut self,
            fund_admin: Proof,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let (degenfi_badge, degen_tokens): (Bucket, Bucket) = degenfi.new_user();
            self.degenfi_vaults.insert(degenfi_badge.resource_address(), Vault::with_bucket(degenfi_badge));
            self.degenfi_vaults.insert(degen_tokens.resource_address(), Vault::with_bucket(degen_tokens));
        }

        pub fn new_lending_pool(
            &mut self,
            fund_admin: Proof,
            token_address: ResourceAddress,
            deposit_amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&token_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let supply_bucket: Bucket = self.withdraw(token_address, deposit_amount);

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof = self.degenfi_vaults[&addresses[0]].create_proof();
            let degen_token: Bucket = degenfi.new_lending_pool(degenfi_badge_proof, supply_bucket);
            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        pub fn deposit_supply(
            &mut self,
            fund_admin: Proof,
            token_address: ResourceAddress,
            deposit_amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&token_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let supply_bucket: Bucket = self.withdraw(token_address, deposit_amount);

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof = self.degenfi_vaults[&addresses[0]].create_proof();
            let degen_token: Bucket = degenfi.deposit_supply(degenfi_badge_proof, supply_bucket);
            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        pub fn deposit_collateral(
            &mut self,
            fund_admin: Proof,
            collateral_address: ResourceAddress,
            collateral_amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&collateral_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let collateral_bucket: Bucket = self.withdraw(collateral_address, collateral_amount);

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof = self.degenfi_vaults[&addresses[0]].create_proof();
            let degen_token: Bucket = degenfi.deposit_collateral(degenfi_badge_proof, collateral_bucket);
            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        pub fn deposit_additional_collateral(
            &mut self,
            fund_admin: Proof,
            loan_id: NonFungibleId,
            collateral_address: ResourceAddress,
            collateral_amount: Decimal,
        )
        {

            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&collateral_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let collateral_bucket: Bucket = self.withdraw(collateral_address, collateral_amount);

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof = self.degenfi_vaults[&addresses[0]].create_proof();
            let degen_token: Bucket = 
            degenfi.deposit_additional_collateral(degenfi_badge_proof, loan_id, collateral_address, collateral_bucket);
            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        pub fn borrow(
            &mut self,
            fund_admin: Proof,
            token_requested: ResourceAddress,
            collateral_address: ResourceAddress,
            amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&collateral_address), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof = self.degenfi_vaults[&addresses[0]].create_proof();
            let (borrow_amount, loan_nft, degen_token): 
            (Bucket, Bucket, Bucket) = degenfi.borrow(
                degenfi_badge_proof, token_requested, collateral_address, amount
            );

            self.borrow_vaults.insert(borrow_amount.resource_address(), Vault::with_bucket(borrow_amount));
            self.loan_vault.get_or_insert(Vault::with_bucket(loan_nft));
            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        pub fn borrow_additional(
            &mut self,
            fund_admin: Proof,
            loan_id: NonFungibleId,
            token_requested: ResourceAddress,
            amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            assert_eq!(self.fund_vaults.contains_key(&token_requested), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof = self.degenfi_vaults[&addresses[0]].create_proof();
            let (return_borrow, degen_token): (Bucket, Bucket) = 
            degenfi.borrow_additional(degenfi_badge_proof, loan_id, token_requested, amount);

            // Entry or insert?
            self.borrow_vaults.insert(return_borrow.resource_address(), Vault::with_bucket(return_borrow));
            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        pub fn repay(
            &mut self,
            fund_admin: Proof,
            loan_id: NonFungibleId,
            token_requested: ResourceAddress,
            amount: Decimal,
        ) 
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );
            let loan_vault = self.loan_vault;

            match loan_vault {
                Some(vault) => {
                    let existing_loan_ids: BTreeSet<NonFungibleId> = vault.non_fungible_ids();
                    assert_eq!(existing_loan_ids.contains(&loan_id), true, 
                        "[{:?} Fund]: This fund does not contain the loan you have requested",
                        self.fund_name
                    );
                }
                None => {
                    info!("[{:?} Fund]: This fund does not contain any loans",
                        self.fund_name
                    );
                }
            }

            assert_eq!(self.fund_vaults.contains_key(&token_requested), true, 
                "[{:?} Fund]: This fund does not hold this asset in its vault",
                self.fund_name
            );

            let repay_amount: Bucket = self.withdraw(token_requested, amount);

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();

            let degenfi_badge_proof: Proof = self.degenfi_vaults[&addresses[0]].create_proof();

            let (return_bucket, degen_token): 
            (Bucket, Bucket) = degenfi.repay(degenfi_badge_proof, loan_id, token_requested, repay_amount);

            self.fund_vaults.get_mut(&return_bucket.resource_address()).unwrap().put(return_bucket);
            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        pub fn redeem_collateral(
            &mut self,
            fund_admin: Proof,
            collateral_address: ResourceAddress,
            amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof: Proof = self.degenfi_vaults[&addresses[0]].create_proof();

            let return_bucket: Bucket = degenfi.redeem_collateral(degenfi_badge_proof, collateral_address, amount);

            self.fund_vaults.get_mut(&return_bucket.resource_address()).unwrap().put(return_bucket);
        }

        pub fn flash_borrow(
            &mut self,
            fund_admin: Proof,
            token_requested: ResourceAddress,
            amount: Decimal,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof: Proof = self.degenfi_vaults[&addresses[0]].create_proof();

            let (return_borrow, transient_token, degen_token): (Bucket, Bucket, Bucket) = degenfi.flash_borrow(token_requested, amount);

            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        pub fn flash_repay(
            &mut self,
            fund_admin: Proof,
            repay_amount: Bucket,
            flash_loan: Bucket,
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.degenfi_address.is_some(), true, 
                "[{:?} Fund]: Leverage feature has not been integrated. You must first integrate DegenFi",
                self.fund_name
            );

            let addresses: Vec<ResourceAddress> = self.addresses();

            let degenfi: DegenFi = self.degenfi_address.unwrap().into();
            let degenfi_badge_proof: Proof = self.degenfi_vaults[&addresses[0]].create_proof();
            
            let degen_token: Bucket = degenfi.flash_repay(repay_amount, flash_loan);
        }

        pub fn view_fund_tokens(
            &self,
        )
        {
            let fund_vaults = self.fund_vaults.iter();
            for (token_address, vaults) in fund_vaults {
                info!("[{:?} Fund]: Token: {:?} | Amount: {:?}",
                self.fund_name, token_address, vaults.amount()
            );

            let price_oracle: PriceOracle = self.price_oracle_address.into();
            let token_price: Decimal = price_oracle.get_price(*token_address);
            let token_value: Decimal = token_price * vaults.amount();
            info!("[{:?} Fund]: Token: {:?} | Value: {:?}",
                self.fund_name, token_address, token_value
            );

            }
        }

        pub fn view_weights(
            &self,
        ) -> HashMap<ResourceAddress, Decimal>
        {
            return self.token_weights.clone()
        }

        pub fn view_token_weights(
            &self,
        ) -> HashMap<ResourceAddress, Decimal>
        {
            let mut token_weights: HashMap<ResourceAddress, Decimal> = HashMap::new();
            let price_oracle: PriceOracle = self.price_oracle_address.into();

            let fund_vaults = self.fund_vaults.iter();
            for (token_address, vaults) in fund_vaults {
                let token_price: Decimal = price_oracle.get_price(*token_address);
                
                let vault = self.fund_vaults.get(token_address).unwrap();

                let vault_amount = vault.amount();

                let token_value = vault_amount * token_price;

                let cumulative_value: Decimal = self.get_cumulative_value();

                let token_weight: Decimal = token_value / cumulative_value;

                token_weights.insert(*token_address, token_weight);
            }

            token_weights
        }

        fn get_cumulative_value(
            &self,
        ) -> Decimal
        {
            let mut cumulative_value: Decimal = Decimal::zero();
            let price_oracle: PriceOracle = self.price_oracle_address.into();

            let fund_vaults = self.fund_vaults.iter();
            for (token_address, vaults) in fund_vaults {
                let token_price: Decimal = price_oracle.get_price(*token_address);
                
                let vault = self.fund_vaults.get(token_address).unwrap();

                let vault_amount = vault.amount();

                let token_value = vault_amount * token_price;

                cumulative_value += token_value;
                
            }

            cumulative_value
        }

        pub fn view_loans(
            &self,
        ) -> BTreeSet<NonFungibleId>
        {
            return self.loan_vault.as_ref().unwrap().non_fungible_ids();
        }

        pub fn calculate_fees(
            &mut self
        )
        {
            let price_oracle: PriceOracle = self.price_oracle_address.into();

            let fee = self.get_cumulative_value() * self.fee_to_pool;
            let fund_token_price: Decimal = price_oracle.get_price(self.fund_token_address);
            let fund_token_amount: Decimal = fee / fund_token_price;

            let fund_tokens: Bucket = self.fund_token_admin_vault.authorize(|| 
                borrow_resource_manager!(self.fund_token_address).mint(fund_token_amount)
            );

            self.fee_vault.put(fund_tokens);
        }

        pub fn claim_fees(
            &mut self,
            fund_admin: Proof,
            amount: Decimal,
        ) -> Bucket
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            let fund_tokens: Bucket = self.withdraw(self.fund_token_address, amount);

            fund_tokens
        }
    }
}