/*!
This module contain two structs mainly designed for the [Commitment Voting Mechanism](crate::align_dao#commitment-voting-mechanism): [DAOMember] and [RetirementProcess].

## Protocol entities
### DAO Members
DAO members are people who have voting right by going through "commitment" according to the Commitment Voting Mechanism.

- Responsibility: DAO members are the core unit that will have to responsible for the DAO's life since their resources are committed to the DAO's BODY. They are required to: vote (directly or indirectly through a representative) for the DAO interest;
comply the retirement process; R&D and create proposals to grow the DAO financially through smartcontracts and use of fund. DAO members before a withdrawal will have to go through their choosen retirement process.
- Trust: The DAO members would need to trust (and also build) the overall DAO's SPIRIT (vision, value) for long term goals.
- Right: DAO members have rights to: propose a concept and get dividend reward if it's accepted; become a representative; join a Community run by a representative and indirectly participate in DeGov;
earn voting power through "commitment" history on the DAO; participate on DeGov to get dividend reward; accumulate dividend reward into current commitment account for more voting power.

A DAO member can also be a Representative or a Vote Delegator.

On Align DAO, each DAO member will have a Soul Bound token to store [DAO member data](DAOMember). Participants can become DAO members and receive a DAO Member SBT through the [become_dao_member()](crate::align_dao::DAO_impl::DAO::become_dao_member) method.

### Representatives
Representatives are DAO members who interested in running and represent a specific Community (Democrat, Republican, Education Department, Agriculture and Rural Development Department,...) according to [Liquid Democracy](crate::align_dao#liquid-democracy).

- Responsibility: Great power comes great responsibility. A Representative must "represent" his/her Community, vote on their behalf,
and act on the best of their interest. When a DAO member become a representative, that mean they risk losing their vote power as well as all their committed resources if their credibility degrade.
Thus, the representative's voting power also represent his/her conviction.
- Trust: From trust to conviction, the representative must have high conviction on the SPIRIT (vision, value) that he/she would bring for his/her Community.
- Right: Representatives have rights to: vote on behalf of their Community; earn Community's vote weight tax and claim more dividend reward accordingly. (The vote weight tax is to pay for transaction fee and his/her mental burden on decision making process).

Representatives cannot accumulate dividend reward into current commitment account for more voting power. They also cannot go into retirement process and abandon their community as long as there are still people follow their communities.

DAO members can create a Community and become representatives through the [become_representative()](crate::align_dao::DAO_impl::DAO::become_representative) method.

Representative will have to manage the Community he/she created through [Community blueprint](crate::community).

## Methods overview

### DAOMember

[DAOMember] struct is the Soul Bound Data stored on DAO member's SBT. Method overview:
- [new()](DAOMember::new): Create new DAO member SBT data.
- [is_following()](DAOMember::is_following): Check if the SBT holder is currently following a community or not - Read only method.
- [is_representing()](DAOMember::is_representing): Check if the SBT holder is currently representing a community or not - Read only method.
- [is_retiring()](DAOMember::is_retiring): Check if the SBT holder is currently on retirement process or not - Read only method.
- [calculate_voting_power()](DAOMember::calculate_voting_power): Helpful method to calculate the SBT holder's current vote power according to Commitment Voting Mechanism
- [calculate_total_commitment_grow_rate()](DAOMember::calculate_total_commitment_grow_rate): Helpful method to calculate total commitment grow rate for each week the SBT holder has committed his/her resource. - Read only method.
- [reward()](DAOMember::reward): Helpful method to change DAOMember SBT data when the holder is rewarded with dividend from participating in the DAO.
- [slash()](DAOMember::slash): Helpful method to change DAOMember SBT data when the holder's commitment resource is slashed because of malicious behaviours.
- [rage_withdrawable()](DAOMember::rage_withdrawable): Helpful method to change DAOMember SBT data to allow the holder do [Rage Withdraw](crate::align_dao::DAO_impl::DAO::rage_withdraw) through the DAO.
- [retire()](DAOMember::retire): Helpful method to begin the retirement periods for the DAO member.
- [accumulate_dividend()](DAOMember::accumulate_dividend): Helpful method to make the DAO member accumulate his/her dividend reward
- [withdraw_by_amount()](DAOMember::withdraw_by_amount): Helpful method to allow DAO member withdraw from his/her account by input amount.
- [withdraw_all()](DAOMember::withdraw_all): Helpful method to allow DAO member withdraw all resource from his/her account.
- [confiscate()](DAOMember::confiscate): Helpful method to confiscate all of the representative commited resource if he/she is corrupted.
- [accept_replacement()](DAOMember::accept_replacement): Helpful method for the DAO to accept the member's replacement and immediately end his/her retirement process.

### RetirementProcess

[RetirementProcess] struct store DAO member's retirement process data. Method overview:
- [new()](RetirementProcess::new): Helpful method to create new retirement process data struct.
- [advance()](RetirementProcess::advance): Helpful method to advance the member's retirement process according to current time data.
- [advance_all()](RetirementProcess::advance_all): Helpful method to end the member's retirement process on the DAO's authority.

*/
use crate::{policies::CommitmentPolicy, utils::expo};
use scrypto::prelude::*;

