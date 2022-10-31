use scrypto::prelude::*;

#[derive(Decode, Encode, TypeId, Describe, Clone)]
pub struct Channel {
    pub channel_id: String,
    pub name: String,
    pub members: BTreeSet<NonFungibleId>,
    pub rewards_address: ResourceAddress,
    pub create_epoch: u64,
    pub creator_id: NonFungibleId,
}
