use scrypto::prelude::*;

// Oracle used to bring real-world time data onto the chain
blueprint! {
    struct TimeOracle {
        // fee that is collected every time a user wants to update the timestamp
        fee_vault: Vault,
        // the adming badge that is used to empty the fee_vault
        admin_badge_def: ResourceAddress,
        time_string: String,  

    }

    impl TimeOracle {
        // Implement the functions and methods which will manage those resources and data
        
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_hello() -> (ComponentAddress, Bucket) {
         
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
            }
            .instantiate();

            // Define the access rules for this blueprint.
        let access_rules = AccessRules::new()
        .method("do_admin_task", rule!(require(badges.resource_address())));
        

        // Return the component and the badges
        (component.add_access_check(access_rules).globalize(), badges)

        
        }
        // Updates the time with the help of an off-chain API and the frontend build with he frontend sdk
        pub fn update_time(&mut self){

        }
        
        pub fn do_admin_task(&self) {
            // This method can only be called if the user
            // presents a badge of the `admin_badge_def` resource definition
        }

    
    }
}