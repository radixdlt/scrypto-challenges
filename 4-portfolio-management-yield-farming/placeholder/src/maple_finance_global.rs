use scrypto::prelude::*;
use crate::fund_manager_dashboard::*;
use crate::borrower_dashboard::*;
use crate::structs::*;
use crate::price_oracle::*;
use crate::utils::*;

blueprint! {
    struct MapleFinance {
        maple_finance_admin_address: ResourceAddress,
        admin_vault: Vault,
        loan_request_nft_admin_address: ResourceAddress,
        loan_request_nft_address: ResourceAddress,
        pool_delegates: HashSet<NonFungibleId>,
        fund_manager_address: ResourceAddress,
        fund_manager_admin_address: ResourceAddress,
        fund_manager_dashbaords: HashMap<NonFungibleId, ComponentAddress>,
        fund_manager_badge_vault: Vault,
        borrowers: HashSet<NonFungibleId>,
        borrower_address: ResourceAddress,
        borrower_dashboards: HashMap<NonFungibleId, ComponentAddress>,
        borrower_admin_badge_vault: Vault,
        investor_admin_address: ResourceAddress,
        temporary_badge_address: ResourceAddress,
        pending_approvals: HashMap<String, NonFungibleId>,
        approvals: HashMap<NonFungibleId, NonFungibleId>,
        global_loan_requests_vault: HashMap<NonFungibleId, Vault>,
        global_debt_funds: HashMap<NonFungibleId, HashSet<ComponentAddress>>,
        global_funding_lockers: HashMap<NonFungibleId, ComponentAddress>,
        global_index_funds: HashMap<(String, String), ComponentAddress>,
        global_index_funds_name: HashMap<String, String>,
        price_oracle_address: ComponentAddress,
        maple_finance_global_address: Option<ComponentAddress>,
    }

    impl MapleFinance {

        pub fn new() -> (ComponentAddress, Bucket)
        {   
            // Badge that will be stored in the component's vault to update loan NFT.
            let admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .metadata("symbol", "AB")
                .metadata("description", "Component Admin authority")
                .initial_supply(1);

            let maple_finance_admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Maple Finance Admin Badge")
                .metadata("symbol", "MFAB")
                .metadata("description", "Maple Finance")
                .initial_supply(1);
                
            // Badge that will be stored in the component's vault to update loan NFT.
            let loan_request_nft_admin_address: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .metadata("symbol", "AB")
                .metadata("description", "Admin authority")
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

            let fund_manager_admin_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Fund Manager Admin Badge")
                .metadata("symbol", "FM_AB")
                .metadata("description", "Admin Badge to control Fund Manager Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();

            let borrower_admin_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Borrower Admin Badge")
                .metadata("symbol", "B_AB")
                .metadata("description", "Admin Badge to control Fund Manager Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();

            let allowed_badge: Vec<ResourceAddress> = vec!(
                admin.resource_address(), 
                fund_manager_admin_address,
            );

            // NFT description for Fund Managers
            let fund_manager_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Fund Manager NFT")
                .metadata("symbol", "PDNFT")
                .metadata("description", "Fund Manager Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require_any_of(allowed_badge)), LOCKED)
                .no_initial_supply();
            
            let investor_admin_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Investor NFT")
                .metadata("symbol", "INFT")
                .metadata("description", "Investor Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();
            
            let allowed_badge: Vec<ResourceAddress> = vec!(
                admin.resource_address(), 
                borrower_admin_address,
            );

            let borrower_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Borrower NFT")
                .metadata("symbol", "BNFT")
                .metadata("description", "Borrower Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require_any_of(allowed_badge)), LOCKED)
                .no_initial_supply();
      
            let temporary_badge_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Temporary Badge NFT")
                .metadata("symbol", "TBNFT")
                .metadata("description", "Temporary Badge NFT for Fund Managers/Borrowers")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();

            // let access_rules: AccessRules = AccessRules::new()
            //     .method("create_temporary_badge", rule!(allow_all))
            //     .method("claim_badge", rule!(allow_all))
            //     .method("retrieve_loan_requests", rule!(allow_all))
            //     .method("broadcast_loan_requests", rule!(allow_all))
            //     .default(rule!(require(maple_finance_admin.resource_address())));
            
            let maple_finance = Self {
                maple_finance_admin_address: maple_finance_admin.resource_address(),
                admin_vault: Vault::with_bucket(admin),
                loan_request_nft_admin_address: loan_request_nft_admin_address,
                loan_request_nft_address: loan_request_nft_address,
                pool_delegates: HashSet::new(),
                fund_manager_address: fund_manager_address,
                fund_manager_admin_address: fund_manager_admin_address,
                fund_manager_dashbaords: HashMap::new(),
                fund_manager_badge_vault: Vault::new(fund_manager_address),
                borrowers: HashSet::new(),
                borrower_address: borrower_address,
                borrower_dashboards: HashMap::new(),
                borrower_admin_badge_vault: Vault::new(borrower_address),
                investor_admin_address: investor_admin_address,
                temporary_badge_address: temporary_badge_address,
                pending_approvals: HashMap::new(),
                approvals: HashMap::new(),
                global_loan_requests_vault: HashMap::new(),
                global_debt_funds: HashMap::new(),
                global_funding_lockers: HashMap::new(),
                global_index_funds: HashMap::new(),
                global_index_funds_name: HashMap::new(),
                price_oracle_address: PriceOracle::new(),
                maple_finance_global_address: None,
            }
            .instantiate()
            // .add_access_check(access_rules)
            .globalize();

            (maple_finance, maple_finance_admin)
        }

        pub fn set_address(
            &mut self,
            maple_finance_global_address: ComponentAddress
        )
        {
            self.maple_finance_global_address = Some(maple_finance_global_address);
        }

        fn get_resource_manager(
            &self,
            nft_id: &NonFungibleId,
            badge_name: Badges) -> BadgeContainer
        {
            let badge_address = match badge_name {
                Badges::FundManager => self.fund_manager_address,
                Badges::Investor => self.borrower_address,
                Badges::Borrower => self.borrower_address,
                Badges::TemporaryBadge => self.temporary_badge_address,
                Badges::LoanRequestNFT => self.loan_request_nft_address,
            };

            let resource_manager = borrow_resource_manager!(badge_address);

            match badge_name {
                Badges::FundManager => {
                    let nft_data: FundManager = resource_manager.get_non_fungible_data(&nft_id);
                    return BadgeContainer::FundManagerContainer(nft_data)
                }
                Badges::Investor => {
                    let nft_data: Investor = resource_manager.get_non_fungible_data(&nft_id);
                    return BadgeContainer::InvestorContainer(nft_data)
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

        fn authorize_update(
            &self,
            nft_id: &NonFungibleId,
            badge_name: Badges,
            nft_data: BadgeContainer)
        {
            let badge_address = match badge_name {
                Badges::FundManager => self.fund_manager_address,
                Badges::Investor => self.investor_admin_address,
                Badges::Borrower => self.borrower_address,
                Badges::TemporaryBadge => self.temporary_badge_address,
                Badges::LoanRequestNFT => self.loan_request_nft_address,
            };

            let resource_manager = borrow_resource_manager!(badge_address);
            
            match nft_data {
                BadgeContainer::FundManagerContainer(pool_delegate) => {
                    self.admin_vault.authorize(|| 
                        resource_manager.update_non_fungible_data(nft_id, pool_delegate)
                    );
                }
                BadgeContainer::InvestorContainer(investor) => {
                    self.admin_vault.authorize(|| 
                        resource_manager.update_non_fungible_data(nft_id, investor)
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
        // Provide Fund Manager Proof? No.
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

        pub fn deposit_loan_requests(
            &mut self,
            borrower_badge: Proof,
            loan_request_nft: Bucket
        )
        {
            assert_eq!(
                borrower_badge.resource_address(), self.borrower_address,
                "[Maple Finance]: The badge does not belong to this protocol."
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

        pub fn create_temporary_badge(
            &mut self,
            name: String,
            user_type: UserType) -> Bucket
        {
            assert!(self.pending_approvals.contains_key(&name) != true,
                "[Maple Finance]: The name you provided already exists."
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

            info!("[Maple Finance]: The resource address of your temporary badge is: {:?}", temporary_badge.resource_address());

            temporary_badge
        }

        /// Creates an admin badge for each Fund Managers and instantiates a Fund Manager Dashboard.
        /// 
        /// This method is used to allow authorized Maple Finance team to onboard approved Fund Managers.
        /// Prospective Fund Managers must first request approval to become a Fund Manager by filing out the request form
        /// via create_temporary_badge method. Maple Finance team will view the approval request via pending_approvals 
        /// data field and approve selected Fund Managers. A Fund Manager admin badge will be minted where approved
        /// Fund Managers can claim via their TemporaryBadge. A Fund Manager Dashboard will be created where approved
        /// Fund Managers can access their controls.
        /// 
        ///  
        pub fn create_fund_manager(
            &mut self,
            maple_finance_admin: Proof,
            name: String
        ) -> ComponentAddress
        {

            assert_eq!(maple_finance_admin.resource_address(), self.maple_finance_admin_address,
                "[Maple Finance]: Unauthorized Access."
            );

            // Mint Fund Manager admin badge.
            let fund_manager_badge = self.admin_vault.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.fund_manager_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    FundManager {
                        name: name.clone(),
                        managed_index_funds: HashMap::new(),
                        managed_debt_funds: HashMap::new(),
                    },
                )
            });

            // Retrieve the prospective Fund Manager via pending_approvals data field.
            let pending_user: &NonFungibleId = self.pending_approvals.get(&name).unwrap();

            // Retrieve the NFT data of the Temporary Badge of the prospective Fund Manager.
            let nft_data = self.get_resource_manager(pending_user, Badges::TemporaryBadge);

            // Change NFT data of the Temporary Badge to indicate the prospective Fund Manager has been approved.
            match nft_data {
                BadgeContainer::FundManagerContainer(_pool_delegate) => {}
                BadgeContainer::InvestorContainer(_investor) => {}
                BadgeContainer::BorrowerContainer(_borrower) => {}
                BadgeContainer::TemporaryBadgeContainer(mut temporary_badge) => {

                    assert_eq!(temporary_badge.user_type, UserType::FundManager,
                        "[Maple Finance - Fund Manager badge creation]: Incorrect user type."
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

            // Retrieve NFT ID of the Fund Manager admin badge.
            let fund_manager_id: NonFungibleId = fund_manager_badge.non_fungible::<FundManager>().id();

            // Insert in the approvals data field. 
            self.approvals.insert(pending_user.clone(), fund_manager_id.clone());

            // Remove prospective Fund Manager from the pending_approvals data field.
            self.pending_approvals.remove_entry(&name);

            // Record the NFT ID of the new approved Fund Manager.
            self.borrowers.insert(fund_manager_id.clone());

            let loan_request_nft_admin = self.admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_request_nft_admin_address);
                    resource_manager.mint(1)
                }
            );

            let fund_manager_admin: Bucket = self.admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.fund_manager_admin_address);
                    resource_manager.mint(1)
                }
            );

            let price_oracle_address: ComponentAddress = self.price_oracle_address.into();
            let maple_finance_global_address: ComponentAddress = self.maple_finance_global_address.unwrap().into();

            // Instantiates the Fund Manager Dashboard for the recently approved Fund Manager.
            let pool_delegate_dashboard: ComponentAddress = FundManagerDashboard::new(
                fund_manager_admin,
                maple_finance_global_address,
                self.fund_manager_address, 
                loan_request_nft_admin,
                self.loan_request_nft_address,
                price_oracle_address,
            );

            // Insert the ComponentAddress of the Fund Manager Dashboard for this particular Fund Manager.
            self.fund_manager_dashbaords.insert(
                fund_manager_id, 
                pool_delegate_dashboard
            );

            // Put the Fund Manager admin badge for the recently approved Fund Manager to claim.
            self.fund_manager_badge_vault.put(fund_manager_badge);

           pool_delegate_dashboard
        }

        pub fn create_borrower(
            &mut self,
            maple_finance_admin: Proof,
            name: String
        ) -> ComponentAddress
        {
            assert_eq!(maple_finance_admin.resource_address(), self.maple_finance_admin_address,
                "[Maple Finance]: Unauthorized Access."
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

            // Change NFT data of the Temporary Badge to indicate the prospective Fund Manager has been approved.
            match nft_data {
                BadgeContainer::FundManagerContainer(_fundmanager) => {}
                BadgeContainer::InvestorContainer(_investor) => {}
                BadgeContainer::BorrowerContainer(_borrower) => {}
                BadgeContainer::TemporaryBadgeContainer(mut temporary_badge) => {

                    assert_eq!(temporary_badge.user_type, UserType::Borrower,
                        "[Maple Finance - Fund Manager badge creation]: Incorrect user type."
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

            let borrower_id = borrower_admin_badge.non_fungible::<FundManager>().id();

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
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.borrower_address);
                    resource_manager.mint(1)
                }
            );

            let maple_finance_global_address: ComponentAddress = self.maple_finance_global_address.unwrap().into();

            let borrower_dashboard: ComponentAddress = BorrowerDashboard::new(
                maple_finance_global_address,
                borrower_admin,
                self.borrower_address, 
                loan_request_nft_admin,
                self.loan_request_nft_address,
            );

            self.borrower_dashboards.insert(
                borrower_id,
                borrower_dashboard
            );

            self.borrower_admin_badge_vault.put(borrower_admin_badge);

            borrower_dashboard
        }

        /// Allows recently approved Fund Managers to claim their admin badge to access their Fund Manager Dashboard.
        /// 
        /// This method performs
        /// 
        /// * **Check 1:** - Checks that the Temporary Badge belongs to this protocol.
        pub fn claim_badge(
            &mut self,
            temporary_badge: Bucket
        ) -> Bucket
        {
            assert_eq!(temporary_badge.resource_address(), self.temporary_badge_address,
                "[Maple Finance]: This badge does not belong to this protocol."
            );
            
            // Retrieves the Temporary Badge NFT ID.
            let temporary_badge_id: NonFungibleId = temporary_badge.non_fungible::<TemporaryBadge>().id();

            //
            let temporary_badge_data: TemporaryBadge = temporary_badge.non_fungible().data();

            let user_type = temporary_badge_data.user_type;

            match user_type {
                UserType::FundManager => {
                    // Matches the Temporary Badge NFT ID with the approved Fund Manager admin badge.
                    let claim_badge: &NonFungibleId = self.approvals.get(&temporary_badge_id).unwrap();

                    // Returns the Fund Manager admin badge.
                    let return_fund_manager_badge: Bucket = self.fund_manager_badge_vault.take_non_fungible(claim_badge);

                    // Removes the entry from the approved list.
                    self.approvals.remove_entry(&temporary_badge_id);

                    self.admin_vault.authorize(||
                        temporary_badge.burn()
                    );

                    info!("[Maple Finance]: The resource address of your NFT is: {:?}", return_fund_manager_badge.resource_address());

                    return_fund_manager_badge
                }
                UserType::Borrower => {

                    // Matches the Temporary Badge NFT ID with the approved Fund Manager admin badge.
                    let claim_badge: &NonFungibleId = self.approvals.get(&temporary_badge_id).unwrap();

                    // Returns the Fund Manager admin badge.
                    let return_borrower_admin_badge: Bucket = self.borrower_admin_badge_vault.take_non_fungible(claim_badge);

                    // Removes the entry from the approved list.
                    self.approvals.remove_entry(&temporary_badge_id);

                    self.admin_vault.authorize(||
                        temporary_badge.burn()
                    );

                    info!("[Maple Finance]: The resource address of your NFT is: {:?}", return_borrower_admin_badge.resource_address());

                    return_borrower_admin_badge
                }
            }
        }

        pub fn retrieve_loan_request_nft(
            &mut self,
            borrower_badge: Proof,
            loan_request_nft_id: NonFungibleId,
        ) -> Bucket
        {
            assert_eq!(
                borrower_badge.resource_address(), self.borrower_address,
                "[Maple Finance]: Badge does not belong to this protocol."
            );

            let borrower_id: NonFungibleId = borrower_badge.non_fungible::<Borrower>().id();

            let loan_request_nft: Bucket = self.global_loan_requests_vault
            .get_mut(&borrower_id)
            .unwrap()
            .take_non_fungible(&loan_request_nft_id);

            loan_request_nft
        }

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

        /// Implement Access Control. Only Debt Fund component can call this method.
        pub fn insert_funding_lockers(
            &mut self,
            loan_id: NonFungibleId,
            funding_locker_address: ComponentAddress,
        )
        {
            assert_ne!(self.global_funding_lockers.contains_key(&loan_id), true, 
                "Pool name already exist, please use a different name"
            );

            self.global_funding_lockers.insert(loan_id, funding_locker_address);
        }

        /// No access control needed since Funding Locker requires proof to access.
        pub fn retrieve_funding_locker_address(
            &self,
            loan_nft_id: NonFungibleId,
        ) -> ComponentAddress
        {
            assert_eq!(
                self.global_funding_lockers.contains_key(&loan_nft_id), true,
                "[Maple Finance]: This loan does not exist."
            );

            return *self.global_funding_lockers.get(&loan_nft_id).unwrap();
        }

        /// Implement Access Control. Only Fund Manager Dashboard can call this method.
        /// Have Fund Manager Proof?
        pub fn insert_debt_fund(
            &mut self,
            fund_manager_id: NonFungibleId,
            debt_fund_address: ComponentAddress,
        )
        {
            assert_ne!(self.global_debt_funds.contains_key(&fund_manager_id), true, 
                "Pool name already exist, please use a different name"
            );

            let mut hashset: HashSet<ComponentAddress> = HashSet::new();
            hashset.insert(debt_fund_address);

            self.global_debt_funds.insert(fund_manager_id, hashset);
        }

        pub fn assert_index_fund(
            &self,
            fund_name: String,
        ) -> bool
        {
            let fund_id: (String, String) = self.get_index_name_pair(fund_name);
            return self.global_index_funds.contains_key(&fund_id);
        }

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

        /// Implement Access Control. Only Fund Manager Dashboard can call this method.
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

        /// Implement Access Control. Only Fund Manager Dashboard can call this method.
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

        pub fn assert_index_fund_name(
            &self,
            fund_name: String,
        ) -> bool
        {
            return self.global_index_funds_name.contains_key(&fund_name);
        }

        pub fn get_index_fund(
            &mut self,
            fund_name: String,
        ) -> ComponentAddress
        {
            let fund_id: (String, String) = self.get_index_name_pair(fund_name);
            return *self.global_index_funds.get_mut(&fund_id).unwrap();
        }

        pub fn index_fund_list(
            &self,
        ) -> HashMap<(String, String), ComponentAddress>
        {
            let mut index_fund_lists: HashMap<(String, String), ComponentAddress> = HashMap::new();
            let global_index_funds = self.global_index_funds.iter();
            for ((fund_name, fund_ticker), index_fund) in global_index_funds {
                index_fund_lists.insert((fund_name.to_string(), fund_ticker.to_string()), index_fund.clone());
            }

            index_fund_lists
        }
    }
}