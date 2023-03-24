use scrypto::prelude::*;

mod calculations;
mod lp_token;
mod user;

#[blueprint]
mod lending {
    struct Lend {
        // The liquidity pool
        liquidity_pool: HashMap<ResourceAddress, Vault>,
        // The minimum collateral ratio for a user
        min_collateral_ratio: Decimal,
        // The max percent of liquidity pool user can borrow
        max_borrow_percent: Decimal,
        // The max percent of debt user can liquidate
        max_liquidation_percent: Decimal,
        // Liquidation bonus
        liquidation_bonus: Decimal,
        // User state
        users: HashMap<ResourceAddress, user::User>,
        // Loan-to-Value ratios for the assets
        asset_ltv_ratios: HashMap<ResourceAddress, Decimal>,
        // Multipliers for the assets
        asset_multipliers: HashMap<ResourceAddress, Decimal>,
        // Base multipliers for the assets
        asset_base_multipliers: HashMap<ResourceAddress, Decimal>,
        // Bases for the assets
        asset_bases: HashMap<ResourceAddress, Decimal>,
        // Reserve factors for the assets
        asset_reserve_factors: HashMap<ResourceAddress, Decimal>,
        // KINKs for the assets
        asset_kinks: HashMap<ResourceAddress, Decimal>,
        // Loan balances
        borrow_balances: HashMap<ResourceAddress, Decimal>,
        // LP tokens made for every asset
        sr_tokens: HashMap<ResourceAddress, ResourceAddress>,
        // Asset deposit interest rate
        deposit_interest_rate: Decimal,
        // Asset borrow interest rate
        borrow_interest_rate: Decimal
    }

    impl Lend {
        // Creates a lending pool
        pub fn instantiate_lending() -> (ComponentAddress, Bucket) {
            // Create the admin badges
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .mint_initial_supply(1);

            // Define the access rules for this blueprint.
            let access_rules = AccessRules::new()
                .method(
                    "add_new_asset",
                    rule!(require(admin_badge.resource_address())),
                    LOCKED,
                )
                .method(
                    "liquidate",
                    rule!(require(admin_badge.resource_address())),
                    LOCKED,
                )
                .default(rule!(allow_all), LOCKED);

            // Instattiante the component
            let mut component = Self {
                liquidity_pool: HashMap::new(),
                min_collateral_ratio: dec!("1.2"),
                max_borrow_percent: dec!("0.3"),
                max_liquidation_percent: dec!("0.5"),
                liquidation_bonus: dec!("0.05"),
                users: HashMap::new(),
                asset_multipliers: HashMap::new(),
                asset_base_multipliers: HashMap::new(),
                asset_bases: HashMap::new(),
                asset_reserve_factors: HashMap::new(),
                asset_kinks: HashMap::new(),
                asset_ltv_ratios: HashMap::new(),
                borrow_balances: HashMap::new(),
                sr_tokens: HashMap::new(),
                deposit_interest_rate: Decimal::from("0"),
                borrow_interest_rate: Decimal::from("0")
            }
            .instantiate();
            component.add_access_check(access_rules);
            let component = component.globalize();

            (component, admin_badge)
        }

        // Adding the new asset
        pub fn add_new_asset(
            &mut self,
            asset_address: ResourceAddress,
            ltv_ratio: Decimal,
            asset_multiplier: Decimal,
            asset_base_multiplier: Decimal,
            asset_base: Decimal,
            asset_reserve_factor: Decimal,
            asset_kink: Decimal,
        ) {
            info!("add_new_asset initiated.");
            assert!(
                ltv_ratio >= 0.into() && ltv_ratio <= 1.into(),
                "LTV must be between 0.0 and 1.0."
            );
            assert!(
                asset_multiplier > 0.into(),
                "Asset Multiplier must be greater then 0."
            );
            assert!(
                asset_base_multiplier > asset_multiplier,
                "Asset Base Multiplier must be greater than Asset Multiplier."
            );
            assert!(
                asset_base > 0.into(),
                "Asset Base must be greater then 0."
            );
            assert!(
                asset_reserve_factor >= 0.into() && asset_reserve_factor <= 1.into(),
                "Asset Reserve Factor must be between 0.0 and 1.0."
            );
            assert!(
                asset_kink >= 0.into() && asset_kink <= 100.into(),
                "Asset kink must be between 0 and 100."
            );


            match self.asset_ltv_ratios.get(&asset_address) {
                Some(..) => {
                    warn!("Asset `{:?}` already exists.", asset_address);
                }
                None => {
                    self.asset_ltv_ratios.insert(asset_address, ltv_ratio);
                    self.asset_multipliers.insert(asset_address, asset_multiplier);
                    self.asset_base_multipliers.insert(asset_address, asset_base_multiplier);
                    self.asset_bases.insert(asset_address, asset_base);
                    self.asset_reserve_factors.insert(asset_address, asset_reserve_factor);
                    self.asset_kinks.insert(asset_address, asset_kink);

                    info!(
                        "Added new asset `{:?}` with LTV ratio of {}, multiplier of {}, base multiplier of {}, bas of {}, reserve factor of {}, kink of {} to the lending pool.",
                        asset_address, ltv_ratio, asset_multiplier, asset_base_multiplier, asset_base, asset_reserve_factor, asset_kink
                    );

                    // Creates corresponding LP token
                    self.create_sr_token(asset_address);
                    info!("Created sr token.");
                }
            };
        }

