use scrypto::prelude::*;

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
        /// Decimal value, that holds the performance fee.
        performance_fee: Decimal,
        /// Vault for minted tokens due to performance fees. Can only be emptied by the fond manager.
        performance_fee_vault: Vault,
    }

    /// # This is an implementation of a Investment Pool.
    /// TODO Insert good documentation
    impl InvestmentPool {
        pub fn instantiate_pool(
            performance_fee: Decimal,
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
                .metadata("description", "This is an admin badge that has the authority to mint and burn pool tokens")
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
                .method("fund_pool", rule!(require(pool_manager_badge.resource_address())))
                .method("collect_fee", rule!(require(pool_manager_badge.resource_address())))
                .method("swap_x_y", rule!(require(pool_manager_badge.resource_address())))
                .method("change_fee", rule!(require(pool_manager_badge.resource_address())))
                .default(rule!(allow_all));

            // Creating the liquidity pool component and instantiating it
            let investment_pool: ComponentAddress = Self {
                pool_vaults: HashMap::new(),
                pool_token_address,
                pool_token_mint_badge: Vault::with_bucket(pool_token_mint_badge),
                pool_manager_badge_address: pool_manager_badge.resource_address(),
                performance_fee,
                performance_fee_vault: Vault::new(pool_token_address),
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            // TODO After finishing Access rights and some other Todos in this compnent: Make initial commit to repo.
            (investment_pool, pool_manager_badge)
        }

        // pub fn fund_pool(&self, funds: Bucket) -> Bucket{
        //     // Only for fund manager
        // }

        // pub fn collect_fee(&self) -> Bucket {
        //     // Only for fund manager
        // }

        /// Perform asset swap - this is the man access point for trading assets of the pool
        // pub fn swap_x_y(&self) -> Bucket {
        //     // Only for fund manager
        //     // Use another abstraction function for swapping funds --> better portability.
        // }

        // pub fn change_fee(&self) -> Bucket {
        //     // Only for fund manager
        // }

        pub fn get_market_cap(&self) -> Decimal{
            Decimal::ONE
        }

    }
}
