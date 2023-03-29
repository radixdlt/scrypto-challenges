//! # Pool Blueprint
//!
//! Implements a concentrated liquidity pool using constant-sum market making for each step. This
//! blueprint is to be instantiated privately.
//!
//! # Functions & Methods
//!
//! ### Function
//! - [new](PoolComponent::new) - Instantiates a new [`PoolComponent`] and returns it.
//!
//! ### Methods
//! - [add_liquidity](PoolComponent::add_liquidity) - Adds liquidity to the pool at the closest rate to the given rate.
//! - [add_liquidity_at_step](PoolComponent::add_liquidity_at_step) - Adds liquidity to the pool at the given step.
//! - [add_liquidity_at_steps](PoolComponent::add_liquidity_at_steps) - Adds liquidity to the pool at the given steps.
//! - [remove_liquidity_at_step](PoolComponent::remove_liquidity_at_step) - Removes all the liquidity associated to a given [`Position`] at the given step.
//! - [remove_liquidity_at_steps](PoolComponent::remove_liquidity_at_steps) - Removes all the liquidity associated to a given [`Position`] at the given steps.
//! - [remove_liquidity_at_rate](PoolComponent::remove_liquidity_at_rate) - Removes all the liquidity associated to a given [`Position`] at the given rate.
//! - [remove_all_liquidity](PoolComponent::remove_all_liquidity) - Removes all the liquidity associated to a given [`Position`].
//! - [claim_fees](PoolComponent::claim_fees) - Claims fees associated to a [`Position`].
//! - [swap](PoolComponent::swap) - Swaps stablecoins/other tokens for other tokens/stablecoins.
//! - [claim_protocol_fees](PoolComponent::claim_protocol_fees) - Claims protocol fees.
//! - [get_state](PoolComponent::get_state) - Returns the full state of the blueprint.
//! - [rate_at_step](PoolComponent::rate_at_step) - Returns the exchange rate associated to a given step.
//! - [step_at_rate](PoolComponent::step_at_rate) - Returns the step associated to a given exchange rate.

use scrypto::blueprint;

#[blueprint]
mod pool {
    use crate::constants::NB_STEP;
    use crate::decimal_maths::{ln, pow};
    use crate::oracle::OracleComponent;
    use crate::pool_step::PoolStepComponent;
    use crate::position::Position;

    pub struct Pool {
        /// Percentage rate increase between each step
        rate_step: Decimal,

        /// Current step
        current_step: u16,

        /// Minimum exchange rate
        min_rate: Decimal,

        /// Pool steps
        steps: HashMap<u16, PoolStepComponent>,

        /// Protocol fees in stablecoins
        stable_protocol_fees: Vault,

        /// Other protocol fees
        other_protocol_fees: Vault,

        /// Price oracle
        oracle: OracleComponent,
    }

    impl Pool {
        /// Instantiates a new [`PoolComponent`] and returns it.
        ///
        /// # Arguments
        /// * `stable` - ResourceAddress of the stablecoin
        /// * `bucket_other` - ResourceAddress of the other token
        /// * `initial_rate` - Initial exhcange rate of the pool
        /// * `min_rate` - Minimum exchange rate of the pool
        /// * `max_rate` - Maximum exchange rate of the pool
        pub fn new(
            stable: ResourceAddress,
            other: ResourceAddress,
            initial_rate: Decimal,
            min_rate: Decimal,
            max_rate: Decimal,
        ) -> PoolComponent {
            assert!(
                min_rate > Decimal::ZERO,
                "The minimum rate should be positive"
            );
            assert!(
                max_rate > min_rate,
                "The maximum rate should be greater than the minimum rate"
            );
            assert!(
                initial_rate >= min_rate && initial_rate <= max_rate,
                "The initial rate should be included in the given rate range"
            );

            // Computes the rate % change between each steps
            // We want the relation: max_rate = min_rate*rate_step^65535
            let exponent = Decimal::ONE / NB_STEP;
            let rate_step = pow::<Decimal, Decimal>(max_rate / min_rate, exponent);

            // Computes the current pool step from input tokens
            let dec_step = ln(initial_rate / min_rate) / ln(rate_step);
            assert!(dec_step >= Decimal::zero() && dec_step <= Decimal::from(NB_STEP));
            let current_step: u16 = ((dec_step.floor().0) / Decimal::ONE.0).try_into().unwrap();

            let mut steps = HashMap::new();
            let initial_step =
                PoolStepComponent::new(stable.clone(), other.clone(), initial_rate.clone());
            steps.insert(current_step.clone(), initial_step);

            let component = Self {
                rate_step,
                current_step,
                min_rate,
                steps,
                stable_protocol_fees: Vault::new(stable),
                other_protocol_fees: Vault::new(other),
                oracle: OracleComponent::new(),
            }
            .instantiate();

            component
        }

