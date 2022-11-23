use scrypto::prelude::*;
use crate::liquidity_pool::*;
use crate::utils::*;

blueprint!{
    /// RaDEX is an implementation of an automated market maker decentralized exchange on the Radix ledger. The 
    /// liquidity pools in this DEX use the constant market maker function `x * y = k` for the trading of tokens.
    /// 
    /// The RaDEX blueprint and components do not perform any kind of mathematics, calculation, or anything like that on 
    /// their own. Instead, a RaDEX component may be thought of as a registry of all of the RaDEX liquidity pools and as 
    /// a router which routes swaps and other method calls to the correct liquidity pool.
    struct RaDEX{
        /// This is a hashmap that maps a tuple of two addresses to a Scrypto component. This scrypto component is a 
        /// liquidity pool meaning that this hashmap maps a tuple of two addresses to a liquidity pool. This hashmap is
        /// used as a way of quickly finding the liquidity pool associated with a given address pair. If a pair of 
        /// addresses does not exist in this hashmap it means that there does not exist a liquidity pool for it on RaDEX
        liquidity_pools: HashMap<(ResourceAddress, ResourceAddress), LiquidityPool>,

        /// That's quite the mouthful. This is a hashmap that is mainly used when liquidity providers are tying to 
        /// remove their portion of liquidity from the liquidity pool. This hashmap is used to find the address pair (
        /// and in turn the liquidity pool) associated with a given tracking token. If the resource address of a given
        /// tracking token does not exist as one of the keys to this hashmap, then this means that this tracking token
        /// does not belong to any of the liquidity pools in RaDEX.
        tracking_token_address_pair_mapping: HashMap<ResourceAddress, (ResourceAddress, ResourceAddress)>
    }

    impl RaDEX {
        /// Instantiates a new RaDEX component. 
        /// 
        /// # Returns 
        /// 
        /// `Component` - A new RaDEX component.
        pub fn new() -> ComponentAddress {
            // The RaDEX AMM does not take any arguments 
            return Self {
                liquidity_pools: HashMap::new(), 
                tracking_token_address_pair_mapping: HashMap::new()
            }
            .instantiate()
            .globalize();
        }

        /// Checks if a liquidity pool for the given pair of tokens exists or not.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (ResourceAddress) - The resource address of the first token.
        /// * `address` (ResourceAddress) - The resource address of the second token.
        /// 
        /// # Returns:
        /// 
        /// * `bool` - A boolean of whether a liquidity pool exists for this trading pair.
        pub fn pool_exists(
            &self,
            address1: ResourceAddress,
            address2: ResourceAddress
        ) -> bool {
            // Sorting the two addresses passed and then checking if the tuple of sorted addresses exists in the hashmap
            // of liquidity pools or not.
            let sorted_addresses: (ResourceAddress, ResourceAddress) = sort_addresses(address1, address2);
            return self.liquidity_pools.contains_key(&sorted_addresses);
        }

        /// Asserts that a liquidity pool for the given address pair exists on the DEX.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (ResourceAddress) - The resource address of the first token.
        /// * `address` (ResourceAddress) - The resource address of the second token.
        pub fn assert_pool_exists(
            &self,
            address1: ResourceAddress,
            address2: ResourceAddress,
            label: String
        ) {
            assert!(
                self.pool_exists(address1, address2), 
                "[{}]: No liquidity pool exists for the given address pair.", 
                label
            );
        }
        
        /// Asserts that a liquidity pool for the given address pair doesn't exist on the DEX.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (ResourceAddress) - The resource address of the first token.
        /// * `address` (ResourceAddress) - The resource address of the second token.
        pub fn assert_pool_doesnt_exists(
            &self,
            address1: ResourceAddress,
            address2: ResourceAddress,
            label: String
        ) {
            assert!(
                !self.pool_exists(address1, address2), 
                "[{}]: A liquidity pool with the given address pair already exists.", 
                label
            );
        }

        /// Creates a new liquidity pool in the DEX.
        /// 
        /// This method is used to create a new liquidity pool between the two provided tokens on RaDEX.
        /// 
        /// This method does a number of checks before a Liquidity Pool is created, these checks are:
        /// 
        /// * **Check 1:** Checks that there does not already exist a liquidity pool for the two given tokens.
        /// 
        /// The majority of the checking is done in the `new` function of the LiquidityPool where it checks to ensure 
        /// that the buckets are not empty, tokens are not both the same, as well as other things. The checks done here
        /// are just DEX checks to ensure that we don't create a liquidity pool for a token pair that already has a 
        /// liquidity pool.
        /// 
        /// # Arguments: 
        /// 
        /// * `token1` (Bucket) - A bucket containing the amount of the first token used to initialize the pool.
        /// * `token2` (Bucket) - A bucket containing the amount of the second token used to initialize the pool.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A bucket containing the tracking tokens issued to the creator of the liquidity pool.
        pub fn new_liquidity_pool(
            &mut self,
            token1: Bucket,
            token2: Bucket,
        ) -> Bucket {
            // Checking if a liquidity pool already exists between these two tokens
            self.assert_pool_doesnt_exists(
                token1.resource_address(), token2.resource_address(), 
                String::from("New Liquidity Pool")
            );

            // Sorting the two buckets according to their resource addresses and creating a liquidity pool from these
            // two buckets.
            let (bucket1, bucket2): (Bucket, Bucket) = sort_buckets(token1, token2);
            let addresses: (ResourceAddress, ResourceAddress) = (bucket1.resource_address(), bucket2.resource_address()); 
            let (liquidity_pool, tracking_tokens): (ComponentAddress, Bucket) = LiquidityPool::new(
                bucket1, bucket2, dec!("0.3")
            );

            // Adding the liquidity pool to the hashmap of all liquidity pools
            self.liquidity_pools.insert(
                addresses,
                liquidity_pool.into()
            );

            // Adding the resource address of the tracking tokens to the hashmap that maps the tracking tokens with 
            // the address of their token pairs
            self.tracking_token_address_pair_mapping.insert(
                tracking_tokens.resource_address(),
                addresses
            );

            // Returning the tracking tokens back to the caller of this method (the initial liquidity provider).
            return tracking_tokens;
        }

        /// Adds liquidity to a new or an already existing liquidity pool.
        /// 
        /// This method is used to add liquidity to a liquidity pool in the DEX. If a liquidity pool for the two tokens
        /// passed already exists then liquidity would be directly added to it. However, if a pool doesn't exist, then a
        /// new liquidity pool is created from the two buckets passed to this method.
        /// 
        /// # Arguments:
        /// 
        /// * `token1` (Bucket) - A bucket containing the amount of the first token to add to the pool.
        /// * `token2` (Bucket) - A bucket containing the amount of the second token to add to the pool.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A bucket of the remaining tokens of the `token1` type.
        /// * `Bucket` - A bucket of the remaining tokens of the `token2` type.
        /// * `Bucket` - A bucket of the tracking tokens issued to the liquidity provider.
        pub fn add_liquidity(
            &mut self,
            token1: Bucket,
            token2: Bucket,
        ) -> (Option<Bucket>, Option<Bucket>, Bucket) {
            // Sorting the two buckets of tokens passed to this method and getting the addresses of their resources.
            let (bucket1, bucket2): (Bucket, Bucket) = sort_buckets(token1, token2);
            let addresses: (ResourceAddress, ResourceAddress) = (bucket1.resource_address(), bucket2.resource_address()); 

            // Attempting to get the liquidity pool component associated with the provided address pair.
            let optional_liquidity_pool: Option<&LiquidityPool> = self.liquidity_pools.get(&addresses);
            match optional_liquidity_pool {
                Some (liquidity_pool) => { // If it matches it means that the liquidity pool exists.
                    info!("[DEX Add Liquidity]: Pool for {:?} already exists. Adding liquidity directly.", addresses);
                    let returns: (Bucket, Bucket, Bucket) = liquidity_pool.add_liquidity(bucket1, bucket2);
                    (Some(returns.0), Some(returns.1), returns.2)
                }
                None => { // If this matches then there does not exist a liquidity pool for this token pair
                    // In here we are creating a new liquidity pool for this token pair since we failed to find an 
                    // already existing liquidity pool. The return statement below might seem somewhat redundant in 
                    // terms of the two empty buckets being returned, but this is done to allow for the add liquidity
                    // method to be general and allow for the possibility of the liquidity pool not being there.
                    info!("[DEX Add Liquidity]: Pool for {:?} doesn't exist. Creating a new one.", addresses);
                    (None, None, self.new_liquidity_pool(bucket1, bucket2))
                }
            }
        }

        /// Removes liquidity from the appropriate liquidity pool in the DEX.
        /// 
        /// The main use of this method is to remove liquidity from one of the liquidity pools in the DEX and return 
        /// back the liquidity provider's share of the liquidity pool. The first thing that this method does is that it
        /// ensures that the tracking tokens are valid tracking tokens that actually belong to a liquidity pool. This 
        /// method then finds the liquidity pool that issues the specified tracking tokens and removes the liquidity 
        /// from it.
        /// 
        /// This method performs a number of checks before liquidity removed from the pool:
        /// 
        /// * **Check 1:** Checks to ensure that the provided tracking tokens are valid.
        /// 
        /// # Arguments:
        /// 
        /// * `tracking_tokens` (Bucket) - A bucket of the tracking tokens that the liquidity provider wishes to 
        /// exchange for their share of the liquidity.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A Bucket of the share of the liquidity provider of the first token.
        /// * `Bucket` - A Bucket of the share of the liquidity provider of the second token.
        pub fn remove_liquidity(
            &mut self,
            tracking_tokens: Bucket
        ) -> (Bucket, Bucket) {
            // Check to make sure that the tracking tokens provided are indeed valid tracking tokens that belong to this
            // DEX.
            assert!(
                self.tracking_token_address_pair_mapping.contains_key(&tracking_tokens.resource_address()),
                "[DEX Remove Liquidity]: The tracking tokens given do not belong to this exchange."
            );

            // Getting the address pair associated with the resource address of the tracking tokens and then requesting
            // the removal of liquidity from the liquidity pool
            let addresses: (ResourceAddress, ResourceAddress) = self.tracking_token_address_pair_mapping[&tracking_tokens.resource_address()];
            return self.liquidity_pools[&addresses].remove_liquidity(tracking_tokens);
        }

        /// Swaps the input tokens for tokens of the desired type.
        /// 
        /// This method is used to swap tokens for other tokens. This method first checks that there does exist a 
        /// liquidity pool between the input and the output tokens. If a liquidity pool is found, then the swap goes
        /// through.
        /// 
        /// This method performs a number of checks before the swap is performed:
        /// 
        /// * **Check 1:** Checks that there does exist a liquidity pool for the given pair of tokens.
        /// 
        /// # Arguments:
        /// 
        /// * `tokens` (Bucket) - A bucket containing the input tokens that will be swapped for other tokens.
        /// * `output_resource_address` (ResourceAddress) - The resource address of the token to receive from the swap.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A bucket of the other tokens.
        pub fn swap(
            &mut self,
            tokens: Bucket,
            output_resource_address: ResourceAddress
        ) -> Bucket {
            // Checking if there does exist a liquidity pool for the given pair of tokens
            self.assert_pool_exists(tokens.resource_address(), output_resource_address, String::from("DEX Swap"));

            // Sorting the two addresses passed, getting the associated liquidity pool and then performing the swap.
            let sorted_addresses: (ResourceAddress, ResourceAddress) = sort_addresses(
                tokens.resource_address(), 
                output_resource_address
            );
            return self.liquidity_pools[&sorted_addresses].swap(tokens);
        }

        /// Swaps the exact amount of input tokens for tokens of the desired type.
        /// 
        /// This method is used to swap all of the given token (let's say Token A) for their equivalent amount of the
        /// other token (let's say Token B). This method supports slippage in the form of the `min_amount_out` where
        /// the caller is given the option to specify the minimum amount of Token B that they're willing to accept for
        /// the swap to go through. If the output amount does not satisfy the `min_amount_out` specified by the user 
        /// then this method fails and all of the parties involved get their tokens back.
        /// 
        /// This method performs a number of checks before the swap is performed:
        /// 
        /// * **Check 1:** Checks that there does exist a liquidity pool for the given pair of tokens.
        /// 
        /// # Arguments:
        /// 
        /// * `tokens` (Bucket) - A bucket containing the input tokens that will be swapped for other tokens.
        /// * `output_resource_address` (ResourceAddress) - The resource address of the token to receive from the swap.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A bucket of the other tokens.
        pub fn swap_exact_tokens_for_tokens(
            &mut self,
            tokens: Bucket,
            output_resource_address: ResourceAddress,
            min_amount_out: Decimal
        ) -> Bucket {
            // Checking if there does exist a liquidity pool for the given pair of tokens
            self.assert_pool_exists(tokens.resource_address(), output_resource_address, String::from("DEX Swap Exact"));

            // Sorting the two addresses passed, getting the associated liquidity pool and then performing the swap.
            let sorted_addresses: (ResourceAddress, ResourceAddress) = sort_addresses(
                tokens.resource_address(), 
                output_resource_address
            );
            return self.liquidity_pools[&sorted_addresses].swap_exact_tokens_for_tokens(tokens, min_amount_out);
        }
        
        /// Swaps the input tokens for a specific amount of tokens of the desired type.
        /// 
        /// This method is used when the user wants to swap a token for a specific amount of another token. This method
        /// calculates the input amount required to get the desired output and if the amount required is provided in the
        /// tokens bucket then the swap takes place and the user gets back two buckets: a bucket of the remaining input
        /// tokens and another bucket of the swapped tokens.
        /// 
        /// This method performs a number of checks before the swap is performed:
        /// 
        /// * **Check 1:** Checks that there does exist a liquidity pool for the given pair of tokens.
        /// 
        /// # Arguments:
        /// 
        /// * `tokens` (Bucket) - A bucket containing the input tokens that will be swapped for other tokens.
        /// * `output_resource_address` (ResourceAddress) - The resource address of the token to receive from the swap.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A bucket of the other tokens.
        pub fn swap_tokens_for_exact_tokens(
            &mut self,
            tokens: Bucket,
            output_resource_address: ResourceAddress,
            output_amount: Decimal
        ) -> (Bucket, Bucket) {
            // Checking if there does exist a liquidity pool for the given pair of tokens
            self.assert_pool_exists(tokens.resource_address(), output_resource_address, String::from("DEX Swap For Exact"));

            // Sorting the two addresses passed, getting the associated liquidity pool and then performing the swap.
            let sorted_addresses: (ResourceAddress, ResourceAddress) = sort_addresses(
                tokens.resource_address(), 
                output_resource_address
            );
            return self.liquidity_pools[&sorted_addresses].swap_tokens_for_exact_tokens(tokens, output_amount);
        }
    }
}