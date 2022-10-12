/*!
This module implement [Delegator](Delegator) struct to store the Delegator Soul Bound Data with and many helpful methods for user pattern. This is mainly designed for [Liquid Democracy](crate::align_dao#liquid-democracy).

### Delegators
Delegators are people who don't want to go through "commitment" but still
want to indirectly participate in decision making through representative choosing and receive suitable reward.

- Responsibility: Delegators cannot withdraw their resource if they still have on-going vote. This is to prevent vote reentrancy.
- Trust: Delegators would need to trust that the representative they follow would concern the best for their interest.
- Right: Delegators can "rage quit" to degrade their representative's credibility. They also receive dividend reward through indirectly participate on DeGov.

On Align DAO, each Delegator will have a Soul Bound token to store [Delegator data](Delegator).
Participants can become Delegators and receive a Delegator SBT through the [become_delegator()](crate::align_dao::DAO_impl::DAO::become_delegator) method.

### Methods overview
- [new()](Delegator::new): Create new Delegator SBT data.
- [is_following()](Delegator::is_following): Check if the Delegator is following a representative or not. - Read only method.
- [reward()](Delegator::reward): Change the Delegator SBT data according to the reward amount that he/she is rewarded.
- [slash()](Delegator::slash): Change the Delegator SBT data according to the slash rate that he/she is slashed from choosed malicious representative.
- [rage_withdrawable()](Delegator::rage_withdrawable): Helpful method to make the Delegator can do rage withdraw.
- [is_not_voting()](Delegator::is_not_voting): Helpful method to check if the Delegator is currently voting on any proposal or not. - Read only method.
*/
use scrypto::prelude::*;

/// The Delegator SBT.
///
/// The SBT will can only be issued when the shareholder delegate an amount of DAO share into the DAO.
#[derive(NonFungibleData)]
pub struct Delegator {
    /// Store the resource amount that the Delegator has delegated into the DAO. This will also be the same as the delegator voting power.
    #[scrypto(mutable)]
    pub delegated_amount: Decimal,

    /// Store the dividend reward amount that the Delegator is able to claim.
    #[scrypto(mutable)]
    pub rewarded: Decimal,

    /// The community's name that the delegator is following.
    #[scrypto(mutable)]
    pub following_community: Option<String>,

    /// If the delegator is eligible for a rage withdraw, it will store the amount and the time limit which the delegator can rage withdraw.
    #[scrypto(mutable)]
    pub rage_withdrawable: Option<(Decimal, u64)>,

    /// The on-voting proposals.
    #[scrypto(mutable)]
    pub voting_proposals: BTreeSet<NonFungibleId>,
}

impl Delegator {
    /// Create new Delegator Soul Bound Data.
    ///
    /// # Input:
    /// - delegate_amount: resource amount that the new delegator want to delegate.
    pub fn new(delegated_amount: Decimal) -> Self {
        Self {
            delegated_amount,
            rewarded: Decimal::ZERO,
            following_community: None,
            rage_withdrawable: None,
            voting_proposals: BTreeSet::new(),
        }
    }

    /// Check if the Delegator is following a representative or not. - Read only method.
    pub fn is_following(&self) -> bool {
        self.following_community.is_some()
    }

    /// Change the Delegator SBT data according to the reward amount that he/she is rewarded
    /// after a proposal is executed from [execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
    ///
    /// # Input
    /// - share: The DAO share amount that the Delegator is rewarded
    pub fn reward(&mut self, share: Decimal) {
        self.rewarded += share
    }

    /// Change the Delegator SBT data according to the slash rate that he/she is slashed from choosed malicious representative
    /// after a proposal is executed from [execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
    ///
    /// # Input
    /// - slash_rate: The DAO slash rate according to [EconomicPolicy](crate::policies::EconomicPolicy).
    /// # Output
    /// The amount that the Delegator was slashed.
    pub fn slash(&mut self, slash_rate: Decimal) -> Decimal {
        let slash_amount = self.delegated_amount * slash_rate;
        self.delegated_amount -= slash_amount;
        slash_amount
    }

    /// Helpful method to make the Delegator can do rage withdraw
    /// after a proposal is executed from [execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
    ///
    /// The DAO member can later do rage withdraw from the [rage_withdraw()](crate::align_dao::DAO_impl::DAO::rage_withdraw) method
    /// # Input
    /// - time_limit: the time limit that Delegator can do rage withdraw according to [TreasuryPolicy](crate::policies::TreasuryPolicy).
    /// - price: share/reserve price at the proposal's execution time.
    pub fn rage_withdrawable(&mut self, time_limit: u64, price: Decimal) {
        self.rage_withdrawable = Some((self.delegated_amount / price, time_limit));
    }

    /// Helpful method to check if the Delegator is currently voting on any proposal or not. - Read only method.
    ///
    /// The method will input the on-going (haven't executed) proposals from the DAO and check the Delegator's voting list,
    /// remove any proposal that isn't on the list (have already executed).
    ///
    /// Return true if the Delegator's voting list is empty
    pub fn is_not_voting(&mut self, proposals: &HashMap<NonFungibleId, ComponentAddress>) -> bool {
        self.voting_proposals
            .retain(|proposal_id| proposals.contains_key(proposal_id));
        self.voting_proposals.is_empty()
    }
}
