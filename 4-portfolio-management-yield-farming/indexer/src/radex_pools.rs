use scrypto::prelude::*;

blueprint! {
    
    struct RadexPool {

        //Vault to hold liquidity for trading 
        radex_pool:Vault
    }

    impl RadexPool {

        pub fn new(
            resource_address:ResourceAddress
        ) -> ComponentAddress {
            let component = Self {
                radex_pool:Vault::new(resource_address)
            }
            .instantiate()
            .globalize();
            return component;
            
        }

        //Method to deposit funds into Radex pools
        pub fn deposit(&mut self, tokens:Bucket){
            self.radex_pool.put(tokens);
        }

        //Method to withdraw funds from Ociswap pools
        pub fn withdraw(&mut self, token_amount:Decimal)->Bucket{
            return self.radex_pool.take(token_amount);
        }

        //Method to get balance
        pub fn balance(&self) -> Decimal {
            return self.radex_pool.amount();
        }
    }
}