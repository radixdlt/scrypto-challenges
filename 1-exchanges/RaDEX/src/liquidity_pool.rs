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

        /// Calculates the k in the constant market maker equation: `x * y = k`.
        /// 
        /// # Returns:
        /// 
        /// `Decimal` - A decimal value of the reserves amount of Token A and Token B multiplied by one another.
        pub fn k(&self) -> Decimal {
            let addresses: Vec<Address> = self.addresses();
            return self.vaults[&addresses[0]].amount() * self.vaults[&addresses[1]].amount()
        }

        /// Calculates the amount of output that can be given for for a given amount of input.
        /// 
        /// This method calculates the amount of output tokens that would be received for a given amount of an input
        /// token. This is calculated through the constant market maker function `x * y = k`. 
        /// 
        /// This method performs a number of checks before the calculation is done:
        /// 
        /// * **Check 1:** Checks that the provided resource address belongs to this liquidity pool.
        /// 
        /// # Arguments:
        /// 
        /// * `input_resource_address` (Address) - The resource address of the input token.
        /// * `input_amount` (Decimal) - The amount of input tokens to calculate the output for.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The output amount for the given input.
        /// 
        /// # Note:
        /// 
        /// This method is equivalent to finding `dy` in the equation `(x + rdx)(y - dy) = xy` where the symbols used
        /// mean the following:
        /// 
        /// * `x` - The amount of reserves of token x (the input token)
        /// * `y` - The amount of reserves of token y (the output token)
        /// * `dx` - The amount of input tokens
        /// * `dy` - The amount of output tokens
        /// * `r` - The fee modifier where `r = (100 - fee) / 100`
        pub fn calculate_output_amount(
            &self,
            input_resource_address: Address,
            input_amount: Decimal
        ) -> Decimal {
            // Checking if the passed resource address belongs to this pool.
            self.assert_belongs(input_resource_address, String::from("Calculate Output"));

            let x: Decimal = self.vaults[&input_resource_address].amount();
            let y: Decimal = self.vaults[&self.other_resource_address(input_resource_address)].amount();
            let dx: Decimal = input_amount;
            let r: Decimal = (dec!("100") - self.fee_to_pool) / dec!("100");

            let dy: Decimal = (dx * r * y) / ( x + r * dx );
            return dy;
        }

        /// Calculates the amount of input required to receive the specified amount of output tokens.
        /// 
        /// This method calculates the amount of input tokens that would be required to receive the specified amount of
        /// output tokens. This is calculated through the constant market maker function `x * y = k`. 
        /// 
        /// This method performs a number of checks before the calculation is done:
        /// 
        /// * **Check 1:** Checks that the provided resource address belongs to this liquidity pool.
        /// 
        /// # Arguments:
        /// 
        /// * `output_resource_address` (Address) - The resource address of the output token.
        /// * `output_amount` (Decimal) - The amount of output tokens to calculate the input for.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The input amount for the given output.
        /// 
        /// # Note:
        /// 
        /// This method is equivalent to finding `dx` in the equation `(x + rdx)(y - dy) = xy` where the symbols used
        /// mean the following:
        /// 
        /// * `x` - The amount of reserves of token x (the input token)
        /// * `y` - The amount of reserves of token y (the output token)
        /// * `dx` - The amount of input tokens
        /// * `dy` - The amount of output tokens
        /// * `r` - The fee modifier where `r = (100 - fee) / 100`
        pub fn calculate_input_amount(
            &self,
            output_resource_address: Address,
            output_amount: Decimal
        ) -> Decimal {
            // Checking if the passed resource address belongs to this pool.
            self.assert_belongs(output_resource_address, String::from("Calculate Output"));

            let x: Decimal = self.vaults[&self.other_resource_address(output_resource_address)].amount();
            let y: Decimal = self.vaults[&output_resource_address].amount();
            let dy: Decimal = output_amount;
            let r: Decimal = (dec!("100") - self.fee_to_pool) / dec!("100");

            let dx: Decimal = (dy * x) / (r * (y - dy));
            return dx;
        }
    }
}