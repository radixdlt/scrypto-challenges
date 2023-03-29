use scrypto::prelude::*;
use std::collections::hash_map::Entry;

use crate::btree_set_ext;
use crate::pool_math;
use crate::tick_math;

#[blueprint]
mod pool_blueprint {

    struct Pool {
        vault0: Vault,
        vault1: Vault,
        live_liq: Decimal,
        tick: i32,
        sqrt_price: Decimal,
        fee: Decimal,
        fee_global0: Decimal,
        fee_global1: Decimal,
        pos_nft_addr: ResourceAddress,
        positions: HashMap<NonFungibleLocalId, Position>,
        used_ticks: BTreeSet<i32>,
        tick_states: HashMap<i32, TickState>,
        pos_nft_minter_badge: Vault,
        admin_badge_addr: ResourceAddress,
    }

    impl Pool {
        /**
         * Creates a new concentrated liquidity pool. Where:
         * - resource0_addr, resource1_addr = fungible tokens address.
         * - fee = pool fee, a percentage of the amount that is swapped, fee >= 0 and fee <= 1
         * - sqrt_price = square root of the price token0 vs token1 when the pool is created.
         * - admin_badge_addr = a badge that allows the pool creator to destroy the pool if conditions are met
         */
        pub fn new(
            resource0_addr: ResourceAddress,
            resource1_addr: ResourceAddress,
            fee: Decimal,
            sqrt_price: Decimal,
            admin_badge_addr: ResourceAddress,
        ) -> ComponentAddress {
            assert!(sqrt_price > Decimal::zero(), "Invalid sqrt price, should be positive.");
            assert!(
                fee >= Decimal::zero() && fee <= Decimal::one(),
                "Invalid fee, should be 0 <= fee <= 1"
            );
            assert!(resource0_addr != resource1_addr, "Pool resources should be different.");
            Pool::validate_resource_type_is_fungible(resource0_addr);
            Pool::validate_resource_type_is_fungible(resource1_addr);

            let pos_nft_minter_badge = ResourceBuilder::new_fungible().mint_initial_supply(1);
            let pos_nft_addr = ResourceBuilder::new_uuid_non_fungible()
                .mintable(rule!(require(pos_nft_minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(pos_nft_minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(pos_nft_minter_badge.resource_address())), LOCKED)
                .create_with_no_initial_supply();

            let auth: AccessRules = AccessRules::new()
                .method("destroy", rule!(require(admin_badge_addr)), AccessRule::DenyAll)
                .default(AccessRule::AllowAll, AccessRule::DenyAll);

            let mut component = Self {
                vault0: Vault::new(resource0_addr),
                vault1: Vault::new(resource1_addr),
                live_liq: Decimal::zero(),
                tick: tick_math::tick_at_sqrt_price(sqrt_price),
                sqrt_price,
                fee,
                fee_global0: Decimal::zero(),
                fee_global1: Decimal::zero(),
                pos_nft_addr,
                positions: HashMap::new(),
                used_ticks: BTreeSet::new(),
                tick_states: HashMap::new(),
                pos_nft_minter_badge: Vault::with_bucket(pos_nft_minter_badge),
                admin_badge_addr,
            }
            .instantiate();
            component.add_access_check(auth);
            component.globalize()
        }

        /**
         * Validates that the pool resource types are fungibles.
         */
        fn validate_resource_type_is_fungible(resource_addr: ResourceAddress) {
            let token_resource_type = borrow_resource_manager!(resource_addr).resource_type();
            assert!(
                matches!(token_resource_type, ResourceType::Fungible { .. }),
                "Pool resource0,1 must be of fungible type."
            );
        }

        /**
         * Adds a new liquidity position in range [low_tick, high_tick] using the resource amounts in bucket0 and bucket1. Depending on the current price, amount0,1 might not be used entirely.
         *
         * Returns a NFT representing the position with the newly created liquidity and the remainders amount0,1.
         */
        pub fn add_position(
            &mut self,
            mut bucket0: Bucket,
            mut bucket1: Bucket,
            low_tick: i32,
            high_tick: i32,
        ) -> (Bucket, Bucket, Bucket) {
            debug!("### Adding a new position...");
            debug!("### Bucket0 resource={:?}", bucket0.resource_address());
            debug!("### Bucket0={:?}", bucket0.amount());
            debug!("### Bucket1 resource={:?}", bucket1.resource_address());
            debug!("### Bucket1={:?}", bucket1.amount());
            debug!("### Low tick={:?}", low_tick);
            debug!("### High tick={:?}", high_tick);

            //validate ticks
            assert!(
                low_tick < high_tick,
                "Lower tick must be less than upper tick. Add position op aborted."
            );

            // validate the passed resources
            self.validate_resources(bucket0.resource_address(), bucket1.resource_address());

            self.log_state("### Internal state before adding the new position");

            //compute the liquidty and amount0,1 required for this liquidity, depending on the current tick
            let (liq, required_amount0, required_amount1) = pool_math::compute_range_liq_given_amounts(
                bucket0.amount(),
                bucket1.amount(),
                self.sqrt_price,
                tick_math::sqrt_price_at_tick(low_tick),
                tick_math::sqrt_price_at_tick(high_tick),
            );

            //update live liq
            self.update_live_liq(liq, low_tick, high_tick);

            //update tick states
            self.tick_states.entry(low_tick).or_insert(TickState::new(low_tick)).modify_liq(
                liq,
                false,
                self.tick,
                self.fee_global0,
                self.fee_global1,
            );
            self.tick_states.entry(high_tick).or_insert(TickState::new(high_tick)).modify_liq(
                liq,
                true,
                self.tick,
                self.fee_global0,
                self.fee_global1,
            );

            //mark ticks as used
            self.used_ticks.insert(low_tick);
            self.used_ticks.insert(high_tick);

            //take the required amounts in the pool vaults
            self.vault0.put(bucket0.take(required_amount0));
            self.vault1.put(bucket1.take(required_amount1));

            //mint the nft corresponding to the position with the liqudity on it
            let pos_res_mg: &mut ResourceManager = borrow_resource_manager!(self.pos_nft_addr);
            let pos_nft = self
                .pos_nft_minter_badge
                .authorize(|| pos_res_mg.mint_uuid_non_fungible(PositionNFTData { liq }));

            //save the new position
            self.positions.insert(
                (pos_nft.non_fungible_local_id()).clone(),
                Position::new(liq, low_tick, high_tick, Decimal::zero(), Decimal::zero()),
            );

            self.log_state("### Internal state after adding the new position");

            debug!("### Position id={:?}", pos_nft.non_fungible_local_id());
            debug!("### Position NFT liq={:?}", liq);
            debug!("### Bucket0={:?}", bucket0.amount());
            debug!("### Bucket1={:?}", bucket1.amount());
            debug!("### Position added.");

            (pos_nft, bucket0, bucket1)
        }

        /**
         * Adds more liquidity to an already existing position. The new liquidity is computed from the provided amounts of tokens0,1 depending on the current price. Any fees already accumulated by the position are collected and put to work as new liquidity.
         *
         * The caller must present a proof issued from the NFT that identifies the already existing, active position.
         *
         * Returns the remainder of the amount0,1, if any.
         */
        pub fn add_liq(&mut self, mut bucket0: Bucket, mut bucket1: Bucket, auth: Proof) -> (Bucket, Bucket) {
            debug!("### Adding liquidity...");
            //validate the resources sent in
            self.validate_resources(bucket0.resource_address(), bucket1.resource_address());

            //add liq
            let (to_deduct_amount0, to_deduct_amount1) = self.add_liq_internal(bucket0.amount(), bucket1.amount(), auth);

            // take the required amounts in the pool vaults
            self.vault0.put(bucket0.take(to_deduct_amount0));
            self.vault1.put(bucket1.take(to_deduct_amount1));

            debug!("### Liquidity added.");
            debug!("### Bucket0={:?}", bucket0.amount());
            debug!("### Bucket1={:?}", bucket1.amount());

            (bucket0, bucket1)
        }

        /**
         * Add the accumulated fees on the given position to the position liquidity.
         */
        pub fn add_accumulated_fees_to_liq(&mut self, auth: Proof) {
            debug!("### Adding collected fees to liquidity...");
            self.add_liq_internal(Decimal::zero(), Decimal::zero(), auth);
            debug!("### Accumulated fees added to liquidity.");
        }

        /**
         * Remove all the liquidity from the position identified by the provided proof.
         *
         * Return the amount0,1 corresponding to the liquidity removed. Amount0,1 contain also the fees already accumulated by the position.
         */
        pub fn remove_pos(&mut self, auth: Proof) -> (Bucket, Bucket) {
            let valid_auth: ValidatedProof = self.validate_auth(auth);
            let pos_nft: NonFungible<PositionNFTData> = valid_auth.non_fungible();
            self.remove_liq_internal(pos_nft.data().liq, valid_auth)
        }

        /**
         * Collect the fees accumulated for the position identified by the NFT in the auth
         */
        pub fn collect_fees(&mut self, auth: Proof) -> (Bucket, Bucket) {
            debug!("### Collecting fees...");
            self.remove_liq_internal(Decimal::zero(), self.validate_auth(auth))
        }

        /**
         * Swaps the provided amount0,1 for the opposite token. Internally the pool modifies the tokens price and live liquidity.
         *
         * Returns the swapped amount1,0 and the remainder of the provided amount0,1
         */
        pub fn swap(&mut self, bucket: Bucket) -> (Bucket, Bucket) {
            debug!("### Swapping...");

            //validate the resource to swap
            assert!(
                (bucket.resource_address() == self.vault0.resource_address()
                    || bucket.resource_address() == self.vault1.resource_address()),
                "Wrong resource type sent. Swap op aborted."
            );

            //depending on the resource type sent swap resource0 or resource1
            let (output_bucket, remainder_bucket) = if bucket.resource_address() == self.vault0.resource_address() {
                self.swap_internal(bucket, true)
            } else {
                self.swap_internal(bucket, false)
            };

            debug!("Swapping done.");

            (output_bucket, remainder_bucket)
        }

        /**
         * Destroy the pool if no more positions
         */
        pub fn destroy(&self) {
            //todo
        }

        /**
         * Validate the type and quantity of the provided proof match the type issued by the pool
         */
        fn validate_auth(&self, auth: Proof) -> ValidatedProof {
            auth.validate_proof(ProofValidationMode::ValidateContainsAmount(self.pos_nft_addr, Decimal::one()))
                .expect("The provided badge is either of an invalid resource address or amount.")
        }

        /**
         * Validate the provided pos_id belongs to an active position in this pool
         */
        fn validate_pos(&self, pos_id: &NonFungibleLocalId) {
            assert!(
                self.positions.contains_key(pos_id),
                "No position exists for given position id. Op aborted."
            );
        }

        /**
         * Validate the resources in the bucket are of the same types as the pool resources
         */
        fn validate_resources(&self, resource0: ResourceAddress, resource1: ResourceAddress) {
            assert!(
                resource0 == self.vault0.resource_address() && resource1 == self.vault1.resource_address(),
                "Wrong resource types passed to the pool. Op aborted."
            );
        }

        /**
         * Adds the given amount0,1 to the liquidty of the given position, together with the fees accumulated by the given position
         */
        fn add_liq_internal(&mut self, amount0: Decimal, amount1: Decimal, auth: Proof) -> (Decimal, Decimal) {
            debug!("### Adding liquidity internally...");
            debug!("### Amount0={:?}", amount0);
            debug!("### Amount1={:?}", amount1);

            let valid_auth: ValidatedProof = self.validate_auth(auth);

            self.log_state("### Internal state before adding the liquidity");

            //identify position and validate it
            let pos: NonFungible<PositionNFTData> = valid_auth.non_fungible();
            let pos_id = pos.local_id();
            self.validate_pos(pos_id);

            debug!("### Pos_id={:?}", pos_id);

            let pos = self.positions.get_mut(&pos_id).unwrap();
            let (low_tick, high_tick) = (pos.low_tick, pos.high_tick);

            // compute the fees already accumulated by the range and the position
            let (range_fee0, range_fee1) = pool_math::compute_range_fees(
                self.tick,
                self.fee_global0,
                self.fee_global1,
                self.tick_states.get(&low_tick).unwrap(),
                self.tick_states.get(&high_tick).unwrap(),
            );
            let (pos_fee0, pos_fee1) = pool_math::compute_pos_fees(pos.liq, pos.range_fee0, pos.range_fee1, range_fee0, range_fee1);

            debug!("### Range_fee0={:?}", range_fee0);
            debug!("### Range_fee1={:?}", range_fee1);
            debug!("### Pos_fee0={:?}", pos_fee0);
            debug!("### Pos_fee1={:?}", pos_fee1);

            // compute the new position liquidity and the required amount0,1, including also the fees in the liquidity
            let (liq, required_amount0, required_amount1) = pool_math::compute_range_liq_given_amounts(
                amount0 + pos_fee0,
                amount1 + pos_fee1,
                self.sqrt_price,
                tick_math::sqrt_price_at_tick(low_tick),
                tick_math::sqrt_price_at_tick(high_tick),
            );

            debug!("### Liq={:?}", liq);
            debug!("### Required_amount0={:?}", required_amount0);
            debug!("### Required_amount1={:?}", required_amount1);

            // update the position with the new liq, also mark the range fees as collected on the position
            let pos_range_fee0 = if required_amount0 >= pos_fee0 {
                //we used for sure all the fee
                range_fee0
            } else {
                // we used just a part of the fee, how much is reflected by the required amount; of course we compute it per unit of liq
                pos.range_fee0 + required_amount0 / pos.liq
            };
            let pos_range_fee1 = if required_amount1 >= pos_fee1 {
                range_fee1
            } else {
                pos.range_fee1 + required_amount1 / pos.liq
            };
            pos.update(liq, pos_range_fee0, pos_range_fee1);

            debug!("### Pos_range_fee0={:?}", pos_range_fee0);
            debug!("### Pos_range_fee1={:?}", pos_range_fee1);

            // update pool liquidity
            self.update_ticks_liq(liq, low_tick, high_tick);
            self.update_live_liq(liq, low_tick, high_tick);
            self.update_pos_nft_liq(valid_auth, liq);

            //compute how much we will deduct from the provided amount0,1
            let to_deduct_amount0 = if required_amount0 > pos_fee0 {
                required_amount0 - pos_fee0
            } else {
                Decimal::zero()
            };
            let to_deduct_amount1 = if required_amount1 > pos_fee1 {
                required_amount1 - pos_fee1
            } else {
                Decimal::zero()
            };

            self.log_state("### Internal state after adding the liquidity");

            debug!("### Liquidity added internally.");
            debug!("### To deduct amount0={:?}", to_deduct_amount0);
            debug!("### To deduct amount1={:?}", to_deduct_amount1);

            (to_deduct_amount0, to_deduct_amount1)
        }

        /**
         * Removes the given liquidity from the position and returns de corresponding amount0,1 and the position uncollected fees
         */
        fn remove_liq_internal(&mut self, liq: Decimal, valid_auth: ValidatedProof) -> (Bucket, Bucket) {
            debug!("### Removing liq internal...");

            debug!("### Liq={:?}", liq);

            assert!(
                liq >= Decimal::zero(),
                "Liquidity must be greater or equal to 0. Remove op aborted."
            );

            self.log_state("### Internal state before removing the liquidity");

            //identify the position
            let pos_nft: NonFungible<PositionNFTData> = valid_auth.non_fungible();
            let pos_id = pos_nft.local_id();
            self.validate_pos(pos_id);

            debug!("### Pos_id={:?}", pos_id);

            //update the liquidity on the position NFT
            self.update_pos_nft_liq(valid_auth, -liq);

            let pos = self.positions.get_mut(&pos_id).unwrap();
            let (low_tick, high_tick) = (pos.low_tick, pos.high_tick);

            debug!("### Pos_liq={:?}", pos.liq);
            debug!("### Pos_low_tick={:?}", low_tick);
            debug!("### Pos_high_tick={:?}", high_tick);

            // compute range and position fees for the position
            let (range_fee0, range_fee1) = pool_math::compute_range_fees(
                self.tick,
                self.fee_global0,
                self.fee_global1,
                self.tick_states.get(&low_tick).unwrap(),
                self.tick_states.get(&high_tick).unwrap(),
            );
            let (pos_fee0, pos_fee1) = pool_math::compute_pos_fees(pos.liq, pos.range_fee0, pos.range_fee1, range_fee0, range_fee1);

            debug!("### Range_fee0={:?}", range_fee0);
            debug!("### Range_fee1={:?}", range_fee1);
            debug!("### Pos_fee0={:?}", pos_fee0);
            debug!("### Pos_fee1={:?}", pos_fee1);

            // update the liquidty on the pool
            pos.update(-liq, range_fee0, range_fee1);
            self.remove_pos_if_empty(pos_id);

            self.update_live_liq(-liq, low_tick, high_tick);

            self.update_ticks_liq(-liq, low_tick, high_tick);
            self.remove_tick_if_empty(low_tick);
            self.remove_tick_if_empty(high_tick);

            // compute amount0,1 to give back to the LP
            let (amount0, amount1) = pool_math::compute_range_amounts_given_liq(
                liq,
                self.sqrt_price,
                tick_math::sqrt_price_at_tick(low_tick),
                tick_math::sqrt_price_at_tick(high_tick),
            );

            debug!("### Amount0={:?}", amount0);
            debug!("### Amount1={:?}", amount1);

            // give back also the fees
            let (total_amount0, total_amount1) = (amount0 + pos_fee0, amount1 + pos_fee1);

            debug!("### Total_amount0={:?}", total_amount0);
            debug!("### Total_amount1={:?}", total_amount1);

            // take the required amounts from vaults, in practice the computed amounts might be a bit larger than available, due to
            // rounding errors, so we do these checks here in order to avoid taking more than available from vaults.
            let bucket0 = if self.vault0.amount() > total_amount0 {
                self.vault0.take(total_amount0)
            } else {
                self.vault0.take_all()
            };
            let bucket1 = if self.vault1.amount() > total_amount1 {
                self.vault1.take(total_amount1)
            } else {
                self.vault1.take_all()
            };

            self.log_state("### Internal state after removing the liquidity");

            debug!("### Internal liquidity removed.");
            debug!("### Bucket0={:?}", bucket0.amount());
            debug!("### Bucket1={:?}", bucket1.amount());

            (bucket0, bucket1)
        }

        /**
         * Updates the liquidity on the position NFT coming with the proof.
         */
        fn update_pos_nft_liq(&self, auth: ValidatedProof, liq: Decimal) {
            let pos_nft: NonFungible<PositionNFTData> = auth.non_fungible();
            let mut pos_nft_data = auth.non_fungible::<PositionNFTData>().data();
            let new_liq = pos_nft_data.liq + liq;
            assert!(new_liq >= Decimal::zero(), "Position NFT liq should be positive, op aborted.");
            pos_nft_data.liq = new_liq;
            self.pos_nft_minter_badge
                .authorize(|| auth.non_fungible().update_data(pos_nft_data));
            debug!("### Liqudity for pos NFT with id {:?} updated to {:?}", pos_nft.local_id(), new_liq);
        }

        /**
         * Updates the pool live liquidity with the given range liquidity, if the pool tick is in range
         */
        fn update_live_liq(&mut self, liq: Decimal, low_tick: i32, high_tick: i32) {
            if self.tick >= low_tick && self.tick < high_tick {
                self.live_liq += liq;
            }
        }

        /**
         * Remove the position from memory, if it has no more liquidity
         */
        fn remove_pos_if_empty(&mut self, pos_id: &NonFungibleLocalId) {
            if let Entry::Occupied(o) = self.positions.entry((*pos_id).clone()) {
                if o.get().liq == Decimal::zero() {
                    o.remove_entry();
                }
            }
        }

        /**
         * Update the state of the ticks with the given liquidity
         */
        fn update_ticks_liq(&mut self, liq: Decimal, low_tick: i32, high_tick: i32) {
            self.tick_states
                .entry(low_tick)
                .and_modify(|low_tick_state| low_tick_state.modify_liq(liq, false, self.tick, self.fee_global0, self.fee_global1));
            self.tick_states
                .entry(high_tick)
                .and_modify(|high_tick_state| high_tick_state.modify_liq(liq, true, self.tick, self.fee_global0, self.fee_global1));
        }

        /**
         * Remove the tick from memory, if it has no more liquidity
         */
        fn remove_tick_if_empty(&mut self, tick: i32) {
            if let Entry::Occupied(entry) = self.tick_states.entry(tick) {
                if entry.get().liq_gross == Decimal::zero() {
                    entry.remove();
                    self.used_ticks.remove(&tick);
                }
            }
        }

        /**
         * Implements the swap algorithm of the pool
         */
        fn swap_internal(&mut self, mut bucket: Bucket, is_token0: bool) -> (Bucket, Bucket) {
            let initial_bucket_amount = bucket.amount();

            debug!(
                "### Swapping {:?} of resource {:?} ...",
                initial_bucket_amount,
                bucket.resource_address()
            );

            self.log_state("### Internal state before swap.");

            let mut available_amount = initial_bucket_amount;

            //compute the amount to give back following the swap
            let mut total_swapped_amount = Decimal::zero();
            let mut total_fee_amount = Decimal::zero();
            while available_amount > Decimal::zero() && self.live_liq > Decimal::zero() {
                self.log_state("### Internal state before swap step");

                debug!("### Available_amount={:?}", available_amount);
                debug!("### Total_swapped_amount={:?}", total_swapped_amount);
                debug!("### Total_fee_amount={:?}", total_fee_amount);

                // get next/previous used tick, to see if we have enough available amount to move the price to it.
                let opt_tick_to_cross = if is_token0 {
                    let prev_tick = btree_set_ext::previous_elem(&self.used_ticks, self.tick);
                    debug!("### Swapping towards the previous tick {:?}", prev_tick);
                    prev_tick
                } else {
                    let next_tick = btree_set_ext::next_elem(&self.used_ticks, self.tick);
                    debug!("### Swapping towards the next tick {:?}", next_tick);
                    next_tick
                };

                if let Some(tick_to_cross) = opt_tick_to_cross {
                    let sqrt_price_at_tick_to_cross = tick_math::sqrt_price_at_tick(*tick_to_cross);

                    debug!("### Sqrt_price_at_tick_to_cross={:?}", sqrt_price_at_tick_to_cross);

                    // compute the amount needed to cross tick
                    let needed_amount_to_cross_tick = if is_token0 {
                        pool_math::compute_range_amount0_given_liq(self.live_liq, sqrt_price_at_tick_to_cross, self.sqrt_price)
                    } else {
                        pool_math::compute_range_amount1_given_liq(self.live_liq, self.sqrt_price, sqrt_price_at_tick_to_cross)
                    };
                    debug!("### Needed_amount_to_cross_tick={:?}", needed_amount_to_cross_tick);

                    let is_tick_cross_needed = needed_amount_to_cross_tick < available_amount;
                    debug!("### Is_tick_cross_needed={:?}", is_tick_cross_needed);

                    // we swap just the amount corresponding to the current tick, or all available if we don't need to cross the tick
                    let mut amount_to_swap = if is_tick_cross_needed {
                        needed_amount_to_cross_tick
                    } else {
                        available_amount
                    };
                    debug!("### Before substracting fee, amount_to_swap={:?}", amount_to_swap);

                    // compute fee
                    let fee_amount = amount_to_swap * self.fee;
                    debug!("### Fee_amount={:?}", fee_amount);

                    // don't swap the fees
                    amount_to_swap -= fee_amount;
                    debug!("### After substracting fee, amount_to_swap={:?}", amount_to_swap);

                    // compute the new sqrt price and the amount we get by swapping the provided amount
                    let (new_sqrt_price, swapped_amount) = if is_token0 {
                        pool_math::compute_swap_amount0_price_and_amount1(self.live_liq, self.sqrt_price, amount_to_swap)
                    } else {
                        pool_math::compute_swap_amount1_price_and_amount0(self.live_liq, self.sqrt_price, amount_to_swap)
                    };
                    debug!("### New_sqrt_price={:?}", new_sqrt_price);
                    debug!("### Swapped_amount={:?}", swapped_amount);

                    // update the global values
                    available_amount = available_amount - amount_to_swap - fee_amount;
                    total_swapped_amount += swapped_amount;
                    total_fee_amount += fee_amount;
                    self.sqrt_price = new_sqrt_price;
                    

                    // update global fees
                    let liq_unit_fee = fee_amount / self.live_liq;
                    if is_token0 {
                        self.fee_global0 += liq_unit_fee;
                    } else {
                        self.fee_global1 += liq_unit_fee;
                    }

                    // cross tick if needed
                    if is_tick_cross_needed {
                        self.cross_tick(*tick_to_cross);
                    } else {
                        self.tick = tick_math::tick_at_sqrt_price(new_sqrt_price);
                    }

                    self.log_state("### Internal state after swap step");
                }
            }

            debug!("### Available_amount={:?}", available_amount);
            debug!("### Total_swapped_amount={:?}", total_swapped_amount);
            debug!("### Total_fee_amount={:?}", total_fee_amount);

            // compute the amount that will be kept by the pool
            let to_deduct_amount = initial_bucket_amount - available_amount;
            debug!("### To_deduct_amount={:?}", to_deduct_amount);

            // update the pool fees and return the tokens
            let swapped_bucket = if is_token0 {
                self.vault0.put(bucket.take(to_deduct_amount));
                self.vault1.take(total_swapped_amount)
            } else {
                self.vault1.put(bucket.take(to_deduct_amount));
                self.vault0.take(total_swapped_amount)
            };

            debug!("### Swapped_bucket={:?}", swapped_bucket.amount());
            debug!("### Remainder_bucket={:?}", bucket.amount());

            self.log_state("### Internal state after swap.");
            debug!("### Swapping {:?} of {:?} done.", initial_bucket_amount, bucket.resource_address());

            (swapped_bucket, bucket)
        }

        /**
         * Updates the pool tick, live liquidty and tick states as we cross the provided tick.
         */
        fn cross_tick(&mut self, cross_to_tick: i32) {
            let cross_up = self.tick < cross_to_tick;
            debug!("### Cross_up to tick {:?}? {:?}", cross_to_tick, cross_up);

            //update the current tick fees
            self.tick_states
                .entry(self.tick)
                .and_modify(|state| state.cross_tick(self.fee_global0, self.fee_global1));

            //update tick
            self.tick = cross_to_tick;

            // update the new current tick fees and the pool live liq
            self.tick_states.entry(self.tick).and_modify(|state| {
                self.live_liq = if cross_up { self.live_liq + state.liq_net } else { self.live_liq - state.liq_net };
                state.cross_tick(self.fee_global0, self.fee_global1)
            });
        }

        fn log_state(&self, ctx_msg: &str) {
            debug!("{:?}", ctx_msg);
            debug!("### Vault0={:?}", self.vault0.amount());
            debug!("### Vault1={:?}", self.vault1.amount());
            debug!("### Life liq={:?}", self.live_liq);
            debug!("### Sqrt price={:?}", self.sqrt_price);
            debug!("### Tick={:?}", self.tick);
            debug!("### Used ticks={:?}", self.used_ticks);
            debug!("### Tick states={:?}", self.tick_states);
            debug!("### Position ids={:?}", self.positions.keys());
            debug!("### Positions={:?}", self.positions.values());
            debug!("### Fee={:?}", self.fee);
            debug!("### Fee global0={:?}", self.fee_global0);
            debug!("### Fee global1={:?}", self.fee_global1);
            debug!("### Pool state logged.")
        }
    }
}

/**
 * Keeps:
 * - the liquidity associated with each tick, so we know how to compute the pool live liqudity.
 * - the fees generated when the price was outside this tick (bellow), this is needed to compute the fees generated by each range and
 * then each position.
 */
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone, Debug)]
pub struct TickState {
    pub tick: i32,
    pub liq_net: Decimal,
    pub liq_gross: Decimal,
    pub fee_outside0: Decimal,
    pub fee_outside1: Decimal,
    pub init: bool,
}

