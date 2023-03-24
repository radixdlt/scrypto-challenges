use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct InsurerBadgeData {
    // amount
    pub amount: Decimal,
    // creation date
    pub date: i64,
    // reward percent rate
    #[mutable]
    pub reward_percent_rate: Decimal,
    // date of the last reclaim of reward
    #[mutable]
    pub last_reward_reclaim_date: i64,
    // insurance policy id
    pub policy_id: u128,
}

//
#[derive(NonFungibleData)]
pub struct InsuredBadgeData {
    // amount to cover
    pub cover_amount: Decimal,
    #[mutable]
    // coverage end date
    pub coverage_end_date: i64,
    // contribution percent rate (annual)
    pub contribution_percent_rate: Decimal,
    // Indicate if there is a current claim report
    #[mutable]
    pub current_claim_report: Option<NonFungibleLocalId>,
    // get accepted claims
    #[mutable]
    pub accepted_claims: Vec<NonFungibleLocalId>,
    // get declined claims
    #[mutable]
    pub declined_claims: Vec<NonFungibleLocalId>,
    // insurance policy id
    pub policy_id: u128,
}
#[derive(
    ScryptoCategorize,
    ScryptoEncode,
    ScryptoDecode,
    LegacyDescribe,
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
)]
pub enum ClaimState {
    Declared = 1,
    UnderInvestigation = 10,
    Accepted = 20,
    Refused = 30,
    Collected = 40,
}

#[derive(NonFungibleData)]
pub struct InsuredClaimBadgeData {
    pub insured_badge_id: NonFungibleLocalId,
    pub claim_report: String,
    pub claim_amount: Decimal,
    #[mutable]
    pub state: ClaimState,
    pub policy_id: u128,
}

#[derive(
    ScryptoCategorize,
    ScryptoEncode,
    ScryptoDecode,
    LegacyDescribe,
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
)]
pub enum ListingState {
    Listed = 1,
    AcceptedAnOffer = 2,
    AmountHasBeenCollected = 3,
    Delisted = 4,
}

#[derive(NonFungibleData)]
pub struct InsurerMarketListingData {
    pub insurer_badge_id: NonFungibleLocalId,
    pub listing_amount: Decimal,
    #[mutable]
    pub listing_state: ListingState,
    pub policy_id: u128,
}
