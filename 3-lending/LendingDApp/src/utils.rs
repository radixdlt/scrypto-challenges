use scrypto::prelude::*;
use sha2::{Digest, Sha256};

/// Check L1 level 
pub fn l1_enabled(number_of_lendings: i32, l1_limit: i32, l2_limit: i32) -> bool {
    let mut value = false;
    if number_of_lendings > l1_limit && number_of_lendings <= l2_limit {
        value = true;
    } else {
        false;
    };
    return value;
}

pub fn l2_enabled(number_of_lendings: i32, _l1_limit: i32, l2_limit: i32) -> bool {
    let mut value = false;
    if number_of_lendings > l2_limit {
        value = true;
    } else {
        false;
    };
    return value;
}

pub fn calculate_ratio(ratio: Decimal, _amount: Decimal) -> Decimal {
    return ratio;
}

pub fn calculate_level(ratio: Decimal, amount: Decimal) -> Decimal {
    return ratio * amount / dec!("100");
}


pub fn pool_low_limit(start_amount: Decimal, low_limit: Decimal) -> Decimal {
    return start_amount*low_limit/dec!("100");
}


pub fn get_non_fungible_id() -> NonFungibleId {
    
    let uuid = Runtime::generate_uuid();    
    let mut hasher = Sha256::new();
    hasher.update(uuid.to_string());
    let uuid_hash = hasher.finalize();
    let id = NonFungibleId::from_bytes(uuid_hash.to_vec());    

    return id;
}
