use crate::utils;
use scrypto::prelude::*;
use std::fmt::Display;

/// Key for a resource metadata field that stores the address of the Callback resource
const METADATA_CALLBACK_RESOURCE_ADDRESS: &str = "callback_resource_address";

blueprint! {

    /// A component that users can use to schedule callbacks to methods in their own components
    struct CallbackScheduler {
        /// Authority for minting NFRs (i.e. Callback, CallbackHandle, CallbackAdminHandle)
        minter: Vault,

        /// A vault containing one Callback NFR for each callback that is currently scheduled
        scheduled_callbacks: Vault,

        /// A vault that stores all newly created CallbackAdminHandles.
        /// Whenever a user schedules a callback, an admin handle for that callback will be
        /// put into this vault. The operator of this component must retrieve these handles
        /// before being able to execute the callbacks.
        new_callback_admin_handles: Vault,

        /// The address of the CallbackHandle NFRs that are given to users after scheduling
        /// a callback.
        callback_handle_resource: ResourceAddress,

        /// A vault for collecting fees
        fees: Vault,

        /// The amount in XRD that is charged for scheduling a callback
        fee_amount: Decimal
    }

    impl CallbackScheduler {

        /// Instantiate a new CallbackScheduler component
        /// # Arguments
        /// * `fee` - The fee to take for each scheduling
        pub fn instantiate_callback_scheduler(fee_amount: Decimal) -> (ComponentAddress, Bucket) {

            // Define an admin badge. This will be used to administer the component
            let admin_badge = ResourceBuilder::new_fungible()
                .metadata("name", "CallbackScheduler Admin Badge")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(Decimal::ONE);

            // Define the authority that will be used internally for minting NFRs
            let minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(Decimal::ONE);

            // Define the resource that will represent a callback.
            // This resource will only be held in this component
            let callback_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", "Callback")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .no_initial_supply();

            // Define a resource that can be given to the admin/operator of
            // this component and that allows them to manage an associated callback,
            // i.e. execute or cancel it
            let callback_admin_handle_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", "CallbackAdminHandle")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .no_initial_supply();

            // Define a resource that can be given to the user of this component
            // and that allows them to cancel a callback from their end
            let callback_handle_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", "CallbackHandle")
                // Store the address of the Callback resource in a metadata field.
                // Client components will need this information so they can verify the
                // validity of a Callback proof
                .metadata(METADATA_CALLBACK_RESOURCE_ADDRESS, callback_resource.to_string())
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .no_initial_supply();

            utils::debug_log_resources!(admin_badge, callback_resource,
                callback_admin_handle_resource, callback_handle_resource);

            // Define the access rules to the component's methods
            let rules = AccessRules::new()
                .method("schedule_callback", rule!(allow_all))
                .method("get_new_callback_admin_handles",
                    rule!(require(admin_badge.resource_address())))
                .method("execute_callback", rule!(allow_all))
                .method("cancel_callbacks", rule!(allow_all))
                .method("get_callback_handle_resource", rule!(allow_all))
                .method("withdraw_fees", rule!(require(admin_badge.resource_address())));

            // Instantiate the component
            let component = Self {
                minter: Vault::with_bucket(minter),
                scheduled_callbacks: Vault::new(callback_resource),
                new_callback_admin_handles: Vault::new(callback_admin_handle_resource),
                callback_handle_resource,
                fees:Vault::new(RADIX_TOKEN),
                fee_amount
            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            (component, admin_badge)
        }

        /// Schedule a new callback
        ///
        /// # Arguments:
        ///
        /// * `callback_request` A struct that describes the callback that should be scheduled
        /// * `fee` A bucket with the fee in XRD that is being payed for the scheduling service
        ///
        /// # Returns:
        /// * a bucket containing a CallbackHandle that may be used to cancel the callback
        /// * a bucket containing any XRD that are left in the fee bucket after deducting the payment
        pub fn schedule_callback(&mut self, callback_request: CallbackRequest, mut fee: Bucket)
            -> (Bucket, Bucket) {

            // First, validate the callback request
            callback_request.assert_valid();

            // Mint a new Callback NFR
            // The same callback ID is used for the Callback, the CallbackHandle and the
            // CallbackAdminHandle NFRs
            let callback_id = NonFungibleId::random();
            let callback_data = Callback::from_request(callback_id.clone(), &callback_request);
            debug!("Scheduling {}", callback_data);
            let callback = self.minter.authorize(|| {
                let rm = borrow_resource_manager!(self.scheduled_callbacks.resource_address());
                rm.mint_non_fungible(&callback_id, callback_data)
            });
            // Store the Callback NFR inside this component
            self.scheduled_callbacks.put(callback);

            // Mint a CallbackAdminHandle NFR that can be given to the admin/operator of this
            // component. The CallbackAdminHandle has the same ID as the Callback
            let callback_admin_handle = self.minter.authorize(|| {
                let rm = borrow_resource_manager!(self.new_callback_admin_handles.resource_address());
                rm.mint_non_fungible(&callback_id, CallbackAdminHandle::from_request(&callback_request))
            });
            utils::debug_log_non_fungible("Minted CallbackAdminHandle", &callback_admin_handle);
            // Store the CallbackAdminHandle NFR inside this scheduler component until it is
            // retrieved by the scheduler operator
            self.new_callback_admin_handles.put(callback_admin_handle);

            // Mint a CallbackHandle NFR that can be given to the caller.
            // The CallbackHandle has the same ID as the Callback
            let callback_handle = self.minter.authorize(|| {
                let rm = borrow_resource_manager!(self.callback_handle_resource);
                rm.mint_non_fungible(&callback_id, CallbackHandle {})
            });
            utils::debug_log_non_fungible("Minted CallbackHandle", &callback_handle);

            // Collect the fee
            self.fees.put(fee.take(self.fee_amount));
            // Return the callback handle as well as any change to the caller
            (callback_handle, fee)
        }

        /// Retrieves all CallbackAdminHandle NFRs that were created after the last call to
        /// this method
        ///
        /// # Returns
        /// A bucket with all new CallbackAdminHandle NFRs
        pub fn get_new_callback_admin_handles(&mut self) -> Bucket {
            self.new_callback_admin_handles.take_all()
        }

        /// Execute the given callback
        ///
        /// This method has multiple failure modes to which an admin/operator must react:
        /// * The callback is ill-defined and e.g. references a non existing method.
        /// In this case the callback should be canceled in a separate transaction.
        /// * The callback is well-defined but still fails e.g. because it's trigger
        /// condition is not met (e.g. executed in the wrong epoch). In this case the
        /// admin/operator must determine if the trigger condition can possibly be met
        /// at a later time and if so, call this method again at such an appropriate time.
        /// If the condition cannot possibly be met anymore, they should cancel the callback
        /// in a separate transaction.
        /// * The target component fails to authorize the callback. In this case the admin/operator
        /// should cancel the callback in a separate transaction.
        /// * The callback may have been canceled by the user. Because recallable resources
        /// are not implemented as of Scrypto 0.4.0, the CallbackAdminHandle NFR that is
        /// associated with the canceled callback still remains in the admins/operators
        /// account. If this happens, the admin/operator should cancel the callback in a
        /// separate transaction.
        ///
        /// # Arguments:
        /// * `callback_admin_handle` - A bucket containing the CallbackAdminHandle that is
        /// associated with the callback that should be executed.
        pub fn execute_callback(&mut self, callback_admin_handle: Bucket) {
            // Make sure we have been given a CallbackAdminHandle NFR
            utils::assert_resource_eq!(callback_admin_handle, self.new_callback_admin_handles);

            // Get the callback ID
            let callback_id = callback_admin_handle.non_fungible::<CallbackHandle>().id();

            // TODO: This check will not be necessary once recallable resources are implemented and
            // when the user cancels a callback, the CallbackAdminHandle will be recalled from
            // the operator's wallet
            // Also see the documentation on method cancel_callbacks
            assert!(
                self.scheduled_callbacks.non_fungible_ids().contains(&callback_id),
                "Callback was canceled by the user"
            );

            // Retrieve the callback NFR from the internal vault
            let callback = self.scheduled_callbacks.take_non_fungible(&callback_id);
            let callback_data = callback.non_fungible::<Callback>().data();
            debug!("Executing {}", callback_data);

            // Assert that the callback can be executed now
            callback_data.trigger.assert_valid_now();

            // Authorize the callback we want to perform with the callee component
            // This will yield us 1) the callee's callback_handle which we must burn
            // after executing the callback and 2) any proofs that are expected by
            // the method that should be called. If no explicit auth_provider has been configured
            // for the callback, assume that the callee component itself implements the
            // `authorize_callback` method.
            let auth_provider = callback_data.auth_provider.unwrap_or(callback_data.component);
            let auth_provider = borrow_component!(auth_provider);
            let (callback_handle, proofs) = auth_provider
                .call::<(Bucket, Vec<Proof>)>("authorize_callback", args!(callback.create_proof()));

            // Assert that the callee component has given us the correct CallbackHandle
            utils::assert_resource_eq!(callback_handle, self.callback_handle_resource);
            assert_eq!(callback_id, callback_handle.non_fungible::<CallbackHandle>().id(),
                "Invalid CallbackHandle: the id of the provided CallbackHandle does not \
                match that of the Callback"
            );

            // Execute the callback using the proofs we were given
            callback_data.execute_call(proofs);

            // Burn all three NFRs that are associated with the executed callback
            self.minter.authorize(|| {
                callback.burn();
                callback_handle.burn();
                callback_admin_handle.burn();
            });
        }

        /// Cancels all of the callbacks that are referenced by the supplied callback handles.
        /// This method can be called from either party, by the user as well as by the
        /// SchedulerComponent admin/operator.
        /// At the moment fees will not be reimbursed.
        ///
        /// Because resources are not yet recallable as of Scrypto 0.4.1,
        /// when one party cancels a callback, the Callback NFR will be burned but the corresponding
        /// CallbackHandle/CallbackAdminHandle NFR will remain in the other party's
        /// wallet. If such an "orphan" handle is then used to cancel the callback again,
        /// the Callback NFR will already have been burned. In that case, this method
        /// will only burn the passed "orphan" callback handle.
        ///
        /// Once recallable resources are implemented, when one party cancels a callback,
        /// this method should recall the other party's CallbackHandle/CallbackAdminHandle NFR
        /// and burn all three NFRs (Callback, CallbackHandle, CallbackAdminHandle).
        ///
        /// # Arguments
        /// * `callback_handles` - A bucket containing the CallbackHandle or CallbackAdminHandle
        /// NFRs of the callbacks that should be canceled
        pub fn cancel_callbacks(&mut self, callback_handles: Bucket) {
            // Determine the actual resource that was given to us and
            // assert that it is either a CallbackAdminHandle or CallbackHandle
            let handle_resource = callback_handles.resource_address();
            assert!(
                handle_resource == self.new_callback_admin_handles.resource_address()
                    || handle_resource == self.callback_handle_resource,
                "Invalid resource: expected the CallbackAdminHandle or CallbackHandle resource"
            );

            // Determine the IDs of the callbacks that should be canceled
            let callbacks_to_cancel = callback_handles.non_fungible_ids()
                // TODO Once recallable resources are implemented, we can prevent "orphan" handles
                // and this intersection computation will not be necessary
                .intersection(&self.scheduled_callbacks.non_fungible_ids())
                .cloned().collect();

            // Determine the IDs of the CallbackAdminHandles that have not yet been
            // retrieved by the component admin/operator and that now should be burned
            let callback_admin_handles_to_burn = callback_handles.non_fungible_ids()
                .intersection(&self.new_callback_admin_handles.non_fungible_ids())
                .cloned().collect();

            // Burn the associated Callback, CallbackHandle and CallbackAdminHandle NFRs
            self.minter.authorize(|| {
                callback_handles.burn();
                self.scheduled_callbacks.take_non_fungibles(&callbacks_to_cancel).burn();
                self.new_callback_admin_handles
                    .take_non_fungibles(&callback_admin_handles_to_burn).burn();
                // TODO Recall and burn the corresponding CallbackHandle/CallbackAdminHandle
                // of the other party once recallable resources are implemented
            });
        }

        /// Returns the resource address of the CallbackHandle NFR
        pub fn get_callback_handle_resource(&self) -> ResourceAddress {
            self.callback_handle_resource
        }

        /// Withdraw all fees that have been collected
        pub fn withdraw_fees(&mut self) -> Bucket {
            self.fees.take_all()
        }
    }
}

/// A trigger that defines when a callback should be executed.
#[derive(scrypto::Encode, scrypto::Decode, scrypto::TypeId, scrypto::Describe, Clone)]
pub enum Trigger {
    /// Execute the callback at a specific epoch.
    AtEpoch(u64),

    /// Execute the callback at a specific date and time
    AtDateTime {
        /// ISO time string
        date_time: String,

        /// On DLTs precise execution times cannot be guaranteed.
        /// This specifies the tolerated deviation from the given
        /// date_time when executing the callback.
        /// Note that at this moment no date_time verification is
        /// implemented. This is left for future work.
        tolerance_seconds: u8,
    },

    /// Execute the callback when a specific condition is met
    ///
    /// This is only in hear for demo purposes.
    /// At the moment there is no specification for OnCondition expressions.
    /// TODO - Future work: Implement an OnCondition specification
    OnCondition(String),
}

impl Trigger {
    /// Asserts that the trigger is valid now.
    /// This can for example be a check that we are in the right epoch or
    /// it also could be a check against a time oracle (not implemented yet).
    ///
    /// Panics if the trigger is not valid now
    fn assert_valid_now(&self) {
        match self {
            Trigger::AtEpoch(epoch) => {
                let now = Runtime::current_epoch();
                assert!(
                    *epoch == now,
                    "Invalid execution, trigger restriction violated: \
                execution is only allowed at epoch {}. Current epoch: {}",
                    epoch,
                    now
                );
            }
            Trigger::AtDateTime { .. } => (), // TODO - Future work: implement this check e.g. by calling a 3rd-party time oracle
            Trigger::OnCondition(_) => (), // TODO - Future work: implement an OnCondition specification and this check
        }
    }
}

impl Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AtEpoch(epoch) => write!(f, "AtEpoch({epoch})"),
            Self::AtDateTime {
                date_time,
                tolerance_seconds,
            } => write!(f, "AtDateTime({date_time} ±{tolerance_seconds}s)"),
            Self::OnCondition(condition) => write!(f, "OnCondition({condition})"),
        }
    }
}

