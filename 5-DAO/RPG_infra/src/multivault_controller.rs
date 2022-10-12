use scrypto::prelude::*;
use crate::multivault::*;

blueprint! {

// 

    struct multivault_controller {
        
        wrapper_token_component_address: ComponentAddress,
        wrapper_token_number: Decimal,
        wrapper_token_burn_vault: Vault,
        multivault_controller_admin_badge_vault: Vault,

    
    impl multivault_controller {

    // accepts the resource/component address of the 'wrapper' tokens
    // together with the total number of them.
    
    // this is in order to instantiate a multivault_controller component that
    // can recognise 'wrapper' tokens and exchange them for fractional contents
    // of its multivault.

        pub fun new(
            wrapper_token_component_address: ComponentAddress,
            wrapper_token_number: Decimal,
        ) -> (Bucket)

        Self {
        Vault::(wrapper_token_burn_vault)

    // Instantiate multivault and put multivault_controller_admin_badge in
    // a vault here

    // Burn method
    // Accepts a bucket of 'wrapper' tokens; returns fractional multivault
    // contents

    pub fun burn(Bucket) -> (Bucket) {
  

    // checks to make sure the correct type of 'wrapper' tokens have been
    // received
    
    // checks the balance of the 'wrapper' tokens in the bucket

        let bucket_balance = 
    
    // calculates the fraction of the multivault to which the caller is
    // entitled

        let withdraw_fraction = bucket_balance / (wrapper_token_number - wrapper_token_burn_vault.balance)

    // sends the 'wrapper' tokens in the bucket to the wrapper_token_burn_vault
    // to burn them

   
        wrapper_token_vault: Vault::with_bucket(Bucket)
    
    // send withdrawal request to multivault
    // need to add logic for multivault_controller_admin_badge

        multivault::withdraw_fractional_contents(withdraw_fraction)
    
    // receive bucket
    // return fractional assets to caller

    }
}


