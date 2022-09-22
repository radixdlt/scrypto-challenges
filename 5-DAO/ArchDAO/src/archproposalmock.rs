//! This is a mock investment vehicle used for testing purposes. It
//! can be instructive to how an investment vehicle may be
//! implemented.
//!
//! If this file is removed from the project the test suite will not
//! be able to run successfully, but it will not affect the correct
//! running of the Radfolio itself.
//!
//! Note that the main cheat in this mock implementation is that we
//! are provided a treasury in our instantiate function from which to
//! dole out rewards. A real investment vehicle would instead get its
//! rewards from real investment instruments out there in the world.
//!
//! This mock investment vehicle gives a fixed non-compounding
//! interest every epoch.

use scrypto::prelude::*;

blueprint! {

    struct ArchProposalMock {
        /// Percent interest per epoch.
        description: String,

        /// The latest epoch we have received interest for.
        last_epoch: u64,

        /// The interest-producing funds in this vehicle.
        money_for_execution: Vault,

        /// Interests accrued and not yet removed.
        rewards: Vault,

        /// Our "never-ending" source of funds to award interest
        /// from. A real investment vehicle wouldn't have such a
        /// thing.
        money_funded: Vault,

        /// Our container for storing received votes from voters
        votes: Vault,        

        /// The badge you need to present to call our restricted
        /// methods
        iv_control_badge: ResourceAddress,
    }

    impl ArchProposalMock {

        /// Creates a mock investment vehicle that provides a
        /// non-compounding interest per epoch.
        ///
        /// The interest is taken out of the `money_printer` passed in
        /// and when that vault runs out this mock will stop
        /// functioning.
        ///
        /// Note that if we estimate a year at 18000 epochs then a 10%
        /// annual interest is 0.0005555...% per epoch
        /// (non-compounding).
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone but note that
        /// only someone who can call us with the named
        /// `iv_control_badge` in their auth zone will be able to make
        /// good use of us.
        ///
        /// **Transaction manifest:**
        /// `rtm/mock/instantiate_interestbearing_mock.rtm`
        /// ```text
        #[doc = include_str!("../rtm/mock/instantiate_proposal_mock.rtm")]
        /// ```
        pub fn instantiate_proposal_mock(
            proposal_description: String,
            money_received: Bucket,
            vote_token_address: ResourceAddress,
            iv_control_badge: ResourceAddress) -> ComponentAddress {

            // This is a mock, so we don't aggressively check
            // preconditions. Production implementations should do so
            // however.

            Self {
                description: proposal_description,
                last_epoch: Runtime::current_epoch(),
                money_for_execution: Vault::new(money_received.resource_address()),
                rewards: Vault::new(money_received.resource_address()),
                money_funded: Vault::with_bucket(money_received),
                votes: Vault::new(vote_token_address),
                iv_control_badge,
            }.instantiate()
                .add_access_check(
                    AccessRules::new()
                        .default(rule!(require(iv_control_badge)))
                        .method("read_investment_value", rule!(allow_all))
                        .method("read_projected_value", rule!(allow_all))
                        .method("read_max_investable", rule!(allow_all))
                        .method("add_funds", rule!(allow_all))
                        .method("remove_funds", rule!(allow_all))
                        .method("add_votes", rule!(allow_all))
                        .method("remove_votes", rule!(allow_all))
                ).globalize()
        }

        /// Increases the investment of this vehicle. If the input
        /// funds exceed our maximum we return the excess.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn add_votes(&mut self, mut new_votes: Bucket) -> Option<Bucket> {
            self.votes.put(new_votes);
            None
        }

        /// Decreases the investment of this vehicle. 
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn remove_votes(&mut self, mut remove_votes: Decimal) -> Bucket {
            self.votes.take(remove_votes)
        }           

        /// Increases the investment of this vehicle. If the input
        /// funds exceed our maximum we return the excess.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn add_funds(&mut self, mut new_funds: Bucket) -> Option<Bucket> {
                self.money_funded.put(new_funds);
                None
        }

        /// Decreases the investment of this vehicle. 
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn remove_funds(&mut self, mut remove_funds: Decimal) -> Bucket {
            self.money_funded.take(remove_funds)
        }        

        /// Pulls funds out of the vehicle. We are fully liquid and so
        /// all our funds can be pulled out at any time.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn reward(&mut self, by_amount: Decimal) -> Option<Bucket> {
            Some(self.rewards.take(by_amount))
        }


        pub fn execute(&mut self, mut _new_funds: Bucket) -> Option<Bucket> {   
            None 
        }

        /// Pulls out profits. All interest generated by this mock
        /// will be put into profits so there will often be something
        /// here.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn airdrop(&mut self) -> Option<Bucket> {
            Some(self.rewards.take(100))
        }


        

    }
}
