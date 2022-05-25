use scrypto::prelude::*;

blueprint! {
    struct EpochDurationOracle {
        epochs_duration_millis: HashMap<u64, u64>,
        last_epoch: u64,
        millis_in_last_epoch: u64,

        // Owner
        owner_badge_ref: ResourceAddress
    }

    impl EpochDurationOracle {
        pub fn new() -> (ComponentAddress, Bucket) {
            Self::new_with_bootstrap(0, 0)
        }

        pub fn new_with_bootstrap(last_epoch: u64, millis_in_last_epoch: u64) -> (ComponentAddress, Bucket) {

            // Owner relative
            let owner_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("Owner of epoch duration oracle."))
                .initial_supply(1);

            let component = Self {
                epochs_duration_millis: HashMap::new(),
                last_epoch,
                millis_in_last_epoch,
                owner_badge_ref: owner_badge.resource_address()
            }.instantiate();

            // Access control
            let access_rules = AccessRules::new()
                .method("tick", rule!(require(owner_badge.resource_address())))
                .default(AccessRule::AllowAll);

            // Component with owner badge
            (component.add_access_check(access_rules).globalize(), owner_badge)
        }

        pub fn tick(&mut self, millis_since_last_tick: u64) -> u64 {
            if self.last_epoch >= Runtime::current_epoch() {
                self.millis_in_last_epoch += millis_since_last_tick;
            }
            else {
                self.epochs_duration_millis.insert(self.last_epoch, self.millis_in_last_epoch + millis_since_last_tick);
                self.last_epoch = Runtime::current_epoch();
                self.millis_in_last_epoch = 0;
            }

            return self.last_epoch
        }

        pub fn millis_since_epoch(&self, epoch: u64) -> u64 {
            if epoch >= self.last_epoch {
                trace!("Requested elapsed on the current or not yet ticked epoch");
                return self.millis_in_last_epoch
            }

            trace!("Requested elapsed on a passed epoch");
            let elapsed: u64 = self.epochs_duration_millis.iter()
            .filter(|(k, _v)| k >= &&epoch)
            .map(|(_k, v)| v)
            .sum();

            elapsed + self.millis_in_last_epoch
        }
    }
}
