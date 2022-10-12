/*!
The blueprint implement Community Component for representatives to manage and contribute to the DAO through [Liquid Democracy](crate::align_dao#liquid-democracy).

## Communities
A community represent group of people governed by the [Community Component](crate::community::CommunityComponent). 

The Community component is created to store all the community followers data and help representative
managing the community.

The Community component will use Permissioned User pattern to keep track of community followers.
The pattern is permissioned to prevent Sybil attack for the [Rage Quit](crate::align_dao#rage-quiting--credibility-score-system) DeGov technique.

## Functions, Methods Overview & Transaction manifests
### Function
- Function [new()](Community_impl::Community::new): Instantiate new Community Component (before globalized).

*No transaction manifest since the function will be called through the [become_representative()](crate::align_dao::DAO_impl::DAO::become_representative) method.*
### User pattern methods
Transaction manifests are on sub-group `rtm/community/user_pattern/`.

- Method [request_join()](Community_impl::Community::request_join):
The method is for DAO's members or delegators to request joining the community.
- Method [join()](Community_impl::Community::join):
The method is for DAO's members or delegators to join the community after they have been accepted by the community representative.
- Method [quit()](Community_impl::Community::quit):
The method is for DAO's members or delegators to quit the community.

### Representative methods
Transaction manifests are on sub-group `rtm/community/representative/`.

- Method [review_request()](Community_impl::Community::review_request):
The method is for Representative to give permission for a DAO member or delegator who want to join his/her community.
- Method [amend_tax_policy()](Community_impl::Community::amend_tax_policy):
The method is for Representatives to amend their community tax.

### DAO Component intra-package call only
- Method [abandon()](Community_impl::Community::abandon):
Allow the Representative to abandon his/her community, not user callable.
- Method [rage_quit()](Community_impl::Community::request_join):
Allow the DAO member or Delegator to rage quit the community, not user callable.

### Read only methods
Transaction manifests are on sub-group `rtm/community/read_only/`.

- Method [tax()](Community_impl::Community::tax):
Read only method to read current tax percent of the community.
- Method [vote_state()](Community_impl::Community::vote_state):
Read only method to read current vote state of the community
*/
use crate::delegator::Delegator;
use crate::member::DAOMember;
use scrypto::prelude::*;

