//
// Creates some validators with fixed test data
// TESTING Hint: For different outcome in the test, just change the hard coded data here
// Usually the data would need to come from ledger or out of a connected database
//

use scrypto::prelude::*;

use crate::data::Validator;

// needs to return a vec with the validator structs
pub fn instantiate_validators() -> Vec<Validator> {

    let mut v: Vec<Validator> = Vec::new();

    v.push( Validator {
            val_name: "Validator 1".to_string(),    
            val_address: "rdx1qspn70eefr2k4dm2zutlggfvdsl1a45haa5l2k5z9w6304p7pdfwp3sl1test".to_string(),
            apy: dec!("9.52"),    
            uptime: dec!("99.8"),
            fee: dec!("2.1"),
            rank: 4,
            does_lottery: false,
            does_charity: false,
            does_airdrops: true,
        }
    );

    v.push( Validator {
            val_name: "Validator 2".to_string(),    
            val_address: "rdx1qspn70eefr2k4dm2zutlggfvdsl1a45haa5l2k5z9w6311p7pdfwp3sl2test".to_string(),
            apy: dec!("9.59"),    
            uptime: dec!("100"),
            fee: dec!("2.5"),
            rank: 36,
            does_lottery: true,
            does_charity: false,
            does_airdrops: false,
        }
    );

    v.push( Validator {
        val_name: "Validator 3".to_string(),    
        val_address: "rdx1qspn80eefr2k4dm2zutlggfvdsl1a66haa5l2k5z9w6311p7pdfwp3sl3test".to_string(),
        apy: dec!("9.59"),    
        uptime: dec!("99.9"),
        fee: dec!("1.5"),
        rank: 11,
        does_lottery: true,
        does_charity: true,
        does_airdrops: true,
        }
    );    

    v.push( Validator {
        val_name: "Validator 3".to_string(),    
        val_address: "rdx1qspn80eefr2k4dm2zutlggfvdsl1a66haa5l2k5z9w6311p7pdfwp3sl4test".to_string(),
        apy: dec!("9.59"),    
        uptime: dec!("99.9"),
        fee: dec!("1.5"),
        rank: 11,
        does_lottery: true,
        does_charity: true,
        does_airdrops: true,
        }
    );
    
    v.push( Validator {
        val_name: "Validator 4".to_string(),    
        val_address: "rdx1qspn80eefr2k4dm2zutlggfvdsl1a66haa5l2k5z9w6311p7pdfwp3sl5test".to_string(),
        apy: dec!("9.38"),    
        uptime: dec!("95.1"),
        fee: dec!("0.5"),
        rank: 13,
        does_lottery: true,
        does_charity: true,
        does_airdrops: true,
        }
    );    
    //return the complete validator list
    v
}



