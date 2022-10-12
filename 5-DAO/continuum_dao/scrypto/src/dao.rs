use scrypto::prelude::*;
  
blueprint! {
    struct DAOComponent {
        internal_admin: Vault,                                                  // internal DAO admin
        external_admin: Vault,                                                  // external admin tokens                        
        managed_component: ComponentAddress,                                    // managed component
        managed_functions: HashSet<String>,                                     // list of managed functions 
        proposal_fee: Decimal,                                                  // fee to submit a vote proposal (xrd)
    }

    impl DAOComponent {
        // =====================================================================
        // FUNCTIONS
        // =====================================================================
        pub fn instantiate(
            component_address: ComponentAddress,
            admin_token: Bucket 
        ) -> (ComponentAddress, Bucket) {
            // transient admin badges to be used for component instantiation
            let mut internal_admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "DAO Admin Badge")
                .initial_supply(2);
            
            // define access rule for DAO methods
            let access_rules = AccessRules::new()
                .method("set_proposal_fee", rule!(require(internal_admin.resource_address())))
                .default(rule!(allow_all));
            
            // DAO instantiation
            let mut component = Self {
                internal_admin: Vault::with_bucket(internal_admin.take(1)),
                external_admin: Vault::with_bucket(admin_token),
                managed_component: component_address,
                managed_functions: HashSet::new(),
                proposal_fee: Decimal::one(),
            }
            .instantiate();

            component.add_access_check(access_rules);

            (component.globalize(), internal_admin)
        }

        // =====================================================================
        // METHODS
        // =====================================================================
        /// Adds to set of  
        pub fn add_external_function_control(
            &mut self, 
            component_address: ComponentAddress, 
            function_name: String,
        ) -> () {
            match component_address == self.managed_component {
                true => {
                    match self.managed_functions.contains(&function_name) {
                        false => {self.managed_functions.insert(function_name);}
                        true => (),
                    }
                }
                false => panic!("Access mismatch"),
            }   
        }
        
        /// DAO internal admin restricted function to reset DAO vote proposal 
        /// fee 
        pub fn set_proposal_fee(&mut self, proposal_fee: Decimal) -> () {
            self.proposal_fee = proposal_fee;
        }

        /// Mock function that "proposes" a change to the existing managed 
        /// component and calls the function to directly change the parameter. 
        /// A voting process should ensue but I'm out of time and so I just 
        /// want to test this mechanism for now. 
        /// This function should not be taken seriously 
        pub fn propose_parameter_change(
            & self, 
            address: ComponentAddress,
            function_name: String,
            new_value: Decimal) -> () {
            
            let proof = self.external_admin.create_proof();
            
            Component::from(address)
                .call::<()>(&function_name, args![new_value, proof]);
        }
    }
}