blueprint! {

    /// Community Component store all the community followers data and help representative
    /// to manage the community according to [Liquid Democracy](crate::align_dao#liquid-democracy).
    struct Community {
        /// Community controller badge, the badge provide access rule to update community followers' SBT data.
        ///
        /// The badge cannot be withdraw or used for any other purpose execept for supporting the DAO's smartcontract logic.
        controller_badge: Vault,

        /// The community name
        name: String,
        /// The representative id
        representative: NonFungibleAddress,

        /// Store followers' addresses.
        followers: Vec<NonFungibleAddress>,
        /// Delegator SBT resource address.
        delegator_sbt: ResourceAddress,
        /// Member SBT resource addess.
        member_sbt: ResourceAddress,

        /// A book store all the joining request of delegators or DAO members
        request_book: HashMap<NonFungibleAddress, bool>,
        /// Extra data state to help on resim test (since working with NonFungibleAddress
        /// is really hard with current resim model and the new bech32 address)
        address_by_id: HashMap<u64, NonFungibleAddress>,
        /// Request id counter
        request_counter: u64,

        /// The community vote power tax percent.
        power_tax: Decimal,

        /// Store the current credibility score of the community.
        credibility_score: u8,
        /// Store the initial credibility of the community according to the DAO's [CommunityPolicy](crate::policies::CommunityPolicy).
        initial_credibility: u8,

        /// Check if the community is abandoned or not
        abandoned: bool,
    }

    impl Community {

        /// This function instantiate new Community Component.
        ///
        /// # Input
        /// - controller_badge: Bucket contain the controller badge from the DAO,
        /// the badge will provide access rule to update community followers' SBT data.
        /// - name: The Community's name.
        /// - delegator_sbt: Delegator SBT resource address.
        /// - member_sbt: DAO Member SBT resource address.
        /// - initial_credibility: The initial credibility for all Community on creation
        /// according to the DAO's [CommunityPolicy](crate::policies::CommunityPolicy).
        /// - power_tax: Initial power tax that the representative set on community creation.
        /// - representative: The representative's SBT Address.
        ///
        /// # Output
        /// The Community Component, the component will be further used to add access rule and globalize
        /// on the [become_representative()](crate::align_dao::DAO_impl::DAO::become_representative) method.
        /// 
        /// # Smartcontract logic
        /// The function should only be called through the 
        /// [become_representative()](crate::align_dao::DAO_impl::DAO::become_representative) method.
        pub fn new(
            controller_badge: Bucket,
            name: String,
            delegator_sbt: ResourceAddress,
            member_sbt: ResourceAddress,
            initial_credibility: u8,
            power_tax: Decimal,
            representative: NonFungibleAddress,
        ) -> CommunityComponent {
            crate::utils::assert_rate(power_tax);

            Self {
                controller_badge: Vault::with_bucket(controller_badge),
                name,
                representative,
                followers: Vec::new(),
                delegator_sbt,
                member_sbt,
                request_book: HashMap::new(),
                address_by_id: HashMap::new(),
                request_counter: 0,
                power_tax: power_tax / dec!(100),
                credibility_score: initial_credibility,
                initial_credibility,
                abandoned: false,
            }
            .instantiate()
        }

        /// This method is for DAO's members or delegators to request joining the community.
        /// # Input
        /// - identity: the member SBT proof or delegator SBT proof.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong proof provided: Not a single proof from DAO member or Delegator.
        /// - The DAO member or delegator has already followed a community.
        /// - The DAO member currently representing a community or on retirement process.
        /// 
        /// ## Intra-package access
        /// - Access many helpful read only method from [Delegator](crate::delegator::Delegator)
        /// or [DAOMember](crate::member::DAOMember) data struct.
        ///
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/community/user_pattern/request_join_community.rtm`
        /// ```text
        #[doc = include_str!("../rtm/community/user_pattern/request_join_community.rtm")]
        /// ```
        pub fn request_join(&mut self, identity: Proof) {
            assert!(
                !self.abandoned,
                "[Community]: This community is already abandoned"
            );

            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[Community]: Can only using 1 SBT at a time!"
            );

            let address = if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let member_data = member_sbt.data();

                assert!(
                    !member_data.is_following(),
                    "[Community]: You are already joining a community!"
                );
                assert!(
                    !member_data.is_representing(),
                    "[Community]: You are currently representing a community!"
                );
                assert!(
                    !member_data.is_retiring(),
                    "[Community]: You are currently on retirement process!"
                );
                info!("[Community]: Requested joining {} community", self.name);
                member_sbt.address()
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();

                let delegator_data = delegator_sbt.data();

                assert!(
                    !delegator_data.is_following(),
                    "[Community]: You are already joining a community!"
                );
                info!("[Community]: Requested joining {} community", self.name);
                delegator_sbt.address()
            } else {
                panic!("[Community]: Wrong proof provided")
            };

            self.request_counter += 1;
            self.request_book.insert(address.clone(), false);
            self.address_by_id.insert(self.request_counter, address);
        }

        /// This method is for DAO's members or delegators to join the community after they have been accepted by the community representative.
        ///
        /// # Input
        /// - identity: the member SBT proof or delegator SBT proof.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong proof provided: Not a single proof from DAO member or Delegator.
        /// - The component state doesn't have the member/delegator request (haven't requested or already rejected and removed).
        /// - The representative haven't review the request.
        /// 
        /// ## Intra-package access
        /// - Access many helpful read only method from [Delegator](crate::delegator::Delegator)
        ///  or [DAOMember](crate::member::DAOMember) data struct.
        /// - Write the following_community data field on [Delegator](crate::delegator::Delegator)
        ///  or [DAOMember](crate::member::DAOMember) data struct
        /// if the delegator/member successfully joined the community.
        ///
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/community/user_pattern/join_community.rtm`
        /// ```text
        #[doc = include_str!("../rtm/community/user_pattern/join_community.rtm")]
        /// ```
        pub fn join(&mut self, identity: Proof) {
            assert!(
                !self.abandoned,
                "[Community]: This community is already abandoned"
            );

            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[Community]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let member_address = member_sbt.address();
                let result = self.request_book.remove(&member_address);

                match result {
                    None => panic!("[Community]: You haven't requested to join this community yet or have been rejected."),

                    Some(result) => {

                        if result {

                            let mut member_data = member_sbt.data();
                            if member_data.is_following() {
                                error!("[Community]: You are already joining a community!");
                            } else if member_data.is_representing() {
                                error!("[Community]: You are currently representing a community!");
                            } else if member_data.is_retiring() {
                                error!("[Community]: You are currently on retirement process!");
                            } else {

                                member_data.following_community = Some(self.name.clone());

                                self.controller_badge.authorize(||{member_sbt.update_data(member_data)});

                                info!("[Community]: Joined {} community", &self.name);

                                self.followers.push(member_address)

                            }

                        } else {
                            panic!("[Community]: The representative haven't review your joining request yet.")
                        }

                    }
                }
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let delegator_address = delegator_sbt.address();
                let result = self.request_book.remove(&delegator_address);
                match result {
                    None => panic!("[Community]: You haven't requested to join this community yet or have been rejected."),
                    Some(result) => {
                        if result {

                            let mut delegator_data = delegator_sbt.data();
                            if delegator_data.is_following() {
                                error!("[Community]: You are already joining a community!");
                            } else {
                                delegator_data.following_community = Some(self.name.clone());
                                self.controller_badge.authorize(||{delegator_sbt.update_data(delegator_data)});
                                info!("[Community]: Joined {} community", self.name);
                                self.followers.push(delegator_address)
                            }

                        } else {
                            panic!("[Community]: The representative haven't review your joining request yet.")
                        }
                    }
                }
            } else {
                panic!("[Community]: Wrong proof provided")
            };
        }

        /// This method is for DAO's members or delegators to quit the community.
        ///
        /// # Input
        /// - identity: the member SBT proof or delegator SBT proof.
        /// # Access Rule
        /// Anyone can call this method
        /// # Smartcontract logic
        /// ## Panics
        /// - Wrong proof provided: Not a single proof from DAO member or Delegator.
        /// - The member/delegator currently not following the community.
        /// 
        /// ## Intra-package access
        /// - Access many helpful read only method from [Delegator](crate::delegator::Delegator)
        ///  or [DAOMember](crate::member::DAOMember) data struct.
        /// - Access the following_community data field on [Delegator](crate::delegator::Delegator)
        ///  or [DAOMember](crate::member::DAOMember) data struct to check if the delegator/member has joined the community.
        /// - Write the following_community data field on [Delegator](crate::delegator::Delegator)
        ///  or [DAOMember](crate::member::DAOMember) data struct if the delegator/member successfully quitted the community.
        ///
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/community/user_pattern/quit_community.rtm`
        /// ```text
        #[doc = include_str!("../rtm/community/user_pattern/quit_community.rtm")]
        /// ```
        pub fn quit(&mut self, identity: Proof) {
            let validated_proof = identity.unsafe_skip_proof_validation();

            assert!(
                validated_proof.amount() == dec!("1"),
                "[Community]: Can only using 1 SBT at a time!"
            );

            if validated_proof.resource_address() == self.member_sbt {
                let member_sbt = validated_proof.non_fungible::<DAOMember>();
                let address = member_sbt.address();
                let mut member_data = member_sbt.data();

                assert!(
                    member_data.is_following(),
                    "[Community]: You're currently not joining any community"
                );

                let community_name = member_data.following_community.unwrap();
                assert!(
                    community_name == self.name,
                    "[Community]: This isn't the community you're following."
                );

                member_data.following_community = None;

                info!("[Community]: Quitted {} community", community_name);
                self.controller_badge
                    .authorize(|| member_sbt.update_data(member_data));

                self.followers.retain(|x| x != &address);
            } else if validated_proof.resource_address() == self.delegator_sbt {
                let delegator_sbt = validated_proof.non_fungible::<Delegator>();
                let address = delegator_sbt.address();
                let mut delegator_data = delegator_sbt.data();

                assert!(
                    delegator_data.is_following(),
                    "[Community]: You're currently not joining any community"
                );

                let community_name = delegator_data.following_community.as_ref().unwrap().clone();
                assert!(
                    community_name == self.name,
                    "[Community]: This isn't the community you're following."
                );

                delegator_data.following_community = None;

                info!("[Community]: Quitted {} community", community_name);
                self.controller_badge
                    .authorize(|| delegator_sbt.update_data(delegator_data));

                self.followers.retain(|x| x != &address);
            } else {
                panic!("[Community]: Wrong proof provided")
            }
        }

        /// This method is for Representative to give permission for a DAO member or delegator who want to join his/her community.
        ///
        /// # Input
        /// - id: the request id.
        /// - result: review result > accept this member/delegator or not.
        /// # Access Rule
        /// Only Representative with the right Proof on transaction manifest Auth Zone can call this method.
        /// # Smartcontract logic
        /// ## Panics
        /// - The community don't have the provided request id.
        ///
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/community/representative/review_request.rtm`
        /// ```text
        #[doc = include_str!("../rtm/community/representative/review_request.rtm")]
        /// ```
        pub fn review_request(&mut self, id: u64, result: bool) {
            let address = self.address_by_id.remove(&id).expect("[Community]: The community doesn't contain this request id or has already reviewed");

            if result {
                if let Some(x) = self.request_book.get_mut(&address) {
                    *x = true;
                    info!("[Community]: Accepted proposal id {}", id);
                } else {
                    panic!("[Community]: Some how the community didn't have this address")
                };
            } else {
                info!("[Community]: Rejected proposal id {}", id);
                self.request_book.remove(&address);
            }
        }

        /// This method is for Representatives to amend their community tax.
        ///
        /// # Input
        /// - new_tax: new tax percent that the representative want to change on his/her community
        /// # Access Rule
        /// Only Representative with the right Proof on transaction manifest Auth Zone can call this method.
        /// # Smartcontract logic
        /// Panic if the new tax isn't on the accepted range (from 0 to 100)
        ///
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/community/representative/amend_tax_policy.rtm`
        /// ```text
        #[doc = include_str!("../rtm/community/representative/amend_tax_policy.rtm")]
        /// ```
        pub fn amend_tax_policy(&mut self, new_tax: Decimal) {
            crate::utils::assert_rate(new_tax);
            info!("[Community]: amended the community tax to {}%", new_tax);
            self.power_tax = new_tax / dec!(100)
        }

        /// This method allow the Representative to abandon his/her community.
        ///
        /// According to Align DAO smartcontract logic, the function should only be called
        /// through the [retire()](crate::align_dao::DAO_impl::DAO::retire) method.
        ///
        /// # Access Rule
        /// Not user callable, can only be called by the DAO controller badge.
        /// # Smartcontract logic
        /// ## Panics
        /// - On component state there are still members/delegators following the community.
        pub fn abandon(&mut self) {
            assert!(
                self.followers.is_empty(),
                "[Community]: You cannot abandon this community since there're still followers"
            );

            self.abandoned = true
        }

        /// This method allow the DAO member or Delegator to rage quit the community
        /// and confiscate the representative committed resource whenever his/her credibility degrade to 0.
        /// # Input
        /// The rage quitted DAO member or Delegator SBT address
        /// # Output
        /// Return None if the representative credibility haven't degrade to 0 and
        /// the wrapped amount of his/her confiscated resource otherwise.
        /// # Access Rule
        /// Not user callable, can only be called by the DAO controller badge.
        /// # Smartcontract logic
        /// The method can only be called
        /// through the [rage_quit()](crate::align_dao::DAO_impl::DAO::rage_quit) method.
        /// 
        /// ## Intra-package access
        /// - Access the write method [confiscate()](crate::member::DAOMember::confiscate)
        /// from [DAOMember](crate::member::DAOMember) data struct to remove and get
        /// the representative's committed resource amount from the SBT data.
        pub fn rage_quit(&mut self, address: NonFungibleAddress) -> Option<Decimal> {
            self.followers.retain(|x| x != &address);

            self.credibility_score -= 1;

            if self.credibility_score == 0 {
                self.controller_badge.authorize(|| {
                    let mgr = borrow_resource_manager!(self.member_sbt);
                    let mut member_data = mgr
                        .get_non_fungible_data::<DAOMember>(&self.representative.non_fungible_id());
                    let confiscate_amount = member_data.confiscate();
                    self.controller_badge.authorize(|| {
                        mgr.update_non_fungible_data(
                            &self.representative.non_fungible_id(),
                            member_data,
                        );
                    });

                    Some(confiscate_amount)
                })
            } else {
                None
            }
        }

        /// Return current tax percent of the community.
        /// 
        /// The method is for test purpose only and didn't contribute for the DAO's smartcontract logic.
        /// # Access Rule
        /// Read only, anyone can call this method.
        /// 
        /// ---
        ///
        /// # Transaction Manifest
        /// `rtm/community/read_only/tax.rtm`
        /// ```text
        #[doc = include_str!("../rtm/community/read_only/tax.rtm")]
        /// ```
        pub fn tax(&self) -> Decimal {
            self.power_tax
        }

        /// Return current vote state of the community (name, followers, tax percent, remain vote power percent).
        ///
        /// - Remain vote power percent: The remain percent of representative's vote power after got his/her credibility degraded.
        /// # Access Rule
        /// Read only, anyone can call this method.
        pub fn vote_state(&self) -> (String, Vec<NonFungibleAddress>, Decimal, Decimal) {
            let remain_vote_power =
                Decimal::from(self.credibility_score) / Decimal::from(self.initial_credibility);
            (
                self.name.clone(),
                self.followers.clone(),
                self.power_tax,
                remain_vote_power,
            )
        }
    }
}
