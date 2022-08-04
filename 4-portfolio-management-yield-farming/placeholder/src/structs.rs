use scrypto::prelude::*;

#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub struct AssetManager {
    pub funds: BTreeSet<NonFungibleId>,
}