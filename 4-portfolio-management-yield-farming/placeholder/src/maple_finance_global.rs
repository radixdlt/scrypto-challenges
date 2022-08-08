use scrypto::prelude::*;
use crate::pool_delegate_dashboard::*;
use crate::borrower_dashboard::*;
use crate::structs::*;
use crate::index_fund::*;
use crate::price_oracle::*;

blueprint! {
    struct MapleFinance {
        maple_finance_admin_address: ResourceAddress,
        admin_vault: Vault,
        loan_request_nft_admin_address: ResourceAddress,
        pool_delegates: HashSet<NonFungibleId>,
        pool_delegate_admin_address: ResourceAddress,
        pool_delegate_dashboards: HashMap<NonFungibleId, ComponentAddress>,
        pool_delegate_admin_badge_vault: Vault,
        borrowers: HashSet<NonFungibleId>,
        borrower_admin_address: ResourceAddress,
        borrower_dashboards: HashMap<NonFungibleId, ComponentAddress>,
        borrower_admin_badge_vault: Vault,
        investor_admin_address: ResourceAddress,
        temporary_badge_address: ResourceAddress,
        pending_approvals: HashMap<String, NonFungibleId>,
        approvals: HashMap<NonFungibleId, NonFungibleId>,
        loan_requests_global: HashMap<ResourceAddress, BTreeSet<NonFungibleId>>,
        global_index_funds: HashMap<String, ComponentAddress>,
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
        
            // NFT description for Pool Delegates
            let pool_delegate_admin_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Pool Delegate NFT")
                .metadata("symbol", "PDNFT")
                .metadata("description", "Pool Delegate Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();
            
            let investor_admin_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Investor NFT")
                .metadata("symbol", "INFT")
                .metadata("description", "Investor Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();
            
            let borrower_admin_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Borrower NFT")
                .metadata("symbol", "BNFT")
                .metadata("description", "Borrower Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();
      
            let temporary_badge_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Temporary Badge NFT")
                .metadata("symbol", "TBNFT")
                .metadata("description", "Temporary Badge NFT for Pool Delegates/Borrowers")
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
                pool_delegates: HashSet::new(),
                pool_delegate_admin_address: pool_delegate_admin_address,
                pool_delegate_dashboards: HashMap::new(),
                pool_delegate_admin_badge_vault: Vault::new(pool_delegate_admin_address),
                borrowers: HashSet::new(),
                borrower_admin_address: borrower_admin_address,
                borrower_dashboards: HashMap::new(),
                borrower_admin_badge_vault: Vault::new(borrower_admin_address),
                investor_admin_address: investor_admin_address,
                temporary_badge_address: temporary_badge_address,
                pending_approvals: HashMap::new(),
                approvals: HashMap::new(),
                loan_requests_global: HashMap::new(),
                global_index_funds: HashMap::new(),
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
                Badges::PoolDelegate => self.pool_delegate_admin_address,
                Badges::Investor => self.borrower_admin_address,
                Badges::Borrower => self.borrower_admin_address,
                Badges::TemporaryBadge => self.temporary_badge_address,
            };

            let resource_manager = borrow_resource_manager!(badge_address);

            match badge_name {
                Badges::PoolDelegate => {
                    let nft_data: PoolDelegate = resource_manager.get_non_fungible_data(&nft_id);
                    return BadgeContainer::PoolDelegateContainer(nft_data)
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
            }
        }

        fn authorize_update(
            &self,
            nft_id: &NonFungibleId,
            badge_name: Badges,
            nft_data: BadgeContainer)
        {
            let badge_address = match badge_name {
                Badges::PoolDelegate => self.pool_delegate_admin_address,
                Badges::Investor => self.investor_admin_address,
                Badges::Borrower => self.borrower_admin_address,
                Badges::TemporaryBadge => self.temporary_badge_address,
            };

            let resource_manager = borrow_resource_manager!(badge_address);
            
            match nft_data {
                BadgeContainer::PoolDelegateContainer(pool_delegate) => {
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

        /// Creates an admin badge for each Pool Delegates and instantiates a Pool Delegate Dashboard.
        /// 
        /// This method is used to allow authorized Maple Finance team to onboard approved Pool Delegates.
        /// Prospective Pool Delegates must first request approval to become a Pool Delegate by filing out the request form
        /// via create_temporary_badge method. Maple Finance team will view the approval request via pending_approvals 
        /// data field and approve selected Pool Delegates. A Pool Delegate admin badge will be minted where approved
        /// Pool Delegates can claim via their TemporaryBadge. A Pool Delegate Dashboard will be created where approved
        /// Pool Delegates can access their controls.
        /// 
        ///  
        pub fn create_pool_delegate(
            &mut self,
            maple_finance_admin: Proof,
            name: String) -> ComponentAddress
        {

            assert_eq!(maple_finance_admin.resource_address(), self.maple_finance_admin_address,
                "[Maple Finance]: Unauthorized Access."
            );

            // Mint Pool Delegate admin badge.
            let pool_delegate_admin_badge = self.admin_vault.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.pool_delegate_admin_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    PoolDelegate {
                        name: name.clone(),
                    },
                )
            });

            // Retrieve the prospective Pool Delegate via pending_approvals data field.
            let pending_user: &NonFungibleId = self.pending_approvals.get(&name).unwrap();

            // Retrieve the NFT data of the Temporary Badge of the prospective Pool Delegate.
            let nft_data = self.get_resource_manager(pending_user, Badges::TemporaryBadge);

            // Change NFT data of the Temporary Badge to indicate the prospective Pool Delegate has been approved.
            match nft_data {
                BadgeContainer::PoolDelegateContainer(_pool_delegate) => {}
                BadgeContainer::InvestorContainer(_investor) => {}
                BadgeContainer::BorrowerContainer(_borrower) => {}
                BadgeContainer::TemporaryBadgeContainer(mut temporary_badge) => {

                    assert_eq!(temporary_badge.user_type, UserType::PoolDelegate,
                        "[Maple Finance - Pool Delegate badge creation]: Incorrect user type."
                    );

                    temporary_badge.status = RequestStatus::Approved;

                    // Authorize update of the NFT data change.
                    self.authorize_update(
                        pending_user, 
                        Badges::TemporaryBadge, 
                        BadgeContainer::TemporaryBadgeContainer(temporary_badge),
                    );
                }
            };

            // Retrieve NFT ID of the Pool Delegate admin badge.
            let pool_delegate_id: NonFungibleId = pool_delegate_admin_badge.non_fungible::<PoolDelegate>().id();

            // Insert in the approvals data field. 
            self.approvals.insert(pending_user.clone(), pool_delegate_id.clone());

            // Remove prospective Pool Delegate from the pending_approvals data field.
            self.pending_approvals.remove_entry(&name);

            // Record the NFT ID of the new approved Pool Delegate.
            self.borrowers.insert(pool_delegate_id.clone());

            let loan_request_nft_admin = self.admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_request_nft_admin_address);
                    resource_manager.mint(1)
                }
            );

            let price_oracle_address: ComponentAddress = self.price_oracle_address.into();
            let maple_finance_global_address: ComponentAddress = self.maple_finance_global_address.unwrap().into();

            // Instantiates the Pool Delegate Dashboard for the recently approved Pool Delegate.
            let pool_delegate_dashboard: ComponentAddress = PoolDelegateDashboard::new(
                maple_finance_admin_address,
                self.pool_delegate_admin_address, 
                pool_delegate_id.clone(),
                loan_request_nft_admin,
                price_oracle_address,
            );

            // Insert the ComponentAddress of the Pool Delegate Dashboard for this particular Pool Delegate.
            self.pool_delegate_dashboards.insert(
                pool_delegate_id, 
                pool_delegate_dashboard
            );

            // Put the Pool Delegate admin badge for the recently approved Pool Delegate to claim.
            self.pool_delegate_admin_badge_vault.put(pool_delegate_admin_badge);

           pool_delegate_dashboard
        }

        pub fn create_borrower(
            &mut self,
            maple_finance_admin: Proof,
            name: String) -> ComponentAddress
        {
            assert_eq!(maple_finance_admin.resource_address(), self.maple_finance_admin_address,
                "[Maple Finance]: Unauthorized Access."
            );

            let borrower_admin_badge = self.admin_vault.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.borrower_admin_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    Borrower {
                        name: name.clone(),
                    },
                )
            });

            // Retrieve the prospective Borrower via pending_approvals data field.
            let pending_user: &NonFungibleId = self.pending_approvals.get(&name).unwrap();

            // Retrieve the NFT data of the Temporary Badge of the prospective Borrower.
            let nft_data = self.get_resource_manager(pending_user, Badges::TemporaryBadge);

            // Change NFT data of the Temporary Badge to indicate the prospective Pool Delegate has been approved.
            match nft_data {
                BadgeContainer::PoolDelegateContainer(_pooldelegate) => {}
                BadgeContainer::InvestorContainer(_investor) => {}
                BadgeContainer::BorrowerContainer(_borrower) => {}
                BadgeContainer::TemporaryBadgeContainer(mut temporary_badge) => {

                    assert_eq!(temporary_badge.user_type, UserType::Borrower,
                        "[Maple Finance - Pool Delegate badge creation]: Incorrect user type."
                    );

                    temporary_badge.status = RequestStatus::Approved;

                    // Authorize update of the NFT data change.
                    self.authorize_update(
                        pending_user, 
                        Badges::TemporaryBadge, 
                        BadgeContainer::TemporaryBadgeContainer(temporary_badge),
                    );
                }
            };

            let borrower_id = borrower_admin_badge.non_fungible::<PoolDelegate>().id();

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

            let maple_finance_global_address: ComponentAddress = self.maple_finance_global_address.unwrap().into();

            let borrower_dashboard: ComponentAddress = BorrowerDashboard::new(
                maple_finance_global_address,
                self.borrower_admin_address, 
                borrower_id.clone(), 
                loan_request_nft_admin
            );

            self.borrower_dashboards.insert(
                borrower_id,
                borrower_dashboard
            );

            self.borrower_admin_badge_vault.put(borrower_admin_badge);

            borrower_dashboard
        }

        /// Allows recently approved Pool Delegates to claim their admin badge to access their Pool Delegate Dashboard.
        /// 
        /// This method performs
        /// 
        /// * **Check 1:** - Checks that the Temporary Badge belongs to this protocol.
        pub fn claim_badge(
            &mut self,
            temporary_badge: Bucket) -> Bucket
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
                UserType::PoolDelegate => {
                    // Matches the Temporary Badge NFT ID with the approved Pool Delegate admin badge.
                    let claim_badge: &NonFungibleId = self.approvals.get(&temporary_badge_id).unwrap();

                    // Returns the Pool Delegate admin badge.
                    let return_pool_delegate_admin_badge: Bucket = self.pool_delegate_admin_badge_vault.take_non_fungible(claim_badge);

                    // Removes the entry from the approved list.
                    self.approvals.remove_entry(&temporary_badge_id);

                    self.admin_vault.authorize(||
                        temporary_badge.burn()
                    );

                    info!("[Maple Finance]: The resource address of your NFT is: {:?}", return_pool_delegate_admin_badge.resource_address());

                    return_pool_delegate_admin_badge
                }
                UserType::Borrower => {

                    // Matches the Temporary Badge NFT ID with the approved Pool Delegate admin badge.
                    let claim_badge: &NonFungibleId = self.approvals.get(&temporary_badge_id).unwrap();

                    // Returns the Pool Delegate admin badge.
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

        fn retrieve_loan_requests(
            &mut self)
        {
            let borrower_dashboards = self.borrower_dashboards.iter();
            for (borrower, _dashboards) in borrower_dashboards {
                let borrower_dashboard_address: ComponentAddress = *self.borrower_dashboards.get(&borrower).unwrap();
                let borrower_dashboard: BorrowerDashboard = borrower_dashboard_address.into();
                let loan_requests: HashMap<ResourceAddress, BTreeSet<NonFungibleId>> = borrower_dashboard.broadcast_loan_requests();
                let loan_requests_iter = loan_requests.iter();
                for (loan_requests_nft_address, loan_requests_nft_id) in loan_requests_iter {
                    self.loan_requests_global.insert(
                        *loan_requests_nft_address, 
                        loan_requests_nft_id.clone()
                    );
                }
            }
        }

        pub fn broadcast_loan_requests(
            &mut self) -> HashMap<ResourceAddress, BTreeSet<NonFungibleId>>
        {
            self.retrieve_loan_requests();
            return self.loan_requests_global.clone()
        }

        pub fn insert_index_fund(
            &mut self,
            fund_ticker: String,
            index_fund_address: ComponentAddress,
        )
        {
            assert_ne!(self.global_index_funds.contains_key(&fund_ticker), true, 
                "Fund ticker already exist, please use a different name"
            );

            self.global_index_funds.insert(fund_ticker, index_fund_address);
        }

        pub fn retrieve_index_fund(
            &self,
            fund_ticker: String,
        ) -> bool
        {
            return self.global_index_funds.contains_key(&fund_ticker);
        }
    }
}