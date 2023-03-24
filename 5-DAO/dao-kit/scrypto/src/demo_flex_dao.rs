use scrypto::prelude::*;

use crate::code_execution_system::*;
use crate::component_address_repo::*;
use crate::simple_dao_system::*;
use crate::membership_system::*;
use crate::utils::IntoComponent;
use crate::voting_system::*;

blueprint! {

    /// A blueprint demonstrating the flexibility of the dao-kit toolkit
    /// Take a deeper look into the method at the bottom of this file.
    /// That's the interesting part!
    ///
    /// Also see the accompanying test in ../tests/demo_flex_dao_test.rs
struct FlexDao {
    dao_system_addresses: DaoSystemAddresses,
    admin_badge: Vault,
    usage_fee_pct: Decimal,
}

impl FlexDao {
    pub fn instantiate_global(initial_members: Vec<(String, ComponentAddress)>)
                              -> (ComponentAddress, DaoSystemAddresses, ComponentAddress) {
        assert!(!initial_members.is_empty(), "At least one initial member must be specified");

        let initial_members: Vec<(DaoMember, ComponentAddress)> = initial_members
            .into_iter()
            .map(|(member_name, member_account_address)| {
                let member = DaoMember {
                    name: member_name,
                    data: None,
                };
                (member, member_account_address)
            })
            .collect();

        let component_address_repo = ComponentAddressRepoComponent::instantiate_global();
        let (dao_system_addresses, admin_badge) =
            SimpleDaoSystemComponent::instantiate_global(initial_members, component_address_repo);

        let access_rules = AccessRules::new()
            .method("get_usage_fee_pct", rule!(allow_all))
            .method("set_usage_fee_pct", rule!(require(admin_badge.resource_address())))
            .method("create_proposal", rule!(require(dao_system_addresses.membership_resource)));

        let mut component = Self {
            dao_system_addresses: dao_system_addresses.clone(),
            admin_badge: Vault::with_bucket(admin_badge),
            usage_fee_pct: dec!("2.5"),
        }.instantiate();

        component.add_access_check(access_rules);
        (component.globalize(), dao_system_addresses, component_address_repo)
    }

    pub fn get_usage_fee_pct(&self) -> Decimal {
        self.usage_fee_pct
    }

    pub fn set_usage_fee_pct(&mut self, new_usage_fee_pct: Decimal) {
        self.usage_fee_pct = new_usage_fee_pct
    }

    /// This is a very flexible way of letting members create proposals.
    /// Members could for example create a proposal to call method "set_usage_fee_pct" with argument
    /// dec!("5") to double the current fee.
    ///
    /// In essence, members can propose any code execution they like but every proposal is subject
    /// to the same strict rules:
    /// - An absolut 2/3 majority is always required
    /// - There is always a hard voting deadline of 100 epochs
    ///
    /// Letting members define proposal entirely by themselves without enforcing some basic rules
    /// should generally be regarded as unsafe and a high security risk. If members were allowed to
    /// configure the entire proposal, they could set the voting deadline to 1 epoch and only require
    /// a single approve vote. They could then run any code they like.
    ///
    /// Please also note a cool property of this method:
    /// If users wanted to do so, they could create a proposal to create another proposal. The second
    /// proposal (or more generally, vote) could then be configured completely freely. This vote could
    /// contain more than two options or it could be a vote that is not associated with any
    /// code execution but is simply there to collect the opinion of members on some topic.
    /// Although that might sound scary at first, it does not introduce a security risk, because members
    /// still have to approve creating that second proposal. (Approving the first proposal triggers a
    /// code execution that creates the second proposal.) While the second proposal could configured
    /// completely freely, the first proposal would still be subject to the strict restrictions
    /// defined in this method.
    pub fn create_proposal(&self, name: String, code_executions: Vec<CodeExecution>) -> Vote {
        let vs: VotingSystemComponent = self.dao_system_addresses.voting_system_component.into_component();

        self.admin_badge.authorize(||
            vs.create_proposal(
                name,
                None, // description
                VotingDeadline::HardEpochDeadline(Runtime::current_epoch() + 100),
                WinRequirement::AbsolutRatio(dec!("0.6666667")),
                code_executions,
                self.dao_system_addresses.membership_resource, // voting_power_resource
        ))
    }
}
}