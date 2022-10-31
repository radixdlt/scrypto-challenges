use core::fmt;
use std::fmt::Formatter;

use scrypto::prelude::*;
use CharityChange::*;

use crate::code_execution_system::*;
use crate::component_address_repo::*;
use crate::simple_dao_system::*;
use crate::membership_system::*;
use crate::utils::IntoComponent;
use crate::voting_system::*;

blueprint! {

    /// A charitable DAO blueprint showing of the possibilities of building a DAO with the dao-kit
    /// toolkit. If this blueprint is a bit overwhelming, start by looking at the demo_flex_dao.rs
    /// file. The blueprint there is much shorter but uses the dao-kit even more effectively.
    ///
    /// Also see the accompanying test in ../tests/demo_do_good_dao_test.rs
    struct DoGoodDao {
        dao_system_addresses: DaoSystemAddresses,
        admin_badge: Vault,
        charities: HashMap<NonFungibleId, Charity>,
        donor_resource: ResourceAddress,
        component_address_repo: ComponentAddress,
    }

    impl DoGoodDao {
        pub fn instantiate_global(
            initial_members: Vec<(BoardMember, ComponentAddress)>,
        ) -> (ComponentAddress, DaoSystemAddresses, ResourceAddress) {
            assert!(!initial_members.is_empty(), "At least one initial member must be specified");

            let initial_members: Vec<(DaoMember, ComponentAddress)> = initial_members
                .into_iter()
                .map(|(member, member_account_address)| {
                    let member = DaoMember {
                        name: format!("{} {}", member.first_name, member.last_name),
                        data: Some(args! {member}),
                    };
                    (member, member_account_address)
                })
                .collect();

            let component_address_repo = ComponentAddressRepoComponent::instantiate_global();
            let (dao_system_addresses, admin_badge) =
                SimpleDaoSystemComponent::instantiate_global(initial_members, component_address_repo);

            let donor_resource = ResourceBuilder::new_non_fungible()
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
                .metadata("name", "Donor Badge")
                .no_initial_supply();

            let board_member_badge = dao_system_addresses.membership_resource;
            let access_rules = AccessRules::new()
                .method("register_as_donor", rule!(allow_all))
                .method("make_donation", rule!(allow_all))
                .method(
                    "implement_charity_change",
                    rule!(require(admin_badge.resource_address())),
                )
                .method(
                    "propose_implement_charity_change",
                    rule!(require_any_of(vec![board_member_badge, donor_resource])),
                )
                .method("propose_add_board_member", rule!(require(board_member_badge)))
                .method("get_charities", rule!(allow_all))
                .method("dao_system_addresses", rule!(allow_all));

            let mut component = Self {
                dao_system_addresses: dao_system_addresses.clone(),
                admin_badge: Vault::with_bucket(admin_badge),
                charities: HashMap::new(),
                donor_resource,
                component_address_repo,
            }
            .instantiate();
            component.add_access_check(access_rules);

            (component.globalize(), dao_system_addresses, donor_resource)
        }

        pub fn register_as_donor(&mut self) -> Bucket {
            self.admin_badge.authorize(|| {
                let rm = borrow_resource_manager!(self.donor_resource);
                rm.mint_non_fungible(&NonFungibleId::random(), Donor::new())
            })
        }

        pub fn make_donation(&mut self, charity_id: NonFungibleId, donation: Bucket, donor_badge: Proof) {
            assert_eq!(donation.resource_address(), RADIX_TOKEN, "You can only donate XRD");
            let charity = self
                .charities
                .get_mut(&charity_id)
                .unwrap_or_else(|| panic!("Charity with ID {} not found", charity_id));
            charity.record_donation(donation.amount());

            let donor_badge = donor_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.donor_resource,
                    dec!(1),
                ))
                .expect("Invalid DonorBadge");

            let mut donor: Donor = donor_badge.non_fungible().data();
            donor.record_donation(charity_id, donation.amount());
            self.admin_badge.authorize(|| {
                let rm = borrow_resource_manager!(self.donor_resource);
                rm.update_non_fungible_data(&donor_badge.non_fungible_id(), donor);
            });

            let charity_account = borrow_component!(charity.account_address);
            charity_account.call::<()>("deposit", args!(donation));
        }

        pub fn implement_charity_change(&mut self, change: CharityChangeDto) {
            let change = CharityChange::from_dto(change, self.component_address_repo.into_component());
            change.implement(&mut self.charities);
        }

        pub fn propose_implement_charity_change(&self, change: CharityChange) -> Vote {
            change.assert_valid(&self.charities);

            self.admin_badge.authorize(|| {
                let vs : VotingSystemComponent = self.dao_system_addresses.voting_system_component.into();
                vs.create_proposal(
                    "implement charity change".to_owned(),
                    None, // description
                    VotingDeadline::SoftEpochDeadline(Runtime::current_epoch() + 350),
                    WinRequirement::majority(true),
                    vec![CodeExecution::MethodCall {
                        component: self
                            .component_address_repo()
                            .create_lookup(Runtime::actor().as_component().0),
                        method: "implement_charity_change".to_owned(),
                        args: args!(change.to_dto(self.component_address_repo.into_component())),
                        required_badges: vec![self.admin_badge.resource_address()],
                    }],
                    self.dao_system_addresses.membership_resource,
                )
            })
        }

        pub fn propose_add_board_member(&self, new_member: BoardMember, new_member_account: ComponentAddress) -> Vote {
            let new_member_bytes = args!(new_member);
            let new_member_account_lookup = self.component_address_repo().create_lookup(new_member_account);
            self.admin_badge.authorize(|| {
                let vs : VotingSystemComponent = self.dao_system_addresses.voting_system_component.into();
                vs.create_proposal(
                    "add board member".to_owned(),
                    Some(format!(
                        "Add new board member {} and send badge to {:?}",
                        new_member, new_member_account
                    )),
                    // Set the deadline to about a weak (given an epoch length of ca. 30m)
                    VotingDeadline::SoftEpochDeadline(Runtime::current_epoch() + 350),
                    WinRequirement::majority(true),
                    vec![CodeExecution::MethodCall {
                        component: self
                            .component_address_repo()
                            .create_lookup(self.dao_system_addresses.membership_system_component),
                        method: "add_member_and_distribute_badge".to_owned(),
                        args: args!(
                            new_member.full_name(),
                            Some(new_member_bytes),
                            new_member_account_lookup
                        ),
                        required_badges: vec![self.admin_badge.resource_address()],
                    }],
                    self.dao_system_addresses.membership_resource,
                )
            })
        }

        pub fn get_charities(&self) -> Vec<Charity> {
            self.charities.values().map(|c| c.to_owned()).collect()
        }

        pub fn dao_system_addresses(&self) -> DaoSystemAddresses {
            self.dao_system_addresses.clone()
        }

        fn component_address_repo(&self) -> ComponentAddressRepoComponent {
            self.component_address_repo.into()
        }
    }
}

