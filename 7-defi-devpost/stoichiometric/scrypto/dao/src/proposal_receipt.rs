//! Definition of the ProposalReceipt NFR which is sent to someone that makes a proposal

use scrypto::prelude::*;

#[derive(
    NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone,
)]
pub struct ProposalReceipt {
    /// Id of the proposal made
    pub proposal_id: u64,
}
