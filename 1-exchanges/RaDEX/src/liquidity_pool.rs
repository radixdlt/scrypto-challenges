use scrypto::prelude::*;
use crate::utils::*;

blueprint!{
    /// This is a struct used to define a liquidity pool in the RaDEX decentralized exchange. A typical liquidity pool 
    /// is made up of two vaults which are used to store the two tokens being traded against one another. In addition to 
    /// that, a number of methods are implemented for this struct in order to make it easier to add, and remove 
    /// liquidity into and from the pool, swap assets, and calculate the k value in the constant market maker function 
    /// `x * y = k`.
    struct LiquidityPool {
        /// These are the vaults where the reserves of Token A and Token B will be stored. The choice of storing the two
        /// vaults in a hashmap instead of storing them in two different struct variables is made to allow for an easier
        /// and more dynamic way of manipulating and making use of the vaults inside the liquidity pool. Keep in mind 
        /// that this `vaults` hashmap will always have exactly two vaults for the two assets being traded against one 
        /// another.
        vaults: HashMap<Address, Vault>,

        /// When liquidity providers provide liquidity to the liquidity pool, they are given a number of tokens that is
        /// equivalent to the percentage ownership that they have in the liquidity pool. The tracking token is the token
        /// that the liquidity providers are given when they provide liquidity to the pool and this is the resource 
        /// definition of the token.
        tracking_token_def: ResourceDef,

        /// The tracking tokens are mutable supply tokens that may be minted and burned when liquidity is supplied or 
        /// removed from the liquidity pool. This badge is the badge that has the authority to mint and burn the tokens
        /// when need be.
        tracking_token_admin_badge: Vault,

        /// This is a decimal value between 0 and 100 which defines the amount of fees paid to the liquidity pool (and
        /// in turn the liquidity providers) when a swap is made through this liquidity pool.
        fee_to_pool: Decimal,
    }

    impl LiquidityPool {
        /// Checks if the given address belongs to this pool or not.
        /// 
        /// This method is used to check if a given resource address belongs to one of the tokens in this liquidity pool
        /// or not. A resource belongs to a liquidity pool if its address is in the addresses in the `vaults` HashMap.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (Address) - The address of the resource that we wish to check if it belongs to the pool.
        /// 
        /// # Returns:
        /// 
        /// * `bool` - A boolean of whether the address belongs to this pool or not.
        pub fn belongs_to_pool(
            &self, 
            address: Address
        ) -> bool {
            return self.vaults.contains_key(&address);
        }

        /// Asserts that the given address belongs to the pool.
        /// 
        /// This is a quick assert method that checks if a given address belongs to the pool or not. If the address does
        /// not belong to the pool, then an assertion error (panic) occurs and the message given is outputted.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (Address) - The address of the resource that we wish to check if it belongs to the pool.
        /// * `label` (String) - The label of the method that called this assert method. As an example, if the swap 
        /// method were to call this method, then the label would be `Swap` so that it's clear where the assertion error
        /// took place.
        pub fn assert_belongs(
            &self, 
            address: Address, 
            label: String
        ) {
            assert!(
                self.belongs_to_pool(address), 
                "[{}]: The provided resource address does not belong to the pool.", 
                label
            );
        }

        /// Gets the resource addresses of the tokens in this liquidity pool and returns them as a `Vec<Address>`.
        /// 
        /// # Returns:
        /// 
        /// `Vec<Address>` - A vector of the resource addresses of the tokens in this liquidity pool.
        pub fn addresses(&self) -> Vec<Address> {
            return self.vaults.keys().cloned().collect::<Vec<Address>>();
        }

        /// Gets the address of the other resource if the passed resource address belongs to the pool.
        /// 
        /// This method takes in a resource address and if this resource address belongs to the pool it returns the 
        /// address of the other token in this liquidity pool.
        /// 
        /// This method performs a number of checks before resource address is obtained:
        /// 
        /// * **Check 1:** Checks that the resource address given does indeed belong to this liquidity pool.
        /// 
        /// # Arguments
        /// 
        /// * `resource_address` (Address) - The resource address for a token from the pool.
        /// 
        /// # Returns:
        /// 
        /// * `Address` - The address of the other token in this pool.
        pub fn other_resource_address(
            &self,
            resource_address: Address
        ) -> Address {
            // Checking if the passed resource address belongs to this pool.
            self.assert_belongs(resource_address, String::from("Other Resource Address"));

            // Checking which of the addresses was provided as an argument and returning the other address.
            let addresses: Vec<Address> = self.addresses();
            return if addresses[0] == resource_address {addresses[1]} else {addresses[0]};
        }
    }
}