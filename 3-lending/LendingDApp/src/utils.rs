

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
    return value;}

