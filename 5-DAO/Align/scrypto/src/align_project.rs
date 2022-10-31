/*!
This is the example blueprint to create Align token and Align DAO badge and boostrap the Align DAO smartcontract.

The DAO badge also have the authority to mint, burn the Align token.

This blueprint can also include vesting, locking, ico,... method to further decentralize Align token.

The [withdraw](AlignProject_impl::AlignProject::withdraw) method should be protected to distribute the project's token, the author left it unprotected for testing convenient.
*/

use crate::align_dao::DAOComponent;
use scrypto::prelude::*;

blueprint! {
    struct AlignProject {
        token_vault: Vault,
        dao: ComponentAddress,
    }

    impl AlignProject {

        /// The function will do the following:
        /// - Create a DAO badge
        /// - Create ALIGN fungible token which can be mint, burn by the DAO badge
        /// - Use the input params, the DAO badge and ALIGN token bucket to create new DAO component
        /// # Input
        /// - initial_supply: ALIGN token initial supply
        /// - liquidity_allocation: ALIGN token allocation percent for treasury liquidity (for the internal DEX)
        /// - stable_coin: The initial primary reserve resource allocation for treasury liquidity
        /// - oracle: Oracle component address
        /// - data_badge: Bucket contain the badge to get data from the Oracle
        /// 
        /// Other input params is same as when instantiate new [DAO component](crate::align_dao::DAO_impl::DAO::new)
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/init/init.rtm`
        /// ```text
        #[doc = include_str!("../rtm/init/init.rtm")]
        /// ```
        pub fn init_project(
            initial_supply: u64,
            liquidity_allocation: Decimal,

            // admin_badge: ResourceAddress,
            stable_coin: Bucket,
            oracle: ComponentAddress,
            data_badge: Bucket,

            swap_fee: Decimal,
            withdraw_threshold: Decimal,
            withdraw_period: u64,
            rage_withdraw_decline_multiply: Decimal,
            rage_withdraw_time_limit: u64,

            dividend: Decimal,
            slash_rate: Decimal,

            initital_commitment_rate: Decimal,
            minimum_retirement: u64,
            maximum_retirement: u64,
            commitment_grow_rate: Decimal,
            maximum_vote_rate: Decimal,
            period_length: u64,

            initial_credibility: u8,
            representative_requirement: Decimal,

            proposal_requirement: Decimal,
            proposal_quorum: Decimal,
            proposal_minimum_delay: u64,
        ) {
            let dao_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Align DAO Badge")
                .initial_supply(1);

            info!(
                "[AlignProject]: Align DAO Badge Address: {}",
                dao_badge.resource_address()
            );

            let mut token_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(18)
                .updateable_metadata(
                    rule!(require(dao_badge.resource_address())),
                    MUTABLE(rule!(require(dao_badge.resource_address()))),
                )
                .mintable(rule!(require(dao_badge.resource_address())), LOCKED)
                .burnable(rule!(require(dao_badge.resource_address())), LOCKED)
                .metadata("name", "Align DAO Share")
                .metadata("symbol", "ALIGN")
                .initial_supply(initial_supply);

            let resource = token_bucket.resource_address();

            info!("[AlignProject]: ALIGN Address: {}", resource);

            let amount = token_bucket.amount();

            let liquidity_allocation = token_bucket.take(amount * liquidity_allocation / dec!(100));

            let dao = DAOComponent::new(
                "Align".to_owned(),
                dao_badge,
                liquidity_allocation,
                stable_coin,
                swap_fee,
                withdraw_threshold,
                withdraw_period,
                rage_withdraw_decline_multiply,
                rage_withdraw_time_limit,
                dividend,
                slash_rate,
                initital_commitment_rate,
                minimum_retirement,
                maximum_retirement,
                commitment_grow_rate,
                maximum_vote_rate,
                period_length,
                initial_credibility,
                representative_requirement,
                (oracle, data_badge),
                proposal_requirement,
                proposal_quorum,
                proposal_minimum_delay,
            );

            info!("[AlignProject]: New Align DAO Component: {}", dao);

            // let rules = AccessRules::new()
            //     .default(rule!(require(admin_badge)));

            let component = Self {
                token_vault: Vault::with_bucket(token_bucket),
                dao,
            }
            .instantiate();

            // component.add_access_check(rules);
            let address = component.globalize();

            info!("[AlignProject]: New AlignProject Component: {}", address);
        }

        /// This method is for admin to distribute ALIGN token.
        /// # Input
        /// - amount: The amount that admin want to withdraw
        /// # Output
        /// The withdrawed ALIGN token bucket
        /// # Access Rule
        /// *Temporary*: Anyone can call this method.
        ///
        /// This method should be protected by access rule in practice. For testing convenient, the author left it unprotected.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/init/withdraw_align.rtm`
        /// ```text
        #[doc = include_str!("../rtm/init/withdraw_align.rtm")]
        /// ```
        pub fn withdraw(&mut self, amount: Decimal) -> Bucket {
            self.token_vault.take(amount)
        }
    }
}