const WEEK: u64 = 604800;

/// The DAO Member SBT data.
///
/// The SBT can only be issued when the shareholder commit an amount of DAO share for a period of time.
#[derive(NonFungibleData)]
pub struct DAOMember {
    /// The initial time the DAO member committed his/her resources.
    committed_start: u64,

    /// The resource amount that the DAO member currently commited.
    #[scrypto(mutable)]
    pub committed_amount: Decimal,

    /// The dividend amount that the DAO member currently be rewarded from participating on the DAO.
    #[scrypto(mutable)]
    pub rewarded: Decimal,

    /// Store the maxed vote rate of the DAO member if he/she has maxed out
    #[scrypto(mutable)]
    max: Decimal,

    /// The retirement length that the member has agreed to go through on the retirement process.
    retirement_length: u64,

    /// Store the community's name if the DAO member is following one.
    #[scrypto(mutable)]
    pub following_community: Option<String>,

    /// The community address that the member is currently representing (if there is one)
    #[scrypto(mutable)]
    pub representing: Option<ComponentAddress>,

    /// Store the retirement process if the member is currently having one.
    #[scrypto(mutable)]
    pub retirement: Option<RetirementProcess>,

    /// If the DAO member is eligible for a rage withdraw, it will store the amount and the time limit which the member can rage withdraw.
    #[scrypto(mutable)]
    pub rage_withdrawable: Option<(Decimal, u64)>,
}

impl DAOMember {
    /// Create new DAOMember Soul Bound Data.
    ///
    /// # Input
    /// - committed_amount: resource amount that the new member want to commit.
    /// - retirement_length: the retirement length that the new member agreed on.
    /// - commitment_policy: the borrowed Commitment Policy struct.
    /// - oracle: borrowed LocalOracleComponent.
    /// # Output
    /// New SBT data
    pub fn new(
        committed_amount: Decimal,
        retirement_length: u64,
        commitment_policy: &CommitmentPolicy,
        current: u64,
    ) -> Self {
        commitment_policy.assert_retirement(retirement_length);

        Self {
            committed_start: current,
            committed_amount,
            rewarded: Decimal::ZERO,
            max: Decimal::ZERO,
            retirement_length,
            following_community: None,
            representing: None,
            retirement: None,
            rage_withdrawable: None,
        }
    }

    /// Check if the DAO member is following a represntative or not. - Read only method.
    pub fn is_following(&self) -> bool {
        self.following_community.is_some()
    }

    /// Check if the DAO member is representing a community or not. - Read only method.
    pub fn is_representing(&self) -> bool {
        self.representing.is_some()
    }

    /// Check if the DAO member is on retirement process or not. - Read only method.
    pub fn is_retiring(&self) -> bool {
        self.retirement.is_some()
    }

