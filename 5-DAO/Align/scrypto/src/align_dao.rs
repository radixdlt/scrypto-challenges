/*!
The blueprint implement DAO Component with many novel Decentralized Governance (DeGov) techniques that's only exist on Web3 to create solid alignment of interests
while at the same time foster individual accountability, allow flexibility between trustlessness and affordable trustness.

# Protocol's entities
- Participants / Shareholders: People who are interesting on becoming a part of the DAO and (or) holding DAO share tokens.

- [DAO members](crate::member#dao-members): People who have voting right by going through "commitment" according to the [Commitment Voting Mechanism](#commitment-voting-mechanism).
- [Representatives](crate::member#representatives): DAO members who interested in running and represent a specific Community
(Democrat, Republican, Education Department, Agriculture and Rural Development Department,...) according to [Liquid Democracy](#liquid-democracy).
- [Delegators](crate::delegator#delegators): People who don't want to go through "commitment" but still want to indirectly participate in decision making through representative choosing and receive suitable reward.
- [Communities](crate::community#communities): A community represent groups of people governed by the [Community Component](crate::community::Community_impl::Community) 
created and managed by representative.
- [DAO badge](DAO_impl::DAO::dao_badge): Represent the DAO authority as a whole, can only be
accessed from [execute()](crate::proposal::Proposal_impl::Proposal::execute) method
when a proposal is accepted or rejected according to the DAO's voting mechanism.
- [Primary reserve resource](DAO_impl::DAO::reserve):
The resource address used to be the primary reserve - the medium exchange token
that the DAO believe will have a stable value and can be used to exchange widely. (Eg: Stablecoins, Gold,...)
- [Proposals](crate::proposal#proposals): a suggestion by DAO member to execute smartcontract functions/methods
through the DAO authority on the [dao badge](DAO_impl::DAO::dao_badge). The proposal will follow the [Permissioned Relative Majority & Quorum Voting Mechanism](#permissioned-relative-majority--quorum-voting-mechanism)
- [Policies](crate::policies): Policies are all the smartcontract configs that will govern the DAO's smartcontract logic.
- [Treasury](crate::treasury): Treasury Component store and manage all the DAO's assets accoring to [Internal Treasury Function](#internal-treasury-function)
- [LocalOracle](crate::local_oracle): Local Oracle Component store the Oracle address and access badge to get real unix time data for the DAO's smartcontract logic.

# Main Features
Through a deep dive into current DeGov mechanisms, the author found 8 common DeGov techniques: Quorum Voting, Quadratic Voting, Liquid Democracy,
Rage Quitting, Holographic Consensus, Conviction Voting, Weighted Voting & Reputation-based Voting, Knowledge-extractable Voting,
Permissioned Relative Majority. Align DAO have carefully adopted and evolved suitable techniques for its design goals as follow:

## Commitment Voting Mechanism
### From Conviction Voting to Commitment Voting
The author believe that distribute of ["Conviction"](https://medium.com/giveth/conviction-voting-a-novel-continuous-decision-making-alternative-to-governance-aa746cfb9475)
 on decision making or R&D is not a good approach since the distributor might easily be biased and make wrong decision but
 still not required to take any responsibility on that decision. Thus, the most effective approach must be the distribution of
 responsibility on decision making, directly align members' interest to the DAO's interest and raise their responsibility for the DAO.

Commitment Voting mechanism is the voting model that require DAO members to commit their resource before participate in decision making. 
Voting power of DAO member will also grow progressively with their commitment history. The longer a member committed their resource on the DAO, 
the larger his/her voting power will become.

This novel approach will undoubtedly become the core technique of Align DAO and create solid [Alignment of Interest](crate#alignment-of-interest).
Through commitment, the members will thrive (evolved) or fail along with the DAO.

Although Commitment Voting is also a time-based voting mechanism, it is at a higher level than
Conviction Voting since it can promote higher level of "unity" in the DAO.

**Workflow on Align DAO:**

- To become a DAO member and directly participate in decision making, participants are required to "commit"
their resource (DAO share) on the DAO through the [become_dao_member()](DAO_impl::DAO::become_dao_member) method.
- After become a DAO member, he/she will receive a Soul Bound Token to track his/her [committed resource](crate::member::DAOMember::committed_amount)
and commitment history through [DAO Member](crate::member::DAOMember) data struct.
- The longer the person "commit" their resource, the more "voting power" he/she will
receive according to the [calculate_voting_power()](crate::member::DAOMember::calculate_voting_power) method.

### Replacement & Retirement Process
Withdrawal of an intelligent person with a large amount of resources is a big lost for the whole DAO.
To reduce mental, financial burden for the DAO on a withdrawal and create higher level Alignment of Interest,
Align DAO will include a retirement process where DAO members who commited their resource have to go
through a predetermined retirement process. On retirement process, DAO Members' resources will
be repaid on periods after a fixed amount of time.

A DAO member can retire whenever they want.
However, at the same time they would lose all their voting power from the commitment history
and have to go through the retirement process.

Representatives cannot retire when there're still people delegated to them.

Members who on retirement process can find an acceptable replacement member for the DAO,
and create [replacement proposal](DAO_impl::DAO::replacement_proposal) for the DAO to
vote on unlocking all of his/her resources immediately.

**Workflow on Align DAO:**

- When make a "commitment", the DAO member also have to determine their retirement length.
- The choosed retirement length must on the acceptable range or it would be rejected by [CommitmentPolicy](crate::policies::CommitmentPolicy::assert_retirement).
- The longer the DAO member determine their retirement process, the more "voting weight"
he/she will receive through his/her commitment history according to the
[calculate_voting_power()](crate::member::DAOMember::calculate_voting_power) method.
- DAO members can always begin their retirement process through the [retire()](DAO_impl::DAO::retire) method.
- Retirement history will be stored on [RetirementProcess](crate::member::RetirementProcess) struct.
- If the DAO member on retirement process find an acceptable replacement member for the DAO,
he/she then can create a proposal for the DAO to vote on unlocking all of his/her resources immediately
through the [replacement_proposal()](DAO_impl::DAO::replacement_proposal) method.

## Liquid Democracy
Liquid democracy is a form of delegative democracy,
whereby an electorate engages in collective decision-making through direct participation and dynamic representation.
This democratic system utilizes elements of both direct and representative democracy.

Voters in a liquid democracy have the right to vote directly on all policy issues - direct democracy;
voters also have the option to delegate their votes to someone who will vote on their behalf - representative democracy.

Not everyone want to participate in decision making, R&D or have enough wisdom to do so.
Thus, there must be a group act as the DAO's MIND, contain the most intelligent representative for decision making,
R&D and allow other people to vote on their behalf.

The Liquid Democracy design on Align DAO must ensure the representative can take
reponsibility for their actions and distribute suitable reward for their work.
At the same time, the representative will have to show his/her wisdom, ability to earn people's trust.

This approach will reduce mental burden of delegator from decision making to representative choosing
and undoubtedly allow [Affordable Trustness](crate#affordable-trustness) on the DAO.

**Workflow on Align DAO:**

- DAO members who meet the vote power requirement according to [CommunityPolicy](crate::policies::CommunityPolicy::check_requirement) can host a "Community" and
become a representative through the [become_representative()](DAO_impl::DAO::become_representative) method.
This will create a new [Community component](crate::community#communities).
- Other participants can join and delegate their vote into the "Community" to become Community's member and indirectly
participate in decision making through the community component.

### From Reputation-based Voting to Power Taxing
Power cannot be quantified on real world.
However, since power "can be" quantified on digital world or specifically "Web3",
more than just increase a person voting power according to his/her reputation,
the author came up with a novel concept called "Power Taxing".

Power Taxing will give more vote power for representative from a percent of each community member's vote power.
The more vote power the representative has, the more reward he/she will receive for participating on DeGov.

**Workflow on Align DAO:**

- A representative can always "amend the vote power tax" on his Community through the [amend_tax_policy()](crate::community::Community_impl::Community::amend_tax_policy) method.
- All current members of the Community will give up a part of his/her vote power for the representative according to the tax.
- Community members can always quit the Community and reclaim their vote power at any time through the [quit()](crate::community::Community_impl::Community::quit) method.

### Rage Quiting & Credibility Score System
Liquid Democracy might reduce mental burden for people on decision making process.
However, it would introduce centralization and the DAO might risk from corruption, the representative might act against the Community's benefit.

To address the problem, Align DAO introduce a "Credibility Score System" for the representatives.
Community member who are unhappy with the representative's action can always do "Rage Quiting" and thus degrade the representative's credibility.

A DAO member when become representative will have an initial credibility score.
The lower the credibility score, the lower the vote power of the representative.

When the credibility score reach 0, all the representative's resource will be confiscated and burned,
the community on the representative's lead will also be removed from the DAO.

People who do rage quitting will also get his/her on-going indirect vote by the community retracted.

Since this approach (and Quadratic Voting) is highly risk from Sybil attack, representatives would need delegators to prove their identity when joining the Community
(This could be done through decentralized service like Proof of Humanity, BrightID or through centralized service (google account, twitter,...) like how
    [GitCoin](https://go.gitcoin.co/gitcoin-passport-0?utm_source=gitcoinco&utm_medium=referral&utm_campaign=topnav&utm_content=Passport) is doing).

On the other hand, if the representative somehow act agaisnt the community members' benefit,
they could always have their credibility degraded and potentially lead to the lost of all representative's resources.
This approach will also ensure unwealthies' voice are hearded equally and thus more effective than Quadratic Voting technique.

**Workflow on Align DAO:**

- Participants can prove their identity and request to join any community through the [request_join()](crate::community::Community_impl::Community::request_join) method.
- Representative can review participants' request through the [review_request()](crate::community::Community_impl::Community::review_request) method.
- After accepted, participants can choose between communities on which is best for them to join through the [join()](crate::community::Community_impl::Community::join) method.
- When joined, if the representative somehow act agaisnt the community members' benefit, the community member can do rage quitting through the [rage_quit()](DAO_impl::DAO::rage_quit) method.
- The rage quitted community member will also get his/her on-going indirect vote by the community retracted.
- When the Community's credibility reach 0, all the representative's resource will immediately be confiscated and burned.

## Permissioned Relative Majority & Quorum Voting Mechanism

Relative majority voting mechanism compares the total number of votes of those supporting
and those against to arrive at a decision after an arranged time.

With the Commitment Voting Mechanism,
Align DAO will compares the total voted weight of those
supporting and those against to arrive at a decision.

Permissioned on Align DAO mean only DAO members meet the vote power requirement can make a proposal.
This will further prevent malicious proposal since DAO members will also
need to have suitable commitment history before a proposal.

In addition, Align DAO will use a delay time on each proposal to help the DAO process through them
in the most effective way within the time range. Therefore, DAO members will have enough
time to review proposals and make right decisions.

After the time delay, proposals will not accept more vote by any mean.

Moreover, to address the risk of un-noticed malicious proposals, a proposal can only be executed
(or rejected) after the voted power passed a predetermined quorum.

In conclusion, a proposal will be accepted only when meeting these three requirements:
- Passed the proposal time delay.
- Passed the predetermined quorum.
- Support voted power > Against voted power.

**Workflow on Align DAO:**

- A DAO member (or representative) who meet the vote power requirement according to [ProposalPolicy](crate::policies::ProposalPolicy::check_requirement) can create a new proposal through the
[new_proposal()](DAO_impl::DAO::new_proposal) method. This will create new [Proposal component](crate::proposal#proposals).
- Other DAO members (or representatives) can then vote on the proposal through the
[vote()](crate::proposal::Proposal_impl::Proposal::vote) method.
- After the proposal end time (or time delay), anyone can call the [execute()](crate::proposal::Proposal_impl::Proposal::execute)
method on the proposal to execute the proposal. The proposal is only accepted when meeting the above requirements.

## Internal Treasury Function
### Normal Treasury Function
The internal treasury will inevitably become the main life-line of the DAO. Align DAO's internal treasury will have normal treasury functions:
the DAO can use the treasury per accepted proposal; debtor (or smartcontract) can deposit directly on the treasury for the DAO's income.

The treasury also added a security layer for withdrawal called [withdraw threshold](crate::policies::TreasuryPolicy::withdraw_threshold): DAO collective actions cannot withdraw from the treasury more than the 
withdraw threshold on a predetermined [time period](crate::policies::TreasuryPolicy::withdraw_period). 
This extra security feature is particularly for centralized DAO where a small group DAO members can vote against the benefit of the whole DAO and the most vulnerable attack target would be the treasury.
When there's such an attack, the attack group cannot withdraw all the resouces (at maximum only the withdraw threshold) before all other token holders, members have already drained all the treasury.

**Workflow on Align DAO:**
- DAO's collective action can access and withdraw treasury resource through the [dao_withdraw()](DAO_impl::DAO::dao_withdraw) or [dao_withdraw_by_amount()](DAO_impl::DAO::dao_withdraw_by_amount) method. 
The [withdraw threshold](crate::policies::TreasuryPolicy::withdraw_threshold) will prevent DAO's collective action from withdraw more than the threshold for a [time period](crate::policies::TreasuryPolicy::withdraw_period).
- Anyone can deposit any resource into the DAO through the [deposit()](DAO_impl::DAO::deposit) method.

### Internal decentralized share market
The treasury will implement an AMM mechanism for it to work as a decentralized share market: people can swap between the DAO share and a choosed primary reserve resource.
This feature is for any shareholder can immediately take interest from the DAO's income whenever they want. The initial share allocation for treasury is fixed, people cannot add any share into this treasury by any mean other than swapping.
Therefore, if the DAO has good and stable income, all shareholders will be rewarded from raising price of the share. Likewise, if the DAO has many bad debt, all shareholders will suffer from declined share price.
This is also to ensure high [Alignment of Interest](crate#alignment-of-interest) from shareholders, they will all thrive or fail along with the DAO.

**Workflow on Align DAO:**
- Anyone can swap between the DAO share token and the primary reserve resource through the [swap()](DAO_impl::DAO::swap) method.
- People who use the DAO's decentralized share market have to pay a fee according to [TreasuryPolicy](crate::policies::TreasuryPolicy::swap_fee).

### Rage Withdraw
Rage Withdraw is another novel DeGov concept that the author evolved from Vitalik's hybrid futarchy "vote as buy order" [governance idea](https://ethresear.ch/t/votes-as-buy-orders-a-new-type-of-hybrid-coin-voting-futarchy/10305)
to solve "tragedy of the commons" problem and achieve high level of [Individual Accountability](crate::individual-accountability). While it's not strictly have voter to make buy order, the author "Rage Withdraw" idea
will instead fork the treasury's state before a proposal is accepted and write the reserve amount that against voters can do "Rage Withdraw".

All the people who's unhappy with a DAO's decision can always do Rage Withdraw and thus don't have to take accountability about that decision. 
At the same time, support voters would need to endure more risk and responsibility.

This novel concept not only solves the tragedy of the commons problem by introducing individual accountability into voting, but also give more dynamicity for the DAO. It will also highly restrict
such attack like "vote buying", since even if they bought vote to implement such an attack, the share price would be really high and people who do rage withdraw are the ones who's benefit on the proposal's execution time.

Moreover, to strengthen the importance of Retirement Process which made "great power come great responsibility",
 DAO members who do rage withdraw will not benefit from the forked reserve wholly but will be charged a part of their resource based on the retirement process they have to go through.

**Workflow on Align DAO:**
- All the DAO members or Delegators who voted against an accepted proposal will have a fork of his/her account based by the primary resource on the time right before the proposal is executed through 
[execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
- Since there will be a [withdraw threshold](crate::policies::TreasuryPolicy::withdraw_threshold), the treasury cannot be drained all immediately on any proposal, 
all the people who's unhappy with the DAO's decision can have plenty of time to do [rage withdraw](DAO_impl::DAO::rage_withdraw).
- Rage withdraw method will not be restricted by the withdraw threshold.

### Asset Distribution Process

Any fungible asset isn't DAO share resource or primary reserve resource can always be voted into a distribution process,
where the support voter can receive a part of the asset amount proportional to their voting power.

This will also strengthen the fork between support voters and against voters
since only support voters on accepted proposal can receive both dividend and asset distribution reward,
while at the same time, against voters can always do Rage Withdraw and take their forked reserve account.

**Workflow on Align DAO:**
- DAO Member with required vote power can create a proposal to distribute a resource through the [new_proposal()](DAO_impl::DAO::new_proposal) method.
- On acceptance, the proposal will withdraw the resource and store it on [proposal component](crate::proposal::Proposal_impl::Proposal::distribution) through the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
- Support voters can then withdraw their distribution share proportional to their voting power through the [take_distribution()](crate::proposal::Proposal_impl::Proposal::take_distribution) method.

## Functions, Methods Overview & Transaction manifests
### Function
- Function [new()](DAO_impl::DAO::new): Instantiate new DAO Component.

*No transaction manifest since the function will be called through the [AlignProject blueprint](crate::align_project).*
### General methods: 
Methods for anyone to interact with the DAO. Transaction manifests are on sub-group `rtm/dao/general/`.

- Method [deposit()](DAO_impl::DAO::deposit):
The method allow DAO members or the protocols it's running (or any donator)
to deposit stable coin directly into the treasury.
- Method [swap()](DAO_impl::DAO::swap):
The method allow anyone to swap between DAO's share 
and the primary reserve resource according to 
[Internal decentralized share market](#internal-decentralized-share-market).
- Method [become_dao_member()](DAO_impl::DAO::become_dao_member):
The method allow anyone to make a commitment with an amount of DAO share and become a DAO member.
- Method [become_delegator()](DAO_impl::DAO::become_delegator):
The method allow anyone to delegate an amount of DAO share to indirectly participate on decision making through a community run by a representative.

### Member/Delegator methods: 
Methods can only be called by DAO Member/Delegator. Transaction manifests are on sub-group `rtm/dao/member_delegator/`.

- Method [withdraw_by_amount()](DAO_impl::DAO::withdraw_by_amount):
The method allow DAO members or delegators to
withdraw their delegated/withdrawable resource by amount.
- Method [withdraw_all()](DAO_impl::DAO::withdraw_all):
The method allow DAO members or delegators to 
 withdraw all their their delegated/withdrawable resource.
- Method [rage_withdraw()](DAO_impl::DAO::rage_withdraw):
he method allow DAO members or delegators 
to do [rage withdraw](#rage-withdraw).

- Method [show_vote_power()](DAO_impl::DAO::show_vote_power):
Read method, allow DAO members or delegators to 
check their current vote power.
- Method [show_account()](DAO_impl::DAO::show_account):
Read method, allow DAO members or delegators to 
check their current account state.
- Method [check_community()](DAO_impl::DAO::check_community):
Read method, allow DAO members or delegators to 
check their current following community.

- Method [claim_dividend()](DAO_impl::DAO::claim_dividend):
The method allow DAO members or delegators to claim their dividend reward.
- Method [rage_quit()](DAO_impl::DAO::rage_quit):
The method allow DAO member or delegator to 
[rage quit](#rage-quiting-&-credibility-score-system) their community.


### Delegator only method
Method can only be called by Delegator. Transaction manifest is on sub-group `rtm/dao/delegator_only/`.

- Method [add_delegate()](DAO_impl::DAO::add_delegate):
The method allow delegator to add resource to their current account.

### DAO Member only methods
Methods can only be called by DAO Member. Transaction manifests are on sub-group `rtm/dao/member_only/`.

- Method [accumulate_dividend()](DAO_impl::DAO::accumulate_dividend):
The method allow DAO members to re-accumulate dividend reward 
on their current SBT account for more vote weight.
- Method [become_representative()](DAO_impl::DAO::become_representative):
The method allow DAO member to create a new community 
and become the representative of that community according to 
[Liquid Democracy](#liquid-democracy).
- Method [retire()](DAO_impl::DAO::retire):
The method allow DAO members to begin (or advance) 
their [retirement process](#replacement-&-retirement-process) 
and settle any withdrawable resource.
- Method [replacement_proposal()](DAO_impl::DAO::replacement_proposal):
The method will allow DAO members on retirement process to propose a replament for his/her to end his/her current retirement process.

### Proposal method
Method to create a new configurable proposal, also can only be called by DAO Member. Transaction manifests are on sub-group `rtm/dao/proposal/`.

- Method [new_proposal()](DAO_impl::DAO::new_proposal):
The method will allow DAO members to propose a concept for others to vote on.

### Proposal Execution intra-package call only
Methods can only be called through the execute method from Proposal component. 
Some malicious test transasction manifests to call these methods are on sub-group `rtm/dao/malicious/`.

- Method [dao_proof()](DAO_impl::DAO::dao_proof):
The method will allow access the dao badge, not user callable.

- Method [accepted()](DAO_impl::DAO::dao_proof):
The method will mint dividend reward, allocate fund for the 
accepted proposal and remove the accepted proposal, not user callable.
- Method [rejected()](DAO_impl::DAO::dao_proof):
The method will burn slashed resource
 and remove the rejected proposal, not user callable.
- Method [ignored()](DAO_impl::DAO::dao_proof):
The method will remove the ignored proposal, not user callable.

- Method [accept_replacement()](DAO_impl::DAO::dao_proof):
The method will allow the DAO Member with the provided ID to end his/her retirement process immediately, not user callable.

- Method [dao_withdraw_by_amount()](DAO_impl::DAO::dao_withdraw_by_amount):
The method allow withdraw resource by amount directly from the treasury, not user callable.
- Method [dao_withdraw()](DAO_impl::DAO::dao_withdraw):
The method allow withdraw resource directly from the treasury, not user callable.

#### DAO's Amendment methods
Methods to change the DAO's policies, configs, also can only be called through the execute method from Proposal component.

- Method [change_oracle()](DAO_impl::DAO::change_oracle):
The method allow the DAO to change its on-using oracle, not user callable.

- Method [amend_treasury_policy()](DAO_impl::DAO::amend_treasury_policy):
The method allow the DAO to amend its treasury policy, not user callable.
- Method [amend_economic_policy()](DAO_impl::DAO::amend_economic_policy):
The method allow the DAO to amend its economic policy, not user callable.
- Method [amend_commitment_policy()](DAO_impl::DAO::amend_commitment_policy):
The method allow the DAO to amend its commitment policy, not user callable.
- Method [amend_proposal_policy()](DAO_impl::DAO::amend_proposal_policy):
The method allow the DAO to amend its proposal policy, not user callable.

### Read only method
Transaction manifests are on sub-group `rtm/dao/read_only/`

- Method [get_price()](DAO_impl::DAO::get_price):
Read only method, allow anyone to get the current price between DAO's share and the primary reserve resource.
- Method [get_rage_withdraw()](DAO_impl::DAO::get_rage_withdraw):
Read only method, allow anyone to get the treasury rage withdraw policy
- Method [check_dividend()](DAO_impl::DAO::check_dividend):
Read only method, allow anyone to check current un-claimed dividend amount on the DAO.
- Method [get_community_address()](DAO_impl::DAO::get_community_address):
Read only method, for going around current Scrypto's [known bug](https://github.com/radixdlt/radixdlt-scrypto/issues/483)
### Internal method
- Method [current()](DAO_impl::DAO::current):
Internal method to get current data from the oracle, not user callable.
*/
use crate::community::CommunityComponent;
use crate::delegator::Delegator;
use crate::local_oracle::LocalOracleComponent;
use crate::member::DAOMember;
use crate::policies::*;
use crate::proposal::*;
use crate::treasury::TreasuryComponent;
use crate::utils::*;

