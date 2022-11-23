# callback-scheduler

This package implements a simple callback scheduling component which functions as a specific type of oracle. It allows
users to schedule callbacks to their own component methods that will happen at some specified time in the future.

# tl;dr - just show me something

[See this](./scrypto) if you are interested in the scrypto implementation.  
[Go here](./pte-demo) if you want to fire up a local webserver and click through a demo in the Babylon PTE.

# Motivation & design goals

On DLT technologies it is generally not possible to schedule method calls at a later time. Everything that happens
on-chain must happen in a transaction and that transaction must be relatively short-lived. It is just not possible to
call methods like `sleep(100)` or set a timer in a background thread that calls some method later. Consequently, some
off-chain process is needed that calls the on-chain methods at the right time. Many people and projects will of course
want to implement such a process themselves, especially if they want to remain in full control of when their methods
will be called.  
However, some people will not want to go through this trouble and will rather choose to use a scheduler process that is
provided as a service by a third party. A key challenge with this is that most methods that one would like to schedule
will have some access restrictions applied to them. Therefore, a safe and efficient mechanism must be devised that
allows the scheduling operator to call such restricted methods and to only call methods that were explicitly scheduled.
This package seeks to implement such a mechanism in the form of the CallbackScheduler blueprint.

The overarching design goal of this package is to provide a scheduling mechanism that is not only safe but also easy to
use. Additionally, it should be possible and relatively simple to integrate scheduling with existing blueprints, which
cannot be changed.

# Simplifying assumptions

- Because transaction fees are not yet implemented as of Scrypto v0.4.1, they were disregarded in this design. Once fees
  have been implemented, the design will have to be revisited.
- As a consequence the design gives not much attention to fees in general and just charges a flat fee per execution. If
  a scheduled callback is canceled, fees are not reimbursed at the moment. That is of course something that one might
  want to reconsider in the future.
- It is assumed that there exist a process running on e.g. AWS that periodically checks the Radix network for newly
  scheduled callbacks and then executes them at the appropriate time via a transaction.

# Mechanism

The CallbackScheduler blueprint heavily relies on Radix's resource centric nature. Callbacks with all their properties
(e.g. the component and method to call, any method arguments and the call time) are stored as non-fungible resources
(NFRs) on-chain. Specifically, they are stored as instances of the `Callback` NFR. Whenever a `Callback` NFR is minted,
two additional NFRs are minted: 1) a `CallbackHandle` that is given to the user and a `CallbackAdminHandle` that is
given to the admin/operator of the SchedulerComponent. Both these NFRs can - among other things - be used by the
respective party to cancel the callback. When a callback is canceled or executed, all three NFRs are eventually burned.

The CallbackScheduler allows users to schedule callbacks on arbitrary methods as long as these methods neither consume
nor produce resources. It is perfectly valid if a method takes a `Decimal` as an argument or returns one. However, it
can, neither specify a `Bucket` as an argument nor produce one.

## Integration of the CallbackScheduler

Integrating with the CallbackScheduler component is relatively simple as it essentially requires the implementation of
only a single method. This is the `authorize_callback` method:

```rust
fn authorize_callback(&mut self, callback: Proof) -> (Bucket, Vec<Proof>) {
    ...
}
```

This method accepts a proof of the `Callback` NFR as it's single argument, which it can use to verify that the caller of
the method is indeed the CallbackScheduler component and not some other party. It returns a bucket containing the
corresponding `CallbackHandle` NFR as well as a vec of proofs that will be needed to call the scheduled method. If the
scheduled method does not require any authorization, an empty vec may be return. Note however, that the
`authorize_callback` will still need to be implemented in that case! This is because there needs to be a way to pass
the `CallbackHandle` back to the CallbackScheduler component, so it can be burned. Otherwise, the user would be left
with an orphaned `Callbackhandle` that would suggest to them that the callback had not happened yet.  
Once recallable resource are implemented in Scrypto, this aspect of the design should be revisited. At that point it
should be possible to simply recall the `CallbackHandle` from a component's vault or from a user's account without their
cooperation. This will make it even simpler to integrate with methods that do not require any authorization.

Please note that there is no requirement that the component implementing the `authorize_callback` method must also be
the component that is the target of the callback (i.e. has the method that gets called). You can also integrate the
CallbackScheduler component such that the component that is the target of the callback does not handle authorization
itself but instead delegates this obligation to another component. This is especially useful if you want to integrate
with existing components to which you simply cannot add the `authorize_callback` method or if you want to maintain a
separation of concerns. In that case you would instantiate another component that only implements the
`authorize_callback` method and equip that component with the required badges.

# Workflow

The following steps describe how a typical workflow from scheduling a callback to its execution looks like:

