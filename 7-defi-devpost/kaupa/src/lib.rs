//! A trade-anything-for-anything general barter component that
//! supports order book trading pairs as a special case and also
//! provides for flash loans.
//!
//! **kaupa** *(verb, old Norse)* **1.** to buy **2.** to make a
//! bargain
//!
//! The Kaupa component allows you to create instances each of which
//! is a full marketplace within the parameters that you specify. The
//! instance owner can collect fees from trade acticity on that
//! instance.
//!
//! A Kaupa can be a general marketplace for trading arbitrary bags of
//! tokens in return for arbitrary bags of tokens, or it can be more
//! limited in terms of which tokens can be in each bag. In the most
//! pared down configuration you are restricted to trading one
//! specific token for one specific other token (e.g. BTC for XRD). In
//! this simplest case you have a traditional trading pair (BTC/XRD)
//! and Kaupa offers special support for this, providing order book,
//! limit trade and market trade functionality if you want it.
//!
//! Both fungibles and non-fungibles are fully supported, and if you
//! wanted to you could for example set up a non-fungible to
//! non-fungible trading pair where people do limit and market orders
//! on e.g. a RaDragon/RaEgg NFTs trading pair.
//!
//! # The Kaupa Instance
//!
//! Each Kaupa instance consists of a number of trade proposals made
//! by market traders. A proposal consists of a bag of tokens that the
//! trader has provided into the component, and a list of tokens (and
//! quantities) that they would like in return for that bag. This is a
//! completely generic type of trade that allows people to trade any
//! set of tokens for any other set of tokens.
//!
//! A trade proposal can be targeted at a specific other trader (by
//! specifying an NFT that the other trader must use to identify
//! themselves) in which case it is a peer-to-peer OTC transaction. Or
//! it can be untargeted in which case it's part of an open
//! marketplace for anyone to jump into.
//!
//! # A Trade Example
//!
//! A typical trade goes like this:
//!
//! - Alice calls the [make_proposal] method, providing buckets of
//! tokens she wants to sell (e.g. 10,000 VKC and 5 RaDragon NFTs) and
//! specifying the payment she expects for this (e.g. 2000 XRD).
//!
//! - Bob comes along and calls the [accept_proposal] method sending
//! along buckets with payment---in this case the 2000 XRD. Since that
//! is enough to cover what Alice is asking, her buckets of 10,000 VKC
//! and 5 RaDragons are immediately returned out of that method to
//! Bob. The 2000 XRD payment is stored for Alice within the Kaupa
//! instance.
//!
//! - Alice calls the [collect_funds] method to collect her payment of
//! 2000 XRD (and anything else she also has waiting for her).
//!
//! - The owner of this Kaupa instance, if they have set fees, later
//! calls the [collect_funds] method to retrieve outstanding fees. (In
//! which case Alice and Bob also had to provide sufficient funds to
//! cover fees in their calls above.)
//!
//! Above, Alice must identify herself when making the proposal and
//! when collecting funds. She can use any NFT she owns to do so
//! although of course she must use the same one for the two calls.
//!
//! Likewise, the Kaupa owner must identify in order to collect fees,
//! using the NFT they specified when instantiating the Kaupa.
//!
//! Bob did not need to identify in this case since Alice's proposal
//! was unrestricted.
//!
//! # Kaupa's Public Face
//!
//! Kaupa presents these functions to the world:
//!
//! - [instantiate_kaupa] Creates a new Kaupa instance.
//!
//! - [make_proposal] Adds a trade proposal to a Kaupa. You provide
//! funds which are held within Kaupa until someone buys them off you.
//!
//! - [accept_proposal] Accepts a trade proposal. You provide funds
//! that will then be held by Kaupa for the proposal owner, and you
//! receive back the funds offered by the proposal.
//!
//! - [rescind_proposal] Removes a trade proposal you previously
//! added, recovering the funds you put in.
//!
//! - [sweep_proposals] For trading pairs, places a market order. This
//! is like an autofire version of [accept_proposal].
//!
//! - [repay_flash_loan] Repays a flash loan you took.
//!
//! - [collect_funds] Claim your payment for proposals you created, or
//! claim your fees if you are the Kaupa owner.
//! 
//!
//! # Makers and Takers
//!
//! We distinguish between makers and takers: Makers are traders who
//! call [make_proposal] to add opportunities to the market. Takers
//! are traders who call [accept_proposal] or [sweep_proposals] to
//! take advantage of those opportunities.
//!
//! # Trading Pairs and Order Books
//!
//! If you set up a Kaupa instance that has only one single token to
//! each side (e.g. BTC/XRD) then you can specify that this is to be
//! handled as a trading pair. Kaupa will then automatically add all
//! incoming trade proposals into order books for you and traders can
//! call the [sweep_proposals] function to place market orders that
//! dip into essentially any number of proposals trying to fulfill the
//! order.
//!
//! In this mode, creating a proposal with [make_proposal] is
//! equivalent to placing a limit order (filling the order book) and
//! calling [sweep_proposals] is equivalent to placing a market order
//! (emptying out the order book).
//!
//! Kaupa allows takers to specify a price limit when calling
//! [sweep_proposals] such that they can use this facility without
//! being burned should someone beat them to the punch.
//!
//! # Flash Loans
//!
//! An alternative to offering a bag of tokens *for sale* for another
//! bag of tokens is to instead offer it for *loan*. Kaupa supports
//! this in the form of flash loans. The maker then puts a bunch of
//! tokens into the flash loan proposal, specifies how much they
//! demand for every flash loan that is made, and then this proposal
//! remains there indefinitely (however many times it gets flash
//! loaned) until the maker decides to rescind it.
//!
//! One of the tests runs a realistic scenario in which someone
//! chooses flash loans as a way of generating steady revenue through
//! Kaupa: take a look at `test_goldland_scenario` and its
//! documentation in the `tests` directory. Towards the end of that
//! function there is a comprehensive example of building a
//! transaction manifest that takes out a flash loan, uses it to
//! generate revenue, asserts that it was able to get the profit it
//! expected from it, and of course pays back the loan.
//!
//! # Fees
//!
//! Kaupa offers a configurable set of fees that is established on
//! instantiation. This allows the Kaupa creator great flexibility in
//! how to profit from their marketplace.
//!
//! - **Per transaction maker fixed fee** is charged once per
//! call to [make_proposal]. It is a list of tokens charged when
//! calling that function, each can be a fungible or a non-fungible.
//!
//! - **Per transaction taker fixed fee** is charged once per
//! call to [accept_proposal] and [sweep_proposals]. It is a list of
//! tokens charged when calling one of those functions, each can be a
//! fungible or a non-fungible.
//!
//! - **Per payment bps fee** is a proportional fee charged on
//! any fungible payment provided by a taker when calling
//! [accept_proposal] or [sweep_proposals]. It is measured in basis
//! points (bps), with one basis point being a hundredth of a
//! percent. For example, if this is set to 10 bps and Bob pays 2000
//! XRD into a proposal then he must additionally provide 2000 *
//! (10/10000) = 2 XRD in fees.
//!
//! - **Per NFT flat fee** is a fungible fee charged per NFT
//! that changes hands in a call to [accept_proposal] or
//! [sweep_proposals]. A fee can be specified per NFT resource so for
//! example there could be a 10 XRD fee per RaDragon NFT, a 50 VKC fee
//! per RaEgg NFT and no fee on Undying Vikings NFTs.
//!
//! All the fees are cumulative, and it is left as an exercise for the
//! Kaupa instance creator to devise a fee system that feels intuitive
//! to its users.
//!
//! # Specifying Costs (and most fees)
//!
//! Kaupa uses the `AskingType` enum to enable a flexible way of
//! specifying expected payments. It encapsulates support for both
//! fungible and non-fungible payment expectations and is always
//! paired with a resource address to form a full resource/amount
//! pair. Usually this is in the form of a `HashMap<ResourceAddress,
//! AskingType>`. In the following examples we use token names instead
//! of token resource addresses.
//!
//! - `(XRD, AskingType::Fungible(dec!("25.5")))` means that a 25.5 XRD
//! payment is expected.
//!
//! - `(RaDragons, AskingType::NonFungible(Some([100,200,500]),
//! Some(5)))` means that RaDragons #100, 200 and 500 as well as 5
//! other RaDragon NFTs of any id are expected in payment (so 8 NFTs
//! are expected in total).
//!
//! - For non-fungibles, either of the two parameters can be `None`.
//!
//! This allows for very flexible specification of the payment that
//! you expect. You could e.g. provide 5000 XRD and specify that you
//! want "any 20 RaDragons", or you could say "these five specific
//! RaDragons and any 10 others" or you could say "these 10 specific
//! RaDragons and none others" etc.
//!
//! (If you happen to specify an `AskingType::Fungible` together with
//! a resource address that is non-fungible, or an
//! `AskingType::NonFungible` together with a resource address that is
//! fungible, expect panics in the code.)
//!
//! # On Performance
//!
//! There are two main performance aspects to consider with this sort
//! of component. One is the maximum transactions per second it is
//! able to offer to the trading public, and the other is how
//! efficient it is internally in carrying out its functions.
//!
//! ## TPS and Parallelizing Kaupa
//!
//! Imagine that your marketplace is proving super successful, you are
//! starting to push the 50 TPS limit, and you are in the future and
//! on Xi'an. In this case your best bet to improve performance is to
//! instantiate multiple identical Kaupa instances and create the
//! illusion of a single marketplace on your front-end by having it
//! collate information from all of those instances. Each instance
//! will then potentially run in a different validator group and you
//! can access the full linear scalability of Xi'an.
//!
//! If you're taking advantage of the built-in order book support in
//! Kaupa then the above isn't going to work for you because the order
//! book support only exists within a single Kaupa instance. In this
//! case you may want to transition away from the built-in trading
//! pair support and implement this in your front-end code instead,
//! running towards multiple Kaupa instances.
//!
//! ## Execution Complexity
//!
//! Separate from the TPS question, there is a limit to how much Kaupa
//! is able to do within *a single* transaction. This is determined by
//! the network's complexity cutoff limit and by how efficient Kaupa's
//! code is in doing its job. In Scrypto 0.8 there is a 100 000 000
//! Cost Unit limit and some of the test suite transactions can
//! consume up to 40-50% of this. This means that if you try to do
//! something that is, perhaps, 2-3x as complex as our heavier tests
//! then your transaction will simply fail because it exceeded the
//! network's complexity threshold.
//!
//! (The final Babylon release will likely have a different complexity
//! threshold, and Cost Units calculated differently, from what we see
//! in 0.8 but there will still be *some* limit there which we are
//! going to need to relate to.)
//!
//! The complexity limit will likely be the most severe in calls to
//! [sweep_proposals] that end up taking from a large number of
//! Proposals, potentially even deleting a number of them in the
//! process. Further, sweeping on non-fungible trading pairs is likely
//! more expensive than sweeping fungible ones.
//!
//! Kaupa v1.0 is prototype level code and there is considerable room
//! for optimization here. Some of the more obvious opportunities for
//! such are noted in the current code, look for TODO comments.
//!
//! With the extensive test suite that comes with Kaupa, code
//! refactoring and optimization is fairly risk free: if a bug is
//! introduced in doing so the tests will flag this.
//!
//! ## Binary Size
//!
//! The size of the Kaupa wasm is past 900kB which indicates that
//! we're approaching the limits of the amount of logic you can have
//! in a component, as binary upload size is capped by the network at
//! 1MB.
//!
//! # The Test Suite
//!
//! Kaupa comes with a comprehensive test suite that runs through its
//! main use cases.
//!
//! Run the test suite with `scrypto test` on the command line.
//!
//! ## Disabled Tests
//!
//! Two of the tests are currently disabled by use of the ignore
//! directive. Both are flash loan tests, one is disabled because
//! flash loans don't seem to work correctly in the current Scrypto
//! version and the other because it has a very long run time (and
//! also again because flash loans).
//!
//! ## Test Framework
//!
//! Those who have followed my contributions to previous Scrypto
//! challenges may note that with this I have moved on from my own
//! homebrew test framework to using the official one provided with
//! Scrypto. I am sure we will all miss the regular expressions.
//!
//! If you were to read the test suite source file from top to bottom
//! you could probably recognize that I experience a slowly increasing
//! level of understanding of how to best use the framework.
//!
//! In particular I have some very use case specific helper functions
//! such as `make_trading_pair_proposal_f2f` and `accept_proposal_f`
//! which are hard limited in the number of resources they can build
//! buckets or proofs for; and then eventually I made helper functions
//! like `make_generic_proposal` and `accept_otc_proposal` that are
//! entirely flexible in this regard.
//!
//! While I could at this point replace the use of the old helper
//! functions with the new more generic ones I have not done this for
//! two reasons:
//!
//! - Refactoring all the tests in this way is a fair bit of work.
//!
//! - It's nice to be able to demonstrate the two different
//! approaches anyway.
//!
//! The main epiphany I had in this regard was that there are two ways
//! to build transaction manifests with the `ManifestBuilder`. One of
//! them, the one I started using intitially, is nice in that it
//! provides nesting of your proofs and buckets such that it helps you
//! organize what's going when things get a bit complex. You can see
//! this nesting in e.g. `make_trading_pair_proposal_nf2nf`.
//!
//! The other approach, using `add_instruction`, gives a more flat
//! structure which I find very handy for programmatically adding
//! arbitrary numbers of proofs and buckets to a manifest. You can see
//! this in e.g.  `accept_otc_proposal` where the whole manifest is
//! harder to spot in between of all the logic but where that logic
//! allows for the helper function to offer much wider functionality.
//!
//! My take home from this is that when you're building a manifest
//! where you know *exactly* what's going into it you're probably best
//! off using the nested approach; but if you're building one that may
//! take several different forms depending on the input then you
//! probably want to make liberal use of the `add_instruction` method.
//!
//! # Other Features Of Note
//!
//! ## Derivatives Trade (kinda, NFA)
//!
//! Note that since you identify yourself with any NFT you want in
//! Kaupa, an advanced user could set up a number of trades governed
//! by some NFT created for the purpose and then offer *this NFT* for
//! trade. They are then effectively selling ownership of the trades
//! that have already been established.
//!
//! ## PreciseDecimal
//!
//! Kaupa uses the `PreciseDecimal` type to avoid unnecessary rounding
//! when people do partial trades against our offers. Worthy of note
//! in this regard is the rounding behaviour used, usually rounding to
//! 18 digits for maximum precision when used with fungibles but in
//! one case rounding to 0 digits when dealing with non-fungibles.
//! Also note that `truncate` function on PreciseDecimal is never used
//! without first explicitly rounding it to our desired precision.
//!
//! # Development environment
//!
//! This project has been developed on Ubuntu Linux and while the
//! author expects everything in here to work on other platforms, if
//! you're having weird problems with it maybe this is the reason.
//!
//! This project has been developed for Scrypto v0.8.0.
//!
//! # License etc.
//!
//! This software is intended for entering into the **Radix DeFi
//! Challenge,** and the author cedes such rights as is necessary to
//! do so, ref. the challenge's official rules which are at time of
//! writing available
//! [here.](https://https://scryptodefi.devpost.com/)
//!
//! The author can be reached at `scryptonight@proton.me`
//!
//! [instantiate_kaupa]: crate::kaupa::Kaupa::instantiate_kaupa
//! [make_proposal]: crate::kaupa::Kaupa::make_proposal
//! [accept_proposal]: crate::kaupa::Kaupa::accept_proposal
//! [rescind_proposal]: crate::kaupa::Kaupa::rescind_proposal
//! [sweep_proposals]: crate::kaupa::Kaupa::sweep_proposals
//! [repay_flash_loan]: crate::kaupa::Kaupa::repay_flash_loan
//! [collect_funds]: crate::kaupa::Kaupa::collect_funds

