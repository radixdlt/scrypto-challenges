use scrypto::prelude::*;

#[derive(
    NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone,
)]
pub struct FlashMint {
    /// Minted amount
    pub amount_minted: Decimal,
}

impl FlashMint {
    pub fn new(amount: Decimal) -> Self {
        Self {
            amount_minted: amount,
        }
    }
}
