//! # Router Blueprint
//!
//! Implements a router to all the existing pools.
//!
//! # Functions & Methods
//!
//! ### Function
//! - [new](RouterComponent::new) - Instantiates and globalizes a new [`RouterComponent`] and returns its address and an admin badge.
//! and returns it.
//!
//! ### Methods
//! - [create_pool](RouterComponent::create_pool) - Creates a new stablecoin/token pool.
//! - [add_liquidity](RouterComponent::add_liquidity) - Adds liquidity to an existing pool to the given rate.
//! - [add_liquidity_at_step](RouterComponent::add_liquidity_at_step) - Adds liquidity to an existing pool at the given step.
//! - [add_liquidity_at_steps](RouterComponent::add_liquidity_at_steps) - Adds liquidity to an existing pool at the given steps.
//! - [remove_liquidity_at_step](RouterComponent::remove_liquidity_at_step) - Removes liquidity from an existing pool at a given step.
//! - [remove_liquidity_at_steps](RouterComponent::remove_liquidity_at_steps) - Removes liquidity from an existing pool at given steps.
//! - [remove_liquidity_at_rate](RouterComponent::remove_liquidity_at_rate) - Removes liquidity from an existing pool at a given rate.
//! - [remove_all_liquidity](RouterComponent::remove_all_liquidity) - Removes all liquidity from the supplied [`Position`]s NFR and burns them.
//! - [claim_fees](RouterComponent::claim_fees) - Claim fees associated to the supplied proof of [`Position`]s.
//! - [swap](RouterComponent::swap) - Swaps tokens.
//! - [claim_protocol_fees](RouterComponent::claim_protocol_fees) - Claims protocol fees.
//! - [get_pool_state](RouterComponent::get_pool_state) - Returns the full state of the blueprint.
//! - [step_at_rate](RouterComponent::step_at_rate) - Returns the step of a pool associated to a given rate

use scrypto::blueprint;

#[blueprint]
mod router {
    use crate::pool::PoolComponent;
    use crate::position::Position;

    pub struct Router {
        /// Address of the stablecoin used in the pairs of the pools.
        stablecoin_address: ResourceAddress,

        /// Pools registered by the router
        pools: HashMap<ResourceAddress, PoolComponent>,

        /// Vault used to mint [`Position`]s
        position_minter: Vault,

        /// ResourceAddress of the [`Position`] NFR.
        position_address: ResourceAddress,

        /// Id of the next position to be minted
        position_id: u64,

        /// Address of the admin badge controlling the Router and its pools
        admin_badge: ResourceAddress,
    }