        /// Adds liquidity to the pool at the closest rate to the given rate.
        ///
        /// # Arguments
        /// * `bucket_stable` - Bucket containing stablecoins to add as liquidity
        /// * `bucket_other` - Bucket containing the other tokens to add as liquidity
        /// * `rate` - Rate at which to provide liquidity
        /// * `position` - [`Position`] of the user
        pub fn add_liquidity(
            &mut self,
            bucket_stable: Bucket,
            bucket_other: Bucket,
            rate: Decimal,
            position: Position,
        ) -> (Bucket, Bucket, Position) {
            let step = self.step_at_rate(rate);
            self.add_liquidity_at_step(bucket_stable, bucket_other, step, position)
        }

        /// Adds liquidity to the pool at the given step.
        ///
        /// # Arguments
        /// * `bucket_a` - Bucket containing first token to add as liquidity
        /// * `bucket_b` - Bucket containing second token to add as liquidity
        /// * `step` - Step at which to provide liquidity
        /// * `position` - [`Position`] of the user
        pub fn add_liquidity_at_step(
            &mut self,
            bucket_a: Bucket,
            bucket_b: Bucket,
            step: u16,
            mut position: Position,
        ) -> (Bucket, Bucket, Position) {
            let step_position = position.get_step(step);
            let (bucket_stable, bucket_other) =
                if bucket_a.resource_address() == self.stable_protocol_fees.resource_address() {
                    (bucket_a, bucket_b)
                } else {
                    (bucket_b, bucket_a)
                };

            // Get or create the given step
            let pool_step = match self.steps.get_mut(&step) {
                Some(ps) => ps,
                None => {
                    let rate = self.rate_at_step(step);
                    let new_step = PoolStepComponent::new(
                        self.stable_protocol_fees.resource_address(),
                        self.other_protocol_fees.resource_address(),
                        rate,
                    );
                    self.steps.insert(step, new_step);
                    self.steps.get(&step).unwrap()
                }
            };

            // Add liquidity to step and return
            let (stable_return, other_return, new_step) = pool_step.add_liquidity(
                bucket_stable,
                bucket_other,
                self.current_step < step,
                step_position,
            );
            position.insert_step(step, new_step);

            (stable_return, other_return, position)
        }

        /// Adds liquidity to the pool at the given steps.
        ///
        /// # Arguments
        /// * `bucket_stable` - Bucket containing stablecoins to add as liquidity
        /// * `bucket_other` - Bucket containing the other tokens to add as liquidity
        /// * `steps` - List of steps and amounts of tokens to add to each steps
        /// * `position` - [`Position`] of the user
        pub fn add_liquidity_at_steps(
            &mut self,
            mut bucket_stable: Bucket,
            mut bucket_other: Bucket,
            steps: Vec<(u16, Decimal, Decimal)>,
            position: Position,
        ) -> (Bucket, Bucket, Position) {
            let mut position = position;
            let mut ret_stable = Bucket::new(bucket_stable.resource_address());
            let mut ret_other = Bucket::new(bucket_other.resource_address());

            for (step, amount_stable, amount_other) in steps {
                let (tmp_stable, tmp_other, tmp_pos) = self.add_liquidity_at_step(
                    bucket_stable.take(amount_stable),
                    bucket_other.take(amount_other),
                    step,
                    position,
                );
                ret_stable.put(tmp_stable);
                ret_other.put(tmp_other);
                position = tmp_pos;
            }
            ret_stable.put(bucket_stable);
            ret_other.put(bucket_other);
            (ret_stable, ret_other, position)
        }

