use scrypto::prelude::{Clock, Decimal, TimePrecision};
use stoichiometric_dex::position::Position;

pub fn get_position_voting_power(position: &Position) -> Decimal {
    let mut voting_power = Decimal::ZERO;
    for (_, step_position) in &position.step_positions {
        voting_power += step_position.liquidity;
    }

    voting_power
}

#[inline]
pub fn get_current_time() -> i64 {
    Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch
}