#[derive(Encode, Decode, TypeId, Describe, Clone)]
pub struct Charity {
    pub id: NonFungibleId,
    pub name: String,
    pub account_address: ComponentAddress,
    pub donations_received: Decimal,
}

impl Charity {
    fn new(id: NonFungibleId, name: String, account_address: ComponentAddress) -> Self {
        Charity {
            id,
            name,
            account_address,
            donations_received: dec!(0),
        }
    }

    fn record_donation(&mut self, amount: Decimal) {
        self.donations_received += amount
    }
}

#[derive(Encode, Decode, TypeId, Describe)]
pub enum CharityChange {
    CreateNewCharity {
        name: String,
        account_address: ComponentAddress,
    },
    UpdateCharity {
        id: NonFungibleId,
        name: Option<String>,
        account_address: Option<ComponentAddress>,
    },
    RemoveCharity {
        id: NonFungibleId,
    },
}

impl CharityChange {
    fn assert_valid(&self, all_charities: &HashMap<NonFungibleId, Charity>) {
        match self {
            CreateNewCharity { account_address, .. } => {
                assert!(
                    matches!(account_address, ComponentAddress::Account(..)),
                    "Not a valid account address"
                );
            }
            UpdateCharity {
                id, account_address, ..
            } => {
                assert!(all_charities.get(id).is_some(), "Charity with ID {} does not exist", id);
                assert!(
                    matches!(account_address, Some(ComponentAddress::Account(..))),
                    "Not a valid account address"
                );
            }
            RemoveCharity { id } => {
                assert!(all_charities.get(id).is_some(), "Charity with ID {} does not exist", id);
            }
        }
    }

