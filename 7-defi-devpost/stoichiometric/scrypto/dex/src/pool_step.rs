//! # PoolStep Blueprint
//!
//! Implements a constant-sum AMM that does not recompound fees. This blueprint is to be
//! instantiated privately.
//!
//! # Functions & Methods
//!
//! ### Function
//! - [new](PoolStepComponent::new) - Instantiates a new [`PoolStepComponent`] and returns it.
//! and returns it.
//!
//! ### Methods
//! - [add_liquidity](PoolStepComponent::add_liquidity) - Adds liquidity to the PoolStep given two buckets and returns the excess amount of tokens.
//! - [remove_liquidity](PoolStepComponent::remove_liquidity) - Removes all liquidity associated to a Position from the PoolStep.
//! - [claim_fees](PoolStepComponent::claim_fees) - Claims fees associated to a StepPosition.
//! - [swap_for_stable](PoolStepComponent::swap_for_stable) - Swaps stablecoins for other tokens.
//! - [swap_for_other](PoolStepComponent::swap_for_other) - Swaps other tokens for stablecoins.
//! - [get_step_state](PoolStepComponent::get_step_state) -  Returns the current state of the PoolStep.

use scrypto::blueprint;

#[blueprint]
mod pool_step {
    use crate::constants::{LP_FEE, PROTOCOL_FEE, RATIO_TRADED};
    use crate::position::StepPosition;

    pub struct PoolStep {
        /// Vault containing stablecoins as liquidity
        stable_vault: Vault,

        /// Vault containing other tokens as liquidity
        other_vault: Vault,

        /// Rate of the pool step
        rate: Decimal,

        /// Accrued fees in stablecoins per liquidity unit
        stable_fees_per_liq: Decimal,

        /// Accrued fees in other tokens per liquidity unit
        other_fees_per_liq: Decimal,

        /// Vault containing stablecoin fees
        stable_fees_vault: Vault,

        /// Vault containing other token fees
        other_fees_vault: Vault,
    }

    impl PoolStep {
        /// Instantiates a new [`PoolStepComponent`] and returns it.
        ///
        /// # Arguments
        /// * `token_stable` - Address of the stablecoin that will be traded by the [`PoolStep`]
        /// * `token_other` - Address of the other token that will be traded by the [`PoolStep`]
        /// * `rate` - Fixed rate for this PoolStep
        pub fn new(
            token_stable: ResourceAddress,
            token_other: ResourceAddress,
            rate: Decimal,
        ) -> PoolStepComponent {
            // Create the component
            let component = Self {
                stable_vault: Vault::new(token_stable.clone()),
                other_vault: Vault::new(token_other.clone()),
                rate: rate,
                stable_fees_per_liq: Decimal::ZERO,
                other_fees_per_liq: Decimal::ZERO,
                stable_fees_vault: Vault::new(token_stable.clone()),
                other_fees_vault: Vault::new(token_other.clone()),
            }
            .instantiate();

            component
        }

        /// Adds liquidity to the [`PoolStep`] given two buckets and returns the excess amount of tokens.
        ///
        /// # Arguments
        /// * `bucket_stable` - Bucket containing stablecoins to be added as liquidity
        /// * `bucket_other` - Bucket containing other tokens to be added as liquidity
        /// * `current_step_is_lower` - Boolean stating whether the underlying pool current step is lower than this step
        /// * `step_position` - [`StepPosition`] to add the liquidity to
        pub fn add_liquidity(
            &mut self,
            mut bucket_stable: Bucket,
            mut bucket_other: Bucket,
            current_step_is_lower: bool,
            step_position: StepPosition,
        ) -> (Bucket, Bucket, StepPosition) {
            // Start by claiming_fees and adding them as potential liquidity
            let (fees_stable, fees_other, mut new_step_position) = self.claim_fees(step_position);
            bucket_other.put(fees_other);
            bucket_stable.put(fees_stable);

            // Right amount of tokens to take from the given buckets
            let right_stable;
            let right_other;

            // Liquidity of the pool
            let l_pool = self.stable_vault.amount() + self.other_vault.amount() * self.rate;

            // If the liquidity of the pool is equal to zero, then the pool is empty.
            // The step should then be full of stablecoins or in the other token
            if l_pool.is_zero() {
                // If the underlying pool is in step that is lower, then the current pool rate is lower
                // This means that step should be filled with the other token
                if current_step_is_lower {
                    right_stable = Decimal::ZERO;
                    right_other = bucket_other.amount();
                } else {
                    right_stable = bucket_stable.amount();
                    right_other = Decimal::ZERO;
                }
            } else {
                // If the pool is not empty, we determine the proportion of stablecoins in the liquidity
                // and we make sure that the tokens from the bucket respect the same proportion

                let pool_stable_fraction = self.stable_vault.amount() / l_pool;
                let l_bucket = bucket_stable.amount() + bucket_other.amount() * self.rate;
                let bucket_stable_fraction = bucket_stable.amount() / l_bucket;

                if bucket_stable_fraction >= pool_stable_fraction {
                    // In this case, there is too much stable token input
                    right_other = bucket_other.amount();
                    right_stable = self.rate * right_other * pool_stable_fraction
                        / (Decimal::ONE - pool_stable_fraction);
                } else {
                    // In this case, there is too much other token input
                    right_stable = bucket_stable.amount();
                    right_other = (Decimal::ONE / pool_stable_fraction - Decimal::ONE)
                        * right_stable
                        / self.rate
                }
            }
            self.stable_vault.put(bucket_stable.take(right_stable));
            self.other_vault.put(bucket_other.take(right_other));

            // Update the StepPosition
            new_step_position.liquidity += right_stable + right_other * self.rate;

            (bucket_stable, bucket_other, new_step_position)
        }