    /// Helpful method to calculate a member's voting power used the Commitment Voting Mechanism.
    ///
    /// This method isn't including taxing mechanism and will only
    /// calculate member's voting power "before" he/she is representing or following
    /// any taxed community according to Align DAO's Liquid Democracy.
    ///
    /// The calculation will use the DAO's [Commitment Policy](crate::policies::CommitmentPolicy) and current time from [Local Oracle component](crate::local_oracle::LocalOracle_impl::LocalOracle).
    ///
    /// The voting power will be calculated as follow:
    ///
    /// - x = current voting power. (Decimal)
    /// - y = DAO member inital committed time. (u64, unix time count by second)
    /// - z = DAO member current committed resource amount. (Decimal)
    /// - m = maximum vote multiply rate. (Decimal)
    /// - t = current time. (u64, unix time count by second)
    /// - k1 = total commitment grow rate for each week the member has committed his/her resources
    /// - k2 - 1 = total vote multiply rate for his/her voting power compared to the committed resources amount
    ///
    /// ```
    /// const WEEK: u64 = 604800;
    /// let k1 = a + Decimal::from(q) * b;
    /// let k2 = expo(dec!(1) + k1, (t-y) / WEEK); // expo: an exponential function, same as x^y
    ///
    /// let x = if k2 >= m {
    ///     z * m
    /// } else {
    ///     z * kz
    /// }
    ///
    /// return x
    /// ```
    ///
    /// The method will write the [max](DAOMember::max) data field on DAO Member SBT if his/her
    /// voting power has maxed compare to the maximum vote rate on [CommitmentPolicy](crate::policies::CommitmentPolicy).
    pub fn calculate_voting_power(
        &mut self,
        commitment_policy: &CommitmentPolicy,
        current: u64,
    ) -> Decimal {
        if self.retirement.is_some() {
            return Decimal::ZERO;
        } else {
            let m = commitment_policy.maximum_vote_rate;
            let z = self.committed_amount;

            if self.max < m {
                let y = self.committed_start;
                assert!(current >= y, "[CommitmentPolicy]: Wrong time provided");
                let t = current;
                let k1 = self.calculate_total_commitment_grow_rate(commitment_policy);
                let k2 = expo(dec!(1) + k1, (t - y) / WEEK);
                if k2 >= m {
                    self.max = m;
                    return z * m;
                } else {
                    return z * k2;
                }
            } else {
                self.max = m;
                z * m
            }
        }
    }

    /// Helpful method to calculate a member's commitment grow rate for each week the member has committed his/her resources. - Read only method.
    ///
    /// The rate will also be used to calculate the rage withdrawable amount of this member on a against vote of accepted proposals.
    ///
    /// The calculation will use the DAO's [Commitment Policy](crate::policies::CommitmentPolicy).
    ///
    /// The voting power will be calculated as follow:
    /// - k1 = total commitment grow rate for each week the member has committed his/her resources
    /// - q = DAO member retirement periods. (u64, count by number)
    /// - a = initial commitment grow rate. (Decimal)
    /// - b = commitment grow rate. (Decimal)
    /// ```
    /// let k1 = a + Decimal::from(q) * b;
    /// return k1
    /// ```
    pub fn calculate_total_commitment_grow_rate(
        &self,
        commitment_policy: &CommitmentPolicy,
    ) -> Decimal {
        let q = commitment_policy.calculate_retirement_period(self.retirement_length);
        let a = commitment_policy.initital_commitment_rate;
        let b = commitment_policy.commitment_grow_rate;
        let k1 = a + Decimal::from(q) * b;
        return k1;
    }

    /// Change the DAO member's SBT data according to the reward amount that he/she is rewarded after a proposal is executed from [execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
    /// # Input
    /// - share: The DAO share amount that the DAO member is rewarded
    pub fn reward(&mut self, share: Decimal) {
        self.rewarded += share
    }

    /// Change the DAO member's SBT data according to the slash rate that he/she is slashed from malicious behaviour after a proposal is executed from [execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
    ///
    /// # Input
    /// - slash_rate: The DAO slash rate according to the DAO's [EconomicPolicy](crate::policies::EconomicPolicy).
    /// # Output
    /// The amount that the DAO member was slashed.
    pub fn slash(&mut self, slash_rate: Decimal) -> Decimal {
        let slash_amount = self.committed_amount * slash_rate;
        self.committed_amount -= slash_amount;
        slash_amount
    }

