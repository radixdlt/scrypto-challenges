//!

use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub enum ProposalStatus {
    VotingStage,
    Accepted,
    Rejected,
    NotEnoughVotes,
}

impl ProposalStatus {
    pub fn is_voting_stage(&self) -> bool {
        match self {
            ProposalStatus::VotingStage => true,
            _ => false,
        }
    }
}
