//! 
//! TODO

use scrypto::prelude::*;

blueprint! {

    struct ArchProposal {}

    impl ArchProposal {

        /// TODO
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn add_funds(&mut self, mut _new_funds: Bucket) -> Option<Bucket>
        { None }
        
        /// TODO
        pub fn execute(&mut self, mut _new_funds: Bucket) -> Option<Bucket>
        { None }

        /// TODO
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn reward(&mut self, _by_amount: Decimal) -> Option<Bucket>
        { None }

        /// TODO
        /// ---
        ///
        /// **Access control:** Can only be called with `proposal_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn airdrop(&mut self) -> Option<Bucket>
        { None }

      
    }
}