    /// Helpful method to make the DAO member can do rage withdraw after 
    /// a proposal is executed from [execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
    ///
    /// The DAO member can later do rage withdraw from the [rage_withdraw()](crate::align_dao::DAO_impl::DAO::rage_withdraw) method
    /// # Input
    /// - time_limit: the time limit that DAO member can do rage withdraw according to [TreasuryPolicy](crate::policies::TreasuryPolicy).
    /// - price: share/reserve resource price at the proposal's execution time.
    /// - rage_withdraw_decline_multiply: The rage withdraw decline multiply rate to decline rage withdraw amount of DAO members who have large commitment grow rate according to [TreasuryPolicy](crate::policies::TreasuryPolicy).
    /// - commitment_policy: The [Commitment Policy](crate::policies::CommitmentPolicy) that the DAO's using.
    pub fn rage_withdrawable(
        &mut self,
        time_limit: u64,
        price: Decimal,
        rage_withdraw_decline_multiply: Decimal,
        commitment_policy: &CommitmentPolicy,
    ) {
        let commitment_grow_rate = self.calculate_total_commitment_grow_rate(commitment_policy);
        let withdrawable = (dec!(1) - commitment_grow_rate * rage_withdraw_decline_multiply)
            * self.committed_amount
            / price;
        self.rage_withdrawable = Some((withdrawable, time_limit));
    }

    /// Begin the retirement periods for the DAO member through the [retire()](crate::align_dao::DAO_impl::DAO::retire),
    /// [withdraw_by_amount()](crate::align_dao::DAO_impl::DAO::withdraw_by_amount) or [withdraw_all()](crate::align_dao::DAO_impl::DAO::withdraw_all) methods.
    ///
    /// # Input
    /// - commitment_policy: The [Commitment Policy](crate::policies::CommitmentPolicy) that the DAO's using.
    /// - current: current time data from [Local Oracle component](crate::local_oracle::LocalOracle_impl::LocalOracle).
    pub fn retire(&mut self, commitment_policy: &CommitmentPolicy, current: u64) {
        match &mut self.retirement {
            None => {
                self.retirement = Some(RetirementProcess::new(&self, commitment_policy, current));
                self.following_community = None;
                info!("[DAOMember]: Begin retirement process.")
            }

            Some(retirement_process) => {
                retirement_process.advance(&mut self.committed_amount, commitment_policy, current);
            }
        }
    }

    /// Helpful method to make the DAO member accumulate his/her dividend reward
    /// through the [accumulate_dividend()](crate::align_dao::DAO_impl::DAO::accumulate_dividend) method.
    ///
    /// # Input
    /// - amount: The accumulate amount that the DAO member want.
    pub fn accumulate_dividend(&mut self, amount: Decimal) {
        assert!(
            self.rewarded >= amount,
            "[DAOMember]: Don't have enough dividend reward."
        );
        self.rewarded -= amount;
        self.committed_amount += amount;
    }

    /// Helpful method to allow DAO member withdraw from his/her account by input amount
    /// through the [withdraw_by_amount()](crate::align_dao::DAO_impl::DAO::withdraw_by_amount) method.
    pub fn withdraw_by_amount(&mut self, amount: Decimal) {
        self.rage_withdrawable = None;

        let retirement_process = self.retirement.as_mut().expect(
            "[DAOMember]: Somehow you're still haven't started the retirement process yet!",
        );
        assert!(
            retirement_process.withdrawable >= amount,
            "[DAOMember]: Not enough resource amount for withdrawal"
        );
        retirement_process.withdrawable -= amount;
    }

    /// Helpful method to allow DAO member withdraw all resource (allowable) from his/her account
    /// through the [withdraw_all()](crate::align_dao::DAO_impl::DAO::withdraw_all) method.
    pub fn withdraw_all(&mut self) -> Decimal {
        self.rage_withdrawable = None;

        let retirement_process = self.retirement.as_mut().expect(
            "[DAOMember]: Somehow you're still haven't started the retirement process yet!",
        );
        let amount = retirement_process.withdrawable;
        retirement_process.withdrawable = Decimal::ZERO;
        amount
    }

