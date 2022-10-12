use scrypto::prelude::*;

blueprint! {
    enum EpochBoxStatus {
        Upcoming,
        Active,
        Expired
    }

    struct EpochBox {
        start_epoch: u64,
        end_epoch: u64,
    }

    impl EpochBox {
        // =====================================================================
        // FUNCTIONS
        // =====================================================================
        pub fn instantiate(start_epoch: u64, end_epoch: u64) -> ComponentAddress {

            assert!(end_epoch > start_epoch);

            Self {
                start_epoch: start_epoch,
                end_epoch: end_epoch,
            }
            .instantiate()
            .globalize()
        }

        // =====================================================================
        // METHODS
        // =====================================================================
        pub fn get_status(&self) -> EpochBoxStatus {
            let current_epoch = Runtime::current_epoch();

            match current_epoch.cmp(&self.start_epoch) {
                Ordering::Less  => EpochBoxStatus::Upcoming, 
                Ordering::Equal => EpochBoxStatus::Active, 
                Ordering::Greater => {
                    match current_epoch.cmp(&self.end_epoch) {
                        Ordering::Greater => EpochBoxStatus::Expired,
                        _ => EpochBoxStatus::Active,
                    }
                }
            }
        }

        pub fn get_start_epoch(&self) -> Decimal {
            self.start_epoch
        }

        pub fn get_end_epoch(&self) -> Decimal {
            self.end_epoch
        }

        pub fn get_duration(&self) -> Decimal {
            self.start_epoch - self.end_epoch + 1 as u64
        }

        pub fn is_expired(&self) -> bool {
            Self::get_status() == EpochBoxStatus::Expired
        }

        pub fn is_active(&self) -> bool {
            Self::get_status() == EpochBoxStatus::Active
        }
        
        pub fn is_upcoming(&self) -> bool { 
            Self::get_status() == EpochBoxStatus::Upcoming
        }
    }
}