use scrypto::prelude::*;

#[derive(Decode, TypeId, Encode, Describe, Debug, PartialEq)]
pub enum Status {
    Pending,
    Active,
    Closed,
}

#[derive(NonFungibleData)]
pub struct Proposal {
    #[scrypto(mutable)]
    pub choices: HashMap<String, ResourceAddress>,

    pub from: NonFungibleId,

    #[scrypto(mutable)]
    pub ipfs_link: String,

    #[scrypto(mutable)]
    pub start: u64,

    #[scrypto(mutable)]
    pub end: u64,

    #[scrypto(mutable)]
    pub timestamp: u64,

    #[scrypto(mutable)]
    pub voters: HashMap<NonFungibleId, (String, Decimal)>,

    #[scrypto(mutable)]
    pub status: Status,

    #[scrypto(mutable)]
    pub winner: (String, Decimal),
}