    /// Helpful method to confiscate all of the representative commited resource if he/she is corrupted
    /// through the [rage_quit()](crate::community::Community_impl::Community::rage_quit) method.
    pub fn confiscate(&mut self) -> Decimal {
        let amount = self.committed_amount;
        self.committed_amount = Decimal::ZERO;
        self.representing = None;
        amount
    }

    /// Helpful method for the DAO to accept the member's replacement and immediately end his/her retirement process
    /// through the [accept_replacement()](crate::align_dao::DAO_impl::DAO::accept_replacement) method.
    pub fn accept_replacement(&mut self) {
        let retirement_process = self.retirement.as_mut().expect(
            "[DAOMember]: The person in question haven't started the retirement process yet!",
        );
        retirement_process.advance_all(&mut self.committed_amount)
    }
}

/// Struct store DAO member's retirement process data.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct RetirementProcess {
    /// The remain periods that the DAO member has to go through on the retirement process.
    retirement_period: u64,

    /// The amount that the DAO member can withdraw after each retirement period.
    retirement_withdraw: Decimal,

    /// The amount that the DAO member can withdraw currently
    withdrawable: Decimal,

    /// Store the end time of the current member's retirement period.
    period_end: u64,
}

impl RetirementProcess {
    /// Helpful method to create new retirement process data struct.
    ///
    /// # Input
    /// - member_data: the DAO member SBT struct data.
    /// - commitment_policy: the [Commitment Policy](crate::policies::CommitmentPolicy) that the DAO's using.
    /// - current: current time data from [Local Oracle component](crate::local_oracle::LocalOracle_impl::LocalOracle).
    /// # Output
    /// New retirement process data struct.
    fn new(member_data: &DAOMember, commitment_policy: &CommitmentPolicy, current: u64) -> Self {
        let retirement_period =
            commitment_policy.calculate_retirement_period(member_data.retirement_length) + 1; // Initial period is the period to the minimum retirement length.
        let retirement_withdraw = member_data.committed_amount / retirement_period;
        let period_end = current + commitment_policy.minimum_retirement;
        Self {
            retirement_period,
            retirement_withdraw,
            withdrawable: Decimal::ZERO,
            period_end,
        }
    }

    /// Helpful method to advance the member's retirement process according to current time data.
    ///
    /// # Input
    /// - commitment_amount: the current committed resource amount of the DAO member.
    /// - commitment_policy: the [Commitment Policy](crate::policies::CommitmentPolicy) that the DAO's using.
    /// - current: current time data from [Local Oracle component](crate::local_oracle::LocalOracle_impl::LocalOracle).
    fn advance(
        &mut self,
        commitment_amount: &mut Decimal,
        commitment_policy: &CommitmentPolicy,
        current: u64,
    ) {
        let mut passed_period = 0u64;
        let mut period_end = self.period_end;
        let mut retirement_period = self.retirement_period;

        while period_end <= current && retirement_period > 0 {
            passed_period += 1;
            retirement_period -= 1;
            period_end += commitment_policy.period_length;
        }

        info!(
            "[RetirementProcess]: You have passed {} retirement period",
            passed_period
        );

        self.retirement_period = retirement_period;
        self.period_end = period_end;

        if retirement_period == 0 {
            info!("[RetirementProcess]: You have passed all the retirement period");
            self.withdrawable += *commitment_amount;
            *commitment_amount = Decimal::ZERO;
        } else {
            let amount = self.retirement_withdraw * Decimal::from(passed_period);
            if *commitment_amount >= amount {
                *commitment_amount -= amount;
                self.withdrawable += amount;
            } else {
                self.withdrawable += *commitment_amount;
                *commitment_amount = Decimal::ZERO;
            }
        }
    }

    /// Helpful method to end the member's retirement process on the DAO's authority.
    ///
    /// # Input
    /// - commitment_amount: the current committed resource amount of the DAO member.
    fn advance_all(&mut self, commitment_amount: &mut Decimal) {
        self.retirement_period = 0;

        self.withdrawable += *commitment_amount;
        *commitment_amount = Decimal::ZERO;

        info!("[RetirementProcess]: The DAO member has been accepted to end the retirement process immediately");
    }
}
