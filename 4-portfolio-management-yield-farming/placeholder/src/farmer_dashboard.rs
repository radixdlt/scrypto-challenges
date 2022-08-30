use scrypto::prelude::*;
use crate::index_fund::*;
use crate::debt_fund::*;
use crate::farmers_market::*;
use crate::structs::*;
use crate::utils::*;

// Allows approved Fund Manager to manage pools.

blueprint! {
    /// This is a struct used to define the FarmerDashboard. This blueprint is used to allow Fund Managers to create new Index Fund and Debt Fund strategies.
    /// Unfortunately, this blueprint is not designed to have Fund Managers to interface with their Debt Fund and Index Fund. 
    struct FarmerDashboard {
        /// The ResourceAddress of the Fund Manager Badge provided to Fund Managers used to provide authorized access 
        /// Fund Manager Dashboard.
        farmer_address: ResourceAddress,
        /// The Vault that contains the FarmerDashboard Admin token. This is held by the component to make authorized
        /// cross blueprint method calls.
        farmer_dashboard_admin_vault: Vault,
        /// The Loan Request NFT Admin to allow the component to update the Loan Request NFT.
        loan_request_nft_admin: Vault,
        /// The Loan Request NFT ResourceAddress to allow the component to view Loan Request NFT data.
        loan_request_nft_address: ResourceAddress,
        /// The Loan NFT Admin that manages the Loan NFT. This badge is created in this component so that when it is
        /// also minted for the Debt Fund component, both components can communicate with each other with Access Rules imposed.
        loan_nft_admin_address: ResourceAddress,
        /// The 
        debt_fund_admin_address: ResourceAddress,
        price_oracle_address: ComponentAddress,
        farmers_market_global_address: ComponentAddress,
    }

    impl FarmerDashboard {

        pub fn new(
            farmer_dashboard_admin: Bucket,
            farmers_market_global_address: ComponentAddress,
            farmer_address: ResourceAddress,
            loan_request_nft_admin: Bucket,
            loan_request_nft_address: ResourceAddress,
            price_oracle_address: ComponentAddress,
        ) -> ComponentAddress
        {
            let debt_fund_admin_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Debt Fund Admin Badge")
                .metadata("symbol", "DF_AB")
                .metadata("description", "Debt Fund Admin Badge for components.")
                .mintable(rule!(require(farmer_dashboard_admin.resource_address())), LOCKED)
                .burnable(rule!(require(farmer_dashboard_admin.resource_address())), LOCKED)
                .no_initial_supply();
            
            let allowed_badge: Vec<ResourceAddress> = vec!(
                farmer_dashboard_admin.resource_address(), 
                debt_fund_admin_address,
            );                

            let loan_nft_admin_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Loan NFT Admin Badge")
                .metadata("symbol", "LNFTAB")
                .metadata("description", "Allows Fund Managers to mint/burn loan NFTs.")
                .mintable(rule!(require_any_of(allowed_badge)), LOCKED)
                .burnable(rule!(require(farmer_dashboard_admin.resource_address())), LOCKED)
                .no_initial_supply();

            let access_rules: AccessRules = AccessRules::new()
                .method("insert_funding_locker", rule!(require(debt_fund_admin_address)))
                .default(rule!(allow_all)
            );

            return Self {
                farmer_dashboard_admin_vault: Vault::with_bucket(farmer_dashboard_admin),
                farmer_address: farmer_address,
                loan_request_nft_admin: Vault::with_bucket(loan_request_nft_admin),
                loan_request_nft_address: loan_request_nft_address,
                loan_nft_admin_address: loan_nft_admin_address,
                debt_fund_admin_address: debt_fund_admin_address,
                price_oracle_address: price_oracle_address,
                farmers_market_global_address: farmers_market_global_address,
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();
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

        /// This method allows Fund Managers to create a new Debt Fund.
        /// 
        /// A 
        pub fn new_debt_fund(
            &mut self,
            farmer_badge: Proof,
            fund_name: String,
            initial_funds: Bucket
        ) -> (ComponentAddress, Bucket, Bucket)
        {
            assert_eq!(farmer_badge.resource_address(), self.farmer_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

            let farmer_id: NonFungibleId = farmer_badge.non_fungible::<Farmer>().id();
            let token_address: ResourceAddress = initial_funds.resource_address();

            let farmer_data: Farmer = self.get_resource_manager(&farmer_id);
            let farmer_name: String = farmer_data.name;

            let debt_fund_admin: Bucket = self.farmer_dashboard_admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.debt_fund_admin_address);
                    resource_manager.mint(1)
                }
            );

            let loan_nft_admin: Bucket = self.farmer_dashboard_admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_nft_admin_address);
                    resource_manager.mint(1)
                }
            );

            let optional_farmer_dashboard_address: Option<ComponentAddress> = self.view_component_address();

            // Instantiates the lending pool.
            let (debt_fund, tracking_tokens, debt_farmer_badge): (ComponentAddress, Bucket, Bucket) = DebtFund::new(
                self.farmers_market_global_address,
                optional_farmer_dashboard_address,
                self.price_oracle_address,
                farmer_name,
                farmer_badge.resource_address(),
                farmer_id.clone(),
                self.loan_request_nft_address,
                debt_fund_admin,
                loan_nft_admin,
                initial_funds
            );

            // * INSERTS LENDING POOL DATA INTO FUND MANAGER NFT * //
            // Resource Address is used as the key for the HashMap to allow Fund Managers
            // to find their lending pools easier. In the future, Fund Managers may
            // have multiple lending pools with the same supported tokens which may cause
            // duplication issues with the way this is set up. However, for this purposes
            // we'll just have it as the token's Resource Address for simplicity.
            let mut farmer_data: Farmer = self.get_resource_manager(&farmer_id);
            farmer_data.managed_debt_funds.insert(token_address, debt_fund);
            self.authorize_update(&farmer_badge, farmer_data);

            // * INSERTS LENDING POOL DATA TO THE GLOBAL INDEX * //
            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();

            self.farmer_dashboard_admin_vault.authorize(||
                farmers_market.insert_debt_fund(fund_name.clone(), debt_fund)
            );

            self.farmer_dashboard_admin_vault.authorize(||
                farmers_market.insert_tracking_tokens(tracking_tokens.resource_address(), fund_name)
            );

            (debt_fund, tracking_tokens, debt_farmer_badge)
        }


        pub fn new_index_fund(
            &mut self,
            farmer_badge: Proof,
            fund_name: String,
            fee_to_pool: Decimal,
            fund_ticker: String,
            starting_share_price: Decimal,
            tokens: HashMap<ResourceAddress, Decimal>,
        ) -> Bucket
        {
            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();

            assert_eq!(
                farmer_badge.resource_address(), self.farmer_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

            assert_ne!(
                farmers_market.assert_index_fund_name(fund_name.clone()), true, 
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

            let farmer_id: NonFungibleId = farmer_badge.non_fungible::<Farmer>().id();
            
            let mut farmer_data: Farmer = self.get_resource_manager(&farmer_id);

            farmer_data.managed_index_funds.insert(fund_id.clone(), index_fund);
            
            self.authorize_update(&farmer_badge, farmer_data);

            
            self.farmer_dashboard_admin_vault.authorize(||
                farmers_market.insert_index_fund_name(fund_name.clone(), fund_ticker.clone())
            );

            self.farmer_dashboard_admin_vault.authorize(||
                farmers_market.insert_index_fund(fund_id, index_fund)
            );

            fund_admin
        }

        /// This method allows Fund Managers to view all the Index Funds they manage. 
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof passed is the Fund Manager that belongs to this protocol.
        /// 
        /// # Arguments:
        /// 
        /// * `farmer_badge` (Proof) - The Proof of the Fund Manager badge.
        /// 
        /// # Returns:
        /// 
        /// * `HashMap<(String, String), ComponentAddress>` - The HashMap of the Fund ID (Fund Name, Fund Ticker) and
        /// the associated ComponentAddress of the Index Fund.
        pub fn view_managed_index_funds(
            &self,
            farmer_badge: Proof,
        ) -> HashMap<(String, String), ComponentAddress>
        {
            assert_eq!(farmer_badge.resource_address(), self.farmer_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

            let farmer_nft_data: Farmer = farmer_badge.non_fungible().data();
            let managed_index_funds: HashMap<(String, String), ComponentAddress> = farmer_nft_data.managed_index_funds.clone();

            managed_index_funds
        }

        pub fn view_managed_debt_funds(
            &self,
            farmer_badge: Proof,
        ) -> HashMap<ResourceAddress, ComponentAddress>
        {
            assert_eq!(farmer_badge.resource_address(), self.farmer_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

            let farmer_nft_data: Farmer = farmer_badge.non_fungible().data();
            let managed_index_funds: HashMap<ResourceAddress, ComponentAddress> = farmer_nft_data.managed_debt_funds.clone();

            managed_index_funds
        }

        /// This method is used to retrieve the NFT data of a selected NFT. 
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `nft_id` (&NonFungibleId) - The NonFungibleId of the NFT data to retrieve.
        /// 
        /// # Returns: 
        /// 
        /// * `Farmer` - The NFT data of the chosen NFT.
        fn get_resource_manager(
            &self,
            nft_id: &NonFungibleId,
        ) -> Farmer
        {
            let resource_manager = borrow_resource_manager!(self.farmer_address);
            let farmer_data: Farmer = resource_manager.get_non_fungible_data(&nft_id);

            farmer_data 
        }

        /// This method is used to authorize update/mutate of the NFT data.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `farmer_badge` (&Proof) - The Proof of the Fund Manager Badge.
        /// * `farmer_data` (Farmer) - The NFT data of the Fund Manager Badge.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        fn authorize_update(
            &self,
            farmer_badge: &Proof,
            farmer_data: Farmer
        )
        {
            let nft_id: NonFungibleId = farmer_badge.non_fungible::<Farmer>().id();
            let resource_manager = borrow_resource_manager!(self.farmer_address);
            self.farmer_dashboard_admin_vault.authorize(|| 
                resource_manager.update_non_fungible_data(&nft_id, farmer_data)
            );
        }
        
        /// This method is used by the DebtFund component to enter a record of the Funding Lockers created. 
        /// 
        /// This method does not perform any checks.
        /// 
        /// The checks are done in the main blueprint.
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
            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();

            self.farmer_dashboard_admin_vault.authorize(||
                farmers_market.insert_funding_locker(loan_id, funding_locker_address)
            );
        }
    }
}