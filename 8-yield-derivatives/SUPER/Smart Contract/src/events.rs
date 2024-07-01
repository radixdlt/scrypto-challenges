use scrypto::prelude::*;
use crate::YieldClaim;

/// Used to communicate changes within the component
/// A version of this is stored in the component itself
/// It is updated whenever necessary but must be defined as both
/// an event and a struct in order for the component to emit it's
/// own state effectively.
#[derive(ScryptoSbor, ScryptoEvent, Clone)]
pub struct SaleDetailEvent {

    pub dapp_definition_caddy: Vec<GlobalAddress>,
    pub component_caddy: ComponentAddress,
    pub pool_caddy: ComponentAddress,

    pub owner_badge_raddy: ResourceAddress,
    pub component_badge_raddy: ResourceAddress,
    pub db_updater_raddy: ResourceAddress,

    pub super_raddy: ResourceAddress,
    pub super_y_raddy: ResourceAddress,
    pub super_t_raddy: ResourceAddress,
    pub yield_nft_raddy: ResourceAddress,

    pub sale_started: bool,
    pub sale_completed: bool,

    pub sale_start_time_unix: i64,
    pub sale_start_time_utc: String,

    pub sale_end_time_unix: i64,
    pub sale_end_time_utc: String,

}


/// This event is emitted when a new Yield NFT is created. 
/// It contains the NFT's ID, the hour of minting, the amount of SUPER tokens minted, 
/// and the amount of trust tokens (SUPERt) minted.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct CreateYieldNFTEvent {
    pub nft_id: u64,
    pub hour_of_mint: u64,
    pub n_super_minted: u64,
    pub n_trust_minted: Decimal,
}


/// This event is emitted when a Yield NFT is burned.
/// It contains the NFT's ID, the hour of minting, the amount of SUPER tokens that were minted,
/// and the amount of trust tokens (SUPERt) minted with the NFT.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct BurnYieldNFTEvent {
    pub nft_id: u64,
    pub hour_of_mint: u64,
    pub n_super_minted: u64,
    pub n_trust_minted: Decimal,
}


/// This event is emitted when the withdrawal epochs are calculated.
/// It contains a vector of strings representing the epochs at which withdrawals from the vested XRD
/// (the XRD governed by the constant `FRACTION_VESTED`) are scheduled.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct WithdrawalCalculationEvent {
    pub withdraw_epochs: Vec<String>,
}


/// This event is emitted when yield is claimed.
/// It contains the hour of the claim, the amount of SUPERy tokens minted, 
/// the NFT's ID used for the claim, and the amount of trust fund tokens (SUPERt) redeemed.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ClaimYieldEvent {
    pub hour_of_claim: u64,
    pub super_y_minted: Decimal,
    pub nft_id: u64,
    pub trust_fund_redemption_amount: Decimal,
}


/// This event is emitted to show the amount of SUPER tokens minted at a specific time.
/// It contains the time of the update and the number of SUPER tokens minted.
/// This event is used by the `show_hourly_super_minted` function.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ShowSuperMintedEvent {
    pub time: u64, // time at which the update occurred
    pub n_super: u64,
}


/// This event is used by the show_hourly_yield_generated function to emit information about the 
/// yield generated for each NFT at each recorded hour.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct YieldUpdateEvent {
    pub time: u64, //time at which the update occurred
    pub nft_id: u64,
    pub yield_generated: Decimal, // Yield received per 'Super' token
}

/// This event is emitted when a Yield NFT is split into multiple NFTs.
/// It contains the ID and data of the burned NFT, the ID and data of the first newly created NFT, 
/// and a vector of IDs and data for the rest of the newly created NFTs.
/// This event is used by the `split_yield_nft` function.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SplitNFTEvent {
    pub burnt_nft_id: u64,
    pub burnt_nft_data: YieldClaim,
    pub first_nft_id: NonFungibleLocalId,
    pub first_nft_data: YieldClaim,
    pub rest_nft_ids: Vec<NonFungibleLocalId>,
    pub rest_nft_data: YieldClaim
}