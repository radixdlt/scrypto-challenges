/*!
The blueprint implement [Proposal](Proposal_impl::Proposal) component to store vote history and manage the DAO's collective actions.

## Proposals
A proposal is created by DAO member to execute smartcontract functions/methods
through the DAO authority on the [dao badge](crate::align_dao::DAO_impl::DAO::dao_badge).

**A proposal will have 2 main components:**
- The on-chain component will be exactly the smartcontract function that they would propose on the DAO,
it could be any composed smartcontract function which would require the DAO authoriy:
Execute a change, a method on a smartcontract state that require DAO authority; withdraw treasury token on a flash loan opportunity trade;
withdraw treasury token to invest on a portfolio management smartcontract or any investment opportunity;...

- The off-chain component will clearly explain the smartcontract function and how it can benefit the DAO.

According to the [Permissioned Relative Majority & Quorum Voting Mechanism](crate::align_dao#permissioned-relative-majority--quorum-voting-mechanism),
when creating a proposal, DAO member also have to arrange a time delay for the proposal:

- The time delay must be in an appropriate range or DAO members won't have time to
thoroughly examinate it, or it would be too slow and some opportunities would be missed.
- The time delay must be longer than a predetermined minimum to prevent centralized, un-noticed malicious behaviour.

Proposal end time will be the time point after the arranged time delay.

When creating a proposal, the DAO member will also receive a [proposal badge](crate::align_dao::ProposalBadge)
which later can be used to withdraw the allocated fund to the proposal on acceptance.

### Accepted proposal
A proposal is accepted and enforced if the total voted power passed a predetermined quorum and the support
voted power greater than those against (supporting vote power > 1/2 total voted power) before the proposal end time.

There are 3 main function on an accepted proposal:
- Call methods: The accepted proposal will use the dao badge to authorize and execute the proposed "methods", which represent the DAO authoriy.
- Withdraw funds: The accepted proposal will use the dao badge to authorize and call the [dao_withdraw_by_amount](crate::align_dao::DAO_impl::DAO::dao_withdraw_by_amount)
method, withdraw fund (primary reserve resource) from [treasury](crate::treasury) and allocate it to the proposal component.
Later, the proposer with right [proposal badge](crate::align_dao::ProposalBadge) can burn the badge to receive his/her fund.
- Distribute resources: The accepted proposal will use the dao badge to authorize and call the [dao_withdraw](crate::align_dao::DAO_impl::DAO::dao_withdraw)
method, withdraw the accepted resource (fungible token beside the primary reserve) from [treasury](crate::treasury) and allocate it to the proposal component
for support voters to get their share of the resource proportional to their vote power. Later, the support voters can withdraw their share from the proposal component.

On accepted proposal, the DAO will also reward dividend to all support voters of that proposal
proportionally to their vote power. The dividend reward can be claimed from [claim_dividend](crate::align_dao::DAO_impl::DAO::claim_dividend)
method or re-accumulated into DAO member account
to get more vote power from [accumulate_dividend](crate::align_dao::DAO_impl::DAO::accumulate_dividend) method.

Representatives cannot re-accumulate vote power from dividend (include the extra dividend from [Power Taxing](crate::align_dao#from-reputation-based-voting-to-power-taxing))
since it would introduce centralization and create unfairness.

Against voters cannot receive any benefit from the accepted proposal, but on proposal execution time,
they're immediately allowed call [rage_withdraw()](crate::align_dao::DAO_impl::DAO::rage_withdraw) method for a limited time.

### Rejected proposal
A proposal is rejected after total voted power passed a predetermined quorum and the support
voted power smaller than those against (supporting vote power < 1/2 total voted power) before the proposal end time.

On rejected proposal, the DAO will slash all support voters of that proposal
proportionally to their delegated (committed) amount.

This is a extra security feature for more "unity" on the DAO and prevent
members from just vote accept on all proposals to receive dividend.

### Ignored proposal
A proposal is ignored after the total voted power cannot pass the predetermined quorum after the proposal end time.

Ignored proposal will just be removed from the DAO's proposal list.

## Functions, Methods Overview & Transaction manifests
### Function
- Function [new()](Proposal_impl::Proposal::new): Instantiate new Proposal Component (before globalized).

*No transaction manifest since the function will be called through the [new_proposal()](crate::align_dao::DAO_impl::DAO::new_proposal) 
or [replacement_proposal()](crate::align_dao::DAO_impl::DAO::replacement_proposal) method.*
### Execute method - Anyone can call
Transaction manifest is on directory `rtm/proposal/execute_proposal.rtm`.

- Method [execute()](Proposal_impl::Proposal::execute):
The method will compute the vote result, execute the proposed methods,
distribute dividend reward or slash malicious vote accordingly.=

### User pattern methods
Transaction manifests are on sub-group `rtm/proposal/user_pattern/`.

- Method [vote()](Proposal_impl::Proposal::vote):
The method is for DAO members to vote on the proposal.
- Method [take_distribution()](Proposal_impl::Proposal::take_distribution):
This method will allow proposal's voters (direct or indirect) 
to get their resource share after the distribution proposal has been accepted, 
executed and got distribution resource from the treasury.

### Proposer method
Transaction manifest is on sub-group `rtm/proposal/proposer_only/`.

- Method [withdraw_fund()](Proposal_impl::Proposal::withdraw_fund):
The method will allow the DAO member who propose this proposal 
can get the allocated fund if the proposal is accepted.

### DAO Component intra-package call only
- Method [retract_vote()](Proposal_impl::Proposal::retract_vote):
Allow the DAO member or Delegator to retract vote from the proposal.

### Read only methods
Transaction manifests are on sub-group `rtm/proposal/read_only/`.

- Method [check_distribution_resource()](Proposal_impl::Proposal::check_distribution_resource):
Read only method to get the current remain distribution resource on the proposal vault.
- Method [check_vote()](Proposal_impl::Proposal::check_vote):
Read only method to check the voter's vote data on this proposal.
- Method [vote_status()](Proposal_impl::Proposal::vote_status):
Read only method to check current vote status of this proposal.

### Internal method
- Method [current()](Proposal_impl::Proposal::current):
Internal method to get current data from the oracle, 
can only be called internally
*/