    impl Router {
        /// Instantiates and globalizes a new [`RouterComponent`] and returns its address and an admin badge.
        ///
        /// # Arguments
        /// * `admin_badge` - ResourceAddress of the admin badge controlling the router.
        /// * `stablecoin` - ResourceAddress of the stablecoin to be used by the pools.
        pub fn new(
            admin_badge: ResourceAddress,
            stablecoin: ResourceAddress,
        ) -> (ComponentAddress, ResourceAddress) {
            // Creates the position minter
            let position_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(Decimal::ONE);

            // Creates the NFR Position address
            let position_resource = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric Position")
                .mintable(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .updateable_non_fungible_data(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .create_with_no_initial_supply();

            // Defines the access rules for the methods of the blueprint. For security reasons,
            // the default access rule is set to require the admin badge.
            let router_rules = AccessRules::new()
                .method("add_liquidity", AccessRule::AllowAll, AccessRule::DenyAll)
                .method(
                    "add_liquidity_at_step",
                    AccessRule::AllowAll,
                    AccessRule::DenyAll,
                )
                .method(
                    "add_liquidity_at_steps",
                    AccessRule::AllowAll,
                    AccessRule::DenyAll,
                )
                .method(
                    "remove_liquidity_at_rate",
                    AccessRule::AllowAll,
                    AccessRule::DenyAll,
                )
                .method(
                    "remove_liquidity_at_step",
                    AccessRule::AllowAll,
                    AccessRule::DenyAll,
                )
                .method(
                    "remove_liquidity_at_steps",
                    AccessRule::AllowAll,
                    AccessRule::DenyAll,
                )
                .method(
                    "remove_all_liquidity",
                    AccessRule::AllowAll,
                    AccessRule::DenyAll,
                )
                .method("claim_fees", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("swap", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("get_pool_state", AccessRule::AllowAll, AccessRule::DenyAll)
                .method("step_at_rate", AccessRule::AllowAll, AccessRule::DenyAll)
                .default(rule!(require(admin_badge)), AccessRule::DenyAll);

            let mut component = Self {
                stablecoin_address: stablecoin,
                pools: HashMap::new(),
                position_minter: Vault::with_bucket(position_minter),
                position_address: position_resource.clone(),
                position_id: 0,
                admin_badge: admin_badge,
            }
            .instantiate();

            component.add_access_check(router_rules);
            (component.globalize(), position_resource)
        }

        /// Creates a new stablecoin/token pool.
        ///
        /// # Access Rule
        /// Can only be called by the owner of the admin badge
        ///
        /// # Arguments
        /// * `token` - ResourceAddress of the new token to create a pool for.
        /// * `initial_rate` - Initial exchange rate of the pool.
        /// * `min_rate` -  Minimum exchange rate of the pool.
        /// * `max_rate` - Maximum exchange rate of the pool.
        pub fn create_pool(
            &mut self,
            token: ResourceAddress,
            initial_rate: Decimal,
            min_rate: Decimal,
            max_rate: Decimal,
        ) {
            assert!(
                token != self.stablecoin_address,
                "Two pools cannot trade the same token"
            );

            assert!(
                self.pools.get(&token).is_none(),
                "A pool trading these tokens already exists"
            );

            let pool = PoolComponent::new(
                self.stablecoin_address,
                token.clone(),
                initial_rate,
                min_rate,
                max_rate,
            );
            self.pools.insert(token, pool);
        }

        /// Adds liquidity to an existing pool at a given rate.
        ///
        /// # Arguments
        /// * `bucket_a` - Bucket containing the first token to be added as liquidity
        /// * `bucket_b` - Bucket containing the second token to be added as liquidity
        /// * `rate` - Rate at which to add the liquidity
        /// * `opt_position_proof` - Optional Proof of an existing [`Position`] NFR
        pub fn add_liquidity(
            &mut self,
            bucket_a: Bucket,
            bucket_b: Bucket,
            rate: Decimal,
            opt_position_proof: Option<Proof>,
        ) -> (Bucket, Bucket, Option<Bucket>) {
            let (bucket_stable, bucket_other) = self.sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_other.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    // If the user supplied a Proof, check that it is indeed a proof of a single position NFR
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();

                    // Extract the data from the Position NFR
                    let data = self.get_position_data(&position_nfr);

                    let (ret_stable, ret_other, new_data) =
                        pool.add_liquidity(bucket_stable, bucket_other, rate, data);
                    self.update_position(position_nfr, new_data);

                    (ret_stable, ret_other, None)
                }
                None => {
                    // If the user did not supply a Proof, create one and add liquidity
                    let empty_pos = Position::from(bucket_other.resource_address());
                    let (ret_stable, ret_other, new_data) =
                        pool.add_liquidity(bucket_stable, bucket_other, rate, empty_pos);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data,
                        )
                    });
                    self.position_id += 1;
                    (ret_stable, ret_other, Some(bucket_pos))
                }
            }
        }

