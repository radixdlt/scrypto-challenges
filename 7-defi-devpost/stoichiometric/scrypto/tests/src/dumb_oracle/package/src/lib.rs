use scrypto::blueprint;

#[blueprint]
mod dumb_oracle {
    pub struct DumbOracle {
        price: Decimal,
    }

    impl DumbOracle {
        pub fn new() -> ComponentAddress {

            Self {
                price: Decimal::ZERO,
            }
                .instantiate()
                .globalize()
        }

        pub fn set_price(&mut self, price: Decimal) {
            self.price = price;
        }

        pub fn get_twap_since(&self, _token: ResourceAddress, _timestamp: i64) -> Decimal
        {
            self.price
        }

        pub fn new_observation(&mut self, _token: ResourceAddress) {}
    }
}