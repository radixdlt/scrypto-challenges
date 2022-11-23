use scrypto::prelude::*;

blueprint! {
    struct OraclePlaceholder {
        xrd_price: Decimal
    }

    impl OraclePlaceholder {
        pub fn new() -> ComponentAddress {
            info!("New OraclePlaceholder with default xrd_price: $0.10");

            Self{
                xrd_price: dec!("0.10") // Default starting price of xrd as .10
            }
            .instantiate()
            .globalize()
        }

        pub fn set_price(&mut self, price: Decimal) {
            info!("Oracle set_price: {}", price);
            self.xrd_price = price;
        }

        pub fn get_price(&self) -> Decimal {
            self.xrd_price
        }
    }
}