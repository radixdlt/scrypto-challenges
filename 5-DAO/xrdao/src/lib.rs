/// **This submission does not need to be judged due to being incomplete***
/// 
/// XRDao protocol is built for medium to long term XRD holders and XRD altcoin
/// investors.  XRDao is the main component where most of the interactions
/// take place.  This blueprint allows users to mint XRDao which is pegged 1:1 to XRD.  The
/// XRDao token has a small tax taken for every trade, transfer, mint, or burn starting at .5%
/// and can change from governance vote.  Platform fees go towards buying protocol owned
/// liquidity from the radiswap component and rewarding users
/// 
/// The basis of this platform is that as
/// Users can add Radix altcoins to the investments available for the platform by 
/// governance proposal and vote.  Users take their XRDao tokens and invest in these
/// tokens in similar ways as doing it themselves. Users are incentivized to use the 
/// platform by earning repuataion (rep) tokens when making successful trades and/or earning
/// fees for the protocol. The more rep a user earns, the higher rank the user can achieve.
/// Higher ranks allow users to 
///     1) Earn substantially higher shares of platform fees distributed
///     2) Increases Users base voting power curve
///     3) 
/// 
///   
/// 
/// the protocol takes a small fee on minting and burning
/// 
/// The  
/// 
/// 
/// 
/// XRDaoUsers component organizes users information and stores this
/// data on soul bound tokens (SBTs) given to each user.  This component is responsible
/// for keeping all data and balances up to date on these SBTs.
/// 
/// XRDaoProposal component allows users to create

mod radiswap;
mod xrdao;  
mod xrdaousers;
mod xrdaoproposal;
mod tokentax;