use scrypto::prelude::*;

/// This alias is just to make the code slightly more
/// self-documenting.
///
/// It is public so we can use it from the test suite
pub type Uuid = u128;

/// We use this for our order book BTreeSets that are sorted by
/// cheapest trade first. Note that we rely here on Rust's default
/// sorting behaviour which depends on the order that the fields are
/// defined in the struct. It is therefore imperative here that the
/// `price_per` field is defined before the `uuid` field because that
/// gives it sort order precedence.
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, PartialEq, Eq, PartialOrd, Ord)]
struct PriceAndUuid {
    price_per: Decimal,
    uuid: Uuid,
}

/// This describes a flash loan debt owed to one of our proposals. It
/// is used as non-fungible data on the transient NFT that forces the
/// borrower to repay the debt.
#[derive(NonFungibleData)]
struct FlashLoanDebt {
    pub proposal_uuid: Uuid,
    pub fungibles_owed: HashMap<ResourceAddress, Decimal>,
    pub non_fungibles_owed: HashMap<ResourceAddress, Vec<NonFungibleLocalId>>,
}


/// This is a general way of describing how many tokens are wanted as
/// payment for some trade. Its purpose is to enable us to specify
/// *either* "n amount of fungibles" *or* "these here non-fungibles"
/// depending on which type of resource it is paired with.
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone, PartialEq, Eq, Debug)]
pub enum AskingType {
    /// Asks for this exact amount of a fungible token.
    Fungible(Decimal),

    /// Asks for non-fungible tokens. We can ask for a specific set of
    /// non-fungible local ids and/or we can ask for a number of
    /// arbitrarily chosen NFTs. If both parameters are in use then we
    /// are asking for the sum of those two so for example,
    /// ```NonFungible(Some(HashSet(1,2,3)), Some(5))``` asks for
    /// nonfungible local ids 1, 2 and 3 PLUS also 5 other random NFTs
    /// of the same NFT resource. (And by random we mean arbitrary.)
    NonFungible(Option<HashSet<NonFungibleLocalId>>, Option<u64>),
}

/// Enumerates the types of trade proposal that we support.
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone, PartialEq, Eq, Debug)]
pub enum ProposalType {
    /// Offer to sell a bag of tokens in exchange for another bag of
    /// tokens
    Barter,
    /// Offer to make a flash loan of a bag of tokens in exchange for
    /// another bag of tokens
    FlashLoan,
}

/// This is the bread and butter of the Kaupa component: the
/// specification of a trade proposal.
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe)]
struct TradeProposal {
    /// We automatically assign a random uuid to new proposals, and
    /// this is used to refer back to them later.
    uuid: Uuid,

    /// Whoever controls the NFT specified here owns the trade
    /// proposal.
    owner: NonFungibleGlobalId,

    /// If this is set, then if you want to interact with this trade
    /// proposal you must be in control of the NFT specified.
    counterparty: Option<NonFungibleGlobalId>,

    /// What kind of trade we're proposing here.
    ptype: ProposalType,

    /// The bag of tokens this proposal offers to someone who comes
    /// along to accept it.
    offering: HashMap<ResourceAddress, Vault>,

    /// The bag of tokens this proposal requests in return for its
    /// offerings.
    asking: HashMap<ResourceAddress, AskingType>,

    /// If `true`, this proposal allows people to accept less than the
    /// full proposal. In this case they will pay proportionally less
    /// and receive proportionally less.
    allow_partial: bool,
}

/// This is the main fee structure that defines the fee levels of each
/// individual Kaupa instance.
// Public so we can use it from the test suite
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone, PartialEq, Eq, Debug)]
pub struct Fees {
    /// This is charged once for every maker interaction.
    pub per_tx_maker_fixed_fee: Option<HashMap<ResourceAddress, AskingType>>,

    /// This is charged once for every taker interaction.
    pub per_tx_taker_fixed_fee: Option<HashMap<ResourceAddress, AskingType>>,

    /// This is charged once for each fungible payment that is made to
    /// accept a proposal.
    pub per_payment_bps_fee: Option<Decimal>,
    
    /// The key is an NFT resource that is being traded. The value is
    /// the flat fee to charge per NFT of that resource that is traded.
    pub per_nft_flat_fee: Option<HashMap<ResourceAddress, (ResourceAddress, Decimal)>>,
}

/// Reduces a PreciseDecimal down to Decimal precision by rounding to
/// the nearest 18 digits and truncating. This is useful when you need
/// to use a PreciseDecimal to move funds around.
fn pdec_to_dec(pdec: PreciseDecimal) -> Decimal {
    pdec.round(18, RoundingMode::TowardsNearestAndHalfAwayFromZero).truncate()
}

#[blueprint]
mod kaupa {
    struct Kaupa {
        /// Our owner can collect fees from us
        owner: NonFungibleGlobalId,

        /// A display name to use for this Kaupa
        name: Option<String>,

