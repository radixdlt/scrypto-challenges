//
// Creates some dex values with fixed test data
// TESTING Hint: For different outcome in the test, just change the hard coded data here
// Usually the data would need to come from ledger or out of a connected database
//

use scrypto::prelude::*;
use crate::data::Dex;

// needs to return a vec with the validator structs
pub fn instantiate_dex() -> Vec<Dex> {

    let mut d: Vec<Dex> = Vec::new();

    d.push( Dex {
            dex_name: "Ociswap".to_string(),    
            token: "DPH".to_string(),
            value: dec!("4.53"), 
            swap_fee: dec!("0.004"),                       
        }
    );

    d.push( Dex {
            dex_name: "Caviarswap".to_string(),    
            token: "DPH".to_string(),
            value: dec!("4.28"),
            swap_fee: dec!("0.005"),                        
        }
    );

    d.push( Dex {
            dex_name: "Caviarswap".to_string(),    
            token: "XSE".to_string(),
            value: dec!("2.05"),
            swap_fee: dec!("0.005"),                        
        }
    );

    d
}



