//! A configurable DAO that can control any badge-administrated
//! component.
//!
//! The SmorgasDAO allows you to choose between a number of different
//! ways of reaching consensus on decisions, and when a decision has
//! been made can make a call to an external component to initiate
//! some administrative action on that component. The necessary badges
//! to establish authority can be passed along in the call. This
//! allows the SmorgasDAO to plug into basically any existing
//! badge-controlled component and change it from being administrated
//! by a central team to being a true DAO.
//!
//! # Advisory Proposals
//!
//! When the DAO completes an advisory proposal it does no further
//! action other than to determine its outcome and publish this on the
//! ledger. It is up to the humans of the DAO to determine what the
//! outcome means and to act on it.
//!
//! It is expected that there may be one or several advisory proposals
//! leading up to an executive proposal (below), hashing out what the
//! execution needs to do and the details of how it should be done.
//!
//! # Executive Proposals
//!
//! Upon the completion of an executive proposal the DAO will call
//! some external component on the ledger, potentially passing along
//! the authority of its various badges. It can send this authority as
//! Proof objects or even send them in Buckets. It can also send XRD
//! from its own funds.
//!
//! In principle the proposer of an executive proposal will write an
//! entirely new component that implements the thing he wants the DAO
//! to do. This component needs to be published on the ledger before
//! he creates the proposal and the source code ought to be made
//! available to the DAO members so they can make an informed decision
//! when casting their vote. DAO members are advised to carefully
//! inspect the code of the component to be called, and maybe the DAO
//! should even hire external auditors to help them decide if the code
//! is safe to run before voting on it.
//!
//! ## How the DAO can control another component
//!
//! Normally a component on the ledger will have a number of
//! administrative methods used to control sensitive aspects of the
//! component's configuration and daily running. These methods will be
//! guarded by various types of admin badges that are in the
//! possession of the component's admin team. This is a centralized
//! model of control. The SmorgasDAO can take over the responsibility
//! of that admin team by having the admin badges stored in the
//! SmorgasDAO component, out of the hands of an admin team. This
//! converts centralized control into a DAO.
//!
//! SmorgasDAO can have any number of external badges stored within it
//! and so will be able to control components that use multiple
//! different admin badges, as well be able to control many different
//! components from the same DAO.
//!
//! In the following we call the SmorgasDAO the "DAO component" and
//! the component having been put under the control of the DAO the
//! "controlled component".
//!
//! Calls made by the DAO will (usually) pass through an intermediary
//! component that receives the DAO's proofs and badges and makes use
//! of these in calls towards the controlled component. The method to
//! call in the intermediary component has a name chosen by the
//! proposer, and must adhere to a strict signature that focuses on
//! sending in and receiving back the DAO's badges and funds.
//!
//! The intermediary component will normally be expected to return any
//! badge buckets it received from the DAO component at the end of the
//! call, but it is perfectly valid also to use the call to extract
//! badges from the DAO in which case those ones will not be returned.
//!
//! ## How the DAO can configure itself
//!
//! In much the same way as the DAO can use stored badges to call
//! third-party components, it can also use its own stored admin badge
//! to call itself via an intermediary component. There is a
//! complication here as the intermediary cannot call the DAO back
//! directly (this would be an invalid re-entrant call) but instead it
//! must store the admin token awaiting a second call from the
//! transaction manifest which then calls the DAO to do the
//! configuration change, before returning the admin badge to the DAO.
//!
//! Note that the current DAO implementation only offers a single
//! configuration method, for the `proposal_duration` option, but of
//! course it could be expanded to support others that interest us.
//!
//! # DAO Configuration Options
//!
//! SmorgasDAO has a number of configuration options to allow for a
//! wide variety of use cases.
//!
//! ## Voting subsidies
//!
//! The DAO can be configured to subsidize the voting transactions of
//! its members. This subsidy is taken from the DAO's funds and there
//! needs to be some strategy to keep these topped up. Such a strategy
//! is outside the scope of the DAO mechanism itself.
//!
//! ## Tally types
//!
//! We offer two different ways of tallying votes. One is one token
//! equals one vote, which is the most straightforward but also can
//! tend to give whales a lot of power - some would say too much. The
//! other is one person equals one vote, in which case how many vote
//! tokens you have doesn't matter so much. This latter method does
//! however depend on having a strong mechanism for ensuring that any
//! one person only has at most one identity NFT. The DAO mechanism
//! does not itself offer a solution to this.
//!
//! ## Quorum
//!
//! We offer three different types of quorum: not caring about quorum
//! at all; or a minimum percentage of votes cast relative to total
//! supply of the vote tokens; or a fixed minimum number of votes cast
//! in total.
//!
//! ## Proposal Duration
//!
//! Proposals last for a number of epochs configured on the DAO, by
//! which times all votes must have been cast. Once the duration has
//! expired for any given proposal anyone can call the
//! `execute_proposal` method to have the DAO record the result.
//!
//! # About Voting Tokens
//!
//! When votes are cast in the DAO they are always cast with voting
//! tokens. It is recommended that the tokens used should be pure
//! governance tokens without any other use case as they need to be
//! bound up in the DAO for each proposal. People would be upset if
//! they had to pull yield tokens out of a farm in order to bind them
//! to a vote so we recommend against getting into such situations.
//!
//! The general procedure is for a proposal to be posted, for a DAO
//! member to bind a number of voting tokens towards that proposal,
//! and for them to recover those voting tokens after the proposal has
//! been decided. Bound voting tokens are held within vaults in the
//! DAO component.
//!
//! If multiple proposals have the same deadline, DAO members must
//! decide how to distribute their votes between them since each
//! proposal has its own set of vaults for binding tokens. Proposers
//! may want to make effort to avoid such collisions.
//!
//! # About Identities
//!
//! The DAO may run with or without requiring identity tokens from its
//! members. If it runs without identity tokens then it will issue
//! voting receipts when votes are cast, and these receipts
//! effectively serve as temporary identity tokens of sorts for
//! recoving bound voting tokens after the proposal has ended.
//!
//! If the DAO runs with identity tokens than those are used for
//! recovering voting tokens.

