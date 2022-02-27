use scrypto::prelude::*;
use crate::liquidity_pool::*;
use crate::utils::*;

blueprint!{
    struct RaDEX{
        /// This is a hashmap that maps a tuple of two addresses to a Scrypto component. This scrypto component is a 
        /// liquidity pool meaning that this hashmap maps a tuple of two addresses to a liquidity pool. This hashmap is
        /// used as a way of quickly finding the liquidity pool associated with a given address pair. If a pair of 
        /// addresses does not exist in this hashmap it means that there does not exist a liquidity pool for it on RaDEX
        liquidity_pools: HashMap<(Address, Address), Component>,

        /// That's quite the mouthful. This is a hashmap that is mainly used when liquidity providers are tying to 
        /// remove their portion of liquidity from the liquidity pool. This hashmap is used to find the address pair (
        /// and in turn the liquidity pool) associated with a given tracking token. If the resource address of a given
        /// tracking token does not exist as one of the keys to this hashmap, then this means that this tracking token
        /// does not belong to any of the liquidity pools in RaDEX.
        tracking_token_address_pair_mapping: HashMap<Address, (Address, Address)>
    }
}