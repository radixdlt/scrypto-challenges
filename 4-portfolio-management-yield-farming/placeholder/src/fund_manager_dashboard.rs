use scrypto::prelude::*;
use crate::index_fund::*;
use crate::fundinglocker::*;
use crate::debt_fund::*;
use crate::maple_finance_global::*;
use crate::structs::*;
use crate::utils::*;

// Allows approved Fund Manager to manage pools.

blueprint! {
    struct FundManagerDashboard {
        fund_manager_address: ResourceAddress,
        fund_manager_id: NonFungibleId,
        fund_manager_admin_vault: Vault,
        loan_request_nft_admin: Vault,
        loan_request_nft_address: ResourceAddress,
        loan_nft_admin: Vault,
        loan_nft_address: ResourceAddress,
        fund_master_admin: Vault,
        price_oracle_address: ComponentAddress,
        maple_finance_global_address: ComponentAddress,
        funding_locker_admin_vault: Option<Vault>,
    }

    impl FundManagerDashboard {

        pub fn new(
            fund_manager_admin: Bucket,
            maple_finance_global_address: ComponentAddress,
            fund_manager_address: ResourceAddress,
            fund_manager_id: NonFungibleId,
            loan_request_nft_admin: Bucket,
            loan_request_nft_address: ResourceAddress,
            price_oracle_address: ComponentAddress,
        ) -> ComponentAddress
        {
            let fund_master_admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Fund Manager Master Admin Badge")
                .metadata("symbol", "PDMAB")
                .metadata("description", "Allows Fund Managers to mint/burn loan NFTs.")
                .initial_supply(1);

            let loan_nft_admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Loan NFT Admin Badge")
                .metadata("symbol", "LNFTAB")
                .metadata("description", "Allows Fund Managers to mint/burn loan NFTs.")
                .mintable(rule!(require(fund_master_admin.resource_address())), LOCKED)
                .burnable(rule!(require(fund_master_admin.resource_address())), LOCKED)
                .initial_supply(1);

            let loan_nft_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Loan NFT")
                .metadata("symbol", "LNFT")
                .metadata("description", "Loan NFT")
                .mintable(rule!(require(loan_nft_admin.resource_address())), LOCKED)
                .burnable(rule!(require(loan_nft_admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(loan_nft_admin.resource_address())), LOCKED)
                .no_initial_supply();

            return Self {
                fund_manager_admin_vault: Vault::with_bucket(fund_manager_admin),
                fund_manager_address: fund_manager_address,
                fund_manager_id: fund_manager_id,
                loan_request_nft_admin: Vault::with_bucket(loan_request_nft_admin),
                loan_request_nft_address: loan_request_nft_address,
                loan_nft_admin: Vault::with_bucket(loan_nft_admin),
                loan_nft_address: loan_nft_address,
                fund_master_admin: Vault::with_bucket(fund_master_admin),
                price_oracle_address: price_oracle_address,
                maple_finance_global_address: maple_finance_global_address,
                funding_locker_admin_vault: None, 
            }
            .instantiate()
            .globalize();
        }

        pub fn new_debt_fund(
            &mut self,
            fund_admin_badge: Proof,
            initial_funds: Bucket
        ) -> (ComponentAddress, Bucket, Bucket)
        {
            //Logic to check if the there's a duplicate lending pool
            assert_eq!(fund_admin_badge.resource_address(), self.fund_manager_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

            let fund_manager_id: NonFungibleId = fund_admin_badge.non_fungible::<FundManager>().id();
            let token_address: ResourceAddress = initial_funds.resource_address();

            let fund_manager_data: FundManager = self.get_resource_manager(&fund_manager_id);
            let fund_manager_name: String = fund_manager_data.name;
            // Instantiates the lending pool and collateral pool.
            let (debt_fund, tracking_tokens, debt_fund_manager_badge): (ComponentAddress, Bucket, Bucket) = DebtFund::new(
                fund_manager_name,
                fund_admin_badge.resource_address(), 
                fund_manager_id.clone(), 
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
            self.authorize_update(fund_manager_data);

            // * INSERTS LENDING POOL DATA TO THE GLOBAL INDEX * //
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            maple_finance.insert_debt_fund(fund_manager_id, debt_fund);

            (debt_fund, tracking_tokens, debt_fund_manager_badge)
        }

        // pub fn retrieve_loan_requests(
        //     &self) -> HashMap<ResourceAddress, BTreeSet<NonFungibleId>>
        // {
        //     let maple_finance_global: MapleFinance = self.maple_finance_global_address.unwrap().into();
        //     let loan_requests = maple_finance_global.broadcast_loan_requests();
        //     loan_requests
        // }

        pub fn instantiate_funding_locker(
            &mut self,
            fund_manager_badge: Proof,
            loan_request_nft_id: NonFungibleId,
            borrower_id: NonFungibleId,
            loan_amount: Decimal,
            asset_address: ResourceAddress,
            collateral_address: ResourceAddress,
            collateral_percent: Decimal,
            annualized_interest_rate: Decimal,
            draw_limit: Decimal,
            draw_minimum: Decimal,
            term_length: u64,
            payment_frequency: PaymentFrequency,
            origination_fee: Decimal,
        ) 
        {
            assert_eq!(fund_manager_badge.resource_address(), self.fund_manager_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );
            
            let fund_manager_id: NonFungibleId = fund_manager_badge.non_fungible::<FundManager>().id();

            let origination_fee_charged = loan_amount * origination_fee;
            let annualized_interest_expense = loan_amount * annualized_interest_rate;
            let remaining_balance = loan_amount + origination_fee;

            let loan_nft = self.loan_nft_admin.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_nft_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    Loan {
                        borrower_id: borrower_id,
                        lender_id: self.fund_manager_id.clone(),
                        lender_address: self.fund_manager_address,
                        principal_loan_amount: loan_amount,
                        asset: asset_address,
                        collateral: collateral_address,
                        collateral_percent: collateral_percent,
                        annualized_interest_rate: annualized_interest_rate,
                        term_length: term_length,
                        payment_frequency: payment_frequency,
                        origination_fee: origination_fee,
                        origination_fee_charged: origination_fee_charged,
                        annualized_interest_expense: annualized_interest_expense,
                        draw_limit: draw_limit,
                        draw_minimum: draw_minimum,
                        remaining_balance: remaining_balance,
                        last_draw: 0,
                        collateral_amount: Decimal::zero(),
                        collateral_amount_usd: Decimal::zero(),
                        health_factor: Decimal::zero(),
                        loan_status: Status::Current,
                    },
                )
            });

            let loan_nft_id = loan_nft.non_fungible::<Loan>().id();

            let loan_nft_admin = self.fund_master_admin.authorize(|| 
                borrow_resource_manager!(self.loan_nft_admin.resource_address()).mint(1)
            );

            let (funding_locker, funding_locker_admin): (ComponentAddress, Bucket) = FundingLocker::new(
                loan_request_nft_id.clone(), 
                self.loan_request_nft_address, 
                loan_nft, 
                loan_nft_admin
            );

            // * INSERTS FUNDING LOCKER DATA INTO FUND MANAGER NFT * //
            // The Fund Admin badge ResourceAddress (for the Funding Locker) is used as the HashMap key 
            // to allow easier quering from Fund Manager's perspective.
            let mut fund_manager_data: FundManager = self.get_resource_manager(&fund_manager_id);
            fund_manager_data.managed_funding_lockers.insert(
                funding_locker_admin.non_fungible::<FundingLockerAdmin>().id(), funding_locker
            );
            
            fund_manager_data.managed_funding_locker_admin.insert(
                loan_nft_id.clone(), funding_locker_admin.non_fungible::<FundingLockerAdmin>().id()
            );
            self.authorize_update(fund_manager_data);

            // * INSERTS FUNDING LOCKER DATA TO THE GLOBAL INDEX * //
            // The Loan NFT Id is used as the HashMap key to allow easier quering from outsider perspective.
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            maple_finance.insert_funding_lockers(loan_nft_id.clone(), funding_locker);

            // * MODIFIES LOAN REQUEST NFT * //
            let resource_manager = borrow_resource_manager!(self.loan_request_nft_address);
            let mut loan_request_nft_data: LoanRequest = resource_manager.get_non_fungible_data(&loan_request_nft_id);

            loan_request_nft_data.status = RequestStatus::Modified;
            loan_request_nft_data.loan_nft_id = Some(loan_nft_id); 
            loan_request_nft_data.funding_locker_address = Some(funding_locker);
            
            self.loan_request_nft_admin.authorize(||
                resource_manager.update_non_fungible_data(&loan_request_nft_id, loan_request_nft_data)
            );

            self.funding_locker_admin_vault = Some(Vault::with_bucket(funding_locker_admin));
        }

        pub fn fund_loan(
            &mut self,
            fund_manager_badge: Proof,
            debt_fund_manager_badge: Proof,
            loan_id: NonFungibleId,
            token_address: ResourceAddress,
            funding_amount: Decimal,
        )
        {
            let fund_manager_id: NonFungibleId = fund_manager_badge.non_fungible::<FundManager>().id();
            let fund_manager_data: FundManager = self.get_resource_manager(&fund_manager_id);

            assert_eq!(
                fund_manager_badge.resource_address(), self.fund_manager_address,
                "[Fund Manager Dashboard - Fund Loan]: This badge does not belong to this protocol."
            );

            assert_eq!(
                fund_manager_data.managed_debt_funds.contains_key(&debt_fund_manager_badge.resource_address()), true,
                "[Fund Manager Dashboard - Fund Loan]: You do not manage this debt fund."
            );

            let optional_funding_locker_admin_vault: Option<&mut Vault> = self.funding_locker_admin_vault.as_mut();

            let funding_locker_admin_id: &NonFungibleId = fund_manager_data.managed_funding_locker_admin.get(&loan_id).unwrap();

            match optional_funding_locker_admin_vault {
                Some(vault) => {
                    let funding_locker_admin: Bucket = vault.take_non_fungible(funding_locker_admin_id);
                }
                None => {

                    info!(
                        "[Fund Manager Dashboard - Fund Loan]: You do not manage any funding lockers."
                    );
                }
            }


            let funding_locker_address: ComponentAddress = *fund_manager_data.managed_funding_lockers
            .get(&loan_id).unwrap();

            let optional_debt_fund: Option<&ComponentAddress> = fund_manager_data.managed_debt_funds.get(&token_address);
            match optional_debt_fund {
                Some (debt_fund) => { // If it matches it means that the debt fund exists.
                    let debt_fund_address: ComponentAddress = *debt_fund;
                    let debt_fund: DebtFund = debt_fund_address.into();
                    debt_fund.fund_loan(
                        token_address,
                        funding_amount,
                        funding_locker_address,
                        debt_fund_address,
                    );
                }
                None => { 

                    info!("[Fund Manager Dashboard]: Pool for {:?} doesn't exist.", token_address);

                }
            }
        } 

        pub fn new_index_fund(
            &mut self,
            fund_admin_badge: Proof,
            fund_name: String,
            fee_to_pool: Decimal,
            fund_ticker: String,
            starting_share_price: Decimal,
            tokens: HashMap<ResourceAddress, Decimal>,
        ) -> Bucket
        {
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();

            let fund_manager_id: NonFungibleId = fund_admin_badge.non_fungible::<FundManager>().id();

            assert_eq!(
                fund_admin_badge.resource_address(), self.fund_manager_address,
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
            
            let mut fund_manager_data: FundManager = self.get_resource_manager(&fund_manager_id);

            fund_manager_data.managed_index_funds.insert(fund_id.clone(), index_fund);
            
            self.authorize_update(fund_manager_data);

            maple_finance.insert_index_fund_name(fund_name.clone(), fund_ticker.clone());
            maple_finance.insert_index_fund(fund_id, index_fund);

            fund_admin
        }

        pub fn view_managed_index_funds(
            &self,
            fund_admin_badge: Proof,
        )
        {
            assert_eq!(fund_admin_badge.resource_address(), self.fund_manager_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );

            assert_eq!(fund_admin_badge.non_fungible::<FundManager>().id(), self.fund_manager_id,
                "[Fund Manager Dashboard]: Incorrect Fund Manager."
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
            fund_manager_data: FundManager
        )
        {
            let resource_manager = borrow_resource_manager!(self.fund_manager_address);
            self.fund_manager_admin_vault.authorize(|| 
                resource_manager.update_non_fungible_data(&self.fund_manager_id, fund_manager_data)
            );
        }
    }
}