use scrypto::prelude::*;
use std::collections::BTreeSet;

/// This is an empty structure used as data in our voting receipt
/// NFTs.
#[derive(NonFungibleData)]
struct VoteReceipt {}

/// Allows us to specify which badges to send when calling another
/// component in an executive proposal.
#[derive(TypeId, Encode, Decode, Describe, Clone, PartialEq)]
pub enum BadgeId {
    /// Send the DAO's admin badge.
    AdminBadge,
    /// Send the DAO's receipt token minting badge.
    IdMintBadge,
    /// Send one of the other NFT badges stored in the DAO.
    ExternalNftBadge(NonFungibleAddress),
    /// Send one of the fungible badges stored in the DAO.
    ExternalFungibleBadge(ResourceAddress, Decimal),
}

/// Determines whether to subsidize voter transaction costs, and by
/// how much. The `Decimal` argument is how many XRD to subsidize each
/// voting transaction with. Subsidies come out of DAO funds.
#[derive(TypeId, Encode, Decode, Describe, Clone, PartialEq)]
pub enum VoteSubsidy {
    /// Don't subsidize voters.
    NoSubsidy,

    /// Subsidize the first vote someone places on a given proposal,
    /// but not any further votes the same person places.
    SubsidizeFirstVote(Decimal),

    /// Subsidize all votes placed. This may open the DAO up to attack
    /// by someone spamming vote/retract vote/vote/retract vote
    /// etc. and so should be chosen with great care.
    SubsidizeAllVotes(Decimal),
}

/// Decides how much weight to give votes towards reaching a decision.
#[derive(TypeId, Encode, Decode, Describe, Clone, PartialEq)]
pub enum TallyType {
    /// One voting token equals one vote. A whale will have more
    /// voting power than your regular hodler.
    Linear,

    /// One voter equals one vote, no matter how many voting tokens
    /// are cast for that vote. This only makes sense when the DAO is
    /// configured to use identity tokens, and further when
    /// distribution of those identity tokens is under strict control
    /// so that it's difficult or impossible for someone to hoard
    /// multiple identity tokens.
    Unity,
}

/// Used to define how many votes must be cast in total in order to
/// achieve quorum. A proposal can only have a valid result if it
/// manages to reach quorum. Note that in deciding quorum we always
/// count the total voting tokens cast, regardless of tally type
/// chosen. This means that even under `Unity` tally, a single whale
/// voter will benefit from being a whale in that he contributes more
/// towards quorum than smaller voters, even if ultimately all of his
/// tokens still only count as a single vote for the final decision.
#[derive(TypeId, Encode, Decode, Describe, Clone, PartialEq)]
pub enum Quorum {
    /// Require that a certain percent of all existing tokens be cast.
    /// For example if there are 1 million tokens in existence and
    /// this is set to 10% then any proposal that has less than 100k
    /// tokens cast in total will fail to meet quorum.
    Percent(Decimal),

    /// Require that a certain minimum number of tokens be cast. For
    /// example if this is set to 500k then if fewer than 500k tokens
    /// are cast in total the proposal will fail to meet quorum.
    Fixed(Decimal),

    /// Don't enforce a minimum quorum for this DAO.
    Any
}

/// Enumerates the different kinds of proposal we support.
///
/// Advisory proposals have no technical effect in the DAO component
/// other than being registered with a vote result. They may have
/// substantial effect in the real life portion of the DAO however.
///
/// Executive proposals, if they pass, lead to the DAO component
/// making a call into a different component, potentially sending
/// along funds and badges so that the other component can perform
/// some activity on behalf of the DAO.
#[derive(TypeId, Encode, Decode, Describe, Clone, PartialEq)]
pub enum ProposalType {
    Advisory,
    Executive,
}

/// This structure is used for ordering proposals by their deadline,
/// so that we can efficiently look up the next proposal that is
/// due. For this purpose it is important that `epoch` is first in the
/// struct since the default implementation of `Ord` gives highest
/// precedent to earlier fields.
#[derive(TypeId, Encode, Decode, Describe, PartialEq, Eq, PartialOrd, Ord)]
pub struct EpochAndPropId {
    /// Deadline epoch
    epoch: u64,
    /// Id of proposal
    prop_id: u64,
}

/// This represents the votes cast by a single identity, and holds the
/// voting tokens in a vault until the proposal concludes.
#[derive(TypeId, Encode, Decode, Describe, PartialEq)]
pub struct VotesCast {
    /// The voting tokens being cast.
    votes: Vault,
    /// The option being voted for.
    option: usize,
}