use scrypto::prelude::*;

/// The proposal badge for the DAO member who propose a 
/// funded proposal. The proposal badge can be burned later to get the allocated fund.
#[derive(NonFungibleData)]
pub struct ProposalBadge {}

blueprint! {

    /// DAO Component is the core component that will govern the whole DAO created by Align blueprint package.
    struct DAO {

        /// DAO controller badge, the badge provide access rule to:
        /// - Mint Delegator, DAO Member SBT;
        /// - Update Delegator, DAO Member SBT data;
        /// - Mint proposal badge, community controller badge, proposal controller badge.
        /// - Call [current()](crate::local_oracle::LocalOracle_impl::LocalOracle::current) method to get current time from the local oracle.
        ///
        /// The badge cannot be withdraw or used for any other purpose execept for supporting the DAO's smartcontract logic.
        controller_badge: Vault,
        /// Hold the dao badge to represent the DAO authority for implementing collective actions of the DAO. This badge also hold the dao share token's minting, burning authority.
        dao_badge: Vault,
        /// The local treasury component.
        treasury: TreasuryComponent,
        /// The local oracle component address
        oracle_address: ComponentAddress,
        /// The resource address used to be the primary reserve - the medium exchange token that the DAO believe will have a stable value and can be used to exchange widely. (Eg: Stablecoins, Gold,…)
        reserve: ResourceAddress,
        /// The Community Components created by DAO members,
        ///
        /// - Key: Community Name
        /// - Value: Community Component
        communities: HashMap<String, ComponentAddress>,

        /// The DAO's commitment vault, store delegated, committed DAO share.
        commitment_vault: Vault,
        /// The DAO's dividend vault, store dividend reward of participants.
        dividend_vault: Vault,

        /// The DAO member SBT resource address.
        member_sbt: ResourceAddress,
        /// The DAO delegator SBT resource address.
        delegator_sbt: ResourceAddress,
        /// Proposal controller badge resource address, the badge provide access rule to:
        /// - Update DAO Member/Delegator's SBT data;
        /// - Call [dao_proof()](crate::align_dao::DAO_impl::DAO::dao_proof) method to access the dao badge;
        /// - Call [ignored()](crate::align_dao::DAO_impl::DAO::ignored) method to remove the proposal from the DAO if the proposal is ignored;
        /// - Call [current()](crate::local_oracle::LocalOracle_impl::LocalOracle::current) method to get current time from the local oracle;
        /// - Burn [proposal badge](crate::align_dao::ProposalBadge).
        ///
        /// The badge cannot be withdraw or used for any other purpose execept for supporting the DAO's smartcontract logic.
        proposal_controller_badge: ResourceAddress,
        /// Community controller badge, the badge provide access rule to update DAO Member/Delegator's SBT data.
        ///
        /// The badge cannot be withdraw or used for any other purpose execept for supporting the DAO's smartcontract logic.
        community_controller_badge: ResourceAddress,

        /// The list keep track of on-going DAO's proposals.
        proposals: HashMap<NonFungibleId, ComponentAddress>,
        /// The proposal badge address.
        proposal_badge: ResourceAddress,
        /// The proposal id counter.
        proposal_id_counter: u64,

        /// A field store [EconomicPolicy].
        economic_policy: EconomicPolicy,

        /// A field store [CommitmentPolicy] for the Commitment Voting Mechanism
        commitment_policy: CommitmentPolicy,

        /// A field store [CommunityPolicy] for Liquid Democracy
        community_policy: CommunityPolicy,

        /// A field store [ProposalPolicy] for Permissioned Relative Majority & Quorum Voting Mechanism
        proposal_policy: ProposalPolicy,

        /// This field is just for going around current Scrypto's [known bug](https://github.com/radixdlt/radixdlt-scrypto/issues/483)
        communities_by_id: HashMap<NonFungibleId, ComponentAddress>,
    }

    impl DAO {

        /// This function will create new DAO component with many advanced DeGov techniques.
        /// # Input
        /// - name: the organization's name.
        ///
        /// *Treasury input*
        /// - dao_badge: DAO badge represent the DAO authority for implementing collective actions of the DAO. This badge also hold the DAO share token's minting, burning authority.
        /// - dao_share: the DAO share initial allocation for treasury.
        /// - primary_reserve_tokens: the primary reserve token initial allocation for treasury.
        /// 
        /// *[Treasury policy](crate::policies::TreasuryPolicy) input*
        /// - swap_fee: initial internal treasury swap fee (%). Must be in range from 0 to 100.
        /// - withdraw_threshold: the fixed withdraw threshold, DAO's collective actions cannot withdraw more than this threshold (%) on a fixed time period. Must be in range from 0 to 100.
        /// - withdraw_period: the primary reserve resource withdraw time period (seconds). 
        /// - rage_withdraw_decline_multiply: the rage withdraw decline multiply for DAO member based on their retirement process.
        /// - rage_withdraw_time_limit: the time limit range for rage withdraw after a proposal in conflict got accepted
        ///
        /// *[Economic policy](crate::policies::EconomicPolicy) input*
        /// - dividend: the initial DAO share dividend amount for each successful executed proposal (DAO share/proposal).
        /// - slash_rate: the initial DAO share slash rate for each failed attempt on a proposal with low agreement rate (%/action). Must be in range from 0 to 100.
        ///
        /// *[Commitment policy](crate::policies::CommitmentPolicy) input*
        /// - initital_commitment_rate: the initial voting power increase rate for each week DAO member commited on contributing for the DAO (%). Must be in range from 0 to 100.
        /// - minimum_retirement: the initial minimum retirement length a DAO member must have on his/her commitment.
        /// 
        /// HIGHLY RECOMMENDED to make this config input high (Recommend at least >= 1 Month, ~ 2,678,400 seconds) since low retirement length will make the DAO vulnerable to vote-reentrancy.
        /// - maximum_retirement: the initial maximum retirement length a DAO member can have on his/her commitment.
        /// - commitment_grow_rate: the initial commitment grow rate per period (%). The more period that the DAO member has, the more vote weight he/she will also get accumulating by commitment time. Must be in range from 0 to 100.
        /// - maximum_vote_rate: the initial maximum vote power multiply rate that a member can achieve after their long commitment (%). Must be equal or greater than 0.
        /// - period_length: the initial period length where a retired member can get back his/her committed resource on each period after passed the minimum retirement length.
        ///
        /// *[Community policy](crate::policies::CommunityPolicy) input*
        /// - initial_credibility: the initial credibility score for a new representative
        /// - representative_requirement: the initial vote power requirement for a DAO member before become a representative
        ///
        /// *Oracle*
        /// - oracle: initial oracle component address and the data badge.
        ///
        /// *[Proposal policy](crate::policies::ProposalPolicy) input*
        /// - proposal_requirement: initial vote power requirement for a DAO member when making a proposal.
        /// - proposal_quorum: initial threshold that total voted power on a proposal must pass before the proposal can be executed (or rejected).
        /// - proposal_minimum_delay: minimum time delay of a proposal.
        ///
        /// # Smartcontract logic
        /// ## Panics
        /// The function will panic if the input params is not in suitable range.
        pub fn new(
            name: String,

            dao_badge: Bucket,
            dao_share: Bucket,
            primary_reserve_tokens: Bucket,
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

            oracle: (ComponentAddress, Bucket),

            proposal_requirement: Decimal,
            proposal_quorum: Decimal,
            proposal_minimum_delay: u64,
        ) -> ComponentAddress {

            assert_rate(swap_fee);
            assert_rate(withdraw_threshold);
            assert_rate(slash_rate);
            assert_rate(initital_commitment_rate);
            assert_rate(commitment_grow_rate);

            assert!(maximum_vote_rate >= Decimal::ZERO, "Wrong data!");

            let controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", name.clone() + "'s DAO Controller Badge")
                .restrict_withdraw(AccessRule::DenyAll, LOCKED)
                .initial_supply(dec!(1));

            let controller_badge_address = controller_badge.resource_address();

            let proposal_controller_badge = ResourceBuilder::new_fungible()
                .metadata("name", name.clone() + "'s DAO Proposal Controller Badge")
                .mintable(rule!(require(controller_badge_address)), LOCKED)
                .restrict_withdraw(AccessRule::DenyAll, LOCKED)
                .no_initial_supply();

            let community_controller_badge = ResourceBuilder::new_fungible()
                .metadata("name", name.clone() + "'s DAO Community Controller Badge")
                .mintable(rule!(require(controller_badge_address)), LOCKED)
                .restrict_withdraw(AccessRule::DenyAll, LOCKED)
                .no_initial_supply();

            let member_sbt = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + " DAO Member SBT")
                .mintable(rule!(require(controller_badge_address)), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require_any_of(vec![
                        controller_badge_address,
                        proposal_controller_badge,
                        community_controller_badge
                    ])),
                    LOCKED,
                )
                .restrict_withdraw(AccessRule::DenyAll, LOCKED)
                .no_initial_supply();

            info!("[AlignDAO]: DAO Member SBT address: {}", member_sbt);

            let delegator_sbt = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + " DAO Delegator SBT")
                .mintable(rule!(require(controller_badge_address)), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require_any_of(vec![
                        controller_badge_address,
                        proposal_controller_badge,
                        community_controller_badge
                    ])),
                    LOCKED,
                )
                .restrict_withdraw(AccessRule::DenyAll, LOCKED)
                .no_initial_supply();

            info!("[AlignDAO]: Delegator SBT address: {}", delegator_sbt);

            let proposal_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name + " DAO Proposal Badge")
                .mintable(rule!(require(controller_badge_address)), LOCKED)
                .burnable(rule!(require(proposal_controller_badge)), LOCKED)
                .no_initial_supply();

            info!("[AlignDAO]: Proposal badge address: {}", proposal_badge);

            let dao_method_rules = AccessRules::new()
                .method("get_community_address", AccessRule::AllowAll) // This is just for going around current Scrypto's [known bug](https://github.com/radixdlt/radixdlt-scrypto/issues/483)
                .method("get_price", AccessRule::AllowAll)
                .method("get_rage_withdraw", AccessRule::AllowAll)
                .method("check_dividend", AccessRule::AllowAll)
                .method("swap", AccessRule::AllowAll)
                .method("deposit", AccessRule::AllowAll)
                .method("become_dao_member", AccessRule::AllowAll)
                .method("become_delegator", AccessRule::AllowAll)
                .method("withdraw_by_amount", AccessRule::AllowAll)
                .method("withdraw_all", AccessRule::AllowAll)
                .method("rage_withdraw", AccessRule::AllowAll)
                .method("show_vote_power", AccessRule::AllowAll)
                .method("show_account", AccessRule::AllowAll)
                .method("check_community", AccessRule::AllowAll)
                .method("rage_quit", AccessRule::AllowAll)
                .method("add_delegate", AccessRule::AllowAll)
                .method("claim_dividend", AccessRule::AllowAll)
                .method("accumulate_dividend", AccessRule::AllowAll)
                .method("retire", AccessRule::AllowAll)
                .method("new_proposal", AccessRule::AllowAll)
                .method("replacement_proposal", AccessRule::AllowAll)
                .method("become_representative", AccessRule::AllowAll)
                .method("dao_proof", rule!(require(proposal_controller_badge)))
                .method("ignored", rule!(require(proposal_controller_badge)))
                .default(rule!(require(dao_badge.resource_address())));

            let dao_share_address = dao_share.resource_address();

            let mut local_oracle = LocalOracleComponent::new(oracle.0, oracle.1);

            local_oracle.add_access_check(
                AccessRules::new()
                    .method("refund_oracle", AccessRule::AllowAll)
                    .default(rule!(require_any_of(vec![
                        controller_badge_address,
                        proposal_controller_badge
                    ]))),
            );

            let oracle_address = local_oracle.globalize();

            info!(
                "[AlignDAO]: Local Oracle Component address: {}",
                oracle_address
            );

            let reserve = primary_reserve_tokens.resource_address();

            let mut comp = Self {
                controller_badge: Vault::with_bucket(controller_badge),
                dao_badge: Vault::with_bucket(dao_badge),
                treasury: TreasuryComponent::new(
                    dao_share,
                    primary_reserve_tokens,
                    TreasuryPolicy {
                        swap_fee: swap_fee / dec!(100),
                        withdraw_threshold: withdraw_threshold / dec!(100),
                        withdraw_period,
                        rage_withdraw_decline_multiply,
                        rage_withdraw_time_limit,
                    },
                ),
                oracle_address,
                reserve,
                communities: HashMap::new(),

                commitment_vault: Vault::new(dao_share_address),
                dividend_vault: Vault::new(dao_share_address),

                member_sbt,
                delegator_sbt,
                proposal_controller_badge,
                community_controller_badge,

                proposals: HashMap::new(),
                proposal_badge,
                proposal_id_counter: 0,

                economic_policy: EconomicPolicy {
                    dividend,
                    slash_rate: slash_rate / dec!(100),
                },

                commitment_policy: CommitmentPolicy {
                    initital_commitment_rate: initital_commitment_rate / dec!(100),
                    minimum_retirement,
                    maximum_retirement,
                    commitment_grow_rate: commitment_grow_rate / dec!(100),
                    maximum_vote_rate: maximum_vote_rate / dec!(100),
                    period_length,
                },

                community_policy: CommunityPolicy {
                    initial_credibility,
                    representative_requirement,
                },

                proposal_policy: ProposalPolicy {
                    proposal_requirement,
                    proposal_quorum,
                    proposal_minimum_delay,
                },

                communities_by_id: HashMap::new(),
            }
            .instantiate();

            comp.add_access_check(dao_method_rules);
            return comp.globalize();
        }

        /// The method allow DAO members or the protocols it's running (or any donator)
        /// to deposit stable coin directly into the treasury
        /// # Input
        /// - bucket: Stablecoin bucket
        ///
        /// # Access Rule
        /// Anyone can call this method
        /// 
        /// # Smartcontract logic
        /// ## Intra-package access
        /// - Access [deposit()](crate::treasury::Treasury_impl::Treasury::deposit) method from Treasury component.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/general/deposit.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/general/deposit.rtm")]
        /// ```
        pub fn deposit(&mut self, bucket: Bucket) {
            self.treasury.deposit(bucket)
        }

        /// The method allow anyone to swap between DAO's share 
        /// and the primary reserve resource according to [Internal decentralized share market](#internal-decentralized-share-market).
        /// 
        /// # Input
        /// - bucket: the bucket contain resource the caller want to swap
        /// # Output
        /// The swapped resource bucket.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Intra-package access
        /// - Access [auto_swap()](crate::treasury::Treasury_impl::Treasury::auto_swap) method from Treasury component.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/general/swap.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/general/swap.rtm")]
        /// ```
        pub fn swap(&mut self, bucket: Bucket) -> Bucket {
            self.treasury.auto_swap(bucket)
        }

        /// The method will allow anyone to make a commitment with an amount of DAO share and become a DAO member.
        ///
        /// The method follow the [Commitment Voting Mechanism](#commitment-voting-mechanism).
        /// 
        /// One account address can get more than one DAO member SBT.
        /// # Input
        /// - commitment_bucket: Bucket contain the DAO share resource that the new member want to commit.
        /// - retirement_length: The retirement length that the new member want to agree on.
        /// # Output
        /// - The DAOMember SBT
        /// # Access Rule
        /// Anyone can call this method.
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong commitment resource provided.
        /// - The provided retirement length is not on the acceptable range according to the [CommitmentPolicy](crate::policies::CommitmentPolicy)
        /// ## Intra-package access
        /// - Access helpful [assert_retirement()](crate::policies::CommitmentPolicy::assert_retirement) method from CommitmentPolicy struct.
        /// - Use method [new()](crate::member::DAOMember::new) to create new DAOMember SBT data struct.
        /// - Use write method [calculate_voting_power()](crate::member::DAOMember::calculate_voting_power) to calculate the DAO member voting power.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/become_member.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/general/become_member.rtm")]
        /// ```
        pub fn become_dao_member(
            &mut self,
            commitment_bucket: Bucket,
            retirement_length: u64,
        ) -> Bucket {
            assert!(
                commitment_bucket.resource_address() == self.commitment_vault.resource_address(),
                "[AlignDAO]: Wrong commitment resource!"
            );

            self.commitment_policy.assert_retirement(retirement_length);

            let commitment_amount = commitment_bucket.amount();
            self.commitment_vault.put(commitment_bucket);

            let mut member_data = DAOMember::new(
                commitment_amount,
                retirement_length,
                &self.commitment_policy,
                self.current(),
            );

            let voting_power =
                member_data.calculate_voting_power(&self.commitment_policy, self.current());

            info!(
                "[AlignDAO]: You have become a new DAO member with voting power {}",
                voting_power
            );

            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.member_sbt)
                    .mint_non_fungible(&NonFungibleId::random(), member_data)
            })
        }

        /// The method allow anyone to delegate an amount of DAO share to indirectly participate on decision making through a community run by a representative.
        ///
        /// The method follow [Liquid Democracy](#liquid-democracy).
        /// # Input
        /// - delegate_bucket: Bucket contain the DAO share resource that the new delegator want to delegate.
        /// # Output
        /// The Delegator SBT
        /// # Access Rule
        /// Anyone can call this method.
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong delegate resource provided.
        /// ## Intra-package access
        /// - Use method [new()](crate::delegator::Delegator::new) to create new Delegator SBT data struct.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/become_delegator.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/general/become_delegator.rtm")]
        /// ```
        pub fn become_delegator(&mut self, delegate_bucket: Bucket) -> Bucket {
            assert!(
                delegate_bucket.resource_address() == self.commitment_vault.resource_address(),
                "[AlignDAO]: Wrong delegate resource!"
            );

            let delegated_amount = delegate_bucket.amount();
            self.commitment_vault.put(delegate_bucket);

            let delegator_data = Delegator::new(delegated_amount);

            info!(
                "[AlignDAO]: You have delegated {} into the DAO",
                delegated_amount
            );

            let delegator_sbt = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.delegator_sbt)
                    .mint_non_fungible(&NonFungibleId::random(), delegator_data)
            });

            delegator_sbt
        }

        /// The method allow DAO members or delegators to
        ///  withdraw their delegated/withdrawable resource by amount
        ///
        /// # Input
        /// - identity: the delegator/DAO member SBT proof.
        /// - amount: resource that the delegator/DAO member wish to withdraw from their account.
        /// # Output
        /// The bucket contain the withdrawed resource
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member/Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - Don't have enough delegated/withdrawable resource on the user SBT.
        /// - The DAO Member haven't begin the retirement process through the [retire()](DAO_impl::DAO::retire) method.
        /// - The Delegator is currently voting on some proposals.
        /// ## Intra-package access
        /// - Access field [retirement](crate::member::DAOMember::retirement) to check if the DAO Member has retired or not.
        /// - Access write method [retire()](crate::member::DAOMember::retire) to advance DAO Member's retirement process.
        /// - Access write method [withdraw_by_amount()](crate::member::DAOMember::withdraw_by_amount) to withdraw from DAO Member's account.
        /// - Access and write field [delegated_amount](crate::delegator::Delegator::delegated_amount) to withdraw the Delegator's resource.
        /// - Write field [rage_withdrawable](crate::delegator::Delegator::rage_withdrawable) to make the Delegator's unable to do rage withdraw after having withdrawed normally.
        /// - Access read only method [is_not_voting()](crate::delegator::Delegator::is_not_voting) to check if the Delegator is currently voting on any proposal or not.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_delegator/withdraw_by_amount.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_delegator/withdraw_by_amount.rtm")]
        /// ```
        pub fn withdraw_by_amount(&mut self, identity: Proof, amount: Decimal) -> Bucket {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[AlignDAO]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let mut member_data = member_sbt.data();

                assert!(
                    member_data.retirement.is_some(),
                    "[AlignDAO]: You must begin your retirement process first!"
                );

                member_data.retire(&self.commitment_policy, self.current());
                member_data.withdraw_by_amount(amount);

                self.controller_badge.authorize(|| {
                    member_sbt.update_data(member_data);
                });

                info!(
                    "[AlignDAO]: Withdrawed {} DAO share from your account",
                    amount
                );

                self.commitment_vault.take(amount)
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let mut delegator_data = delegator_sbt.data();

                assert!(
                    delegator_data.delegated_amount >= amount,
                    "[AlignDAO]: Don't have enough resource on account."
                );

                assert!(delegator_data.is_not_voting(&self.proposals), "[AlignDAO]: You can only withdraw if you're currently not voting in any proposal.");
                delegator_data.delegated_amount -= amount;
                delegator_data.rage_withdrawable = None;

                self.controller_badge.authorize(|| {
                    delegator_sbt.update_data(delegator_data);
                });

                info!(
                    "[AlignDAO]: Withdrawed {} DAO share from your account",
                    amount
                );

                self.commitment_vault.take(amount)
            } else {
                panic!("[AlignDAO]: Wrong proof provided")
            }
        }

        /// The method allow DAO members or delegators to 
        /// withdraw all their their delegated/withdrawable resource.
        ///
        /// # Input
        /// - identity: the delegator/DAO member SBT proof.
        /// # Output
        /// The bucket contain the withdrawed resource
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member/Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - The DAO Member haven't begin the retirement process through the [retire()](DAO_impl::DAO::retire) method.
        /// - The Delegator is currently voting on some proposals.
        /// ## Intra-package access
        /// - Access field [retirement](crate::member::DAOMember::retirement) to check if the DAO Member has retired or not.
        /// - Access write method [retire()](crate::member::DAOMember::retire) to advance DAO Member's retirement process.
        /// - Access write method [withdraw_all()](crate::member::DAOMember::withdraw_all) to withdraw from DAO Member's account.
        /// - Access and write field [delegated_amount](crate::delegator::Delegator::delegated_amount) to withdraw the Delegator's resource.
        /// - Write field [rage_withdrawable](crate::delegator::Delegator::rage_withdrawable) to make the Delegator's unable to do rage withdraw after having withdrawed normally.
        /// - Access read only method [is_not_voting()](crate::delegator::Delegator::is_not_voting) to check if the Delegator is currently voting on any proposal or not.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_delegator/withdraw_all.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_delegator/withdraw_all.rtm")]
        /// ```
        pub fn withdraw_all(&mut self, identity: Proof) -> Bucket {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[AlignDAO]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let mut member_data = member_sbt.data();

                assert!(
                    member_data.retirement.is_some(),
                    "[AlignDAO]: You must begin your retirement process first!"
                );

                member_data.retire(&self.commitment_policy, self.current());

                let amount = member_data.withdraw_all();

                self.controller_badge.authorize(|| {
                    member_sbt.update_data(member_data);
                });

                info!(
                    "[AlignDAO]: Withdrawed {} DAO share from your account",
                    amount
                );

                self.commitment_vault.take(amount)
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let mut delegator_data = delegator_sbt.data();

                assert!(delegator_data.is_not_voting(&self.proposals), "[AlignDAO]: You can only withdraw if you're currently not voting in any proposal.");

                let amount = delegator_data.delegated_amount;
                delegator_data.delegated_amount = Decimal::ZERO;
                delegator_data.rage_withdrawable = None;

                self.controller_badge.authorize(|| {
                    delegator_sbt.update_data(delegator_data);
                });

                info!(
                    "[AlignDAO]: Withdrawed {} DAO share from your account",
                    amount
                );

                self.commitment_vault.take(amount)
            } else {
                panic!("[AlignDAO]: Wrong proof provided")
            }
        }

        /// The method allow DAO members or delegators 
        /// to do [rage withdraw](#rage-withdraw).
        ///
        /// # Input
        /// - identity: the delegator/DAO member SBT proof.
        /// # Output
        /// The bucket contain the rage withdrawed resource
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member/Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - The DAO Member haven't begin the retirement process through the [retire()](DAO_impl::DAO::retire) method.
        /// - The Delegator is currently voting on some proposals.
        /// - The DAO Member/Delegator wasn't allowed to do rage withdraw 
        /// or have already passed the time limit to do rage withdraw.
        /// ## Intra-package access
        /// - Access field [retirement](crate::member::DAOMember::retirement) to check if the DAO Member has retired or not.
        /// - Access field rage_withdrawable of [DAOMember SBT struct](crate::member::DAOMember::rage_withdrawable) or
        /// [Delegator SBT struct](crate::delegator::Delegator::rage_withdrawable) to check if the DAO Member/Delegator is able to do rage withdraw or not
        /// and write None after the participants have successfully done rage withdraw.
        /// - Access and write field [delegated_amount](crate::delegator::Delegator::delegated_amount) to withdraw the Delegator's resource.
        /// - Access and write field [committed_amount](crate::member::DAOMember::committed_amount) to withdraw the DAO Member's resource.
        /// - Access read only method [is_not_voting()](crate::delegator::Delegator::is_not_voting) to check if the Delegator is currently voting on any proposal or not.
        /// - Access [rage_withdraw()](crate::treasury::Treasury_impl::Treasury::rage_withdraw) method from Treasury component.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_delegator/rage_withdraw.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_delegator/rage_withdraw.rtm")]
        /// ```
        pub fn rage_withdraw(&mut self, identity: Proof) -> Bucket {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[AlignDAO]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let mut member_data = member_sbt.data();
                assert!(
                    member_data.retirement.is_some(),
                    "[AlignDAO]: You must begin your retirement process first!"
                );

                let amount = match member_data.rage_withdrawable.take() {
                    None => panic!("[AlignDAO]: You are not eligible to do rage withdraw"),
                    Some((amount, time_limit)) => {
                        assert!(
                            time_limit > self.current(),
                            "[AlignDAO]: You can no longer do rage withdraw"
                        );
                        self.dao_badge.authorize(|| {
                            self.commitment_vault
                                .take(member_data.committed_amount)
                                .burn()
                        });
                        member_data.committed_amount = Decimal::ZERO;
                        amount
                    }
                };

                self.controller_badge.authorize(|| {
                    member_sbt.update_data(member_data);
                });

                info!(
                    "[AlignDAO]: Rage Withdrawed {} primary reserve resource from the treasury",
                    amount
                );

                self.treasury.rage_withdraw(amount)
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let mut delegator_data = delegator_sbt.data();

                assert!(delegator_data.is_not_voting(&self.proposals), "[AlignDAO]: You can only withdraw if you're currently not voting in any proposal.");

                let amount = match delegator_data.rage_withdrawable {
                    None => panic!("[AlignDAO]: You are not eligible to do rage withdraw"),
                    Some((amount, time_limit)) => {
                        assert!(
                            time_limit > self.current(),
                            "[AlignDAO]: You can no longer do rage withdraw"
                        );
                        self.dao_badge.authorize(|| {
                            self.commitment_vault
                                .take(delegator_data.delegated_amount)
                                .burn()
                        });
                        delegator_data.delegated_amount = Decimal::ZERO;
                        amount
                    }
                };

                self.controller_badge.authorize(|| {
                    delegator_sbt.update_data(delegator_data);
                });

                info!(
                    "[AlignDAO]: Rage Withdrawed {} primary reserve resource from the treasury",
                    amount
                );

                self.treasury.rage_withdraw(amount)
            } else {
                panic!("[AlignDAO]: Wrong proof provided")
            }
        }

        /// The method allow DAO members or delegators to 
        /// check their current vote power.
        ///
        /// The method is for test purpose only and didn't 
        /// contribute for the DAO's smartcontract logic.
        /// # Input
        /// - identity: the delegator/DAO member SBT proof.
        /// # Output
        /// Current voting power
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member/Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// ## Intra-package access
        /// - Access field [following_community](crate::member::DAOMember::following_community) 
        /// to check if the DAO Member currently following a community or not and get his/her community address.
        /// - Access field [following_community](crate::delegator::Delegator::following_community) 
        /// to check if the Delegator currently following a community or not and get his/her community address.
        /// - Access the [representing](crate::member::DAOMember::representing) data field from
        /// [DAOMember](crate::member::DAOMember) data struct to check if the voter is a representative or not and get his/her community address.
        /// - Use write method [calculate_voting_power()](crate::member::DAOMember::calculate_voting_power)
        /// from [DAOMember](crate::member::DAOMember) data struct to calculate the DAO member voting power.
        /// - Access the [delegated_amount](crate::delegator::Delegator::delegated_amount) data field to get the Delegator voting power.
        /// - Access the representative's community name, follower list, tax percent, remain vote power percent
        /// from the read only method [vote_state()](crate::community::Community_impl::Community::vote_state) for calculating community vote power.
        /// - Access the community tax percent from the read only method [tax()](crate::community::Community_impl::Community::tax) 
        /// for calculating member/delegator vote power after taxed.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_delegator/show_vote_power.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_delegator/show_vote_power.rtm")]
        /// ```
        pub fn show_vote_power(&self, identity: Proof) -> Decimal {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[AlignDAO]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let mut member_data = member_sbt.data();

                let mut vote_power =
                    member_data.calculate_voting_power(&self.commitment_policy, self.current());

                let community_vote_power: Decimal = match member_data.following_community {
                    None => match member_data.representing {
                        None => {
                            info!("[AlignDAO]: Your vote power is: {}", vote_power);
                            Decimal::ZERO
                        }

                        Some(community_address) => {
                            let community: CommunityComponent = community_address.into();

                            let (_, followers, tax, remain_vote_power) = community.vote_state();

                            let mut community_vote_power = Decimal::ZERO;
                            let mut community_tax = Decimal::ZERO;

                            for address in followers {
                                if address.resource_address() == self.delegator_sbt {
                                    let mgr = borrow_resource_manager!(self.delegator_sbt);
                                    let delegator_data = mgr.get_non_fungible_data::<Delegator>(
                                        &address.non_fungible_id(),
                                    );
                                    let vote_weight = delegator_data.delegated_amount;
                                    let power_tax = vote_weight * tax;
                                    community_tax = community_tax + power_tax;
                                    community_vote_power =
                                        community_vote_power + vote_weight - power_tax;
                                } else if address.resource_address() == self.member_sbt {
                                    let mgr = borrow_resource_manager!(self.member_sbt);
                                    let id = address.non_fungible_id();
                                    let mut member_data =
                                        mgr.get_non_fungible_data::<DAOMember>(&id);
                                    let vote_weight = member_data.calculate_voting_power(
                                        &self.commitment_policy,
                                        self.current(),
                                    );
                                    let power_tax = vote_weight * tax;
                                    community_tax = community_tax + power_tax;
                                    community_vote_power =
                                        community_vote_power + vote_weight - power_tax;
                                }
                            }

                            vote_power += community_tax;
                            let final_vote_power = vote_power * remain_vote_power;

                            info!("[AlignDAO]: Your vote power is: {}", final_vote_power);
                            info!(
                                "[AlignDAO]: Vote power got from your community tax: {}",
                                community_tax
                            );
                            info!("[AlignDAO]: Remain vote power percent according to your credibility: {}%", remain_vote_power * 100);
                            info!(
                                "[AlignDAO]: Your community vote power: {}",
                                community_vote_power
                            );

                            community_vote_power
                        }
                    },

                    Some(community_name) => {
                        let community: CommunityComponent = self
                            .communities
                            .get(&community_name)
                            .unwrap()
                            .clone()
                            .into();
                        let tax = community.tax();
                        let vote_power_tax = vote_power * tax;
                        vote_power = vote_power - vote_power_tax;
                        info!("[AlignDAO]: Your vote power is: {}", vote_power);
                        info!(
                            "[AlignDAO]: Vote power got taxed to your community: {}",
                            vote_power_tax
                        );
                        Decimal::ZERO
                    }
                };

                vote_power + community_vote_power
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let delegator_data = delegator_sbt.data();
                let mut vote_power = delegator_data.delegated_amount;
                match delegator_data.following_community {
                    Some(community_name) => {
                        let community: CommunityComponent = self
                            .communities
                            .get(&community_name)
                            .unwrap()
                            .clone()
                            .into();
                        let tax = community.tax();
                        let vote_power_tax = vote_power * tax;
                        vote_power = vote_power - vote_power_tax;
                        info!("[AlignDAO]: Your vote power is: {}", vote_power);
                        info!(
                            "[AlignDAO]: Vote power got taxed to your community: {}",
                            vote_power_tax
                        );
                    }
                    None => {}
                };

                vote_power
            } else {
                panic!("[AlignDAO]: Wrong proof provided")
            }
        }

        /// The method allow DAO members or delegators to 
        /// check their current account state.
        ///
        /// The method is for test purpose only and didn't 
        /// contribute for the DAO's smartcontract logic.
        ///
        /// # Input
        /// - identity: the delegator/DAO member SBT proof.
        /// # Output
        /// Sum of current account's total delegated/committed amount and reward
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member/Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// ## Intra-package access
        /// - Access field [committed_amount](crate::member::DAOMember::committed_amount) 
        /// to check the DAO Member's current committed resource amount.
        /// - Access field [rewarded](crate::member::DAOMember::rewarded) 
        /// to check the DAO Member's current dividend reward amount.
        /// - Access field [delegated_amount](crate::delegator::Delegator::delegated_amount) 
        /// to check the Delegator's current delegated resource amount.
        /// - Access field [rewarded](crate::delegator::Delegator::rewarded) 
        /// to check the Delegator's current dividend reward amount.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_delegator/show_account.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_delegator/show_account.rtm")]
        /// ```
        pub fn show_account(&self, identity: Proof) -> Decimal {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[AlignDAO]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let member_data = member_sbt.data();
                let committed_amount = member_data.committed_amount;
                let rewarded = member_data.rewarded;
                info!(
                    "[AlignDAO]: Your committed resource amount is: {}",
                    committed_amount
                );
                info!(
                    "[AlignDAO]: Your current reward from participating in the DAO is: {}",
                    rewarded
                );

                committed_amount + rewarded
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let delegator_data = delegator_sbt.data();
                let delegated_amount = delegator_data.delegated_amount;
                let rewarded = delegator_data.rewarded;
                info!(
                    "[AlignDAO]: Your delegated resource amount is: {}",
                    delegated_amount
                );
                info!(
                    "[AlignDAO]: Your current reward from participating in the DAO is: {}",
                    rewarded
                );
                delegated_amount + rewarded
            } else {
                panic!("[AlignDAO]: Wrong proof provided")
            }
        }

        /// The method allow DAO members or delegators to 
        /// check their current following community.
        ///
        /// The method is for test purpose only and didn't 
        /// contribute for the DAO's smartcontract logic.
        ///
        /// # Input
        /// - identity: the delegator/DAO member SBT proof.
        /// # Output
        /// Current following community name
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member/Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// ## Intra-package access
        /// - Access field [following_community](crate::member::DAOMember::following_community) 
        /// to check the community that the DAO Member is currently following.
        /// - Access field [following_community](crate::delegator::Delegator::following_community) 
        /// to check the community that the Delegator is currently following.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_delegator/check_community.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_delegator/check_community.rtm")]
        /// ```
        pub fn check_community(&self, identity: Proof) -> String {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[AlignDAO]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let member_data = member_sbt.data();
                match member_data.following_community {
                    None => {
                        info!("[AlignDAO]: You're currently not following any community");
                        String::default()
                    }
                    Some(community_name) => {
                        info!(
                            "[AlignDAO]: You're currently following {} community",
                            community_name
                        );
                        community_name
                    }
                }
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let delegator_data = delegator_sbt.data();
                match delegator_data.following_community {
                    Some(community_name) => {
                        info!(
                            "[AlignDAO]: You're currently following {} community",
                            community_name
                        );
                        community_name
                    }
                    None => {
                        info!("[AlignDAO]: You're currently not following any community");
                        String::default()
                    }
                }
            } else {
                panic!("[AlignDAO]: Wrong proof provided")
            }
        }

        /// The method allow DAO member or delegator to 
        /// [rage quit](#rage-quiting-&-credibility-score-system) their community.
        ///
        /// # Input
        /// - identity: the delegator/DAO member SBT proof.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member/Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - The DAO Member/Delegator is not following any community.
        /// 
        /// ## Intra-package access
        /// - Access read only method [is_following()](crate::member::DAOMember::is_following) and write field [following_community](crate::member::DAOMember::following_community) 
        /// to check and let DAO Member quit the community that he/she is currently following.
        /// - Access read only method [is_following()](crate::delegator::Delegator::is_following) and write field [following_community](crate::delegator::Delegator::following_community) 
        /// to check and let Delegator quit the community that he/she is currently following.
        /// - Access write method [rage_quit()](crate::community::Community_impl::Community::rage_quit) from Community component for DAO Member/Delegator to rage quit his/her community.
        /// - Access write method [retract_vote()](crate::proposal::Proposal_impl::Proposal::retract_vote) from Proposal components for DAO Member/Delegator to retract his/her indirect vote after rage quitted the community.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_delegator/rage_quit.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_delegator/rage_quit.rtm")]
        /// ```
        pub fn rage_quit(&mut self, identity: Proof) {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[AlignDAO]: Can only using 1 SBT at a time!"
            );

            let (address, community_name) = if validated_proof.resource_address() == self.member_sbt
            {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let address = member_sbt.address();
                let mut member_data = member_sbt.data();

                assert!(
                    member_data.is_following(),
                    "[AlignDAO]: You're currently not joining any community"
                );

                let community_name = member_data.following_community.unwrap();

                member_data.following_community = None;

                info!("[AlignDAO]: Rage quitted {} community", community_name);
                self.controller_badge
                    .authorize(|| member_sbt.update_data(member_data));

                (address, community_name)
            } else if validated_proof.resource_address() == self.delegator_sbt {

                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let address = delegator_sbt.address();
                let mut delegator_data = delegator_sbt.data();

                assert!(
                    delegator_data.is_following(),
                    "[AlignDAO]: You're currently not joining any community"
                );

                let community_name = delegator_data.following_community.unwrap();

                delegator_data.following_community = None;

                info!("[AlignDAO]: Rage quitted {} community", community_name);
                self.controller_badge
                    .authorize(|| delegator_sbt.update_data(delegator_data));

                (address, community_name)

            } else {
                panic!("[AlignDAO]: Wrong proof provided")
            };

            let community_address = self.communities.get(&community_name).unwrap().clone();
            let community: CommunityComponent = community_address.into();

            self.controller_badge.authorize(||{

                for proposal in self.proposals.values() {
                    let proposal: ProposalComponent = proposal.clone().into();
                    proposal.retract_vote(address.clone(), community_name.clone())
                };

                match community.rage_quit(address) {
                    None => {},
                    Some(confiscate_amount) => {
                        self.dao_badge.authorize(||{self.commitment_vault.take(confiscate_amount).burn()});
                        self.communities.remove(&community_name);
                        self.communities_by_id.retain(|_, &mut x| x != community_address);
                        info!("[AlignDAO]: Confiscate resources of the corrupted {} community's representative", community_name)
                    }
                }
            })
        }

        /// The method allow DAO members or delegators to claim their dividend reward.
        ///
        /// # Input
        /// - identity: the Delegator/DAO member SBT proof.
        /// # Output
        /// The dividend reward bucket
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member/Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// 
        /// ## Intra-package access
        /// - Access and write field [rewarded](crate::delegator::Delegator::rewarded) 
        /// to let Delegator claim dividend reward from his/her account.
        /// - Access and write field [rewarded](crate::member::DAOMember::rewarded) 
        /// to let DAO Member claim dividend reward from his/her account.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_delegator/claim_dividend.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_delegator/claim_dividend.rtm")]
        /// ```
        pub fn claim_dividend(&mut self, identity: Proof) -> Bucket {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[AlignDAO]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let mut member_data = member_sbt.data();
                let amount = member_data.rewarded;
                member_data.rewarded = Decimal::ZERO;
                self.controller_badge
                    .authorize(|| member_sbt.update_data(member_data));
                let vault_amount = self.dividend_vault.amount();
                if vault_amount < amount {
                    // Since there's some high level math behind the voting process, the received dividend might be rounded up (at some rate) between each beneficiary,
                    // the last one will be the one to take all remain (while it might smaller a bit than the real amount he/she can actually be received, maybe < 0.00001 token)
                    info!(
                        "[AlignDAO]: You have claimed {} dividend from the DAO",
                        vault_amount
                    );
                    self.dividend_vault.take_all()
                } else {
                    info!(
                        "[AlignDAO]: You have claimed {} dividend from the DAO",
                        amount
                    );
                    self.dividend_vault.take(amount)
                }
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let mut delegator_data = delegator_sbt.data();
                let amount = delegator_data.rewarded;
                delegator_data.rewarded = Decimal::ZERO;
                self.controller_badge
                    .authorize(|| delegator_sbt.update_data(delegator_data));
                let vault_amount = self.dividend_vault.amount();
                if vault_amount < amount {
                    // Since there's some high level math behind the voting process, the received dividend might be rounded up (at some rate) between each beneficiary,
                    // the last one will be the one to take all remain (while it might smaller a bit than the real amount he/she can actually be received, maybe < 0.00001 token)
                    info!(
                        "[AlignDAO]: You have claimed {} dividend from the DAO",
                        vault_amount
                    );
                    self.dividend_vault.take_all()
                } else {
                    info!(
                        "[AlignDAO]: You have claimed {} dividend from the DAO",
                        amount
                    );
                    self.dividend_vault.take(amount)
                }
            } else {
                panic!("[AlignDAO]: Wrong proof provided")
            }
        }

        /// The method allow delegator to add resource to their current account.
        ///
        /// # Input
        /// - identity: the delegator SBT proof.
        /// - delegate_bucket: the delegate bucket that the delegator want to add into his/her account
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong Delegator SBT Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// 
        /// ## Intra-package access
        /// - Access and write field [delegated_amount](crate::delegator::Delegator::delegated_amount) 
        /// to let Delegator add resource to his/her account.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/delegator_only/add_delegate.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/delegator_only/add_delegate.rtm")]
        /// ```
        pub fn add_delegate(&mut self, identity: Proof, delegate_bucket: Bucket) {
            assert!(
                delegate_bucket.resource_address() == self.commitment_vault.resource_address(),
                "[AlignDAO]: Wrong delegate resource!"
            );

            let validated_proof = identity
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.delegator_sbt,
                    dec!("1"),
                ))
                .expect("[AlignDAO]: Error validating proof");

            let delegator_sbt = validated_proof.non_fungible::<Delegator>();
            let mut delegator_data = delegator_sbt.data();

            delegator_data.delegated_amount += delegate_bucket.amount();

            self.controller_badge
                .authorize(|| delegator_sbt.update_data(delegator_data));

            self.commitment_vault.put(delegate_bucket)
        }

        /// The method allow DAO members to re-accumulate dividend reward 
        /// on their current SBT account for more vote weight.
        ///
        /// # Input
        /// - amount: the DAO share dividend amount that DAO members want to accumulate.
        /// - member_proof: the member SBT proof.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - The DAO Member is a Representative.
        /// 
        /// ## Intra-package access
        /// - Use write method [accumulate_dividend()](crate::member::DAOMember::accumulate_dividend) 
        /// to let DAO Member re-accumulate dividend reward in his/her committed SBT account.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_only/accumulate_dividend.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_only/accumulate_dividend.rtm")]
        /// ```
        pub fn accumulate_dividend(&mut self, amount: Decimal, member_proof: Proof) {
            let validated_proof = member_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.member_sbt,
                    dec!("1"),
                ))
                .expect("[AlignDAO]: Error validating proof");

            let member_sbt = validated_proof.non_fungible::<DAOMember>();
            let mut member_data = member_sbt.data();
            assert!(
                !member_data.is_representing(),
                "[AlignDAO]: Representatives cannot accumulate more vote power"
            );
            member_data.accumulate_dividend(amount);
            self.commitment_vault.put(self.dividend_vault.take(amount));
            info!("[AlignDAO]: Accumulated {} dividend", amount);
            self.controller_badge
                .authorize(|| member_sbt.update_data(member_data));
        }

        /// The method allow DAO members to begin (or advance) their [retirement process](#replacement-&-retirement-process) and settle any withdrawable resource.
        ///
        /// # Input
        /// - member_proof: the member SBT proof.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - The DAO Member is a Representative and his/her community still have followers.
        /// 
        /// ## Intra-package access
        /// - Access and write field [representing](crate::member::DAOMember::representing) 
        /// to check if the DAO Member is a representative and let the Representative abandon his/her community.
        /// - Use write method [abandon()](crate::community::Community_impl::Community::abandon) from Community component
        /// to let the Representative abandon his/her community.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_only/retire.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_only/retire.rtm")]
        /// ```
        pub fn retire(&mut self, member_proof: Proof) {
            let validated_proof = member_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.member_sbt,
                    dec!("1"),
                ))
                .expect("[AlignDAO]: Error validating proof");

            let member_sbt = validated_proof.non_fungible::<DAOMember>();
            let mut member_data = member_sbt.data();

            match member_data.representing.take() {
                None => {}
                Some(community_address) => {
                    let community: CommunityComponent = community_address.clone().into();
                    self.controller_badge.authorize(|| community.abandon());
                    self.communities
                        .retain(|_, &mut address| address != community_address);
                    // This is just for going around current Scrypto's [known bug](https://github.com/radixdlt/radixdlt-scrypto/issues/483)
                    self.communities_by_id.remove(&member_sbt.id());
                }
            };

            member_data.retire(&self.commitment_policy, self.current());
            self.controller_badge.authorize(|| {
                member_sbt.update_data(member_data);
            });
        }

        /// The method allow DAO member to create a new community and become the representative of that community according to [Liquid Democracy](#liquid-democracy).
        ///
        /// # Input
        /// - member_proof: the member SBT proof.
        /// - name: community name.
        /// - tax_percent: tax percent for running the community.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// 
        /// - Wrong DAO Member Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - Provided community name is already used.
        /// - The DAO Member is currently following a community.
        /// - The DAO Member is currently representing a community.
        /// - The DAO Member didn't meet the vote power requirement to become Representative according to the [CommunityPolicy](crate::policies::CommunityPolicy).
        /// 
        /// ## Intra-package access
        /// 
        /// - Access many helpful read only method from [DAOMember](crate::member::DAOMember) data struct.
        /// - Use write method [calculate_voting_power()](crate::member::DAOMember::calculate_voting_power)
        /// from [DAOMember](crate::member::DAOMember) data struct to calculate the DAO member voting power.
        /// - Access read only method [check_requirement](crate::policies::CommunityPolicy::check_requirement) from CommunityPolicy struct
        /// to check if the DAO Member meet the vote power requirement to become Representative or not.
        /// - Use function [new()](crate::community::Community_impl::Community::new) on Community blueprint
        /// to let the DAO Member create new community component.
        /// - Access and write field [representing](crate::member::DAOMember::representing) 
        /// to make the DAO Member become Representative of the created community.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_only/become_representative.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_only/become_representative.rtm")]
        /// ```
        pub fn become_representative(
            &mut self,
            member_proof: Proof,
            name: String,
            tax_percent: Decimal,
        ) {
            assert!(!self.communities.contains_key(&name), "[AlignDAO]: There's already a community with this name, please use diffirent name.");

            let validated_proof = member_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.member_sbt,
                    dec!("1"),
                ))
                .expect("[AlignDAO]: Error validating proof");

            let member_sbt = validated_proof.non_fungible::<DAOMember>();

            let mut member_data = member_sbt.data();

            assert!(
                !member_data.is_representing(),
                "[AlignDAO]: You're already representing for a community."
            );

            assert!(!member_data.is_following(), "[AlignDAO]: You're currently following a community, please quit the community before become a representative.");

            self.community_policy.check_requirement(member_data.calculate_voting_power(&self.commitment_policy, self.current()));

            let representative_address = member_sbt.address();

            let community_controller_badge = self
                .controller_badge
                .authorize(|| borrow_resource_manager!(self.community_controller_badge).mint(1));

            let mut community_component = CommunityComponent::new(
                community_controller_badge,
                name.clone(),
                self.delegator_sbt,
                self.member_sbt,
                self.community_policy.initial_credibility,
                tax_percent,
                representative_address.clone(),
            );

            community_component.add_access_check(
                AccessRules::new()
                    .method(
                        "amend_tax_policy",
                        rule!(require(representative_address.clone())),
                    )
                    .method(
                        "review_request",
                        rule!(require(representative_address)))
                    .method(
                        "abandon",
                        rule!(require(self.controller_badge.resource_address())),
                    )
                    .method(
                        "rage_quit",
                        rule!(require(self.controller_badge.resource_address())),
                    )
                    .default(rule!(allow_all)),
            );

            let community_address = community_component.globalize();

            member_data.representing = Some(community_address);

            self.controller_badge
                .authorize(|| member_sbt.update_data(member_data));

            info!(
                "[AlignDAO]: You have become the representative for {} community.",
                &name
            );

            self.communities.insert(name, community_address);

            // This is just for going around current Scrypto's [known bug](https://github.com/radixdlt/radixdlt-scrypto/issues/483)
            self.communities_by_id
                .insert(member_sbt.id(), community_address);
        }

        /// The method will allow DAO members on retirement process to propose a replament for his/her to end his/her current retirement process.
        /// # Input
        /// - member_proof: the member SBT proof.
        /// - time_delay: the time delay for the proposal
        /// # Output
        /// The badge keep track of the proposal
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// 
        /// - Wrong DAO Member Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - The DAO Member isn't currently on retirement process.
        /// 
        /// ## Intra-package access
        /// 
        /// - Access read only method [is_retiring()](crate::member::DAOMember::is_retiring)
        /// from [DAOMember](crate::member::DAOMember) data struct to check if the DAO Member is on retirement process.
        /// - Use function [new()](crate::proposal::Proposal_impl::Proposal::new) from Proposal blueprint
        /// to let the DAO Member create new proposal component.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/dao/member_only/replacement_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/member_only/replacement_proposal.rtm")]
        /// ```
        pub fn replacement_proposal(&mut self, member_proof: Proof, time_delay: u64) {
            let validated_proof = member_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.member_sbt,
                    dec!("1"),
                ))
                .expect("[AlignDAO]: Error validating proof");

            let current = self.current();

            let member_sbt = validated_proof.non_fungible::<DAOMember>();
            let member_data = member_sbt.data();
            assert!(
                member_data.is_retiring(),
                "[AlignDAO]: You're not currently on retirement!"
            );

            self.proposal_id_counter += 1;

            let id = NonFungibleId::from_u64(self.proposal_id_counter);

            let proposal_controller_badge = self
                .controller_badge
                .authorize(|| borrow_resource_manager!(self.proposal_controller_badge).mint(1));

            let member_id = member_sbt.id();

            let mut proposal_component = ProposalComponent::new(
                Runtime::actor().as_component().0,
                proposal_controller_badge,
                id.clone(),
                time_delay + current,
                self.reserve,
                self.proposal_badge,
                self.member_sbt,
                self.delegator_sbt,
                self.commitment_policy.clone(),
                self.economic_policy.clone(),
                self.proposal_policy.proposal_quorum,
                self.oracle_address,
                Methods(vec![Method {
                    component: Runtime::actor().as_component().0,
                    method: "accept_replacement".to_owned(),
                    args: hex::encode(args!(member_id)),
                }]),
                Decimal::ZERO,
                None,
            );

            proposal_component.add_access_check(
                AccessRules::new()
                    .method(
                        "retract_vote",
                        rule!(require(self.controller_badge.resource_address())),
                    )
                    .default(rule!(allow_all)),
            );

            self.proposals
                .insert(id.clone(), proposal_component.globalize());

            info!("[AlignDAO]: You have made replacement proposal no.{} for the DAO to accept your replacement", id.clone());
        }

        /// The method will allow DAO members to propose a concept for others to vote on.
        /// # Input
        /// - member_proof: the member SBT proof.
        /// - methods: method list that the proposer want to call by the DAO's authority.
        /// - fund_demand: the fund that the proposer demanded, it can be a reward for his/her work, or funding for his/her project.
        /// - time_delay: the time delay for the proposal
        /// # Output
        /// The badge keep track of the proposal
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong DAO Member Proof provided.
        /// - Provided proof of more than 1 SBT at a time.
        /// - Provided proposal time delay is inappropriate.
        /// - The requested distribution resource isn't fungible token.
        /// - The DAO Member didn't meet the vote power requirement to make a proposal according to the [ProposalPolicy](crate::policies::ProposalPolicy).
        /// 
        /// ## Intra-package access
        /// - Use write method [calculate_voting_power()](crate::member::DAOMember::calculate_voting_power)
        /// from [DAOMember](crate::member::DAOMember) data struct to calculate the DAO member voting power.
        /// - Access read only method [check_requirement](crate::policies::ProposalPolicy::check_requirement) from ProposalPolicy struct
        /// to check if the DAO Member meet the vote power requirement to make a proposal or not.
        /// - Use function [new()](crate::proposal::Proposal_impl::Proposal::new) from Proposal blueprint
        /// to let the DAO Member create new proposal component.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// **Proposal with fund demand only**
        /// 
        /// `rtm/dao/proposal/get_fund_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/proposal/get_fund_proposal.rtm")]
        /// ```
        /// 
        /// **Proposal with one method only**
        /// 
        /// `rtm/dao/proposal/one_method_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/proposal/one_method_proposal.rtm")]
        /// ```
        /// 
        /// **Proposal to distribute resource**
        /// 
        /// `rtm/dao/proposal/distribution_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/proposal/distribution_proposal.rtm")]
        /// ```
        /// 
        /// **Combined test proposal**
        /// 
        /// `rtm/test/test_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/test/test_proposal.rtm")]
        /// ```
        pub fn new_proposal(
            &mut self,
            member_proof: Proof,
            methods: Methods,
            fund_demand: Decimal,
            time_delay: u64,
            distribute: Option<ResourceAddress>,
        ) -> Option<Bucket> {
            let validated_proof = member_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.member_sbt,
                    dec!("1"),
                ))
                .expect("[AlignDAO]: Error validating proof");

            assert!(
                time_delay >= self.proposal_policy.proposal_minimum_delay,
                "[AlignDAO]: Cannot set proposal time delay smaller than the minimum!"
            );

            match distribute {
                None => {}
                Some(resource) => {
                    assert!(
                        matches!(
                            borrow_resource_manager!(resource).resource_type(),
                            ResourceType::Fungible { .. }
                        ),
                        "[AlignDAO]: The resource isn't of fungible type."
                    );
                }
            };

            let current = self.current();

            let member_sbt = validated_proof.non_fungible::<DAOMember>();
            let mut member_data = member_sbt.data();
            let voting_power = member_data.calculate_voting_power(&self.commitment_policy, current);

            self.proposal_policy.check_requirement(voting_power);

            self.proposal_id_counter += 1;

            let id = NonFungibleId::from_u64(self.proposal_id_counter);

            let proposal_controller_badge = self
                .controller_badge
                .authorize(|| borrow_resource_manager!(self.proposal_controller_badge).mint(1));

            let mut proposal_component = ProposalComponent::new(
                Runtime::actor().as_component().0,
                proposal_controller_badge,
                id.clone(),
                time_delay + current,
                self.reserve,
                self.proposal_badge,
                self.member_sbt,
                self.delegator_sbt,
                self.commitment_policy.clone(),
                self.economic_policy.clone(),
                self.proposal_policy.proposal_quorum,
                self.oracle_address,
                methods,
                fund_demand,
                distribute,
            );

            proposal_component.add_access_check(
                AccessRules::new()
                    .method(
                        "retract_vote",
                        rule!(require(self.controller_badge.resource_address())),
                    )
                    .default(rule!(allow_all)),
            );

            self.proposals
                .insert(id.clone(), proposal_component.globalize());

            info!("[AlignDAO]: You have made proposal id {} for the DAO's collective action on your list of methods with {} fund demand", id.clone(), fund_demand);

            if fund_demand > Decimal::ZERO {
                Some(self.controller_badge.authorize(|| {
                    member_sbt.update_data(member_data);
                    borrow_resource_manager!(self.proposal_badge)
                        .mint_non_fungible(&id, ProposalBadge {})
                }))
            } else {
                None
            }
        }

        /// The method will allow Proposal Component with the right controller badge to access the dao badge
        ///  through its [execute()](crate::proposal::Proposal_impl::Proposal::execute) method.
        /// 
        /// The method represent collective actions of the DAO
        /// 
        /// # Output
        /// The badge keep track of the proposal
        /// 
        /// # Access Rule
        /// Not user callable, can only be called from the right controller badge created from [new_proposal()](DAO_impl::DAO::new_proposal) method
        /// or [replacement_proposal()](DAO_impl::DAO::replacement_proposal) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, this method can only be called from the 
        /// [execute()](crate::proposal::Proposal_impl::Proposal::execute) method from Proposal Blueprint.
        pub fn dao_proof(&self) -> Proof {
            self.dao_badge.create_proof()
        }

        /// The method will do the following:
        /// - Mint new dividend reward for support voters on an accepted proposal.
        /// - Take the allocated fund for the accepted proposal (if the proposal has a fund demand).
        /// - Remove the accepted proposal from the DAO.
        /// 
        /// # Input
        /// - id: The proposal ID
        /// - dividend: The dividend amount on this proposal according to [EconomicPolicy](crate::policies::EconomicPolicy).
        /// - fund_demand: The fund demand amount for this proposal.
        /// 
        /// # Output
        /// - None if the input fund_demand <= 0.
        /// - Wrapped fund bucket allocate for the proposal if fund_demand > 0.
        /// 
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        /// 
        /// ## Panics
        /// - The DAO's proposal list didn't have the proposal ID.
        pub fn accepted(
            &mut self,
            id: NonFungibleId,
            dividend: Decimal,
            fund_demand: Decimal,
        ) -> Option<Bucket> {
            self.proposals.remove(&id).expect(
                "[AlignDAO]: The DAO didn't have this proposal or it has already executed!",
            );

            let dao_share_address = self.commitment_vault.resource_address();
            let mgr = borrow_resource_manager!(dao_share_address);
            let dividend = mgr.mint(dividend);
            self.dividend_vault.put(dividend);

            if fund_demand > Decimal::ZERO {
                let bucket = self.dao_withdraw_by_amount(self.reserve, fund_demand);
                info!(
                    "[AlignDAO]: The proposal have received {} fund.",
                    bucket.amount()
                );
                Some(bucket)
            } else {
                None
            }
        }

        /// The method will do the following:
        /// - Take the slash resource amount of support voters on the 
        /// rejected proposal from the [commitment_vault](DAO_impl::DAO::commitment_vault) and burn it.
        /// - Remove the rejected proposal from the DAO.
        /// 
        /// # Input
        /// - id: The proposal ID
        /// - slash: The slash amount of all support voters on the rejected proposal
        /// 
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method of Proposal Blueprint when the proposal is rejected.
        /// 
        /// ## Panics
        /// - The DAO's proposal list didn't have the proposal ID.
        pub fn rejected(&mut self, id: NonFungibleId, slash: Decimal) {
            self.proposals.remove(&id).expect(
                "[AlignDAO]: The DAO didn't have this proposal or it has already executed!",
            );
            self.commitment_vault.take(slash).burn()
        }

        /// The method will remove the ignored proposal from the DAO.
        /// 
        /// # Input
        /// - id: The proposal ID
        /// 
        /// # Access Rule
        /// Not user callable, can only be called from the right controller badge created from [new_proposal()](DAO_impl::DAO::new_proposal) method
        /// or [replacement_proposal()](DAO_impl::DAO::replacement_proposal) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, this method can only be called from the 
        /// [execute()](crate::proposal::Proposal_impl::Proposal::execute) method from Proposal Blueprint when the proposal is ignored.
        /// 
        /// ## Panics
        /// - The DAO's proposal list didn't have the proposal ID.
        pub fn ignored(&mut self, id: NonFungibleId) {
            self.proposals.remove(&id).expect(
                "[AlignDAO]: The DAO didn't have this proposal or it has already executed!",
            );
        }

        /// The method will allow the DAO Member with the provided ID to end his/her retirement process immediately
        /// 
        /// # Input
        /// - id: The DAO Member ID
        /// 
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        /// 
        /// ## Panics
        /// - Wrong DAO Member id.
        /// - The DAO Member haven't begun the retirement process yet.
        /// 
        /// ## Intra-package access
        /// - Access write method [accept_replacement()](crate::member::DAOMember::accept_replacement)
        /// from [DAOMember](crate::member::DAOMember) data struct to allow the DAO Member end his/her current retirement process.
        pub fn accept_replacement(&self, id: NonFungibleId) {
            let mgr = borrow_resource_manager!(self.member_sbt);
            let mut member_data = mgr.get_non_fungible_data::<DAOMember>(&id);
            member_data.accept_replacement();
            self.controller_badge.authorize(|| {
                mgr.update_non_fungible_data(&id, member_data);
            });
            info!("[AlignDAO]: Accepted replacement for member id {}", id)
        }

        /// The method allow withdraw resource by amount directly from the treasury.
        /// 
        /// # Input
        /// - resource_address: The withdraw resource address.
        /// - amount: The withdraw amount.
        ///
        /// # Output
        /// The withdraw resource Bucket
        /// 
        /// Return empty Bucket if the withdraw threshold has met on the withdraw period according to [TreasuryPolicy](crate::policies::TreasuryPolicy).
        /// Also return empty Bucket if the treasury didn't have the resource.
        /// 
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        /// 
        /// ## Intra-package access
        /// - Access [withdraw_by_amount()](crate::treasury::Treasury_impl::Treasury::withdraw_by_amount) method from Treasury component.
        pub fn dao_withdraw_by_amount(
            &mut self,
            resource_address: ResourceAddress,
            amount: Decimal,
        ) -> Bucket {
            self.treasury
                .withdraw_by_amount(amount, resource_address, self.current())
        }

        /// The method allow withdraw resource directly from the treasury.
        /// 
        /// # Input
        /// - resource_address: The withdraw resource address.
        ///
        /// # Output
        /// The withdraw resource Bucket
        ///
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        /// 
        /// ## Intra-package access
        /// - Access [withdraw()](crate::treasury::Treasury_impl::Treasury::withdraw) method from Treasury component.
        pub fn dao_withdraw(&mut self, resource_address: ResourceAddress) -> Bucket {
            self.treasury.withdraw(resource_address)
        }

        /// The method allow the DAO to change its on-using oracle.
        /// 
        /// # Input
        /// - oracle: New oracle component address.
        /// - oracle: New data badge bucket to use data from the oracle.
        ///
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        /// 
        /// ## Intra-package access
        /// - Access [new()](crate::local_oracle::LocalOracle_impl::LocalOracle::new) function from LocalOracle blueprint.
        pub fn change_oracle(&mut self, oracle: ComponentAddress, data_badge: Bucket) {
            let mut local_oracle = LocalOracleComponent::new(oracle, data_badge);

            local_oracle.add_access_check(
                AccessRules::new()
                    .method("refund_oracle", AccessRule::AllowAll)
                    .default(rule!(require_any_of(vec![
                        self.controller_badge.resource_address(),
                        self.proposal_controller_badge
                    ]))),
            );

            self.oracle_address = local_oracle.globalize();
        }

        /// The method allow the DAO to amend its treasury policy.
        /// 
        /// # Input
        /// - new_fee: New fee policy for the treasury.
        ///
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        pub fn amend_treasury_policy(&mut self, new_fee: Decimal) {
            assert_rate(new_fee);
            self.treasury.amend_fee_policy(new_fee / dec!(100))
        }

        /// The method allow the DAO to amend its economic policy.
        ///
        /// New policy will only affect new proposal.
        /// Current on-voting proposal is not affected and will keep the previous economic policy for fairness.
        /// 
        /// # Input
        /// - slash_rate: New slash rate for the DAO's economic policy.
        /// - dividend: New dividend for the DAO's economic policy.
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        pub fn amend_economic_policy(&mut self, slash_rate: Decimal, dividend: Decimal) {
            assert_rate(slash_rate);
            self.economic_policy = EconomicPolicy {
                slash_rate: slash_rate / dec!(100),
                dividend,
            }
        }

        /// The method allow the DAO to amend its commitment policy.
        ///
        /// New policy will only affect new proposal.
        /// Current on-voting proposal is not affected and will keep the previous commitment policy for fairness.
        /// 
        /// # Input
        /// New Commitment Policy for the DAO.
        /// 
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        pub fn amend_commitment_policy(
            &mut self,
            initital_commitment_rate: Decimal,
            minimum_retirement: u64,
            maximum_retirement: u64,
            commitment_grow_rate: Decimal,
            maximum_vote_rate: Decimal,
            period_length: u64,
        ) {
            assert_rate(initital_commitment_rate);
            assert_rate(commitment_grow_rate);
            self.commitment_policy = CommitmentPolicy {
                initital_commitment_rate: initital_commitment_rate / dec!(100),
                minimum_retirement,
                maximum_retirement,
                commitment_grow_rate: commitment_grow_rate / dec!(100),
                maximum_vote_rate: maximum_vote_rate / dec!(100),
                period_length,
            }
        }

        /// The method allow the DAO to amend its proposal policy.
        ///
        /// New policy will only affect new proposal.
        /// Current on-voting proposal is not affected and will keep the previous proposal policy for fairness.
        /// 
        /// # Input
        /// New Proposal Policy for the DAO.
        /// 
        /// # Access Rule
        /// Not user callable, can only be called from the dao badge, 
        /// which can be access through the [dao_proof()](DAO_impl::DAO::dao_proof) method.
        /// 
        /// # Smartcontract logic
        /// According to the smartcontract logic, both this method and the [dao_proof()](DAO_impl::DAO::dao_proof) method
        ///  can only be called from the [execute()](crate::proposal::Proposal_impl::Proposal::execute) method on Proposal Blueprint when the proposal is accepted.
        pub fn amend_proposal_policy(
            &mut self,
            proposal_requirement: Decimal,
            proposal_quorum: Decimal,
        ) {
            self.proposal_policy.proposal_requirement = proposal_requirement;
            self.proposal_policy.proposal_quorum = proposal_quorum
        }

        /// The method allow anyone to get the current price between DAO's share and the primary reserve resource.
        /// # Output
        /// Current DAO share/reserve price
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Intra-package access
        /// - Access [get_price()](crate::treasury::Treasury_impl::Treasury::get_price) method from Treasury component.
        /// 
        /// ---
        /// 
        /// # Transaction Manifest
        /// `rtm/dao/read_only/get_price.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/read_only/get_price.rtm")]
        /// ```
        pub fn get_price(&self) -> Decimal {
            self.treasury.get_price()
        }

        /// The method allow anyone to get the treasury rage withdraw policy
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// The method mean to be called by Proposal component.
        /// ## Intra-package access
        /// - Access [get_rage_withdraw()](crate::treasury::Treasury_impl::Treasury::get_rage_withdraw) method from Treasury component.
        pub fn get_rage_withdraw(&self) -> (Decimal, u64) {
            self.treasury.get_rage_withdraw()
        }

        /// The method allow anyone to check current un-claimed dividend amount on the DAO.
        ///
        /// The method is for test purpose only and didn't contribute for the DAO's smartcontract logic.
        /// 
        /// # Output
        /// Current remain dividend on the DAO.
        /// # Access Rule
        /// Anyone can call this method
        /// 
        /// # Transaction Manifest
        /// `rtm/read_only/check_dividend.rtm`
        /// ```text
        #[doc = include_str!("../rtm/dao/read_only/check_dividend.rtm")]
        /// ```
        pub fn check_dividend(&self) -> Decimal {
            let amount = self.dividend_vault.amount();
            info!("[AlignDAO]: Current remain dividend on DAO: {}", amount);
            amount
        }

        /// This is just for going around current Scrypto's [known bug](https://github.com/radixdlt/radixdlt-scrypto/issues/483)
        pub fn get_community_address(&self, id: NonFungibleId) -> ComponentAddress {
            self.communities_by_id
                .get(&id)
                .expect("[AlignDAO]: The DAO didn't have this community")
                .clone()
        }

        /// Internal method to get current data from the oracle, can only be called internally
        fn current(&self) -> u64 {
            let local_oracle: LocalOracleComponent = self.oracle_address.into();
            let current = self.controller_badge.authorize(|| local_oracle.current());
            current
        }
    }
}