1. The `schedule_callback(&mut self, callback_request: CallbackRequest, mut fee: Bucket) -> (Bucket, Bucket) `
   method is called, either by a component or by a user. In any case the caller makes sure that the
   received `CallbackHandle` NFR (1st bucket) ends up in the component implementing the `authorize_callback` method.
2. The admin/operator of the CallbackScheduler component periodically polls their component for newly scheduled
   callbacks using the `get_new_callback_admin_handles(&mut self) -> Bucket`. The returned bucket contains
   the `CallbackAdminHandle`s that have been created since the last invocation of the method. The operator must feed
   these callbacks into their off-chain system, such that it will execute an appropriate transaction at the right time.
3. The off-chain system decides that the right moment has come to execute a callback. It executes a transaction that
   calls the `execute_callback(&mut self, callback_admin_handle: Bucket)` handing it the appropriate
   `CallbackAdminHandle`. The `execute_callback` method then
    1. calls the `authorize_callback` method on the component that handles authorization and presents a proof of
       the `Callback` NFR to it. In return, it receives the `CallbackHandle` NFR and any proofs required by the
       scheduled method.
    2. calls the scheduled method passing it all proves it has previously received. Finally, it burns all three NFRs,
       the `Callback`, the `CallbackHandle` and the `CallbackAdminHandle`.

# Safety

In a permission- and trustless environment, such as on a DLT, safety is always a big concern. This is doubly true in
this case, where the CallbackScheduler as a third party component should be allowed to call protected methods on one's
own components. However, if the right care is taken, integrating with the CallbackScheduler should be totally safe.

Whether the integration is secure, stands and falls with whether an authentic CallbackScheduler component is connected.
The code of this component guarantees that exactly the scheduled methods and only the scheduled methods are called and
that any borrowed proofs are not misused. Each implementer should therefore carefully check the code of the component to
be connected and ensure that it is the code in this package. In addition to this very basic requirement, the only
condition that must be met is the following:
A call to the `Callback::verify` function must be made in the `authorize_callback` method. This function ensures that
the caller is actually the CallbackScheduler component and not some other party. If this check is omitted, an attacker
could just send a proof of some NFR that pretends to be a `Callback` but is in fact some totally different NFR.  
Here is the most basic implementation of the `authorize_callback` method:

```rust
pub fn authorize_callback(&mut self, callback: Proof) -> (Bucket, Vec<Proof>) {
    // Verify the callback is authentic. It is VERY IMPORTANT that this method is called here!
    let callback_handle = Callback::verify(
        // The methods requires a reference to the callback proof
        &callback,
        // And also a closure that can take a callback ID and produce a bucket
        // containing the corresponding CallbackHandle.
        // In this case we are taking the CallbackHandle out of a vault within our component
        |callback_id| self.callback_handles.take_non_fungible(callback_id)
    );

    // Create a vec with one proof
    // This proof will be used by the scheduler component to call the target method
    let proofs = vec![self.admin_badge.create_proof()];

    // Return the CallbackHandle and any proofs to the scheduler component
    (callback_handle, proofs)
}
```

One can observe that the `authorize_callback` method takes a proof of the `Callback` NFR as its argument. This NFR is
only ever handled by the CallbackScheduler component and never by its admin/operator. If a proof of a `Callback`
NFR is received and the receiving component verifies that it owns the corresponding `CallbackHandle` NFR, it can be sure
that it is in fact being called by an authentic CallbackScheduler component and - equally importantly - is called with
proof of a callback that it has previously scheduled.

Naturally, the `authorize_callback` method could also access the callback information and perform some additional checks
and, if it wanted to prevent execution of the scheduled method for some reason, could do so by panicking. In that case,
the admin/operator of the CallbackScheduler component should cancel the callback from their end.

As you may have noticed, the `Callback::verify` function is a piece of code that needs to be imported into an
implementing blueprint/project as a library. This is another point were good care should be taken that the right code is
imported. Using a fake library voids all safety guarantees!

You may wonder why the `authorize_callback` method does not simply execute the callback itself, but instead produces
proofs and hands them to the CallbackScheduler component so that that component can then execute the callback. This is
because a method in a component cannot easily call another method in the same component in a generic way. Such behavior
is prevented by the Radix engine's reentrancy protection.

# Execution guarantees and limitations

Callbacks can be scheduled with the following triggers:

```rust
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
```

The current implementation of the CallbackScheduler grants that a callback that was scheduled for a certain epoch can
only be executed in exactly this epoch - not earlier and not later. A callback scheduled for a specific time or based on
the occurrence of a condition can *currently* be executed at any time. These two triggers are included more for
illustrative purposes. In a future version a time oracle could be connected, so that execution at a certain time can
also be guaranteed.

What cannot be guaranteed is that the execution actually takes place. Theoretically, the admin/operator of the scheduler
can always decide to skip execution of a callback. Of course, it is in the commercial interest of the operator to act
dutifully here, since he offers scheduling as a service for which he wants to retain paying customers.
