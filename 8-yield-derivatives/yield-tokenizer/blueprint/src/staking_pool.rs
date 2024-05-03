//! This blueprints implement a single asset staking pool based on the Liquity stability pool concept:
//! https://www.liquity.org/blog/scaling-liquitys-stability-pool.
//! The scaling feature described in the blog post is not implemented in this blueprint,
//! we chose to use PreciseDecimal to mitigate the scaling issue of the running product.
//! Ensuring 1:1 peg of sXRD to XRD peg required hard peg mechanism: sXRD need to be redeem with XRD at any time. in case the system lake XRD ,
//! sXRD will redeem for corresponding LSU, from a redemption value perspective, and redeemed sXRD will be shared to the LSU provider.
//! The purpose of this staking pool is to allow fair distribution of sXRD to LSU contributor.

use scrypto::prelude::*;

/// Data structure storing the state of contributor deposit snapshot
/// Info stored in this struct is used to calculate the amount of redeemable compounded deposit and gains
///
#[derive(ScryptoSbor, Clone, Debug)]
pub struct DepositSnapshot {
    /// The amount of deposit at the time of snapshot
    ///
    pub deposit_amount: Decimal,

    /// Snapshot of the epoch at the time of snapshot
    ///
    pub epoch: u8,

    /// Snapshot of running product at the time of snapshot
    ///
    pub running_sum: PreciseDecimal,

    /// Snapshot of running product at the time of snapshot
    ///
    pub running_product: PreciseDecimal,
}

#[blueprint]
#[types(NonFungibleLocalId, u8, PreciseDecimal, DepositSnapshot, Decimal)]
pub mod staking_pool {

    pub struct StakingPool {
        /// The address of the pool
        pool_res_address: ResourceAddress,

        /// FungibleVault  used to store the deposit of the contributors.
        deposits: FungibleVault,

        /// FungibleVault storing gain distributed to the contributors.
        distributed_resources: FungibleVault,

        /// Each time the pool is emptied, a new epoch is created by incrementing the epoch field.
        current_epoch: u8,

        /// The running sum of each epoch.
        running_sum: KeyValueStore<u8, PreciseDecimal>,

        /// The running product: the product of the consecutive inflation or depletion on deposit and withdraw.
        running_product: PreciseDecimal,

        /// This variable is use to track number of contribution to the pool.
        contribution_counter: u64,

        /// This variable is use to track the total amount of redemption from the pool.
        /// in conjunction with the contribution_counter, it is use to track the emptying of the pool.
        /// - if redemption_counter == contribution_counter, the pool is empty.
        /// - if contribution_counter - redemption_counter == 1, last contributor is interacting with the pool.
        redemption_counter: u64,

        /// The Product-Sum algorithm used to calculate the earned share of the contributors is based on snapshots of some variables: the running product, the running sum, the epoch and the scale.
        /// Each time a contribution or a redemption is made, a snapshot of the 3 variables is taken and store in the `snapshots` variable.
        /// We also keep the value of the initial deposit amount in the snapshot to be able to calculate the reward of the contributor.
        snapshots: KeyValueStore<NonFungibleLocalId, DepositSnapshot>,
    }

    impl StakingPool {
        /// Instantiate a new StakingEngine
        ///
        /// # Arguments
        /// * `pool_mode` - The mode of the staking engine, either a resource or a decimal value.
        /// * `config` - The config of the staking engine.
        ///
        /// # Returns
        /// * The instantiated `StakingEngine` Component.
        ///
        pub fn instantiate(
            pool_res_address: ResourceAddress,
            distributed_res_address: ResourceAddress,
        ) -> Owned<StakingPool> {
            Self {
                pool_res_address,
                current_epoch: 0,
                running_product: PreciseDecimal::ONE,
                contribution_counter: 0,
                redemption_counter: 0,
                deposits: FungibleVault::new(pool_res_address),
                running_sum: KeyValueStore::new_with_registered_type(),
                snapshots: KeyValueStore::new_with_registered_type(),
                distributed_resources: FungibleVault::new(distributed_res_address),
            }
            .instantiate()
        }

        /// * Pool management methods * ///

