use scrypto::prelude::*;
  
blueprint! {
    struct MockInstapass {
        veri_token_address: ResourceAddress,
        mint_auth: Vault
    }

    impl MockInstapass {
        // =====================================================================
        // FUNCTIONS
        // =====================================================================
        pub fn instantiate() -> ComponentAddress {
            // generate transient resource buckets  to instantiate the component    
            let mint_auth = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "VERI Mint Authorization")
                .initial_supply(1);

            let veri_tokens_address: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Verified Person Token")
                .metadata("symbol", "VERI")
                .mintable(rule!(require(mint_auth.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();
            
            let access_rules = AccessRules::new()
                .default(rule!(allow_all));

            let mut component = Self {
                veri_token_address: veri_tokens_address,
                mint_auth: Vault::with_bucket(mint_auth),
            }
            .instantiate();

            component.add_access_check(access_rules);
            component.globalize()
        }

        // =====================================================================
        // METHODS
        // =====================================================================
        // Mock function to verify an account as associated with a unique person 
        pub fn verify_account(&mut self, is_unique_person: u16 ) -> Bucket {
            assert!(is_unique_person == u16::one(), "Account determined as non-unique, VERI token will not be distributed");

            self.mint_auth.authorize(|| {
                borrow_resource_manager!(self.veri_token_address).mint(1)
            })  
        }
    }
}