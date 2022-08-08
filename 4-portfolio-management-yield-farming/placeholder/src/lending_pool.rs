use scrypto::prelude::*;
use crate::fundinglocker::*;

// Pool Management

blueprint! {
    struct LendingPool {
        vault: Vault,
        pool_delegate_admin_badge_address: ResourceAddress,
        pool_delegate_id: NonFungibleId,
        /// When liquidity providers provide liquidity to the liquidity pool, they are given a number of tokens that is
        /// equivalent to the percentage ownership that they have in the liquidity pool. The tracking token is the token
        /// that the liquidity providers are given when they provide liquidity to the pool and this is the resource 
        /// address of the token.
        tracking_token_address: ResourceAddress,
        /// The tracking tokens are mutable supply tokens that may be minted and burned when liquidity is supplied or 
        /// removed from the liquidity pool. This badge is the badge that has the authority to mint and burn the tokens
        /// when need be.
        tracking_token_admin_badge: Vault,
    }

    impl LendingPool {

        pub fn new(
            pool_delegate_admin_badge_address: ResourceAddress,
            pool_delegate_id: NonFungibleId,
            initial_funds: Bucket) -> (ComponentAddress, Bucket)
        {
            assert_ne!(
                borrow_resource_manager!(initial_funds.resource_address()).resource_type(), ResourceType::NonFungible,
                "[Pool Creation]: Asset must be fungible."
            );

            assert!(    
                !initial_funds.is_empty(), 
                "[Pool Creation]: Can't deposit an empty bucket."
            ); 

            // Creating the admin badge of the liquidity pool which will be given the authority to mint and burn the
            // tracking tokens issued to the liquidity providers.
            let tracking_token_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Tracking Token Admin Badge")
                .metadata("symbol", "TTAB")
                .metadata("description", "This is an admin badge that has the authority to mint and burn tracking tokens")
                .initial_supply(1);

            // Creating the tracking tokens and minting the amount owed to the initial liquidity provider
            let tracking_tokens: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "LP Tracking Token")
                .metadata("symbol", "TT")
                .metadata("description", "A tracking token used to track the percentage ownership of liquidity providers over the liquidity pool")
                .mintable(rule!(require(tracking_token_admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(tracking_token_admin_badge.resource_address())), LOCKED)
                .initial_supply(100);

            let lender_dashboard = Self {
                vault: Vault::with_bucket(initial_funds),
                pool_delegate_admin_badge_address: pool_delegate_admin_badge_address,
                pool_delegate_id: pool_delegate_id,
                tracking_token_address: tracking_tokens.resource_address(),
                tracking_token_admin_badge: Vault::with_bucket(tracking_token_admin_badge),
            }
            .instantiate()
            .globalize();

            (lender_dashboard, tracking_tokens)
        }

        pub fn supply_liquidity(
            &mut self,
            liquidity_amount: Bucket) -> Bucket
        {
            // Checking if the token belong to this liquidity pool.
            assert_eq!(
                liquidity_amount.resource_address(), self.vault.resource_address(),
                "Placeholder"
            );

            // Checking that the bucket passed is not empty
            assert!(
                !liquidity_amount.is_empty(), 
                "[Add Liquidity]: Can not add liquidity from an empty bucket"
            );

            let amount = liquidity_amount.amount();

            let m: Decimal = self.vault.amount();

            // Computing the amount of tracking tokens that the liquidity provider is owed and minting them. In the case
            // that the liquidity pool has been completely emptied out (tracking_tokens_manager.total_supply() == 0)  
            // then the first person to supply liquidity back into the pool again would be given 100 tracking tokens.
            let tracking_tokens_manager: &ResourceManager = borrow_resource_manager!(self.tracking_token_address);
            let tracking_amount: Decimal = if tracking_tokens_manager.total_supply() == Decimal::zero() { 
                dec!("100.00") 
            } else {
                amount * tracking_tokens_manager.total_supply() / m
            };
            let tracking_tokens: Bucket = self.tracking_token_admin_badge.authorize(|| {
                tracking_tokens_manager.mint(tracking_amount)
            });
            info!("[Add Liquidity]: Owed amount of tracking tokens: {}", tracking_amount);

            tracking_tokens
        }

        // pub fn fund_loan(
        //     &mut self,
        //     pool_admin: Proof,
        //     funding_amount: Decimal,
        //     funding_terms: Bucket) -> ComponentAddress
        // {
        //     assert_eq!(
        //         pool_admin.resource_address(), self.pool_delegate_admin_badge_address,
        //         "[Lending Pool]: Incorrect proof passed."
        //     );

        //     assert!(
        //         self.vault.amount() >= funding_amount,
        //         "[Lending Pool]: Not enough liquidity available for withdraw."
        //     );

        //     let funding: Bucket = self.vault.take(funding_amount);

        //     let loan_factory: ComponentAddress = FundingLocker::new(funding_terms, funding);

        //     loan_factory
        // }
    }
}