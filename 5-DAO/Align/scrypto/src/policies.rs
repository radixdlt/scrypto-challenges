/*!
Policies are all the smartcontract configs that will govern the DAO's smartcontract logic. 
Currently there are four policies on Align package.

### TreasuryPolicy
[TreasuryPolicy] store the DAO's internal treasury policy, mainly used for [Internal Treasury Function](crate::align_dao#internal-treasury-function).
- [swap_fee](TreasuryPolicy::swap_fee): The swap fee when using the DAO's 
internal treasury to swap between DAO share token and the primary reserve resource. The data is mutable.
- [withdraw_threshold](TreasuryPolicy::withdraw_threshold): The primary reserve resource withdraw threshold 
that the DAO's executed proposal can withdraw (%). DAO's executed proposal cannot withdraw more than this threshold for a time period.
The data is immutable.
- [withdraw_period](TreasuryPolicy::withdraw_period): The primary reserve resource withdraw time period (seconds). 
For each time period, DAO's executed proposal can only withdraw the treasury's primary reserve resource 
at maximum the withdraw threshold. The data is immutable.
- [rage_withdraw_decline_multiply](TreasuryPolicy::withdraw_period): The rage withdraw decline mutiply rate. 
The more commitment grow rate that a DAO Member has (longer retirement process), the less resource he/she will receive when do rage withdraw. 
The data is immutable.
- [rage_withdraw_time_limit](TreasuryPolicy::rage_withdraw_time_limit): The rage withdraw time limit. 
DAO's participant can only do rage withdraw within this time limit. The data is immutable.

### EconomicPolicy
[EconomicPolicy] store DAO's economic policy, mainly used to distribute 
dividend reward for DAO's participants or slash resource of malicious users.
- [dividend](EconomicPolicy::dividend): DAO share dividend amount for each successful 
executed proposal (DAO share/proposal) - Mutable
- [slash_rate](EconomicPolicy::slash_rate): DAO share slash rate for each failed 
attempt to vote on a proposal with low agreement rate (%/action) - Mutable

### ProposalPolicy
[ProposalPolicy] store the DAO's proposal policy, mainly used for [Permissioned Relative Majority & Quorum Voting Mechanism](crate::align_dao#permissioned-relative-majority--quorum-voting-mechanism).
- [proposal_requirement](ProposalPolicy::proposal_requirement): The vote power requirement for a DAO member when making a proposal - Mutable
- [proposal_quorum](ProposalPolicy::proposal_quorum): The threshold that total voted power on a 
proposal must pass before the proposal can be executed (or rejected) - Mutable
- [proposal_minimum_delay](ProposalPolicy::proposal_minimum_delay): Minimum delay time of a proposal (seconds) 
- Immutable, to prevent attacker propose a short delay proposal.

Proposal Policy include a helpful method [check_requirement()](ProposalPolicy::check_requirement) 
to check if the provided vote power meet the proposal requirement or not.

### CommunityPolicy 
[CommunityPolicy] store community policy according to [Liquid Democracy](crate::align_dao#liquid-democracy).
- [initial_credibility](CommunityPolicy::initial_credibility): 
the initial credibility score for a new representative - Immutable, to prevent unfairness between communities.
- [representative_requirement](CommunityPolicy::representative_requirement): The vote power 
requirement for a DAO member before become a representative - Mutable

Community Policy include a helpful method [check_requirement()](CommunityPolicy::check_requirement) 
to check if the provided vote power meet the representative requirement or not.

### CommitmentPolicy
[CommitmentPolicy] store commitment policy according to the [Commitment Voting Mechanism](crate::align_dao#commitment-voting-mechanism).
- [initital_commitment_rate](CommitmentPolicy::initital_commitment_rate): 
the initial voting power increase rate for each week DAO's member commited on contributing for the DAO - Mutable
- [minimum_retirement](CommitmentPolicy::minimum_retirement): The minimum retirement length a 
DAO's member must have on his/her commitment (seconds) - Mutable
- [maximum_retirement](CommitmentPolicy::maximum_retirement): 
The maximum retirement length a DAO's member can have on his/her commitment (seconds) - Mutable
- [commitment_grow_rate](CommitmentPolicy::commitment_grow_rate): 
The commitment grow rate for each extra period the DAO's member set their
retirement length minus the minimum retirement length.
- [maximum_vote_rate](CommitmentPolicy::maximum_vote_rate): 
The maximum vote power multiply rate that a member can achieve after their long commitment - Mutable
- [period_length](CommitmentPolicy::period_length): The retirement period length, 
when the DAO Member is on the retirement process, 
they can claim a part of their committed resource after each period (seconds) - Mutable

Commitment Policy include two helpful methods:
- Method [calculate_retirement_period()](CommitmentPolicy::calculate_retirement_period): 
calculate the total periods that DAO Member have to go through given the provided retirement length.
- Method [assert_retirement()](CommitmentPolicy::assert_retirement): 
Check if the provided retirement length is on the acceptable range or not.
*/

use scrypto::prelude::*;
/// A helpful struct to store DAO's internal treasury policy, mainly used for [Internal Treasury Function](crate::align_dao#internal-treasury-function).
///
/// Since the treasury is vulnerable on attack, most of it policy have to be immutable.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct TreasuryPolicy {
    /// The swap fee when using the DAO's 
    /// internal treasury to swap between DAO share token and the primary reserve resource (%) - Mutable
    pub swap_fee: Decimal,
    /// The primary reserve resource withdraw threshold 
    /// that the DAO's executed proposal can withdraw (%). DAO's executed proposal cannot withdraw more than this threshold for a time period.
    ///  - Immutable
    pub withdraw_threshold: Decimal,
    /// The primary reserve resource withdraw time period (seconds). 
    /// For each time period, DAO's executed proposal can only withdraw the treasury's primary reserve resource 
    /// at maximum the withdraw threshold.
    ///  - Immutable
    pub withdraw_period: u64,
    /// The rage withdraw decline mutiply rate. 
    /// The more commitment grow rate that a DAO Member has (longer retirement process), the less resource he/she will receive when do rage withdraw - Immutable
    pub rage_withdraw_decline_multiply: Decimal,
    /// The time limit range for rage withdraw after a proposal in conflict got accepted- Immutable
    pub rage_withdraw_time_limit: u64,
}