/// Represents a request for a callback
#[derive(scrypto::Encode, scrypto::Decode, scrypto::TypeId, scrypto::Describe)]
pub struct CallbackRequest {
    /// When should the callback be triggered?
    trigger: Trigger,

    /// The target component of the callback
    component: ComponentAddress,

    /// The target method of the callback
    method: String,

    /// The args that should be passed to the target method
    args: Vec<Vec<u8>>,

    /// The component that will provide the necessary authorization (proofs)
    /// that is required by the method that should be called.
    /// If set to `None`, the target component itself will be used to obtain any necessary proofs.
    auth_provider: Option<ComponentAddress>,
}

impl CallbackRequest {
    /// Create a new CallbackRequest.
    ///
    /// # Arguments:
    /// * `trigger` - When should the callback be triggered?
    /// * `component` - The target component of the callback
    /// * `method` - The target component of the callback
    /// * `args` - The args that should be passed to the target method
    /// * `auth_provider` - The component that will provide the necessary authorization (proofs)
    /// that is required by the method that should be called.
    /// If set to `None`, the target component itself will be used to obtain any necessary proofs.
    pub fn new(
        trigger: Trigger,
        component: ComponentAddress,
        method: &str,
        args: Vec<Vec<u8>>,
        auth_provider: Option<ComponentAddress>,
    ) -> Self {
        Self {
            trigger,
            component,
            method: method.to_owned(),
            args,
            auth_provider,
        }
    }

