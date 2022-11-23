use scrypto::prelude::*;

// Oracle used to bring real-world time data onto the chain
blueprint! {
    struct TimeOracle {
        // fee that is collected every time a user wants to update the timestamp
        fee_vault: Vault,
        // the adming badge that is used to empty the fee_vault
        admin_badge_def: ResourceAddress,
        // the time string UNIX format
        time_string: String, 
        //the counter that is used to check if the next update_time() is paid for 
        paid_requests: Decimal,  

    }

    impl TimeOracle {
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_time_oracle() -> (ComponentAddress, Bucket) {
         
        // Create the admin badges
        let badges: Bucket = ResourceBuilder::new_fungible()
        .divisibility(DIVISIBILITY_NONE)
        .metadata("name", "Admin Badge")
        .initial_supply(dec!(1)); 

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let component = Self {
                fee_vault: Vault::new(RADIX_TOKEN),
                admin_badge_def: badges.resource_address(),
                time_string: String::new(),
                paid_requests: dec!(0),
            }
            .instantiate();

            // Define the access rules for this blueprint.
        let access_rules = AccessRules::new()
        .method("collect_fees", rule!(require(badges.resource_address()))).default(rule!(allow_all));

  
        // Return the component and the badges
        (component.add_access_check(access_rules).globalize(), badges)

        }

        // allows users to pay for time updates
        pub fn pay_for_update_time(&mut self, mut payment: Bucket) -> Bucket {
            // Put 1 (xrd) in the fee vault 
            self.fee_vault.put(payment.take(dec!(1)));
            // increase the number of outstanding paid_requests
            self.paid_requests = self.paid_requests + dec!(1);
            // returns bucket of xrd if too much was paid 
            payment
        }

        // Updates the time with the help of an off-chain API and the frontend build with he frontend sdk
        pub fn update_time(&mut self, new_time: String){
            // Check that there is at least one request that is paid for
            assert!(self.paid_requests > dec!(1), "Need to pay for request first");
            //Decrease counter 
            self.paid_requests -= 1;
            //Updates the time 
            self.time_string = new_time;
        }

        // returns the last UNIX time_string 
        pub fn get_time(&self) -> String {
           let time = self.time_string.clone();
           time
        }

        // collects fees. Can only be called by component owner (has admin_badge)
        pub fn collect_fees(&mut self){
            self.fee_vault.take_all();
        }
    
    }
}