/// A helpful struct to store DAO's economic policy.
#[derive(TypeId, Encode, Decode, Describe, Clone)]
pub struct EconomicPolicy {
    /// DAO share dividend amount for each successful executed proposal (DAO share/proposal) - Mutable
    pub dividend: Decimal,
    /// DAO share slash rate for each failed attempt to vote on a proposal with low agreement rate (%/action) - Mutable
    pub slash_rate: Decimal,
}

/// A helpful struct to store DAO's proposal policy, mainly used for [Permissioned Relative Majority & Quorum Voting Mechanism](crate::align_dao#permissioned-relative-majority--quorum-voting-mechanism).
#[derive(TypeId, Encode, Decode, Describe)]
pub struct ProposalPolicy {
    /// The vote power requirement for a DAO member when making a proposal - Mutable
    pub proposal_requirement: Decimal,
    /// The threshold that total voted power on a proposal must pass before the proposal can be executed (or rejected) - Mutable
    pub proposal_quorum: Decimal,
    /// Minimum delay time of a proposal (seconds) - Immutable, to prevent very short delay proposal for attacker.
    pub proposal_minimum_delay: u64,
}

impl ProposalPolicy {

    /// Helpful method to check if the provided vote power meet the proposal requirement or not.
    pub fn check_requirement(&self, vote_power: Decimal) {
        assert!(
            vote_power >= self.proposal_requirement,
            "[ProposalPolicy]: You don't have enough vote weight to make a proposal."
        );
    }
}

/// A helpful struct to store community policy according to [Liquid Democracy](crate::align_dao#liquid-democracy).
#[derive(TypeId, Encode, Decode, Describe)]
pub struct CommunityPolicy {
    /// the initial credibility score for a new representative - Immutable, to prevent unfairness between communities.
    pub initial_credibility: u8,
    /// The vote power requirement for a DAO member before become a representative- Mutable
    pub representative_requirement: Decimal,
}

impl CommunityPolicy {

    /// Helpful method to check if the provided vote power meet the representative requirement or not.
    pub fn check_requirement(&self, vote_power: Decimal) {
        assert!(
            vote_power >= self.representative_requirement,
            "[CommunityPolicy]: You don't have enough vote weight to become a representative."
        );
    }
}

/// A helpful struct to store commitment policy, mainly used for [Internal Treasury Function](crate::align_dao#internal-treasury-function).
#[derive(TypeId, Encode, Decode, Describe, Clone)]
pub struct CommitmentPolicy {
    /// The longer a members commit their DAO share, the more voting power they get.
    ///
    /// This is the initial voting power increase rate for each week DAO's member commited on contributing for the DAO.
    /// Any participant successfully become DAO Member will get this initial commitment rate - Mutable
    pub initital_commitment_rate: Decimal,
    /// The minimum retirement length a DAO's member must have on his/her commitment (seconds) - Mutable
    /// 
    /// The instantiator can always set the minimum retirement low (at most 0u64) 
    /// but it's HIGHLY RECOMMENDED to make this config high (Recommend at least >= 1 Month, ~ 2,678,400 seconds) since low retirement length will make the DAO vulnerable to vote-reentrancy.
    pub minimum_retirement: u64,
    /// The maximum retirement length a DAO's member can have on his/her commitment (seconds) - Mutable
    pub maximum_retirement: u64,
    /// The longer a member set his retirement length, the more their commitment rate would increase - Mutable
    ///
    /// This is the commitment grow rate for each extra period the DAO's member set their
    /// retirement length minus the minimum retirement length.
    pub commitment_grow_rate: Decimal,
    /// The maximum vote power multiply rate that a member can achieve after their long commitment - Mutable
    pub maximum_vote_rate: Decimal,
    /// The retirement period length, when the DAO Member is on the retirement process, they can claim a part of their committed resource after each period (seconds) - Mutable
    pub period_length: u64,
}

impl CommitmentPolicy {

    /// Helpful method to calculate the total periods that DAO Member have to go through 
    /// given the provided retirement length.
    ///
    /// With each extra period on retirement, the DAO Member will also have extra vote weight.
    ///
    /// # Input
    /// x: the provided retirement length
    /// # Output
    /// Number of retirement period that the person with provided length have to go through
    ///
    /// Perform flooring division: (x-a)/b
    pub fn calculate_retirement_period(&self, x: u64) -> u64 {
        let a = self.minimum_retirement;
        if x < a {
            return 0;
        } else {
            let b = self.period_length;
            (x - a) / b
        }
    }

    /// Helpful method to check if the provided retirement length is on the acceptable range or not.
    /// 
    /// The provided retirement length should be greater than [minimum retirement length](CommitmentPolicy::minimum_retirement) 
    /// and smaller than [maximum retirement length](CommitmentPolicy::maximum_retirement).
    pub fn assert_retirement(&self, length: u64) {
        assert!(
            length <= self.maximum_retirement && length >= self.minimum_retirement,
            "[CommitmentPolicy]: Retirement length provided is not acceptable"
        );
    }
}
