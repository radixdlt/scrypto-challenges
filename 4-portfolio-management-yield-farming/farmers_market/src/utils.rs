use scrypto::prelude::*;

/// Sorts the two addresses passed to it and returns them.
/// 
/// # Arguments:
/// 
/// * `string1` (String) - The first resource address
/// * `string2` (String) - The second resource address
/// 
/// # Returns:
/// 
/// * (String, String) - A tuple containing the two addresses passed after they had been sorted.
/// 
/// # Notes:
///    
/// This function performs what is called "Lexicographical comparison" on the vector representation of the two addresses
/// passed to this function. The order in which these two addresses are sorted is not important. The only thing that is
/// important is that this function must be deterministic such that on all runs on the same two addresses it sorts them
/// in the same way. The reason as to why the sorting is not as important as determinism is because this function will 
/// mainly be used for the keys and values of the hashmap storing the liquidity pools. As long as this function output
/// is deterministic, the way of sorting the addresses is irrelevant. 
pub fn sort_string(string1: String, string2: String) -> (String, String) {
    return if string1 > string2 {
        (string1, string2)
    } else {
        (string2, string1)
    };
}
