use scrypto::prelude::*;

blueprint! {
    
    struct OciPool {

        //Vault to hold liquidity for trading 
        oci_pool:Vault
    }

    impl OciPool {

        pub fn new(
            resource_address:ResourceAddress
        ) -> ComponentAddress {
            let component = Self {
                oci_pool:Vault::new(resource_address)
            }
            .instantiate()
            .globalize();
            return component;
            
        }

        //Method to deposit funds into Ociswap pools
        pub fn deposit(&mut self, tokens:Bucket){
            self.oci_pool.put(tokens);
        }

        //Method to withdraw funds from Ociswap pools
        pub fn withdraw(&mut self, token_amount:Decimal)->Bucket{
            return self.oci_pool.take(token_amount);
        }

        //Method to get balance
        pub fn balance(&self) -> Decimal {
            return self.oci_pool.amount();
        }
    }
}