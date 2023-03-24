use scrypto::prelude::*;

blueprint! {

    struct multivault {
        
        // somehow 

        vaults: <ResourceAddress, Vault>,
        
    
    impl multivault {

        pub fun new()

        // bunch of stuff goes here
        // returns multivault_controller_admin_badge to
        // multivault_controller component
    
    }
    
        pub fun deposit (&mut self, bucket: Bucket) {

        // somehow


        }



        pub fun withdraw_fractional_contents(Decimal) -> (Bucket) {

            // accepts withdraw_fraction decimal but only from the appropriate
            // multivault_controller component with the appropriate
            // multivault_controller_admin_badge

            // calculates, withdraws, and returns fractions of the balance of each
            // of the assets in the multivault
        }}}