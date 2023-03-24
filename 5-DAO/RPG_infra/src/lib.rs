use scrypto::prelude::*;
use crate::multivault_controller::*;

blueprint! {
    
    struct RPG_infra {
    
    wrapper_token_number: Decimal,
    no_of_issues: Decimal,
    issue_interval: u64,
    wrapper_token_vault: Vault,
    wrapper_token_component_address: ComponentAddress,
    timestamp: u64,

    impl RPG_infra {

    // new function takes arguments for total number of 'wrapper' tokens
    // to be issued, number of token tranches to be issued and the time-interval
    // after which each tranche becomes available for withdrawal

    // Returns an admin_badge to the caller that is required for withdrawals

    pub fun new(
        wrapper_token_number: Decimal,
        no_of_issues: Decimal,
        issue_interval: Decimal,
    ) -> (Bucket)

    // gets a timestamp

    let timestamp = scrypto::core::Runtime::current_epoch();

    // creates 'wrapper' tokens and puts them in the wrapper_token_vault vault

    let wrapper_tokens: Bucket = ResourceBuilder::new_fungible()
    .divisibility(DIVISIBILITY_MAXIMUM)
    .metadata("name", "wrapper_token")
    .metadata("symbol", "WT")
    .initial_supply(wrapper_token_number);

    Self {
        wrapper_token_vault: Vault::with_bucket(wrapper_tokens)
    
        // [add all the other stuff in one of these to populate the struct?]
    
    }

    // passes the resource/component address of the 'wrapper' tokens to the
    // new() of the multivault_controller component, together with the total 
    // number ofthem.
    
    // this is in order to instantiate a multivault_controller component that
    // can recognise 'wrapper' tokens and exchange them for fractional contents
    // of its multivault.
    //
    // [don't know how to do this properly]

    let wrapper_token_component_address = self.wrapper_token_vault.get(&address)

    multivault_controller::new(
        wrapper_token_component_address, wrapper_token_number
    )
    
    // creates admin badge to return to caller
    
    let admin_badge: Bucket = ResourceBuilder::new_fungible()
    .divisibility(DIVISIBILITY_NONE)
    .metadata("name", "admin badge")
    .metadata("symbol", "AB")
    .initial_supply(1);
    
    // returns admin badge?

    return Bucket

}

    .instantiate()
    .globalize();

}

// Method for the owner of the admin_badge to withdraw 'wrapper' tokens.
// Withdrawals are only allowed according to time elapsed after timestamp
// specified by no_of_issues and issue_interval
//
// [need to add admin_badge logic]

pub fun withdraw_wrapper_tokens() {

let tranche_size <Decimal> = wrapper_token_number / no_of_issues
let time_elapsed <u64> = scrypto::core::Runtime::current_epoch() - timestamp
let issues_elapsed <u64> = (time_elapsed / no_of_issues).floor
let issues_elapsed = issues_elapsed <Decimal>
let available_wrapper_tokens = (issues_elapsed * tranche_size) - (wrapper_token_number - self.wrapper_token_vault.get(&balance))

// [don't know how to do this properly]

return wrapper_token_vault.take(available_wrapper_tokens);

return Bucket;

}
}