
use scrypto::prelude::*;

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

pub fn check_ratio(min_ratio: Decimal, amount: Decimal) -> Decimal {
    return min_ratio * amount / dec!("100");
}


pub fn pool_low_limit(start_amount: Decimal, low_limit: Decimal) -> Decimal {
    return start_amount*low_limit/dec!("100");
}