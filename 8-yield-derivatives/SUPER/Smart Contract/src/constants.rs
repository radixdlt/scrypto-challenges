use scrypto::prelude::*;

// [(7 d/wk) * (24 hr/d) * (60 min/hr)] / (5 min/epoch) = 2016 epochs/week
// Withdrawal of 1/16th of vault will be allowed every 2016 epochs

/// Fraction of XRD paid by user to be unlocked to Owner over the `WEEKS_VESTED`
pub const FRACTION_VESTED: Decimal = dec!("0.4");

/// Fraction of XRD paid by user to place in `OneResourcePool` and mint `SUPERt` against
pub const FRACTION_TRUST_FUND: Decimal = dec!("0.6");

/// May also be referred to as the development period, weeks over which SUPERy is generated
pub const WEEKS_VESTED: u64 = 16;

///Number of seconds in an hour.
pub const TIME_SECONDS_PER_HOUR: u64 = 60 * 60;

///Number of hours in a week. 
pub const _TIME_HOURS_PER_WEEK: u64 = 7 * 24;

///Number of days per vesting period, each period unlocks an equal fraction of the tokens locked within the component from the fraction `FRACTION_VESTED`
pub const DAYS_PER_VEST_PERIOD: u64 = 7;

///Duration of the sale in days.    
pub const SALE_DURATION_DAYS: u64 = 7;

///Euler's number, accurate to 18 decimal places. 
pub const EULER: Decimal = dec!("2.718281828459045235");

///π (pi), accurate to 18 decimal places
pub const PI: Decimal = dec!("3.141592653589793238");