        /// Withdraw assets from the pool
        /// This method is use to "deflate" the pool with compound effect on future rewards.
        /// The method update the running product of the pool.
        ///
        /// # Arguments
        /// * `amount` - The amount to withdraw
        ///
        /// # Returns
        /// * `FungibleBucket` - Asset withdrawn from the pool
        ///
        pub fn withdraw(&mut self, amount: Decimal) -> FungibleBucket {
            assert!(amount > dec!(0));

            assert!(!self._is_pool_empty());

            let deposit_amount = self.deposits.amount();

            if deposit_amount == amount {
                self.current_epoch += 1;
                self.running_product = PreciseDecimal::ONE;
            } else {
                self.running_product *=
                    PreciseDecimal::from(dec!(1) - (amount / self.deposits.amount()));
            }

            self.deposits.take(amount)
        }

        /// Distribute gains to the contributors
        /// This method is use to distribute gains to the contributors. the gains are distributed based on the share of the contributor in the pool.
        /// If this method is call with the base asset of the pool, it will not be compounded. To have the compound effect, use the `deposit` method.
        ///
        /// # Arguments
        /// * `gain: FungibleBucket` - The gains to distribute
        ///
        pub fn distribute(&mut self, gain: FungibleBucket) {
            assert!(!self._is_pool_empty(), "DISTRIBUTE_EMPTY_POOL");

            let deposits_amount = PreciseDecimal::from(self.deposits.amount());

            if self.running_sum.get_mut(&(self.current_epoch)).is_none() {
                self.running_sum
                    .insert(self.current_epoch, PreciseDecimal::ZERO);
            };

            let mut running_sum = self.running_sum.get_mut(&(self.current_epoch)).unwrap();

            let gain_amount = PreciseDecimal::from(gain.amount());
            *running_sum += gain_amount * (self.running_product / deposits_amount);

            self.distributed_resources.put(gain);
        }

        /// * Pool contributors methods * ///

        pub fn contribute(&mut self, id: NonFungibleLocalId, deposit: FungibleBucket) {
            assert!(self.snapshots.get(&id).is_none());

            let deposit_amount = deposit.amount();

            assert!(deposit_amount > dec!(0));

            self.deposits.put(deposit);

            if deposit_amount == dec!(0) {
                return;
            }

            // If a snapshot does not exist for this id, create a new one.
            if self.running_sum.get(&(self.current_epoch)).is_none() {
                self.running_sum
                    .insert(self.current_epoch, PreciseDecimal::ZERO);
            }
            let running_sum = self.running_sum.get(&self.current_epoch).unwrap();

            self.snapshots.insert(
                id.clone(),
                DepositSnapshot {
                    deposit_amount,
                    epoch: self.current_epoch,
                    running_product: self.running_product,
                    running_sum: *running_sum,
                },
            );

            self.contribution_counter += 1;
        }

        /// Redeem deposit and Claim gains of a contributor
        ///
        /// # Arguments
        /// * `id` - The id of the contributor
        ///
        /// # Returns
        /// * `FungibleBucket` - The gains of the contributor
        ///
        pub fn redeem(&mut self, id: NonFungibleLocalId) -> (FungibleBucket, FungibleBucket) {
            let snapshot = self
                .snapshots
                .remove(&id)
                .expect("SNAPSHOT_NOT_FOUND_ERROR");

            // Calculate the compounded deposit amount using the processed running product at the epoch of the snapshot.
            let compounded_deposit_amount = if snapshot.epoch == self.current_epoch {
                PreciseDecimal::from(snapshot.deposit_amount)
                    * (self.running_product / snapshot.running_product)
            } else {
                PreciseDecimal::ZERO
            };

            // calculate the gain amount using the processed running sum at the epoch of the snapshot.
            let epoch_running_sum = match self.running_sum.get(&(snapshot.epoch)) {
                Some(running_sum) => *running_sum,
                None => PreciseDecimal::ZERO,
            };
            let mut gain_amount = PreciseDecimal::from(snapshot.deposit_amount);
            gain_amount = (gain_amount / snapshot.running_product)
                * (epoch_running_sum - snapshot.running_sum);

            let compounded_deposit_amount = compounded_deposit_amount
                .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                .unwrap();

            let gain_amount = gain_amount
                .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                .unwrap();

            self.redemption_counter += 1;

            if self.contribution_counter - self.redemption_counter == 0 {
                (
                    self.deposits.take_all(),
                    self.distributed_resources.take_all(),
                )
            } else {
                (
                    self.deposits.take(compounded_deposit_amount),
                    self.distributed_resources.take(gain_amount),
                )
            }
        }

        /// * Internal methods * ///

        /// Evaluate if the pool is empty.
        fn _is_pool_empty(&self) -> bool {
            self.contribution_counter == self.redemption_counter
                || self.deposits.amount() == dec!(0)
        }
    }
}
