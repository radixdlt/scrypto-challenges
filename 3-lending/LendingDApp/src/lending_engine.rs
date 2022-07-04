use scrypto::prelude::*;
use crate::lending_app::*;

blueprint!{
    /// The LendingEngine blueprint do not perform any kind of mathematics, it is a registry 
    /// of all of the Loan pools and routes all the method calls to the correct loan pool.
    struct LendingEngine{

        loan_pool: HashMap<ResourceAddress, LendingApp>
    }

    impl LendingEngine {
        /// Instantiates a new LendingEngine component. 
        /// # Returns a new LendingEngine component.
        pub fn instantiate_pool(
            starting_tokens: Bucket,
            start_amount: Decimal,
            fee: Decimal,
            reward: Decimal,
        ) -> ComponentAddress {

            let  add: ResourceAddress = starting_tokens.resource_address(); 
            let lending_app: ComponentAddress  = LendingApp::instantiate_pool(
                starting_tokens, start_amount, fee, reward
            );
            let mut loan_pool_int: HashMap<ResourceAddress, LendingApp> = HashMap::new();
            loan_pool_int.insert(add,lending_app.into());

            // no arguments 
            return Self {
                loan_pool: loan_pool_int,            
            }
            .instantiate()
            .globalize();
       }

        pub fn register(&mut self,tokens: Bucket) -> (Bucket,Bucket) {

            info!("Registering ");

            // Checking if exist a lending app for the token received
            self.assert_pool_exists(tokens.resource_address());

            return (self.loan_pool[&tokens.resource_address()].register(),tokens);
        }

        pub fn lend(
            &mut self,
            tokens: Bucket,
            ticket: Proof,
        ) -> Bucket {
            info!("Lending ");
            // Checking if exist a lending app for the token received
            self.assert_pool_exists(tokens.resource_address());

            return self.loan_pool[&tokens.resource_address()].lend_money(tokens, ticket);
        }        

        pub fn pool_exist(
            &self,
            address1: ResourceAddress
        ) -> bool {
            //  checking if the addresses exists in the hashmap of loan pools or not.
            return self.loan_pool.contains_key(&address1);
        }

        pub fn assert_pool_exists(
            &self,
            address1: ResourceAddress
        ) {
            assert!(
                self.pool_exist(address1), 
                "[{}]: No Loan pool exists for the given address.",
                address1
            );
        }        

        pub fn assert_pool_doesnt_exist(
            &self,
            address1: ResourceAddress
        ) {
            assert!(
                !self.pool_exist(address1), 
                "[{}]: A Loan pool with the given address already exists.",
                address1
            );
        }

    }
}