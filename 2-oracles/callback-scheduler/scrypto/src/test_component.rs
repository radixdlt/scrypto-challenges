use scrypto::prelude::*;

use crate::callback_scheduler;
use crate::callback_scheduler::*;
use crate::utils;

blueprint! {

    /// This is an example component that demonstrates
    /// how the CallbackScheduler component may be used
    struct TestComponent {
        /// This is just a regular admin badge
        admin_badge: ResourceAddress,

        /// These are two example badges that will be used to authorize callback access to methods
        /// of this component
        guardian_badge: Vault,
        protector_badge: Vault,

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

    impl TestComponent {

        /// Instantiates our ExampleComponent
        ///
        /// # Arguments:
        /// * `scheduler_component` - The component address of the CallbackScheduler component we
        /// want to connect our component to. It is vitally important that we double check that the
        /// address we pass in here is of the correct component. We will be very well advised to
        /// check that component's blueprint code and make sure it is legit! After all, we are
        /// handing some authorization privileges to that component!
        pub fn instantiate_test_component(
            scheduler_component: ComponentAddress
        ) -> (ComponentAddress, Bucket) {
            let admin_badge = ResourceBuilder::new_fungible()
                .metadata("name", "ExampleProject Admin Badge")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("1"));

            let guardian_badge = ResourceBuilder::new_fungible()
                .metadata("name", "ExampleProject Guardian Badge")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("1"));

            let protector_badge = ResourceBuilder::new_fungible()
                .metadata("name", "ExampleProject Protector Badge")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("1"));

            utils::debug_log_resources!(admin_badge);

            let rules = AccessRules::new()
                .method("public_method", rule!(allow_all))
                .method("guarded_method", rule!(require(guardian_badge.resource_address())))
                .method("guarded_protected_method", rule!(require(guardian_badge.resource_address())
                    && require(protector_badge.resource_address())))
                .method("schedule_example_callbacks", rule!(require(admin_badge.resource_address())))
                .method("cancel_callback", rule!(require(admin_badge.resource_address())))
                .method("authorize_callback", rule!(allow_all));

            let component =  Self {
                admin_badge: admin_badge.resource_address(),
                guardian_badge: Vault::with_bucket(guardian_badge),
                protector_badge: Vault::with_bucket(protector_badge),
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

        /// This method is part of the SchedulerComponent's API amd must be implemented as part of
        /// the integration. It will be called just before the actual method call (described in the
        /// callback) will be performed. It is used to authenticate the callback and to also
        /// authorize the method call by handing out any required proofs to the scheduler component.
        ///
        /// This is the only method that must be implemented as part of the scheduler integration.
        ///
        /// # Arguments:
        /// * `callback`: A proof containing a single Callback NFR. This demonstrates that the caller
        /// is the same party
        /// that the callback was scheduled with.
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
            let proofs = match callback.non_fungible::<Callback>().data().method.as_str() {
                "public_method" => vec![], // Method requires no auth
                "guarded_method" => vec![self.guardian_badge.create_proof()], // Method requires single proof
                "guarded_protected_method" => vec![ // Method requires multiple proofs
                    self.guardian_badge.create_proof(),
                    self.protector_badge.create_proof()],
                m => panic!("Unhandled method: {}", m)
            };

            // Return the CallbackHandle and any proofs to the scheduler component
            (callback_handle, proofs)
        }


        /// A dummy method for demo purposes
        ///
        /// This method shows that we can schedule callbacks to methods
        /// that require no authorization and return nothing.
        pub fn public_method(&mut self, name: String) {
           info!("Hello {}", name);
        }

        /// A dummy method for demo purposes
        ///
        /// This method shows that we can schedule callbacks to methods
        /// that require a single proof for authorization and that return a Scrypto primitive value.
        /// Of course that return value will be ignored when called via a callback.
        pub fn guarded_method(&mut self, receiver: ComponentAddress, amount: Decimal)  -> Decimal {
            info!("Sending {} XRD to {}", amount, receiver);
            amount * 2
        }

        /// A dummy method for demo purposes
        ///
        /// This method shows that we can schedule callbacks to methods
        /// that require a multiple proofs for authorization and that return a complex value.
        pub fn guarded_protected_method(&mut self, question: Question) -> Answer {
            info!("The question is: {}", question.0);
            info!("The answer is: 42");
            Answer(42)
        }

        /// This method schedules several demo callbacks
        ///
        /// This demonstrates how callbacks can be scheduled from within a component.
        ///
        /// Of course it is also possible to schedule callbacks from outside a component through the
        /// use of the transaction manifest. In that case the CallbackHandle must be deposited into the
        /// callee component, as it is required to authorize the callback.
        ///
        /// Scheduling the callbacks this way from inside the component is mainly done because it is
        /// easier than doing it through the transaction manifest
        pub fn schedule_example_callbacks(&mut self, fee: Bucket) -> Bucket {
            let current_component = Runtime::actor().component_address().unwrap();

            // Schedule a 1st example callback
            let(callback_handle, fee) = CallbackRequest::new(
                Trigger::AtEpoch(1), current_component, "public_method", args!("Satoshi"), None)
                .schedule_callback(self.scheduler_component, fee);
            self.callback_handles.put(callback_handle);
            debug!("");

            // Schedule a 2nd example callback
            let(callback_handle, fee) = CallbackRequest::new(
                Trigger::AtEpoch(1),
                current_component,
                "guarded_method",
                args!(Runtime::actor().component_address().unwrap(), dec!("1000")),
                None)
                .schedule_callback(self.scheduler_component, fee);
            self.callback_handles.put(callback_handle);
            debug!("");

            // Schedule a 3rd example callback
            let(callback_handle, fee) = CallbackRequest::new(
                Trigger::AtDateTime {
                    date_time: "2022-10-1T12:42+00:00".to_owned(),
                    tolerance_seconds: 60
                },
                current_component,
                "guarded_protected_method",
                args!(Question("What do you get if you multiply six by nine?".to_owned())),
                None)
                .schedule_callback(self.scheduler_component, fee);
            self.callback_handles.put(callback_handle);
            debug!("");

            // Schedule a 4th example callback
            let(callback_handle, fee) = CallbackRequest::new(
                Trigger::AtEpoch(10), current_component, "public_method", args!("Vitalik"), None)
                .schedule_callback(self.scheduler_component, fee);
            self.callback_handles.put(callback_handle);
            debug!("");

            // Schedule a 5th example callback
            let(callback_handle, fee) = CallbackRequest::new(
                Trigger::AtEpoch(10), current_component, "public_method", args!("Sam"), None)
                .schedule_callback(self.scheduler_component, fee);
            self.callback_handles.put(callback_handle);

            fee
        }

        /// Cancels a callback with the given ID
        pub fn cancel_callback(&mut self, callback_id: NonFungibleId) {
            let callback_handle = self.callback_handles.take_non_fungible(&callback_id);
            borrow_component!(self.scheduler_component)
                .call("cancel_callbacks", args!(callback_handle))
        }
    }
}

/// Dummy struct
#[derive(scrypto::Encode, scrypto::Decode, scrypto::TypeId, scrypto::Describe)]
pub struct Question(String);

/// Dummy struct
#[derive(scrypto::Encode, scrypto::Decode, scrypto::TypeId, scrypto::Describe)]
pub struct Answer(u32);