        /// Removes all the liquidity associated to a given [`Position`] at a given step.
        ///
        /// # Arguments
        /// * `step` - Step at which to remove the liquidity
        /// * `position` - [`Position`] of the user
        pub fn remove_liquidity_at_step(
            &mut self,
            step: u16,
            mut position: Position,
        ) -> (Bucket, Bucket, Position) {
            let step_position = position.remove_step(step);
            let mut bucket_stable = Bucket::new(self.stable_protocol_fees.resource_address());
            let mut bucket_other = Bucket::new(position.token);

            if step_position.liquidity > Decimal::ZERO {
                let pool_step = self.steps.get(&step).unwrap();
                let (tmp_stable, tmp_other) = pool_step.remove_liquidity(step_position);
                bucket_stable.put(tmp_stable);
                bucket_other.put(tmp_other);
            }
            (bucket_stable, bucket_other, position)
        }

        /// Removes all the liquidity associated to a given [`Position`] between the given steps.
        ///
        /// # Arguments
        /// * `start_step` - Start step at which to provide liquidity
        /// * `stop_step` - Stop step at which to provide liquidity
        /// * `position` - [`Position`] of the user
        pub fn remove_liquidity_at_steps(
            &mut self,
            start_step: u16,
            stop_step: u16,
            position: Position,
        ) -> (Bucket, Bucket, Position) {
            let mut ret_stable = Bucket::new(self.stable_protocol_fees.resource_address());
            let mut ret_other = Bucket::new(position.token);
            let mut ret_pos = position;

            for i in start_step..stop_step + 1 {
                let (tmp_stable, tmp_other, tmp_pos) = self.remove_liquidity_at_step(i, ret_pos);
                ret_stable.put(tmp_stable);
                ret_other.put(tmp_other);
                ret_pos = tmp_pos;
            }
            (ret_stable, ret_other, ret_pos)
        }

        /// Removes all the liquidity associated to a given [`Position`] at a given rate.
        ///
        /// # Arguments
        /// * `step` - Rate at which to remove the liquidity
        /// * `position` - [`Position`] of the user
        pub fn remove_liquidity_at_rate(
            &mut self,
            rate: Decimal,
            position: Position,
        ) -> (Bucket, Bucket, Position) {
            let step = self.step_at_rate(rate);
            self.remove_liquidity_at_step(step, position)
        }

        /// Removes all the liquidity associated to a given [`Position`].
        ///
        /// # Arguments
        /// * `position` - [`Position`] of the user
        pub fn remove_all_liquidity(&mut self, position: Position) -> (Bucket, Bucket) {
            let step_positions = position.step_positions;
            let mut bucket_stable = Bucket::new(self.stable_protocol_fees.resource_address());
            let mut bucket_other = Bucket::new(position.token);

            for (step, step_position) in step_positions {
                let pool_step = self.steps.get(&step).unwrap();
                let (tmp_stable, tmp_other) = pool_step.remove_liquidity(step_position);
                bucket_stable.put(tmp_stable);
                bucket_other.put(tmp_other);
            }
            (bucket_stable, bucket_other)
        }

        /// Claims fees associated to a given [`Position`].
        ///
        /// # Arguments
        /// * `position` -  Position value of the caller
        pub fn claim_fees(&mut self, mut position: Position) -> (Bucket, Bucket, Position) {
            let mut bucket_stable = Bucket::new(self.stable_protocol_fees.resource_address());
            let mut bucket_other = Bucket::new(position.token);

            for (step, step_position) in position.step_positions.iter_mut() {
                let pool_step = self.steps.get(step).unwrap();
                let (tmp_stable, tmp_other, new_step_position) =
                    pool_step.claim_fees(step_position.clone());
                bucket_stable.put(tmp_stable);
                bucket_other.put(tmp_other);
                step_position.update(&new_step_position)
            }

            (bucket_stable, bucket_other, position)
        }

        /// Swaps stablecoins/other tokens for other tokens/stablecoins.
        ///
        /// # Arguments
        /// * `input_bucket` - bucket containing stablecoins/other tokens
        pub fn swap(&mut self, input_bucket: Bucket) -> (Bucket, Bucket) {
            if input_bucket.resource_address() == self.stable_protocol_fees.resource_address() {
                self.swap_for_other(input_bucket)
            } else {
                self.swap_for_stable(input_bucket)
            }
        }

