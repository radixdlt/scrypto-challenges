use scrypto::prelude::*;

use crate::callback_scheduler;
use crate::callback_scheduler::*;
use crate::utils;

blueprint! {

    /// This is an dummy component that demonstrates
    /// how the CallbackScheduler component may be used
    struct DummyComponent {
        /// This is just a regular admin badge
        admin_badge: Vault,

        /// This is a vault holding some tokens
        tokens: Vault,

        /// A vault containing a NFR that is allowed to burn tokens
        burner_badge: Vault,

        /// The address of the CallbackScheduler component is stored so that we can schedule
        /// callbacks from inside our component.
        /// If callbacks were scheduled from outside this component this field would not be needed
        scheduler_component: ComponentAddress,

        /// This vault stores all CallbackHandle NFRs that are associated with
        /// callbacks scheduled for this method.
        /// Having this vault is a necessary part of the integration with the CallbackScheduler.
        ///
        /// **It is very important that deposit access to this method is restricted.**
        /// An attack must not be allowed to deposit CallbackHandles here as they would then
        /// be able to schedule arbitrary callbacks that this component would authorize!
        callback_handles: Vault,
    }

    impl DummyComponent {

        /// Instantiates our ExampleComponent
        ///
        /// # Arguments:
        /// * `scheduler_component` - The component address of the CallbackScheduler component we
        /// want to connect our component to. It is vitally important that we double check that the
        /// address we pass in here is of the correct component. We will be very well advised to
        /// check that component's blueprint code and make sure it is legit! After all, we are
        /// handing some authorization privileges to that component!
        pub fn instantiate_demo_component(
            scheduler_component: ComponentAddress
        ) -> (ComponentAddress, Bucket) {
            let mut admin_badge = ResourceBuilder::new_fungible()
                .metadata("name", "ExampleProject Admin Badge")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("2"));

            let burner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("1"));

            let tokens = ResourceBuilder::new_fungible()
                .metadata("name", "Shiny Token")
                .divisibility(DIVISIBILITY_MAXIMUM)
                .burnable(rule!(require(burner_badge.resource_address())), LOCKED)
                .initial_supply(dec!("1000000"));

            utils::debug_log_resources!(admin_badge, tokens);

            let rules = AccessRules::new()
                .method("burn_tokens", rule!(allow_all))
                .method("authorize_callback", rule!(allow_all))
                .method("deposit_callback_handle", rule!(require(admin_badge.resource_address())));

            let component =  Self {
                admin_badge: Vault::with_bucket(admin_badge.take(dec!("1"))),
                burner_badge: Vault::with_bucket(burner_badge),
                tokens: Vault::with_bucket(tokens),
                scheduler_component,
                // Here we retrieve the CallbackHandle resource address from the scheduler component
                callback_handles: Vault::new(
                    callback_scheduler::get_callback_handle_resource(scheduler_component)),
            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            (component, admin_badge)
        }

        /// Burn some of the tokens held by this component
        ///
        /// # Arguments:
        /// * `amount` - The amount of tokens to burn
        pub fn burn_tokens(&mut self, amount: Decimal) {
            self.burner_badge.authorize(||
                self.tokens.take(amount).burn()
            );
        }

        /// This method is part of the SchedulerComponent's API amd must be implemented as part of
        /// the integration. It will be called just before the actual method call (described in the
        /// callback) will be performed. It is used to authenticate the callback and to also
        /// authorize the method call by handing out any required proofs to the scheduler component.
        ///
        /// This is the only method that must be implemented as part of the scheduler integration.
        ///
        /// # Arguments:
        /// * `callback`: A proof containing a single Callback NFR. This demonstrates that the caller
        /// is the same party that the callback was scheduled with.
        ///
        /// # Returns:
        /// * `Bucket` - The CallbackHandle NFR that is associated with the given callback
        /// * `Vec<Proof>` - Any proofs that are required by the method that will be called during
        /// the callback
        pub fn authorize_callback(&mut self, callback: Proof) -> (Bucket, Vec<Proof>) {

            // Verify the callback is authentic. It is VERY IMPORTANT that this method
            // is called here!
            let callback_handle = Callback::verify(&callback,
                |callback_id| self.callback_handles.take_non_fungible(callback_id));

            // Produce any proofs that are required by the target method
            let proofs = vec![self.admin_badge.create_proof()];

            // Return the CallbackHandle and any proofs to the scheduler component
            (callback_handle, proofs)
        }

        /// Deposits a Callbackhandle NFR into this component so that it may accept the associated callback
        ///
        /// # Arguments:
        /// * `callback_handle`: A bucket containing a Callback NFR
        pub fn deposit_callback_handle(&mut self, callback_handle: Bucket) {
            self.callback_handles.put(callback_handle);
        }
    }
}

#[cfg(test)]
mod test {
    use scrypto::values::ScryptoValue;

    use super::*;
    use crate::callback_scheduler::{CallbackRequest, Trigger};
    use std::str::FromStr;

    /// This test may be used to generate a CallbackRequest
    #[test]
    fn abc() {
        let trigger = Trigger::AtDateTime {
            date_time: "2022-10-01T12:30:55+00".to_owned(),
            tolerance_seconds: 120,
        };
        let req = CallbackRequest::new(
            trigger,
            ComponentAddress::from_str("02ab11a23a3f62486e5a99a4fbc6e8b640b17f531a0d90d54565f3")
                .unwrap(),
            "burn_tokens",
            args!(dec!("1000")),
            None,
        );

        println!("{}", ScryptoValue::from_value(&req).to_string());
    }
}
