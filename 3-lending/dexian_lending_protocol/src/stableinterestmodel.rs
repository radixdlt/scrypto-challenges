use scrypto::prelude::*;

blueprint! {
    struct StableInterestModel{

    }

    impl StableInterestModel {
        pub fn new() -> ComponentAddress {
            Self{}.instantiate().globalize()
        }

        pub fn get_borrow_interest_rate(&self, borrow_ratio: Decimal) -> Decimal{
            let x2 = 
                if borrow_ratio > Decimal::ONE {
                    // let x = Decimal::ONE;
                    // x * x / Decimal::ONE;
                    Decimal::ONE
                }
                else{
                    borrow_ratio * borrow_ratio / Decimal::ONE
                };
            let x4 = x2 * x2 / Decimal::ONE;
            let x8 = x4 * x4 / Decimal::ONE;
            let hundred = Decimal::from("100");
            Decimal::from("55") * x4 / hundred + Decimal::from("45") * x8 / hundred
        }
    }
}