        /// Internal functions that swaps stablecoins for the other tokens.
        ///
        /// # Arguments
        /// * `input buckets` - bucket containing stablecoins to swap
        fn swap_for_other(&mut self, input_bucket: Bucket) -> (Bucket, Bucket) {
            // Input bucket has stable tokens

            let mut other_ret = Bucket::new(self.other_protocol_fees.resource_address());
            let mut stable_ret = Bucket::from(input_bucket);

            loop {
                match self.steps.get_mut(&self.current_step) {
                    Some(pool_step) => {
                        let (stable_tmp, other_tmp, stable_protocol_fees, is_empty) =
                            pool_step.swap_for_other(stable_ret);
                        self.stable_protocol_fees.put(stable_protocol_fees);
                        other_ret.put(other_tmp);
                        stable_ret = stable_tmp;

                        if !is_empty {
                            break;
                        }
                    }
                    None => {}
                };

                if self.current_step == 65535 {
                    break;
                }
                self.current_step += 1;
            }

            (stable_ret, other_ret)
        }

        /// Internal functions that swaps the other tokens for stablecoins.
        ///
        /// # Arguments
        /// * `input buckets` - bucket containing other tokens to swap
        fn swap_for_stable(&mut self, input_bucket: Bucket) -> (Bucket, Bucket) {
            // Input bucket has other tokens

            let mut other_ret = Bucket::from(input_bucket);
            let mut stable_ret = Bucket::new(self.stable_protocol_fees.resource_address());

            loop {
                match self.steps.get_mut(&self.current_step) {
                    Some(pool_step) => {
                        let (stable_tmp, other_tmp, other_protocol_fees, is_empty) =
                            pool_step.swap_for_stable(other_ret);
                        self.other_protocol_fees.put(other_protocol_fees);
                        other_ret = other_tmp;
                        stable_ret.put(stable_tmp);

                        if !is_empty {
                            break;
                        }
                    }
                    None => {}
                };
                if self.current_step == 0 {
                    break;
                } else {
                    self.current_step -= 1;
                }
            }

            (stable_ret, other_ret)
        }

        /// Claims protocol fees.
        pub fn claim_protocol_fees(&mut self) -> (Bucket, Bucket) {
            (
                self.stable_protocol_fees.take_all(),
                self.other_protocol_fees.take_all(),
            )
        }

        /// Makes a new oracle observations if last observations happened more than 20 seconds ago
        pub fn new_observation(&mut self) {
            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            self.oracle.new_observation(current_time, self.current_step);
        }

        pub fn get_twap_since(&self, timestamp: i64) -> Decimal {
            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let twas = self
                .oracle
                .get_time_weighted_average_step_since(timestamp, current_time);
            let twap = self.rate_step.powi(twas as i64);
            twap
        }

        /// Returns the full state of the blueprint.
        pub fn get_state(
            &self,
        ) -> (
            Decimal,
            u16,
            Decimal,
            (Decimal, Decimal),
            Vec<(u16, Vec<Decimal>)>,
        ) {
            let mut pool_steps_state = vec![];

            for (step_id, pool_step) in &self.steps {
                let state = pool_step.get_step_state();

                pool_steps_state.push((*step_id, state));
            }

            (
                self.rate_step,
                self.current_step,
                self.min_rate,
                (
                    self.stable_protocol_fees.amount(),
                    self.other_protocol_fees.amount(),
                ),
                pool_steps_state,
            )
        }

        #[inline]
        /// Returns the exchange rate associated to a given step.
        pub fn rate_at_step(&self, step: u16) -> Decimal {
            self.min_rate * (self.rate_step).powi(step.into())
        }

        /// Returns the step associated to a given exchange rate.
        pub fn step_at_rate(&self, rate: Decimal) -> u16 {
            // rate = min_rate*(1 + rate_step)**step => ln(rate/min_rate) = step*ln(1 + rate_step)
            let dec_step = ln(rate / self.min_rate) / ln(self.rate_step);
            assert!(dec_step >= Decimal::zero() && dec_step <= Decimal::from(NB_STEP));
            let step_id: u16 = ((dec_step.floor().0) / Decimal::ONE.0).try_into().unwrap();
            step_id
        }
    }
}