use crate::align_dao::DAOComponent;
use crate::utils::Methods;
use crate::community::CommunityComponent;
use crate::delegator::Delegator;
use crate::local_oracle::LocalOracleComponent;
use crate::member::DAOMember;
use crate::policies::{CommitmentPolicy, EconomicPolicy};
use scrypto::prelude::*;

/// Store the vote data on a proposal of a DAO member or a delegator
#[derive(TypeId, Encode, Decode, Describe)]
pub struct VoteData {
    pub vote: bool,
    pub vote_weight: Decimal,
    pub voted_by: Option<String>,
}

blueprint! {

    /// Proposal Component keep track of voting status on a proposal and all voters data.
    /// A proposal can be executed according to the [Permissioned Relative Majority & Quorum Voting Mechanism](crate::align_dao).
    pub struct Proposal {
        /// The DAO component address
        dao: ComponentAddress,
        /// The Proposal id
        id: NonFungibleId,

        /// Proposal controller badge, the badge provide access rule to:
        /// - Update DAO Member/Delegator's SBT data;
        /// - Call [dao_proof()](crate::align_dao::DAO_impl::DAO::dao_proof) method to access the dao badge;
        /// - Call [ignored()](crate::align_dao::DAO_impl::DAO::ignored) method to remove the proposal from the DAO if the proposal is ignored;
        /// - Call [current()](crate::local_oracle::LocalOracle_impl::LocalOracle::current) method to get current time from the local oracle;
        /// - Burn [proposal badge](crate::align_dao::ProposalBadge).
        ///
        /// The badge cannot be withdraw or used for any other purpose execept for supporting the DAO's smartcontract logic.
        controller_badge: Vault,
        /// Proposal Badge resource address
        proposal_badge: ResourceAddress,
        /// Member SBT resource address
        member_sbt: ResourceAddress,
        /// Delegator SBT resource address
        delegator_sbt: ResourceAddress,
        /// DAO's [CommitmentPolicy](crate::policies::CommitmentPolicy) on this proposal
        commitment_policy: CommitmentPolicy,
        /// DAO's [EconomicPolicy](crate::policies::EconomicPolicy) on this proposal
        economic_policy: EconomicPolicy,
        /// DAO's proposal quorum on this proposal according to the DAO's [ProposalPolicy](crate::policies::ProposalPolicy)
        proposal_quorum: Decimal,
        /// DAO's local oracle address
        oracle_address: ComponentAddress,

        /// Total voted weight that support this proposal.
        ///
        /// If after the proposal end time, voted weight >= 1/2 of total voted weight
        /// and total voted weight > proposal quorum, the proposal is accepted
        support_voted_weight: Decimal,
        /// Total voted weight of this proposal
        total_voted_weight: Decimal,
        /// End time of the proposal
        end: u64,

        /// Store all the voters data on this proposal
        voters: HashMap<NonFungibleAddress, VoteData>,

        /// Store the methods that will be called on proposal acceptance.
        methods: Methods,
        /// Store the amount of funding that the proposer request on this proposal.
        fund_demand: Decimal,
        /// Store the funding that the proposer can receive on proposal acceptance (if this is a proposal have fund demand > 0)
        fund: Option<Vault>,
        /// Check if the proposal is executed or not
        execute: bool,

        /// If this is a distribution proposal,
        /// the field store the distribute resource and it's initial amount
        distribution: Option<(Vault, Decimal)>,
    }

    impl Proposal {

        /// This function instantiate new Proposal Component.
        ///
        /// # Input
        /// - dao: The DAO component address.
        /// - controller_badge: Bucket contain the controller badge from the DAO,
        /// the badge will provide access rule to update community member's SBT data
        /// or call many restricted method on the DAO.
        /// - id: ID of this proposal.
        /// - end: End time of the proposal.
        /// - primary_reserve_resource: Primary reserve resource address of the DAO.
        /// - proposal_badge: proposal badge address.
        /// - member_sbt: DAO Member SBT address.
        /// - delegator_sbt: Delegator SBT address.
        /// - commitment_policy: DAO's [CommitmentPolicy](crate::policies::CommitmentPolicy) on this proposal.
        /// - economic_policy: DAO's [EconomicPolicy](crate::policies::EconomicPolicy) on this proposal.
        /// - proposal_quorum: DAO's proposal quorum on this proposal according to the DAO's [ProposalPolicy](crate::policies::ProposalPolicy).
        /// - oracle_address: DAO's local oracle address.
        /// - methods: the methods that will be called on proposal acceptance.
        /// - fund_demand: the amount of funding that the proposer request on this proposal.
        /// - distribute: None if this isn't a distribution proposal or the wraped distribution resource address
        ///
        /// # Output
        /// The Proposal Component, the component will be further used to add access rule and globalize on the
        /// [new_proposal()](crate::align_dao::DAO_impl::DAO::new_proposal) method
        /// or [replacement_proposal()](crate::align_dao::DAO_impl::DAO::replacement_proposal) method.
        /// 
        /// # Smartcontract logic
        /// The function should only be called
        /// through the [new_proposal()](crate::align_dao::DAO_impl::DAO::new_proposal) method
        /// or [replacement_proposal()](crate::align_dao::DAO_impl::DAO::replacement_proposal) method.
        pub fn new(
            dao: ComponentAddress,
            controller_badge: Bucket,
            id: NonFungibleId,
            end: u64,
            primary_reserve_resource: ResourceAddress,
            proposal_badge: ResourceAddress,
            member_sbt: ResourceAddress,
            delegator_sbt: ResourceAddress,
            commitment_policy: CommitmentPolicy,
            economic_policy: EconomicPolicy,
            proposal_quorum: Decimal,
            oracle_address: ComponentAddress,
            methods: Methods,
            fund_demand: Decimal,
            distribute: Option<ResourceAddress>,
        ) -> ProposalComponent {
            let fund = if fund_demand > Decimal::ZERO {
                Some(Vault::new(primary_reserve_resource))
            } else {
                None
            };

            let distribution = match distribute {
                Some(address) => Some((Vault::new(address), Decimal::ZERO)),
                None => None,
            };

            Self {
                dao,
                id,
                controller_badge: Vault::with_bucket(controller_badge),
                proposal_badge,
                member_sbt,
                delegator_sbt,
                commitment_policy,
                economic_policy,
                proposal_quorum,
                oracle_address,
                support_voted_weight: Decimal::ZERO,
                total_voted_weight: Decimal::ZERO,
                end,
                voters: HashMap::new(),
                methods,
                fund_demand,
                fund,
                execute: false,
                distribution,
            }
            .instantiate()
        }

        /// This method will compute the vote result, execute the proposed methods,
        /// distribute dividend reward or slash malicious vote accordingly.
        ///
        /// On acceptance:
        /// - If the proposal have a fund demand
        /// the method will also withdraw primary reserve
        /// resource from the DAO's treasury and
        /// deposit it into the fund Vault of this component.
        ///
        /// - If this is a distribution proposal,
        /// the method will also withdraw the distribution
        /// resource from the DAO's treasury and
        /// deposit it into the distribution Vault of this component.
        /// # Access Rule
        /// Anyone can call this method.
        /// # Smartcontract logic
        /// ## Panics
        /// - The proposal haven't ended according to the time data from [LocalOracle Component](crate::local_oracle).
        /// - The proposal have already executed according to the [execute](Proposal_impl::Proposal::execute) field.
        /// 
        /// ## Intra-package access
        /// ***DAOMember/Delegator:***
        /// - Use many helpful write method (reward(), slash(), rage_withdrawable()) from [Delegator](crate::delegator::Delegator)
        ///  or [DAOMember](crate::member::DAOMember) data struct.
        /// ***DAO:***
        /// - Use [get_rage_withdraw()](crate::align_dao::DAO_impl::DAO::get_rage_withdraw) method to get 
        /// rage withdraw time limit and rage withdraw decline multiply rate from [TreasuryPolicy](crate::policies::TreasuryPolicy)
        /// - Use [get_price()](crate::align_dao::DAO_impl::DAO::get_price) method to get current share/reserve price from the
        /// [DAO treasury](crate::treasury) Local Component.
        /// - Use [dao_proof()](crate::align_dao::DAO_impl::DAO::get_price) method to access 
        /// the [dao badge](crate::align_dao::DAO_impl::DAO::dao_badge).
        /// - Use [dao_withdraw()](crate::align_dao::DAO_impl::DAO::dao_withdraw) method to withdraw 
        /// distribution resource from the DAO treasury on accepted distribution proposal.
        /// - Use [accepted()](crate::align_dao::DAO_impl::DAO::accepted) method to mint dividend for rewarding voters and
        /// withdraw fund from the DAO treasury to deposit on the proposal vault if it's accepted.
        /// - Use [rejected()](crate::align_dao::DAO_impl::DAO::rejected) method to burn slash token of malicious voters 
        /// if the proposal is rejected.
        /// - Use [ignored()](crate::align_dao::DAO_impl::DAO::ignored) method to remove the proposal from the DAO.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/proposal/execute_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/proposal/execute_proposal.rtm")]
        /// ```
        pub fn execute(&mut self) {
            assert!(
                self.current() >= self.end,
                "[Proposal]: The proposal has't ended yet."
            );
            assert!(!self.execute, "[Proposal]: The proposal has executed.");
            self.execute = true;

            let dao: DAOComponent = self.dao.into();

            let proposal_id = self.id.clone();

            if self.total_voted_weight >= self.proposal_quorum {
                if self.support_voted_weight > self.total_voted_weight / dec!(2) {
                    let dividend = self.economic_policy.dividend;

                    let (rage_withdraw_decline_multiply, mut time_limit) = dao.get_rage_withdraw();

                    time_limit += self.current();

                    let price = dao.get_price();

                    let dao_proof = self.controller_badge.authorize(|| {
                        for (voter_address, vote_data) in &self.voters {
                            if vote_data.vote {
                                let share_amount =
                                    vote_data.vote_weight / self.support_voted_weight * dividend;
                                let id = voter_address.non_fungible_id();

                                if voter_address.resource_address() == self.delegator_sbt {
                                    let mgr = borrow_resource_manager!(self.delegator_sbt);
                                    let mut delegator_data =
                                        mgr.get_non_fungible_data::<Delegator>(&id);
                                    delegator_data.reward(share_amount);
                                    mgr.update_non_fungible_data(&id, delegator_data);
                                } else if voter_address.resource_address() == self.member_sbt {
                                    let mgr = borrow_resource_manager!(self.member_sbt);
                                    let mut member_data =
                                        mgr.get_non_fungible_data::<DAOMember>(&id);
                                    member_data.reward(share_amount);
                                    mgr.update_non_fungible_data(&id, member_data);
                                }
                            } else {
                                let id = voter_address.non_fungible_id();

                                if voter_address.resource_address() == self.delegator_sbt {
                                    let mgr = borrow_resource_manager!(self.delegator_sbt);
                                    let mut delegator_data =
                                        mgr.get_non_fungible_data::<Delegator>(&id);
                                    delegator_data.rage_withdrawable(time_limit, price);
                                    mgr.update_non_fungible_data(&id, delegator_data);
                                } else if voter_address.resource_address() == self.member_sbt {
                                    let mgr = borrow_resource_manager!(self.member_sbt);
                                    let mut member_data =
                                        mgr.get_non_fungible_data::<DAOMember>(&id);
                                    member_data.rage_withdrawable(
                                        time_limit,
                                        price,
                                        rage_withdraw_decline_multiply,
                                        &self.commitment_policy,
                                    );
                                    mgr.update_non_fungible_data(&id, member_data);
                                }
                            }
                        }

                        dao.dao_proof()
                    });

                    ComponentAuthZone::push(dao_proof);

                    info!("[Proposal]: The proposal id {} been accepted", proposal_id);

                    let result = std::panic::catch_unwind(|| {
                        self.methods.call_all();
                    });

                    match result {
                        Ok(_) => info!("[Proposal]: Successfully executed methods"),
                        Err(err) => error!(
                            "[Proposal]: Something wrong with the provided methods: {:?}",
                            err
                        ),
                    };

                    match &mut self.distribution {
                        None => {}
                        Some((vault, amount)) => {
                            let resource_address = vault.resource_address();
                            vault.put(dao.dao_withdraw(resource_address));
                            *amount = vault.amount()
                        }
                    };

                    let fund = dao.accepted(proposal_id, dividend, self.fund_demand);

                    ComponentAuthZone::pop().drop();

                    match fund {
                        Some(fund) => {
                            match self.fund.as_mut() {
                                Some(fund_vault) => fund_vault.put(fund),
                                None => {
                                    error!("[Proposal]: Somehow this proposal didn't have a fund vault.")
                                }
                            }
                        }
                        None => {}
                    }
                } else {
                    let mut slash = Decimal::ZERO;

                    let dao_proof = self.controller_badge.authorize(|| {
                        for (voter_address, vote_data) in &self.voters {
                            if vote_data.vote {
                                let id = voter_address.non_fungible_id();

                                if voter_address.resource_address() == self.delegator_sbt {
                                    let mgr = borrow_resource_manager!(self.delegator_sbt);
                                    let mut delegator_data =
                                        mgr.get_non_fungible_data::<Delegator>(&id);
                                    slash += delegator_data.slash(self.economic_policy.slash_rate);
                                    mgr.update_non_fungible_data(&id, delegator_data);
                                } else if voter_address.resource_address() == self.member_sbt {
                                    let mgr = borrow_resource_manager!(self.member_sbt);
                                    let mut member_data =
                                        mgr.get_non_fungible_data::<DAOMember>(&id);
                                    slash += member_data.slash(self.economic_policy.slash_rate);
                                    mgr.update_non_fungible_data(&id, member_data);
                                }
                            }
                        }

                        dao.dao_proof()
                    });

                    ComponentAuthZone::push(dao_proof);

                    error!(
                        "[Proposal]: The proposal id {} has been rejected",
                        &proposal_id
                    );

                    dao.rejected(proposal_id, slash);

                    ComponentAuthZone::pop().drop();
                }
            } else {
                error!(
                    "[Proposal]: The proposal id {} has been ignored",
                    &proposal_id
                );
                self.controller_badge.authorize(|| dao.ignored(proposal_id));
            }
        }

        /// This method will allow DAO members to vote on the proposal.
        ///
        /// If the voter is a Representative, the method will also vote for all the representative's Community members.
        /// # Input
        /// - member_proof: the member SBT proof.
        /// - vote: support or against.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong proof provided: Not a single proof from DAO member or Delegator.
        /// - The DAO member currently following a community, on retirement process or have already voted the proposal.
        /// 
        /// ## Intra-package access
        /// - Access many helpful read only method from [Delegator](crate::delegator::Delegator)
        ///  or [DAOMember](crate::member::DAOMember) data struct.
        /// - Use write method [calculate_voting_power()](crate::member::DAOMember::calculate_voting_power)
        /// from [DAOMember](crate::member::DAOMember) data struct to calculate the DAO member voting power.
        /// - Access the [representing](crate::member::DAOMember::representing) data field from
        /// [DAOMember](crate::member::DAOMember) data struct to check if the voter is a representative or not.
        /// - Access the [delegated_amount](crate::delegator::Delegator::delegated_amount) data field to get the Delegator voting power.
        /// - Write new voting proposal on the [voting_proposals](crate::delegator::Delegator::voting_proposals) data field on Delegator SBT.
        /// - Access the representative's community name, follower list, tax percent, remain vote power percent
        /// from the read only method [vote_state()](crate::community::Community_impl::Community::vote_state) for making community vote.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/proposal/user_pattern/vote.rtm`
        /// ```text
        #[doc = include_str!("../rtm/proposal/user_pattern/vote.rtm")]
        /// ```
        pub fn vote(&mut self, member_proof: Proof, vote: bool) {
            let validated_proof = member_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.member_sbt,
                    dec!("1"),
                ))
                .expect("[Proposal]: Error validating proof");

            let current = self.current();

            assert!(
                current < self.end,
                "[Proposal]: The proposal has already ended."
            );

            let proposal_id = self.id.clone();

            let voter_sbt = validated_proof.non_fungible::<DAOMember>();

            let mut voter_data = voter_sbt.data();
            assert!(!voter_data.is_following(), "[Proposal]: You're currently following a community, please consider quit the community first before directly participate in voting.");
            assert!(
                !voter_data.is_retiring(),
                "[Proposal]: You're currently on retirement process."
            );
            let member_address = voter_sbt.address();
            assert!(
                !self.voters.contains_key(&member_address),
                "[Proposal]: You already voted this proposal."
            );

            let mut vote_weight =
                voter_data.calculate_voting_power(&self.commitment_policy, current);

            let community_weight = match voter_data.representing {
                None => {
                    info!(
                        "[Proposal]: You have voted {} on the concept proposal id {}",
                        vote, &proposal_id
                    );
                    Decimal::ZERO
                }

                Some(_community_address) => {
                    info!("[Proposal]: On your community behalf, you have voted {} on the concept proposal id {}", vote, &proposal_id);

                    // let community: CommunityComponent = community_address.into();

                    let dao: DAOComponent = self.dao.into();

                    let community: CommunityComponent =
                        dao.get_community_address(voter_sbt.id()).into(); // This is just for going aroung current Scrypto's [known bug](https://github.com/radixdlt/radixdlt-scrypto/issues/483)

                    let (name, followers, tax, remain_vote_power) = community.vote_state();

                    let mut community_vote_list = HashMap::new();
                    let mut community_weight = Decimal::ZERO;
                    let mut community_tax = Decimal::ZERO;

                    self.controller_badge.authorize(|| {
                        for address in followers {
                            if address.resource_address() == self.delegator_sbt {
                                let mgr = borrow_resource_manager!(self.delegator_sbt);
                                let id = address.non_fungible_id();
                                let mut delegator_data =
                                    mgr.get_non_fungible_data::<Delegator>(&id);
                                delegator_data.voting_proposals.insert(proposal_id.clone());
                                let follower_vote_weight = delegator_data.delegated_amount;
                                if follower_vote_weight > Decimal::ZERO {
                                    mgr.update_non_fungible_data(&id, delegator_data);
                                    let power_tax = follower_vote_weight * tax;
                                    let follower_final_weight = follower_vote_weight - power_tax;
                                    community_tax = community_tax + power_tax;
                                    community_weight = community_weight + follower_final_weight;
                                    community_vote_list.insert(
                                        address.clone(),
                                        VoteData {
                                            vote,
                                            vote_weight: follower_final_weight,
                                            voted_by: Some(name.clone()),
                                        },
                                    );
                                }
                            } else if address.resource_address() == self.member_sbt {
                                let mgr = borrow_resource_manager!(self.member_sbt);
                                let id = address.non_fungible_id();
                                let mut member_data = mgr.get_non_fungible_data::<DAOMember>(&id);
                                let follower_vote_weight = member_data
                                    .calculate_voting_power(&self.commitment_policy, current);
                                if follower_vote_weight > Decimal::ZERO {
                                    mgr.update_non_fungible_data(&id, member_data);
                                    let power_tax = follower_vote_weight * tax;
                                    let follower_final_weight = follower_vote_weight - power_tax;
                                    community_tax = community_tax + power_tax;
                                    community_weight = community_weight + follower_final_weight;
                                    community_vote_list.insert(
                                        address.clone(),
                                        VoteData {
                                            vote,
                                            vote_weight: follower_final_weight,
                                            voted_by: Some(name.clone()),
                                        },
                                    );
                                }
                            }
                        }
                        voter_sbt.update_data(voter_data);
                    });

                    vote_weight += community_tax;
                    vote_weight = vote_weight * remain_vote_power;
                    self.voters.extend(community_vote_list); // If the community member is already voted on this proposal, it will be replaced with this vote by the community representative.

                    community_weight
                }
            };

            self.total_voted_weight += vote_weight + community_weight;
            if vote {
                self.support_voted_weight += vote_weight + community_weight
            };

            self.voters.insert(
                member_address,
                VoteData {
                    vote,
                    vote_weight,
                    voted_by: None,
                },
            );
        }

        /// Read only method to check the voter's vote data on this proposal.
        /// 
        /// The method is for test purpose only and didn't contribute for the DAO's smartcontract logic.
        /// # Input
        /// The DAO Member / Delegator SBT proof.
        /// # Access Rule
        /// Read only, anyone can call this method.
        /// # Smartcontract logic
        /// Panic if wrong proof provided.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/proposal/user_pattern/check_vote.rtm`
        /// ```text
        #[doc = include_str!("../rtm/proposal/user_pattern/check_vote.rtm")]
        /// ```
        pub fn check_vote(&self, identity: Proof) {
            let validated_proof = identity.unsafe_skip_proof_validation();
            let address = if validated_proof.resource_address() == self.member_sbt {
                validated_proof.non_fungible::<DAOMember>().address()
            } else if validated_proof.resource_address() == self.delegator_sbt {
                validated_proof.non_fungible::<Delegator>().address()
            } else {
                panic!("[Proposal]: Wrong proof provided")
            };

            match self.voters.get(&address) {
                None => info!("[Proposal]: You haven't voted on this proposal"),
                Some(vote) => {
                    let vote_weight = vote.vote_weight;
                    if vote.vote {
                        info!(
                            "[Proposal]: You have vote accepted on this proposal with {} vote power",
                            vote_weight
                        )
                    } else {
                        info!(
                            "[Proposal]: You have vote rejected on this proposal with {} vote power",
                            vote_weight
                        )
                    };
                    match &vote.voted_by {
                        None => {}
                        Some(community_name) => {
                            info!(
                                "[Proposal]: Your vote was done by {} community",
                                community_name
                            )
                        }
                    }
                }
            }
        }

        /// This method will allow proposal's voters (direct or indirect) 
        /// to get their resource share after the distribution proposal has been accepted, 
        /// executed and got distribution resource from the treasury.
        /// 
        /// The more voted power that the voter has voted support on the proposal, 
        /// the more distribution share he/she will receive after call this method
        /// # Input
        /// - identity: DAO Member or Delegator SBT.
        /// # Output
        /// - None if the voter voted against the proposal.
        /// - The wrapped resource bucket if the voter support the proposal.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong proof provided: Not a single proof from DAO member or Delegator.
        /// - None on distribution field: The proposal wasn't a distribution proposal.
        /// - Distribution vault empty: The proposal haven't executed successfully or the treasury didn't have the resource.
        /// - Didn't on the voters list: The user didn't vote on the proposal or have already removed from the list after took his/her share.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/proposal/user_pattern/take_distribution.rtm`
        /// ```text
        #[doc = include_str!("../rtm/proposal/user_pattern/take_distribution.rtm")]
        /// ```
        pub fn take_distribution(&mut self, identity: Proof) -> Option<Bucket> {

            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[Proposal]: Can only using 1 SBT at a time."
            );

            let (vault, total_amount) = self
                .distribution
                .as_mut()
                .expect("[Proposal]: This isn't a resource distribution proposal.");

            assert!(!vault.is_empty(), "[Proposal]: The proposal haven't executed successfully or the treasury didn't have this resource.");

            let id = validated_proof.non_fungible_id();

            let resource_address = validated_proof.resource_address();

            let address = NonFungibleAddress::new(resource_address, id);

            match self.voters.remove(&address) {
                None => panic!("[Proposal]: You haven't voted on this proposal or already took your share."),
                Some(vote_data) => {
                    if vote_data.vote {
                        let share_percent = vote_data.vote_weight / self.support_voted_weight;
                        let share_amount = *total_amount * share_percent;
                        let vault_amount = vault.amount();

                        if vault_amount < share_amount {
                            // Since there's some high level math behind the voting process, the received dividend might be rounded up (at some rate) between each beneficiary,
                            // the last one will be the one to take all remain (while it might smaller a bit than the real amount he/she can actually be received, maybe < 0.00001 token)
                            info!("[Proposal]: You have been allocated {} token", vault_amount);
                            Some(vault.take_all())
                        } else {
                            info!("[Proposal]: You have been allocated {} token", share_amount);
                            Some(vault.take(share_amount))
                        }
                    } else {
                        error!("[Proposal]: You didn't vote support this proposal.");
                        return None;
                    }
                }
            }
        }

        /// This method allow the DAO member or Delegator to retract vote from the proposal.
        ///
        /// According to Align DAO smartcontract logic, the function should only be called
        /// through the [rage_quit()](crate::align_dao::DAO_impl::DAO::rage_quit) method.
        /// # Input
        /// - address: The rage quitted DAO member or Delegator SBT address
        /// - community_name: The community name that the member/delegator following before do the rage quitting.
        /// # Access Rule
        /// Not user callable, can only be called by the DAO controller badge.
        /// # Smartcontract logic
        /// The method can only be called
        /// through the [rage_quit()](crate::align_dao::DAO_impl::DAO::rage_quit) method.
        /// 
        /// ## Intra-package access
        /// - Remove the proposal id from Delegator SBT data field [voting_proposals](crate::delegator::Delegator::voting_proposals)
        /// if the delegator is voted on this proposal by the community.
        pub fn retract_vote(&mut self, address: NonFungibleAddress, community_name: String) {
            if address.resource_address() == self.delegator_sbt {
                let vote_data = self.voters.get(&address);

                let remove = match vote_data {
                    None => false,
                    Some(vote_data) => match &vote_data.voted_by {
                        None => false,
                        Some(name) => {
                            if name == &community_name {
                                self.total_voted_weight -= vote_data.vote_weight;
                                if vote_data.vote {
                                    self.support_voted_weight -= vote_data.vote_weight
                                };
                                true
                            } else {
                                false
                            }
                        }
                    },
                };

                if remove {
                    self.voters.remove(&address);
                    let mgr = borrow_resource_manager!(self.delegator_sbt);
                    let mut delegator_data =
                        mgr.get_non_fungible_data::<Delegator>(&address.non_fungible_id());
                    delegator_data.voting_proposals.remove(&self.id);
                    mgr.update_non_fungible_data(&address.non_fungible_id(), delegator_data);
                }
            }
        }

        /// This method will allow the DAO member who propose this proposal can get the allocated fund if the proposal is accepted.
        /// 
        /// If the proposal is rejected or ignored, the method will only return empty bucket.
        /// # Input
        /// - proposal_badges: the DAO member's proposal badges (can be more than 1)
        /// # Output
        /// - The allocated fund to the proposal.
        /// - Other proposal badge (wasn't belong to this proposal).
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong resource provided: Not the right proposal badge address from the DAO.
        /// - Proposal id not match: The bucket doesn't contain the right badge of this proposal.
        /// - The proposal didn't have a fund demand (fund demand = 0 or <0).
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/proposal/proposer_only/withdraw_fund.rtm`
        /// ```text
        #[doc = include_str!("../rtm/proposal/proposer_only/withdraw_fund.rtm")]
        /// ```
        pub fn withdraw_fund(&mut self, mut proposal_badges: Bucket) -> (Bucket, Bucket) {
            assert!(
                proposal_badges.resource_address() == self.proposal_badge,
                "[Proposal]: Wrong resource."
            );

            assert!(
                proposal_badges.non_fungible_ids().contains(&self.id),
                "[Proposal]: This is not your proposal."
            );

            assert!(
                self.execute, 
                "[Proposal]: The proposal hasn't executed yet."
            );

            match self.fund.as_mut() {
                Some(fund_vault) => {
                    let amount = fund_vault.amount();
                    info!(
                        "[Proposal]: Withdrawed {} fund allocated to this proposal",
                        amount
                    );
                    let badge = proposal_badges.take_non_fungible(&self.id);
                    self.controller_badge.authorize(|| badge.burn());
                    (fund_vault.take_all(), proposal_badges)
                }
                None => {
                    panic!("[Proposal]: This isn't a proposal with fund demand.")
                }
            }
        }

        /// Read only method to get the current remain distribution resource on the proposal vault.
        /// 
        /// The method is for test purpose only and didn't contribute for the DAO's smartcontract logic.
        /// # Access Rule
        /// Read only, anyone can call this method.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/proposal/read_only/check_distribution_resource.rtm`
        /// ```text
        #[doc = include_str!("../rtm/proposal/read_only/check_distribution_resource.rtm")]
        /// ```
        pub fn check_distribution_resource(&self) -> Decimal {
            let vault = self
                .distribution
                .as_ref()
                .expect("[Proposal]: This isn't a resource distribution proposal.");
            let amount = vault.0.amount();
            info!(
                "[Proposal]: Currently there's {} resource remain on the distribution proposal",
                amount
            );
            amount
        }

        /// Read only method to check current vote status of this proposal.
        ///
        /// The method is for test purpose only and didn't contribute for the DAO's smartcontract logic.
        /// # Output
        /// support and against voted weight
        /// # Access Rule
        /// Read only, anyone can call this method.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/proposal/read_only/vote_status.rtm`
        /// ```text
        #[doc = include_str!("../rtm/proposal/read_only/vote_status.rtm")]
        /// ```
        pub fn vote_status(&self) -> (Decimal, Decimal) {
            let support = self.support_voted_weight;
            let against = self.total_voted_weight - self.support_voted_weight;
            info!(
                "Current voting status: FOR: {}, AGAINST: {}, TOTAL_VOTED: {}",
                support, against, self.total_voted_weight
            );
            (support, against)
        }

        /// Internal method to get current data from the oracle, can only be called internally
        /// 
        /// # Smartcontract logic
        /// ## Intra-package access
        /// - Access the read only method [current()](crate::local_oracle::LocalOracle_impl::LocalOracle::current).
        fn current(&self) -> u64 {
            let local_oracle: LocalOracleComponent = self.oracle_address.into();
            let current = self.controller_badge.authorize(|| local_oracle.current());
            current
        }
    }
}
