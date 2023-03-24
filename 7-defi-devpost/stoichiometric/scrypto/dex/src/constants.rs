use scrypto::math::BnumI256;
use scrypto::prelude::Decimal;

pub const NB_STEP: u16 = 65535;

/// Constant equal to 0.0025 that represents the liquidity providers fee for a swap
pub const LP_FEE: Decimal = Decimal(BnumI256::from_digits([2500000000000000, 0, 0, 0]));

/// Constant equal to 0.0005 that represents the protocol fee for a swap
pub const PROTOCOL_FEE: Decimal = Decimal(BnumI256::from_digits([500000000000000, 0, 0, 0]));

/// Constant equal to 0.997 that represents the real amount of tokens traded (after fees)
pub const RATIO_TRADED: Decimal = Decimal(BnumI256::from_digits([997000000000000000, 0, 0, 0]));
