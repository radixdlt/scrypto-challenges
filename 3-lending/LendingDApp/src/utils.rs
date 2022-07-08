
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

pub fn calculate_ratio(ratio: Decimal, amount: Decimal) -> Decimal {
    if amount<=dec!(2000) {
        return ratio;
    } else if amount<=dec!(10000) {
        info!("Lowering ratio limit ... ");
        return ratio-1;
    } else if amount<=dec!(100000) {
        info!("Lowering ratio limit ... ");
        return ratio-2;
    } else {
        info!("Lowering ratio limit ... ");
        return ratio-3;
    } 
}

pub fn calculate_level(ratio: Decimal, amount: Decimal) -> Decimal {
    if amount<=dec!(2000) {
        //min ratio for lenders is ratio value, if main loan is below 2000 token
        return ratio * amount / dec!("100");
    } else if amount<=dec!(10000) {
        info!("Lowering ratio limit ... ");
        //min ratio for lenders is ratio -1  value, if main loan is between 2000 and 10000 token
        return (ratio-1) * amount / dec!("100");
    } else if amount<=dec!(100000) {
        info!("Lowering ratio limit ... ");
        //min ratio for lenders is ratio -2  value, if main loan is between 10000 and 100000 token
        return (ratio-2) * amount / dec!("100");
    } else {
        info!("Lowering ratio limit ... ");
        //min ratio for lenders is ratio -1  value, if main loan is above 100000
        return (ratio-3) * amount / dec!("100");
    } 
}


pub fn pool_low_limit(start_amount: Decimal, low_limit: Decimal) -> Decimal {
    return start_amount*low_limit/dec!("100");
}