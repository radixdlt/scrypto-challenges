//! TWAP oracle blueprint for a Pool

use scrypto::blueprint;

pub const TIME_BETWEEN_OBSERVATIONS: i64 = 20;

#[blueprint]
mod oracle {
    use crate::observation_array::ObservationArray;

    pub struct Oracle {
        observations: ObservationArray,
        last_observation_time: i64,
    }

    impl Oracle {
        pub fn new() -> OracleComponent {
            Self {
                observations: ObservationArray::new(),
                last_observation_time: 0,
            }
            .instantiate()
        }

        pub fn new_observation(&mut self, timestamp: i64, step: u16) {
            if timestamp - self.last_observation_time > TIME_BETWEEN_OBSERVATIONS {
                self.observations.push(timestamp, step);
            }
        }

        pub fn get_time_weighted_average_step_since(
            &self,
            timestamp: i64,
            current_timestamp: i64,
        ) -> u16 {
            self.observations
                .get_time_weighted_average_step_since(timestamp, current_timestamp)
        }
    }
}
