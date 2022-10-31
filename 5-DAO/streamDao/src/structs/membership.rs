use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct Membership {
    #[scrypto(mutable)]
    pub name: String,

    // <channel id, (start subscribe, end subscribe, last claim rewards)>
    #[scrypto(mutable)]
    pub channels: HashMap<String, (u64, u64, u64)>,

    //voting power for each subscribed channel reward
    // ** each channel has a reward address
    #[scrypto(mutable)]
    pub vote_power: HashMap<ResourceAddress, Decimal>,
}
