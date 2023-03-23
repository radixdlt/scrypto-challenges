use scrypto::prelude::*;

// User NFT is an NFT that represents users for this protocol. This NFT contains all the records of the user
// interacting with this protocol.
#[derive(NonFungibleData, ScryptoEncode, ScryptoCategorize, ScryptoDecode, LegacyDescribe)]
pub struct User {
    #[mutable]
    pub xrd_collateral: Decimal,
    #[mutable]
    pub usds_borrowed:  Decimal,
    #[mutable]
    pub loan_collateralization_rate: Option<Decimal>,
    #[mutable]
    pub current_usds_in_shield: Decimal,
    #[mutable]
    pub current_scrl_staked: Decimal,
    #[mutable]
    pub shield_deposits: Vec<ShieldDeposit>,
    #[mutable]
    pub scrilla_deposits: Vec<ScrillaDeposit>,
    // #[mutable]
    // pub loan_health_rating: Option<LoanHealthRating>,
}
/// This struct is used to store deposit information for each individual Shield
/// deposit.  These events are stored in a vector and are cycled through to 
/// return your deposits and associated shield rewards when withdrawing.
#[derive(ScryptoDecode, ScryptoEncode, ScryptoCategorize, LegacyDescribe)]
pub struct ShieldDeposit {
    pub event_number: u128,
    pub usds_shield_deposit_amount:  Decimal,
    pub product_at_time_of_deposit: Decimal,
    pub summation_at_time_of_deposit: Decimal,
}

/// This struct is used to store deposit information for each individual Scrilla 
/// deposit into staking.  They are cycled through to return your deposits when unstaking
/// and removed from this list.  These events are stored in a vector on each user's NFT.
#[derive(ScryptoDecode, ScryptoEncode, ScryptoCategorize, LegacyDescribe)]
pub struct ScrillaDeposit {
    pub event_number: u128,
    pub scrilla_deposit_amount:  Decimal,
    pub scrilla_summation_at_deposit: Decimal,
}

/// ***Under construction***
#[derive(ScryptoEncode, ScryptoDecode, LegacyDescribe)]
pub enum OperationalMode {
    Normal,
    Recovery,
}

// ***Under Construction***
// #[derive(TypeId, Encode, Decode)]
// pub enum LoanHealthRating {
//     Perfect,
//     Excellent,
//     Great,
//     Good,
//     Bad,
//     Warning,
//     Liquidation,
//     None,
// }
