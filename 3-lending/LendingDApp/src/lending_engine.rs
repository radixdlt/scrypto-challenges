use scrypto::prelude::*;
use crate::lending_app::LendingApp;

blueprint!{
    /// The LendingEngine blueprint do not perform any kind of mathematics, it is a registry 
    /// of all of the Loan pools and routes all the method calls to the correct loan pool.
    struct LendingEngine{

        loan_pool: HashMap<ResourceAddress, LendingApp>
    }

    impl LendingEngine {
        /// Instantiates a new LendingEngine component. 
        pub fn new() -> ComponentAddress {
            return Self {
                loan_pool: HashMap::new()
            }
            .instantiate()
            .globalize();            
        }

       /// # Returns a new LendingApp component.
       pub fn new_loan_pool(
            &mut self,
            token1: Bucket,
            start_amount: Decimal,
            fee: Decimal,
            reward: Decimal,
        )   {
            info!("Check if it already exists");
            // Checking if a loan pool already exists between these two tokens
            self.assert_pool_doesnt_exist(token1.resource_address());

            let  add: ResourceAddress = token1.resource_address(); 

            info!("Creating new LendingApp ") ;
            let lending_app: ComponentAddress  = LendingApp::instantiate_pool(
                token1, start_amount, fee, reward
            );
            self.loan_pool.insert(add,lending_app.into());
        }


        pub fn register(&mut self,address: ResourceAddress) -> Bucket {

            info!("Registering for lending with {} ", address) ;

            // Checking if exist a lending app for the token received
            self.assert_pool_exists(address);

            return self.loan_pool[&address].register();
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

        pub fn show_pools(
            &mut self
        ) {
            info!("How many loan pools does exist ? ");

            for (key, _value) in &self.loan_pool {
                info!("ResourceAddress of token managed by pool is {}", key); 
                info!("Loan pool size is {}", self.loan_pool[&key].loan_pool_size());
                info!("Main pool size is {}", self.loan_pool[&key].main_pool_size());
            }
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

        pub fn assert_parameters_are_different(
            &self,
            tokens: ResourceAddress,
            fee: Decimal,
            reward: Decimal,
        ) {
            if self.pool_exist(tokens) {
                assert!(
                    !(fee<=self.loan_pool[&tokens].fee()+1 && fee>=self.loan_pool[&tokens].fee()-1), 
                    "[{}]: A Loan pool with similar parameters already exists.",
                    self.loan_pool[&tokens].fee()
                );
            } 
            if self.pool_exist(tokens) {
                assert!(
                    !(reward<=self.loan_pool[&tokens].reward()+1 && reward>=self.loan_pool[&tokens].reward()-1), 
                    "[{}]: A Loan pool with similar parameters already exists.",
                    self.loan_pool[&tokens].reward()
                );
            }             
        }

    }
}