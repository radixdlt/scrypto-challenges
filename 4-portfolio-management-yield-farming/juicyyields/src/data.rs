//
// Here we define the structs and enums that are containing the needed data
// for our Juicy Yields
//

use scrypto::prelude::*;

// Note: usually a user would at least have an email address and a password as well. 
// To simplify the concept, the user will just be represented by a name and ResourceAddress here
// We also just work with XRD for the deposit balance. For the future it would make sense to add other token as well of course.
#[derive(Describe, Encode, Decode, TypeId)]
pub struct User {    
    pub user_name: String,
    pub user_address: ResourceAddress,
    pub preferences: UserPref,    
}

// defines levels the user can choose for risk and network contribution
#[derive(Describe, Encode, Decode, TypeId)]
pub enum LevelChoice{
    Low,
    Medium,
    High
}

// holds user preferences on investment strategy
#[derive(Describe, Encode, Decode, TypeId)]
pub struct UserPref {    
    pub risk_level: LevelChoice,
    pub contribution_level: LevelChoice,
    pub wants_lottery: bool,
    pub wants_charity: bool,        
    pub wants_airdrops: bool,
}

// holds necessary data of a validator considered for staking
#[derive(Debug, Describe, Encode, Decode, TypeId)]
pub struct Validator {    
    pub val_name: String,    
    pub val_address: String,
    pub apy: Decimal,    
    pub uptime: Decimal,
    pub fee: Decimal,
    pub rank: u8,
    pub does_lottery: bool,
    pub does_charity: bool,
    pub does_airdrops: bool,
}

// holds necessary data of a dex considered for arbitrage
#[derive(Debug, Describe, Encode, Decode, TypeId)]
pub struct Dex {    
    pub dex_name: String,    
    pub token: String,    
    pub value: Decimal,
    pub swap_fee: Decimal,
}

// defines all possible investment types
#[derive(Debug, Describe, Encode, Decode, TypeId)]
pub enum InvestmentType {
    Staking,
    Lending,
    Betting,
    Lottery,
    Arbitrage
}

// holds chosen investment types and values for the user
#[derive(Debug, Describe, Encode, Decode, TypeId)]
pub struct Investment {    
    pub inv_type: InvestmentType,
    pub inv_value: Decimal,
    pub inv_address: String,    
}

