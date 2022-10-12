/*!
The blueprint implement Local Oracle Component for the DAO to get precise unix time data for its smartcontract logic.

The blueprint use a cross-blueprint call design pattern to get unix time data from a real Oracle.

## Functions and Methods Overview
- Function [new()](LocalOracle_impl::LocalOracle::new): Instantiate new Local Oracle Component (before globalized).
- Method [refund_oracle()](LocalOracle_impl::LocalOracle::refund_oracle):
The method is for the DAO (or any donator) to refund the oracle account.
- Method [current()](LocalOracle_impl::LocalOracle::current):
The method is for the DAO component or proposal components to get unix time data from the Oracle.
*/


use scrypto::prelude::*;

external_component! {

    Oracle {
        fn check_badge(&self, a: Proof) -> bool;
        fn refund_account(&mut self, a: Proof, b: Bucket);
        fn get_data_string(&mut self, a: Proof) -> String;
        fn easy_request(&self, identity: Proof);
    }
}

blueprint! {

    /// The LocalOracle Component for the DAO to get precise time data (unix time, time unit: second).
    struct LocalOracle {
        /// The oracle component address.
        address: ComponentAddress,
        /// Vault store the data access badge, to get data from the Oracle.
        badge: Vault,
    }
    
    impl LocalOracle {

        /// This function instantiate new Proposal Component.
        ///
        /// # Input
        /// - address: The Oracle component address that the DAO would use.
        /// - badge: Bucket contain the data access badge to get data from the Oracle.
        ///
        /// # Output
        /// The Local Oracle Component, the component will be 
        /// further used to add access rule and globalize along with the DAO Component
        /// 
        /// # Smartcontract logic
        /// The function should only be called
        /// through the [new()](crate::align_dao::DAO_impl::DAO::new) function
        pub fn new(address: ComponentAddress, badge: Bucket) -> LocalOracleComponent {
            let oracle: Oracle = address.into();
            assert!(
                oracle.check_badge(badge.create_proof()),
                "[LocalOracle]: The provided badge is not from this oracle address"
            );

            Self {
                address,
                badge: Vault::with_bucket(badge),
            }
            .instantiate()
        }

        /// This method is for the DAO (or any donator) to refund the oracle account.
        /// # Input
        /// - bucket: Oracle token bucket
        /// # Access Rule
        /// Anyone with the right resource provided can call and execute this method.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/init/fund_oracle.rtm`
        /// ```text
        #[doc = include_str!("../rtm/init/fund_oracle.rtm")]
        /// ```
        pub fn refund_oracle(&self, bucket: Bucket) {
            let mut oracle: Oracle = self.address.into();
            let amount = bucket.amount();
            info!(
                "[LocalOracle]: Refunded {} token into the oracle account.",
                amount
            );
            oracle.refund_account(self.badge.create_proof(), bucket);
            oracle.easy_request(self.badge.create_proof())
        }

        /// This method is for the DAO component or proposal components to get unix time data from the Oracle.
        /// # Access Rule
        /// The method can only be called through the DAO component or proposal components' controller badges
        pub fn current(&self) -> u64 {
            let mut oracle: Oracle = self.address.into();
            oracle
                .get_data_string(self.badge.create_proof())
                .parse()
                .expect("[LocalOracle]: Cannot parse the oracle data into u64 type.")
        }
    }
}