    fn implement(self, all_charities: &mut HashMap<NonFungibleId, Charity>) {
        match self {
            CreateNewCharity { name, account_address } => {
                let charity_id = NonFungibleId::random();
                let charity = Charity::new(charity_id.to_owned(), name.to_owned(), account_address);
                all_charities.insert(charity_id, charity);
            }
            UpdateCharity {
                id,
                name,
                account_address,
            } => {
                let mut charity = all_charities
                    .get_mut(&id)
                    .expect("Charity with ID {} does not exist anymore");
                if let Some(name) = name {
                    charity.name = name;
                }
                if let Some(account_address) = account_address {
                    charity.account_address = account_address;
                }
            }
            RemoveCharity { id } => {
                all_charities.remove(&id);
            }
        }
    }

    fn to_dto(self, component_address_repo: ComponentAddressRepoComponent) -> CharityChangeDto {
        match self {
            CreateNewCharity { name, account_address } => CharityChangeDto::CreateNewCharity {
                name,
                account_address: component_address_repo.create_lookup(account_address),
            },
            UpdateCharity {
                id,
                name,
                account_address,
            } => CharityChangeDto::UpdateCharity {
                id,
                name,
                account_address: account_address.map(|ac| component_address_repo.create_lookup(ac)),
            },
            RemoveCharity { id } => CharityChangeDto::RemoveCharity { id },
        }
    }

    fn from_dto(dto: CharityChangeDto, component_address_repo: ComponentAddressRepoComponent) -> Self {
        match dto {
            CharityChangeDto::CreateNewCharity { name, account_address } => CreateNewCharity {
                name,
                account_address: component_address_repo.lookup_address(account_address),
            },
            CharityChangeDto::UpdateCharity {
                id,
                name,
                account_address,
            } => UpdateCharity {
                id,
                name,
                account_address: account_address.map(|ac_lookup| component_address_repo.lookup_address(ac_lookup)),
            },
            CharityChangeDto::RemoveCharity { id } => RemoveCharity { id },
        }
    }
}

#[derive(Encode, Decode, TypeId, Describe)]
pub enum CharityChangeDto {
    CreateNewCharity {
        name: String,
        account_address: ComponentAddressLookup,
    },
    UpdateCharity {
        id: NonFungibleId,
        name: Option<String>,
        account_address: Option<ComponentAddressLookup>,
    },
    RemoveCharity {
        id: NonFungibleId,
    },
}

#[derive(NonFungibleData)]
struct Donor {
    #[scrypto(mutable)]
    donations_made: HashMap<NonFungibleId, Decimal>,
}

impl Donor {
    pub fn new() -> Self {
        Self {
            donations_made: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn donations_made(&self) -> &HashMap<NonFungibleId, Decimal> {
        &self.donations_made
    }

    fn record_donation(&mut self, charity_id: NonFungibleId, amount: Decimal) {
        self.donations_made
            .entry(charity_id)
            .and_modify(|current_donation_amount| {
                *current_donation_amount += amount;
            })
            .or_insert(amount);
    }
}

#[derive(Encode, Decode, TypeId, Describe)]
pub struct BoardMember {
    pub first_name: String,
    pub last_name: String,
}

impl BoardMember {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl fmt::Display for BoardMember {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}
