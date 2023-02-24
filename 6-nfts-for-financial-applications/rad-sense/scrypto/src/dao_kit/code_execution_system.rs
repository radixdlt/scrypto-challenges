use super::component_address_repo::*;
use scrypto::prelude::*;
use std::ops::DerefMut;

blueprint! {

    /// A component that can execute arbitrary method and function calls. If privileged methods
    /// should be called, the proper badges must be placed in this component.
    struct CodeExecutionSystem {
        /// Internal minting authority
        minter: Vault,
        /// Address of a resource that represents an AuthorizedCodeExecution
        code_execution_resource: ResourceAddress,
        /// Any access badges that might be needed to call privileged methods
        access_badges: KeyValueStore<ResourceAddress, Vault>,

        /// Workaround for bug https://github.com/radixdlt/radixdlt-scrypto/issues/483
        component_address_repo: ComponentAddress,
    }

    impl CodeExecutionSystem {

        /// Instantiates a new CodeExecutionSystem component
        ///
        /// Arguments:
        /// `access_badges`: Any access badges that might be needed to call privileged methods
        /// `component_address_repo`: The address of a ComponentAddressRepo
        ///
        /// Returns:
        /// - The newly instantiated CodeExecutionSystemComponent
        /// - The resource address of the AuthorizedCodeExecution NFR
        pub fn instantiate(
            access_badges: Vec<Bucket>,
            component_address_repo: ComponentAddress,
        ) -> (CodeExecutionSystemComponent, ResourceAddress) {
            let minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!(1));
            let code_execution_resource = ResourceBuilder::new_non_fungible()
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .restrict_deposit(rule!(deny_all), LOCKED)
                .metadata("name", "Code Execution Token")
                .no_initial_supply();

            let mut abs = KeyValueStore::new();
            for access_badge in access_badges {
                let badge_address = access_badge.resource_address();
                match abs.get_mut(&badge_address) {
                    None => abs.insert(badge_address, Vault::with_bucket(access_badge)),
                    Some(mut vault) => vault.deref_mut().put(access_badge),
                }
            }

            let component = Self {
                minter: Vault::with_bucket(minter),
                code_execution_resource,
                access_badges: abs,
                component_address_repo,
            }
            .instantiate();

            (component, code_execution_resource)
        }

        /// Instantiates a new CodeExecutionSystem component and globalizes it
        ///
        /// Arguments:
        /// `access_badges`: Any access badges that might be needed to call privileged methods
        /// `admin_badge`: The address of an access badge that should be able to call the
        /// CodeExecutionSystem's privileged methods
        /// `component_address_repo`: The address of a ComponentAddressRepo
        ///
        /// Returns:
        /// - The global address of the newly instantiated CodeExecutionSystemComponent
        /// - The resource address of the AuthorizedCodeExecution NFR
        pub fn instantiate_global(
            access_badges: Vec<Bucket>,
            admin_badge: ResourceAddress,
            component_address_repo: ComponentAddress,
        ) -> (ComponentAddress, ResourceAddress) {
            let rules = AccessRules::new()
                .method("authorize_code_execution", rule!(require(admin_badge)))
                .method("execute_code", rule!(allow_all))
                .method("add_access_badge", rule!(allow_all));

            let (mut component, code_execution_resource) = Self::instantiate(access_badges, component_address_repo);
            component.add_access_check(rules);

            (component.globalize(), code_execution_resource)
        }

        /// "Authorizes" the given `code_executions`. This essential wraps them in a
        /// [AuthorizedCodeExecution] NFR which is returned to the caller. The caller must then
        /// call the `execute_code` method with this NFR within the same transaction. This style of
        /// doing things is needed to avoid reentrancy, which might happen in certain cases and
        /// is prohibited under the rules of the RadixEngine.
        ///
        /// If the user fails to call the `execute_code` method, the transaction will fail as the
        /// AuthorizedCodeExecution NFR cannot be deposited!
        ///
        /// Arguments:
        /// - `code_executions`: A vec of all [CodeExecution]s that should be authorized
        ///
        /// Returns: a bucket with the AuthorizedCodeExecution NFR
        pub fn authorize_code_execution(&self, code_executions: Vec<CodeExecution>) -> Bucket {
            self.minter.authorize(|| {
                let rm = borrow_resource_manager!(self.code_execution_resource);
                rm.mint_non_fungible(&NonFungibleId::random(), AuthorizedCodeExecution { code_executions })
            })
        }

        /// Executes the [CodeExecution]s contained in the supplied [AuthorizedCodeExecution] NFR
        ///
        /// Arguments:
        /// - `authorized_code_execution`: A bucket containing a single AuthorizedCodeExecution NFR
        pub fn execute_code(&self, authorized_code_execution: Bucket) {
            // Validate that the authorized_code_execution bucket contains the right resource and
            // amount
            authorized_code_execution
                .create_proof()
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.code_execution_resource,
                    dec!(1),
                ))
                .expect("Invalid code execution resource/amount");

            // Extract the code execution that should be run
            let code_executions = authorized_code_execution
                .non_fungible::<AuthorizedCodeExecution>()
                .data()
                .code_executions;

            // Execute each
            for code_execution in code_executions {
                code_execution.execute(&self.access_badges, self.component_address_repo);
            }

            // Burn the AuthorizedCodeExecution NFR
            self.minter.authorize(|| {
                let rm = borrow_resource_manager!(self.code_execution_resource);
                rm.burn(authorized_code_execution);
            });
        }

        /// Adds an access badge to enable the invocation of further privileged methods
        ///
        /// Arguments:
        /// `access_badge`: The new badge to add
        pub fn add_access_badge(&mut self, access_badge: Bucket) {
            let badge_address = access_badge.resource_address();
            match self.access_badges.get_mut(&badge_address) {
                None => self
                    .access_badges
                    .insert(badge_address, Vault::with_bucket(access_badge)),
                Some(mut vault) => vault.deref_mut().put(access_badge),
            };
        }

        pub fn code_execution_resource(&self) -> ResourceAddress {
            self.code_execution_resource
        }
    }
}

