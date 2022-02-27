use scrypto::prelude::*;

/// Sorts the two addresses passed to it and returns them.
/// 
/// # Arguments:
/// 
/// * `address1` (Address) - The first address
/// * `address2` (Address) - The second address
/// 
/// # Returns:
/// 
/// * (Address, Address) - A tuple containing the two addresses passed after they had been sorted.
/// 
/// # Notes:
///    
/// This function performs what is called "Lexicographical comparison" on the vector representation of the two addresses
/// passed to this function. The order in which these two addresses are sorted is not important. The only thing that is
/// important is that this function must be deterministic such that on all runs on the same two addresses it sorts them
/// in the same way. The reason as to why the sorting is not as important as determinism is because this function will 
/// mainly be used for the keys and values of the hashmap storing the liquidity pools. As long as this function output
/// is deterministic, the way of sorting the addresses is irrelevant. 
pub fn sort_addresses(address1: Address, address2: Address) -> (Address, Address) {
    return if address1.to_vec() > address2.to_vec() {
        (address1, address2)
    } else {
        (address2, address1)
    };
}

/// Sorts the two buckets passed depending on their resource addresses and returns them.
/// 
/// # Arguments:
/// 
/// * `bucket1` (Bucket) - The first bucket
/// * `bucket2` (Bucket) - The second bucket
/// 
/// # Returns:
/// 
/// * (Bucket, Bucket) - A tuple of the two buckets sorted according to their resource addresses.
pub fn sort_buckets(bucket1: Bucket, bucket2: Bucket) -> (Bucket, Bucket) {
    // Getting the sorted addresses of the two buckets given
    let sorted_addresses: (Address, Address) = sort_addresses(
        bucket1.resource_address(), 
        bucket2.resource_address()
    );

    // Sorting the buckets and returning them back
    return if bucket1.resource_address() == sorted_addresses.0 {
        (bucket1, bucket2)
    } else {
        (bucket2, bucket1)
    }
}