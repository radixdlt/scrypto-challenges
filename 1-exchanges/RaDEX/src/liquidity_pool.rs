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
}