    /// Validates the request, e.g. by checking that the trigger is not in the past
    /// Panics if the request is invalid
    pub fn assert_valid(&self) {
        match self.trigger {
            Trigger::AtEpoch(epoch) => assert!(
                epoch > Runtime::current_epoch(),
                "Callback execution must be in the future"
            ),
            _ => (), // TODO - Future work: Implement validations for the other triggers
        }
    }

    /// Schedules a callback for this request. This calls the scheduler component with this request
    /// object.
    ///
    /// # Arguments:
    /// * `scheduler_component` - The component that should execute the callback
    /// * `fee` - The fee in XRD that will be paid to the scheduler component
    ///
    /// # Returns
    /// Two buckets where:
    /// - the first bucket contains the CallbackHandle NFR that the callee component will hold on to
    /// - the second bucket contains any fee amount that was overpaid
    pub fn schedule_callback(
        &self,
        scheduler_component: ComponentAddress,
        fee: Bucket,
    ) -> (Bucket, Bucket) {
        let scheduler = borrow_component!(scheduler_component);
        let (callback_handle, fee) = scheduler.call::<(Bucket, Bucket)>("schedule_callback", args!(*self, fee));
        (callback_handle, fee)
    }
}

/// Represents a scheduled callback
#[derive(NonFungibleData)]
pub struct Callback {
    /// The ID of the callback, this is also the ID of the
    /// associated CallbackHandle and CallbackAdminHandle
    pub id: NonFungibleId,