        // Registers a new user
        pub fn create_new_user(&mut self) -> Bucket {
            info!("create_new_user initiated.");

            let user_badge = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "User Badge")
                .mint_initial_supply([(
                    IntegerNonFungibleLocalId::new(1u64),
                    self::UserNft::new(),
                )]);

            info!(
                "new_user resource address `{:?}`.",
                user_badge.resource_address()
            );

            let user_id = user_badge.resource_address();

            let user = user::User {
                user_badge_resource_address: user_id,
                deposit_balances: HashMap::new(),
                borrow_balances: HashMap::new(),
            };

            self.users.insert(user_id, user);
            info!("Created new user with an id of `{:?}`.", user_id);

            user_badge
        }

        // Deposit assets for a user
        pub fn deposit(&mut self, asset: Bucket, user_badge: Proof) -> Bucket {
            info!("deposit initiated.");

            // Get user data
            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.get_user(user_badge_resource_address);

            let asset_address = asset.resource_address();
            let amount = asset.amount();

            match self.liquidity_pool.get_mut(&asset_address) {
                Some(x) => x.put(asset),
                None => {
                    let vault = Vault::with_bucket(asset);
                    self.liquidity_pool.insert(asset_address, vault);
                }
            };

            (&mut self.users).into_iter().for_each(|(_key, value)| {
                if let Some(current_balance) = value.deposit_balances.get(&asset_address) {
                    let interest = current_balance.balance * self.deposit_interest_rate * user::Deposit::deposit_time_elapsed(current_balance);
    
                    // Increase balance by the interest.
                    let new_balance = current_balance.balance + interest;
    
                    let deposit = user::Deposit {
                        balance: new_balance, 
                        last_update: Runtime::current_epoch(),
                    };
        
                    value.deposit_balances.insert(asset_address, deposit);
                }
            });


            user.on_deposit(asset_address, amount);
            self.users.insert(user_badge_resource_address, user);
            
            info!(
                "User `{:?}` is depositing `{:?}` of `{:?}`.",
                user_badge_resource_address, amount, asset_address
            );

            info!(
                "New liquidity balance for `{:?}` is `{:?}`.",
                asset_address,
                self.liquidity_pool.get(&asset_address).unwrap().amount()
            );

            // Update deposit rate
            self.update_deposit_rate(asset_address);

            // Mint sr token and return it to user
            let minted_sr_tokens = self.mint_sr_token(asset_address);

            minted_sr_tokens
        }