/// Each proposal is represented by this structure.
#[derive(TypeId, Encode, Decode, Describe, PartialEq)]
pub struct Proposal {
    /// This is the last epoch in which we accept votes. The epoch
    /// after this we allow the `execute_proposal` method to be
    /// called.
    deadline: u64,

    /// Unique id for this proposal.
    id: u64,

    /// What kind of proposal this is, either `Executive` or
    /// `Advisory`.
    ptype: ProposalType,

    /// The identity of the person creating this proposal. If it was
    /// an anonymous proposal then this is set to `None`.
    proposer: Option<NonFungibleAddress>,

    /// Short synopsis, headling format, of the proposal.
    title: String,

    /// The full proposal description, should include detailed
    /// description of all the options.
    pitch: String,

    /// The options presented. Each should be a very short text
    /// inteded for use in a button or similar in a front-end. The
    /// texts used should refer back to the full proposal `pitch`.
    ///
    /// There must be at least one option provided. (Why would you
    /// even want to run a proposal with only one option? Memes I
    /// guess.)
    options: Vec<String>,

    /// This is the final decision made by the DAO on this proposal.
    ///
    /// Note the peculiar nested Options. This is indicative of the
    /// author not having taken the time to create a more intuitive
    /// data structure to represent this.
    ///
    /// Interpret it as follows.
    ///
    /// 1. If the outer Option is `None` then a decision has not yet
    /// been made on this proposal.
    ///
    /// 2. If the outer Option is `Some` then a decision has been
    /// made.
    ///
    /// 3. If the inner Option is `None` then the proposal did not
    /// reach quorum and there is no binding result from the DAO.
    ///
    /// 4. If the inner Option is `Some` then it contains the binding
    /// decision of the DAO.
    decision: Option<Option<usize>>,

    /// These are all the votes cast so far, keyed by the identify of
    /// the voter.
    ///
    /// Note that after a decision has been made voters will withdraw
    /// their votes and so if you want to reconstruct the full vote
    /// count you will need to go back to an earlier snapshot of the
    /// component state.
    votes_cast: HashMap<NonFungibleAddress, VotesCast>,

    /// For an executive proposal, this is the component that will be
    /// called.
    target_component: Option<ComponentAddress>,

    /// For an executive proposal, this is the function or method that
    /// will be called.
    target_method: Option<String>,

    /// For an executive proposal, these proofs will be sent to the
    /// call being made.
    target_proofs: Vec<BadgeId>,

    /// For an executive proposal, these badges will be sent in
    /// buckets to the call being made.
    target_buckets: Vec<BadgeId>,

    /// For an executive proposal, this amount of XRD will be sent to
    /// the call being made. The XRD is taken out of DAO funds.
    target_funding: Option<Decimal>,
}

