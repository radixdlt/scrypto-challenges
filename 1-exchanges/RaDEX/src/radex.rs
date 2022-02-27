use scrypto::prelude::*;
use crate::liquidity_pool::*;
use crate::utils::*;

blueprint!{
    /// RaDEX is an implementation of an automated market maker decentralized exchange on the Radix ledger. The 
    /// liquidity pools in this DEX use the constant market maker function `x * y = k` for the trading of tokens.
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

    impl RaDEX {
        /// Instantiates a new RaDEX component. 
        /// 
        /// # Returns 
        /// 
        /// `Component` - A new RaDEX component.
        pub fn new() -> Component {
            // The RaDEX AMM does not take any arguments 
            return Self {
                liquidity_pools: HashMap::new(), 
                tracking_token_address_pair_mapping: HashMap::new()
            }.instantiate();
        }

        /// Checks if a liquidity pool for the given pair of tokens exists or not.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (Address) - The address of the first token.
        /// * `address` (Address) - The address of the second token.
        /// 
        /// # Returns:
        /// 
        /// * `bool` - A boolean of whether a liquidity pool exists for this trading pair.
        pub fn pool_exists(
            &self,
            address1: Address,
            address2: Address
        ) -> bool {
            // Sorting the two addresses passed and then checking if the tuple of sorted addresses exists in the hashmap
            // of liquidity pools or not.
            let sorted_addresses: (Address, Address) = sort_addresses(address1, address2);
            return self.liquidity_pools.contains_key(&sorted_addresses);
        }

        /// Asserts that the given address pair exists in the DEX.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (Address) - The address of the first token.
        /// * `address` (Address) - The address of the second token.
        pub fn assert_pool_exists(
            &self,
            address1: Address,
            address2: Address,
            label: String
        ) {
            assert!(
                self.pool_exists(address1, address2), 
                "[{}]: No liquidity pool exists for the given address pair.", 
                label
            );
        }
    }
}