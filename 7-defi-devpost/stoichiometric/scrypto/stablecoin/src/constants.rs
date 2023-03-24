use scrypto::math::BnumI256;
use scrypto::prelude::Decimal;

pub const SECONDS_PER_DAY: Decimal =
    Decimal(BnumI256::from_digits([13897502818169782272, 4683, 0, 0]));

pub const FLASH_LOAN_FEE: Decimal = Decimal(BnumI256::from_digits([1000500000000000000, 0, 0, 0]));
