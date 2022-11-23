//! This is a test blueprint which demonstrates how the intermediary
//! component can look which is relaying calls from SmorgasDAO to a
//! controlled component. Its most immediately useful feature is that
//! it demonstrates the method signature you need to implement in
//! order to perform this function. This can be seen in the `call_dao`
//! and `call_controlled` methods.
//!
//! We also demonstrate the special case of the controlled component
//! being the SmorgasDAO itself. This is somewhat more complicated
//! since we need to avoid re-entrant calls.
//!
//! The Intermediary component has no useful function in the
//! SmorgasDAO, it is only here for testing and demonstration
//! purposes, and it is recommended that you remove it before
//! publishing the SmorgasDAO to a real ledger. (Keeping it in would
//! just needlessly cost you extra transaction fees for publishing
//! it.)

use scrypto::prelude::*;

blueprint! {
    struct Intermediary {
        dao_component: ComponentAddress,
        controlled_component: ComponentAddress,
        dao_admin_badge: Vault
    }

    impl Intermediary {

        /// ---
        ///
        /// **Access control:** Anyone can instantiate this component.
        ///
        /// **Transaction manifest:** 
        /// `rtm/intermediary/instantiate_intermediary.rtm`
        /// ```text
        #[doc = include_str!("../rtm/intermediary/instantiate_intermediary.rtm")]
        /// ```
        pub fn instantiate_intermediary(dao_component: ComponentAddress,
                                        controlled_component: ComponentAddress,
                                        controlled_admin_badge: ResourceAddress)
                                        -> ComponentAddress {
            Self {
                dao_component,
                controlled_component,
                dao_admin_badge: Vault::new(controlled_admin_badge),
            }
            .instantiate().globalize()
        }

        /// This is the first of three stages needed for the
        /// SmorgasDAO to alter its own configuration with an
        /// executive proposal.
        ///
        /// This method receives the SmorgasDAO's admin badge after an
        /// executive proposal passes, and stores it for the
        /// multi-stage implmenetation of the proposal. 
        ///
        /// ---
        ///
        /// **Access control:** This method doesn't do anything useful
        /// unless it's called with the DAO's admin badge in
        /// `badges[0]`
        ///
        /// **Transaction manifest:** This method is only ever called
        /// by a different component. Since users are not meant to
        /// call it directly no transaction manifest is provided.
        pub fn store_dao_admin_badge(&mut self, _: Vec<Proof>,
                                     mut badges: Vec<Bucket>, funds: Option<Bucket>)
                        -> (Vec<Bucket>, Option<Bucket>) {
            // Note that often we want to authenticate the caller
            // here, perhaps by checking that the proof vector has the
            // admin badge of the DAO itself. That isn't needed here
            // since the code only works if we've been given the DAO
            // admin badge in a bucket.

            self.dao_admin_badge.put(badges.remove(0));
            (badges, funds)
        }

        /// This is the second stage of effecting a configuration
        /// change of the SmorgasDAO.
        ///
        /// This method uses the Admin badge that was stored in the
        /// first stage, and uses its authority to make a
        /// configuration change on the originating SmorgasDao
        /// component.
        ///
        /// ---
        ///
        /// **Access control:** This method doesn't do anything useful
        /// unless it's called with the DAO's admin badge in
        /// `badges[0]`
        ///
        /// **Transaction manifest:** This method is only ever called
        /// by a different component. Since users are not meant to
        /// call it directly no transaction manifest is provided.
        pub fn execute_dao_call(&mut self) {
            self.dao_admin_badge.authorize(
                ||
                    borrow_component!(self.dao_component).call::<()>(
                        "set_proposal_duration",
                        args!(100u64)));
        }

        /// This is the third stage of effecting a configuration
        /// change of the SmorgasDAO.
        ///
        /// This method return the Admin badge we stored here back to
        /// the DAO.
        ///
        /// ---
        ///
        /// **Access control:** This method doesn't do anything useful
        /// unless it's called with the DAO's admin badge in
        /// `badges[0]`
        ///
        /// **Transaction manifest:** This method is only ever called
        /// by a different component. Since users are not meant to
        /// call it directly no transaction manifest is provided.
        pub fn return_dao_admin_token(&mut self) {
            borrow_component!(self.dao_component).call::<()>(
                "return_internal_badge",
                args!(self.dao_admin_badge.take_all()));
        }

        /// This method relays a call from a SmorgasDAO executive
        /// proposal to a "third-party" component ostensibly in "a
        /// different package". While the scare quotes there both
        /// technically surround lies, this is nevertheless a useful
        /// proof of concept demonstration.
        ///
        /// ---
        ///
        /// **Access control:** This method doesn't do anything useful
        /// unless it's called with the controlled component's admin
        /// badge in `badges[0]`
        ///
        /// **Transaction manifest:** This method is only ever called
        /// by a different component. Since users are not meant to
        /// call it directly no transaction manifest is provided.
        pub fn call_controlled(&self, _: Vec<Proof>, badges: Vec<Bucket>, funds: Option<Bucket>)
                               -> (Vec<Bucket>, Option<Bucket>) {
            // Note that normally we want to authenticate the caller
            // here, perhaps by checking that the proof vector has the
            // admin badge of the DAO itself. That isn't needed here
            // since the below code only works if we've been given the
            // controlled component admin badge in a bucket, and only
            // the DAO could give us that.
            badges[0].authorize(
                ||
                    borrow_component!(self.controlled_component).call::<()>(
                        "count_me",
                        args!()));
            
            (badges, funds)
        }
    }
}

       
