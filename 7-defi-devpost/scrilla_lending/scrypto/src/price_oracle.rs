use scrypto::prelude::*;

#[blueprint]
mod price_oracle_module {
    struct PriceOracle {
        xrd_price: Decimal,
    }
    #[derive(Copy)]
    impl PriceOracle {
        pub fn new() -> ComponentAddress {
            
            let component = Self {
                xrd_price: dec!("0.05"),
            };
            component.instantiate()
            .globalize()
        }

        pub fn set_price(&mut self, new_price: Decimal) {
            self.xrd_price = new_price;
            info!("Price of xrd has been set to {:?}", new_price);
        }

        pub fn get_price(&self) -> Decimal {
            self.xrd_price
        }
    }
}