impl TickState {
    pub fn new(tick: i32) -> TickState {
        Self {
            tick,
            liq_net: Decimal::zero(),
            liq_gross: Decimal::zero(),
            fee_outside0: Decimal::zero(),
            fee_outside1: Decimal::zero(),
            init: false,
        }
    }

    /**
     * Modify the liquidty that this tick provides as it is crossed in both directions.
     *
     * Also if the tick was just created update the fee generated ourtside of it. By convention, on init,
     * all fees were generated outside of the tick.
     */
    pub fn modify_liq(&mut self, liq: Decimal, is_high_tick: bool, current_tick: i32, fee_global0: Decimal, fee_global1: Decimal) {
        if !self.init {
            if self.tick <= current_tick {
                self.fee_outside0 = fee_global0;
                self.fee_outside1 = fee_global1;
            }
            self.init = true;
        }
        self.liq_net += if is_high_tick { -liq } else { liq };
        self.liq_gross += liq;
    }

    /**
     * Update the fees generated outside this tick
     */
    pub fn cross_tick(&mut self, fee_global0: Decimal, fee_global1: Decimal) {
        self.fee_outside0 = fee_global0 - self.fee_outside0;
        self.fee_outside1 = fee_global1 - self.fee_outside1;
    }
}

/**
 * Keeps the liquidity associated with each position and the fees that were already collected
 */
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Debug)]
struct Position {
    liq: Decimal,
    low_tick: i32,
    high_tick: i32,
    range_fee0: Decimal,
    range_fee1: Decimal,
}

impl Position {
    pub fn new(liq: Decimal, low_tick: i32, high_tick: i32, range_fee0: Decimal, range_fee1: Decimal) -> Position {
        Self {
            liq,
            low_tick,
            high_tick,
            range_fee0,
            range_fee1,
        }
    }

    pub fn update(&mut self, liq_delta: Decimal, new_range_fee0: Decimal, new_range_fee1: Decimal) {
        self.liq += liq_delta;
        self.range_fee0 = new_range_fee0;
        self.range_fee1 = new_range_fee1;
    }
}

/**
 * The NFT that the LP holds for each range it provided liquidty too
 */
#[derive(NonFungibleData)]
pub struct PositionNFTData {
    #[mutable]
    pub liq: Decimal,
}