        /// Removes all liquidity associated to a [`StepPosition`] from the [`PoolStep`].
        ///
        /// # Arguments
        /// * `step_position` - [`StepPosition`] to remove liquidity from
        pub fn remove_liquidity(&mut self, step_position: StepPosition) -> (Bucket, Bucket) {
            let liquidity = step_position.liquidity;

            // Start by claiming fees
            let (mut fees_stable, mut fees_other, _) = self.claim_fees(step_position);

            // Compute amount of tokens to return. The tokens returned should have the same proportion
            // as the tokens in the pool
            let stable = self.stable_vault.amount();
            let other = self.other_vault.amount();
            let l = stable + other * self.rate;
            let stable_fraction = stable / l;
            let other_take = (Decimal::ONE - stable_fraction) * liquidity / self.rate;

            fees_stable.put(self.stable_vault.take(stable_fraction * liquidity));
            fees_other.put(self.other_vault.take(other_take));

            (fees_stable, fees_other)
        }

        /// Claims fees associated to a [`StepPosition`].
        ///
        /// # Arguments
        /// * `step_position` - [`StepPosition`] to claim fees for
        pub fn claim_fees(
            &mut self,
            step_position: StepPosition,
        ) -> (Bucket, Bucket, StepPosition) {
            // Compute the fees to give
            let stable_fees = (self.stable_fees_per_liq - step_position.last_stable_fees_per_liq)
                * step_position.liquidity;
            let other_fees = (self.other_fees_per_liq - step_position.last_other_fees_per_liq)
                * step_position.liquidity;

            // Put the fees in buckets
            let bucket_stable = self.stable_fees_vault.take(stable_fees);
            let bucket_other = self.other_fees_vault.take(other_fees);

            //
            let mut new_step_position = step_position.clone();
            new_step_position.last_stable_fees_per_liq = self.stable_fees_per_liq;
            new_step_position.last_other_fees_per_liq = self.other_fees_per_liq;

            (bucket_stable, bucket_other, new_step_position)
        }

        /// Swaps other tokens for stablecoins.
        ///
        /// # Arguments
        /// * `other` - bucket containing other tokens to be swapped for stablecoins.
        pub fn swap_for_stable(&mut self, mut other: Bucket) -> (Bucket, Bucket, Bucket, bool) {
            // Compute the real amount of tokens to be traded
            let max_stable = other.amount() * self.rate;
            let real_stable = max_stable.min(self.stable_vault.amount());
            let real_other = real_stable / self.rate;

            // Take fees
            let fees = real_other * LP_FEE;
            let l = self.stable_vault.amount() + self.rate * self.other_vault.amount();
            self.other_fees_per_liq += fees / l;
            self.other_fees_vault.put(other.take(fees));
            let other_protocol_fees = other.take(PROTOCOL_FEE * real_other);

            // Make the swap
            self.other_vault.put(other.take(real_other * RATIO_TRADED));
            let stable = self.stable_vault.take(real_stable * RATIO_TRADED);

            (
                stable,
                other,
                other_protocol_fees,
                self.stable_vault.is_empty(),
            )
        }

        /// Swaps stablecoins for other tokens.
        ///
        /// # Arguments
        /// * `stable` - bucket containing stablecoins to be swapped for other tokens.
        pub fn swap_for_other(&mut self, mut stable: Bucket) -> (Bucket, Bucket, Bucket, bool) {
            // Compute the real amount of tokens to be traded
            let max_other = stable.amount() / self.rate;
            let real_other = max_other.min(self.other_vault.amount());
            let real_stable = real_other * self.rate;

            // Take fees
            let fees = real_stable * LP_FEE;
            let l = self.stable_vault.amount() + self.rate * self.other_vault.amount();
            self.stable_fees_per_liq += fees / l;
            self.stable_fees_vault.put(stable.take(fees));
            let stable_protocol_fees = stable.take(PROTOCOL_FEE * real_stable);

            // Make the swap
            self.stable_vault
                .put(stable.take(real_stable * RATIO_TRADED));
            let other = self.other_vault.take(real_other * RATIO_TRADED);

            (
                stable,
                other,
                stable_protocol_fees,
                self.other_vault.is_empty(),
            )
        }

        /// Returns the current state of the [`PoolStep`].
        pub fn get_step_state(&self) -> Vec<Decimal> {
            vec![
                self.stable_vault.amount(),
                self.other_vault.amount(),
                self.rate,
                self.stable_fees_per_liq,
                self.other_fees_per_liq,
                self.stable_fees_vault.amount(),
                self.other_fees_vault.amount(),
            ]
        }
    }
}
