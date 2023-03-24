//! # Proposal Blueprint
//!
//! Defines a Proposal that can change the state of the ecosystem.

use scrypto::blueprint;

#[blueprint]
mod proposal {

    use crate::proposal_status::ProposalStatus;
    use crate::proposed_change::ProposedChange;
    use crate::utils::get_current_time;
    use crate::voter_card::VoterCard;

    pub struct Proposal {
        /// id of the proposal
        proposal_id: u64,

        /// Current status of the proposal
        proposal_status: ProposalStatus,

        /// End of the voter period of the proposal
        vote_end: i64,

        /// Number of votes casted for the proposal
        votes_for: Decimal,

        /// Number of votes casted against the proposal
        votes_against: Decimal,

        /// Number of votes that should be casted to consider the vote valid
        votes_threshold: Decimal,

        /// Changed proposed to the ecosystem state
        proposed_change: ProposedChange,

        /// Address of the VoterCard NFR
        voter_card_address: ResourceAddress,

        /// NFR that allows to change the non-fungible data of a voter card
        voter_card_updater: Vault,
    }

    impl Proposal {
        /// Instantiates a new [`PoolComponent`] and returns it.
        ///
        /// # Arguments
        /// * `proposal_id` - Id of the proposal
        /// * `vote_end` - End of the voting period of the proposal
        /// * `votes_threshold` - Number of votes that should be casted to consider the vote valid
        /// * `proposed_change` - Changed proposed to the ecosystem state
        /// * `voter_card_address` - Address of the VoterCard NFR
        /// * `voter_card_update` - NFR that allows to change the non-fungible data of a voter card
        /// * `admin_bagde` - Address of the admin badge needed for certains calls
        pub fn new(
            proposal_id: u64,
            vote_end: i64,
            votes_threshold: Decimal,
            proposed_change: ProposedChange,
            voter_card_address: ResourceAddress,
            voter_card_updater: Bucket,
            admin_badge: ResourceAddress,
        ) -> ComponentAddress {
            // Makes sure that apart for voting, only the DAO can interact with the Proposal
            let proposal_rules = AccessRules::new()
                .method("vote_for", rule!(allow_all), AccessRule::AllowAll)
                .method("vote_against", rule!(allow_all), AccessRule::AllowAll)
                .method("is_voting_stage", rule!(allow_all), AccessRule::AllowAll)
                .default(rule!(require(admin_badge)), AccessRule::DenyAll);

            let mut component = Self {
                proposal_id,
                proposal_status: ProposalStatus::VotingStage,
                vote_end,
                votes_for: Decimal::ZERO,
                votes_against: Decimal::ZERO,
                votes_threshold,
                proposed_change,
                voter_card_address,
                voter_card_updater: Vault::with_bucket(voter_card_updater),
            }
            .instantiate();

            component.add_access_check(proposal_rules);

            component.globalize()
        }

        /// Votes for the proposal with a VoterCard proof
        ///
        /// # Arguments
        /// * `voter_card_proof` - proof of a voter card
        pub fn vote_for(&mut self, voter_card_proof: Proof) {
            self.vote(voter_card_proof, true);
        }

        /// Votes against the proposal with a VoterCard proof
        ///
        /// # Arguments
        /// * `voter_card_proof` - proof of a voter card
        pub fn vote_against(&mut self, voter_card_proof: Proof) {
            self.vote(voter_card_proof, false);
        }

        /// Returns whether the current proposal is still in voting stage
        pub fn is_voting_stage(&self) -> bool {
            self.proposal_status.is_voting_stage()
        }

        /// Sends changes to make if the proposal was approved
        ///
        /// # Acces Rule
        /// Can ony be called by the DAO
        pub fn execute(&mut self) -> Option<ProposedChange> {
            let current_time = get_current_time();
            assert!(current_time >= self.vote_end, "Vote has not finished yet!");

            if self.votes_for + self.votes_against >= self.votes_threshold {
                if self.votes_for > self.votes_against {
                    self.proposal_status = ProposalStatus::Accepted;
                    return Some(self.proposed_change.clone());
                } else {
                    self.proposal_status = ProposalStatus::Rejected;
                    return None;
                }
            } else {
                self.proposal_status = ProposalStatus::NotEnoughVotes;
                return None;
            }
        }

        /// Internal function to cast a vote
        fn vote(&mut self, voter_card_proof: Proof, vote_for: bool) {
            let current_time = get_current_time();
            assert!(
                current_time <= self.vote_end,
                "Cannot vote for this proposal anymore!"
            );

            let validated_proof = voter_card_proof
                .validate_proof(ProofValidationMode::ValidateResourceAddress(
                    self.voter_card_address,
                ))
                .expect("Please provide a valid proof of your voter card(s)");

            let mut voting_power = Decimal::ZERO;
            for voter_card in validated_proof.non_fungibles::<VoterCard>() {
                let mut data: VoterCard = borrow_resource_manager!(self.voter_card_address)
                    .get_non_fungible_data(voter_card.local_id());
                let did_not_contained = data.add_proposals_to_voted(self.proposal_id);

                if did_not_contained {
                    voting_power += data.voting_power;
                    self.voter_card_updater.authorize(|| {
                        borrow_resource_manager!(self.voter_card_address)
                            .update_non_fungible_data(voter_card.local_id(), data)
                    });
                }
            }

            if vote_for {
                self.votes_for += voting_power
            } else {
                self.votes_against += voting_power
            };
        }
    }
}