        // Withdraw assets for a user
        pub fn withdraw(
            &mut self,
            asset_address: ResourceAddress,
            sr_tokens: Bucket,
            user_badge: Proof,
        ) -> Bucket {
            info!("withdraw initiated.");

            //Get user data
            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.get_user(user_badge_resource_address);

            
            
            // calculate the amount
            let sr_token_total_supply = borrow_resource_manager!(asset_address).total_supply();
            let reserve_factor = self.asset_reserve_factors.get(&asset_address).unwrap().clone();
            let exchange_rate = calculations::calculate_sr_token_exchange_rate(self.liquidity_pool.get(&asset_address).unwrap().amount(), sr_token_total_supply, reserve_factor);
            let amount = sr_tokens.amount() * exchange_rate;

            // returning the interest that was made
            let interest = calculations::calculate_interest_amount(&user, asset_address, self.deposit_interest_rate);
            
            let current_pool_liquitidy_balance; 

            let withdrawn_asset = match self.liquidity_pool.get_mut(&asset_address) {
                Some(x) => {
                    current_pool_liquitidy_balance = x.amount();
                    assert!(
                        x.amount() >= amount,
                        "Liquidity pool only has `{:?}` of asset `{:?}` but `{:?}` was requested",
                        x.amount(),
                        asset_address,
                        amount
                    );
                    x.take(amount)
                }
                None => {
                    panic!("No liquidity pool of asset `{:?}` found", asset_address)
                }
            };

            info!(
                "User `{:?}` is getting `{:?}`, interest is `{:?}`.",
                user_badge_resource_address, amount, interest
            );

            info!(
                "User `{:?}` is getting `{:?}``.",
                user_badge_resource_address, amount
            );

            // Update User Deposit Balance
            info!(
                "Removing `{:?}` of `{:?}`.",
                 amount, asset_address
            );

            (&mut self.users).into_iter().for_each(|(_key, value)| {
                if let Some(current_balance) = value.deposit_balances.get(&asset_address) {
                    let interest = current_balance.balance * self.deposit_interest_rate * user::Deposit::deposit_time_elapsed(current_balance);
    
                    // Increase balance by the interest.
                    let new_balance = current_balance.balance + interest;
    
                    let deposit = user::Deposit {
                        balance: new_balance, 
                        last_update: Runtime::current_epoch(),
                    };
        
                    value.deposit_balances.insert(asset_address, deposit);
                }
            });

            let amount_to_decrease = amount - interest;

            user.on_withdraw(
                asset_address,
                amount_to_decrease,
            );
            self.users.insert(user_badge_resource_address, user);

            info!(
                "Updated liquidity balance for `{:?}` from `{:?}` to `{:?}`.",
                asset_address,
                current_pool_liquitidy_balance,
                self.liquidity_pool.get(&asset_address).unwrap().amount()
            );

            // Burn LP Tokens
            let get_lp_token = self.sr_tokens.get(&asset_address).unwrap().clone();
            borrow_resource_manager!(get_lp_token).burn(sr_tokens);

            self.update_deposit_rate(asset_address);

            //recall tokens?
            //let minted_sr_tokens = borrow_resource_manager!(sr_token_address).recall(amount);

            withdrawn_asset
        }

