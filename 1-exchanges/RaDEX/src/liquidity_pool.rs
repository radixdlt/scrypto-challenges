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
        /// Creates a new liquidity pool of the two token types passed to this function.
        /// 
        /// This method is used to instantiate a new liquidity pool of the two token types that were passed to this
        /// function in the two buckets. This token pair may be swapped and all swaps will have a fee of `fee` imposed
        /// on it. 
        /// 
        /// This function does a number of checks before a Liquidity Pool is created, these checks are:
        /// 
        /// * **Check 1:** Checks that `token1` and `token2` are not of the same type.
        /// * **Check 2:** Checks that both `token1` and `token2` are fungible tokens.
        /// * **Check 3:** Checks that neither of the buckets are empty.
        /// * **Check 4:** Checks that the fee is between 0 and 100.
        /// 
        /// If these checks are successful, then a new liquidity pool is created from the two buckets passed to this 
        /// function and tracking tokens are minted for the creator of this liquidity pool. Keep in mind that this 
        /// function has now way of checking if a liquidity pool of this kind exists or not. So, it is the job of the 
        /// RaDEX component to only call this function if there does not already exist a liquidity pool for the given
        /// token pair.
        /// 
        /// # Arguments: 
        /// 
        /// * `token1_bucket` (Bucket) - A bucket containing the amount of the first token used to initialize the pool.
        /// * `token2_bucket` (Bucket) - A bucket containing the amount of the second token used to initialize the pool.
        /// * `fee` (Decimal) - A decimal value of the fee imposed on all swaps from this liquidity pool. This should be
        /// a value between 0 and 100.
        /// 
        /// # Returns:
        /// 
        /// * `Component` - A LiquidityPool component of the newly created liquidity pool.
        /// * `Bucket` - A bucket containing the tracking tokens issued to the creator of the liquidity pool.
        pub fn new(
            token1: Bucket,
            token2: Bucket,
            fee_to_pool: Decimal
        ) -> (Component, Bucket) {
            // Performing the checks to see if this liquidity pool may be created or not.
            assert_ne!(
                token1.resource_address(), token2.resource_address(),
                "[Pool Creation]: Liquidity pools may only be created between two different tokens."
            );

            assert_ne!(
                token1.resource_def().resource_type(), ResourceType::NonFungible,
                "[Pool Creation]: Both assets must be fungible."
            );
            assert_ne!(
                token2.resource_def().resource_type(), ResourceType::NonFungible,
                "[Pool Creation]: Both assets must be fungible."
            );

            assert!(
                !token1.is_empty() & !token2.is_empty(), 
                "[Pool Creation]: Can't create a pool from an empty bucket."
            );

            assert!(
                (fee_to_pool >= Decimal::zero()) & (fee_to_pool <= dec!("100")), 
                "[Pool Creation]: Fee must be between 0 and 100"
            );

            // At this point, we know that the pool creation can indeed go through. 
            
            // Sorting the buckets and then creating the hashmap of the vaults from the sorted buckets
            let (bucket1, bucket2): (Bucket, Bucket) = sort_buckets(token1, token2);
            let lp_id: String = format!("{}-{}", bucket1.resource_address(), bucket2.resource_address());
            info!(
                "[Pool Creation]: Creating new pool between tokens: {}, Ratio: {}:{}", 
                lp_id, bucket1.amount(), bucket2.amount()
            );
            
            let mut vaults: HashMap<Address, Vault> = HashMap::new();
            vaults.insert(bucket1.resource_address(), Vault::with_bucket(bucket1));
            vaults.insert(bucket2.resource_address(), Vault::with_bucket(bucket2));

            // Creating the admin badge of the liquidity pool which will be given the authority to mint and burn the
            // tracking tokens issued to the liquidity providers.
            let tracking_token_admin_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Tracking Token Admin Badge")
                .metadata("symbol", "TTAB")
                .metadata("description", "This is an admin badge that has the authority to mint and burn tracking tokens")
                .metadata("lp_id", format!("{}", lp_id))
                .initial_supply_fungible(1);

            // Creating the tracking tokens and minting the amount owed to the initial liquidity provider
            let tracking_tokens: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Tracking Token")
                .metadata("symbol", "TT")
                .metadata("description", "A tracking token used to track the percentage ownership of liquidity providers over the liquidity pool")
                .metadata("lp_id", format!("{}", lp_id))
                .initial_supply_fungible(100);

            // Creating the liquidity pool component and instantiating it
            let liquidity_pool: Component = Self { 
                vaults: vaults,
                tracking_token_def: tracking_tokens.resource_def(),
                tracking_token_admin_badge: Vault::with_bucket(tracking_token_admin_badge),
                fee_to_pool: fee_to_pool,
            }.instantiate();

            return (liquidity_pool, tracking_tokens);
        }

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
            self.assert_belongs(output_resource_address, String::from("Calculate Input"));

            let x: Decimal = self.vaults[&self.other_resource_address(output_resource_address)].amount();
            let y: Decimal = self.vaults[&output_resource_address].amount();
            let dy: Decimal = output_amount;
            let r: Decimal = (dec!("100") - self.fee_to_pool) / dec!("100");

            let dx: Decimal = (dy * x) / (r * (y - dy));
            return dx;
        }

        /// Deposits a bucket of tokens into this liquidity pool.
        /// 
        /// This method determines if a given bucket of tokens belongs to the liquidity pool or not. If it's found that
        /// they belong to the pool, then this method finds the appropriate vault to store the tokens and deposits them
        /// to that vault.
        /// 
        /// This method performs a number of checks before the deposit is made:
        /// 
        /// * **Check 1:** Checks that the resource address given does indeed belong to this liquidity pool.
        /// 
        /// # Arguments:
        /// 
        /// * `bucket` (Bucket) - A buckets of the tokens to deposit into the liquidity pool
        fn deposit(
            &mut self,
            bucket: Bucket 
        ) {
            // Checking if the passed resource address belongs to this pool.
            self.assert_belongs(bucket.resource_address(), String::from("Deposit"));

            self.vaults.get_mut(&bucket.resource_address()).unwrap().put(bucket);
        }

        /// Withdraws tokens from the liquidity pool.
        /// 
        /// This method is used to withdraw a specific amount of tokens from the liquidity pool. 
        /// 
        /// This method performs a number of checks before the withdraw is made:
        /// 
        /// * **Check 1:** Checks that the resource address given does indeed belong to this liquidity pool.
        /// * **Check 2:** Checks that the there is enough liquidity to perform the withdraw.
        /// 
        /// # Arguments:
        /// 
        /// * `resource_address` (Address) - The address of the resource to withdraw from the liquidity pool.
        /// * `amount` (Decimal) - The amount of tokens to withdraw from the liquidity pool.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A bucket of the withdrawn tokens.
        fn withdraw(
            &mut self,
            resource_address: Address,
            amount: Decimal
        ) -> Bucket {
            // Performing the checks to ensure tha the withdraw can actually go through
            self.assert_belongs(resource_address, String::from("Withdraw"));
            
            // Getting the vault of that resource and checking if there is enough liquidity to perform the withdraw.
            let vault: &mut Vault = self.vaults.get_mut(&resource_address).unwrap();
            assert!(
                vault.amount() >= amount,
                "[Withdraw]: Not enough liquidity available for the withdraw."
            );

            return vault.take(amount);
        }

        /// Adds liquidity to this liquidity pool in exchange for liquidity provider tracking tokens.
        /// 
        /// This method calculates the appropriate amount of liquidity that may be added to the liquidity pool from the
        /// two token buckets provided in this method call. This method then adds the liquidity and issues tracking 
        /// tokens to the liquidity provider to keep track of their percentage ownership over the pool. 
        /// 
        /// This method performs a number of checks before liquidity is added to the pool:
        /// 
        /// * **Check 1:** Checks that the buckets passed are of tokens that belong to this liquidity pool.
        /// * **Check 2:** Checks that the buckets passed are not empty.
        /// 
        /// From the perspective of adding liquidity, these are all of the checks that need to be done. The RaDEX 
        /// component does not need to perform any additional checks when liquidity is being added.
        /// 
        /// # Arguments:
        /// 
        /// * `token1_bucket` (Bucket) - A bucket containing the amount of the first token to add to the pool.
        /// * `token2_bucket` (Bucket) - A bucket containing the amount of the second token to add to the pool.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A bucket of the remaining tokens of the `token1` type.
        /// * `Bucket` - A bucket of the remaining tokens of the `token2` type.
        /// * `Bucket` - A bucket of the tracking tokens issued to the liquidity provider.
        /// 
        /// # Note:
        /// 
        /// This method uses the ratio of the tokens in the reserve to the ratio of the supplied tokens to determine the
        /// appropriate amount of tokens which need to be supplied. To better explain it, let's use some symbols to make
        /// the ratios a little bit clearer. Say that `m` and `n` are the tokens reserves of the two tokens stored in 
        /// the vaults respectively. Say that `dm` and `dn` are positive non-zero `Decimal` numbers of the amount of 
        /// liquidity which the provider wishes to add to the liquidity pool. If `(m / n)/(dm / dn) = 1` then all of the
        /// tokens sent in the transactions will be added to the liquidity. However, what about the other cases where 
        /// this is not equal to one? We could say that we have three cases in total:
        /// 
        /// * `(m / n) = (dm / dn)` - There is no excess of tokens and all of the tokens given to the method may be 
        /// added to the liquidity pool.
        /// * `(m / n) < (dm / dn)` - In this case, there would be an excess of `dm` meaning that `dn` would be consumed
        /// fully while `dm` would be consumed partially.
        /// * `(m / n) > (dm / dn)` - In this case, there would be an excess of `dn` meaning that `dm` would be consumed
        /// fully while `dn` would be consumed partially.
        /// 
        /// This method takes into account all three of these cases and appropriately accounts for them.
        pub fn add_liquidity(
            &mut self,
            token1: Bucket,
            token2: Bucket,
        ) -> (Bucket, Bucket, Bucket) {
            // Checking if the tokens belong to this liquidity pool.
            self.assert_belongs(token1.resource_address(), String::from("Add Liquidity"));
            self.assert_belongs(token2.resource_address(), String::from("Add Liquidity"));

            // Checking that the buckets passed are not empty
            assert!(token1.is_empty(), "[Add Liquidity]: Can not add liquidity from an empty bucket");
            assert!(token2.is_empty(), "[Add Liquidity]: Can not add liquidity from an empty bucket");

            // Sorting out the two buckets passed and getting the values of `dm` and `dn`.
            let (mut bucket1, mut bucket2): (Bucket, Bucket) = sort_buckets(token1, token2);
            let dm: Decimal = bucket1.amount();
            let dn: Decimal = bucket2.amount();

            // Getting the values of m and n from the liquidity pool vaults
            let m: Decimal = self.vaults[&bucket1.resource_address()].amount();
            let n: Decimal = self.vaults[&bucket2.resource_address()].amount();

            // Computing the amount of tokens to deposit into the liquidity pool from each one of the buckets passed
            let (amount1, amount2): (Decimal, Decimal) = if (m / n) == (dm / dn) { // Case 1
                (dm, dn)
            } else if (m / n) < (dm / dn) { // Case 2
                (dn * m / n, dn)
            } else { // Case 3
                (dm, dm * n / m)
            };

            // Depositing the amount of tokens calculated into the liquidity pool
            self.deposit(bucket1.take(amount1));
            self.deposit(bucket2.take(amount2));

            // Computing the amount of tracking tokens that the liquidity provider is owed and minting them
            let tracking_amount: Decimal = dm * self.tracking_token_def.total_supply() / m;
            let tracking_tokens: Bucket = self.tracking_token_admin_badge.authorize(|x| {
                self.tracking_token_def.mint(tracking_amount, x)
            });

            // Returning the remaining tokens from `token1`, `token2`, and the tracking tokens
            return (bucket1, bucket2, tracking_tokens);
        }
    }
}