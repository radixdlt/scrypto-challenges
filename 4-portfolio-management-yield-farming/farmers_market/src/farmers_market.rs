use scrypto::prelude::*;
use crate::farmer_dashboard::*;
use crate::borrower_dashboard::*;
use crate::structs::*;
use crate::price_oracle::*;
use crate::utils::*;
use crate::investor_dashboard::*;

blueprint! {
    /// This struct is used to define the _____. The ______ is used for protocol admins to approve users. It is also where other components
    /// can index and retrieve all the funds that exist within this protocol. 
    struct FarmersMarket {
        /// The ResourceAddress of the protocol admin.
        protocol_admin_address: ResourceAddress,
        admin_vault: Vault,
        /// The Resourceaddress of the Loan Request NFT Admin used to mint/burn/update Loan Request NFTs.
        loan_request_nft_admin_address: ResourceAddress,
        /// The Resource Address of the Loan Request NFT.
        loan_request_nft_address: ResourceAddress,
        /// The sets of all the Farmers in the protocol.
        farmers: HashSet<NonFungibleId>,
        /// The ResourceAddress of the Farmers Badge.
        farmer_address: ResourceAddress,
        /// The ResourceAddress of the FarmerDashboard Admin token. This is held by the component to make authorized
        /// cross blueprint method calls.
        farmer_dashboard_admin_address: ResourceAddress,
        /// Delete?
        farmer_dashboards: HashMap<NonFungibleId, ComponentAddress>,
        /// The vault that contains the Farmers Badge for approved Farmers to claim their Farmers.
        /// Farmers must deposit their temporary badge to claim their Farmers Badge.
        farmer_badge_vault: Vault,
        /// The sets of all the Borrowers in the protocol.
        borrowers: HashSet<NonFungibleId>,
        /// The ResourceAddress of the Borrwer Badge.
        borrower_address: ResourceAddress,
        /// The ResourceAddress of the BorrowerDashboard Admin token. This is held by the component to make authorized
        /// cross blueprint method calls.
        borrower_dashboard_admin_address: ResourceAddress,
        /// Delete?
        borrower_dashboards: HashMap<NonFungibleId, ComponentAddress>,
        /// The vault that contains the Borrower Badge for approved Borrower to claim their Borrower Badge.
        /// Borrowers must deposit their temporary badge to claim their Borrower Badge.
        borrower_badge_vault: Vault,
        /// The ResourceAddress of the Temporary Badge.
        temporary_badge_address: ResourceAddress,
        /// The HashMap of the Temporary Badge that are pending approval.
        pending_approvals: HashMap<String, NonFungibleId>,
        /// HashMap<Temporary Badge, Borrower/Farmers Badge>. This contains the HashMap of the Temporary Badge
        /// and its associated approved Borrower/Farmers Badge.
        approvals: HashMap<NonFungibleId, NonFungibleId>,
        /// HashMap<Borrower Badge, Vault>. This contains the HashMap of the Borrower and their associated Loan Request NFT.
        global_loan_requests_vault: HashMap<NonFungibleId, Vault>,
        /// Record of all the debt funds in this protocol. HashMap<Fund Name, ComponentAddress>.
        global_debt_funds: HashMap<String, ComponentAddress>,
        global_tracking_tokens_address_mapping: HashMap<ResourceAddress, String>,
        global_funding_lockers: HashMap<NonFungibleId, ComponentAddress>,
        /// Record of all the index funds in this protocol. HashMap<(Fund Name, Fund Ticker), ComponentAddress>.
        global_index_funds: HashMap<(String, String), ComponentAddress>,
        global_index_funds_name: HashMap<String, String>,
        price_oracle_address: ComponentAddress,
        investor_dashboard_address: Option<ComponentAddress>,
    }

    impl FarmersMarket {

        pub fn new() -> (ComponentAddress, Bucket)
        {   
            // Badge that will be stored in the component's vault to update loan NFT.
            let admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .metadata("symbol", "AB")
                .metadata("description", "Component Admin authority")
                .initial_supply(1);

            let protocol_admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Farmers Market Admin Badge")
                .metadata("symbol", "FM_AB")
                .metadata("description", "Protocol Admin for Farmers Market")
                .initial_supply(1);
                
            // Badge that will be stored in the component's vault to update loan NFT.
            let loan_request_nft_admin_address: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Loan Request Admin Badge")
                .metadata("symbol", "LR_AB")
                .metadata("description", "Admin authority for minting/burning/updating loan request NFTs.")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();

            let allowed_badge: Vec<ResourceAddress> = vec!(
                admin.resource_address(), 
                loan_request_nft_admin_address,
            );

            // NFT description for Pool Delegates
            let loan_request_nft_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Loan Request NFT")
                .metadata("symbol", "LRNFT")
                .metadata("description", "Loan Request Terms")
                .mintable(rule!(require(loan_request_nft_admin_address)), LOCKED)
                .burnable(rule!(require(loan_request_nft_admin_address)), LOCKED)
                .updateable_non_fungible_data(rule!(require_any_of(allowed_badge)), LOCKED)
                .no_initial_supply();

            let farmer_dashboard_admin_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Farmer Dashboard Admin Badge")
                .metadata("symbol", "F_AB")
                .metadata("description", "Admin Badge to control Farmers Dashboard Component")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();

            let borrower_dashboard_admin_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Borrower Admin Badge")
                .metadata("symbol", "B_AB")
                .metadata("description", "Admin Badge to control Borrower Dashboard Component")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();

            let allowed_badge: Vec<ResourceAddress> = vec!(
                admin.resource_address(), 
                farmer_dashboard_admin_address,
            );

            // NFT description for Farmers
            let farmer_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Farmer NFT")
                .metadata("symbol", "F_NFT")
                .metadata("description", "Farmers Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require_any_of(allowed_badge)), LOCKED)
                .no_initial_supply();
            
            let allowed_badge: Vec<ResourceAddress> = vec!(
                admin.resource_address(), 
                borrower_dashboard_admin_address,
            );

            let borrower_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Borrower NFT")
                .metadata("symbol", "B_NFT")
                .metadata("description", "Borrower Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require_any_of(allowed_badge)), LOCKED)
                .no_initial_supply();
      
            let temporary_badge_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Temporary Badge NFT")
                .metadata("symbol", "TB_NFT")
                .metadata("description", "Temporary Badge NFT for Farmers/Borrowers")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();

            let access_rules: AccessRules = AccessRules::new()
                .method("insert_debt_fund", rule!(require(farmer_dashboard_admin_address)))
                .method("insert_index_fund", rule!(require(farmer_dashboard_admin_address)))
                .method("insert_tracking_tokens", rule!(require(farmer_dashboard_admin_address)))
                .method("insert_index_fund_name", rule!(require(farmer_dashboard_admin_address)))
                .method("insert_funding_locker", rule!(require(farmer_dashboard_admin_address)))
                .method("retrieve_loan_request_nft", rule!(require(borrower_dashboard_admin_address)))
                .method("return_loan_request_nft", rule!(require(borrower_dashboard_admin_address)))
                .default(rule!(allow_all)
            );
            
            let farmers_market = Self {
                protocol_admin_address: protocol_admin.resource_address(),
                admin_vault: Vault::with_bucket(admin),
                loan_request_nft_admin_address: loan_request_nft_admin_address,
                loan_request_nft_address: loan_request_nft_address,
                farmers: HashSet::new(),
                farmer_address: farmer_address,
                farmer_dashboard_admin_address: farmer_dashboard_admin_address,
                farmer_dashboards: HashMap::new(),
                farmer_badge_vault: Vault::new(farmer_address),
                borrowers: HashSet::new(),
                borrower_address: borrower_address,
                borrower_dashboard_admin_address: borrower_dashboard_admin_address,
                borrower_dashboards: HashMap::new(),
                borrower_badge_vault: Vault::new(borrower_address),
                temporary_badge_address: temporary_badge_address,
                pending_approvals: HashMap::new(),
                approvals: HashMap::new(),
                global_loan_requests_vault: HashMap::new(),
                global_debt_funds: HashMap::new(),
                global_tracking_tokens_address_mapping: HashMap::new(),
                global_funding_lockers: HashMap::new(),
                global_index_funds: HashMap::new(),
                global_index_funds_name: HashMap::new(),
                price_oracle_address: PriceOracle::new(),
                investor_dashboard_address: None,
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (farmers_market, protocol_admin)
        }

        /// This method retrieves the ComponentAddress of this component.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `Option<ComponentAddress>` - If the ComponentAddress exist, it will return it.
        pub fn view_component_address(
            &self,
        ) -> Option<ComponentAddress>
        {
            let component_address = Runtime::actor().component_address();
            component_address
        }

        /// This method is used to retrieve the NFT data of a selected NFT. 
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `nft_id` (&NonFungibleId) - The NonFungibleId of the NFT data to retrieve.
        /// * `badge_name` (Badges) - The Enum that matches and retrieves the ResourceAddress of the NFT.
        /// 
        /// # Returns: 
        /// 
        /// * `BadgeContainer` - The NFT data of the chosen NFT.
        fn get_resource_manager(
            &self,
            nft_id: &NonFungibleId,
            badge_name: Badges) -> BadgeContainer
        {
            let badge_address = match badge_name {
                Badges::Farmer => self.farmer_address,
                Badges::Borrower => self.borrower_address,
                Badges::TemporaryBadge => self.temporary_badge_address,
                Badges::LoanRequestNFT => self.loan_request_nft_address,
            };

            let resource_manager = borrow_resource_manager!(badge_address);

            match badge_name {
                Badges::Farmer => {
                    let nft_data: Farmer = resource_manager.get_non_fungible_data(&nft_id);
                    return BadgeContainer::FarmerContainer(nft_data)
                }
                Badges::Borrower => {
                    let nft_data: Borrower = resource_manager.get_non_fungible_data(&nft_id);
                    return BadgeContainer::BorrowerContainer(nft_data)
                }
                Badges::TemporaryBadge => {
                    let nft_data: TemporaryBadge = resource_manager.get_non_fungible_data(&nft_id);
                    return BadgeContainer::TemporaryBadgeContainer(nft_data)
                }
                Badges::LoanRequestNFT => {
                    let nft_data: LoanRequest = resource_manager.get_non_fungible_data(&nft_id);
                    return BadgeContainer::LoanRequestNFTContainer(nft_data)
                }
            }
        }

        /// This method is used to authorize update/mutate of the NFT data.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `nft_id` (&NonFungibleId) - The NFT ID of the NFT data to update.
        /// * `badge_name` (Badges) - The Enum of the badge that matches and retrieves the ResourceAddress of the NFT. 
        /// * `nft_data` (BadgeContainer) - The Enum that matches and retrieves the NFT data of the NFT.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        fn authorize_update(
            &self,
            nft_id: &NonFungibleId,
            badge_name: Badges,
            nft_data: BadgeContainer
        )
        {
            let badge_address = match badge_name {
                Badges::Farmer => self.farmer_address,
                Badges::Borrower => self.borrower_address,
                Badges::TemporaryBadge => self.temporary_badge_address,
                Badges::LoanRequestNFT => self.loan_request_nft_address,
            };

            let resource_manager = borrow_resource_manager!(badge_address);
            
            match nft_data {
                BadgeContainer::FarmerContainer(pool_delegate) => {
                    self.admin_vault.authorize(|| 
                        resource_manager.update_non_fungible_data(nft_id, pool_delegate)
                    );
                }
                BadgeContainer::BorrowerContainer(borrower) => {
                    self.admin_vault.authorize(|| 
                        resource_manager.update_non_fungible_data(nft_id, borrower)
                    );
                }
                BadgeContainer::TemporaryBadgeContainer(temporary_badge) => {
                    self.admin_vault.authorize(|| 
                        resource_manager.update_non_fungible_data(nft_id, temporary_badge)
                    );
                }
                BadgeContainer::LoanRequestNFTContainer(loan_request_nft) => {
                    self.admin_vault.authorize(|| 
                        resource_manager.update_non_fungible_data(nft_id, loan_request_nft)
                    );
                }
            }
        }

        // Implement Access Rule
        // Provide Farmers Proof? No.
        pub fn authorize_loan_request_update(
            &mut self,
            loan_request_nft_id: NonFungibleId,
            loan_request_nft_data: LoanRequest,
        )
        {
            let resource_manager = borrow_resource_manager!(self.loan_request_nft_address);

            self.admin_vault.authorize(|| 
                resource_manager.update_non_fungible_data(&loan_request_nft_id, loan_request_nft_data)
            );
        }

        /// This method is used by the BorrowerDashboard component to deposit Loan Request NFTs.
        /// The Loan Request NFTs are used to propose loans for Farmers to underwrite and provide funding.
        /// The Loan Request NFTs are contained in thhis component as it allows other components to have visibility
        /// of the loan requests produced by all borrowers.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof provided is a borrower.
        /// * **Check 2:** - Checks that the Bucket passed is a Loan Request NFT that belongs to this protocol.
        /// 
        /// # Arguments: 
        /// 
        /// * `borrower_badge` (Proof) - The Proof of the Borrower Badge.
        /// * `loan_request_nft` (Bucket) - The Bucket that contains the Loan Request NFT.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn deposit_loan_requests(
            &mut self,
            borrower_badge: Proof,
            loan_request_nft: Bucket
        )
        {
            assert_eq!(
                borrower_badge.resource_address(), self.borrower_address,
                "[Vault Protocol]: The badge does not belong to this protocol."
            );

            assert_eq!(
                loan_request_nft.resource_address(), self.loan_request_nft_address,
                "The bucket passed must contains a loan request NFT."
            );

            let borrower_id: NonFungibleId = borrower_badge.non_fungible::<Borrower>().id();

            if self.global_loan_requests_vault.contains_key(&borrower_id) {
                self.global_loan_requests_vault.get_mut(&borrower_id).unwrap().put(loan_request_nft);
            } else {
                self.global_loan_requests_vault.insert(borrower_id, Vault::with_bucket(loan_request_nft));
            }
        }

        /// This method is used to allow users to essentially apply to be a Farmers or a Borrower.
        /// A Temporary Badge is created with the name of their entity. The user (either Farmers or Borrower)
        /// turns this Temporary Badge in to claim their actual badges once approved by the protocol owner.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks whether the name provided already exist or not.
        /// 
        /// # Arguments:
        /// 
        /// * `name` (String) - The name of the entity of the user.
        /// * `user_type` (UserType) - The Enum of the type of user (Farmers or Borrower).
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the Temporary Badge.
        pub fn create_temporary_badge(
            &mut self,
            name: String,
            user_type: UserType
        ) -> Bucket
        {
            assert!(self.pending_approvals.contains_key(&name) != true,
                "[Vault Protocol]: The name you provided already exists."
            );

            let temporary_badge = self.admin_vault.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.temporary_badge_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    TemporaryBadge {
                        name: name.clone(),
                        user_type: user_type,
                        status: RequestStatus::Pending,
                    },
                )
            });

            self.pending_approvals.insert( 
                name,
                temporary_badge.non_fungible::<TemporaryBadge>().id()
            );

            info!("[Vault Protocol]: The resource address of your temporary badge is: {:?}", temporary_badge.resource_address());

            temporary_badge
        }

        pub fn new_investor_dashboard(
            &mut self,
            protocol_admin: Proof,
        ) -> ComponentAddress
        {
            assert_eq!(protocol_admin.resource_address(), self.protocol_admin_address,
                "[Vault Protocol]: Unauthorized Access."
            );

            let farmers_market_address: ComponentAddress = self.view_component_address().unwrap().into();

            let investor_dashboard_address: ComponentAddress = InvestorDashboard::new(
                farmers_market_address,
            );

            self.investor_dashboard_address = Some(investor_dashboard_address);

            investor_dashboard_address
        }

        /// Creates an admin badge for each Farmers and instantiates a Farmers Dashboard.
        /// 
        /// This method is used to allow authorized protocol owner(s) to onboard approved Farmers.
        /// Prospective Farmers must first request approval to become a Farmers by filing out the request form
        /// via create_temporary_badge method. The protocol owner(s) will view the approval request via pending_approvals 
        /// data field and approve selected Farmers. A Farmers admin badge will be minted where approved
        /// Farmers can claim via their TemporaryBadge. A Farmers Dashboard will be created where approved
        /// Farmers can access their controls.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof provided is the protocol owner.
        /// 
        /// # Arguments:
        /// 
        /// * `protocol_admin` (Proof) - The Proof of the protocol admin badge.
        /// * `name` (String) - The name of the entity the protocol admin wishes to approve.
        /// 
        /// # Returns:
        /// 
        /// * `ComponentAddress` - Returns the ComponentAddress of the FarmerDashboard
        pub fn create_farmer(
            &mut self,
            protocol_admin: Proof,
            name: String
        ) -> ComponentAddress
        {

            assert_eq!(protocol_admin.resource_address(), self.protocol_admin_address,
                "[Vault Protocol]: Unauthorized Access."
            );

            // Mint Farmers admin badge.
            let farmer_badge = self.admin_vault.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.farmer_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    Farmer {
                        name: name.clone(),
                        managed_index_funds: HashMap::new(),
                        managed_debt_funds: HashMap::new(),
                    },
                )
            });

            // Retrieve the prospective Farmers via pending_approvals data field.
            let pending_user: &NonFungibleId = self.pending_approvals.get(&name).unwrap();

            // Retrieve the NFT data of the Temporary Badge of the prospective Farmers.
            let nft_data = self.get_resource_manager(pending_user, Badges::TemporaryBadge);

            // Change NFT data of the Temporary Badge to indicate the prospective Farmers has been approved.
            match nft_data {
                BadgeContainer::FarmerContainer(_pool_delegate) => {}
                BadgeContainer::BorrowerContainer(_borrower) => {}
                BadgeContainer::TemporaryBadgeContainer(mut temporary_badge) => {

                    assert_eq!(temporary_badge.user_type, UserType::Farmer,
                        "[Vault Protocol - Farmers badge creation]: Incorrect user type."
                    );

                    temporary_badge.status = RequestStatus::Approved;

                    // Authorize update of the NFT data change.
                    self.authorize_update(
                        pending_user, 
                        Badges::TemporaryBadge, 
                        BadgeContainer::TemporaryBadgeContainer(temporary_badge),
                    );
                }
                BadgeContainer::LoanRequestNFTContainer(_loan_request_nft) => {}
            };

            // Retrieve NFT ID of the Farmers admin badge.
            let farmer_id: NonFungibleId = farmer_badge.non_fungible::<Farmer>().id();

            // Insert in the approvals data field. 
            self.approvals.insert(pending_user.clone(), farmer_id.clone());

            // Remove prospective Farmers from the pending_approvals data field.
            self.pending_approvals.remove_entry(&name);

            // Record the NFT ID of the new approved Farmers.
            self.borrowers.insert(farmer_id.clone());

            let loan_request_nft_admin = self.admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_request_nft_admin_address);
                    resource_manager.mint(1)
                }
            );

            let farmer_admin: Bucket = self.admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.farmer_dashboard_admin_address);
                    resource_manager.mint(1)
                }
            );

            let price_oracle_address: ComponentAddress = self.price_oracle_address.into();
            let farmers_market_address: ComponentAddress = self.view_component_address().unwrap().into();

            // Instantiates the Farmers Dashboard for the recently approved Farmers.
            let pool_delegate_dashboard: ComponentAddress = FarmerDashboard::new(
                farmer_admin,
                farmers_market_address,
                self.farmer_address, 
                loan_request_nft_admin,
                self.loan_request_nft_address,
                price_oracle_address,
            );

            // Insert the ComponentAddress of the Farmers Dashboard for this particular Farmers.
            self.farmer_dashboards.insert(
                farmer_id, 
                pool_delegate_dashboard
            );

            // Put the Farmers admin badge for the recently approved Farmers to claim.
            self.farmer_badge_vault.put(farmer_badge);

           pool_delegate_dashboard
        }

        /// Creates an admin badge for each Borrower and instantiates a Borrower Dashboard.
        /// 
        /// This method is used to allow authorized protocol owner(s) to onboard approved Borrowers.
        /// Prospective Borrowers must first request approval to become a Borrower by filing out the request form
        /// via create_temporary_badge method. The protocol owner(s) will view the approval request via pending_approvals 
        /// data field and approve selected Borrowers. A Borrower admin badge will be minted where approved
        /// Borrowers can claim via their TemporaryBadge. A Borrower Dashboard will be created where approved
        /// Borrowers can access their controls.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof provided is the protocol owner.
        /// 
        /// # Arguments:
        /// 
        /// * `protocol_admin` (Proof) - The Proof of the protocol admin badge.
        /// * `name` (String) - The name of the entity the protocol admin wishes to approve.
        /// 
        /// # Returns:
        /// 
        /// * `ComponentAddress` - Returns the ComponentAddress of the BorrowerDashboard
        pub fn create_borrower(
            &mut self,
            protocol_admin: Proof,
            name: String
        ) -> ComponentAddress
        {
            assert_eq!(protocol_admin.resource_address(), self.protocol_admin_address,
                "[Vault Protocol]: Unauthorized Access."
            );

            let borrower_admin_badge = self.admin_vault.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.borrower_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    Borrower {
                        name: name.clone(),
                        loan_requests: BTreeSet::new(),
                        loans: BTreeSet::new(),
                    },
                )
            });

            // Retrieve the prospective Borrower via pending_approvals data field.
            let pending_user: &NonFungibleId = self.pending_approvals.get(&name).unwrap();

            // Retrieve the NFT data of the Temporary Badge of the prospective Borrower.
            let nft_data = self.get_resource_manager(pending_user, Badges::TemporaryBadge);

            // Change NFT data of the Temporary Badge to indicate the prospective Farmers has been approved.
            match nft_data {
                BadgeContainer::FarmerContainer(_farmer) => {}
                BadgeContainer::BorrowerContainer(_borrower) => {}
                BadgeContainer::TemporaryBadgeContainer(mut temporary_badge) => {

                    assert_eq!(temporary_badge.user_type, UserType::Borrower,
                        "[Vault Protocol - Farmers badge creation]: Incorrect user type."
                    );

                    temporary_badge.status = RequestStatus::Approved;

                    // Authorize update of the NFT data change.
                    self.authorize_update(
                        pending_user, 
                        Badges::TemporaryBadge, 
                        BadgeContainer::TemporaryBadgeContainer(temporary_badge),
                    );
                }
                BadgeContainer::LoanRequestNFTContainer(_loan_request_nft) => {}
            };

            let borrower_id = borrower_admin_badge.non_fungible::<Farmer>().id();

            // Insert in the approvals data field. 
            self.approvals.insert(pending_user.clone(), borrower_id.clone());

            // Remove prospective Borrower from the pending_approvals data field.
            self.pending_approvals.remove_entry(&name);

            self.borrowers.insert(borrower_id.clone());

            let loan_request_nft_admin = self.admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_request_nft_admin_address);
                    resource_manager.mint(1)
                }
            );

            let borrower_admin: Bucket = self.admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.borrower_dashboard_admin_address);
                    resource_manager.mint(1)
                }
            );

            let farmers_market_address: ComponentAddress = self.view_component_address().unwrap().into();

            let borrower_dashboard: ComponentAddress = BorrowerDashboard::new(
                farmers_market_address,
                borrower_admin,
                self.borrower_address, 
                loan_request_nft_admin,
                self.loan_request_nft_address,
            );

            self.borrower_dashboards.insert(
                borrower_id,
                borrower_dashboard
            );

            self.borrower_badge_vault.put(borrower_admin_badge);

            borrower_dashboard
        }

        /// Allows recently approved Farmers and Borrowers to claim their admin badge to access their respective dashboard.
        /// 
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Temporary Badge belongs to this protocol.
        /// 
        /// # Arguments:
        /// 
        /// * `temporary_badge` (Bucket) - The Bucket that contains the Temporary Badge.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the Borrower/Farmers badge.
        pub fn claim_badge(
            &mut self,
            temporary_badge: Bucket
        ) -> Bucket
        {
            assert_eq!(temporary_badge.resource_address(), self.temporary_badge_address,
                "[Vault Protocol]: This badge does not belong to this protocol."
            );
            
            // Retrieves the Temporary Badge NFT ID.
            let temporary_badge_id: NonFungibleId = temporary_badge.non_fungible::<TemporaryBadge>().id();

            //
            let temporary_badge_data: TemporaryBadge = temporary_badge.non_fungible().data();

            let user_type = temporary_badge_data.user_type;

            match user_type {
                UserType::Farmer => {
                    // Matches the Temporary Badge NFT ID with the approved Farmers admin badge.
                    let claim_badge: &NonFungibleId = self.approvals.get(&temporary_badge_id).unwrap();

                    // Returns the Farmers admin badge.
                    let return_farmer_badge: Bucket = self.farmer_badge_vault.take_non_fungible(claim_badge);

                    // Removes the entry from the approved list.
                    self.approvals.remove_entry(&temporary_badge_id);

                    self.admin_vault.authorize(||
                        temporary_badge.burn()
                    );

                    info!("[Vault Protocol]: The resource address of your NFT is: {:?}", return_farmer_badge.resource_address());

                    return_farmer_badge
                }
                UserType::Borrower => {

                    // Matches the Temporary Badge NFT ID with the approved Farmers admin badge.
                    let claim_badge: &NonFungibleId = self.approvals.get(&temporary_badge_id).unwrap();

                    // Returns the Farmers admin badge.
                    let return_borrower_admin_badge: Bucket = self.borrower_badge_vault.take_non_fungible(claim_badge);

                    // Removes the entry from the approved list.
                    self.approvals.remove_entry(&temporary_badge_id);

                    self.admin_vault.authorize(||
                        temporary_badge.burn()
                    );

                    info!("[Vault Protocol]: The resource address of your NFT is: {:?}", return_borrower_admin_badge.resource_address());

                    return_borrower_admin_badge
                }
            }
        }

        /// This method is used by the BorrowerDashboard component to retrieve the Loan Request NFT in order to allow the
        /// BorrowerDashboard to create a Proof of the Loan Request NFT and access the Funding Locker (where loans are funded
        /// and drawn). The Borrower must first deposit collateral before they can receive the Loan NFT (which will be used to access 
        /// the Funding Locker).
        /// 
        /// This method does not perform any checks but is imposed by an Access Rule that requires the proof of Borrower Badge
        /// to be present.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_request_nft_id` (NonFungibleId) - The NFT ID of the Loan Request NFT.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the Loan Request NFT.
        pub fn retrieve_loan_request_nft(
            &mut self,
            loan_request_nft_id: NonFungibleId,
        ) -> Bucket
        {
            let badge_container: BadgeContainer = self.get_resource_manager(&loan_request_nft_id, Badges::LoanRequestNFT);

            match badge_container {
                BadgeContainer::LoanRequestNFTContainer(loan_request_nft_data) => {
                    let borrower_id: NonFungibleId = loan_request_nft_data.borrower;

                    let loan_request_nft: Bucket = self.global_loan_requests_vault
                    .get_mut(&borrower_id)
                    .unwrap()
                    .take_non_fungible(&loan_request_nft_id);
        
                    loan_request_nft
                }
                _ => { self.admin_vault.take(0) }
            }
        }

        /// This method is used by the BorrowerDashboard to return the Loan Request NFT. If the Borrower meets their
        /// collateralizaiton requirement, the Loan Request NFT is burnt. If not, the Loan Request NFT is returned
        /// to this component's vault.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Bucket passed contains the Loan Reuqest NFT in this protocol.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_request_nft` (Bucket) - The Bucket that contains the Loan Request NFT
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn return_loan_request_nft(
            &mut self,
            loan_request_nft: Bucket,
        )
        {
            assert_eq!(
                loan_request_nft.resource_address(), self.loan_request_nft_address,
                "The resource passed is not the Loan Request NFT."
            );

            let loan_request_nft_data: LoanRequest = loan_request_nft.non_fungible().data();
            let borrower_id: NonFungibleId = loan_request_nft_data.borrower;

            self.global_loan_requests_vault.get_mut(&borrower_id).unwrap().put(loan_request_nft);
        }

        /// This method is used to view the loan request of a particular borrower.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `borrower_id` (NonFungibleId) - The NFT ID of the Borrower.
        /// 
        /// # Returns:
        /// 
        /// * `BTreeSet<NonFungibleId>` - The list of all the Borrower's loan requests.
        pub fn view_loan_requests(
            &mut self,
            borrower_id: NonFungibleId,
        ) -> BTreeSet<NonFungibleId>
        {
            return self.global_loan_requests_vault
            .get_mut(&borrower_id)
            .unwrap()
            .non_fungible_ids()
            .clone();
        }

        /// This method is used to view all the loan request of all the Borrowers.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `HashMap<NonFungibleId, BTreeSet<NonFungibleId>>` - The HashMap of the Borrower NFT ID and NFT ID of all 
        /// Loan Request NFT from this Borrower.
        pub fn loan_request_list(
            &self,
        ) -> HashMap<NonFungibleId, BTreeSet<NonFungibleId>>
        {
            let mut loan_request_list: HashMap<NonFungibleId, BTreeSet<NonFungibleId>> = HashMap::new();
            let global_loan_requests_vault = self.global_loan_requests_vault.iter();
            for (borrowers, loan_request_vault) in global_loan_requests_vault {
                loan_request_list.insert(borrowers.clone(), loan_request_vault.non_fungible_ids());
            };

            loan_request_list
        }

        /// This method is used by the DebtFund component to enter a record of the Funding Lockers created. 
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks whether the Funding Locker for the particular Loan NFT ID already exist.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_id` (NonFungibleId) - The the NFT ID of the Loan NFT.
        /// * `funding_locker_address` (ComponentAddress) - The ComponentAddress of the Funding Locker.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything. 
        pub fn insert_funding_locker(
            &mut self,
            loan_id: NonFungibleId,
            funding_locker_address: ComponentAddress,
        )
        {
            assert_ne!(self.global_funding_lockers.contains_key(&loan_id), true, 
                "Funding Locker for this loan already exist."
            );

            self.global_funding_lockers.insert(loan_id, funding_locker_address);
        }

        /// This method is used to the retrieve the ComponentAddress of the Funding Locker.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Loan NFT exist.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_nft_id` (NonFungibleId) - The NFT ID of the Loan NFT.
        /// 
        /// # Returns:
        /// 
        /// * `ComponentAddress` - The ComponentAddress of the Funding Locker.
        pub fn retrieve_funding_locker_address(
            &self,
            loan_nft_id: NonFungibleId,
        ) -> ComponentAddress
        {
            assert_eq!(
                self.global_funding_lockers.contains_key(&loan_nft_id), true,
                "[Vault Protocol]: This loan does not exist."
            );

            return *self.global_funding_lockers.get(&loan_nft_id).unwrap();
        }

        /// This method is used by the FarmerDashboard to insert record of DebtFunds created.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks whether the Fund Name already exist.
        /// 
        /// This method has Access Rule imposed to require Farmers Dashboard Admin Badge present before
        /// the method can be called.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund.
        /// * `debt_fund_address` (ComponentAddress) - The ComponentAddress of the Debt Fund.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn insert_debt_fund(
            &mut self,
            fund_name: String,
            debt_fund_address: ComponentAddress,
        )
        {
            assert_ne!(self.global_debt_funds.contains_key(&fund_name), true, 
                "Fund name already exist, please use a different name"
            );

            self.global_debt_funds.insert(fund_name, debt_fund_address);
        }

        /// Asserts that the Index Fund exist.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund.
        /// 
        /// # Returns:
        /// 
        /// * `bool` - The bool of whether the fund exist or not.
        pub fn assert_index_fund(
            &self,
            fund_name: String,
        ) -> bool
        {
            let fund_id: (String, String) = self.get_index_name_pair(fund_name);
            return self.global_index_funds.contains_key(&fund_id);
        }

        /// This method is used to retrieve the Index Fund name pair which is the name of the fund and the symbol
        /// of the fund.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund.
        /// 
        /// # Returns:
        /// 
        /// * `(String, String)` - The Fund Name and the Fund Symbol.
        fn get_index_name_pair(
            &self,
            fund_name: String,
        ) -> (String, String)
        {
            let fund_ticker: String = self.global_index_funds_name.get(&fund_name).unwrap().to_string();
            let (fund_name, fund_ticker): (String, String) = sort_string(fund_name, fund_ticker);
            let fund_id: (String, String) = (fund_name, fund_ticker);
            fund_id
        }

        /// This method is used by the FarmerDashboard to record Index Funds created in this protocol.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks whether the fund name exists.
        /// 
        /// This method imposes and Access Rule that requires the Farmers Dashboard Admin Badge present.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_id` (String, String) - The Fund Name and the Fund Symbol.
        /// * `index_fund_address` (ComponentAddress) - The ComponentAddress of the Index Fund.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn insert_index_fund(
            &mut self,
            fund_id: (String, String),
            index_fund_address: ComponentAddress,
        )
        {
            assert_ne!(self.global_index_funds.contains_key(&fund_id), true, 
                "Fund ticker already exist, please use a different name"
            );

            self.global_index_funds.insert(fund_id, index_fund_address);
        }

        /// This method is used by the FarmerDashboard to record the fund name and fund ticker.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks whether the fund name exist.
        /// 
        /// This method has an Access Rule imposed that requires the Farmers Dashboard Admin Badge present.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The fund name.
        /// * `fund_ticker` (String) - The fund symbol.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.  
        pub fn insert_index_fund_name(
            &mut self,
            fund_name: String,
            fund_ticker: String,
        )
        {
            assert_ne!(self.global_index_funds_name.contains_key(&fund_name), true, 
                "Fund ticker already exist, please use a different name"
            );

            self.global_index_funds_name.insert(fund_name, fund_ticker);
        }

        /// Asserts whether the Index Fund name exist or not.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The fund name.
        /// 
        /// # Returns:
        /// 
        /// * `bool` - The bool whether the fund name exist or not.
        pub fn assert_index_fund_name(
            &self,
            fund_name: String,
        ) -> bool
        {
            return self.global_index_funds_name.contains_key(&fund_name);
        }

        /// This method is used to retrieve the Index Fund ComponentAddress.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arugments:
        /// 
        /// * `fund_name` (String) - The fund name.
        /// 
        /// # Returns:
        /// 
        /// * `ComponentAddress` - The ComponentAddress of the Index Fund.
        pub fn get_index_fund(
            &mut self,
            fund_name: String,
        ) -> ComponentAddress
        {
            let fund_id: (String, String) = self.get_index_name_pair(fund_name);
            return *self.global_index_funds.get(&fund_id).unwrap();
        }

        /// This method is used to retrieve a list of all the Index Fund created in this protocol.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `HashMap<(String, String), ComponentAddress>` - The HashMap of the fund_id (fund name, fund ticker) and the
        /// associated ComponentAddress.
        pub fn index_fund_list(
            &self,
        ) -> HashMap<(String, String), ComponentAddress>
        {
            let mut index_fund_list: HashMap<(String, String), ComponentAddress> = HashMap::new();
            let global_index_funds = self.global_index_funds.iter();
            for ((fund_name, fund_ticker), index_fund) in global_index_funds {
                index_fund_list.insert((fund_name.to_string(), fund_ticker.to_string()), index_fund.clone());
            }

            index_fund_list
        }

        /// This method is used by the FarmerDashboard to enter the record of LP tracking tokens of the Debt Funds.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks whether the tracking tokens already exist or not.
        /// 
        /// This method imposes an Access Rule that requires the Farmers Dashboard Admin Badge present.
        /// 
        /// # Arguments:
        /// 
        /// * `tracking_token_address` (ResourceAddress) - The ResourceAddress of the tracking tokens.
        /// * `fund_name` (String) - The fund name.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn insert_tracking_tokens(
            &mut self,
            tracking_token_address: ResourceAddress,
            fund_name: String,
        )
        {
            assert_ne!(
                self.global_tracking_tokens_address_mapping.contains_key(&tracking_token_address), true, 
                "Tracking token already exist"
            );

            self.global_tracking_tokens_address_mapping.insert(tracking_token_address, fund_name);
        }

        /// This method retrieves the tracking token pairs to identify which debt fund it belongs to.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that tracking token has an associated debt fund.
        /// 
        /// # Arguments:
        /// 
        /// * `tracking_token_address` (ResourceAddress) - The ResourceAddress of the tracking tokens.
        /// 
        /// # Returns:
        /// 
        /// * `String` - The name of the Debt Fund this tracking token belongs to.
        pub fn get_tracking_tokens_mapping(
            &mut self,
            tracking_token_address: ResourceAddress
        ) -> String
        {
            assert_eq!(
                self.global_tracking_tokens_address_mapping.contains_key(&tracking_token_address), true,
                "This Fund does not exist."
            );

            let fund_name: String = self.global_tracking_tokens_address_mapping
            .get_mut(&tracking_token_address)
            .unwrap()
            .to_string();

            fund_name
        }

        /// Retrieves all the debt funds created in this protocol.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `HashMap<String, ComponentAddress>` - The HashMap of the debt fund name and its associated
        /// ComponentAddress.
        pub fn debt_fund_list(
            &self,
        ) -> HashMap<String, ComponentAddress>
        {
            // let mut debt_fund_list: HashMap<NonFungibleId, HashSet<ComponentAddress>> = HashMap::new();
            // let global_debt_funds = self.global_debt_funds.iter();
            // for (farmer, debt_funds) in global_debt_funds {
            //     debt_fund_list.insert(farmer.clone(), debt_funds.clone());
            // }

            return self.global_debt_funds.clone();
        }

        /// This method is used to retrieve the ComponentAddress of the associated Debt Fund.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the debt fund exist.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund.
        /// 
        /// # Returns:
        /// 
        /// * `ComponentAddress` - The ComponentAddress of the Debt Fund.
        pub fn get_debt_fund(
            &self,
            fund_name: String,
        ) -> ComponentAddress
        {
            assert_eq!(
                self.global_debt_funds.contains_key(&fund_name), true,
                "This Fund does not exist."
            );

            return *self.global_debt_funds.get(&fund_name).unwrap();
        }
    }
}