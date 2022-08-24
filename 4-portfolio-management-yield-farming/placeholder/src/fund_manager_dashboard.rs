use scrypto::prelude::*;
use crate::index_fund::*;
use crate::debt_fund::*;
use crate::maple_finance_global::*;
use crate::structs::*;
use crate::utils::*;

// Allows approved Fund Manager to manage pools.

blueprint! {
    /// This is a struct used to define the FundManagerDashboard. This blueprint is used to allow Fund Managers to create new Index Fund and Debt Fund strategies.
    /// Unfortunately, this blueprint is not designed to have Fund Managers to interface with their Debt Fund and Index Fund. 
    struct FundManagerDashboard {
        /// The ResourceAddress of the Fund Manager Badge provided to Fund Managers used to provide authorized access 
        /// Fund Manager Dashboard.
        fund_manager_address: ResourceAddress,
        /// The Vault that contains the FundManagerDashboard Admin token. This is held by the component to make authorized
        /// cross blueprint method calls.
        fund_manager_dashboard_admin_vault: Vault,
        loan_request_nft_admin: Vault,
        loan_request_nft_address: ResourceAddress,
        loan_nft_admin_address: ResourceAddress,
        price_oracle_address: ComponentAddress,
        maple_finance_global_address: ComponentAddress,

        access_badge_vault: Option<Vault>,
    }

    impl FundManagerDashboard {

        pub fn new(
            fund_manager_dashboard_admin: Bucket,
            maple_finance_global_address: ComponentAddress,
            fund_manager_address: ResourceAddress,
            loan_request_nft_admin: Bucket,
            loan_request_nft_address: ResourceAddress,
            price_oracle_address: ComponentAddress,
        ) -> ComponentAddress
        {
            let loan_nft_admin_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Loan NFT Admin Badge")
                .metadata("symbol", "LNFTAB")
                .metadata("description", "Allows Fund Managers to mint/burn loan NFTs.")
                .mintable(rule!(require(fund_manager_dashboard_admin.resource_address())), LOCKED)
                .burnable(rule!(require(fund_manager_dashboard_admin.resource_address())), LOCKED)
                .no_initial_supply();

            return Self {
                fund_manager_dashboard_admin_vault: Vault::with_bucket(fund_manager_dashboard_admin),
                fund_manager_address: fund_manager_address,
                loan_request_nft_admin: Vault::with_bucket(loan_request_nft_admin),
                loan_request_nft_address: loan_request_nft_address,
                loan_nft_admin_address: loan_nft_admin_address,
                price_oracle_address: price_oracle_address,
                maple_finance_global_address: maple_finance_global_address,
                access_badge_vault: None, 
            }
            .instantiate()
            .globalize();
        }

        pub fn view_component_address(
            &self,
        ) -> Option<ComponentAddress>
        {
            let component_address = Runtime::actor().component_address();
            component_address
        }

        pub fn new_debt_fund(
            &mut self,
            fund_manager_badge: Proof,
            fund_name: String,
            initial_funds: Bucket
        ) -> (ComponentAddress, Bucket, Bucket)
        {
            //Logic to check if the there's a duplicate lending pool
            assert_eq!(fund_manager_badge.resource_address(), self.fund_manager_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

            let fund_manager_id: NonFungibleId = fund_manager_badge.non_fungible::<FundManager>().id();
            let token_address: ResourceAddress = initial_funds.resource_address();

            let fund_manager_data: FundManager = self.get_resource_manager(&fund_manager_id);
            let fund_manager_name: String = fund_manager_data.name;

            let loan_nft_admin: Bucket = self.fund_manager_dashboard_admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_nft_admin_address);
                    resource_manager.mint(1)
                }
            );

            let optional_fund_manager_dashboard_address: Option<ComponentAddress> = self.view_component_address();

            // Instantiates the lending pool.
            let (debt_fund, tracking_tokens, debt_fund_manager_badge): (ComponentAddress, Bucket, Bucket) = DebtFund::new(
                self.maple_finance_global_address,
                optional_fund_manager_dashboard_address,
                self.price_oracle_address,
                fund_manager_name,
                fund_manager_badge.resource_address(),
                fund_manager_id.clone(),
                self.loan_request_nft_address,
                loan_nft_admin,
                initial_funds
            );

            // * INSERTS LENDING POOL DATA INTO FUND MANAGER NFT * //
            // Resource Address is used as the key for the HashMap to allow Fund Managers
            // to find their lending pools easier. In the future, Fund Managers may
            // have multiple lending pools with the same supported tokens which may cause
            // duplication issues with the way this is set up. However, for this purposes
            // we'll just have it as the token's Resource Address for simplicity.
            let mut fund_manager_data: FundManager = self.get_resource_manager(&fund_manager_id);
            fund_manager_data.managed_debt_funds.insert(token_address, debt_fund);
            self.authorize_update(&fund_manager_badge, fund_manager_data);

            // * INSERTS LENDING POOL DATA TO THE GLOBAL INDEX * //
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            maple_finance.insert_debt_fund(fund_name.clone(), debt_fund);
            maple_finance.insert_tracking_tokens(tracking_tokens.resource_address(), fund_name);

            (debt_fund, tracking_tokens, debt_fund_manager_badge)
        }

        // pub fn retrieve_loan_requests(
        //     &self) -> HashMap<ResourceAddress, BTreeSet<NonFungibleId>>
        // {
        //     let maple_finance_global: MapleFinance = self.maple_finance_global_address.unwrap().into();
        //     let loan_requests = maple_finance_global.broadcast_loan_requests();
        //     loan_requests
        // }

        pub fn new_index_fund(
            &mut self,
            fund_manager_badge: Proof,
            fund_name: String,
            fee_to_pool: Decimal,
            fund_ticker: String,
            starting_share_price: Decimal,
            tokens: HashMap<ResourceAddress, Decimal>,
        ) -> Bucket
        {
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();

            assert_eq!(
                fund_manager_badge.resource_address(), self.fund_manager_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

            assert_ne!(
                maple_finance.assert_index_fund_name(fund_name.clone()), true, 
                "[Fund Manager Dashboard]: The name or ticker for this fund already exist. Please choose another."
            );

            let (fund_name, fund_ticker): (String, String) = sort_string(fund_name.clone(), fund_ticker);
            let fund_id: (String, String) = (fund_name.clone(), fund_ticker.clone());

            let price_oracle_address: ComponentAddress = self.price_oracle_address;

            let (index_fund, fund_admin): (ComponentAddress, Bucket) = IndexFund::new(
                fund_name.clone(), 
                fund_ticker.clone(),
                fee_to_pool, 
                starting_share_price,
                tokens,
                price_oracle_address,
            );

            let fund_manager_id: NonFungibleId = fund_manager_badge.non_fungible::<FundManager>().id();
            
            let mut fund_manager_data: FundManager = self.get_resource_manager(&fund_manager_id);

            fund_manager_data.managed_index_funds.insert(fund_id.clone(), index_fund);
            
            self.authorize_update(&fund_manager_badge, fund_manager_data);

            maple_finance.insert_index_fund_name(fund_name.clone(), fund_ticker.clone());
            maple_finance.insert_index_fund(fund_id, index_fund);

            fund_admin
        }

        pub fn view_managed_index_funds(
            &self,
            fund_manager_badge: Proof,
        )
        {
            assert_eq!(fund_manager_badge.resource_address(), self.fund_manager_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

        }

        fn get_resource_manager(
            &self,
            fund_manager_id: &NonFungibleId,
        ) -> FundManager
        {
            let resource_manager = borrow_resource_manager!(self.fund_manager_address);
            let fund_manager_data: FundManager = resource_manager.get_non_fungible_data(&fund_manager_id);

            fund_manager_data 
        }

        fn authorize_update(
            &self,
            fund_manager_badge: &Proof,
            fund_manager_data: FundManager
        )
        {
            let fund_manager_id: NonFungibleId = fund_manager_badge.non_fungible::<FundManager>().id();
            let resource_manager = borrow_resource_manager!(self.fund_manager_address);
            self.fund_manager_dashboard_admin_vault.authorize(|| 
                resource_manager.update_non_fungible_data(&fund_manager_id, fund_manager_data)
            );
        }
        
        pub fn insert_funding_locker(
            &mut self,
            loan_nft_admin: Proof,
            loan_id: NonFungibleId,
            funding_locker_address: ComponentAddress,
        )
        {
            assert_eq!(
                loan_nft_admin.resource_address(), self.loan_nft_admin_address,
                "[Fund Manager Dashboard - Insert Funding Locker]: Incorrect Loan NFT Badge."
            );

            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            self.fund_manager_dashboard_admin_vault.create_proof();
            maple_finance.insert_funding_locker(loan_id, funding_locker_address);
        }
    }
}