        /// A catchy blurb to attract trade to us
        blurb: Option<String>,

        /// The URL of our front-end
        url: Option<String>,

        /// The fees this Kaupa charges on use
        fees: Option<Fees>,

        /// If set, limits which resources can be used on the one side
        /// of trade proposals in this Kaupa
        side1_token: Option<HashSet<ResourceAddress>>,

        /// If set, limits which resources can be used on the other
        /// side of trade proposals in this Kaupa
        side2_token: Option<HashSet<ResourceAddress>>,

        /// All our currently active (waiting) proposals.
        proposals: HashMap<Uuid, TradeProposal>,

        /// Payments to owners of successful proposals. The outer
        /// map's keys are the NFTs of the proposal owners (i.e. the
        /// people we now owe these funds). The inner map holds the
        /// vaults with all the funds we owe them.
        payouts: HashMap<NonFungibleGlobalId, HashMap<ResourceAddress, Vault>>,

        /// Fees we have collected for our owner.
        collected_fees: HashMap<ResourceAddress, Vault>,

        /// The badge (if any) to use to control our transient flash
        /// loan NFTs
        flash_loan_badge: Option<Vault>,

        /// The resource address (if any) of the transient flash loan
        /// NFTs we use.
        flash_loan_resource: Option<ResourceAddress>,

        /// Whether or not this Kaupa is a trading pair.
        is_trading_pair: bool,

        /// Whether or not this Kaupa forces its proposals to allow
        /// partial trade.
        force_allow_partial: bool,

        /// Whether or not this Kaupa allows for flash loan proposals.
        allow_flash_loans: bool,

        /// If this Kaupa is a trading pair, this holds the buy side
        /// order book. Market sell orders eat proposals from this
        /// book.
        buy_book: Option<BTreeSet<PriceAndUuid>>,

        /// If this Kaupa is a trading pair, this holds the sell side
        /// order book. Market buy orders eat proposals from this
        /// book.
        sell_book: Option<BTreeSet<PriceAndUuid>>,

        /// Since we cannot just destroy vaults when we're done with
        /// them, we put them here. These are vaults from old dead
        /// Proposals that have been emptied out and will never be
        /// used again.
        garbage_vaults: Vec<Vault>,
    }

    impl Kaupa {
        /// Creates a new Kaupa instance. Everyone is allowed to do
        /// this.
        ///
        /// While you can specify any `owner` you want, mostly you
        /// will want to use one of your own NFTs here.
        ///
        /// If you make obvious mistakes in the input arguments, Kaupa
        /// will try to help you out by panicking with a descriptive
        /// error message.
        ///
        /// We return a tuple with the following values.
        ///
        /// - .0 is the component address of your new Kaupa.
        ///
        /// - .1 is the address of our transient flash loan resource,
        /// if any. You will need this when you're writing transaction
        /// manifests that employ our flash loans.
        pub fn instantiate_kaupa(
            owner: NonFungibleGlobalId,
            name: Option<String>,
            blurb: Option<String>,
            url: Option<String>,
            fees: Option<Fees>,
            side1_token: Option<HashSet<ResourceAddress>>,
            side2_token: Option<HashSet<ResourceAddress>>,
            is_trading_pair: bool,
            force_allow_partial: bool,
            allow_flash_loans: bool) -> (ComponentAddress, Option<ResourceAddress>) {

            if is_trading_pair {
                assert!(side1_token.is_some(),
                        "trading pair: side1 must have a token");
                assert!(side1_token.as_ref().unwrap().len() == 1,
                        "trading pair: side1 too many tokens");

                assert!(side2_token.is_some(),
                        "trading pair: side2 must have a token");
                assert!(side2_token.as_ref().unwrap().len() == 1,
                        "trading pair: side2 too many tokens");

                assert!(side1_token.as_ref().unwrap().iter().next() !=
                        side2_token.as_ref().unwrap().iter().next(),
                        "trading pair: side1 and side2 must be different tokens");

                assert!(force_allow_partial,
                        "trading pair must force_allow_partial");

                assert!(!allow_flash_loans,
                        "trading pairs do not support flash loans");
            }

            let flash_loan_badge;
            let flash_loan_resource;
            if allow_flash_loans {
                let flash_loan_badge_bucket =
                    ResourceBuilder::new_fungible()
                    .divisibility(DIVISIBILITY_NONE)
                    .metadata("name", "Kaupa flash loan badge")
                    .mint_initial_supply(1);

                flash_loan_resource =
                    Some(ResourceBuilder::new_uuid_non_fungible()
                         .metadata(
                             "name",
                             "Transient tokens for Kaupa flash loans",
                         )
                         .mintable(rule!(require(flash_loan_badge_bucket.resource_address())),
                                   AccessRule::DenyAll)
                         .burnable(rule!(require(flash_loan_badge_bucket.resource_address())),
                                   AccessRule::DenyAll)

                         // NOTE: If you comment out the following
                         // line the flash loan tests will be able to
                         // run successfully. Be warned though that
                         // when you do that, repayment of the flash
                         // loan is not enforced by the ledger and so
                         // this cannot be used for actual business.
                         //
                         // See the test `test_flash_loans` for a note
                         // on the state of transient tokens and flash
                         // loans atm.
                         .restrict_deposit(AccessRule::DenyAll, AccessRule::DenyAll)
                         .create_with_no_initial_supply());
                flash_loan_badge = Some(Vault::with_bucket(flash_loan_badge_bucket));
            } else {
                flash_loan_badge = None;
                flash_loan_resource = None;
            }

            Self::check_fee_validity(&fees);
            
            // Note that all our functions can be called by
            // everyone. What little access checking we need we do in
            // code.
            let kaupa = 
                Self {
                    owner,
                    name,
                    blurb,
                    url,
                    fees,
                    side1_token,
                    side2_token,
                    proposals: HashMap::new(),
                    payouts: HashMap::new(),
                    collected_fees: HashMap::new(),
                    flash_loan_badge,
                    flash_loan_resource: flash_loan_resource.clone(),
                    is_trading_pair,
                    force_allow_partial,
                    allow_flash_loans,
                    buy_book: if is_trading_pair { Some(BTreeSet::new()) } else { None },
                    sell_book: if is_trading_pair { Some(BTreeSet::new()) } else { None },
                    garbage_vaults: Vec::new(),
                }
            .instantiate()
                .globalize();

            (kaupa, flash_loan_resource)
        }

        /// Creates a new trade proposal and adds it to this Kaupa.
        ///
        /// Your proposal will be to sell all the funds in the
        /// `offering` buckets in exchange for the payment specified
        /// by your `asking` map.
        ///
        /// You need to identify yourself with a `trader` proof (so
        /// that we can know you're the same person later when you
        /// come back to claim your payment). You can use any NFT you
        /// have for this but make sure not to lose it or you won't be
        /// able to collect payment.
        ///
        /// If you specify a `counterparty` then only someone who can
        /// provide a proof of the NFT specified will be able to
        /// accept the proposal. If you want everyone to be able to
        /// have a go, instead specify `None` here.
        ///
        /// If you pass `true` for `allow_partial` then you are saying
        /// that you're ok with someone buying just a smaller portion
        /// of what you're offering. Conversely, if you pass `false`
        /// then you are saying that this is an all-or-nothing kind of
        /// offer. Note that `true` is only supported for proposals
        /// that have one single resource on each side of the deal
        /// (e.g. BTC on one side and XRD on the other).
        ///
        /// If per transaction maker fees are set you will need to
        /// provide sufficient funds in `maker_fee_buckets` to cover
        /// these.
        ///
        /// The method returns the unique id of the proposal that was
        /// created (you can use this later if you want to rescind the
        /// proposal, or you may want to tell other people aobut your
        /// proposal) and also buckets containing any leftovers from
        /// the fee buckets you passed in.
        pub fn make_proposal(&mut self,
                             trader: Proof,
                             counterparty: Option<NonFungibleGlobalId>,
                             ptype: ProposalType,
                             offering: Vec<Bucket>,
                             asking: HashMap<ResourceAddress, AskingType>,
                             allow_partial: bool,
                             mut maker_fee_buckets: Vec<Bucket>)
                             -> (Uuid, Vec<Bucket>)
        {
            // We don't care which NFTs you use to identify yourself
            // so there is no need to validate the proof's
            // ResourceAddress when you're registering a proposal.
            let trader = trader.unsafe_skip_proof_validation();
            assert!(trader.non_fungible_local_ids().len() == 1,
                    "The Proof needs to have exactly one NFT in it");

            if matches!(ptype, ProposalType::Barter) {
                assert!(!self.force_allow_partial || allow_partial,
                        "this Kaupa only permits allow_partial");
                if allow_partial {
                    assert!(offering.len() == 1 && asking.len() == 1,
                            "allow_partial cannot have multiple resources to a side");
                }
                if self.is_trading_pair {
                    assert_eq!(1, asking.len(),
                               "trading pair must have exactly one asking resource");
                    assert_eq!(1, offering.len(),
                               "trading pair must have exactly one offering resource");
                }
            }

            // Sanity check the asking argument
            Self::check_asking_map_sanity(&asking);
            
            if self.fees.is_some() {
                maker_fee_buckets =
                    Self::charge_per_tx_fees(self.fees.as_ref().unwrap()
                                                   .per_tx_maker_fixed_fee.as_ref(),
                                                   &mut self.collected_fees,
                                                   maker_fee_buckets);
            }
            
            let vaults = Self::bucket_vec_to_vault_map(offering);

            assert!(self.check_token_reqs(&vaults, &asking),
                    "Token types mismatch");

            let uuid = Runtime::generate_uuid();
            self.add_proposal(TradeProposal {
                owner: NonFungibleGlobalId::new(trader.resource_address(),
                                                trader.non_fungible_local_id()),
                counterparty,
                uuid,
                ptype,
                offering: vaults,
                asking,
                allow_partial,
            });

            (uuid, maker_fee_buckets)
        }

        /// Removes a proposal from this Kaupa, returning its funds.
        /// Any maker fees that were charged are not returned.
        ///
        /// The caller must provide a proof that they are the owner of
        /// the proposal.
        ///
        /// Will panic if the proof doesn't check out, or if the
        /// proposal doesn't exist.
        pub fn rescind_proposal(&mut self,
                                trader: Proof,
                                proposal_uuid: Uuid) -> Vec<Bucket> {
            // We don't call validate_proof but instead check the
            // particulars manually.
            let owner = &self.proposals.get(&proposal_uuid).unwrap().owner;
            assert_eq!(owner.resource_address(), trader.resource_address(),
                       "wrong trader resource");
            assert!(trader.non_fungible_local_ids().contains(owner.local_id()),
                    "wrong trader nflid");

            self.remove_proposal(proposal_uuid)
        }