    /// When should the callback be triggered?
    pub trigger: Trigger,

    /// The target component of the callback
    pub component: ComponentAddress,

    /// The target method of the callback
    pub method: String,

    /// The args that should be passed to the target method
    pub args: Vec<Vec<u8>>,

    /// The component that will provide the necessary authorization (proofs)
    /// that is required by the method that should be called.
    /// If set to `None`, the target component itself will be used to obtain any necessary proofs.
    auth_provider: Option<ComponentAddress>,
}

impl Callback {
    /// Create a new Callback NFR from the given requests object
    ///
    /// # Arguments:
    /// * `id` - The ID of the Callback NFR
    /// * `request` - The CallbackRequest from which to create the Callback
    pub fn from_request(id: NonFungibleId, request: &CallbackRequest) -> Self {
        Self {
            id,
            trigger: request.trigger.clone(),
            component: request.component,
            method: request.method.to_owned(),
            args: request.args.to_vec(),
            auth_provider: request.auth_provider,
        }
    }

    /// Executes the method call that is described by this callback object.
    ///
    /// # Arguments:
    /// * `proofs` - Any proofs that are required by the callee method
    pub fn execute_call(&self, proofs: Vec<Proof>) {
        // Put all proofs in the component's authorization zone
        let proof_count = proofs.len();
        for proof in proofs {
            ComponentAuthZone::push(proof);
        }

        // Actually execute the call
        Runtime::call_method(self.component, &self.method, self.args.to_vec());

        // Remove the proofs from the component's authorization zone
        for _ in 0..proof_count {
            ComponentAuthZone::pop().drop();
        }
    }

