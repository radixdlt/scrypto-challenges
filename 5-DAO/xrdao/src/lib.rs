/// **This submission does not need to be judged due to being incomplete***
/// 
/// XRDao protocol is built for medium to long term XRD holders and XRD altcoin
/// investors that are interested in a sustainable rewards structure.  XRDao is the main
/// component where most of the interactions take place.  This blueprint allows users to mint 
/// XRDao which is pegged 1:1 to XRD.  The XRDao token has a small tax taken for every trade, 
/// transfer, mint, or burn starting at .5% and can change from governance vote.  A percent of platform 
/// fees (which also can change via governanve vote) go towards buying protocol owned liquidity from the 
/// radiswap component.  The rest of platform fees collected are distributed to users of XRDao
/// that make successful trades and strategies  when investing through the platform. Future investment
/// choices will be added along with altcoin investing such as staking to trustworthy validators, loaning
/// funds to a lending platform for interest.  In order to use XRDao, a user would send XRD
/// to the platform and mint XRDao token.  The exchange rate for minting and burning XRDao always remains
/// 1:1 minus the buy/sell fee.  The user would then use these XRDao tokens to pick investment options
/// just as they would with XRD, but the platform would be making the actual trades/stakes for you.
/// When a user makes a successful trade, Repuation (rep) tokens are awarded for the amount of xrd earned.
/// 
/// For example:
///     User 1 has 100 xrd and mints 99.5 xrdao, 
///     User 1 invests 99.5 xrdao into oci,
///     Oci goes 2x up in price relative to xrd,
///     User 1 exits oci position
///     User 1 sends xrdao to burn in return for xrd
/// 
/// User 1 now has 198.005 xrdao tokens that can be burned and exchanged for xrd minus the burn fee, as
/// well as 1.495 Rep. Rep is a soulbound token created by the XRDao platform to signify who is most profitable
/// for the platform.  It can't be bought, sold, or traded, but it can be earned and minted for good behavior
/// or removed and burned for bad behavior.  Users are catagorized into Rank by the amount of rep they hold 
/// into the following ranks:
///     
///    President, Highest rep, linear relationship between xrdao and voting power, top 1% of users, 3x reward multiplier
///    VicePresident, x = y^1.1 voting power, top 10% of users, 1.67x reward multiplier
///    Senator, x = y^1.2 voting power, top 30% of users, 1.25x reward multiplier
///    Governor, x = y^1.3 voting power, top 50% of users, 1x reward multiplier
///    Citizen, x = y^1.5 voting power, top 75% of users, .925x reward multiplier
///    Pleb,  Lowest rep, quadratic relationship, x = y^2 voting power, top 100% of users, 0x reward multiplier
/// 
/// As you can see, different ranks give different curves to determine voting power and different shares of the
/// fees generated from the platform.  The more money you make, the more rep you earn, the more fees you acrue.
/// You might be asking yourself, why wouldnt these users just invest themselves and avoid platform fees
/// alltogether?  XRDao harneses sustainable revenues from market volatility.  When the protocol buys liquidity
/// from radiswap for XRDao-XRD, XRD volatility will generate price differences between the two tokens in the pool.
/// When there is ever a price difference above the fee that is being charged (.5%) there will be a profitable
/// arbitration opportunity within the pool.  Whether an arbitrage bot or a user makes it, fees are then
/// generated from that trade and go back to buying more liquidity and rewarding each user based on reputation. 
/// Reputation is basically earning a share of xrd market volatility combined with platform fees.
/// 
/// The majority of my work here is to the xrdaouser and xrdao components.  Tokentax is a start of an attempt to 
/// design a component that a token must pass through in order to extract taxes from dex transactions.  My 
/// understanding is this feature doesnt current work at the resource level, but hopefully it will in the
/// future. Radiswap is instantiated to trade and provide liquidity for XRDao-XRD token.
/// 
/// 
/// XRDaoProposal is a copy paste from the Liquidity DAO scrypto examples that I am in process of modifying to fit
/// my own purposes.  This component is for creating voting proposals that must be funded and voted on.   Vote weight is calculated by
/// total xrdao tokens held on the platform and the curve associated with the user's Rank.  Proposals can be challenge
/// any proposal they deem harmful to the platform by voting with reputation instead of xrdao token balance.
/// When voting with rep, you stake your rep tokens on the correct outcome of the proposal.  Those on the losing side
/// of the vote forfeit their tokens to the winning side.  This way the voters with the most conviction on the best 
/// outcome will be willing to risk their earning power from the platform on the outcome.
/// 
/// written by Austin
/// @aus877 on twitter
/// @aus87 on telegram


mod radiswap;
mod xrdao;  
mod xrdaousers;
mod xrdaoproposal;
mod tokentax;