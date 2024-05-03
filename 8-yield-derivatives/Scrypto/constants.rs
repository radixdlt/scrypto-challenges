use scrypto::prelude::*;

// [(7 d/wk) * (24 hr/d) * (60 min/hr)] / (5 min/epoch) = 2016 epochs/week
// Withdrawal of 1/16th of vault will be allowed every 2016 epochs
pub const FRACTION_VESTED: Decimal = dec!("0.4");
pub const FRACTION_TRUST_FUND: Decimal = dec!("0.6");
pub const WEEKS_VESTED: u64 = 16;
pub const TIME_SECONDS_PER_HOUR: u64 = 60 * 60;

pub const _TIME_HOURS_PER_WEEK: u64 = 7 * 24;
pub const DAYS_PER_VEST_PERIOD: u64 = 7;
pub const SALE_DURATION_DAYS: u64 = 7;
pub const EULER: Decimal = dec!("2.718281828459045235");
