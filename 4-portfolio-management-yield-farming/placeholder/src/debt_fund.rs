use scrypto::prelude::*;

use crate::fundinglocker::*;

// Pool Management

blueprint! {
    struct DebtFund {
        vault: Vault,
        debt_fund_manager_badge_address: ResourceAddress,
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
        funding_locker_address: Option<ComponentAddress>,
        access_badge_vault: Option<Vault>,
    }

    impl DebtFund {

        pub fn new(
            fund_manager_name: String,
            pool_delegate_admin_badge_address: ResourceAddress,
            pool_delegate_id: NonFungibleId,
            initial_funds: Bucket
        ) -> (ComponentAddress, Bucket, Bucket)
        {
            assert_ne!(
                borrow_resource_manager!(initial_funds.resource_address()).resource_type(), ResourceType::NonFungible,
                "[Debt Fund Creation]: Asset must be fungible."
            );

            assert!(    
                !initial_funds.is_empty(), 
                "[Debt Fund Creation]: Can't deposit an empty bucket."
            ); 

            let debt_fund_manager_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("Admin Badge for {}'s {} Debt Fund", 
                    fund_manager_name, initial_funds.resource_address())
                )
                .metadata("symbol", "FO")
                .metadata("description", "Badge that represents admin authority of the fund.")
                .initial_supply(1);

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
                debt_fund_manager_badge_address: debt_fund_manager_badge.resource_address(),
                vault: Vault::with_bucket(initial_funds),
                pool_delegate_admin_badge_address: pool_delegate_admin_badge_address,
                pool_delegate_id: pool_delegate_id,
                tracking_token_address: tracking_tokens.resource_address(),
                tracking_token_admin_badge: Vault::with_bucket(tracking_token_admin_badge),
                funding_locker_address: None,
                access_badge_vault: None,
            }
            .instantiate()
            .globalize();

            (lender_dashboard, tracking_tokens, debt_fund_manager_badge)
        }

        pub fn supply_liquidity(
            &mut self,
            liquidity_amount: Bucket
        ) -> Bucket
        {
            // Checking if the token belong to this liquidity pool.
            assert_eq!(
                liquidity_amount.resource_address(), self.vault.resource_address(),
                "[Debt Fund - Add Liquidity]: The bucket contains the wrong tokens."
            );

            // Checking that the bucket passed is not empty
            assert!(
                !liquidity_amount.is_empty(), 
                "[Debt Fund - Add Liquidity]: Can not add liquidity from an empty bucket"
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

        fn withdraw(
            &mut self,
            token_address: ResourceAddress,
            amount: Decimal
        ) -> Bucket 
        {
            // Performing the checks to ensure tha the withdraw can actually go through
            assert_eq!(
                token_address, self.vault.resource_address(),
                "[Debt Fund - Withdraw]: The Debt Fund vault does not contain this ResourceAddress"
            );
            
            // Getting the vault of that resource and checking if there is enough liquidity to perform the withdraw.
            assert!(
                self.vault.amount() >= amount,
                "[Debt Fund - Withdraw]: Not enough liquidity available for the withdraw."
            );

            return self.vault.take(amount);
        }

        /// Removes the percentage of the liquidity owed to this liquidity provider.
        /// 
        /// This method is used to calculate the amount of tokens owed to the liquidity provider and take them out of
        /// the liquidity pool and return them to the liquidity provider. If the liquidity provider wishes to only take
        /// out a portion of their liquidity instead of their total liquidity they can provide a `tracking_tokens` 
        /// bucket that does not contain all of their tracking tokens (example: if they want to withdraw 50% of their
        /// liquidity, they can put 50% of their tracking tokens into the `tracking_tokens` bucket.). When the liquidity
        /// provider is given the tokens that they are owed, the tracking tokens are burned.
        /// 
        /// This method performs a number of checks before liquidity removed from the pool:
        /// 
        /// * **Check 1:** Checks to ensure that the tracking tokens passed do indeed belong to this liquidity pool.
        /// 
        /// # Arguments:
        /// 
        /// * `tracking_tokens` (Bucket) - A bucket of the tracking tokens that the liquidity provider wishes to 
        /// exchange for their share of the liquidity.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A Bucket of the share of the liquidity provider of the first token.
        /// * `Bucket` - A Bucket of the share of the liquidity provider of the second token.
        pub fn remove_liquidity(
            &mut self,
            tracking_tokens: Bucket
        ) -> Bucket
        {
            // Checking the resource address of the tracking tokens passed to ensure that they do indeed belong to this
            // liquidity pool.
            assert_eq!(
                tracking_tokens.resource_address(), self.tracking_token_address,
                "[Debt Fund - Remove Liquidity]: The tracking tokens given do not belong to this liquidity pool."
            );

            // Calculating the percentage ownership that the tracking tokens amount corresponds to
            let tracking_tokens_manager: &ResourceManager = borrow_resource_manager!(self.tracking_token_address);
            let percentage: Decimal = tracking_tokens.amount() / tracking_tokens_manager.total_supply();

            // Burning the tracking tokens
            self.tracking_token_admin_badge.authorize(|| {
                tracking_tokens.burn();
            });

            let bucket: Bucket = self.withdraw(self.vault.resource_address(), self.vault.amount() * percentage);

            bucket
        }

        pub fn fund_loan(
            &mut self,
            token_address: ResourceAddress,
            amount: Decimal,
            funding_locker_address: ComponentAddress,
            debt_fund_address: ComponentAddress,
        )
        {
            let bucket: Bucket = self.withdraw(token_address, amount);

            self.funding_locker_address = Some(funding_locker_address);
            let funding_locker: FundingLocker = funding_locker_address.into();
            funding_locker.fund_loan(debt_fund_address, bucket);
        }

        /// Think about Access Rule
        /// Perhaps fund_loan method can provide access badge to fund locker
        pub fn deposit_access_badge(
            &mut self,
            access_badge: Bucket
        )
        {
            self.access_badge_vault = Some(Vault::with_bucket(access_badge));
        }

        pub fn claim_fees(
            &mut self,
            tracking_tokens: Proof,
        ) -> Option<Bucket>
        {
            // Checking the resource address of the tracking tokens passed to ensure that they do indeed belong to this
            // liquidity pool.
            assert_eq!(
                tracking_tokens.resource_address(), self.tracking_token_address,
                "[Debt Fund - Claim Fees]: The tracking tokens given do not belong to this liquidity pool."
            );

            // Calculating the percentage ownership that the tracking tokens amount corresponds to
            let tracking_tokens_manager: &ResourceManager = borrow_resource_manager!(self.tracking_token_address);
            let percentage: Decimal = tracking_tokens.amount() / tracking_tokens_manager.total_supply();

            let optional_funding_locker: Option<ComponentAddress> = self.funding_locker_address;
            match optional_funding_locker {
                Some(funding_locker) => {
                    let funding_locker: FundingLocker = funding_locker.into();
                    let fee_bucket: Bucket = funding_locker.claim_fees(percentage);
                    return Some(fee_bucket)
                }
                None => {
                    info!(
                        "[Debt Fund - Claim Fees]: This Debt Fund has not funded any loan opportunities yet."
                    );

                    return None
                }
            }
        }
    }
}