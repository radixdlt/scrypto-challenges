use scrypto::prelude::*;

blueprint! {
    struct IndexPool {

        //index pool vault that holds tokens
        index_pool:Vault
    }

    impl IndexPool {

        pub fn new(
            resource_address:ResourceAddress
        ) -> ComponentAddress {
            let component = Self {
                index_pool:Vault::new(resource_address)
            }
            .instantiate()
            .globalize();
            return component;
        }

        //Deposit tokens into index pool vault
        pub fn deposit(&mut self, tokens:Bucket){
            self.index_pool.put(tokens);
        }

        //Get blance of index pool vault
        pub fn balance(&self) -> Decimal {
            return self.index_pool.amount();
        }

        //Withdraws tokens from index pool vault
        pub fn withdraw(&mut self, amount:Decimal) -> Bucket {
            return self.index_pool.take(amount);
        }

        pub fn take_all(&mut self) -> Bucket {
            return self.index_pool.take_all();
        }

    }
}