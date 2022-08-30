//! This is a mock investment vehicle used for testing purposes. It
//! can be instructive to how an investment vehicle may be
//! implemented.
//!
//! If this file is removed from the project the test suite will not
//! be able to run successfully, but it will not affect the correct
//! running of the Radfolio itself.
//!
//! Note that the main cheat in this mock implementation is that we
//! are provided a treasury in our instantiate function from which to
//! dole out rewards. A real investment vehicle would instead get its
//! rewards from real investment instruments out there in the world.
//!
//! This mock investment vehicle gives a fixed non-compounding
//! interest every epoch.

use scrypto::prelude::*;

blueprint! {

    struct InterestBearingMock {
        /// Percent interest per epoch.
        epoch_interest: Decimal,

        /// The latest epoch we have received interest for.
        last_epoch: u64,

        /// The interest-producing funds in this vehicle.
        invested: Vault,

        /// Interests accrued and not yet removed.
        rewards: Vault,

        /// Our "never-ending" source of funds to award interest
        /// from. A real investment vehicle wouldn't have such a
        /// thing.
        money_printer: Vault,

        /// We don't allow a higher interest-producing investment than
        /// this.
        max_total_investment: Option<Decimal>,

        /// The badge you need to present to call our restricted
        /// methods
        iv_control_badge: ResourceAddress,
    }

    impl InterestBearingMock {

        /// Creates a mock investment vehicle that provides a
        /// non-compounding interest per epoch.
        ///
        /// The interest is taken out of the `money_printer` passed in
        /// and when that vault runs out this mock will stop
        /// functioning.
        ///
        /// Note that if we estimate a year at 18000 epochs then a 10%
        /// annual interest is 0.0005555...% per epoch
        /// (non-compounding).
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone but note that
        /// only someone who can call us with the named
        /// `iv_control_badge` in their auth zone will be able to make
        /// good use of us.
        ///
        /// **Transaction manifest:**
        /// `rtm/mock/instantiate_interestbearing_mock.rtm`
        /// ```text
        #[doc = include_str!("../rtm/mock/instantiate_interestbearing_mock.rtm")]
        /// ```
        pub fn instantiate_interestbearing_mock(
            interest_percent_per_epoch: Decimal,
            money_printer: Bucket,
            iv_control_badge: ResourceAddress,
            max_total_investment: Option<Decimal>) -> ComponentAddress {

            // This is a mock, so we don't aggressively check
            // preconditions. Production implementations should do so
            // however.

            Self {
                epoch_interest: interest_percent_per_epoch,
                last_epoch: Runtime::current_epoch(),
                invested: Vault::new(money_printer.resource_address()),
                rewards: Vault::new(money_printer.resource_address()),
                money_printer: Vault::with_bucket(money_printer),
                max_total_investment,
                iv_control_badge,
            }.instantiate()
                .add_access_check(
                    AccessRules::new()
                        .default(rule!(require(iv_control_badge)))
                        .method("read_investment_value", rule!(allow_all))
                        .method("read_projected_value", rule!(allow_all))
                        .method("read_max_investable", rule!(allow_all))
                ).globalize()
        }

        /// Increases the investment of this vehicle. If the input
        /// funds exceed our maximum we return the excess.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn add_funds(&mut self, mut new_funds: Bucket) -> Option<Bucket> {
            self.harvest_interest();
            if let Some(max) = self.max_total_investment {
                let amount = std::cmp::min(max - self.invested.amount(), new_funds.amount());
                self.invested.put(new_funds.take(amount));
                Some(new_funds)
            } else {
                self.invested.put(new_funds);
                None
            }
        }

        /// Pulls funds out of the vehicle. We are fully liquid and so
        /// all our funds can be pulled out at any time.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn reduce_funds(&mut self, by_amount: Decimal) -> Option<Bucket> {
            self.harvest_interest();
            Some(self.invested.take(by_amount))
        }

        /// Pulls out profits. All interest generated by this mock
        /// will be put into profits so there will often be something
        /// here.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn withdraw_profits(&mut self) -> Option<Bucket> {
            self.harvest_interest();
            Some(self.rewards.take_all())
        }

        /// We include our invested sum, our stored profits and also
        /// any interest that has theoretically accrued from passage
        /// of time but has not yet been actually added to profits.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/investmentvehicle/read_investment_value.rtm`
        /// ```text
        #[doc = include_str!("../rtm/investmentvehicle/read_investment_value.rtm")]
        /// ```
        pub fn read_investment_value(&self) -> Decimal {
            self.invested.amount() + self.rewards.amount() + self.calc_outstanding_interest()
        }

        /// Will return the max investment possible in this vehicle.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/investmentvehicle/read_max_investable.rtm`
        /// ```text
        #[doc = include_str!("../rtm/investmentvehicle/read_max_investable.rtm")]
        /// ```
        pub fn read_max_investable(&self) -> Option<Decimal> {
            if let Some(max) = self.max_total_investment {
                Some(max - self.invested.amount())
            } else {
                None
            }
        }



        // ---
        // Internal methods follow

        /// Based on what epoch we are in, calculates how much
        /// interest we would receive if it was generated right now.
        fn calc_outstanding_interest(&self) -> Decimal {
            let epoch = Runtime::current_epoch();
            assert!(epoch >= self.last_epoch,
                    "Weird time travel not supported");

            self.invested.amount() * (self.epoch_interest / 100) * (epoch - self.last_epoch)
        }

        /// Generates outstanding interest and adds to our `rewards`.
        fn harvest_interest(&mut self) {
            self.rewards.put(
                self.money_printer.take(self.calc_outstanding_interest()));
            self.last_epoch = Runtime::current_epoch();
        }
    }
}
