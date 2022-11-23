use crate::code_execution_system::*;
use crate::component_address_repo::*;
use crate::membership_system::*;
use crate::utils::IntoComponent;
use crate::voting_system::*;
use scrypto::prelude::*;

blueprint! {

    /// The individual blueprints in the dao-kit toolkit can be combined freely. Advanced users
    /// may want to use multiple instances of the same blueprint in their components (e.g. two
    /// membership systems for different types of members).
    ///
    /// However, for simple use cases where one membership system one voting system and one code
    /// execution system are required, the SimpleDaoSystem provides a ready to go solution.
    /// It performs the setup of all three systems and can easily be incorporated in a DAO
    /// blueprint a developer is building. That blueprint can then be kept free of most boilerplate
    /// functionality, like management of members and managing of votes/proposals, that DAOs
    /// typically require.
    ///
    /// Note that this DAO uses the non-fungible-based MembershipSystem. If membership is represented
    /// via fungible tokens, this component is not suitable!
    struct SimpleDaoSystem {
        /// The address of the MembershipSystem
        membership_system: ComponentAddress,
        /// The address of the CodeExecutionSystem
        code_execution_system: ComponentAddress,
        /// The address of the VotingSystem
        voting_system: ComponentAddress,
        /// The address of a ComponentAddressRepo
        /// Workaround for bug https://github.com/radixdlt/radixdlt-scrypto/issues/483
        component_address_repo: ComponentAddress,
        /// The address of the admin badge that is required by te privileged methods of all systems
        admin_badge_resource: ResourceAddress,
    }

    impl SimpleDaoSystem {

        /// Instantiates a new SimpleDaoSystem component and globalizes it
        ///
        /// Arguments:
        /// - `initial_members`: The initial members of the dao. This includes the members' account
        /// addresses so that membership badges can be sent directly to them without going through
        /// an intermediary that must be trusted.
        /// - `component_address_repo`: The address of a ComponentAddressRepo
        /// Workaround for bug https://github.com/radixdlt/radixdlt-scrypto/issues/483
        ///
        /// Returns:
        /// - The global address of the SimpleDaoSystem component
        /// - A bucket with one admin badge for all systems (membership, voting, code execution).
        /// This badge is intended to be stored safely in the component that uses this
        /// blueprint/component and to be kept there out of reach of any individual users.
        ///
        /// It is not intended that individual users get access to this badge as this would go against
        /// the core idea behind a democratically managed DAO!
        pub fn instantiate_global(
            initial_members: Vec<(DaoMember, ComponentAddress)>,
            component_address_repo: ComponentAddress,
        ) -> (DaoSystemAddresses, Bucket) {

            // Mint 3 admin badges, 2 of which are used internally to authorize the different systems
            // among each other. The final badge is given to the instantiator of this component.
            let mut admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "DAO Admin Badge")
                .initial_supply(dec!(3));

            // Set up the membership system
            let (membership_system, membership_resource) =
                MembershipSystemComponent::instantiate_global(admin_badge.resource_address(), component_address_repo);
            let msc: MembershipSystemComponent = membership_system.into();
            // And add the initial members
            for (member, member_account_address) in initial_members {
                admin_badge.authorize(|| {
                    let repo: ComponentAddressRepoComponent = component_address_repo.into();
                    let member_account_address_lookup = repo.create_lookup(member_account_address);
                    msc.add_member_and_distribute_badge(member.name, member.data, member_account_address_lookup)
                });
            }

            // Set up code execution system
            let (code_execution_system, code_execution_resource) = CodeExecutionSystemComponent::instantiate_global(
                // Supply the admin badge to the code execution system so that it can call the membership system
                // and the voting system
                vec![admin_badge.take(dec!(1))],
                admin_badge.resource_address(),
                component_address_repo,
            );

            // Set up the voting system
            let (voting_system, vote_resource, vote_receipt_resource) = VotingSystemComponent::instantiate_global(
                // Enable the voting system to securely talk to the code execution system
                Some((code_execution_system, admin_badge.take(dec!(1)))),
                admin_badge.resource_address(),
            );

            let access_rules = AccessRules::new()
                .method("dao_system_addresses", rule!(allow_all));

            // Instantiate the SimpleDaoSystem
            let mut component = Self {
                membership_system,
                code_execution_system,
                voting_system,
                component_address_repo,
                admin_badge_resource: admin_badge.resource_address()
            }
            .instantiate();
            component.add_access_check(access_rules);

            // Globalize the SimpleDaoSystem
            let dao_system_component = component.globalize();

            // Compile a struct with all relevant addresses that are associated with the SimpleDaoSystem
            let dao_addresses = DaoSystemAddresses {
                dao_system_component,
                membership_system_component: membership_system,
                code_execution_system_component: code_execution_system,
                voting_system_component: voting_system,
                membership_resource,
                code_execution_resource,
                vote_resource,
                vote_receipt_resource,
                dao_system_admin_badge_resource: admin_badge.resource_address()
            };

            (dao_addresses, admin_badge)
        }

        /// Returns a struct with all relevant addresses that are associated with the SimpleDaoSystem
        pub fn dao_system_addresses(&self) -> DaoSystemAddresses {
            DaoSystemAddresses {
                dao_system_component: Runtime::actor().as_component().0,
                membership_system_component: self.membership_system,
                code_execution_system_component: self.code_execution_system,
                voting_system_component: self.voting_system,
                membership_resource: self
                    .membership_system
                    .into_component::<MembershipSystemComponent>()
                    .membership_resource(),
                code_execution_resource: self
                    .code_execution_system
                    .into_component::<CodeExecutionSystemComponent>()
                    .code_execution_resource(),
                vote_resource: self
                    .voting_system
                    .into_component::<VotingSystemComponent>()
                    .vote_resource(),
                vote_receipt_resource: self
                    .voting_system
                    .into_component::<VotingSystemComponent>()
                    .vote_receipt_resource(),
                dao_system_admin_badge_resource: self.admin_badge_resource,
            }
        }
    }
}

/// A struct to keep track of all relevant dao system addresses
#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub struct DaoSystemAddresses {
    pub dao_system_component: ComponentAddress,
    pub membership_system_component: ComponentAddress,
    pub code_execution_system_component: ComponentAddress,
    pub voting_system_component: ComponentAddress,
    pub membership_resource: ResourceAddress,
    pub code_execution_resource: ResourceAddress,
    pub vote_resource: ResourceAddress,
    pub vote_receipt_resource: ResourceAddress,
    pub dao_system_admin_badge_resource: ResourceAddress,
}

