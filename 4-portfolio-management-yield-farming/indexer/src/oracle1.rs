use scrypto::prelude::*;

blueprint! {
    struct Oracle1 {

        //Hashmap used to store token prices
        token_price: HashMap<ResourceAddress, Decimal>,

    }

    impl Oracle1 {
        
        pub fn new() -> ComponentAddress{

            let component = Self {
                token_price: HashMap::new(),
                }
                .instantiate()
                .globalize();
                return component;
        }

        //Method used to set token prices
        pub fn set_price(&mut self, token_address:ResourceAddress, price:Decimal) {
            self.token_price.insert(token_address, price);
        }

        //Method used to get token prices
        pub fn get_price(&self, token_address: ResourceAddress) -> Decimal{
            let token_price:Decimal = *self.token_price.get(&token_address).unwrap();
            return token_price;
        }
    }  
}