blueprint! {
    struct SmorgasDao {
        /// This holds the XRD we use for our vote subsidies and for
        /// sending to executive proposal function calls.
        funds: Vault,

        /// This is a map of all our proposals, both ongoing and
        /// settled ones.
        proposals: HashMap<u64, Proposal>,

        /// The next proposal created will receive this id, and then
        /// it increments.
        next_proposal_id: u64,

        /// This is an efficient data structure for finding the
        /// proposal whose deadline expires the soonest.
        active_proposals: BTreeSet<EpochAndPropId>,

        /// This is a list of the ids of all our settled proposals.
        closed_proposals: Vec<u64>,

        /// The number of epochs the DAO has to vote on a proposal
        /// before it closes.
        proposal_duration: u64,

        /// Set to `None` to allow anonymous proposals. Otherwise this
        /// is the resource address used for the tokens that impart
        /// proposal authority.
        proposal_requires_token: Option<ResourceAddress>,

        /// If `proposal_requires_token` is set then this is the
        /// number of those tokens needed for you to be allowed to
        /// make a proposal.
        proposal_required_quantity: Option<Decimal>,

        /// Holds the rule we use for determining quorum.
        quorum: Quorum,

        /// The token used for voting.
        vote_token: ResourceAddress,

        /// How and whether to offer tranasction cost subsidy to our
        /// voters.
        vote_subsidy: VoteSubsidy,

        /// If an `id_token` is set then only people who can prove an
        /// NFT of this type are allowed to vote.
        id_token: Option<ResourceAddress>,

        /// How we tally votes.
        vote_tally: TallyType,

        /// If we allow anonymous voting then this is the receipts NFT
        /// token we use.
        receipts_resource: Option<ResourceAddress>,

        /// This is the badge used for minting our receipts tokens.
        /// It is held internally in the component.
        id_mint_badge: Vault,

        /// This is the badge used to perform administrative calls to
        /// the DAO component.
        /// It is held internally in the component.
        admin_badge: Vault,

        /// This are other badges we have been provided with which can
        /// be used when running executive proposals.
        external_badges: HashMap<ResourceAddress, Vault>,
    }

    impl SmorgasDao {

        /// Creates a new DAO.
        ///
        /// Panics if it doesn't like the parameters you send it.
        ///
        /// ---
        ///
        /// **Access control:** Anyone can make a DAO
        ///
        /// **Transaction manifest:**
        /// `rtm/smorgasdao/instantiate_smorgasdao.rtm`
        /// ```text
        #[doc = include_str!("../rtm/smorgasdao/instantiate_smorgasdao.rtm")]
        /// ```
        pub fn instantiate_smorgasdao(proposal_duration: u64,
                                      quorum: Quorum,
                                      vote_token: ResourceAddress,
                                      id_token: Option<ResourceAddress>,
                                      vote_tally: TallyType,
                                      vote_subsidy: VoteSubsidy,
                                      proposal_requires_token: Option<ResourceAddress>,
                                      proposal_required_quantity: Option<Decimal>)
                                      -> (ComponentAddress, ResourceAddress) {
            // quantity makes no sense without a token to have a quantity of
            assert!(proposal_requires_token.is_some() || proposal_required_quantity.is_none(),
                    "Cannot require a quantity of tokens without giving a token type");

            // if we require a token we must also require a quantity,
            // except that, if the token is an NFT we will default to
            // 1 if quantity is None so allow that
            if proposal_requires_token.is_some() && proposal_required_quantity.is_none() {
                Self::assert_is_non_fungible(proposal_requires_token,
                                             "Proposal required token must be an NFT when no quantity is given");
            }

            Self::assert_is_non_fungible(id_token, "ID token must be an NFT");

            match quorum {
                Quorum::Percent(p) => assert!((p.is_zero() || p.is_positive()) && p <= dec!("100"),
                                              "Quorum percent out of bounds: {}", p),
                Quorum::Fixed(f) => assert!(f.is_zero() || f.is_positive(),
                                            "Quorum fixed out of bounds: {}", f),
                Quorum::Any => {},
            }

            match vote_subsidy {
                VoteSubsidy::NoSubsidy => { /* this is always ok */ },
                VoteSubsidy::SubsidizeFirstVote(s) | VoteSubsidy::SubsidizeAllVotes(s) =>
                    assert!(s.is_positive(), "Subsidy should be positive"),
            }

            if let VoteSubsidy::SubsidizeFirstVote(_) = vote_subsidy {
                assert!(id_token.is_some(),
                    "First vote subsidy only available with an id_token");
            }

            let id_mint_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name",
                          "SmorgasDAO id mint badge")
                .initial_supply(1);

            let mut receipts_resource = None;
            if id_token.is_none() {
                receipts_resource =
                    Some(ResourceBuilder::new_non_fungible()
                         .metadata("name", "SmorgasDAO receipt NFT".to_string())
                         .mintable(rule!(require(id_mint_badge.resource_address())), LOCKED)
                         .burnable(rule!(require(id_mint_badge.resource_address())), LOCKED)
                         .no_initial_supply());
            }

            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name",
                          "SmorgasDAO admin badge")
                .initial_supply(1);

            let admin_addr = admin_badge.resource_address();
            let auth: AccessRules = AccessRules::new()
                .default(rule!(require(admin_addr)))
                .method("fund", rule!(allow_all))
                .method("create_proposal", rule!(allow_all))
                .method("vote_with_receipt", rule!(allow_all))
                .method("vote_with_id", rule!(allow_all))
                .method("withdraw_votes_with_receipt", rule!(allow_all))
                .method("withdraw_votes_with_id", rule!(allow_all))
                .method("execute_proposal", rule!(allow_all))
                .method("add_external_badges", rule!(allow_all))
                .method("read_proposal_duration", rule!(allow_all))
                .method("read_proposal_result", rule!(allow_all))
                .method("return_internal_badge", rule!(allow_all))
                ;

            let mut smorgasdao =
                Self {
                    funds: Vault::new(RADIX_TOKEN),
                    proposals: HashMap::new(),
                    next_proposal_id: 0,
                    active_proposals: BTreeSet::new(),
                    closed_proposals: Vec::new(),
                    proposal_duration,
                    proposal_requires_token,
                    proposal_required_quantity,
                    quorum,
                    vote_token,
                    id_token,
                    vote_tally,
                    vote_subsidy,
                    receipts_resource,
                    id_mint_badge: Vault::with_bucket(id_mint_badge),
                    admin_badge: Vault::with_bucket(admin_badge),
                    external_badges: HashMap::new(),
                }
            .instantiate();
            smorgasdao.add_access_check(auth);

            (smorgasdao.globalize(), admin_addr)
        }

        /// Adds funds to the DAO. They must be XRD.
        ///
        /// ---
        ///
        /// **Access control:** Anyone can help fund the DAO.
        ///
        /// **Transaction manifest:**
        pub fn fund(&mut self, funds: Bucket) {
            self.funds.put(funds);
        }

        /// Creates a new proposal and returns its unique proposal id.
        ///
        /// Will panic if you're not allowed to do this.
        ///
        /// ---
        ///
        /// **Access control:** If configured to do so, will require
        /// the `authority` proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/smorgasdao/create_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/smorgasdao/create_proposal.rtm")]
        /// ```
        pub fn create_proposal(&mut self,
                               authority: Option<Proof>,
                               ptype: ProposalType,
                               options: Vec<String>,
                               title: String,
                               pitch: String,
                               deadline: u64,
                               target_component: Option<ComponentAddress>,
                               target_method: Option<String>,
                               target_proofs: Vec<BadgeId>,
                               target_buckets: Vec<BadgeId>,
                               target_funding: Option<Decimal>)
                               -> u64
        {
            // If you provide authority when none is needed you are
            // confused and need to be rescued from yourself. If you
            // fail to provide authority when it's needed then
            // obviously we can't have that.
            assert!(authority.is_some() == self.proposal_requires_token.is_some(),
                    "Authority mismatch");

            let mut proposer: Option<NonFungibleAddress> = None;

            // Check proposal token and quantity if configured to do so
            if let Some(required_tok) = self.proposal_requires_token {
                let required_q = self.proposal_required_quantity.unwrap_or(Decimal::ONE);
                if let Ok(authority) = authority.unwrap().validate_proof(
                    ProofValidationMode::ValidateContainsAmount(required_tok, required_q))
                {
                    if borrow_resource_manager!(authority.resource_address()).resource_type() ==
                        ResourceType::NonFungible
                    {
                        // We need to pick an NFT to be "the proposer"
                        let nfids = authority.non_fungible_ids();
                        let proposer_nfid = nfids.iter().next().unwrap();
                        proposer = Some(NonFungibleAddress::new(authority.resource_address(),
                                                                proposer_nfid.clone()));
                    }
                } else {
                    panic!("You lack authority to make a proposal");
                }
            }

            assert!(deadline >= Runtime::current_epoch()
                    && deadline <= Runtime::current_epoch() + self.proposal_duration,
                    "Deadline out of bounds");

            let mut options = options;
            match ptype {
                ProposalType::Executive => {
                    // checks that options is empty and sensible targets are given
                    assert!(options.len() == 0,
                            "You cannot specify options on an Executive proposal.");
                    assert!(target_component.is_some(),
                            "An executive proposal must have a target component");
                    assert!(target_method.is_some(),
                            "An executive proposal must have a target function/method");
                    options = vec!["Refuse the proposal".to_string(), "Accept the proposal".to_string()];
                },
                ProposalType::Advisory => {
                    // check that options has at least one option, and no targets given
                    assert!(options.len() >= 1,
                            "Advisory proposal must have at least one option");
                    assert!(target_component.is_none()
                            && target_method.is_none()
                            && target_proofs.len() == 0
                            && target_buckets.len() == 0
                            && target_funding.is_none(),
                            "Advisory proposal cannot have a target");
                }
            }

            // Make proposal
            let id = self.next_proposal_id;
            let proposal = Proposal {
                deadline,
                id,
                ptype,
                proposer,
                title,
                pitch,
                options,
                decision: None,
                votes_cast: HashMap::new(),
                target_component,
                target_method,
                target_proofs,
                target_buckets,
                target_funding
            };
            self.next_proposal_id += 1;
            self.active_proposals.insert(EpochAndPropId{epoch: deadline, prop_id: id});
            self.proposals.insert(id, proposal);

            id
        }

        /// Call this method to vote on a DAO that allows anonymous
        /// voting.
        ///
        /// Provide the unique id of the proposal to vote on, the
        /// bucket of tokens you want to bind up to this vote, and the
        /// id of the option you're voting for.
        ///
        /// You will be returned a receipt NFT which you will need
        /// later to recover your voting tokens.
        ///
        /// ---
        ///
        /// **Access control:** If the DAO is configured to require an
        /// id, no one can call this method.
        ///
        /// **Transaction manifest:**
        /// `rtm/smorgasdao/vote_with_receipt.rtm`
        /// ```text
        #[doc = include_str!("../rtm/smorgasdao/vote_with_receipt.rtm")]
        /// ```
        pub fn vote_with_receipt(&mut self,
                                 proposal: u64, tokens: Bucket, vote_for: usize) -> Bucket {
            assert!(self.id_token.is_none(),
                    "We only accept votes with an id NFT");

            self.check_voting_parameters(proposal, &tokens, vote_for);

            let proposal = self.proposals.get_mut(&proposal).expect(
                "This proposal does not exist");

            // Create receipt
            let receipt_nfid: NonFungibleId = NonFungibleId::random();
            let receipt_nft: Bucket = self.id_mint_badge.authorize(||
                borrow_resource_manager!(self.receipts_resource.unwrap())
                    .mint_non_fungible(
                        &receipt_nfid,
                        VoteReceipt{}
                    )
            );
            let receipt_nfaddr = NonFungibleAddress::new(
                receipt_nft.resource_address(), receipt_nfid);

            // Give subsidy if permitted
            if let VoteSubsidy::SubsidizeAllVotes(amount) = self.vote_subsidy {
                self.funds.lock_contingent_fee(amount);
            }

            // Record votes
            proposal.votes_cast.insert(receipt_nfaddr, VotesCast {
                option: vote_for,
                votes: Vault::with_bucket(tokens) });

            receipt_nft
        }

        /// Call this to pull out your votes from a proposal when you
        /// voted without providing an identity token. You can do this
        /// before voting is over to abandon voting on this proposal,
        /// or you can do it after the proposal has ended.
        ///
        /// Provide the unique proposal id you previously voted on and
        /// the receipt NFT you received when voting.
        ///
        /// You will be returned your voting tokens.
        ///
        /// Will panic if this DAO doesn't use anonymous voting, or if
        /// there's something wrong with the data you send in.
        ///
        /// ---
        ///
        /// **Access control:** If the DAO is configured to require an
        /// id, no one can call this method.
        ///
        /// **Transaction manifest:** This method is not in the test
        /// suite and does not yet have a transaction manifest.
        pub fn withdraw_votes_with_receipt(&mut self,
                                           proposal: u64,
                                           id: Bucket) -> Bucket {
            assert!(self.id_token.is_none(),
                    "We do not use receipts");

            assert_eq!(self.receipts_resource.unwrap(), id.resource_address(),
                       "Wrong receipt type");

            let id_nfaddr = NonFungibleAddress::new(
                id.resource_address(),
                id.non_fungible_id());
            
            let proposal = self.proposals.get_mut(&proposal).expect(
                "This proposal does not exist");

            let removed_votes =
                proposal.votes_cast.remove(&id_nfaddr);
            
            // burn the temporary id NFT
            self.id_mint_badge.authorize(|| id.burn());

            // return the voting funds
            removed_votes.unwrap().votes.take_all()
        }

        /// Call this method to vote on a DAO that identity NFTs when
        /// voting.
        ///
        /// Provide the unique id of the proposal to vote on, a proof
        /// of your identity, the bucket of tokens you want to bind up
        /// to this vote, and the id of the option you're voting for.
        ///
        /// ---
        ///
        /// **Access control:** If the DAO configured to not require
        /// an id, no one can call this method.
        ///
        /// **Transaction manifest:** This method is not in the test
        /// suite and does not yet have a transaction manifest.
        pub fn vote_with_id(&mut self,
                            proposal: u64, id: Proof, tokens: Bucket, vote_for: usize) {
            assert!(self.id_token.is_some(),
                    "We don't accept votes with an id NFT");

            self.check_voting_parameters(proposal, &tokens, vote_for);

            if let Ok(id) = id.validate_proof(self.id_token.unwrap()) {
                let id_nfaddr = NonFungibleAddress::new(
                    id.resource_address(),
                    id.non_fungible_id());

                let proposal = self.proposals.get_mut(&proposal).expect(
                    "This proposal does not exist");

                // Record votes
                let first_vote = !proposal.votes_cast.contains_key(&id_nfaddr);
                if first_vote {
                    proposal.votes_cast.insert(
                        id_nfaddr.clone(),
                        VotesCast {
                            votes: Vault::with_bucket(tokens),
                            option: vote_for,
                        });
                } else {
                    let voting_record = proposal.votes_cast.get_mut(&id_nfaddr).unwrap();
                    assert!(voting_record.votes.amount().is_zero(),
                            "You must withdraw your existing votes before placing more");
                    voting_record.votes.put(tokens);
                    voting_record.option = vote_for;
                }
                
                // Give subsidy if permitted
                match &self.vote_subsidy {
                    VoteSubsidy::SubsidizeFirstVote(amount) => {
                        if first_vote {
                            self.funds.lock_contingent_fee(*amount);
                        }
                    },
                    VoteSubsidy::SubsidizeAllVotes(amount) => {
                        self.funds.lock_contingent_fee(*amount);
                    },
                    VoteSubsidy::NoSubsidy => {},
                };

            } else {
                panic!("Invalid proof");
            };
        }

        /// Call this to pull out your votes from a proposal when you
        /// voted with an identity token. You can do this before
        /// voting is over to abandon voting on this proposal, or you
        /// can do it after the proposal has ended.
        ///
        /// Provide the unique proposal id you previously voted on and
        /// the identity NFT you used for voting.
        ///
        /// You will be returned your voting tokens.
        ///
        /// Will panic if this DAO uses anonymous voting, or if
        /// there's something wrong with the data you send in.
        ///
        /// ---
        ///
        /// **Access control:** If the DAO configured to not require
        /// an id, no one can call this method.
        ///
        /// **Transaction manifest:** This method is not in the test
        /// suite and does not yet have a transaction manifest.
        pub fn withdraw_votes_with_id(&mut self,
                                      proposal: u64,
                                      id: Proof) -> Bucket {
            assert!(self.id_token.is_some(),
                    "We do not use id tokens");

            if let Ok(id) = id.validate_proof(self.id_token.unwrap()) {
                let id_nfaddr = NonFungibleAddress::new(
                    id.resource_address(),
                    id.non_fungible_id());

                let proposal = self.proposals.get_mut(&proposal).expect(
                    "This proposal does not exist");

                // Note that we leave a voting record with zero
                // funds. This lets us determine later that this
                // identity has already placed a vote before which is
                // useful for enforcing our first vote subsidy policy.
                
                // return the voting funds
                proposal.votes_cast.get_mut(&id_nfaddr).unwrap().votes.take_all()
            } else {
                panic!("Invalid proof");
            }
        }

        /// Call this to make the DAO reach a conclusion on a proposal
        /// that has reached its deadline. The conclusion will be
        /// recorded on ledger, and if it's an executive one that
        /// passes it will perform the external component call
        /// required.
        ///
        /// Will panic if the proposal has already been settled or is
        /// not ready to be settled.
        ///
        /// ---
        ///
        /// **Access control:** Anyone can call this but it will fail
        /// if the proposal has been settled already, or if it's not
        /// yet ready to be settled.
        ///
        /// **Transaction manifest:**
        /// `rtm/smorgasdao/execute_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/smorgasdao/execute_proposal.rtm")]
        /// ```
        pub fn execute_proposal(&mut self, proposal: u64) {
            {
                let proposal = self.proposals.get_mut(&proposal).expect(
                    "This proposal does not exist");

                assert!(Runtime::current_epoch() > proposal.deadline,
                        "The proposal is still being voted on");

                assert!(proposal.decision.is_none(),
                        "The proposal has already been settled");
            }
            
            // Count votes and find the decision
            let decision = Some(self.find_proposal_result(proposal));
            let proposal = self.proposals.get_mut(&proposal).unwrap();
            proposal.decision = decision;

            // If executive and approved, call the function
            if proposal.ptype == ProposalType::Executive
                && decision.is_some()
                && decision.unwrap().is_some()
                && decision.unwrap().unwrap() == 1 {
                // Fetch the proofs we need to send
                let mut proofs: Vec<Proof> = Vec::new();
                let mut badge_buckets: Vec<Bucket> = Vec::new();
                for badge in &proposal.target_proofs {
                    match badge {
                        BadgeId::AdminBadge => proofs.push(self.admin_badge.create_proof()),
                        BadgeId::IdMintBadge => proofs.push(self.id_mint_badge.create_proof()),
                        BadgeId::ExternalNftBadge(nfaddr) => {
                            let badge_bucket =
                                self.external_badges.get_mut(&nfaddr.resource_address()).unwrap()
                                .take_non_fungible(&nfaddr.non_fungible_id());
                            proofs.push(badge_bucket.create_proof());
                            badge_buckets.push(badge_bucket);
                        },
                        BadgeId::ExternalFungibleBadge(addr, amount) => {
                            let badge_bucket =
                                self.external_badges.get_mut(addr).unwrap()
                                .take(*amount);
                            proofs.push(badge_bucket.create_proof());
                            badge_buckets.push(badge_bucket);
                        }
                    }
                }

                // Fetch the buckets we need to send
                let mut buckets: Vec<Bucket> = Vec::new();
                for badge in &proposal.target_buckets {
                    match badge {
                        BadgeId::AdminBadge => buckets.push(self.admin_badge.take_all()),
                        BadgeId::IdMintBadge => buckets.push(self.id_mint_badge.take_all()),
                        BadgeId::ExternalNftBadge(nfaddr) => {
                            let badge_bucket =
                                self.external_badges.get_mut(&nfaddr.resource_address()).unwrap()
                                .take_non_fungible(&nfaddr.non_fungible_id());
                            buckets.push(badge_bucket);
                        },
                        BadgeId::ExternalFungibleBadge(addr, amount) => {
                            let badge_bucket =
                                self.external_badges.get_mut(addr).unwrap().take(*amount);
                            buckets.push(badge_bucket);
                        }
                    }
                }

                // Provide funding if specified
                let funds: Option<Bucket>;
                if let Some(funding) = proposal.target_funding {
                    funds = Some(self.funds.take(funding));
                } else {
                    funds = None;
                }
                
                // Make the external call
                let (mut buckets, funds) =
                    borrow_component!(proposal.target_component.unwrap())
                    .call::<(Vec<Bucket>, Option<Bucket>)>(
                        proposal.target_method.as_ref().unwrap(),
                        args!(
                            proofs,
                            buckets,
                            funds
                        ));

                // Stash away funds returned to us
                if let Some(funds) = funds {
                    self.funds.put(funds);
                }

                // Put badges returned to us in their correct places
                badge_buckets.append(&mut buckets);
                for bucket in badge_buckets {
                    if bucket.resource_address() == self.admin_badge.resource_address() {
                        self.admin_badge.put(bucket);
                    } else if bucket.resource_address() == self.id_mint_badge.resource_address() {
                        self.id_mint_badge.put(bucket);
                    } else if self.external_badges.contains_key(&bucket.resource_address()) {
                        self.external_badges.get_mut(&bucket.resource_address()).unwrap().put(bucket);
                    } else {
                        self.external_badges.insert(bucket.resource_address(), Vault::with_bucket(bucket));
                    }
                }
            }

            // Move proposal out of active_proposals
            self.active_proposals.remove(&EpochAndPropId {
                epoch: proposal.deadline,
                prop_id: proposal.id });
            self.closed_proposals.push(proposal.id);
        }

        /// Adds badges to the DAO which it can later use to establish
        /// authority in executive proposals.
        ///
        /// ---
        ///
        /// **Access control:** Anyone can contribute badges.
        ///
        /// **Transaction manifest:**
        /// `rtm/smorgasdao/add_external_badges.rtm`
        /// ```text
        #[doc = include_str!("../rtm/smorgasdao/add_external_badges.rtm")]
        /// ```
        pub fn add_external_badges(&mut self, badges: Vec<Bucket>) {
            for bucket in badges {
                if self.external_badges.contains_key(&bucket.resource_address()) {
                    self.external_badges.get_mut(&bucket.resource_address()).unwrap().put(bucket);
                } else {
                    self.external_badges.insert(bucket.resource_address(), Vault::with_bucket(bucket));
                }
            }
        }

        /// Returns to the DAO its admin badge or its id mint badge,
        /// after someone borrowed it.
        ///
        /// ---
        ///
        /// **Access control:** Anyone can return badges. They need to
        /// be the correct ones.
        ///
        /// **Transaction manifest:** This is currently only called
        /// from other components and does not have a transaction
        /// manifest.
        pub fn return_internal_badge(&mut self, badge: Bucket) {
            if badge.resource_address() == self.admin_badge.resource_address() {
                self.admin_badge.put(badge);
            } else if badge.resource_address() == self.id_mint_badge.resource_address() {
                self.id_mint_badge.put(badge);
            } else {
                panic!("This isn't our badge");
            }
        }

        /// Change the `proposal_duration` of the DAO.
        ///
        /// ---
        ///
        /// **Access control:** Requires the `admin_badge`.
        ///
        /// **Transaction manifest:** This can only be called from
        /// other components and does not have a transaction manifest.
        pub fn set_proposal_duration(&mut self, duration: u64) {
            self.proposal_duration = duration;
        }

        /// Retrieve the `proposal_duration` of the DAO.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/smorgasdao/read_proposal_duration.rtm`
        /// ```text
        #[doc = include_str!("../rtm/smorgasdao/read_proposal_duration.rtm")]
        /// ```
        pub fn read_proposal_duration(&self) -> u64 {
            self.proposal_duration
        }
        
        /// Retrieve a proposal result.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/smorgasdao/read_proposal_result.rtm`
        /// ```text
        #[doc = include_str!("../rtm/smorgasdao/read_proposal_result.rtm")]
        /// ```
        pub fn read_proposal_result(&self, prop_id: u64) -> Option<Option<usize>> {
            self.proposals.get(&prop_id).expect("No such proposal").decision
        }
        

        //
        // Internal helper functions follow
        //

        /// Converts votes cast into votes counted, based on our tally type
        fn count_votes(&self, tallytype: &TallyType, votes: Decimal) -> Decimal {
            match tallytype {
                TallyType::Linear => votes,
                TallyType::Unity => Decimal::ONE,
                // I wanted to add this but Decimal doesn't seem to support
                // square root
                //TallyType::Quadratic => votes.sqrt(),
            }
        }

        /// Goes through all votes cast and determines what the
        /// winning option is. If it returns `None` then we failed to
        /// meet quorum so there is no result.
        fn find_proposal_result(&mut self, proposal: u64) -> Option<usize> {
            let proposal = self.proposals.get(&proposal).unwrap();

            // Note that the `result` vector we build here will
            // already take tally type into account
            let mut result = vec![Decimal::ZERO; proposal.options.len()];

            // We use PreciseDecimal here and further down because
            // we're multiplying Decimals together and want to prevent
            // overflow when we do so.
            let mut tokens_cast = PreciseDecimal::ZERO;

            for voter in proposal.votes_cast.values() {
                result[voter.option] +=
                    self.count_votes(&self.vote_tally, voter.votes.amount());
                tokens_cast += voter.votes.amount();
            }

            // Determine the winning option.
            //
            // NOTE:
            // 1. In case of ties, the earlier option is preferred
            // 2. In case of zero votes having been cast option 0 is chosen
            // 3. Option 0 is "do nothing" for executive votes so any tie
            //    will result in no action being taken
            // 4. For advisory votes deciding what a tie really means
            //    is a matter of social media drama and outside of
            //    scope for this code. If an advisory vote is intended
            //    to have some legal weight or otherwise be of significance,
            //    presumably a tied vote is non-conclusive and
            //    another vote would be needed.
            //
            // Also note that there is probably a very clever and
            // efficient way you could write the following using Rust
            // collections but it escapes me at the moment.
            let mut highest = Decimal::ZERO;
            let mut winner = 0;
            let mut counter = 0;
            for result in result {
                if result > highest {
                    highest = result;
                    winner = counter;
                }
                counter += 1;
            }

            // Check quorum
            let meets_quorum;
            match self.quorum {
                Quorum::Percent(p) => {
                    let cmgr: &ResourceManager = borrow_resource_manager!(self.vote_token);
                    let supply: PreciseDecimal = cmgr.total_supply().into();
                    if supply.is_zero() {
                        // Avoid divide by zero
                        meets_quorum = p.is_zero();
                    } else {
                        let attendance = PreciseDecimal::from("100") * tokens_cast / supply;
                        meets_quorum = attendance >= p.into();
                    }
                },
                Quorum::Fixed(f) => {
                    meets_quorum = tokens_cast >= f.into();
                },
                Quorum::Any => {
                    meets_quorum = true;
                },
            }

            if meets_quorum {
                return Some(winner);
            } else {
                return None;
            }
        }

        /// Panics if the token is a fungible token.
        fn assert_is_non_fungible(token: Option<ResourceAddress>, msg: &str) {
            if let Some(token) = token {
                assert_eq!(ResourceType::NonFungible,
                           borrow_resource_manager!(token).resource_type(),
                           "{}", msg);
            }
        }


        /// Sanity checks some voting input parameters, causing a
        /// panic if any one seems wrong.
        fn check_voting_parameters(&mut self, proposal: u64, tokens: &Bucket, vote_for: usize)
        {
            let proposal = self.proposals.get_mut(&proposal).expect(
                "This proposal does not exist");

            // Check that tokens is of type vote_token
            assert_eq!(self.vote_token, tokens.resource_address(),
                       "Wrong token for voting with");

            // Check that the proposal is open for voting
            assert!(proposal.decision.is_none(),
                    "This proposal has already been decided");

            assert!(proposal.deadline > Runtime::current_epoch(),
                    "The voting window has ended");

            // Check that the vote option exists
            assert!(vote_for < proposal.options.len(),
                    "Your selected vote option does not exist");
        }

    }
}
