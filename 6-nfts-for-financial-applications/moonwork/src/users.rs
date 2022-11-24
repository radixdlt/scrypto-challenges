use scrypto::prelude::*;

#[derive(Debug, NonFungibleData)]
pub struct Client {
    #[scrypto(mutable)]
    pub jobs_paid_out: u64,
    #[scrypto(mutable)]
    pub jobs_created: u64,
    #[scrypto(mutable)]
    pub total_paid_out: Decimal,
    #[scrypto(mutable)]
    pub disputed: u64,
}

#[derive(Debug, NonFungibleData)]
pub struct Contractor {
    #[scrypto(mutable)]
    pub jobs_completed: u64,
    #[scrypto(mutable)]
    pub total_worth: Decimal,
    #[scrypto(mutable)]
    pub disputed: u64,
}
