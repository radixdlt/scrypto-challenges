use scrypto::prelude::*;
mod user;

#[blueprint]
mod yield_derivatives {

    enable_method_auth! {
        roles {
            admin => updatable_by: [admin];
        },
        methods {
            add_new_asset => restrict_to: [admin];
            create_user_and_deposit_principal => PUBLIC;
            deposit_principal => PUBLIC;
            redeem => PUBLIC;
            get_users_deposit_balance => PUBLIC;
        }
    }
    struct YieldDerivatives {
        principal_liquidity_pools: HashMap<ResourceAddress, Vault>,
        yield_liquidity_pools: HashMap<ResourceAddress, Vault>,
        principal_tokens_symbols: HashMap<ResourceAddress, String>,
        yield_tokens_symbols: HashMap<ResourceAddress, String>,
        yield_tokens: HashMap<ResourceAddress, ResourceAddress>,
        yield_rates: HashMap<ResourceAddress, Decimal>,
        total_balances: HashMap<ResourceAddress, Decimal>,
        users: HashMap<ResourceAddress, user::User>,
    }

    impl YieldDerivatives {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate() -> (ComponentAddress, FungibleBucket) {
            // Create the admin badges
            let admin_badge: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata! (
                    init {
                        "name" => "Yield Admin Badge".to_string(), locked;
                    }
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1)
                .into();

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let component = Self {
                principal_liquidity_pools: HashMap::new(),
                yield_liquidity_pools: HashMap::new(),
                principal_tokens_symbols: HashMap::new(),
                yield_tokens_symbols: HashMap::new(),
                yield_tokens: HashMap::new(),
                yield_rates: HashMap::new(),
                total_balances: HashMap::new(),
                users: HashMap::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(
                admin_badge.resource_address()
            ))))
            .roles(roles!(
                admin => rule!(require(admin_badge.resource_address()));
            ))
            .metadata(metadata! (
                roles {
                    metadata_setter => rule!(allow_all);
                    metadata_setter_updater => rule!(allow_all);
                    metadata_locker => rule!(allow_all);
                    metadata_locker_updater => rule!(allow_all);
                },
                init {
                    "name" => "SRWA Yield Derivatives component".to_string(), locked;
                }
            ))
            .globalize();
            (component.address(), admin_badge)
        }

        // Adding the new asset
        pub fn add_new_asset(&mut self, asset_address: ResourceAddress, yield_rate: Decimal) {
            match self.yield_rates.get(&asset_address) {
                Some(..) => {
                    warn!("Asset `{:?}` already exists.", asset_address);
                }
                None => {
                    self.yield_rates.insert(asset_address, yield_rate);
                    info!(
                        "Added new asset `{:?}` with yield_rate of {} to the lending pool.",
                        asset_address, yield_rate
                    );

                    // Creates corresponding LP token - commented out for now, will be updated later
                    self.create_yield_token(asset_address);
                    info!("Created yield token.");
                }
            };
        }

        fn create_new_user(&mut self) -> NonFungibleBucket {
            info!("create_new_user initiated.");
            let global_address = Runtime::global_address();
            let component_address = Runtime::bech32_encode_address(global_address);
            let user_badge: NonFungibleBucket =
                ResourceBuilder::new_integer_non_fungible(OwnerRole::None)
                    .metadata(metadata! (
                        init {
                            "name" => "Yield User Badge".to_string(), locked;
                            "component" => component_address, locked;
                        }
                    ))
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
            };

            self.users.insert(user_id, user);
            user_badge
        }

        pub fn create_user_and_deposit_principal(
            &mut self,
            principal: Bucket,
        ) -> NonFungibleBucket {
            let principal_address = principal.resource_address();
            let user_badge = self.create_new_user();
            let user_badge_resource_address = user_badge.resource_address();

            let yield_rate = match self.yield_rates.get(&principal_address) {
                Some(&x) => x,
                None => Decimal::from(0),
            };
            if yield_rate == Decimal::ZERO {
                panic!("The deposited resource is not the accepted principal token.");
            }
            let principal_amount = principal.amount();
            let yield_amount = yield_rate * principal_amount;
            let mut user = self.get_user(user_badge_resource_address);
            user.on_deposit(principal_address, principal_amount, yield_amount);
            self.users.insert(user_badge_resource_address, user);

            //update total principal and yield balance
            self.update_balances(principal_address, principal_amount, yield_amount);

            //put principal tokens to vault
            match self.principal_liquidity_pools.get_mut(&principal_address) {
                Some(x) => x.put(principal),
                None => {
                    let vault = Vault::with_bucket(principal);
                    self.principal_liquidity_pools
                        .insert(principal_address, vault);
                }
            };
            user_badge
        }

        pub fn deposit_principal(&mut self, principal: Bucket, user_badge: Proof) {
            let principal_address = principal.resource_address();
            let yield_rate = match self.yield_rates.get(&principal_address) {
                Some(&x) => x,
                None => Decimal::from(0),
            };
            if yield_rate == Decimal::ZERO {
                panic!("The deposited resource is not the accepted principal token.");
            }

            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.get_user(user_badge_resource_address);
            let user_principal_balance = match user.deposit_balances.get_mut(&principal_address) {
                Some(current_balance) => current_balance.principal_balance,
                None => Decimal::ZERO,
            };
            if user_principal_balance != Decimal::ZERO {
                panic!("Stake is available if the user does not have an active staking deposit.");
            }
            let principal_amount = principal.amount();

            let yield_amount = yield_rate * principal_amount;
            user.on_deposit(principal_address, principal_amount, yield_amount);
            self.users.insert(user_badge_resource_address, user);

            //update total principal and yield balance
            self.update_balances(principal_address, principal_amount, yield_amount);

            match self.principal_liquidity_pools.get_mut(&principal_address) {
                Some(x) => x.put(principal),
                None => {
                    let vault = Vault::with_bucket(principal);
                    self.principal_liquidity_pools
                        .insert(principal_address, vault);
                }
            };
        }

        pub fn redeem(
            &mut self,
            principal_address: ResourceAddress,
            user_badge: Proof,
        ) -> (Bucket, Bucket) {
            let user_badge_resource_address = user_badge.resource_address();
            let mut user = self.get_user(user_badge_resource_address);
            let principal_balance = match user.deposit_balances.get_mut(&principal_address) {
                Some(current_balance) => current_balance.principal_balance,
                None => Decimal::ZERO,
            };
            if principal_balance == Decimal::ZERO {
                panic!("Nothing to redeem.");
            }
            let now = Clock::current_time_rounded_to_minutes();

            let yield_balance = match user.deposit_balances.get_mut(&principal_address) {
                Some(current_balance) => current_balance.yield_balance,
                None => Decimal::ZERO,
            };

            let deposited_at = match user.deposit_balances.get_mut(&principal_address) {
                Some(current_balance) => current_balance.deposited_at,
                None => now,
            };
            let maturity_date = deposited_at.add_days(30).unwrap();
            let principal_bucket = match self.principal_liquidity_pools.get_mut(&principal_address)
            {
                Some(x) => x.take(principal_balance),
                None => Bucket::new(principal_address),
            };
            let yield_address = self.yield_tokens.get(&principal_address).unwrap().clone();

            let yield_bucket;
            user.on_redeem(principal_address);
            self.users.insert(user_badge_resource_address, user);
            if now.compare(maturity_date, TimeComparisonOperator::Gt) {
                yield_bucket = match self.yield_liquidity_pools.get_mut(&yield_address) {
                    Some(x) => x.take(yield_balance),
                    None => Bucket::new(yield_address),
                };
                info!("Matured.");
            } else {
                yield_bucket = Bucket::new(yield_address);
                info!("Not Matured.");
            }
            //update total principal and yield balance
            self.update_balances(principal_address, -principal_balance, -yield_balance);
            (principal_bucket, yield_bucket)
        }

        fn get_user(&self, user_badge_resource_address: ResourceAddress) -> user::User {
            let user = self
                .users
                .get(&user_badge_resource_address)
                .unwrap()
                .clone();
            user
        }

        fn update_balances(
            &mut self,
            principal_address: ResourceAddress,
            principal_amount: Decimal,
            yield_amount: Decimal,
        ) {
            let yield_address = self.yield_tokens.get(&principal_address).unwrap().clone();
            //update principal balance
            let mut total_principal_deposit_balance =
                match self.total_balances.get(&principal_address) {
                    Some(&x) => x,
                    None => Decimal::from(0),
                };
            total_principal_deposit_balance += principal_amount;
            self.total_balances
                .insert(principal_address, total_principal_deposit_balance);

            //update yield balance
            let mut total_yield_deposit_balance = match self.total_balances.get(&yield_address) {
                Some(&x) => x,
                None => Decimal::from(0),
            };
            total_yield_deposit_balance += yield_amount;
            self.total_balances
                .insert(yield_address, total_yield_deposit_balance);
        }

        fn create_yield_token(&mut self, asset_address: ResourceAddress) {
            let manager = ResourceManager::from(asset_address);

            // check resource manager for get_metadata
            let asset_name_option: Option<String> = manager.get_metadata("symbol").unwrap();
            let asset_name: String = asset_name_option.unwrap_or_default().to_owned();
            let mut yield_token_symbol = "yt".to_owned();

            yield_token_symbol.push_str(&asset_name);
            let yt_symbol = yield_token_symbol.clone();

            let yield_token = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata! {
                    roles {
                        metadata_locker => rule!(allow_all);
                        metadata_locker_updater => rule!(allow_all);
                        metadata_setter => rule!(allow_all);
                        metadata_setter_updater => rule!(deny_all);
                    },
                  init {

                    "name" => "Yield Token", locked;
                    "symbol" => yield_token_symbol, locked;
                  }
                })
                .mint_roles(mint_roles!(
                    minter => rule!(allow_all);
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles!(
                        burner => rule!(allow_all);
                        burner_updater => rule!(deny_all);
                ))
                .recall_roles(recall_roles!(
                    recaller => rule!(allow_all);
                    recaller_updater => rule!(deny_all);
                ))
                .mint_initial_supply(1000000);
            let yield_address = yield_token.resource_address();
            match self.yield_liquidity_pools.get_mut(&yield_address) {
                Some(x) => x.put(yield_token.into()),
                None => {
                    let vault = Vault::with_bucket(yield_token.into());
                    self.yield_liquidity_pools.insert(yield_address, vault);
                }
            };

            self.yield_tokens.insert(asset_address, yield_address);
            self.yield_tokens_symbols.insert(asset_address, yt_symbol);
            self.principal_tokens_symbols
                .insert(asset_address, asset_name);
        }

        // Retrieve a user's current deposit balance for a specific asset
        pub fn get_users_deposit_balance(
            &self,
            asset_address: ResourceAddress,
            user_badge: Proof,
        ) -> (Decimal, Decimal) {
            info!("get_resource_deposit_balance initiated.");

            // Retrieve user data
            let user_badge_resource_address = user_badge.resource_address();
            let user = self.users.get(&user_badge_resource_address).unwrap();

            // Update User Borrowed Balance
            let current_principal_balance = match user.deposit_balances.get(&asset_address) {
                Some(current_balance) => current_balance.principal_balance,
                None => Decimal::from(0),
            };
            let current_yield_balance = match user.deposit_balances.get(&asset_address) {
                Some(current_balance) => current_balance.yield_balance,
                None => Decimal::from(0),
            };
            info!(
                " Principal balance for asset {:?} and yield balance is {:?}.",
                current_principal_balance, current_yield_balance
            );
            (current_principal_balance, current_yield_balance)
        }
    }
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct UserNft {
    pub name: String,
    // #[mutable]
    // pub flag: bool,
}

impl UserNft {
    pub fn new() {
        ResourceBuilder::new_ruid_non_fungible::<UserNft>(OwnerRole::None)
            .create_with_no_initial_supply();
    }
}
