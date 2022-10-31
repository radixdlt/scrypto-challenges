use scrypto::prelude::*;

blueprint! {
    struct DefaultInterestModel{

    }

    impl DefaultInterestModel {
        pub fn new() -> ComponentAddress {
            Self{}.instantiate().globalize()
        }

        pub fn get_borrow_interest_rate(&self, borrow_ratio: Decimal) -> Decimal{
            if borrow_ratio > Decimal::ONE {
                Decimal::ONE / Decimal::from("5") + Decimal::ONE * Decimal::ONE / Decimal::ONE / Decimal::from("2")
            }
            else{
                borrow_ratio / Decimal::from("5") + borrow_ratio * borrow_ratio / Decimal::ONE / Decimal::from("2")
            }
        }
    }
}