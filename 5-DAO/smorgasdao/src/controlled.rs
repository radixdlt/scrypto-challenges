//! This is a test blueprint which assumes the role of a third-party
//! component in a different package which we are using our DAO to
//! control.
//!
//! It has a protected method `count_me` which you can only call if
//! you have this component's admin badge, and that badge gets put
//! into the SmorgasDAO so it can control this component through
//! executive proposals.
//!
//! The Controlled component has no useful function in the SmorgasDAO,
//! it is only here for testing, and it is recommended that you remove
//! it before publishing the SmorgasDAO to a real ledger. (Keeping it
//! in would just needlessly cost you extra transaction fees for
//! publishing it.)

use scrypto::prelude::*;

blueprint! {
    struct Controlled {
        call_count: u64,
    }

    impl Controlled {

        /// ---
        ///
        /// **Access control:** Anyone can instantiate this component.
        ///
        /// **Transaction manifest:** 
        /// `rtm/controlled/instantiate_controlled.rtm`
        /// ```text
        #[doc = include_str!("../rtm/controlled/instantiate_controlled.rtm")]
        /// ```
        pub fn instantiate_controlled() -> (ComponentAddress, Bucket, ResourceAddress) {

            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name",
                          "Controlled admin badge")
                .initial_supply(1);
            let admin_addr = admin_badge.resource_address();

            let auth: AccessRules = AccessRules::new()
                .default(rule!(require(admin_addr)))
                .method("read_count", rule!(allow_all))
                ;

            let mut controlled =
                Self { call_count: 0 }
            .instantiate();
            controlled.add_access_check(auth);
            (controlled.globalize(), admin_badge, admin_addr)
        }

        /// Increases our internal counter by one. We call this to
        /// establish that we can call protected methods from the DAO.
        ///
        /// ---
        ///
        /// **Access control:** Requires the admin badge.
        ///
        /// **Transaction manifest:** This method is only ever called
        /// by a different component. Since users are not meant to
        /// call it directly no transaction manifest is provided.
        pub fn count_me(&mut self) {
            self.call_count += 1;
        }

        /// Reads our counter. We use it to confirm that we were able
        /// to successfully execute the `count_me` method.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        pub fn read_count(&self) -> u64{
            self.call_count
        }
    }
}

       