        /// Accepts a trade proposal. You must provide necessary
        /// funds, and will receive the offered bag of tokens back out
        /// of this function.
        ///
        /// If the proposal you name has been restricted then you must
        /// identify yourself as the owner of the relevant NFT with
        /// the `taker` argument. Otherwise you can pass `None` here.
        ///
        /// If you specify `allow_partial` you are permitting Kaupa to
        /// cover your order in part if necessary. Otherwise the trade
        /// will only happen if it can happen in full.
        ///
        /// Note that any fees you're paying *must* be in the
        /// `fee_paying_buckets` argument, fees will not be taken from
        /// `paying_buckets` which only go towards the cost of the
        /// proposal itself.
        ///
        /// Returned out of this function are the funds you purchased
        /// from the proposal, as well sa any excess funds you sent in
        /// that haven't been spent.
        pub fn accept_proposal(&mut self,
                               taker: Option<Proof>,
                               proposal_uuid: Uuid,
                               allow_partial: bool,
                               paying_buckets: Vec<Bucket>,
                               mut fee_paying_buckets: Vec<Bucket>) -> Vec<Bucket>
        {
            let proposal = self.proposals.get_mut(&proposal_uuid).expect("no such proposal");
            
            // Check if the taker is allowed to accept this proposal
            if proposal.counterparty.is_some() {
                let counterparty = proposal.counterparty.as_ref().unwrap();
                let taker = taker.unwrap();
                assert!(counterparty.resource_address() == taker.resource_address(),
                        "This proposal is not for you: wrong NFT resource");
                assert!(taker.non_fungible_local_ids().contains(counterparty.local_id()),
                        "This proposal is not for you: wrong NFT id");
            }

            if allow_partial && matches!(proposal.ptype, ProposalType::FlashLoan) {
                panic!("partial flash loans not supported");
            }

            if self.fees.is_some() {
                fee_paying_buckets =
                    Self::charge_per_tx_fees(self.fees.as_ref().unwrap()
                                                   .per_tx_taker_fixed_fee.as_ref(),
                                                   &mut self.collected_fees,
                                                   fee_paying_buckets);
            }

            // Calculate payment

            let partial = Self::calculate_partial_ratio(&proposal, &paying_buckets[0]);
            assert!(allow_partial || partial == PreciseDecimal::ONE,
                    "insufficient funding for the full proposal");
            assert!(!partial.is_zero(), "insufficient funding to make a trade");

            // Converts the input bucket vectors to maps for easier handling
            let mut paying_buckets = Self::bucket_vec_to_bucket_map(paying_buckets);
            let mut fee_paying_buckets = Self::bucket_vec_to_bucket_map(fee_paying_buckets);

            let mut paid_fees;
            let mut return_buckets;

            match proposal.ptype {
                ProposalType::Barter => {
                    let remove_proposal;
                    (return_buckets, paid_fees, remove_proposal) =
                        self.execute_barter(&mut paying_buckets,
                                           &mut fee_paying_buckets,
                                           proposal_uuid,
                                           partial);

                    if remove_proposal {
                        return_buckets.append(&mut self.remove_proposal(proposal_uuid));
                    }
                    
                    // Note that at this point return_buckets only contains
                    // tokens that were actually purchased.
                    self.charge_per_nft_fees(&return_buckets,
                                             &mut fee_paying_buckets,
                                             &mut paid_fees);
                },
                ProposalType::FlashLoan => {
                    (return_buckets, paid_fees) =
                        self.execute_flash_loan(&mut paying_buckets,
                                                &mut fee_paying_buckets,
                                                proposal_uuid);
                }
            }
            
            // Store fees in the Kaupa for its owner to collect later
            if paid_fees.len() > 0 {
                for (_, bucket) in paid_fees.into_iter() {
                    Self::put_bucket_in_vault_map(&mut self.collected_fees, bucket);
                }
            }

            // These are remaining empty payment/fee buckets, and
            // overfunded such buckets. We return them to the caller.
            paying_buckets.into_values().for_each(|b| return_buckets.push(b));
            fee_paying_buckets.into_values().for_each(|b| return_buckets.push(b));

            return_buckets
        }

        /// Repays a flash loan. You *must* call this method after
        /// taking a flash loan, and it must happen in the same
        /// transaction manifest. You must provide the `transient`
        /// token you received when taking out the loan and you must
        /// provide sufficient `funds` to repay the full loan.
        ///
        /// There are no fees for repaying a loan. Instead, consider
        /// charging fixed taker fees for this purpose.
        ///
        /// Note that the access control here is that you need to
        /// provide the transient badge for the flash loan for
        /// anything useful to happen.
        ///
        /// This method will panic if it doesn't like the transient
        /// token or if there is insufficient funds to repay the loan.
        pub fn repay_flash_loan(&mut self,
                                transient: Bucket,
                                funds: Vec<Bucket>) -> Vec<Bucket>
        {
            assert_eq!(self.flash_loan_resource.unwrap(),
                       transient.resource_address(),
                       "wrong transient resource");

            let debt: FlashLoanDebt = transient.non_fungible().data();

            let proposal = self.proposals.get_mut(&debt.proposal_uuid).expect(
                "no such proposal");

            let mut funds = Self::bucket_vec_to_bucket_map(funds);

            // Claw back the proposal's fungibles
            for (resaddr, amount) in debt.fungibles_owed {
                Self::put_bucket_in_vault_map(
                    &mut proposal.offering,
                    funds.get_mut(&resaddr).unwrap().take(amount));
            }

            // Claw back the proposal's non-fungibles
            for (resaddr, nflids) in debt.non_fungibles_owed {
                Self::put_bucket_in_vault_map(
                    &mut proposal.offering,
                    funds.get_mut(&resaddr).unwrap().take_non_fungibles(
                        &nflids.into_iter().collect()));
            }

            // If we got here without panic, the loan is repaid
            self.flash_loan_badge.as_ref().unwrap().authorize(
                || transient.burn());

            // Return any excess funds back to the caller
            funds.into_values().collect()
        }
                                
        
        /// Buys from or sells into our order book as far as provided
        /// funds reach. This can result in multiple proposals being
        /// fulfilled and removed.
        ///
        /// Which side of the trade you are on is automatically
        /// determined by the contents of the bucket you provide. For
        /// example in a BTC/XRD trading pair if you send a bucket
        /// with BTC then you're selling and if you send a bucket with
        /// XRD you are buying.
        ///
        /// Note that fee-paying funds provided must be enough to
        /// cover all the purchases you can afford so you need to make
        /// a prediction before calling this function as to how much
        /// will be purchased so you can provide those fees. If you
        /// provide insufficient fees this function will panic.
        ///
        /// The price limit is a maximum price to permit for market
        /// buy orders, and the inverse of the minimum price to permit
        /// for market sell orders.
        ///
        /// For example, for a BTC/XRD trading pair, if 1 BTC is worth
        /// ~5000 XRD then a market **buy** order that wants to buy at
        /// max price 5005 XRD will pass 5005 here. Whereas a market
        /// **sell** order that wants to sell BTC at a minimum price
        /// of of 4995 XRD will pass 1/4995 = 0.0002002002... here.
        ///
        /// The function will keep sweeping until you run out of
        /// funds, you reach your price limit, or you run out of order
        /// book to sweep. (Or until you reach the network complexity
        /// limit and it panics.)
        ///
        /// This function is only supported for trading pair type
        /// Kaupa instances.
        ///
        /// This operation is analogous to doing a market trade on a
        /// traditional CEX trading pair.
        pub fn sweep_proposals(&mut self,
                               taker: Option<Proof>,
                               price_limit: Option<Decimal>,
                               paying_bucket: Bucket,
                               mut fee_paying_buckets: Vec<Bucket>) -> Vec<Bucket>
        {
            assert!(self.is_trading_pair,
                    "sweep only allowed on trading pairs");

            let order_book = if paying_bucket.resource_address()
                == *self.side1_token.as_ref().unwrap().iter().next().unwrap()
            {
                self.buy_book.as_ref().unwrap()
            } else {
                self.sell_book.as_ref().unwrap()
            };

            if self.fees.is_some() {
                fee_paying_buckets =
                    Self::charge_per_tx_fees(self.fees.as_ref().unwrap()
                                             .per_tx_taker_fixed_fee.as_ref(),
                                             &mut self.collected_fees,
                                             fee_paying_buckets);
            }

            // First convert the input bucket vectors to maps for easier handling later
            let mut paying_buckets = Self::bucket_vec_to_bucket_map([paying_bucket].into());
            let mut fee_paying_buckets = Self::bucket_vec_to_bucket_map(fee_paying_buckets);
            let mut remove_proposals = Vec::new();
            let mut return_buckets = Vec::new();
            let mut paid_fees = HashMap::new();
            let mut uuids_to_execute = Vec::new();
            for entry in order_book {
                if price_limit.is_some() && entry.price_per > price_limit.unwrap() { break; }

                let proposal = self.proposals.get_mut(&entry.uuid).expect(
                    "error: proposal from order book not found in proposals");

                if proposal.counterparty.is_some() {
                    if taker.is_none() { continue; } // don't panic
                    let taker = taker.as_ref().unwrap();
                    let counterparty = proposal.counterparty.as_ref().unwrap();
                    
                    if counterparty.resource_address() != taker.resource_address()
                        || !taker.non_fungible_local_ids().contains(counterparty.local_id())
                    {
                        // this proposal is for someone else
                        continue;
                    }
                }

                uuids_to_execute.push((entry.uuid, entry.price_per));
            }

            // TODO The above loop made a much longer list of UUIDs to
            // sweep than is strictly necessary since it has
            // insufficient information to cut it off once funds run
            // out. The below loop has that information, and it would
            // likely be advantageous to combine the two loops to cut
            // down on execution overhead.
            
            for (uuid, price_per) in uuids_to_execute {
                let proposal = self.proposals.get_mut(&uuid).expect(
                    "error: proposal from order book not found in proposals");

                if !Self::is_resource_fungible(proposal.offering.keys().next().expect(
                    "error: proposal offers nothing"))
                {
                    // if we can no longer afford the cheapest
                    // available non-fungible, we're done here
                    if paying_buckets.values().next().unwrap().amount() < price_per { break; }
                }

                let partial_ratio =
                    Self::calculate_partial_ratio(proposal,
                                                  paying_buckets.values().next().unwrap());

                if partial_ratio == PreciseDecimal::ZERO { continue; }

                let (mut received_buckets, proposal_fees, remove_proposal) =
                    self.execute_barter(&mut paying_buckets,
                                       &mut fee_paying_buckets,
                                       uuid,
                                       partial_ratio);

                for fee in proposal_fees.into_values() {
                    Self::put_bucket_in_bucket_map(&mut paid_fees, fee);
                }

                return_buckets.append(&mut received_buckets);
                
                if remove_proposal {
                    remove_proposals.push(uuid);
                }

                // are we there yet?
                if paying_buckets.values().next().unwrap().is_empty() { break; }
            }

            for uuid in remove_proposals {
                return_buckets.append(&mut self.remove_proposal(uuid));
            }

            // Note that at this point return_buckets only contains
            // tokens that were actually purchased.
            self.charge_per_nft_fees(&return_buckets,
                                     &mut fee_paying_buckets,
                                     &mut paid_fees);

            if paid_fees.len() > 0 {
                for (_, bucket) in paid_fees.into_iter() {
                    Self::put_bucket_in_vault_map(&mut self.collected_fees, bucket);
                }
            }

            // These are remaining empty payment/fee buckets, and
            // overfunded such buckets
            paying_buckets.into_values().for_each(|b| return_buckets.push(b));
            fee_paying_buckets.into_values().for_each(|b| return_buckets.push(b));

            return_buckets
        }

