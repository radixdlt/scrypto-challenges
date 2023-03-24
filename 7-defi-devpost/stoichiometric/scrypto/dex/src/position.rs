//! Definition of [`StepPosition`] and [`Position`]

use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct StepPosition {
    /// Liquidity of the position
    pub liquidity: Decimal,

    /// Value of the `stable_fees_per_liq` variable of the PoolStep last time that the
    /// associated [`Position`] collected fees.
    pub last_stable_fees_per_liq: Decimal,

    /// Value of the `other_fees_per_liq` variable of the PoolStep last time that the associated
    /// Position collected fees
    pub last_other_fees_per_liq: Decimal,
}

impl StepPosition {
    /// Returns a new [`StepPosition`].
    pub fn new() -> Self {
        Self {
            liquidity: Decimal::zero(),
            last_stable_fees_per_liq: Decimal::zero(),
            last_other_fees_per_liq: Decimal::zero(),
        }
    }

    /// Updates the [`StepPosition`] from another one.
    pub fn update(&mut self, new_position: &StepPosition) {
        self.liquidity = new_position.liquidity;
        self.last_stable_fees_per_liq = new_position.last_stable_fees_per_liq;
        self.last_other_fees_per_liq = new_position.last_other_fees_per_liq;
    }
}

#[derive(
    NonFungibleData, ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone,
)]
pub struct Position {
    /// Other token of the position
    pub token: ResourceAddress,

    #[mutable]
    /// [`StepPosition`]s in the [`Position`]
    pub step_positions: HashMap<u16, StepPosition>,
}

impl Position {
    /// Creates a new [`Position`] for a token.
    pub fn from(token: ResourceAddress) -> Self {
        Self {
            token,
            step_positions: HashMap::new(),
        }
    }

    /// Returns a [`StepPosition`] for the given step.
    pub fn get_step(&self, step: u16) -> StepPosition {
        match self.step_positions.get(&step) {
            None => StepPosition::new(),
            Some(step_position) => step_position.clone(),
        }
    }

    /// Inserts a new [`StepPosition`] to the steps of the [`Position`].
    pub fn insert_step(&mut self, step: u16, pool_step: StepPosition) {
        self.step_positions.insert(step, pool_step);
    }

    /// Removes a [`StepPosition`] from the steps of the [`Position`].
    pub fn remove_step(&mut self, step: u16) -> StepPosition {
        match self.step_positions.remove(&step) {
            None => StepPosition::new(),
            Some(step_position) => step_position,
        }
    }
}
