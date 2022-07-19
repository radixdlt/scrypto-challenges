use scrypto::prelude::*;

pub fn assert_rate(rate: Decimal) {
    assert!(rate <= dec!("100") && rate >= Decimal::ZERO, "Wrong data!");
}

pub fn power(rate: Decimal, number: u8) -> Decimal {

    let mut counter = 0;

    let mut result = Decimal::ONE;

    while counter != number {

        counter += 1;
        result *= Decimal::ONE + rate;

    }

    result

}