        /// Collect any funds owed to you by the Kaupa instance,
        /// either because your trade proposal got accepted or because
        /// you are the Kaupa owner and want to collect fees.
        ///
        /// You can do either of those, or both, in one call.
        ///
        /// You must provide a proof of your ownership or we shall
        /// panic.
        ///
        /// If you only want to collect a single specific type of
        /// token at this time, specify it in the `collect_token`
        /// argument. Your other tokens will still be waiting for you
        /// after.
        pub fn collect_funds(&mut self,
                             trader: Proof,
                             collect_payments: bool,
                             collect_fees: bool,
                             collect_token: Option<ResourceAddress>) -> Vec<Bucket>
        {
            // We don't care what type of NFT you're using - we will
            // later only allow you the actions permitted by the one
            // you passed us.
            let trader = trader.unsafe_skip_proof_validation();

            let mut funds = Vec::new();

            if collect_payments {
                for nflid in trader.non_fungible_local_ids() {
                    let nfgid = NonFungibleGlobalId::new(trader.resource_address(),
                                                         nflid);
                    let banked = self.payouts.get_mut(&nfgid);
                    if banked.is_some() {
                        for (k, v) in banked.unwrap().iter_mut() {
                            if collect_token.is_none() || k == collect_token.as_ref().unwrap() {
                                funds.push(v.take_all());
                            }
                        }
                    }
                }
            }

            if collect_fees {
                assert_eq!(trader.resource_address(), self.owner.resource_address(),
                           "missing component owner badge");
                assert!(trader.non_fungible_local_ids().contains(self.owner.local_id()),
                        "missing component owner badge");
                
                for (k, v) in self.collected_fees.iter_mut() {
                    if collect_token.is_none() || k == collect_token.as_ref().unwrap() {
                        funds.push(v.take_all());
                    }
                }
            }
            
            funds
        }

        /// Implements the flash loan specific portions of a flash
        /// loan type proposal.
        ///
        /// We return a tuple consisting of the following.
        ///
        /// - .0 is a bucket of tokens to return to the invoker of
        /// `accept_proposal`.
        ///
        /// - .1 are all the fees that got paid, these should
        /// eventually be deposited in our Kaupa's `collected_fees`
        /// field.
        fn execute_flash_loan(&mut self,
                             paying_buckets: &mut HashMap<ResourceAddress, Bucket>,
                             fee_paying_buckets: &mut HashMap<ResourceAddress, Bucket>,
                             uuid: Uuid)
                             -> (Vec<Bucket>, HashMap<ResourceAddress, Bucket>)
        {
            let (_, paid_fees, partial) = 
                self.take_payment(paying_buckets, fee_paying_buckets, uuid, PreciseDecimal::ONE);

            assert_eq!(PreciseDecimal::ONE, partial,
                       "insufficient payment for a flash loan");
            
            let proposal = self.proposals.get_mut(&uuid).unwrap();
            let mut return_buckets: Vec<Bucket> =
                proposal.offering.values_mut().map(|v| v.take_all()).collect();

            // Remember how much we're owed
            let mut fungibles_owed = HashMap::new();
            let mut non_fungibles_owed = HashMap::new();
            for lending in &return_buckets {
                let resaddr = &lending.resource_address();
                if Self::is_resource_fungible(resaddr) {
                    *fungibles_owed.entry(resaddr.clone()).or_insert(Decimal::ZERO)
                        += lending.amount();
                } else {
                    let mut nflids : Vec<NonFungibleLocalId> = 
                        lending.non_fungible_local_ids().clone().into_iter().collect();
                    non_fungibles_owed.entry(resaddr.clone()).or_insert(
                        Vec::new()).append(&mut nflids);
                }
            }
            
            // This transient token forces the borrower to return all
            // the funds by the end of the transaction manifest.
            let transient = self.flash_loan_badge.as_ref().unwrap().authorize(|| {
                borrow_resource_manager!(self.flash_loan_resource.unwrap()).mint_uuid_non_fungible(
                    FlashLoanDebt {
                        proposal_uuid: uuid,
                        fungibles_owed,
                        non_fungibles_owed,
                    },
                )
            });

            return_buckets.push(transient);

            (return_buckets, paid_fees)
        }

        /// Implements the barter-specific portion of a barter-type
        /// proposal.
        ///
        /// We return a tuple consisting of the following.
        ///
        /// - .0 is a bucket of tokens to return to the invoker of
        /// `accept_proposal`.
        ///
        /// - .1 are all the fees that got paid, these should
        /// eventually be deposited in our Kaupa's `collected_fees`
        /// field.
        ///
        /// - .2 is `true` if the calling method should now delete
        /// this proposal because it has been exhausted. Note that in
        /// this case, we *have not yet* removed its `offering` funds
        /// since that happens instead in the call to
        /// `remove_proposal` later. Those funds should then be added
        /// to the return bucket tokens (in .0) and passed back with
        /// them.
        fn execute_barter(&mut self,
                         paying_buckets: &mut HashMap<ResourceAddress, Bucket>,
                         fee_paying_buckets: &mut HashMap<ResourceAddress, Bucket>,
                         uuid: Uuid,
                         partial: PreciseDecimal)
                         -> (Vec<Bucket>, HashMap<ResourceAddress, Bucket>, bool)
        {
            let (taken_nflids, paid_fees, partial) = 
                self.take_payment(paying_buckets, fee_paying_buckets, uuid, partial);

            let mut remove_proposal = false;
            // Grab the tokens that the caller wanted to buy
            let mut return_buckets = Vec::new();
            if partial == PreciseDecimal::ONE {
                // This if a full buyout of the proposal, so just
                // remove it and return all its vault contents
                remove_proposal = true;
            } else {
                // This is a partial buyout, so pilfer its vaults only
                // a little. (Stricly speaking we know it's only a
                // single vault because we only support partial buyout
                // on single-resource trades.)

                // We need to fetch the proposal again here, because
                // the author doesn't know Rust very well.
                let proposal = self.proposals.get_mut(&uuid).unwrap();
                for (resaddr, vault) in proposal.offering.iter_mut() {
                    if Self::is_resource_fungible(resaddr) {
                        let amount = partial * vault.amount();
                        return_buckets.push(vault.take(pdec_to_dec(amount)));
                    } else {
                        // We already know this rounding is ok for NFTs
                        let amount = (partial * vault.amount())
                            .round(0, RoundingMode::TowardsNearestAndHalfAwayFromZero);
                        return_buckets.push(vault.take(pdec_to_dec(amount)));
                    }
                }

                // Now reduce the asking price appropriately for the
                // remaining tokens
                for (_, asking) in &mut proposal.asking {
                    match asking {
                        AskingType::Fungible(price) => {
                            let precise_price = PreciseDecimal::from(*price);
                            let precise_partial = PreciseDecimal::from(partial);
                            let new_price = precise_price - precise_price * precise_partial;
                            *asking = AskingType::Fungible(pdec_to_dec(new_price));
                        },
                        AskingType::NonFungible(nflids, amount) => {
                            // We remove as many named NFT asks as we
                            // can from the asking price, and put the
                            // rest into reducing the randos.
                            //
                            // Note that our use of local ids (rather
                            // than global ids) in taken_nflids only
                            // works because there is only ever a
                            // single NFT resource involved in payment
                            // for a partial trade.
                            
                            let mut nflids_orig_ask_amount = 0;
                            let mut nflids_new_ask_amount = 0;
                            if nflids.is_some() {
                                let nflids = nflids.as_mut().unwrap();
                                nflids_orig_ask_amount = nflids.len();
                                nflids.retain(|nflid| !taken_nflids.contains(nflid));
                                nflids_new_ask_amount = nflids.len();
                            }
                            if amount.is_some() {
                                let total_nfts_taken = taken_nflids.len();
                                let orig_ask_total = Decimal::from(
                                    amount.unwrap() + nflids_orig_ask_amount as u64);
                                let remaining_to_ask =
                                    (orig_ask_total - total_nfts_taken - nflids_new_ask_amount)
                                    .round(0, RoundingMode::TowardsNearestAndHalfAwayFromZero);
                                *(amount.as_mut().unwrap()) = Self::dec_to_u64(remaining_to_ask);
                            }
                            *asking = AskingType::NonFungible(nflids.clone(), *amount);
                        }
                    }
                }
            }
            
            (return_buckets, paid_fees, remove_proposal)
        }

