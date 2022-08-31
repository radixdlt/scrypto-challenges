use scrypto::prelude::*;
use radex::radex::*;
use degenfi::degenfi::*;
use crate::price_oracle::*;

blueprint! {
    struct IndexFund {
        fund_admin_address: ResourceAddress,
        fund_admin_vault: Vault,
        fund_trader_address: ResourceAddress,
        fund_name: String,
        fund_ticker: String,
        fund_vaults: HashMap<ResourceAddress, Vault>,
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

            let fund_trader_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("{} Trader", fund_name))
                .metadata("symbol", "FT")
                .metadata("description", "Trader authority of the fund.")
                .mintable(rule!(require(fund_admin.resource_address())), LOCKED)
                .burnable(rule!(require(fund_admin.resource_address())), LOCKED)
                .no_initial_supply();
            
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
            
            // Sets the starting price of the fund tokens.
            let price_oracle: PriceOracle = price_oracle_address.into();
            price_oracle.set_price(fund_token_address, starting_share_price);

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

                info!("[Fund Creation]: Token: {:?} Weight: {:?} ",
                token_address, token_weight);
            };

            assert_eq!(cumulative_token_weight.round(1, RoundingMode::TowardsNearestAndHalfAwayFromZero), Decimal::one(), 
                "[Fund Creation]: The total weighting of collections of tokens must equal to 100%.",
            );

            let index_fund: ComponentAddress = Self {
                fund_admin_address: fund_admin.resource_address(),
                fund_admin_vault: Vault::new(fund_admin.resource_address()),
                fund_trader_address: fund_trader_address,
                fund_name: fund_name,
                fund_ticker: fund_ticker.clone(),
                fund_vaults: fund_vaults,
                starting_share_price: starting_share_price,
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

        /// Allows Fund Manager to use RaDEX in order to make trades and rebalance portfolio.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not take any arguments.
        pub fn integrate_dex(
            &mut self, 
            fund_admin: Proof,
            radex_address: ComponentAddress
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: You do not have permission to integrate a DEX.",
                self.fund_name
            );

            self.radex_address = Some(radex_address);

            info!(
                "[{:?} Fund]: RaDEx has been integrated! You may now use its controls.",
                self.fund_name
            );
        }

        /// Allows Fund Manager to use DegenFi in order to take out loans and leverage the fund.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not take any arguments. 
        pub fn integrate_lending(
            &mut self, 
            fund_admin: Proof,
            degenfi_address: ComponentAddress
        )
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: You do not have permission to integrate a lending protocol.",
                self.fund_name
            );

            self.degenfi_address = Some(degenfi_address);

            info!(
                "[{:?} Fund]: DegenFi has been integrated! You may now use its controls.",
                self.fund_name
            );
        }

        pub fn create_trader_badge(
            &mut self,
            fund_admin: Proof,
        ) -> Bucket
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            let fund_trader_badge: Bucket = self.fund_admin_vault.authorize(|| 
                borrow_resource_manager!(self.fund_trader_address).mint(1)
            );
            
            fund_trader_badge
        }

        /// Enforcing weight doesn't matter after inception of the fund.
        pub fn add_token(
            &mut self,
            token: Vec<ResourceAddress>
        )
        {
            let tokens = token.iter();
            for token_address in tokens {
                self.fund_vaults.entry(*token_address).or_insert(Vault::new(*token_address));
            }
        }

        /// This method is used to allow investors to buy a stake of the Index Fund.
        /// 
        /// # Checks: 
        /// 
        /// * **Check 1:** - Checks that the Bucket passed contains XRD.
        /// 
        /// # Arguments:
        /// 
        /// * `xrd` (Bucket) - The Bucket that contains the XRD to purchase fund tokens.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the fund tokens.
        pub fn buy(
            &mut self,
            xrd: Bucket
        )  -> Bucket
        {
            assert_eq!(xrd.resource_address(), RADIX_TOKEN,
                "[{:?} Fund]: You can only purchase Fund Tokens with XRD.",
                self.fund_name
            );

            let output_token: ResourceAddress = self.fund_token_address; 
            let radex: RaDEX = self.radex_address.unwrap().into();
            let return_bucket: Bucket = radex.swap(xrd, output_token);

            info!(
                "[{:?} Fund]: You have purchased {:?} amount of {:?}.",
                self.fund_name,
                return_bucket.amount(),
                self.fund_ticker
            );

            return_bucket
        }

        /// This method allows investors to sell their fund tokens in exchange for equivalent value in XRD.
        /// 
        /// # Checks: 
        /// 
        /// * **Check 1:** - Checks that the Bucket passed contains the fund tokens that belongs to this protocol.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_token` (Bucket) - The Bucket that contains the fund tokens.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the XRD.
        pub fn sell(
            &mut self,
            fund_token: Bucket
        ) -> Bucket
        {
            assert_eq!(fund_token.resource_address(), self.fund_token_address,
                "[{:?} Fund]: You can only sell Fund Tokens.",
                self.fund_name
            );

            let output_token = RADIX_TOKEN;
            let radex: RaDEX = self.radex_address.unwrap().into();
            let return_bucket: Bucket = radex.swap(fund_token, output_token);

            return_bucket   
        }

        /// This method allows investors who already have the underlying asset of the Index Fund and convert them to equivalent
        /// value in fund tokens.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the vector of Buckets passed contains the tokens that are supported by the Index Fund.
        /// 
        /// # Arguments:
        /// 
        /// * `tokens` (Vec<Bucket>) - The vector of Buckets that contains the underlying assets.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the fund tokens.
        pub fn issue_tokens(
            &mut self,
            mut tokens: Vec<Bucket>,
        ) -> Bucket
        {
            let bucket_of_tokens = tokens.iter();

            for tokens in bucket_of_tokens {
                let token_address: ResourceAddress = tokens.resource_address();
                assert_eq!(
                    self.fund_vaults.contains_key(&token_address), true,
                    "[{:?} Fund]: This token is not part of the fund's basket of supported asset",
                    self.fund_name
                );
            };

            // Retrieves how many bucket of tokens are being passed.
            let number_of_tokens = tokens.len();
            
            let amount_to_mint: Decimal = if self.get_vault_cumulative_value() == Decimal::zero() { 
                let amount_to_mint: Decimal = self.get_amount_to_mint(&tokens);
                amount_to_mint
            } else {
                let amount_to_mint: Decimal = self.get_amount_to_mint2(&tokens);
                amount_to_mint
            };

            info!("[{:?} Fund]: Amount of {:?} tokens issued: {:?}", 
                self.fund_name,
                self.fund_ticker,
                amount_to_mint
            );

            // * MINTS FUND TOKENS * //
            let fund_token = self.fund_token_admin_vault.authorize(|| 
                borrow_resource_manager!(self.fund_token_address).mint(amount_to_mint)
            );

            info!(
                "[{:?} Fund]: The resource address of {:?} token is: {:?}",
                self.fund_name,
                self.fund_ticker,
                fund_token.resource_address()
            );

            let mut counter = 0;
            while counter < number_of_tokens {
                // Retrieves each bucket of tokens.
                let token_buckets: Option<Bucket> = tokens.pop();

                match token_buckets {
                    Some(token) => { // If a bucket exists...

                        assert_ne!(
                            borrow_resource_manager!(token.resource_address()).resource_type(), ResourceType::NonFungible,
                            "[{:?} Fund]: Assets must be fungible.",
                            self.fund_name
                        );

                        let token_address: ResourceAddress = token.resource_address();

                        // * CALCULATES AMOUNT OF FUND TOKENS TO MINT * //
                        // Takes the weight of one of the collateral and multiplies against the total value of the tokens
                        // deposited. The total weight of each collateral should equal to 100%.

                        assert_eq!(self.fund_vaults.contains_key(&token_address), true,
                            "[{:?} Fund]: This token does not belong to this fund.",
                            self.fund_name
                        );

                        self.fund_vaults.get_mut(&token_address).unwrap().put(token);
                    
                    }
                    None => {
                        
                        info!("[{:?} Fund]: All tokens deposited!",
                        self.fund_name);

                    }
                }

                counter += 1;

            }

            fund_token
        }

        /// This method allows investors to redeem the fund tokens for the underlying asset of the given Index Fund.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Bucket passed contains the fund tokens that belongs to this Index Fund.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_tokens` (Bucket) - The Bucket that contains the fund tokens.
        /// 
        /// # Returns:
        /// 
        /// * `Vec<Bucket>` - The vector of Buckets that contains the underlying assets.
        pub fn redeem(
            &mut self,
            fund_token: Bucket,
        ) -> Vec<Bucket>
        {
            assert_eq!(fund_token.resource_address(), self.fund_token_address,
                "[{:?} Fund]: You may only redeem fund tokens that belongs to this index.",
                self.fund_name
            );

            // * VALUES THE TOTAL AMOUNT OF FUND TOKENS PASSED. * //
            let token_amount: Decimal = fund_token.amount();
            let price_oracle: PriceOracle = self.price_oracle_address.into();
            let token_price: Decimal = price_oracle.get_price(fund_token.resource_address());
            let token_value: Decimal = token_price * token_amount;

            info!(
                "[{:?} Fund]: Fund token value: {:?}.",
                self.fund_name,
                token_value
            );

            // * TAKES THE NUMBER OF COLLATERAL IN THE FUND TO BE LOOPED OVER * //
            let number_of_tokens: usize = self.fund_vaults.len();
            let mut token_addresses = self.fund_vaults.keys().cloned().collect::<Vec<ResourceAddress>>();
            let mut counter = 0;

            let mut return_collateral: Vec<Bucket> = Vec::new();

            // * LOOPS OVER EACH COLLATERAL OF THE FUND TO GET THE WEIGHT OF EACH COLLATERAL * //
            while counter < number_of_tokens {

                let collateral_token: Option<ResourceAddress> = token_addresses.pop();

                match collateral_token {
                    Some(token) => { 

                        // * RETRIEVES EACH COLLATERAL WEIGHT AND MULTIPLY AGAINST THE TOTAL VALUE OF THE TOKEN FUND * //
                        let cumulative_value: Decimal = self.get_vault_cumulative_value();
                        let price_oracle: PriceOracle = self.price_oracle_address.into();
                        let collateral_price: Decimal = price_oracle.get_price(token);
                        let collateral_amount: Decimal = self.fund_vaults.get(&token).unwrap().amount();
                        let collateral_value: Decimal = collateral_price * collateral_amount;
                        let collateral_weight: Decimal = collateral_value / cumulative_value;
                        let amount_to_return: Decimal = token_value * collateral_weight; 

                        // * PUSHES THE BUCKET OF EACH COLLATERAL TO BE RETURNED TO THE INVESTOR * //
                        let collateral_bucket: Bucket = self.fund_vaults.get_mut(&token).unwrap().take(amount_to_return);
                        return_collateral.push(collateral_bucket);

                        info!("[Redeem]: {:?} of {:?}", token, amount_to_return);

                    }
                    None => {}
                }
                counter += 1;
            }

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
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

            let (_option_bucket1, _option_bucket2, tracking_tokens): 
            (Option<Bucket>, Option<Bucket>, Bucket) = radex.add_liquidity(token1, token2);

            // Retrieves the corresponding tracking token address. If the key exist, the value is
            // returned and the bucket of tokens is deposited into the vault. If not, a vault is 
            // created with the bucket of tokens. 
            if self.fund_vaults.contains_key(&tracking_tokens.resource_address()) == true {
                self.fund_vaults.get_mut(&tracking_tokens.resource_address()).unwrap().put(tracking_tokens);
            } else {
                self.fund_vaults.insert(tracking_tokens.resource_address(), Vault::with_bucket(tracking_tokens));
            };
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

        /// Swaps the input tokens for tokens of the desired type.
        /// 
        /// This method is used to swap tokens for other tokens. This method first checks that there does exist a 
        /// liquidity pool between the input and the output tokens. If a liquidity pool is found, then the swap goes
        /// through.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that RaDEX has been integrated to the component.
        /// * **Check 3:** - Checks that the component vault has the token the Fund admin wishes to swap.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `token_address` (ResourceAddress) - The ResourceAddress the Fund admin wishes to swap.
        /// * `amount` (Decimal) - The amount of the selected token the Fund admin wishes to swap.
        /// * `output_token` (ResourceAddress) - The resource address of the token to receive from the swap.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn swap(
            &mut self,
            fund_admin: Proof,
            token_address: ResourceAddress,
            amount: Decimal,
            output_token: ResourceAddress,
        ) 
        {
            assert_eq!(fund_admin.resource_address(), self.fund_admin_address, 
                "[{:?} Fund]: Badge not authorized.",
                self.fund_name
            );

            assert_eq!(self.radex_address.is_some(), true, 
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

            self.fund_vaults.get_mut(&return_bucket.resource_address()).unwrap().put(return_bucket);

            // Add logic to view token weights after swaps are performed.
            self.view_token_weights();
        }

        /// Creates a new user for the lending protocol.
        /// 
        /// This method is used to create a new user for DegenFi. A "Soul Bound Token" (SBT) is
        /// created and sent to the user's wallet which cannot be transferred or burnt. The SBT tracks
        /// user interactions within the protocol. Its major use case is to attempt to create a borrowing
        /// track record to underwrite the user's credit worthines. The user has to submit their
        /// wallet's component address to prevent the creation of multiple SBTs. Most of the protocol's
        /// method will require users to submit a proof of their SBT in order to use the protocol. 
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// 
        /// # Arguments: 
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

        /// Creates a new lending pool with the deposited asset.
        /// 
        /// This method is used to create a new lending pool of the deposited asset.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// * **Check 3:** - Checks that the fund vault contains the token that the fund admin wishes to create a lending
        /// pool with.
        /// 
        /// # Arguments: 
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `token_address` (ResourceAddress) - The ResourceAddress of the token the Fund admin wishes to create a lending pool
        /// with.
        /// * `deposit_amount` (Decimal) - The amount of the tokens the Fund admin wishes to supply liquidity to the lending pool.
        /// 
        /// # Returns:
        /// 
        /// * This method does not return anything.
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

        /// Deposits supply of a given asset.
        /// 
        /// This method is used to add aditional liquidity to the lending pool. The user
        /// must first identify which
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// * **Check 3:** - Checks that the fund vault contains the token that the fund admin wishes to create a lending
        /// pool with.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `token_address` (ResourceAddress) - The ResourceAddress of the token the Fund admin wishes to deposit supply with.
        /// * `deposit_amount` (Decimal) - The amount of the tokens the Fund admin wishes to supply liquidity to the lending pool.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

        /// Deposits collateral of a given asset.
        /// 
        /// This method is used to add collateral of the given asset. Currently the collateral
        /// design locks up the asset. Future iterations may provide ability to redeploy collateral
        /// as supply to provide more liquidity and allows borrowers (who use their collateral)
        /// earn APY.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// * **Check 3:** - Checks that the fund vault contains the token that the fund admin wishes to create a lending
        /// pool with.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `collateral_address` (ResourceAddress) - The ResourceAddress of the token the Fund admin wishes to deposit collateral.
        /// * `collateral_amount` (Decimal) - The amount of the tokens the Fund admin wishes to supply collateral to the lending pool.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

        /// Tops off additional collateral for a given loan.
        /// 
        /// This method is used to add additionall collateral of a given loan.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// * **Check 3:** - Checks that the fund vault contains the token that the fund admin wishes to create a lending
        /// pool with.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `loan_id` (NonFungibleId) - The NFT ID of the Loan NFT.
        /// * `collateral_address` (ResourceAddress) - The ResourceAddress of the token the Fund admin wishes to deposit collateral.
        /// * `collateral_amount` (Decimal) - The amount of the tokens the Fund admin wishes to supply collateral to the lending pool.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

        /// Allows user to borrow funds from the pool.
        ///
        /// This method is used to allow users to borrow funds from the pool.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// * **Check 3:** - Checks that the fund vault contains the token that the fund admin wishes to create a lending
        /// pool with.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `token_requested` (ResourceAddress) - The asset the Fund admin wishes to borrow from the lending pool.
        /// * `collateral_address` (ResourceAddress) - The ResourceAddress of the token the Fund admin wishes to deposit collateral.
        /// * `collateral_amount` (Decimal) - The amount of the tokens the Fund admin wishes to supply collateral to the lending pool.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

        /// Allows user to top off additional funds from the pool.
        ///
        /// This method is used to allow users to borrow additional funds from the pool.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// * **Check 3:** - Checks that the fund vault contains the token that the fund admin wishes to create a lending
        /// pool with.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `loan_id` (NonFungibleId) - The NFT ID of the Loan NFT.
        /// * `token_requested` (ResourceAddress) - The asset the Fund admin wishes to borrow from the lending pool.
        /// * `amount` (Decimal) - The amount of the tokens the Fund admin wishes to borrow more of.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

        /// Repays the loan in partial or in full.
        /// 
        /// This method is used to pay down or pay off the loan.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// * **Check 3:** - Checks that the fund vault contains the token that the fund admin wishes to create a lending
        /// pool with.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `loan_id` (NonFungibleId) - The NFT ID of the Loan NFT.
        /// * `token_requested` (ResourceAddress) - The asset the Fund admin wishes to borrow from the lending pool.
        /// * `amount` (Decimal) - The amount of the tokens the Fund admin wishes to borrow more of.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

            let loan_vault = self.loan_vault.as_ref();

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

        /// Removes the collateral owed to the user.
        /// 
        /// This method is used to redeem the collateral the user deposited.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `collateral_address` (ResourceAddress) - The ResourceAddress of the token the Fund admin wishes to redeem.
        /// * `collateral_amount` (Decimal) - The amount of the tokens the Fund admin wishes to redeem.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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

        /// Allows user to perform a flash loan.
        ///
        /// This method is used to allow users to perform a flash loan. A transient token is created to record the amount
        /// that was borrowed. The transient token must be burnt for the transaction to complete. Currently, there is no
        /// fee for performing flash loans. 
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `token_requested` (ResourceAddress) - The asset the Fund admin wishes to borrow from the lending pool.
        /// * `amount` (Decimal) - The amount of the tokens the Fund admin wishes to borrow.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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
            let (_return_borrow, _transient_token, degen_token): (Bucket, Bucket, Bucket) = degenfi.flash_borrow(token_requested, amount);

            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        /// Allows user to repay the flash loan borrow.
        ///
        /// This method is used to allow users to repay their flash loan. The amount repaid must
        /// equal what was recorded in the flash loan token data structure.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund admin badge that belongs to this Index Fund.
        /// * **Check 2:** - Checks that DegenFi is integrated in this component.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_admin` (Proof) - The Proof of the fund master badge.
        /// * `repay_amount` (Bucket) - The bucket that contains the asset to be repaid.
        /// * `flash_loan` (Bucket) - The bucket that contains the flash loan.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
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
            
            let degen_token: Bucket = degenfi.flash_repay(repay_amount, flash_loan);

            self.degenfi_vaults.get_mut(&addresses[1]).unwrap().put(degen_token);
        }

        /// This method is used to return the tokens held in this Index Fund along with the amount
        /// and value of each token.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// This method does not return anything.
        // pub fn view_fund_tokens(
        //     &self,
        // )
        // {
        //     let fund_vaults = self.fund_vaults.iter();
        //     for (token_address, vaults) in fund_vaults {
        //     let price_oracle: PriceOracle = self.price_oracle_address.into();
        //     let token_price: Decimal = price_oracle.get_price(*token_address);
        //     let token_value: Decimal = token_price * vaults.amount();
        //     info!("[{:?} Fund]: Token: {:?} | Amount: {:?} | Value: {:?}",
        //         self.fund_name, token_address, vaults.amount(), token_value
        //     );

        //     }
        // }

        /// This method is used to view the current weightings of the tokens in this Index Fund.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// This method does not return anything (other than info! messages of the token weights).
        pub fn view_token_weights(
            &self,
        ) 
        {
            let price_oracle: PriceOracle = self.price_oracle_address.into();
            info!("[{:?} Fund]: The token weights are:", self.fund_name);
            let fund_vaults = self.fund_vaults.iter();
            for (token_address, vaults) in fund_vaults {

                let token_price: Decimal = price_oracle.get_price(*token_address);

                let vault_amount = vaults.amount();

                let token_value = vault_amount * token_price;

                let cumulative_value: Decimal = self.get_vault_cumulative_value();

                let token_weight: Decimal = token_value / cumulative_value;

                info!(
                    "Token Address: {:?} | Token Amount: {:?} | Token Value: {:?} | Token Weight: {:?}", 
                    token_address, 
                    vault_amount,
                    token_value,
                    token_weight
                );
            }
        }

        /// This method is used to calculate the total value of the vector of Buckets that contains
        /// the tokens passed into the Index Fund. It is used to determine the weights of the tokens passed.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The cumulative value of the tokens in this Index Fund.
        fn get_vault_cumulative_value(
            &self,
        ) -> Decimal
        {
            let mut cumulative_value: Decimal = Decimal::zero();
            let price_oracle: PriceOracle = self.price_oracle_address.into();

            let fund_vaults = self.fund_vaults.iter();
            for (token_address, _vaults) in fund_vaults {
                let token_price: Decimal = price_oracle.get_price(*token_address);
                
                let vault = self.fund_vaults.get(token_address).unwrap();

                let vault_amount = vault.amount();

                let token_value = vault_amount * token_price;

                cumulative_value += token_value;
                
            }

            cumulative_value
        }

        /// This method is used to calculate the amount of fund tokens that needs to be issued based on the value
        /// of the tokens passed.
        /// 
        /// 
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `tokens` (&Vec<Bucket>) - The vector of Buckets that contains the tokens passed.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The amount of fund tokens to be minted.
        fn get_amount_to_mint(
            &self,
            tokens: &Vec<Bucket>
        ) -> Decimal
        {
            let cumulative_value: Decimal = self.get_total_bucket_value(&tokens);

            info!("Cumulative value of the tokens passed: {:?}", cumulative_value);

            let mut cumulative_amount: Decimal = Decimal::zero();

            let mut amount_to_mint: Decimal = Decimal::zero();

            let buckets = tokens.iter();
            for token in buckets {

                let token_amount: Decimal = token.amount();
                let price_oracle: PriceOracle = self.price_oracle_address.into();
                let token_price: Decimal = price_oracle.get_price(token.resource_address());
                let token_value: Decimal = token_amount * token_price;
                let token_weight: Decimal = token_value / cumulative_value;
                let mint: Decimal = cumulative_value * token_weight;

                info!("Token Address: {:?}", token.resource_address());
                info!("Token weight: {:?}", token_weight);
                info!("Amount of tokens passed: {:?}", token_amount);
                info!("Amount to mint: {:?}", mint);

                cumulative_amount += token_amount;

                amount_to_mint += mint;
            }

            amount_to_mint
        }

        /// This method is used to calculate the amount of fund tokens that needs to be issued based on the value
        /// of the tokens passed.
        /// 
        /// 
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `tokens` (&Vec<Bucket>) - The vector of Buckets that contains the tokens passed.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The amount of fund tokens to be minted.
        fn get_amount_to_mint2(
            &self,
            tokens: &Vec<Bucket>
        ) -> Decimal
        {

            let mut cumulative_value: Decimal = self.get_vault_cumulative_value();

            info!("Cumulative value of the tokens passed: {:?}", cumulative_value);

            let mut cumulative_amount: Decimal = Decimal::zero();

            // The purpose of this iteration is to calculate the cumulative value. 
            // Which takes the cumulative value of all the tokens currently existing the vault
            // and adds all the value of each bucket (since the buckets will be deposited)
            // when they are exchanged for Fund Tokens. We retrieve the cumulative value this way
            // because we need to calculate the weights based on updated numbers. 
            let buckets = tokens.iter();
            for token in buckets {
                let token_address: ResourceAddress = token.resource_address();

                let token_amount: Decimal = token.amount();

                let price_oracle: PriceOracle = self.price_oracle_address.into();
                let token_price: Decimal = price_oracle.get_price(token_address);
                let token_value: Decimal = token_amount * token_price;

                cumulative_value += token_value;

                info!("Token Address: {:?}", token_address);
                info!("Amount of tokens passed: {:?}", token_amount);

                cumulative_amount += token_amount;
                // multiply current cumulative token weight with total bucket value
            }

            let total_bucket_value: Decimal = self.get_total_bucket_value(&tokens);

            info!("Total value of tokens passed: {:?}", total_bucket_value);

            let mut amount_to_mint: Decimal = Decimal::zero();
            let buckets = tokens.iter();


            // This is where the logic to specify amount to be minted.
            for token in buckets {
                let token_address: ResourceAddress = token.resource_address();

                // We calculate the cumulative token amount with what is existing in the vaults
                // along with what is in the buckets so that we have the current + new amounts.
                let token_amount: Decimal = token.amount();
                let vault_amount: Decimal = self.fund_vaults.get(&token_address).unwrap().amount();
                let cumulative_token_amount = token_amount + vault_amount;

                // Calculate the value of the existing tokens in the vault + the tokens in the bucket.
                let price_oracle: PriceOracle = self.price_oracle_address.into();
                let token_price: Decimal = price_oracle.get_price(token_address);
                let token_value: Decimal = cumulative_token_amount * token_price;

                // Calculate the cumulative individual token value against the new cumulative value
                // that was calculated in the previous iteration.
                let token_weight: Decimal = token_value / cumulative_value;

                // The amount to mint is calculated by the total value of all the tokens in each bucket
                // passed multiplied by the updated weights.
                let mint: Decimal = total_bucket_value * token_weight;

                info!("Token Address: {:?}", token_address);
                info!("Token weight: {:?}", token_weight);
                info!("Cumulative amount of tokens passed: {:?}", cumulative_token_amount);
                info!("Fund vault value: {:?}", token_value);
                info!("Amount to mint: {:?}", mint);

                amount_to_mint += mint;
            }

            amount_to_mint
        }

        /// This method is used to calculate the total value of the vector of Buckets that contains
        /// the tokens passed into the Index Fund. It is used to determine the weights of the tokens passed.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `tokens` (&Vec<Bucket>) - The vector of Buckets that contains the tokens passed.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The cumulative value of the tokens passed.
        fn get_total_bucket_value(
            &self,
            tokens: &Vec<Bucket>
        ) -> Decimal
        {
            let mut cumulative_value: Decimal = Decimal::zero();
            let buckets = tokens.iter();

            for token in buckets {
                let token_amount: Decimal = token.amount();
                let price_oracle: PriceOracle = self.price_oracle_address.into();
                let token_price: Decimal = price_oracle.get_price(token.resource_address());
                let token_value: Decimal = token_amount * token_price;

                cumulative_value += token_value;
            }

            cumulative_value
        }

        /// This method is used to view the loans the component has taken out.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `BTreeSet<NonFungibleId> - The sets of all the Loan NFT IDs.
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

            let fee = self.get_vault_cumulative_value() * self.fee_to_pool;
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