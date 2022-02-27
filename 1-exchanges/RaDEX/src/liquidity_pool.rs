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
        pub fn belongs_to_pool(&self, address: Address) -> bool {
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
        /// * `message` (String) - The message to output if the assertion fails.
        pub fn assert_belongs(&self, address: Address, message: String) {
            assert!(self.belongs_to_pool(address), format!("{}", message));
        }
    }
}