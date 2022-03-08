use scrypto::prelude::*;

blueprint! {
    struct TestTokens {
        chest : Vault,
        pot : Vault,
    }

    impl TestTokens {
        pub fn init(name: String ) -> Component {
            let token_bucket: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", name)
                .metadata("symbol", "DUCKM")
                .metadata("description", "Test tokens for the Scrypto DEX Challenge ! (02/2022-03/2022)")
                .initial_supply_fungible(7500);

            Self {
                chest: Vault::with_bucket(token_bucket),
                pot: Vault::new(RADIX_TOKEN),
            }.instantiate()
        }

        //Writes in console the number of DUCKM test tokens in the component
        pub fn count(&self ) {
            info!("The special containers have : {} DUCKM.",self.chest.amount());
        }
        pub fn next_count(&self, nbr: Decimal ) {
            info!("The special containers will have : {} DUCKM.",self.chest.amount()-nbr);
        }

        //Return as many duckm test tokens as requested for free
        pub fn get_for_free(&mut self, nbr: Decimal ) -> Bucket {
            self.count();
            info!("A safe removal will take place : {} DUCKM will be taken !",nbr);
            self.next_count(nbr);
            return self.chest.take(nbr);
        }

        //Return as many duckm test tokens as requested for 1 xrd per token. taken from the given bucket
        pub fn get_with_radix(&mut self, nbr: Decimal, mut payment: Bucket ) -> (Bucket, Bucket) {
            self.count();
            info!("A safe removal will take place : {} DUCKM will be taken !",nbr);
            self.next_count(nbr);

            self.pot.put(payment.take(nbr));

            return (self.chest.take(nbr), payment);
        }
    }
}