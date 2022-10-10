//! This is a mock blueprint used for testing purposes. 
//!
//! If this file is removed from the project the test suite will not
//! be able to run successfully.
//!

use scrypto::prelude::*;

blueprint! {

    struct ArchProposalMock {
        /// description
        description: String,

        /// The latest epoch.
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
        proposal_control_badge: ResourceAddress,
    }

    impl ArchProposalMock {

        /// ---
        ///
        /// **Access control:** Can be called by anyone but note that
        /// only someone who can call us with the named
        /// `proposal_control_badge` in their auth zone will be able to make
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
            proposal_control_badge: ResourceAddress) -> ComponentAddress {

            // This is a mock, so we don't check
            // preconditions. 

            let mut mock = Self {
                description: proposal_description,
                last_epoch: Runtime::current_epoch(),
                money_for_execution: Vault::new(money_received.resource_address()),
                rewards: Vault::new(money_received.resource_address()),
                money_funded: Vault::with_bucket(money_received),
                votes: Vault::new(vote_token_address),
                proposal_control_badge,
            }.instantiate();

            mock.add_access_check(
                    AccessRules::new()
                        .default(rule!(require(proposal_control_badge)))
                        .method("reward", rule!(allow_all))
                        .method("airdrop", rule!(allow_all))
                        .method("add_funds", rule!(allow_all))
                        .method("remove_funds", rule!(allow_all))
                        .method("add_votes", rule!(allow_all))
                        .method("remove_votes", rule!(allow_all))
                );
            mock.globalize()
        }

        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn add_votes(&mut self, new_votes: Bucket) -> Option<Bucket> {
            self.votes.put(new_votes);
            None
        }

        /// remove votes
        /// 
        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn remove_votes(&mut self, remove_votes: Decimal) -> Bucket {
            self.votes.take(remove_votes)
        }           

        /// add funds
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn add_funds(&mut self, new_funds: Bucket) -> Option<Bucket> {
                self.money_funded.put(new_funds);
                None
        }

        /// remove funds
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn remove_funds(&mut self, remove_funds: Decimal) -> Bucket {
            self.money_funded.take(remove_funds)
        }        

        /// reward. to be done.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn reward(&mut self, by_amount: Decimal) -> Option<Bucket> {
            Some(self.rewards.take(by_amount))
        }


        pub fn execute(&mut self, mut _new_funds: Bucket) -> Option<Bucket> {   
            None 
        }

        /// airdrop. to be done.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn airdrop(&mut self) -> Option<Bucket> {
            Some(self.rewards.take(100))
        }


        

    }
}
