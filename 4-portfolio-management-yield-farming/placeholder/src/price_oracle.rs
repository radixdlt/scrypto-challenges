use scrypto::prelude::*;

blueprint! {
    struct PriceOracle {
        prices: HashMap<ResourceAddress, Decimal>,
    }

    impl PriceOracle {
        pub fn new(
        ) -> ComponentAddress
        {
            return Self {
                prices: HashMap::new(),
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
            self.prices.entry(token_address).and_modify(|e| { *e = set_price }).or_insert(set_price);
            info!("Price of {:?} has been set to {:?}", token_address, set_price);
        }

        pub fn get_price(
            &self,
            token_address: ResourceAddress
        ) -> Decimal
        {
            return *self.prices.get(&token_address).unwrap()
        }
    }
}