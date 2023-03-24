use super::component_address_repo::*;
use scrypto::prelude::*;

external_component! {
    AccountInterface {
        fn deposit(&mut self, bucket: Bucket);
    }
}

blueprint! {

    /// A simple MembershipSystem that can be used to create, update and delete members.
    ///
    /// Members are represented by non fungible resources. When creating a new member a membership
    /// token is given to the member. If a VotingSystem is used, the membership token can be used as
    /// the voting power token. This way it can be enforced that each member can only cast one vote.
    struct MembershipSystem {

        /// The Address of the membership token
        membership_resource: ResourceAddress,

        /// An internal authority to mint and burn member NFRs
        minter: Vault,

        /// The address of a ComponentAddressRepo for looking up component addresses
        /// This is a workaround for issue https://github.com/radixdlt/radixdlt-scrypto/issues/483
        component_address_repo: ComponentAddress,
    }

    impl MembershipSystem {

        /// Instantiates a new MembershipSystem component
        ///
        /// Arguments:
        /// - `component_address_repo`: The address of a ComponentAddressRepo
        ///
        /// Returns:
        /// - The newly created MembershipSystemComponent instance
        /// - The resource address of the membership token
        pub fn instantiate(component_address_repo: ComponentAddress) -> (MembershipSystemComponent, ResourceAddress) {
            let minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);
            let membership_resource = ResourceBuilder::new_non_fungible()
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .metadata("name", "Membership Badge")
                .no_initial_supply();

            let component = Self {
                membership_resource,
                minter: Vault::with_bucket(minter),
                component_address_repo,
            }
            .instantiate();

            (component, membership_resource)
        }

        /// Instantiates a new MembershipSystem component and globalizes it
        ///
        /// Arguments:
        /// - `admin_badge`: An admin badge that is allowed to call the MembershipSystem's
        /// privileged methods
        /// - `component_address_repo`: The address of a ComponentAddressRepo
        ///
        /// Returns:
        /// - The global component address of the newly created MembershipSystem component
        /// - The resource address of the membership token
        pub fn instantiate_global(
            admin_badge: ResourceAddress,
            component_address_repo: ComponentAddress,
        ) -> (ComponentAddress, ResourceAddress) {
            let rules = AccessRules::new()
                .method("add_member", rule!(require(admin_badge)))
                .method("add_member_and_distribute_badge", rule!(require(admin_badge)))
                .method("update_member", rule!(require(admin_badge)))
                .method("remove_member", rule!(allow_all))
                .method("remove_member_by_id", rule!(require(admin_badge)))
                .method("membership_resource", rule!(allow_all));

            let (mut component, membership_resource) = Self::instantiate(component_address_repo);
            component.add_access_check(rules);
            (component.globalize(), membership_resource)
        }

        /// Adds a new member
        ///
        /// Arguments:
        /// - `name`: The new member's name
        /// - `data`: The new member's data as bytes. Optional. This may be used to take an
        /// arbitrary scrypto-encodable and -decodable Struct, encode it to a byte array and store
        /// it inside the new member NFR
        ///
        /// Returns: a bucket with a newly minted membership token
        pub fn add_member(&self, name: String, data: Option<Vec<u8>>) -> Bucket {
            let dao_member: DaoMember = DaoMember { name, data, };
            let member_id = NonFungibleId::random();
            self.minter.authorize(|| {
                let rm = borrow_resource_manager!(self.membership_resource);
                rm.mint_non_fungible(&member_id, dao_member)
            })
        }

        /// Adds a new member and deposits the membership badge directly into the new members
        /// account. This method is provided so that no other entity may take illegal possession of
        /// the membership badge.
        ///
        /// Arguments:
        /// - `name`: The new member's name
        /// - `data`: The new member's data as bytes. Optional. This may be used to take an
        /// arbitrary scrypto-encodable and -decodable Struct, encode it to a byte array and store
        /// it inside the new member NFR
        /// - `new_member_account_address`: The account address of the new member
        pub fn add_member_and_distribute_badge(
            &self,
            name: String,
            data: Option<Vec<u8>>,
            new_member_account_address: ComponentAddressLookup,
        ) {
            let membership_token = self.add_member(name, data);
            let new_member_account_address = self
                .component_address_repo()
                .lookup_address(new_member_account_address);
            let mut new_member_account = AccountInterface::from(new_member_account_address);
            new_member_account.deposit(membership_token);
        }

        /// Updates the member with the given ID
        ///
        /// Arguments:
        /// - `member_id`: The ID of the member to update
        /// - `new_name`: The new name of the member
        /// - `new_data`: The new data of the member
        pub fn update_member(&self, member_id: NonFungibleId, new_name: String, new_data: Option<Vec<u8>>) {
            let rm = borrow_resource_manager!(self.membership_resource);
            self.minter
                .authorize(|| rm.update_non_fungible_data(&member_id, DaoMember::new(new_name, new_data)));
        }

        /// Removes the member(s) referenced in the given bucket by burning their membership tokens
        pub fn remove_member(&self, membership_token: Bucket) {
            self.minter.authorize(|| {
                membership_token.burn();
            });
        }

        /// Not implemented right now! Requires recallable resources.
        ///
        /// Removes the member by their ID. The membership token is recalled from the members vault
        /// and is subsequently burned.
        ///
        /// Arguments:
        /// `_member_id`: The ID of the member to remove
        pub fn remove_member_by_id(&self, _member_id: NonFungibleId) {
            // let membership_token = borrow_resource_manager!(self.membership_token).recall(member_id);
            // self.remove_member(membership_token);
            todo!("Requires recallable resources to be implemented")
        }

        pub fn membership_resource(&self) -> ResourceAddress {
            self.membership_resource
        }

        fn component_address_repo(&self) -> ComponentAddressRepoComponent {
            self.component_address_repo.into()
        }
    }
}

/// Represents a DAO member
#[derive(NonFungibleData, Encode, Decode, TypeId, Describe)]
pub struct DaoMember {
    /// The name of the member
    pub name: String,

    /// Optional member data encoded as bytes
    pub data: Option<Vec<u8>>,
}

impl DaoMember {
    pub fn new(name: String, data: Option<Vec<u8>>) -> Self {
        Self { name, data }
    }
}