    /// Verify the validity of this Callback against a CallbackHandle. This method must be
    /// called from the client component to make sure that:
    /// - The Callback NFR is valid (correct resource)
    /// - The client component owns the corresponding ClientHandle NFR and is thus
    /// indeed the party that scheduled the callback.
    ///
    /// **It is vitally important that any callee component calls this method!**
    ///
    /// # Arguments:
    /// * `callback` - The callback to verify
    /// * `callback_handle_provider` - A function/closure that will produce
    /// the correct CallbackHandle when given the callback ID
    ///
    /// # Returns:
    /// A bucket with the CallbackHandle NFR
    pub fn verify<F>(callback: &Proof, callback_handle_provider: F) -> Bucket
    where
        for<'a> F: FnOnce(&'a NonFungibleId) -> Bucket,
    {
        // Determine the actual resource address of the supposed
        // Callback NFR that was given to this method
        let actual_callback_resource = callback.resource_address();
        // Load the CallbackHandle corresponding to this callback
        let callback = callback.non_fungible::<Callback>().data();
        let callback_handle = callback_handle_provider(&callback.id);
        // Assert that the IDs of the Callback NFR and CallbackHandle NFR match
        // This ensures that the callback_handle_provider is correctly implemented by the user
        assert_eq!(
            callback.id,
            callback_handle.non_fungible::<CallbackHandle>().id(),
            "Invalid CallbackHandle provided: id does not match that of the Callback"
        );

        // Load the metadata of the CallbackHandle resource and retrieve
        // the expected resource address that the Callback NFR must have.
        let metadata = borrow_resource_manager!(callback_handle.resource_address()).metadata();
        let expected_callback_resource = metadata.get(METADATA_CALLBACK_RESOURCE_ADDRESS).unwrap();
        let expected_callback_resource =
            ResourceAddress::from_str(expected_callback_resource).unwrap();

        // Assert that the actual resource address matches the expected resource address
        assert_eq!(
            actual_callback_resource, expected_callback_resource,
            "Invalid Callback resource"
        );

        // Return the CallbackHandle NFR
        callback_handle
    }
}

impl Display for Callback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Callback(method={}, component={}, trigger={})",
            &self.method, &self.component, &self.trigger
        )
    }
}

/// A handle for a callback that can be used by admins/operators of a CallbackScheduler component
/// to execute and cancel callbacks
#[derive(NonFungibleData)]
pub struct CallbackAdminHandle {
    // When should the callback be executed?
    trigger: Trigger,
}

impl CallbackAdminHandle {
    /// Instantiates a new CallbackAdminHandle from the given CallbackRequest
    ///
    /// # Arguments:
    /// * `request` - The request from which to construct the handle
    fn from_request(request: &CallbackRequest) -> Self {
        Self {
            trigger: request.trigger.clone(),
        }
    }
}

/// A handle for a callback that will be given to users of a CallbackScheduler component
/// and that they can use to verify incoming callback calls and to cancel callbacks.
#[derive(NonFungibleData)]
pub struct CallbackHandle {}

/// Helper method to easily retrieve the CallbackHandle resource address that is used by a specific
/// CallbackScheduler component.
///
/// # Arguments:
/// * `scheduler_component` - The address of the CallbackScheduler component that
/// should be queried for the CallbackHandle resource address
///
/// # Returns:
/// The component address of the CallbackHandle
pub fn get_callback_handle_resource(scheduler_component: ComponentAddress) -> ResourceAddress {
    let scheduler_component = borrow_component!(scheduler_component);
    scheduler_component.call("get_callback_handle_resource", args!())
}
