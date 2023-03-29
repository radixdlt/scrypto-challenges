use crate::utils::get_position_voting_power;
use scrypto::prelude::*;
use stoichiometric_dex::position::Position;

#[derive(
    NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone,
)]
pub struct VoterCard {
    #[mutable]
    pub voting_power: Decimal,
    #[mutable]
    pub stablecoins_locked: Decimal,
    #[mutable]
    pub positions_locked_ids: Vec<NonFungibleLocalId>,
    #[mutable]
    pub last_proposal_voted_id: u64,
    #[mutable]
    pub proposals_voted: HashSet<u64>,
}

impl VoterCard {
    pub fn new() -> Self {
        Self {
            voting_power: Decimal::ZERO,
            stablecoins_locked: Decimal::ZERO,
            positions_locked_ids: Vec::new(),
            last_proposal_voted_id: 0,
            proposals_voted: HashSet::new(),
        }
    }

    pub fn add_stablecoins(&mut self, amount: Decimal) -> Decimal {
        self.voting_power += amount;
        self.stablecoins_locked += amount;
        amount
    }

    pub fn add_position(&mut self, position: &Position, id: NonFungibleLocalId) -> Decimal {
        let voting_power_increase = get_position_voting_power(position);
        self.voting_power += voting_power_increase;
        self.positions_locked_ids.push(id);
        voting_power_increase
    }

    pub fn add_proposals_to_voted(&mut self, proposal_id: u64) -> bool {
        let did_not_contained = self.proposals_voted.insert(proposal_id);

        if did_not_contained {
            self.last_proposal_voted_id = proposal_id
        };

        did_not_contained
    }
}
