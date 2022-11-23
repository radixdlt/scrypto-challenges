/*!

Align blueprint package is to create a Decentralized Autonomous Organization (DAO) on Web3. It used many advanced
Decentralized Governance (DeGov) techiques to create solid alignment of interests while at the same 
time foster individual accountability, allow flexibility between trustlessness and affordable trustness.
The blueprint package will promote responsive, collective wisdom on decision making, create a better DAO environment.

# Design Goals
To precisely address the need for a Decentralized Autonomous Organization and it's current problems.
The author have worked backward from organization managers' perspective to see what exactly an organization need.
In the book "*The 8th Habit: From Effectiveness to Greatness*", Covey writes about the four needs of the organization:

1. (BODY) Survival — financial health.
2. (MIND) Growth and development — economic growth, customer growth, innovation of new products and services, increasing professional and institutional competency.
3. (HEART) Relationships — strong synergy, strong external networks, and partnering, teamwork, trust, caring, valuing differences.
4. (SPIRIT) Meaning, integrity and contribution — serving and lifting all stakeholders: customers, suppliers, employees and their families, communities, society — making a difference in the world.

With each need of an organization, it would demand different component:

1. To support BODY, an organization need "income".
2. To support MIND, an organization need "R&D".
3. To support HEART, an organization need "unity".
4. To support SPIRIT, an organization need "vision and value".

Follow Covey's analogy, each individual would be a CELL on an organization.
A DAO will mainly be governed by smartcontract logic to ensure people's (CELL's) connection happend
with the most transparancy and security through "transaction".
On a DAO, there's no doubt that the DeGov mechanism will support its HEART to maintain the organization's "unity"
through people's (CELL's) connection ensured by smartcontract logic.
Since each individual would be the core contruction unit of an organization,
the person would need to have some requirement before engagement on a DAO:

1. Responsibility: an individual must take the responsibility for building that DAO,
must follow the DAO rules, must contribute for the DAO more or less in some way through "transactions",
and will also thrive (evolved) or fail along with the DAO.
Apathy is unacceptable behaviour of a participant, same as irresponsible.
2. Trust: each individual live individually but must have strong unity to support the DAO's purpose or the DAO will just "fall apart".
3. Right: each individual can have his/her own rights in accordance with his/her responsibility on the DAO.

Align blueprint package design will consider the best DeGov mechanism for a DAO's HEART through "transactions" between each individual, while at the same time ensure the most growth capacity for its BODY, MIND and SPIRIT.

There are **3 main design goals** for Align package:

## Alignment of Interest
Alignment of Interest is "*the degree to which the members of the organization are motivated to behave in line with organizational goals*" - *[Paper: INTEREST ALIGNMENT AND COMPETITIVE ADVANTAGE](https://www.jstor.org/stable/20159309)*.
In general, higher level Alignment of Interest will eventually lead to higher motivation from people to achieve common goals.
Therefore, alignment of interest will ensure high "unity" for any kind of communities, create a healthy, united "HEART" of the organization to achive the aligned "vision and value" (SPIRIT), promote healthy growth for it's "income" (BODY).

Align blueprint package will pursue the DAO's design with high level on alignment of interest to achieve solid "unity" through a time-based voting mechanism.
## Individual Accountability
***"Great power comes great responsibility"***.
Individual accountability is "*the measurement of whether or not each group member has achieved the groups’ goal. Assessing the quality and quantity of each member’s contributions and giving the results to all group members*” - *Johnson, Johnson, & Holubec, 1998*.
Higher level of individual accountability in a DAO will of course result in higher effecient in governance, greatly enhance the organization's "unity" for the "HEART" and accelerate members' growth for the "MIND".

Moreover, Individual Accountability will also prevent any malicious behaviour that might lead the organization "astray" such as "corruption" 
and solve the "tragedy of the commons" problem as Vitalik Buterin has [pointed out](https://vitalik.ca/general/2021/08/16/voting3.html#:~:text=Solution%203%3A%20skin%20in%20the%20game)
that lead to many DAO's problems: "Voter Apathy", "Vote Buying",...

Align blueprint package will pursue the DAO's design with high level of individual accountability and solve the "tragedy of the commons" 
problem through [hybrid futarchy](https://vitalik.ca/general/2021/08/16/voting3.html#:~:text=work.%20Examples%20of-,hybrid%20futarchy,-include%3A).
## Affordable Trustness
As widely know, trustlessness on any distributed ledger technology (DLT) from user perspective
can only achieved through sacrification of two costs: financial cost paying node runners, miners;
mental cost researching the DLT to ensure it's credibility, decentralization.

While DLT users all consider the first cost (financial) acceptable,
they mostly just ignore the second cost (mental) and choose to trust others instead of paying the cost.
Regard these wide-accepted events about people choosing trust instead of trustless, the author named that "Affordable Trustness".

Align blueprint package will further evolve the "Trust" property of DLTs and bring it to "social level" through Liquid Democracy, allow two choice for the DAO's participants:
- Pursue "Affordable Trustness" by trusting a representative to indirectly participate on decision making.
- Pursue "Trustlessness" by becoming a true DAO's member, self-research the proposals to directly participate on decision making and fulfill responsibility;
further pursue achievements through becoming a representative.

# Use cases:
According to Vitalik Buterin, there would be two main use cases that would need DeGov on web 3:
- [Funding public goods](https://vitalik.ca/general/2021/08/16/voting3.html#:~:text=The%20need%20for%20DeGov%20for%20funding%20public%20goods).

Public goods mean all those goods that belong to the DAO as a whole, and will also benefit the DAO as a whole. On general organization, it could be the organization infrastructure (land, building, computers,
furnitures,...), treasury, documents,... On a DAO, beside physical assets, public goods could be many digital assets: Web3 Decentralized Application;
fungible, non-fungible token (which might also be the ownership proof of physical assets); decentralized lending contract, investment contract; ...
Funding these public goods should be from collective actions of the DAO.

- [Protocol's maintenance and upgrades](https://vitalik.ca/general/2021/08/16/voting3.html#:~:text=The%20need%20for%20DeGov%20for%20protocol%20maintenance%20and%20upgrades)

Protocol's maintenance and upgrades is a term used on Web3 decentralized application to upgrade the current protocol's smartcontract through a fork or just change some of it's characteristic.
On general organization, there's a more suitable term for this usecase: "Policy review and amendments".

On the tests, the author will show how Align blueprint package can achieve these use cases with the most security and decentralization through specific examples.

# Tech Stack
## Development
The package is developed by Scrypto v0.6.0 with many advanced modules and design patterns: Runtime, Access Control, Resource Manager, Cross-blueprint Calls, User Badge pattern, Validated Proof, Local Component,...

The package efficiently utilized Rust's Struct type to create a strong typed DAO's structure: [ProposalComponent](proposal::Proposal_impl::Proposal), [CommunityComponent](community::Community_impl::Community),
 [Delegator](delegator::Delegator), [Member](crate::member::DAOMember), [CommitmentPolicy](policies::CommitmentPolicy), [EconomicPolicy](policies::EconomicPolicy), [TreasuryPolicy](policies::TreasuryPolicy), [ProposalPolicy](policies::ProposalPolicy),
[Methods](utils::Methods), [RetirementProcess](member::RetirementProcess).

The author used "hex" rust crate for a novel design pattern: General Scrypto type, where you can input, output general Scrypto type on function or method through making use of the hex rust crate.

The package used an extra test method: "encoder()" to encode method argument for input General Scrypto Type on transaction manifest:
```
use scrypto::prelude::{args, dec};

let data = args!(dec!("0.5"));
let hex = hex::encode(&data);
assert_eq!(data, hex::decode(&hex).expect("cannot decode hex"));
println!("{}", hex);
```

Detail explainations contain on each module of the package:
- [AlignDAO](crate::align_dao): Core module of the blueprint package, explain about 4 main features of the DAO and how it would be used to achieve above design goals, implement [DAO](crate::align_dao::DAO_impl::DAO) component for interacting with users or other modules.
- [Member](crate::member): implement [DAOMember](crate::member::DAOMember) struct to store the DAO Member's data and many helpful methods for user pattern and the voting mechanism.
- [Delegator](crate::delegator): implement [Delegator](crate::delegator::Delegator) struct to store the Delegator data and many helpful methods for user pattern.
- [Community](crate::community): implement [Community](community::Community_impl::Community) component for the DAO's Liquid Democracy technique.
- [Policies](crate::policies): implement many policy structs: [TreasuryPolicy](crate::policies::TreasuryPolicy), [EconomicPolicy](crate::policies::EconomicPolicy),
[ProposalPolicy](crate::policies::ProposalPolicy), [CommunityPolicy](crate::policies::CommunityPolicy), [CommitmentPolicy](crate::policies::CommitmentPolicy) which will govern the DAO's smartcontract logic.
- [Proposal](crate::proposal): implement [Proposal](crate::proposal::Proposal_impl::Proposal) component to store vote history and manage the DAO's collective actions.
- [Treasury](crate::treasury): implement [Treasury](crate::treasury::Treasury_impl::Treasury) local component to store and manage the DAO's assets.
- [LocalOracle](crate::local_oracle): implement [LocalOracle](crate::local_oracle::LocalOracle_impl::LocalOracle) component to store Oracle badge for using off-chain unix time data on the DAO.
- [utils](crate::utils): implement two helpful function: [assert_rate](crate::utils::assert_rate) and [expo](crate::utils::expo). Also implement two helpful struct: [Methods](crate::utils::Methods) and [Method](crate::utils::Method) to support general method call.
- [AlignProject](crate::align_project): A blueprint that the author used for testing convenient, use to create the Align token which later become the core vote unit on the DAO and the DAO badge which later represent the DAO authority.
- [TestFundRaising](crate::test_fund_raising): A blueprint that the author used for testing.
- [TestProposal](crate::test_proposal_comp): A blueprint that the author used for testing.

## Test environment
For testing real usecases of the DAO created by Align blueprint package, the package used two extra blueprints: [TestFundRaising](crate::test_fund_raising) and [TestProposal](crate::test_proposal_comp).

The test package are built by bash script on the `test` folder. There are total 8 comprehensive tests:

- `test/test_user_and_withdraw_pattern.sh`: Test Align DAO's user and withdraw patterns, include DAO Member user pattern, Delegator user pattern, Community member pattern (on Community blueprint).
- `test/test_commitment_voting_mechanism.sh`: Test Align DAO's Commitment Voting Mechanism.
- `test/test_liquid_democracy_mechanism.sh`: Test Align DAO's Liquid Democracy.
- `test/test_prm_quorum_voting_mechanism.sh`: Test Align DAO's Permissioned Relative Majority & Quorum Voting Mechanism.
- `test/test_internal_treasury_function.sh`: Test Align DAO's Internal Treasury Function.
- `test/test_tokenomic.sh`: Test Align DAO's Tokenomic.
- `test/test_distribution_proposal.sh`: Test Align DAO's Distribution Proposal.
- `test/test_real_use_case.sh`: Test Align DAO's real use cases, include "funding public goods" test and "protocol's maintenance and upgrades" test as mentioned above.

To run all the test, run file `../tests.sh`

The test included detail info logs for testers to clearly understand what's happening when running the test on resim.

The test used many extra helpful bash script scrypto unit function for testing:
- assert_failure: Assert if the transaction receipt is failure and log the error message.
- assert_success: Assert if the transaction receipt is success.
- inspect: Immediately stop the test track and inspect the provided address (by `resim show [address]`).
- info: Show all the info logs on the transaction receipt.
- error: Show all the error logs on the transaction receipt.

Tester can test changing the protocol configs on script file `test/init.sh` or changing many params, configs on each comprehensive test.

For more information about these voting mechanisms or DeGov techniques, please check [align_dao](crate::align_dao) module.

### Real use case test
`test/test_real_use_case.sh`

The real use case test go through a specific example on public good funding and protocol's maintainance, upgrade:
- Current Align DAO has 3 DAO Members: Alice, Bob and Kevin; 2 Delegators: Lyn, Bond.
- Kevin want to build a Car Assembly Workshop for the DAO and will leverage funding for the project through a [Fund Raising Platform](crate::test_fund_raising).
- He instantiate and globalize the Fund Raising platform component, as well as the proposal component to atomic call many methods which would need the DAO's authority.
- He then create a proposal for the DAO to fund through the Fund Raising Component, and at the same time distribute the "bond" token for all support voters of his proposal.
- After the proposal has successfully passed, he then create another proposal for the DAO to assign him the fund from the Fund Raising Component.
- After it also passed, he then use the fund to make profit, and deposit the profit along with initial fund to the Fund Raising Component to benefit all shareholders.
- Now he has successfully stablized the DAO's business and would not need much fund like before,
 he then propose for the DAO to raise the fund management fee on Fund Raising Component.

## Transaction manifest
All useful transaction manifests to interact with the Align DAO smartcontract is included on `rtm` folder, transaction manifests are divided into 4 groups:
- `rtm/init/`: Contain 4 transaction manifests and 2 blobs used to boostrap the Align DAO smartcontract and test users.
- `rtm/dao/`: Contain 7 sub-groups of transaction manifests to interact with the [DAO Component](crate::align_dao). 
Also contain 2 extra non-callable transation manifests to test malicious user pattern.
- `rtm/community/`: Contain 3 sub-groups of transaction manifests to interact with the [Community Component](crate::community).
- `rtm/proposal/`: Contain 3 sub-groups of transaction manifests and 1 transaction manifest to interact with the [Proposal Component](crate::proposal).

Detail about transaction manifest sub-groups on each blueprint module documentation.

# License
This project is licensed under [Apache 2.0](https://github.com/radixdlt/scrypto-challenges/blob/main/LICENSE).

The author can be contacted on discord by name: `Peter Kim#9374`
*/

mod align_dao;
mod align_project;
mod community;
mod delegator;
mod local_oracle;
mod member;
mod policies;
mod proposal;
mod test_fund_raising;
mod test_proposal_comp;
mod treasury;
mod utils;

/// The encoder to help encode Scrypto Type to hex
#[test]
pub fn encoder() {
    use scrypto::prelude::{args, dec};

    let data = args!(dec!("0.5"));
    let hex = hex::encode(&data);
    assert_eq!(data, hex::decode(&hex).expect("cannot decode hex"));
    println!("{}", hex);
}
