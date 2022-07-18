use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct UserNft {}

impl UserNft {
    pub fn new() -> Self {
        return Self {};
    }
}