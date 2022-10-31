# dao-kit

The dao-kit package contains a few useful building blocks (blueprints) that make development of DAOs simpler for other
developers. As a toolkit, it is designed to enable DAO builders to hit the ground running and focus their energy on the
core functionality they want to build into their DAO. Boilerplate functionality, such as keeping track of members,
holding votes and even executing code on ledger based on the outcome of those votes, is abstracted away in the
blueprints that the dao-kit package provides.

# Disclaimer

Time is always too short when it comes to Scrypto challenges, which is why some much-needed unit and integration
tests are yet to be implemented!

DO NOT USE IN PRODUCTION!

# The toolkit

The code is itself documented quite extensively, but I will give a short summary here:

The dao-kit package contains 3 main blueprints that are each called `*System` and handle some specific aspect that will
be common to most DAOs. These Systems can be used in isolation to only provide some specific functionality, or they can
be
set up to work together:

- [MemberSystem](scrypto/src/membership_system.rs): This blueprint is designed to keep track of the members of a DAO.
  Each member is represented by a
  non-fungible resource (NFR) that stores the members name and optionally some arbitrary byte encoded data of the
  member. This means that any scrypto-encodable struct can be associated with a member. Each NFR is owned by its
  respective member and thus acts as their membership badge. If combined with the `VotingSystem`, this badge may allow
  members to cast their votes.
- [VotingSystem](scrypto/src/voting_system.rs): The VotingSystem blueprint is designed around the `Vote` NFR
  which represents a vote with multiple
  options that users can vote on. The Vote NFR has been designed to be rather flexible so that many kinds of votes can
  be represented by it. One could for example build a vote, where members of the DAO can decide between 5 colors for the
  DAOs new logo. This wouldn't be a very high profile vote and so it could be configured such that the color with the
  most votes wins, even if only 1 out of 100 members votes. There might also be more high profile votes, for example
  deciding whether to add a new member to the DAO. As this concerns the security of the DAO itself, one might want to
  configure this vote such that at least 2/3 of members have to approve it. Or maybe it is enough if 5 out of
  an arbitrary number of people vote to approve the change. All these constellations and more are possible to implement
  using the voting system.  
  A very common use case that is worth mentioning here, is what is referred to as a "proposal" in the
  code base. A proposal is a special but very common vote configuration, where there is one primary "approve" option and
  one fallback "reject" option. If the "approve" option does not get the required votes, the proposal is automatically
  rejected. This configuration is especially powerful when combined with `CodeExecution`s that run when a proposal is
  approved.
- [CodeExecutionSystem](scrypto/src/code_execution_system.rs):
  Every Option of a vote can be associated with 0, 1 or more code executions that are run if the
  resp. option wins. Code execution can be arbitrary method calls or even function calls. The CodeExecutionSystem is the
  blueprint that facilitates this. What code should be executed is kept track of on each option of a vote. If an option
  has won the vote, it's associated code executions can be given to the CodeExecutionSystem to be run. The
  CodeExecutionSystem takes care of presenting the required badges to the target component. Of course these badges must
  be placed in the custody of the CodeExecutionSystem beforehand.

The dao-kit package contains an additional 4th blueprint called [SimpleDaoSystem](scrypto/src/simple_dao_system.rs).
This blueprint combines all three aforementioned systems in a meaningful way and is the of-the-shelf solution for users
that want to get started with their DAO quickly.
Note that it uses the MembershipSystem and is therefore representing members via NFRs. If members
should be represented by fungible tokens, which may e.g. be traded on an open market, this component should not be used.
As the VotingSystem is designed to work with both fungible and non-fungible voter tokens, it and the CodeExecutionSystem
are still very much useful in this case, though! They then only need to be used directly.

# Security consideration

DAOs can of course take on many shapes and sizes and builders might want to set up different rules, on how their
specific DAO is being governed. However, a central idea that will be common to many DAO project, is that governance
should be some form of a democratic process and that the DAO should not be at the mercy of one single user that happens
to hold the central admin badge. Misplaced trust in combination with power being concentrated in the hands of only a few
people or even a single person has been the downfall of many otherwise sensible projects. Unfortunately, rug pulls and
other missuses of power have become all to common these days.
This is of course a problem that must be addressed at a fundamental level. We have to start building our components and
especially our DAOs in a way that makes them resilient to bad actors. Changes to the DAO should only be performed by
going through an official democratic voting process. This property must be enforced from the DAOs instantiation through
its entire lifetime.

The dao-kit package has been designed around this central idea. The VotingSystem, in combination with the
CodeExecutionSystem, offers an easy and safe way to effect changes only after members have had their say. Each system
offers a default way of instantiating it that ensures that the critical method of the component are protected from
unauthorized use. Next to being useful as a ready-to-use blueprint itself, the `SimpleDaoSystem` component also
demonstrates how the individual systems can be combined safely. When instantiating a SimpleDaoSystem component, the
instantiator ist provided with a single admin badge which allows access to all privileged methods of the individual
systems. The instantiator, which always should be a component (the actual DAO), must then only take care of this badge,
lock it away in a vault and never allow it to be accessed by individual users in any way.

## Configuring of votes and proposals

The VotingSystem offers a multitude of different ways in which votes may be set up. While it is very easy to configure
votes that are secure (e.g. require a majority of member votes) it is also possible to come up with insecure voting
setups. Additionally, the great flexibility of the VotingSystem also means that it is possible to configure votes in a
nonsensical way. Developers should keep this in mind!