        /// Takes payment and fees as specified by the proposal
        /// identified by `uuid`, out of `paying_buckets` and
        /// `fee_paying_buckets`, respectively.
        ///
        /// We return a tuple with three values:
        ///
        /// - .0 contains the non-fungible local ids that we took as
        /// payment.
        ///
        /// - .1 contains the buckets we placed fee payments
        /// into. This should eventually go to the Kaupa owner.
        ///
        /// - .2 contains a new `partial` ratio to use.
        ///
        /// If we couldn't honour the original `partial` ratio we
        /// return a new `partial` which reflects how much we were
        /// able to take. The caller should use this to calculate the
        /// payout to the buyer.
        ///
        /// (Note that the `taken_nflids` vector is currently only
        /// used in trading pairs and this implies that there is only
        /// going to be a single NFT resource involved so we can use
        /// local ids to identify the NFTs. If we were to expand the
        /// "allow partial" functionality so that it could affect
        /// multiple resources at the same time we should need to
        /// change this to global ids.)
        fn take_payment(&mut self,
                        paying_buckets: &mut HashMap<ResourceAddress, Bucket>,
                        fee_paying_buckets: &mut HashMap<ResourceAddress, Bucket>,
                        uuid: Uuid,
                        mut partial: PreciseDecimal)
                        -> (BTreeSet<NonFungibleLocalId>,
                            HashMap<ResourceAddress, Bucket>,
                            PreciseDecimal)
        {
            let mut maker_buckets = Vec::new();
            let proposal = self.proposals.get_mut(&uuid).unwrap();

            // Move funds from the taker's payment and fees to the
            // maker and fee repositories
            let mut paid_fees: HashMap<ResourceAddress, Bucket> = HashMap::new();
            let mut taken_nflids = BTreeSet::new();
            for (resaddr, cost) in &proposal.asking {
                let paying_bucket = paying_buckets.get_mut(resaddr).expect("insufficient payment");
                match cost {
                    AskingType::Fungible(price) => {
                        // Fees for the Kaupa instance owner
                        Self::charge_taker_fees_fungible(
                            &mut paid_fees,
                            fee_paying_buckets.get_mut(resaddr),
                            &price,
                            &partial,
                            &self.fees);
                        // Payment to the proposal's maker
                        maker_buckets.push(paying_bucket.take(pdec_to_dec(
                            partial * *price)));
                    },
                    AskingType::NonFungible(nflids, random_q) => {
                        let max_take;
                        (taken_nflids, max_take) = 
                            Self::pick_nfts(&Some(paying_bucket),
                                            &partial,
                                            &nflids,
                                            random_q.unwrap_or_default());

                        if Decimal::from(taken_nflids.len()) < max_take {
                            partial = partial * taken_nflids.len() / max_take;
                        }
                        
                        if self.is_trading_pair &&
                            !Self::is_resource_fungible(&paying_bucket.resource_address()) &&
                            !Self::is_resource_fungible(
                                &proposal.offering.values().next().unwrap().resource_address())
                        {
                            // In a non-fungible to non-fungible
                            // trading pair, if we were not able
                            // to pick a whole number of NFTs for
                            // a whole number of NFTs, cancel this
                            // pick
                            let side1_tokens =
                                (partial *
                                 proposal.offering.values().next().unwrap().amount())
                                .round(18, RoundingMode::TowardsNearestAndHalfAwayFromZero);
                            if side1_tokens != side1_tokens.floor() {
                                partial = PreciseDecimal::ZERO;
                                taken_nflids.clear();
                            }
                        }
                        
                        Self::charge_taker_fees_nonfungible(
                            &mut paid_fees,
                            fee_paying_buckets,
                            &resaddr,
                            taken_nflids.len() as u64,
                            &self.fees);


                        maker_buckets.push(
                            paying_bucket.take_non_fungibles(&taken_nflids));
                    },
                }
            }

            // Pay the proposal owner
            let owner_nfgid = proposal.owner.clone();
            self.add_payouts(owner_nfgid.clone(), maker_buckets);

            (taken_nflids, paid_fees, partial)
        }
        
        /// Checks if the values in the provided map are consistent
        /// and make sense. Panics if this does not hold.
        fn check_asking_map_sanity(map: &HashMap<ResourceAddress, AskingType>) {
            for (resaddr, ask) in map {
                let fung_res = Self::is_resource_fungible(resaddr);
                match ask {
                    AskingType::Fungible(amount) => {
                        assert!(fung_res, "fungible AskingType used for non-fungible resource");
                        assert!(!amount.is_negative(), "cannot ask for negative amounts");
                    },
                    AskingType::NonFungible(_, _) => {
                        assert!(!fung_res, "non-fungible AskingType used for fungible resource");
                    }
                }
            }
        }
        
        /// Converts a `Decimal` to a `u64`. Make sure the input `d`
        /// is already a whole number before calling this function.
        fn dec_to_u64(d: Decimal) -> u64 {
            // TODO This seems unnecessarily convoluted
            //      I need a Decimal::round_to_u64 method
            d.to_string().parse().unwrap()
        }

        /// Finds what ratio of a proposal we are able to make a trade
        /// for while still exchanging as high a number as possible of
        /// whole NFTs for whole NFTs. This is useful for calculating
        /// trade ratios for non-fungible to non-fungible trading
        /// pairs where we cannot allow fractions of either token.
        ///
        /// `paying_q` is how many NFTs are being paid. `offering_q`
        /// is how many NFTs are on offer. `asking_q` is how many NFTs
        /// are being asked for the full batch of NFTs on offer.
        ///
        /// If we cannot find any ratio that works we return zero.
        fn calculate_gcd_ratio(paying_q: u64, offering_q: u64, asking_q:u64)
                               -> PreciseDecimal
        {
            // TODO There must exist a more mathematical way
            // of doing this. Presumably what we want to do is
            // find the greatest common denominator and then
            // that gives a small interval to scan for a
            // usable value. Euclid's algorithm anyone?
            for test_paying_q in (1..paying_q+1).rev() {
                // We can only guarantee a whole number of
                // NFTs in exchange for a whole number of
                // NFTs if the following holds.
                let everything_is_fine =
                    (test_paying_q * offering_q) % asking_q == 0;
                if everything_is_fine {
                    return PreciseDecimal::from(test_paying_q)
                        / PreciseDecimal::from(asking_q);
                }
            }
            return PreciseDecimal::ZERO;
        }
        
        
        /// Investigates a collection of buckets vs the expected
        /// payment in a proposal and determines how big a fraction of
        /// the proposal those buckets are able to cover. The result
        /// is returned as a decimal number between 0 and 1 inclusive.
        ///
        /// Notice that we use `PreciseDecimal` for this to avoid too
        /// much rounding shenanigans in our payments etc.
        ///
        /// For example: if the proposal asks 1000 XRD while offering
        /// 1500 VKC and you send in a bucket with 2 XRD in it, the
        /// return value will be 0.002.
        ///
        /// In the case where non-fungible tokens are involved this
        /// function makes sure that the fraction it returns is
        /// suitable for ensuring that all non-fungibles get traded in
        /// exactly whole numbers. If such a trade is not possible it
        /// will return 0. (Funnily enough zero *is* considered a
        /// whole number.)
        ///
        /// For example: if the proposal asks 500 XRD for 10 RaDragon
        /// NFTs and this function gets passed a bucket with 125 XRD
        /// in it, it will determine that the closest affordable whole
        /// NFT trade is 2 RaDragons for 100 XRD total and so return
        /// 0.2.
        ///
        /// Note that this function is not the final arbiter of
        /// whether a trade should succeed or fail, but rather a
        /// prediction of what we think will happen.
        ///
        /// If `proposal` doesn't allow partial payments this function
        /// always returns 1 on the expectation that if there is
        /// insufficient payment the transaction will simply fail for
        /// other reasons at some later point.
        /// 
        /// Note that the code in this function distinguishes heavily
        /// between fungible and non-fungible tokens, on the basis
        /// that non-fungibles need to always be counted in whole
        /// numbers whereas fungibles can be infinitely subdivided
        /// into arbitrarily long decimal numbers. This is not
        /// strictly the case since fungibles also have a max
        /// divisibility---and taken to the extreme, divisibility
        /// could be zero in which case the fungible has the same mess
        /// going on as non-fungibles do when it comes to calculating
        /// these ratios.
        ///
        /// TODO A more industrial strength version of this code would
        /// take a fungible's divisibility into account when
        /// calculating the ratios but currently the algorithm works
        /// best when your fungible has the usual divisibility of
        /// 18. There may be some rounding weirdness going on towards
        /// the end of those 18 decimal digits but chances are you
        /// don't care so much at that point.
        fn calculate_partial_ratio(proposal: &TradeProposal,
                                   paying_bucket: &Bucket)
                                   -> PreciseDecimal
        {
            if !proposal.allow_partial {
                // This requires payment in full, or the tx will fail
                return PreciseDecimal::ONE;
            }

            let paying_resaddr = &paying_bucket.resource_address();
            let asking_amount = Self::asking_to_amount(&proposal.asking[paying_resaddr]);
            let fungible_offering = Self::is_resource_fungible(
                &proposal.offering.values().next().unwrap().resource_address());
            let fungible_paying = Self::is_resource_fungible(paying_resaddr);


            if (fungible_paying || fungible_offering) &&
                paying_bucket.amount() >= asking_amount
            {
                // Taker has sent us enough so that we are guaranteed
                // to fully clear out this proposal
                return PreciseDecimal::ONE;
            } else {
                // Taker has sent less than the proposal's full asking
                // amount

                if !fungible_paying && !fungible_offering {
                    // If both sides of the trade are non-fungible
                    // we need to make sure that a whole number of
                    // one results in a whole number of the other.

                    // We know that all of the three following
                    // Decimal values are already whole numbers.
                    let offering_q =
                        Self::dec_to_u64(proposal.offering.values().next().unwrap().amount());
                    let asking_q =
                        Self::dec_to_u64(asking_amount);
                    let paying_q =
                        Self::dec_to_u64(paying_bucket.amount());

                    return Self::calculate_gcd_ratio(paying_q, offering_q, asking_q);
                } else if !fungible_offering {
                    // Paying side is fungible and offering side is
                    // non-fungible. This means that the fungible side
                    // can absorb any amount of subdivision to
                    // accomodate the non-fungible side needing to
                    // have a whole number of units.

                    let offering_amount = PreciseDecimal::from(
                        proposal.offering.values().next().unwrap().amount());
                    let from_vault_amount =
                        (offering_amount * paying_bucket.amount())
                        / asking_amount;
                    let floor_amount = from_vault_amount.floor();
                    // We can only sell whole numbers of NFTs so round
                    // down to the nearest whole number of NFTs we can
                    // sell.
                    return floor_amount / offering_amount;
                } else {
                    // Both sides are fungible or only paying side is
                    // non-fungible and so already a whole number, in
                    // either case a simple division does the job
                    return PreciseDecimal::from(paying_bucket.amount()) / asking_amount;
                }
            }
        }

