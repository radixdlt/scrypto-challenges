use scrypto::prelude::*;

blueprint! {
    struct PseudoPriceOracle {
        prices: HashMap<ResourceAddress, Decimal>,
        system_time: u64,
        round_length: u64,
    }

    impl PseudoPriceOracle {
        pub fn new(
        ) -> ComponentAddress
        {
            return Self {
                prices: HashMap::new(),
                system_time: 0,
                round_length: 0,
            }
            .instantiate()
            .globalize();
        }

        pub fn insert_resource(
            &mut self,
            token_address: ResourceAddress,
        )
        {
        self.prices.insert(token_address, Decimal::one());
        }

        pub fn set_price(
            &mut self,
            token_address: ResourceAddress,
            set_price: Decimal
        )
        {
            *self.prices.get_mut(&token_address).unwrap() = set_price;
            info!("Price of {:?} has been set to {:?}", token_address, set_price);
        }

        pub fn get_price(
            &self,
            token_address: ResourceAddress
        ) -> Decimal
        {
            return *self.prices.get(&token_address).unwrap()
        }

        pub fn get_current_epoch(
            &self,
        ) -> u64
        {
            return Runtime::current_epoch()
        }
    }
}