Please see the in code documentation of the `VoteConfig` struct and especially of the `WinRequirement` enum as I have
really gone into detail there about the different config options!

## The states of a vote and how code executions work

A vote normally transitions through the following states: `Open` --> `Decided` --> `Implemented`

1. First the vote is `Open` and up to a deadline members can vote. The vote is then evaluated and a winning option is
   determined. If configured so, even multiple options may be accepted together as winning options.
2. The vote is now in state `Decided`. This means the winning option(s) has/have been determined but no code has been
   executed so far. If the vote is in this state, voting is no longer possible and any member that was allowed to vote
   may now send a transaction that implements the vote. This is of course only meaningful if code executions are
   associated with the winning options(s).
3. The vote is now in state `Implemented`, meaning that associated code executions have been run. It can of course not
   be implemented again.

Under certain conditions a vote may fail. This might happen if no option reaches it's required votes and there is
no fallback option configured. It may also happen if two or more options reach an equal number of votes but this is
disallowed by the vote configuration. In these cases an `Open` vote transitions directly into state `Failed`. In this
state code executions are of course not possible.

As just said, when a vote is in state `Decided`, every voter can immediately execute the associated code executions. To
do so, the voter must call the `implement_vote` method of the VotingSystem. They are then provided with a non-fungible
token that contains all code executions that must be run. This token cannot be deposited. The only way to get rid of it,
is to call the CodeExecutionSystem's `execute_code` method, which runs the code executions, takes the token and burns
it. This might seem a little complicated but is necessary to work around reentrancy restrictions imposed by the
Radix engine. In my opinion this is a very cool use case of the restrict-deposit feature ;-)

# Demo

As the dao-kit is a toolkit for builders and not something flashy that can easily be presented on its own, I have built
two very simple blueprints to showcase the toolkit's potential:

- The first blueprint is called the [DoGoodDao](scrypto/src/demo_do_good_dao.rs) and is inspired by
  the [GiveWell NPO](https://www.givewell.org/), which does charity research and recommends the most effective
  charities, i.e. the charities that have the highest impact for every dollar spent.  
  This blueprint has an accompanying test as well as a GUI Demo on Babylon Alphanet
- The second blueprint is the [FlexDao](scrypto/src/demo_flex_dao.rs) which is super simple but really showcases
  the power of the toolkit very well. Definitely take a look at the comments in the last method in that blueprint as I
  have highlighted a very cool feature of the dao-kit toolkit there!    
  This blueprint has an accompanying test but no GUI demo.

## DoGoodDao

In my opinion the crypto space is both super exiting, innovative and fun to be in but at the same time incredibly toxic
and very selfish. Everyone wants to go to the moon and buy themselves 10 lambos. While it is ok to want to make a lot of
money, boasting expensive cars is not something that is particularly helpful to society. Wouldn't it be better if
instead of bragging about expansive possessions, wealthy people would rather brag about how much lives they have saved
by giving to charity? Wouldn't it be far better if our idols were the people doing the most for society rather than the
people who mostly spent money on themselves?

I think blockchain/crypto technology can really do much good in this corner. As humans, we are hard-wired to want to be
respected and recognized by the people around us and by society in general. In many cases, this is the reason why we buy
expensive cars, clothing and luxury items in the first place. Giving to charity is not something that is recognized
prominently enough. Many people do it of course but most don't brag about it. Oftentimes doing so is even seen as
immodest. This is of course nonsense and should be changed. If people are allowed to brag about all the good things they
are doing, that might encourage others to do the same. In my opinion we need a positive culture around this!
Because everything is public on the blockchain, everyone can see how much someone has given to charity and every donor
can also proof how charitable they have been.

In this sense the `DoGoodDao` is a very (very!) small step towards bringing about some positive change. As written
above, it is inspired by the GiveWell NPO. The DAO is administered by a small board of members that performs the actual
research work, identifies the most effective charities and then lists them live on the blockchain in the `DoGoodDao`
component. Donors can register with the DAO and send their donations through the dao towards any of the listed
charities. At any time the movement of funds can be tracked on the blockchain. It can be proven that 100% of donations
have gone to the listed charities and donors have a badge that clearly shows how much they have given to good causes.

While the DAO itself doesn't really reward donors aside from making their charitableness publicly visible, nothing stops
other people from coming in and giving perks to donors. Maybe there is some talented artist who doesn't have a lot of
money but who creates really cool NFTs. They might decide to recognize the donor badges of the `DoGoodDao` in their own
component and let donors redeem cool and unique NFTs that they then can show around and feel good about.

To end on a more technical note:  
The `DoGoodDao` blueprint showcases:

- how the basic DAO functionalities can quickly be onboarded by wrapping a `SimpleDaoSystem` component
- how members can vote on adding new members
- how members can vote on adding, updating or removing charities

A very simple GUI demo running on babylon Alphanet can be found in directory [alphanet-demo](alphanet-demo)

## FlexDao

This is a very simple blueprint without any proper use case whatsoever. It is just there to show the power of the
VotingSystem and the CodeExecutionSystem. I know I'm not really hyping it here, but definitely check it out!

# Technical note

I had intended to separate the core dao-kit blueprints from the demo blueprints but had problems using the `import!`
macro to import the necessary ABIs. As a workaround, I put all blueprints in the same crate. That is of course not
how the dao-kit package is intended to be used. In reality the dao-kit package would only be deployed to the network
once and then be reused from there by simply instantiating the resp. *System components.




