use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct Creator {
    #[scrypto(mutable)]
    pub power_vote: Decimal,
}