        /// Adds liquidity to an existing pool at a given step.
        ///
        /// # Arguments
        /// * `bucket_a` - Bucket containing the first token to be added as liquidity
        /// * `bucket_b` - Bucket containing the second token to be added as liquidity
        /// * `step` - Step to which to add liquidity
        /// * `opt_position_proof` - Optional Proof of an existing [`Position`] NFR
        pub fn add_liquidity_at_step(
            &mut self,
            bucket_a: Bucket,
            bucket_b: Bucket,
            step: u16,
            opt_position_proof: Option<Proof>,
        ) -> (Bucket, Bucket, Option<Bucket>) {
            let (bucket_stable, bucket_other) = self.sort_buckets(bucket_a, bucket_b);
            let pool = self.get_pool(bucket_other.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    // If the user supplied a Proof, check that it is indeed a proof of a single position NFR
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();

                    // Extract the data from the Position NFR
                    let data = self.get_position_data(&position_nfr);

                    let (ret_stable, ret_other, new_data) =
                        pool.add_liquidity_at_step(bucket_stable, bucket_other, step, data);
                    self.update_position(position_nfr, new_data);

                    (ret_stable, ret_other, None)
                }
                None => {
                    // If the user did not supply a Proof, create one and add liquidity
                    let empty_pos = Position::from(bucket_other.resource_address());
                    let (ret_stable, ret_other, new_data) =
                        pool.add_liquidity_at_step(bucket_stable, bucket_other, step, empty_pos);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data,
                        )
                    });
                    self.position_id += 1;
                    (ret_stable, ret_other, Some(bucket_pos))
                }
            }
        }

        /// Adds liquidity to an existing pool at given steps.
        ///
        /// # Arguments
        /// * `bucket_a` - Bucket containing the first token to be added as liquidity
        /// * `bucket_b` - Bucket containing the second token to be added as liquidity
        /// * `steps` - List of steps and amounts of tokens to add to each steps
        /// * `opt_position_proof` - Optional Proof of an existing [`Position`] NFR
        pub fn add_liquidity_at_steps(
            &mut self,
            bucket_stable: Bucket,
            bucket_other: Bucket,
            steps: Vec<(u16, Decimal, Decimal)>,
            opt_position_proof: Option<Proof>,
        ) -> (Bucket, Bucket, Option<Bucket>) {
            let pool = self.get_pool(bucket_other.resource_address());

            match opt_position_proof {
                Some(position_proof) => {
                    // If the user supplied a Proof, check that it is indeed a proof of a single position NFR
                    let valid_proof = self.check_single_position_proof(position_proof);
                    let position_nfr = valid_proof.non_fungible::<Position>();

                    // Extract the data from the Position NFR
                    let data = self.get_position_data(&position_nfr);

                    let (ret_stable, ret_other, new_data) =
                        pool.add_liquidity_at_steps(bucket_stable, bucket_other, steps, data);
                    self.update_position(position_nfr, new_data);
                    (ret_stable, ret_other, None)
                }
                None => {
                    // If the user did not supply a Proof, create one and add liquidity
                    let empty_pos = Position::from(bucket_other.resource_address());
                    let (ret_stable, ret_other, new_data) =
                        pool.add_liquidity_at_steps(bucket_stable, bucket_other, steps, empty_pos);

                    let bucket_pos = self.position_minter.authorize(|| {
                        borrow_resource_manager!(self.position_address).mint_non_fungible(
                            &NonFungibleLocalId::Integer(self.position_id.into()),
                            new_data,
                        )
                    });
                    self.position_id += 1;
                    (ret_stable, ret_other, Some(bucket_pos))
                }
            }
        }

        /// Removes liquidity from an existing pool at a given step.
        ///
        /// # Arguments
        /// * `position_proof` - Proof of an existing [`Position`] NFR
        pub fn remove_liquidity_at_step(
            &mut self,
            position_proof: Proof,
            step: u16,
        ) -> (Bucket, Bucket) {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token);
            let (ret_stable, ret_other, new_data) = pool.remove_liquidity_at_step(step, data);
            self.update_position(position_nfr, new_data);
            (ret_stable, ret_other)
        }

        /// Removes liquidity from an existing pool at a given step range.
        ///
        /// # Arguments
        /// * `position_proof` - Proof of an existing [`Position`] NFR
        pub fn remove_liquidity_at_steps(
            &mut self,
            position_proof: Proof,
            start_step: u16,
            stop_step: u16,
        ) -> (Bucket, Bucket) {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token);
            let (ret_stable, ret_other, new_data) =
                pool.remove_liquidity_at_steps(start_step, stop_step, data);
            self.update_position(position_nfr, new_data);
            (ret_stable, ret_other)
        }

        /// Removes liquidity from an existing pool at a given exchange rate.
        ///
        /// # Arguments
        /// * `position_proof` - Proof of an existing [`Position`] NFR
        pub fn remove_liquidity_at_rate(
            &mut self,
            position_proof: Proof,
            rate: Decimal,
        ) -> (Bucket, Bucket) {
            let valid_proof = self.check_single_position_proof(position_proof);
            let position_nfr = valid_proof.non_fungible::<Position>();
            let data = self.get_position_data(&position_nfr);

            let pool = self.get_pool(data.token);
            let (ret_stable, ret_other, new_data) = pool.remove_liquidity_at_rate(rate, data);
            self.update_position(position_nfr, new_data);
            (ret_stable, ret_other)
        }

        /// Removes all liquidity from the supplied [`Position`]s NFR and burns them.
        ///
        /// # Arguments
        /// * `positions_bucket` - Proof of an existing [`Position`] NFR
        pub fn remove_all_liquidity(&mut self, positions_bucket: Bucket) -> Vec<Bucket> {
            assert!(positions_bucket.resource_address() == self.position_address);

            let mut buckets: Vec<Bucket> = Vec::new();
            let mut stable_bucket = Bucket::new(self.stablecoin_address);
            for position_nfr in positions_bucket.non_fungibles::<Position>() {
                let data = self.get_position_data(&position_nfr);
                let pool = self.get_pool(data.token);
                let (ret_stable, ret_other) = pool.remove_all_liquidity(data);

                stable_bucket.put(ret_stable);
                buckets.push(ret_other);
            }
            buckets.push(stable_bucket);
            self.position_minter.authorize(|| positions_bucket.burn());
            buckets
        }

        /// Claim fees associated to the supplied proof of [`Position`]s.
        ///
        /// # Arguments
        /// * `positions_proof` - Proof of existing [`Position`]s NFR
        pub fn claim_fees(&mut self, positions_proof: Proof) -> Vec<Bucket> {
            let valid_proof = self.check_multiple_position_proof(positions_proof);

            let mut buckets: Vec<Bucket> = Vec::new();
            let mut stable_bucket = Bucket::new(self.stablecoin_address);
            for position_nfr in valid_proof.non_fungibles::<Position>() {
                let data = self.get_position_data(&position_nfr);
                let pool = self.get_pool(data.token);
                let (ret_stable, ret_other, new_data) = pool.claim_fees(data);
                self.update_position(position_nfr, new_data);

                stable_bucket.put(ret_stable);
                buckets.push(ret_other);
            }
            buckets.push(stable_bucket);
            buckets
        }

        /// Swaps tokens.
        ///
        /// # Arguments
        /// `input` - tokens to be swapped
        /// `output` - tokens to receive
        pub fn swap(&mut self, input: Bucket, output: ResourceAddress) -> (Bucket, Bucket) {
            let pool;
            if output == self.stablecoin_address {
                pool = self.get_pool(input.resource_address())
            } else {
                pool = self.get_pool(output)
            }
            pool.swap(input)
        }

        /// Claims protocol fees.
        ///
        /// # Access Rule
        /// Can only be called by the owner of the admin badge
        pub fn claim_protocol_fees(&mut self) -> Vec<Bucket> {
            let mut buckets: Vec<Bucket> = Vec::new();
            let mut stable_bucket = Bucket::new(self.stablecoin_address);

            for (_, pool) in &self.pools {
                let (stable_tmp, other_bucket) = pool.claim_protocol_fees();
                buckets.push(other_bucket);
                stable_bucket.put(stable_tmp);
            }
            buckets.push(stable_bucket);
            buckets
        }

        /// Makes a new oracle observations if last observations happened more than 20 seconds ago
        pub fn new_observation(&mut self, token: ResourceAddress) {
            let pool = self.get_pool(token);
            pool.new_observation();
        }

        /// Returns Time-wieghted average price of a given token since a given time
        pub fn get_twap_since(&self, token: ResourceAddress, timestamp: i64) -> Decimal {
            let pool = self.get_pool(token);
            pool.get_twap_since(timestamp)
        }

        /// Return the state of the given pool.
        ///
        /// # Arguments
        /// `token` - other token traded by the pool to get the state of
        pub fn get_pool_state(
            &mut self,
            token: ResourceAddress,
        ) -> (
            Decimal,
            u16,
            Decimal,
            (Decimal, Decimal),
            Vec<(u16, Vec<Decimal>)>,
        ) {
            let pool = self.get_pool(token);
            pool.get_state()
        }

        /// Returns the step of a pool associated to a given rate.
        ///
        /// # Arguments
        /// `token` - ResourceAddress of the other token traded by the pool
        /// `rate` - Rate for which to compute the associated step
        pub fn step_at_rate(&self, token: ResourceAddress, rate: Decimal) -> u16 {
            let pool = self.get_pool(token);
            pool.step_at_rate(rate)
        }

        /// Internal method that returns the pool trading the pair stablecoin/token.
        #[inline]
        fn get_pool(&self, token: ResourceAddress) -> &PoolComponent {
            match self.pools.get(&token) {
                None => {
                    panic!("There is no pool trading this pair")
                }
                Some(pool) => pool,
            }
        }

        /// Internal method that checks that a given proof is a proof of a single [`Position`] NFR.
        #[inline]
        fn check_single_position_proof(&self, position_proof: Proof) -> ValidatedProof {
            position_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.position_address,
                    Decimal::ONE,
                ))
                .expect("The provided proof is invalid")
        }

        /// Internal method that checks that a given proof is a proof of [`Position`](s) NFR.
        #[inline]
        fn check_multiple_position_proof(&self, positions_proof: Proof) -> ValidatedProof {
            positions_proof
                .validate_proof(ProofValidationMode::ValidateResourceAddress(
                    self.position_address,
                ))
                .expect("The provided proof is invalid")
        }

        /// Internal method that returns the data associated to a [`Position`] NFR.
        #[inline]
        fn get_position_data(&self, position_nfr: &NonFungible<Position>) -> Position {
            borrow_resource_manager!(self.position_address)
                .get_non_fungible_data::<Position>(position_nfr.local_id())
        }

        /// Internal method that updates the data of a [`Position`] NFR.
        #[inline]
        fn update_position(&self, position_nfr: NonFungible<Position>, new_data: Position) {
            self.position_minter
                .authorize(|| position_nfr.update_data(new_data));
        }

        /// Internal method that sorts two buckets by putting the stablecoin buckets in the first
        /// position of the pair.
        #[inline]
        fn sort_buckets(&self, bucket_a: Bucket, bucket_b: Bucket) -> (Bucket, Bucket) {
            if bucket_a.resource_address() == self.stablecoin_address {
                (bucket_a, bucket_b)
            } else {
                (bucket_b, bucket_a)
            }
        }
    }
}
