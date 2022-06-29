use scrypto::prelude::*;

blueprint! {
    struct EpochDurationOracle {
        epochs_duration_millis: HashMap<u64, u64>,
        current_epoch: u64,
        millis_in_current_epoch: u64,

        // Owner
        owner_badge_ref: ResourceAddress
    }

    impl EpochDurationOracle {
        pub fn new() -> (ComponentAddress, Bucket) {
            Self::new_with_bootstrap(0, 0)
        }

        pub fn new_with_bootstrap(current_epoch: u64, millis_in_current_epoch: u64) -> (ComponentAddress, Bucket) {

            // Owner relative
            let owner_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("Owner of epoch duration oracle."))
                .initial_supply(1);

            let component = Self {
                epochs_duration_millis: HashMap::new(),
                current_epoch,
                millis_in_current_epoch,
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
            if self.current_epoch >= Runtime::current_epoch() {
                self.millis_in_current_epoch += millis_since_last_tick;
            }
            else {
                self.epochs_duration_millis.insert(self.current_epoch, self.millis_in_current_epoch + millis_since_last_tick);
                self.current_epoch = Runtime::current_epoch();
                self.millis_in_current_epoch = 0;
            }

            return self.current_epoch
        }

        pub fn millis_since_epoch(&self, epoch: u64) -> u64 {

            assert!(epoch <= self.current_epoch, "The requested epoch has not yet happened or was not yet registered on ledger.");

            if epoch == self.current_epoch {
                trace!("Requested elapsed millis since the current epoch");
                return self.millis_in_current_epoch
            }

            trace!("Requested elapsed on a passed epoch");
            let elapsed: u64 = self.epochs_duration_millis.iter()
            .filter(|(k, _v)| k >= &&epoch)
            .map(|(_k, v)| v)
            .sum();

            elapsed + self.millis_in_current_epoch
        }

        pub fn millis_in_epoch(&self, epoch: u64) -> u64 {

            assert!(epoch <= self.current_epoch, "The requested epoch has not yet happened or was not yet registered on ledger.");

            if epoch == self.current_epoch {
                trace!("Requested elapsed millis on the current epoch");
                return self.millis_in_current_epoch
            }

            trace!("Requested elapsed on a passed epoch");
            let elapsed: &u64 = self.epochs_duration_millis.get(&epoch)
            .unwrap_or_else(|| {
                warn!("Epoch was not registered on the oracle, sorry for the inconvenience. We are returning 0 and suggest you call millis_since_epoch or contact an administrator if you absolutely need this epoch duration.");
                &0u64
            });

            if self.epochs_duration_millis.contains_key(&epoch){
                return elapsed + self.millis_in_current_epoch
            }

            *elapsed
        }
    }
}
