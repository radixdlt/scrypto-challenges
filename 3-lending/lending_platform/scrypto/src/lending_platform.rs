use scrypto::prelude::*;

mod user;
mod user_nft;
mod calculations;


blueprint! {
    struct LendingPlatform {
        assets: LazyMap<ResourceAddress, Vault>,
        asset_ltv_ratios: HashMap<ResourceAddress, Decimal>,
        loan_balances: LazyMap<ResourceAddress, Decimal>,
        users: LazyMap<ResourceAddress, user::User>,
    }

    impl LendingPlatform {

        /// Instantiate instance of LendingPlatform blueprint
        pub fn instantiate_lending_platform() -> (ComponentAddress, Bucket) {
            // Create the admin badges
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Lending Platform Admin Badge")
                .initial_supply(1);

            // Define the access rules for this blueprint.
            let access_rules = AccessRules::new()
                .method("new_asset", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            // Initialize our component, placing the minting authority badge within its vault, where it will remain forever
            let component = Self {
                assets: LazyMap::new(),
                asset_ltv_ratios: HashMap::new(),
                loan_balances: LazyMap::new(),
                users: LazyMap::new(),
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            // Return the instantiated component and the admin badge we just minted
            (component, admin_badge)
        }

        pub fn new_asset(
            &mut self,
            asset_address: ResourceAddress,
            ltv_ratio: Decimal
        ) {
            info!("[LendingPlatform][new_asset] Function initiated.");
            assert! (
                ltv_ratio >= 0.into() && ltv_ratio <= 1.into(),
                "[LendingPlatform][new_asset][POOL] LTV Ratio must be between 0.0 & 1.0 (inclusive)."
                );

            match self.asset_ltv_ratios.get(&asset_address) {
                Some(..) =>  {
                    panic!("[LendingPlatform][new_asset][POOL] Asset {} already exists in lending pool.", asset_address);
                },
                None => {
                    self.asset_ltv_ratios.insert(asset_address, ltv_ratio);
                    info!(
                        "[LendingPlatform][new_asset][POOL] Added asset {} with LTV ratio of {} to the lending pool.",
                        asset_address,
                        ltv_ratio
                    )
                }
            };
        }

        /// Registers a new user
        pub fn new_user(&self) -> Bucket {
            info!("[LendingPlatform][new_user] Function initiated.");


            let non_fungible_id: NonFungibleId = NonFungibleId::random();

            let user_badge = ResourceBuilder::new_non_fungible()
                // .metadata("name", "Lending Platform User Badge")
                .initial_supply([
                    (
                        non_fungible_id,
                        user_nft::UserNft::new(),
                    )
            ]);

            let user_id = user_badge.resource_address();

            let user = user::User {
                user_badge_resource_address: user_badge.resource_address(),
                deposit_balances: HashMap::new(),
                borrow_balances: HashMap::new()
            };
            self.users.insert(user_id, user);
            info!(
                "[LendingPlatform][new_user] Created new user with an id of {}.",
                user_id
            );
            user_badge
        }

        /// Deposit assets for a user
        pub fn deposit_asset(&self, asset: Bucket, user_badge: Proof) {
            info!("[LendingPlatform][deposit_asset] Function initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.users.get(&user_badge_resource_address).unwrap();

            let asset_address = asset.resource_address();
            let amount = asset.amount();

            match self.assets.get(&asset_address) {
                Some(mut x) =>  x.put(asset),
                None => {
                    let vault = Vault::with_bucket(asset);
                    self.assets.insert(asset_address, vault);
                }
            };

            // Update User Deposit Balance
            user.increase_deposit_balance(asset_address, amount);
            self.users.insert(user_badge_resource_address, user);
            info!(
                "[LendingPlatform][deposit_asset][USER:{}] Depositing {} of {}.",
                user_badge_resource_address,
                amount,
                asset_address
            );

            info!(
                "[LendingPlatform][deposit_asset][POOL] New liquidity balance for {} is {}.",
                asset_address,
                self.assets.get(&asset_address).unwrap().amount()
            );
        }

        /// Withdraw assets for a user
        pub fn withdraw_asset(
            &self,
            asset_address: ResourceAddress,
            amount: Decimal,
            user_badge: Proof
        ) -> Bucket {
            info!("[LendingPlatform][withdraw_asset] Function initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.users.get(&user_badge_resource_address).unwrap();

            let current_pool_liquitidy_balance;

            let withdrawn_asset = match self.assets.get(&asset_address) {
                Some(mut x) =>  {
                    current_pool_liquitidy_balance = x.amount();
                    assert! (
                        x.amount() >= amount,
                        "[LendingPlatform][withdraw_asset][POOL] Pool only has {} of asset {} but {} was requested",
                        x.amount(),
                        asset_address,
                        amount
                        );
                    x.take(amount)
                },
                None => {
                     panic!(
                        "[LendingPlatform][withdraw_asset][POOL] Cannot decrease balance of asset {}. \
                        No liquidity exists yet for it.",
                        asset_address
                    )
                }
            };

            // Update User Deposit Balance
            info!(
                "[LendingPlatform][withdraw_asset][USER:{}] Removing {} of {}.",
                user_badge_resource_address,
                amount,
                asset_address
            );
            user.decrease_deposit_balance(asset_address, amount);
            self.users.insert(user_badge_resource_address, user);

            info!(
                "[LendingPlatform][withdraw_asset][POOL] Updated liquidity balance for {} from {} to {}.",
                asset_address,
                current_pool_liquitidy_balance,
                self.assets.get(&asset_address).unwrap().amount()
            );
            withdrawn_asset
        }

        /// Borrow assets for a user
        pub fn borrow_asset(
            &self,
            asset_address: ResourceAddress,
            amount: Decimal,
            user_badge: Proof
        ) -> Bucket {
            info!("[LendingPlatform][borrow_asset] Function initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.users.get(&user_badge_resource_address).unwrap();

            let current_pool_liquitidy_balance;
            let borrowed_asset = match self.assets.get(&asset_address) {
                Some(mut x) =>  {
                    current_pool_liquitidy_balance = x.amount();
                    assert! (
                        x.amount() >= amount,
                        "[LendingPlatform][borrow_asset][POOL] Pool only has {} of asset {} but {} was requested",
                        x.amount(),
                        asset_address,
                        amount
                        );
                    x.take(amount) // This only occurs if the assertion does not fail
                },
                None => {
                    panic!(
                        "[LendingPlatform][borrow_asset][POOL] No asset of type {} are currently in the liquidity pool",
                        asset_address
                    );
                }
            };

            // Calculate the XRD value of the asset the user is attempting to borrow
            // TODO: Pull this data from an oracle - right now assuming all assets have a 1:1 ratio with the price of radix
            let cost_of_asset_in_terms_of_xrd = Decimal::from(10000)/10000;
            let borrow_amount_in_terms_of_xrd = amount * cost_of_asset_in_terms_of_xrd;

            // Check if user has enough collateral available for the loan (this takes into account LTV)
            let user_available_collateral = calculations::calculate_available_collateral(&user, &self.asset_ltv_ratios);
            info!(
                "[LendingPlatform][borrow_asset][USER:{}] Available collateral in terms of XRD: {}",
                user_badge_resource_address,
                user_available_collateral
            );
            assert! (
                user_available_collateral >= borrow_amount_in_terms_of_xrd,
                "[LendingPlatform][borrow_asset][POOL] User does not have enough collateral. Requested loan with \
                value of {} XRD but only has {} XRD of available collateral.",
                borrow_amount_in_terms_of_xrd,
                user_available_collateral
            );

            // Update User Borrowed Balance
            user.increase_borrowed_balance(asset_address, amount);
            self.users.insert(user_badge_resource_address, user);
            info!(
                "[LendingPlatform][borrow_asset][USER:{}] Borrowing {} of {}.",
                user_badge_resource_address,
                amount,
                asset_address
            );

            let updated_asset_amount = self.assets.get(&asset_address).unwrap().amount();
            info!(
                "[LendingPlatform][borrow_asset][POOL] Updated liquidity balance for {} from {} to {}.",
                asset_address,
                current_pool_liquitidy_balance,
                updated_asset_amount
            );
            borrowed_asset
        }

        /// Repay a loan balance for a user
        pub fn repay_asset(
            &self,
            asset: Bucket,
            user_badge: Proof
        ) {
            info!("[LendingPlatform][repay_asset] Function initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.users.get(&user_badge_resource_address).unwrap();

            let asset_address = asset.resource_address();
            let amount = asset.amount();

            info!(
                "[LendingPlatform][repay_asset][USER:{}] Repaying {} of {}.",
                user_badge_resource_address,
                amount,
                asset_address
            );

            let current_pool_liquitidy_balance;
            match self.assets.get(&asset_address) {
                Some(mut x) =>  {
                    current_pool_liquitidy_balance = x.amount();
                    x.put(asset);
                    let updated_asset_amount = x.amount();

                    info!(
                        "[LendingPlatform][repay_asset][POOL] Updated liquidity balance for {} from {} to {}.",
                        asset_address,
                        current_pool_liquitidy_balance,
                        updated_asset_amount
                        );
                },
                None => {
                    panic!("[LendingPlatform][repay_asset] No asset of type {} are currently in the liquidity pool", asset_address);
                }
            }

            // Update User Borrowed Balance
            let current_borrow_balance = user.get_resource_borrow_balance_value(asset_address);
            let updated_borrow_balance = user.decrease_borrowed_balance(asset_address, amount);
            self.users.insert(user_badge_resource_address, user);
            info!(
                "[LendingPlatform][repay_asset][USER:{}] Updated borrow balance for asset {} from {} to {}.",
                user_badge_resource_address,
                asset_address,
                current_borrow_balance,
                updated_borrow_balance
            );
        }

        /// Retrieve a user's current borrow balance for a specific asset
        pub fn get_resource_borrow_balance(
            &self,
            asset_address: ResourceAddress,
            user_badge: Proof
        ) -> Decimal {
            info!("[LendingPlatform][get_resource_borrow_balance] Function initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let user = self.users.get(&user_badge_resource_address).unwrap();

            // Update User Borrowed Balance
            let current_borrow_balance = user.get_resource_borrow_balance_value(asset_address);
            info!(
                "[LendingPlatform][get_resource_borrow_balance][USER:{}] Borrow balance for asset {} is {}.",
                user_badge_resource_address,
                asset_address,
                current_borrow_balance
            );
            current_borrow_balance
        }

        /// Retrieve a user's current deposit balance for a specific asset
        pub fn get_resource_deposit_balance(
            &self,
            asset_address: ResourceAddress,
            user_badge: Proof
        ) -> Decimal {
            info!("[LendingPlatform][get_resource_deposit_balance] Function initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let user = self.users.get(&user_badge_resource_address).unwrap();

            // Update User Borrowed Balance
            let current_deposit_balance = user.get_resource_deposit_balance_value(asset_address);
            info!(
                "[LendingPlatform][get_resource_deposit_balance][USER:{}] Deposit balance for asset {} is {}.",
                user_badge_resource_address,
                asset_address,
                current_deposit_balance
            );
            current_deposit_balance
        }



    }
}