        // Borrow assets for a user
        pub fn borrow(
            &mut self,
            asset_address: ResourceAddress,
            amount: Decimal,
            user_badge: Proof,
        ) -> Bucket {
            info!("borrow_asset initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.get_user(user_badge_resource_address);

            let current_pool_liquitidy_balance;
            let borrowed_asset = match self.liquidity_pool.get_mut(&asset_address) {
                Some(x) => {
                    current_pool_liquitidy_balance = x.amount();

                    // Check if there is enough amount of assets that the user requested
                    assert!(
                        x.amount() >= amount,
                        "Liquidity pool only has `{:?}` of asset `{:?}` but `{:?}` was requested.",
                        x.amount(),
                        asset_address,
                        amount
                    );
                    x.take(amount)
                }
                None => {
                    panic!("No liquidity pool of asset `{:?}` found", asset_address);
                }
            };

            // Calculate the XRD value of the asset
            // In the future we'll pull this data from an oracle, righn now all assets are 1:1
            let price_in_xrd = Decimal::from(10000) / 10000;
            let borrow_amount_in_terms_of_xrd = amount * price_in_xrd;

            // Check if user has enough collateral 
            let user_available_collateral =
                calculations::calculate_available_collateral(&user, &self.asset_ltv_ratios);
            info!(
                "[borrow_asset][USER:`{:?}`] Available collateral in terms of XRD: `{:?}`",
                user_badge_resource_address, user_available_collateral
            );
            assert!(
                user_available_collateral >= borrow_amount_in_terms_of_xrd,
                "[borrow_asset][POOL] User does not have enough collateral. Requested loan with \
                value of `{:?}` XRD but only has `{:?}` XRD of available collateral.",
                borrow_amount_in_terms_of_xrd,
                user_available_collateral
            );

            // Update User Borrowed Balance
            user.on_borrow(asset_address, amount);
            self.users.insert(user_badge_resource_address, user);
            info!(
                "[borrow_asset][USER:`{:?}`] Borrowing `{:?}` of `{:?}`.",
                user_badge_resource_address, amount, asset_address
            );

            let updated_asset_amount = self.liquidity_pool.get(&asset_address).unwrap().amount();
            info!(
                "[borrow_asset][POOL] Updated liquidity balance for `{:?}` from `{:?}` to `{:?}`.",
                asset_address, current_pool_liquitidy_balance, updated_asset_amount
            );

            // Update borrow balance
          let borrow_balance = match self.borrow_balances.get(&asset_address) {
                Some(&x) => x,
                None => {
                    Decimal::from("0")
                }
            };
            let updated_borrow_ballance = borrow_balance + amount;
            self.borrow_balances.insert(asset_address, updated_borrow_ballance);

            self.update_borrow_rate(asset_address);

            borrowed_asset
        }

        // Repay a loan balance for a user
        pub fn repay(&mut self, mut repaid: Bucket, user_badge: Proof) -> Bucket {
            info!("repay_asset initiated.");

            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.get_user(user_badge_resource_address);

            let to_return_amount = user.on_repay(repaid.amount(), repaid.resource_address(), self.borrow_interest_rate);
            let to_return = repaid.take(to_return_amount);
            let asset_address = repaid.resource_address();
            //let repaid_amount = repaid.amount();

            let current_pool_liquitidy_balance;
            match self.liquidity_pool.get_mut(&repaid.resource_address()) {
                Some(x) => {
                    current_pool_liquitidy_balance = x.amount();
                    x.put(repaid);
                    let updated_asset_amount = x.amount();

                    info!(
                        "[repay_asset][POOL] Updated liquidity balance for `{:?}` from `{:?}` to `{:?}`.",
                        asset_address,
                        current_pool_liquitidy_balance,
                        updated_asset_amount
                        );
                }
                None => {
                    panic!(
                        "[repay_asset] No asset of type `{:?}` is currently in the liquidity pool",
                        asset_address
                    );
                }
            }

            self.users.insert(user_badge_resource_address, user);

            (&mut self.users).into_iter().for_each(|(_key, value)| {
                if let Some(current_balance) = value.borrow_balances.get(&asset_address) {
                    let interest = current_balance.balance * self.borrow_interest_rate * user::Borrow::borrow_time_elapsed(current_balance);
    
                    // Increase balance by the interest.
                    let new_balance = current_balance.balance + interest;
    
                    let borrow = user::Borrow {
                        balance: new_balance, 
                        last_update: Runtime::current_epoch(),
                    };
        
                    value.borrow_balances.insert(asset_address, borrow);
                }
            });

            // Update borrow rate
            self.update_borrow_rate(asset_address);

            to_return
        }

        // Liquidates one user's position, if it's under collateralized.
        pub fn liquidate(&mut self, user_id: ResourceAddress, repaid: Bucket) -> Bucket {
            // Retrieve user data

            let mut user = self.users.get(&user_id).unwrap().clone();

            // Check if the user is under collateralized
            let collateral_ratio = user.get_collateral_ratio(self.borrow_interest_rate);

            info!(
                "[liquidate][POOL] Collateral ratio is `{:?}`.",
                collateral_ratio
            );

            if let Some(ratio) = collateral_ratio {
                assert!(
                    ratio <= self.min_collateral_ratio,
                    "Liquidation not allowed."
                );
            } else {
                panic!("No borrow from the user");
            }

            // Check liquidation size
            assert!(
                repaid.amount()
                    <= user.get_asset_borrow_balance(user_id)
                        * self.max_liquidation_percent,
                "Max liquidation percent exceeded."
            );

            let liquidity_pool = match self.liquidity_pool.get_mut(&user_id) {
                Some(x) => x,
                None => {
                    panic!(
                        "[repay_asset] No asset of type `{:?}` is currently in the liquidity pool",
                        user_id
                    );
                }
            };

            // Update user state
            let to_return_amount = user.on_liquidate(repaid.amount(), self.max_liquidation_percent, self.deposit_interest_rate);
            let to_return = liquidity_pool.take(to_return_amount);

            // Commit state changes
            self.users.insert(user_id, user);
            to_return
        }

        // Get user's current borrow balance
        pub fn get_users_borrow_balance(
            &self,
            asset_address: ResourceAddress,
            user_badge: Proof,
        ) -> Decimal {
            info!("get_users_borrow_balance initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let user = self.users.get(&user_badge_resource_address).unwrap();

            // Update User Borrowed Balance
            let current_borrow_balance = user.get_asset_borrow_balance(asset_address);
            info!(
                "[get_resource_borrow_balance][USER:`{:?}`] Borrow balance for asset `{:?}` is `{:?}`.",
                user_badge_resource_address,
                asset_address,
                current_borrow_balance
            );
            current_borrow_balance
        }

        // Retrieve a user's current deposit balance for a specific asset
        pub fn get_users_deposit_balance(
            &self,
            asset_address: ResourceAddress,
            user_badge: Proof,
        ) -> Decimal {
            info!("get_resource_deposit_balance initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let user = self.users.get(&user_badge_resource_address).unwrap();

            // Update User Borrowed Balance
            let current_deposit_balance = user.get_asset_deposit_balance(asset_address);
            info!(
                "[get_resource_deposit_balance][USER:{:?}] Deposit balance for asset {:?} is {:?}.",
                user_badge_resource_address, asset_address, current_deposit_balance
            );
            current_deposit_balance
        }

        fn create_sr_token(
            &mut self,
            asset_address: ResourceAddress,
        ) {
            let manager = borrow_resource_manager!(asset_address);

            let mut sr_token_symbol = "sr".to_owned();
            let asset_name = manager
                .get_metadata(String::from("symbol"))
                .unwrap()
                .to_owned();

            sr_token_symbol.push_str(&asset_name);

            let srToken = ResourceBuilder::new_fungible()
                .metadata("name", "SRWA Token")
                .metadata("symbol", sr_token_symbol)
                .mintable(rule!(allow_all), AccessRule::DenyAll)
                .burnable(rule!(allow_all), AccessRule::DenyAll)
                .recallable(rule!(allow_all), AccessRule::DenyAll)
                .create_with_no_initial_supply();

            self.sr_tokens.insert(asset_address, srToken);
        }

        fn mint_sr_token(&self, asset_address: ResourceAddress) -> Bucket {
            let lp_token = self.sr_tokens.get(&asset_address).unwrap().clone();
            let sr_token_total_supply = borrow_resource_manager!(asset_address).total_supply();
            let reserve_factor = self.asset_reserve_factors.get(&asset_address).unwrap().clone();
            let exchange_rate = calculations::calculate_sr_token_exchange_rate(self.liquidity_pool.get(&asset_address).unwrap().amount(), sr_token_total_supply, reserve_factor);
            let amount_to_mint = calculations::calculate_sr_tokens_to_mint(self.liquidity_pool.get(&asset_address).unwrap().amount(), exchange_rate);
            let minted_sr_tokens = borrow_resource_manager!(lp_token).mint(amount_to_mint);

            info!(
                "[Minted {:?}, of {:?}.",
                amount_to_mint, lp_token
            );

            minted_sr_tokens
        }

        fn get_user(&self, user_badge_resource_address: ResourceAddress) -> user::User {
            let user = self
                .users
                .get(&user_badge_resource_address)
                .unwrap()
                .clone();
            user
        }

        fn update_deposit_rate(&mut self, asset_address: ResourceAddress) {
            let deposit_balance = self.liquidity_pool.get(&asset_address).unwrap().amount();

            let borrow_balance = match self.borrow_balances.get(&asset_address) {
                Some(&x) => x,
                None => {
                    Decimal::from("0")
                }
            };

            // Update asset deposit rate
            let utilisation = calculations::get_utilisation(deposit_balance, borrow_balance);
            let multiplier = self.asset_multipliers.get(&asset_address).unwrap().clone();
            let base_multiplier = self.asset_base_multipliers.get(&asset_address).unwrap().clone();
            let base = self.asset_bases.get(&asset_address).unwrap().clone();
            let kink = self.asset_kinks.get(&asset_address).unwrap().clone();

            let borrow_interest_rate = calculations::calculate_borrow_rate(multiplier, base_multiplier, base, kink, utilisation);
            let deposit_interest_rate = calculations::calculate_deposit_rate(borrow_interest_rate);
            info!("Deposit interest rate is {}.", deposit_interest_rate);


            let new_rate = calculations::calculate_new_rate(deposit_interest_rate);
            info!("New rate is {}.", new_rate);

            self.deposit_interest_rate = new_rate;
        }

        fn update_borrow_rate(&mut self, asset_address: ResourceAddress) {  

            let deposit_balance = self.liquidity_pool.get(&asset_address).unwrap().amount();
            let borrow_balance = match self.borrow_balances.get(&asset_address) {
                Some(&x) => x,
                None => {
                    Decimal::from("0")
                }
            };
            let utilisation = calculations::get_utilisation(deposit_balance, borrow_balance);
            let multiplier = self.asset_multipliers.get(&asset_address).unwrap().clone();
            let base_multiplier = self.asset_base_multipliers.get(&asset_address).unwrap().clone();
            let base = self.asset_bases.get(&asset_address).unwrap().clone();
            let kink = self.asset_kinks.get(&asset_address).unwrap().clone();

            let borrow_interest_rate = calculations::calculate_borrow_rate(multiplier, base_multiplier, base, kink, utilisation);

            let new_rate = calculations::calculate_new_rate(borrow_interest_rate);
            info!("New rate is {}.", new_rate);

            self.borrow_interest_rate = new_rate;
        }
    }
}

#[derive(NonFungibleData)]
pub struct UserNft {}

impl UserNft {
    pub fn new() -> Self {
        return Self {};
    }
}