        /// Converts a vector of buckets to a map of buckets indexed
        /// by the resource address of each bucket.
        fn bucket_vec_to_bucket_map(vector: Vec<Bucket>) -> HashMap<ResourceAddress, Bucket> {
            let mut map: HashMap<ResourceAddress, Bucket> = HashMap::new();
            for bucket in vector {
                let entry = map.get_mut(&bucket.resource_address());
                if entry.is_none() {
                    map.insert(bucket.resource_address(), bucket);
                } else {
                    entry.unwrap().put(bucket);
                }
            }
            map
        }

        /// Charge fees (if any) for purchased NFTs
        fn charge_per_nft_fees(&mut self,
                               payment_buckets: &Vec<Bucket>,
                               fee_paying_buckets: &mut HashMap<ResourceAddress, Bucket>,
                               fee_receiving_buckets: &mut HashMap<ResourceAddress, Bucket>) {
            if self.fees.is_some() {
                let nft_fees = self.fees.as_ref().unwrap().per_nft_flat_fee.as_ref();
                if nft_fees.is_some() {
                    for bucket in payment_buckets {
                        // For any NFTs in the return buckets, charge fees
                        if !Self::is_resource_fungible(&bucket.resource_address()) {
                            // Is there a fee for this NFT type?
                            let nft_fee = nft_fees.unwrap().get(&bucket.resource_address());
                            if nft_fee.is_some() {
                                // Yep, so charge the fee
                                let (fee_res, fee_per) = nft_fee.unwrap();
                                if fee_per > &Decimal::ZERO {
                                    Self::charge_fixed_fee(fee_receiving_buckets,
                                                           fee_paying_buckets.get_mut(&fee_res).expect(
                                                               "missing fee payment"),
                                                           *fee_per * bucket.amount());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        /// Charges the fixed per transaction fees for an
        /// interaction. Funds will be taken from `fee_paying_buckets`
        /// and deposited into `fee_receiving_vaults` by the fee rules
        /// (if any) in `fee`.
        fn charge_per_tx_fees(fee: Option<&HashMap<ResourceAddress, AskingType>>,
                              fee_receiving_vaults: &mut HashMap<ResourceAddress, Vault>,
                              fee_paying_buckets: Vec<Bucket>) -> Vec<Bucket> {
            match fee {
                Some(fee) => {
                    let mut fee_paying_buckets = Self::bucket_vec_to_bucket_map(fee_paying_buckets);
                    let mut paid_fees: HashMap<ResourceAddress, Bucket> = HashMap::new();
                    for (fee_res, fee_ask) in fee {
                        let fee_paying_bucket = fee_paying_buckets.get_mut(fee_res);
                        Self::charge_fee(&mut paid_fees,
                                         fee_paying_bucket,
                                         fee_ask);
                    }

                    for bucket in paid_fees.into_values() {
                        Self::put_bucket_in_vault_map(fee_receiving_vaults, bucket);
                    }
                    
                    // Return left-over fees
                    fee_paying_buckets.into_values().collect()
                },
                None => fee_paying_buckets
            }
        }

        /// Helper function to charge a single fee. We take funds from
        /// `payer` and put them into `recipient`.
        fn charge_fee(recipient: &mut HashMap<ResourceAddress, Bucket>,
                      payer: Option<&mut Bucket>,
                      howmuch: &AskingType) {
            match howmuch {
                AskingType::Fungible(price) => {
                    if *price > Decimal::ZERO {
                        Self::charge_fixed_fee(
                            recipient,
                            payer.expect("missing fungible fee"),
                            *price);
                    }
                },
                AskingType::NonFungible(nflids, random_q) => {
                    let random_q = random_q.unwrap_or_default();
                    let (taken_nflids, _) =
                        Self::pick_nfts(&payer,
                                        &PreciseDecimal::ONE,
                                        &nflids,
                                        random_q);

                    if taken_nflids.len() > 0 {
                        Self::put_bucket_in_bucket_map(
                            recipient,
                            payer.unwrap().take_non_fungibles(&taken_nflids));
                    }
                }
            }
        }

        /// Picks NFTs from a `payer` bucket and returns the set of
        /// local ids that were picked. No NFTs are actually removed
        /// from the bucket by this function, it just makes picks.
        /// Think of this as an advisory function, leaving the final
        /// decision to you.
        ///
        /// It tries to pick the local ids in `asking_nflids` and in
        /// addition a number of arbitrary ones equal in quantity to
        /// `random_q`. The total number of NFTs picked gets reduced
        /// by `partial_ratio` if it's not one.
        ///
        /// In addition to returning the picked local ids, it also
        /// returns the number of NFTs it would have picked had this
        /// been possible. If this number is lower than the length of
        /// the set returned, it was prevented from from a full pick
        /// by the `payer` bucket not containing all of the local ids
        /// specified in `asking_nflids`. The caller needs to decide
        /// how to then deal with this situation.
        fn pick_nfts(payer: &Option<&mut Bucket>,
                     partial_ratio: &PreciseDecimal,
                     asking_nflids: &Option<HashSet<NonFungibleLocalId>>,
                     random_q: u64) -> (BTreeSet<NonFungibleLocalId>, Decimal)
        {
            let emptyset: HashSet<NonFungibleLocalId> = HashSet::new();
            let asking_nflids = asking_nflids.as_ref().unwrap_or(&emptyset);
            let max_nfts_to_take : Decimal =
                (*partial_ratio * (random_q + asking_nflids.len() as u64))
                .round(0, RoundingMode::TowardsNearestAndHalfAwayFromZero).truncate();
            let mut nfts_taken = Decimal::ZERO;
            let mut picked_nflids = BTreeSet::new();
            if random_q == 0 && asking_nflids.len() == 0 {
                return (picked_nflids, max_nfts_to_take);
            }
            let mut payer_nflids: HashSet<NonFungibleLocalId> =
                payer.as_ref().expect("insufficient non-fungibles provided")
                .non_fungible_local_ids().iter().map(|v| v.clone()).collect();

            // First grab the named ones so we don't risk
            // taking them as part of the randoms
            for nflid in asking_nflids {
                if payer_nflids.contains(&nflid) {
                    picked_nflids.insert(nflid.clone());
                    payer_nflids.remove(&nflid);
                    nfts_taken += 1;
                    if nfts_taken >= max_nfts_to_take { break; }
                }
            }

            let mut rand_quantity = max_nfts_to_take - nfts_taken;

            if rand_quantity > random_q.into() { rand_quantity = random_q.into(); }
            
            // Then grab the random ones
            let mut payer_nflids_iter = payer_nflids.into_iter();
            let mut counter = Decimal::ZERO;
            // We already know that this rounding, if needed, is correct
            while counter < rand_quantity {
                // Picks an arbitrary NFT
                picked_nflids.insert(payer_nflids_iter.next().expect(
                    "insufficient non-fungibles provided"));
                counter += 1;
                nfts_taken += 1;
            }
            (picked_nflids, max_nfts_to_take)
        }

        /// Charges basis point based fees on fungibles that are being
        /// used by a taker as payment for a proposal.
        fn charge_taker_fees_fungible(fee_receiving_buckets: &mut HashMap<ResourceAddress, Bucket>,
                                      paying_bucket: Option<&mut Bucket>,
                                      price: &Decimal,
                                      partial_ratio: &PreciseDecimal,
                                      fees: &Option<Fees>) {
            if fees.is_none() { return; }
            let fees = fees.as_ref().unwrap();

            if fees.per_payment_bps_fee.is_some() {
                let fee = fees.per_payment_bps_fee.as_ref().unwrap();
                if *fee > Decimal::ZERO {
                    Self::charge_fixed_fee(fee_receiving_buckets,
                                           paying_bucket.expect("missing bps fee payment"),
                                           pdec_to_dec(*partial_ratio * *price * *fee / 10000));
                }
            }
        }

        /// Charges transfer fees for NFTs that are being moved from
        /// taker to maker and vice versa.
        fn charge_taker_fees_nonfungible(fee_receiving_buckets: &mut HashMap<ResourceAddress, Bucket>,
                                         paying_buckets: &mut HashMap<ResourceAddress, Bucket>,
                                         resaddr: &ResourceAddress,
                                         price: u64,
                                         fees: &Option<Fees>)
        {
            if fees.is_none() { return; }
            let fees = fees.as_ref().unwrap();

            if fees.per_nft_flat_fee.is_some() {
                let fee = fees.per_nft_flat_fee.as_ref().unwrap().get(resaddr);
                if let Some((fee_res, fee_q)) = fee {
                    if *fee_q > Decimal::ZERO {
                        Self::charge_fixed_fee(fee_receiving_buckets,
                                               paying_buckets.get_mut(&fee_res).expect(
                                                   "missing fee payment"),
                                               *fee_q * price);
                    }
                }
            }
        }

        /// Convenience function for moving actual funds once we've
        /// decided how much of a fee exactly to charge for a
        /// resource.
        fn charge_fixed_fee(fee_receiving_buckets: &mut HashMap<ResourceAddress, Bucket>,
                            paying_bucket: &mut Bucket,
                            amount: Decimal)
        {
            if Self::is_resource_fungible(&paying_bucket.resource_address()) {
                // Fungible fee is trivial
                Self::put_bucket_in_bucket_map(
                    fee_receiving_buckets,
                    paying_bucket.take(amount));
            } else {
                // Non-fungible fee is non-trivial
                assert_eq!(amount, amount.floor(),
                           "NFT fees must be in whole numbers");
                let mut counter = Decimal::ZERO;
                // We round here because in the case that we got
                // adjusted down by a partial_ratio we *might* be
                // slightly off a whole number. In that case however
                // we have already determined that the whole number is
                // what the user wanted so we just round back to it.
                while counter < amount.round(0, RoundingMode::TowardsNearestAndHalfAwayFromZero) {
                    // We pick arbitrary NFTs until we have enough of them
                    Self::put_bucket_in_bucket_map(
                        fee_receiving_buckets,
                        paying_bucket.take_non_fungible(
                            paying_bucket.non_fungible_local_ids().first().expect(
                                "missing non-fungible fees")));
                    counter += 1;
                }
                // The above would likely be more processing efficient
                // if there was a first_n(amount) method we could call
                // on BTreeSet so we could then call
                // take_non_fungibles (and put_bucket_in_bucketmap)
                // just once instead of calling take_non_fungible etc.
                // multiple times.
            }
        }
        
        /// Puts the contents of `bucket` into the correct entry of
        /// `vault_map`, creating that entry if necessary.
        fn put_bucket_in_vault_map(vault_map: &mut HashMap<ResourceAddress, Vault>,
                                  bucket: Bucket) {
            let resaddr = &bucket.resource_address();
            if vault_map.contains_key(&resaddr) {
                vault_map.get_mut(&resaddr).unwrap().put(bucket);
            } else {
                vault_map.insert(resaddr.clone(), Vault::with_bucket(bucket));
            }
        }

        /// Puts the contents of `bucket` into the correct entry of
        /// `bucket_map`, creating that entry if necessary.
        fn put_bucket_in_bucket_map(bucket_map: &mut HashMap<ResourceAddress, Bucket>,
                                    bucket: Bucket) {
            let resaddr = &bucket.resource_address();
            if bucket_map.contains_key(&resaddr) {
                bucket_map.get_mut(&resaddr).unwrap().put(bucket);
            } else {
                bucket_map.insert(resaddr.clone(), bucket);
            }
        }

        /// Places the input buckets into the payout vaults of the
        /// named `maker`, creating new such vaults as necessary.
        fn add_payouts(&mut self, maker: NonFungibleGlobalId, payment: Vec<Bucket>) {
            // Find or create payout vault storage for this maker
            let mut target_funds = self.payouts.get_mut(&maker);
            if target_funds.is_none() {
                self.payouts.insert(maker.clone(), HashMap::new());
                target_funds = self.payouts.get_mut(&maker);
            }
            let target_funds = target_funds.unwrap();

            // Put the payment in the appropriate payment vaults for
            // the maker, creating new ones as needed
            for bucket in payment {
                Self::put_bucket_in_vault_map(target_funds, bucket);
            }
        }

        /// Converts a vector of buckets to a map of vaults containing
        /// those buckets, with their resource address as key. The
        /// input vector is cannibalized in order to create the map.
        ///
        /// Note that if the input contains multiple buckets of the
        /// same resource then that resource's map entry will contain
        /// the total from all those buckets.
        fn bucket_vec_to_vault_map(buckets: Vec<Bucket>) -> HashMap<ResourceAddress, Vault> {
            let mut vaults = HashMap::new();
            for b in buckets {
                if !vaults.contains_key(&b.resource_address()) {
                    vaults.insert(b.resource_address().clone(),
                                  Vault::with_bucket(b));
                } else {
                    vaults.get_mut(&b.resource_address()).unwrap().put(b);
                }
            }
            vaults
        }
        
        /// Checks if the tokens in `vaults` are compliant with the
        /// rule described by `addrs`. They are compliant if either
        /// `addrs` is `None` *or* if all tokens in `vaults` can be
        /// found in `addrs`. Returns `true` if compliant, or `false`
        /// if not.
        fn check_vault_token_req(vaults: &HashMap<ResourceAddress, Vault>,
                                 addrs: &Option<HashSet<ResourceAddress>>) -> bool {
            if addrs.is_none() { return true; }
// The following commented code is useful for debug output during development
//            vaults.keys().for_each(|&a| info!("vault: is {:?} in {:?}: {:?}",
//                                                 a,
//                                                 addrs.as_ref().unwrap().iter().next().unwrap(),
//                                                 addrs.as_ref().unwrap().contains(&a)));
            vaults.keys().all(|&vault| addrs.as_ref().unwrap().contains(&vault))
        }

        /// Checks if the tokens in `map` are compliant with the rule
        /// described by `addrs`. They are compliant if either `addrs`
        /// is `None` *or* if all tokens in `map` can be found in
        /// `addrs`. Returns `true` if compliant, or `false` if not.
        fn check_map_token_req(map: &HashMap<ResourceAddress, AskingType>,
                               addrs: &Option<HashSet<ResourceAddress>>) -> bool {
            if addrs.is_none() { return true; }
// The following commented code is useful for debug output during development
//            map.keys().for_each(|&a| info!("map: is {:?} in {:?}: {:?}",
//                                              a,
//                                              addrs.as_ref().unwrap().iter().next().unwrap(),
//                                              addrs.as_ref().unwrap().contains(&a)));
            map.keys().all(|&key| addrs.as_ref().unwrap().contains(&key))
        }

        /// Checks if two collections of tokens form a valid
        /// offering/asking pair for this Kaupa instance, according to
        /// its configuration. Returns `true` if they do or `false`
        /// otherwise.
        fn check_token_reqs(&self, 
                           offering: &HashMap<ResourceAddress, Vault>,
                           asking: &HashMap<ResourceAddress, AskingType>) -> bool {
            return (Self::check_vault_token_req(offering, &self.side1_token)
                && Self::check_map_token_req(asking, &self.side2_token))
                ||
                (Self::check_vault_token_req(offering, &self.side2_token)
                 && Self::check_map_token_req(asking, &self.side1_token));
        }

        /// Determines how many tokens in total are being asked for in
        /// an instance of the `AskingType` enum.
        fn asking_to_amount(asking: &AskingType) -> Decimal {
            match asking {
                AskingType::Fungible(price) => price.clone(),
                AskingType::NonFungible(set, amount)
                    => Decimal::from(amount.unwrap_or_default()
                                     + Self::length_of_option_set(set) as u64),
            }
        }

        /// Unwraps an option'd set and returns its length. Returns 0
        /// when the option is None.
        fn length_of_option_set<T>(set: &Option<HashSet<T>>) -> usize {
            match set {
                Some(s) => s.len(),
                None => 0,
            }
        }
        
        /// Will determine what price the `proposal` is asking for
        /// each of its offered tokens. For example if it is offering
        /// 5 XRD for 2 VKC then the "price per" XRD is 2.5.
        ///
        /// Only supported for trading pairs, will panic otherwise.
        fn price_per(&self, proposal: &TradeProposal) -> Decimal {
            assert!(self.is_trading_pair, "not a trading pair");
            Self::asking_to_amount(&proposal.asking.values().next().unwrap())
                / proposal.offering.values().next().unwrap().amount()
        }

        /// Returns the resource type the input `proposal` is offering
        /// for sale.
        ///
        /// Only supported for trading pairs, will panic otherwise.
        fn selling_token(&self, proposal: &TradeProposal) -> ResourceAddress {
            assert!(self.is_trading_pair, "not a trading pair");
            *proposal.offering.keys().next().unwrap()
        }

        /// Returns `true` if the input `proposal` seeks to acquire
        /// this Kaupa instance's side1 token.
        ///
        /// Only supported for trading pairs, will panic otherwise.
        fn is_buying(&self, proposal: &TradeProposal) -> bool {
            assert!(self.is_trading_pair, "not a trading pair");
            *self.side1_token.as_ref().unwrap().iter().next().unwrap()
                != self.selling_token(&proposal)
        }

        /// Adds a new trade proposal to our internal
        /// structures. After this it will be available for users to
        /// accept or rescind etc. The input `proposal` must already
        /// have its `uuid` set or there will be disappointment.
        fn add_proposal(&mut self, proposal: TradeProposal) {
            if self.is_trading_pair {
                let pau = PriceAndUuid { price_per: self.price_per(&proposal), uuid: proposal.uuid };
                if self.is_buying(&proposal) {
                    self.buy_book.as_mut().unwrap().insert(pau);
                } else {
                    self.sell_book.as_mut().unwrap().insert(pau);
                }
            }
            self.proposals.insert(proposal.uuid, proposal);
        }

        /// Takes a proposal out of all our internal structures and
        /// returns the contents of its vaults. The proposal will no
        /// longer be available to operate on after this. If `uuid`
        /// does not refer to an active proposal will panic.
        fn remove_proposal(&mut self, uuid: Uuid) -> Vec<Bucket> {
            // Clean up our data structures
            let proposal = self.proposals.remove(&uuid).expect("no such proposal");
            if self.is_trading_pair {
                if self.is_buying(&proposal) {
                    self.buy_book.as_mut().unwrap().retain(|v| v.uuid != uuid);
                } else {
                    self.sell_book.as_mut().unwrap().retain(|v| v.uuid != uuid);
                }
            }

            // Return the funds in the proposal
            let mut return_buckets = Vec::new();
            proposal.offering.into_values().for_each(|mut v| {
                return_buckets.push(v.take_all());
                self.garbage_vaults.push(v);
            });
            return_buckets
        }

        /// Checks if the input resource is fungible and if so returns
        /// `true`. Returns `false` for non-fungible resources.
        fn is_resource_fungible(resource: &ResourceAddress) -> bool {
            match borrow_resource_manager!(*resource).resource_type() {
                ResourceType::Fungible{divisibility: _} => true,
                ResourceType::NonFungible{id_type: _} => false,
            }
        }

        /// Checks if a Fees structure is consistent with itself and
        /// our component's expectations of it.
        ///
        /// Panics if we don't like the Fees structure.
        fn check_fee_validity(fees: &Option<Fees>) {
            if let Some(fees) = fees.as_ref() {
                if let Some(bps) = fees.per_payment_bps_fee.as_ref() {
                    assert!(!bps.is_negative(),
                            "basis point fee cannot be negative");
                    assert!(*bps <= dec!("10000"),
                            "basis point fee cannot be more than 10000");
                }
                if let Some(map) = fees.per_tx_maker_fixed_fee.as_ref() {
                    Self::check_asking_map_sanity(map);
                }
                if let Some(map) = fees.per_tx_taker_fixed_fee.as_ref() {
                    Self::check_asking_map_sanity(map);
                }
                if let Some(map) = fees.per_nft_flat_fee.as_ref() {
                    for (nftaddr, (_, amount)) in map {
                        assert!(!Self::is_resource_fungible(nftaddr),
                                "NFT fees can only be charged on NFT type resources");
                        assert!(!amount.is_negative(),
                                "cannot charge negative fees for NFTs");
                    }
                }
            }
        }
    }
}
