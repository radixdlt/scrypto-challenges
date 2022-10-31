use scrypto::prelude::*;
use crate::epoch_box::EpochBox;

enum BinaryVoteResult {
    Yes,
    No,
    ChangeVoteSettings,
}

struct UserVerificationRequirements {
    resource_address: Option<ResourceAddress>,      // resource to use a unique user identifier
    amount: u64,                                    // amount to use a unique user identifier
    metadata: Option<Hashmap<String, _>>            // metadata to use as 
}

struct QuorumRequirements {
    resource_address: Option<ResourceAddress>,      // resource to use as quorum checking 
    amount: Decimal,                                // amount to use as quorum
}

impl QuorumRequirements {
    pub fn is_quorum(&self, amount:Decimal) -> bool {
        amount >= self.amount
    }

    pub fn is_quorum_quantity(&self, )
}

blueprint! {
    struct BinaryVoteSession {
        admin_token::Vault,                         // 
        duration::EpochBox,                         // tracks the voting period 
        quorum::QuorumRequirements,
        VoteTracker::Hashmap<(u16, String), u64>,
    }

    impl BinaryVoteSession {
        // =====================================================================
        // FUNCTIONS
        // =====================================================================
        pub fn instantiate(
            start_epoch: u64,
            end_epoch: u64,
            quorum: Option<QuorumRequirements>,
            name: Option<String>,
            description: Option<String>,
            user_requirements: Option<UniqueUserVerificationRequirements>,
            unique_user_requirements: Option<UniqueUserVerificationRequirements>,
        ) -> ComponentAddress {
            // generate transient resource buckets  to instantiate the component    
            let admin_token = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "BinaryVoteSession Admin Badge")
                .initial_supply(1);
            
            let voting_token_name   = format!("Voting Token ({})")
            let voting_token_symbol = 
            let voting_tokens_address: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", voting_token_name)
                .metadata("symbol", "VOTE")
                .mintable(rule!(require(admin_token.resource_address())), LOCKED)
                .burnable(rule!(require(admin_token.resource_address())), LOCKED)
                // .recallable(YET-TO-BE-AVAILABLE API)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();
            
            let access_rules = AccessRules::new()
                .default(rule!(allow_all));

            let mut component = Self {
                duration: EpochBox::instantiate(start_epoch, end_epoch),
                admin_token: Vault::with_bucket(admin_token),
            }
            .instantiate();

            component.add_access_check(access_rules);
            component.globalize()
        }

        // =====================================================================
        // METHODS
        // =====================================================================
        /// register as a user of the product behind the DAO
        pub fn register_as_user(&mut self, user_nft: Proof) {
            let mode = ProofValidationMode::ValidateResourceAddress()
        }

        /// register as a user with proof-of-personhood (ensure uniuqe user)
        pub fn register_as_unique_user(&mut self, veri: Proof) {
            
        }

        fn conclude_vote_session(&mut self) -> () {

        }
    }
}