/// Represents a method or a function that may be executed
#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub enum CodeExecution {
    /// Represents a method call
    MethodCall {
        /// The target component
        component: ComponentAddressLookup,
        /// The target method
        method: String,
        /// Any args that the method should be called with. Must be encoded as bytes using the
        /// args!() macro
        args: Vec<u8>,
        /// Any badges that must be presented when calling the method
        required_badges: Vec<ResourceAddress>,
    },
    FunctionCall {
        /// The target package
        package: PackageAddress,
        /// The target blueprint
        blueprint: String,
        /// The target fucntion
        function: String,
        /// Any args that the function should be called with. Must be encoded as bytes using the
        /// args!() macro
        args: Vec<u8>,
    },
}

impl CodeExecution {
    pub(crate) fn execute(
        &self,
        available_badges: &KeyValueStore<ResourceAddress, Vault>,
        component_address_repo: ComponentAddress,
    ) {
        match self {
            CodeExecution::MethodCall { component, method, args, required_badges } => {
                let component_address_repo: ComponentAddressRepoComponent = component_address_repo.into();
                let component = component_address_repo.lookup_address(component.to_owned());
                Self::authorize(available_badges, required_badges, || {
                    Runtime::call_method::<&str, ()>(component, method, args.to_owned());
                });
            }

            CodeExecution::FunctionCall { package, blueprint, function, args } => {
                Runtime::call_function::<&str, ()>(*package, blueprint, function, args.to_owned());
            }
        }
    }

    fn authorize<F>(
        available_badges: &KeyValueStore<ResourceAddress, Vault>,
        required_badges: &Vec<ResourceAddress>,
        block: F,
    ) where
        F: FnOnce(),
    {
        for required_badge in required_badges {
            let proof = available_badges.get(required_badge).expect("Required badge is not available").create_proof();
            ComponentAuthZone::push(proof);
        }
        block();
        for _ in required_badges {
            ComponentAuthZone::pop().drop();
        }
    }
}

#[derive(NonFungibleData)]
struct AuthorizedCodeExecution {
    code_executions: Vec<CodeExecution>,
}
