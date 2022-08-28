use price_oracle::*;
use radex::radex::*;
use scrypto::prelude::*;

const TOKEN_START_PRICE: Decimal = Decimal(10); // [USD Stablecoin]

blueprint! {
    struct InvestmentPool {
        /// HashMap to store all Vaults of the resources held by this pool.
        pool_vaults: HashMap<ResourceAddress, Vault>,
        /// Address of the pool token that represents ownership shares of the pools assets.
        pool_token_address: ResourceAddress,
        /// Badge that provides authorization to the investment_pool component to mint pool tokens
        pool_token_mint_badge: Vault,
        /// Badge that represents the rights of the fund manager
        pool_manager_badge_address: ResourceAddress, // TODO: Think about MultiAdmin/MultiManager scheme
        /// Integer
        /// Decimal value, that holds the performance fee.
        performance_fee: Decimal,
        /// Vault for minted tokens due to performance fees. Can only be emptied by the fond manager.
        performance_fee_vault: Vault,
        /// Stores the address of the price-oracle used for this InvestmentPool
        oracle: PriceOracle, // TODO: Option: Implement this via harcode --> In real life this would already exist.
        /// Stores the address of the decentralized exchange (DEX) thats being used for this pool.
        dex: RaDEX,
        /// Address of the base currency used in this pool.
        base_currency: ResourceAddress,
    }

    /// # This is an implementation of a Investment Pool.
    /// TODO Insert good documentation
    impl InvestmentPool {
        pub fn instantiate_pool(
            performance_fee: Decimal,
            oracle_address: ComponentAddress,
            dex_address: ComponentAddress,
            base_currency: ResourceAddress,
            fund_name: String,
            fund_symbol: String,
        ) -> (ComponentAddress, Bucket) {
            // Performing the checks to see if this liquidity pool may be created or not.
            // assert_ne!(
            //     token1.resource_address(), token2.resource_address(),
            //     "[Pool Creation]: Liquidity pools may only be created between two different tokens."
            // );

            // TODO Implement all necessary assertions.
            // Assert length of "symbol"
            // Assert that the given oracle address is really the oracle we'd expect.

            // At this point, we know that the pool creation can indeed go through.

            info!(
                "[Pool Creation]: Creating a new investment pool with the name: {} and symbol: {}.",
                fund_name, fund_symbol
            );

            // Create the Hash Map. This will later be used to store all Vaults with the pools assets.
            // let mut pool_vaults: HashMap<ResourceAddress, Vault> = HashMap::new();

            // Creating the admin badge of the investment pool which will be given the authority to mint and burn the
            // pools tracking tokens.
            let pool_token_mint_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Pool Token Admin Badge")
                .metadata("symbol", "PTAB")
                .metadata(
                    "description",
                    "This is an admin badge that has the authority to mint and burn pool tokens",
                )
                .initial_supply(1);

            // Creating the tracking tokens and minting the amount owed to the initial liquidity provider
            let pool_token_address: ResourceAddress = ResourceBuilder::new_fungible()
                    .divisibility(DIVISIBILITY_MAXIMUM)
                    .metadata("name", fund_name)
                    .metadata("symbol", "TT")
                    .metadata("description", "A tracking token used to track the percentage ownership of liquidity providers over the liquidity pool")
                    .mintable(rule!(require(pool_token_mint_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(pool_token_mint_badge.resource_address())), LOCKED)
                    .no_initial_supply();

            // Creating the fond manager badge of the investment pool which will be given various rights only the fund manager has.
            let pool_manager_badge: Bucket = ResourceBuilder::new_fungible()
                    .divisibility(DIVISIBILITY_NONE)
                    .metadata("name", "Pool Manager Admin Badge")
                    .metadata("symbol", "PMAB")
                    .metadata("description", "This is the badge that gives certain rights to the fund manager such as withdrawing accrued performance fees")
                    .initial_supply(Decimal::ONE);

            let access_rules: AccessRules = AccessRules::new()
                .method(
                    "fund_pool",
                    rule!(require(pool_manager_badge.resource_address())),
                )
                .method(
                    "collect_fee",
                    rule!(require(pool_manager_badge.resource_address())),
                )
                .method(
                    "swap_x_y",
                    rule!(require(pool_manager_badge.resource_address())),
                )
                .method(
                    "change_fee",
                    rule!(require(pool_manager_badge.resource_address())),
                )
                .default(rule!(allow_all));

            // Creating the liquidity pool component and instantiating it
            let investment_pool: ComponentAddress = Self {
                pool_vaults: HashMap::new(),
                pool_token_address,
                pool_token_mint_badge: Vault::with_bucket(pool_token_mint_badge),
                pool_manager_badge_address: pool_manager_badge.resource_address(),
                performance_fee,
                performance_fee_vault: Vault::new(pool_token_address),
                oracle: oracle_address.into(),
                dex: dex_address.into(),
                base_currency,
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            // TODO After finishing Access rights and some other Todos in this compnent: Make initial commit to repo.
            (investment_pool, pool_manager_badge)
        }

        /// This method can be used to fund the pool with an arbitray asset without rebalancing.
        ///
        /// This method is only callable by the pool manager.
        /// The intention of this method is mainly to initially fund the pool with arbitrary asset without triggering
        /// an automatic rebalancing via a DEX.
        ///
        /// This method fulfills three main tasks:
        /// 1. Determine the value added by the provided funds and mint the corresponding amount of pool tokens
        /// 2. Add the provided assets to the investment pool
        /// 3. Return the newly minted pool token
        pub fn fund_pool(&mut self, funds: Bucket) -> Bucket {
            assert!(
                !funds.is_empty(),
                "Can not fund this pool from an empty bucket"
            );

            // TODO: Think about how to optimize this code.
            // Determine present marketcap of the pool.
            let pool_market_cap = self.get_market_cap();
            // Determine marketcap of the provided asset.
            let provided_value = self.get_asset_price(funds.resource_address()) * funds.amount();

            // Determine the tokens to be minted. TODO Check first if the pool was empty or marketcap == 0 before! Aktuell: Division by zero!
            let tokens_to_mint: Decimal = if self.amount_pool_token() == Decimal::ZERO {
                // This is the first time this pool is funded with anything! Mint first tokens with a base NAV (net asset value) of TOKEN_START_PRICE
                provided_value / TOKEN_START_PRICE
            } else {
                (provided_value / pool_market_cap) * self.amount_pool_token()
            };

            // Add funds to the pool.
            self.deposit_to_pool(funds);

            // Add this value as a datapoint to the amount of tokens that the pool manager holds (too compute stake of the pool manager)
            // TODO

            // Return a bucket with the newly minted pool tokens.
            self.mint_pool_tokens(tokens_to_mint)
        }

        /// This method is used to collect all accrued performance fees and only callable by the pool manager.
        ///
        /// Performance fees are determined based on the principle of a high water mark.
        /// In this method it is checked whether any performance fees have accrued.
        /// If there are some they are withdrawn.
        // pub fn collect_fee(&self) -> Bucket {
        //     // Only for fund manager
        // }

        /// Perform asset swap - this is the main access point for the pool manager for trading pool assets
        ///
        /// This method is only callable by the pool manager.
        /// TODO Add argument description
        pub fn trade_assets(
            &mut self,
            asset_to_sell: ResourceAddress,
            amount_to_sell: Decimal,
            asset_to_buy: ResourceAddress
        ) {
            // All necessary assertions are made within the swap method.
            // Swap all assets.
            self.swap_assets(asset_to_sell, amount_to_sell, asset_to_buy);
        }

        // pub fn change_fee(&self) -> Bucket {
        //     // Only for fund manager
        // }

        /// Returns the total supply of pool tokens.
        pub fn amount_pool_token(&self) -> Decimal {
            let pool_tokens_manager: &ResourceManager =
                borrow_resource_manager!(self.pool_token_address);
            pool_tokens_manager.total_supply()
        }

        /// Determine the price in USD of the given token via the Oracle provided during instantiation.
        /// This only works if the token price is actually known by the oracle and otherwise aborts.
        pub fn get_asset_price(&self, token: ResourceAddress) -> Decimal {
            match self.oracle.get_price(self.base_currency, token) {
                Some(token_price) => token_price,
                None => std::process::abort(),
            }
        }

        /// Determines the present price of the pool token based on its net asset value (NAV) using the underlying price oracle.
        ///
        /// This method needs to do quite a lot of computing. Don't call this method unnecessarily often!
        ///
        /// # Returns
        ///
        /// `Decimal` - The price of the pool tracking token based on its NAV.
        /// `Decimal` - Total value of all assets in the investment pool.
        pub fn pool_token_price_marketcap(&self) -> (Decimal, Decimal) {
            // Assert that there are existing pool tokens.
            assert!(
                (self.amount_pool_token() > Decimal::ZERO),
                "This pool hasn't been funded yet. There are no pool tokens representing any value."
            );

            let mut total_value: Decimal = Decimal::ZERO;

            // Determine NAV --> Iterate through all assets, add up each total value TODO: We could also determine the value percentages of each asset in here.
            //  Iterate through all vaults of the investment_pool.
            for (asset_address, asset_vault) in self.pool_vaults.iter() {
                // Determine price and token-amount for each asset in the investment pool and add it up.
                total_value += self.get_asset_price(*asset_address) * asset_vault.amount();
            }

            assert!(
                total_value > Decimal::ZERO,
                "This pool hasn't been funded yet. Total asset value is ZERO."
            );

            // 3. Finally, determine token (NAV) price via dividing the total market_cap by the amount of existing tokens.
            ((total_value / self.amount_pool_token()), total_value)
        }

        /// Determines and returns the present market cap of the investment_pool based on its NAV.
        pub fn get_market_cap(&self) -> Decimal {
            let (_, marketcap) = self.pool_token_price_marketcap();
            marketcap
        }

        /// Method that mints the given amount of pool tokens.
        fn mint_pool_tokens(&self, amnt_tokens_to_mint: Decimal) -> Bucket {
            assert!(
                amnt_tokens_to_mint > Decimal::ZERO,
                "The given amount of tokens to mint is <= ZERO."
            );

            let pool_tokens_manager: &ResourceManager =
                borrow_resource_manager!(self.pool_token_address);
            self.pool_token_mint_badge
                .authorize(|| pool_tokens_manager.mint(amnt_tokens_to_mint)) // Returns bucket with new pool tokens.
        }

        /// Method that deposits assets to the pool.
        fn deposit_to_pool(&mut self, bucket: Bucket) {
            // Assert whether bucket is empty.
            assert!(!bucket.is_empty(), "Bucket is empty.");

            // // 1. Check whether the asset is already in the pool.
            // if self.pool_vaults.contains_key(&bucket.resource_address()){
            //  // 2. If yes, add to the vault
            // self.pool_vaults.get_mut(&bucket.resource_address()).unwrap().put(bucket);
            // }else {
            //     // 3. If no, add new vault with the provided asset.
            //     self.pool_vaults.insert(bucket.resource_address(), Vault::with_bucket(bucket));
            // }

            // 1. Check whether the asset is already in the pool.
            match self.pool_vaults.get_mut(&bucket.resource_address()) {
                // 2. If yes, add to the vault
                Some(asset_vault) => {
                    asset_vault.put(bucket);
                    info!("Added the given asset to the existing vault in the investment pool.");
                }
                // 3. If no, add new vault with the provided asset.
                None => {
                    self.pool_vaults
                        .insert(bucket.resource_address(), Vault::with_bucket(bucket));
                    info!("Opened a new vault in the investment pool for the given asset.");
                }
            }
        }

        /// This method handles all asset swaps that are performed within the investment_pool component.
        ///
        /// This method only completes successfully if the asset_to_sell exists in its amount_to_sell in this pool.
        /// Note: at the present version, there is no optimization for slippage. Assets are just swapped "blindly" on the DEX.
        fn swap_assets(
            &mut self,
            asset_to_sell: ResourceAddress,
            amount_to_sell: Decimal,
            asset_to_buy: ResourceAddress,
        ){
            // Check whether the asset_to_sell exists in the pool and whether the quantity is sufficient.
            // TODO assert whether the asset_to_sell and assets_to_buy actually exist.
            assert!((amount_to_sell > Decimal::ZERO), "The given amount_to_sell is <= zero. This transaction can't be processed.");
            assert!(self.pool_vaults.contains_key(&asset_to_sell), "The asset_to_sell doesn't exist in this pool.");
            assert!((self.pool_vaults[&asset_to_sell].amount() >= amount_to_sell), "The asset_to_sell doesn't exist in a sufficient quantity.");
            // Check whether there is a liquidity pool on the dex for our token pair.
            assert!(self.dex.pool_exists(asset_to_sell, asset_to_buy), "No liquidity pool exists for the given address pair.");

            // Take the asset_to_sell out of its vault and swap it on the DEX against the asset_to_buy
            // unwrap() can be used here, because it was already checked whether the key exists in the hashmap.
            let vault_to_sell: &mut Vault = self.pool_vaults.get_mut(&asset_to_sell).unwrap();
            let bucket_to_deposit: Bucket = self.dex.swap(vault_to_sell.take(amount_to_sell), asset_to_buy);

            // Deposit the swapped asset to the pool.
            self.deposit_to_pool(bucket_to_deposit);

            // TODO Add an "info!"" here which resources where swapped successfully.
        }
    }
}
