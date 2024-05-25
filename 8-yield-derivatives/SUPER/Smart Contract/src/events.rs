use scrypto::prelude::*;

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

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct WithdrawalCalculationEvent {
    pub withdraw_epochs: Vec<String>,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct CreateYieldNFTEvent {
    pub nft_id: u64,
    pub hour_of_mint: u64,
    pub n_super_minted: u64,
    pub n_trust_minted: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct BurnYieldNFTEvent {
    pub nft_id: u64,
    pub hour_of_mint: u64,
    pub n_super_minted: u64,
    pub n_trust_minted: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct CreateSplitNFTEvent {
    pub nft_id: u64,
    pub hour_of_mint: u64,
    pub n_super_minted: u64,
    pub n_trust_minted: Decimal,
}



#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ClaimYieldEvent {
    pub hour_of_claim: u64,
    pub super_y_minted: Decimal,
    pub nft_id: u64,
    pub trust_fund_redemption_amount: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct YieldUpdateEvent {
    pub time: u64, //time at which the update occurred
    pub nft_id: u64,
    pub yield_generated: Decimal, // Yield received per 'Super' token
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ShowSuperMintedEvent {
    pub time: u64, // time at which the update occurred
    pub n_super: u64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct CurrentSaleStatusEvent {
    pub sale_currently_active: bool, //  time at which the update occurred
}
