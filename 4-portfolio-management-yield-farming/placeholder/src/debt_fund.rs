use scrypto::prelude::*;
use crate::structs::*;
use crate::fundinglocker::*;
use crate::maple_finance_global::*;
use crate::fund_manager_dashboard::*;

// Pool Management

blueprint! {
    struct DebtFund {
        fund_manager_address: ResourceAddress,
        fund_manager_id: NonFungibleId,
        vault: Vault,
        debt_fund_admin_vault: Vault,
        /// The ResourceAddress of the Debt Fund Badge that represents Admin Authority over the fund. The reason we have
        /// a separate badge for Fund Manager Badge and Debt Fund Badge is because Fund Managers (while not currently supported)
        /// may wish to transfer ownership of the fund. 
        debt_fund_badge_address: ResourceAddress,
        /// When liquidity providers provide liquidity to the liquidity pool, they are given a number of tokens that is
        /// equivalent to the percentage ownership that they have in the liquidity pool. The tracking token is the token
        /// that the liquidity providers are given when they provide liquidity to the pool and this is the resource 
        /// address of the token.
        tracking_token_address: ResourceAddress,
        /// The tracking tokens are mutable supply tokens that may be minted and burned when liquidity is supplied or 
        /// removed from the liquidity pool. This badge is the badge that has the authority to mint and burn the tokens
        /// when need be.
        tracking_token_admin_badge: Vault,
        funding_locker_badge_vault: HashMap<NonFungibleId, Vault>,
        loan_request_nft_address: ResourceAddress,
        loan_nft_address: ResourceAddress,
        loan_nft_admin_vault: Vault,
        maple_finance_global_address: ComponentAddress,
        optional_fund_manager_dashboard_address: Option<ComponentAddress>,
        price_oracle_address: ComponentAddress,
        funding_lockers: HashMap<NonFungibleId, ComponentAddress>,
        access_badge_vault: Option<Vault>,
        fee_vault: HashMap<ResourceAddress, Vault>,
    }

    impl DebtFund {

        pub fn new(
            maple_finance_global_address: ComponentAddress,
            optional_fund_manager_dashboard_address: Option<ComponentAddress>,
            price_oracle_address: ComponentAddress,
            fund_manager_name: String,
            fund_manager_address: ResourceAddress,
            fund_manager_id: NonFungibleId,
            loan_request_nft_address: ResourceAddress,
            debt_fund_admin: Bucket,
            loan_nft_admin: Bucket,
            initial_funds: Bucket
        ) -> (ComponentAddress, Bucket, Bucket)
        {
            assert_ne!(
                borrow_resource_manager!(initial_funds.resource_address()).resource_type(), ResourceType::NonFungible,
                "[Debt Fund Creation]: Asset must be fungible."
            );

            assert!(    
                !initial_funds.is_empty(), 
                "[Debt Fund Creation]: Can't deposit an empty bucket."
            ); 

            let debt_fund_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("Admin Badge for {}'s {} Debt Fund", 
                    fund_manager_name, initial_funds.resource_address())
                )
                .metadata("symbol", "DF_B")
                .metadata("description", "Badge that represents admin authority of the fund.")
                .initial_supply(1);

            // Creating the admin badge of the liquidity pool which will be given the authority to mint and burn the
            // tracking tokens issued to the liquidity providers.
            let tracking_token_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Tracking Token Admin Badge")
                .metadata("symbol", "TTAB")
                .metadata("description", "This is an admin badge that has the authority to mint and burn tracking tokens")
                .initial_supply(1);

            // Creating the tracking tokens and minting the amount owed to the initial liquidity provider
            let tracking_tokens: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "LP Tracking Token")
                .metadata("symbol", "TT")
                .metadata("description", "A tracking token used to track the percentage ownership of liquidity providers over the liquidity pool")
                .mintable(rule!(require(tracking_token_admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(tracking_token_admin_badge.resource_address())), LOCKED)
                .initial_supply(100);

            let loan_nft_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Loan NFT")
                .metadata("symbol", "LNFT")
                .metadata("description", "Loan NFT")
                .mintable(rule!(require(loan_nft_admin.resource_address())), LOCKED)
                .burnable(rule!(require(loan_nft_admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(loan_nft_admin.resource_address())), LOCKED)
                .no_initial_supply();

            let access_rules: AccessRules = AccessRules::new()
                .method("deposit_access_badge", rule!(require(loan_nft_admin.resource_address())))
                .default(rule!(allow_all)
            );

            let debt_fund = Self {
                maple_finance_global_address: maple_finance_global_address,
                optional_fund_manager_dashboard_address: optional_fund_manager_dashboard_address,
                price_oracle_address: price_oracle_address,
                fund_manager_address: fund_manager_address,
                fund_manager_id: fund_manager_id,
                debt_fund_badge_address: debt_fund_badge.resource_address(),
                vault: Vault::with_bucket(initial_funds),
                debt_fund_admin_vault: Vault::with_bucket(debt_fund_admin),
                tracking_token_address: tracking_tokens.resource_address(),
                tracking_token_admin_badge: Vault::with_bucket(tracking_token_admin_badge),
                funding_locker_badge_vault: HashMap::new(),
                loan_request_nft_address: loan_request_nft_address,
                loan_nft_address: loan_nft_address,
                loan_nft_admin_vault: Vault::with_bucket(loan_nft_admin),
                funding_lockers: HashMap::new(),
                access_badge_vault: None,
                fee_vault: HashMap::new(),
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (debt_fund, tracking_tokens, debt_fund_badge)
        }

        pub fn supply_liquidity(
            &mut self,
            liquidity_amount: Bucket
        ) -> Bucket
        {
            // Checking if the token belong to this liquidity pool.
            assert_eq!(
                liquidity_amount.resource_address(), self.vault.resource_address(),
                "[Debt Fund - Add Liquidity]: The bucket contains the wrong tokens."
            );

            // Checking that the bucket passed is not empty
            assert!(
                !liquidity_amount.is_empty(), 
                "[Debt Fund - Add Liquidity]: Can not add liquidity from an empty bucket"
            );

            let amount = liquidity_amount.amount();

            let m: Decimal = self.vault.amount();

            // Computing the amount of tracking tokens that the liquidity provider is owed and minting them. In the case
            // that the liquidity pool has been completely emptied out (tracking_tokens_manager.total_supply() == 0)  
            // then the first person to supply liquidity back into the pool again would be given 100 tracking tokens.
            let tracking_tokens_manager: &ResourceManager = borrow_resource_manager!(self.tracking_token_address);
            let tracking_amount: Decimal = if tracking_tokens_manager.total_supply() == Decimal::zero() { 
                dec!("100.00") 
            } else {
                amount * tracking_tokens_manager.total_supply() / m
            };
            let tracking_tokens: Bucket = self.tracking_token_admin_badge.authorize(|| {
                tracking_tokens_manager.mint(tracking_amount)
            });
            info!("[Add Liquidity]: Owed amount of tracking tokens: {}", tracking_amount);

            self.vault.put(liquidity_amount);

            tracking_tokens
        }

        pub fn transfer_liquidity(
            &mut self,
            debt_fund_badge: Proof,
            loan_nft_id: NonFungibleId,
        )
        {
            self.assert_admin(&debt_fund_badge);

            let optional_funding_locker_address: Option<&ComponentAddress> = self.funding_lockers.get(&loan_nft_id);
            match optional_funding_locker_address {
                Some(funding_locker_address) => {
                    let access_badge_proof: Proof = self.access_badge_vault.as_mut().unwrap().create_proof();
                    let funding_locker_address: ComponentAddress = *funding_locker_address;
                    let funding_locker: FundingLocker = funding_locker_address.into();

                    let liquidity_bucket: Bucket = self.loan_nft_admin_vault.authorize(|| 
                        funding_locker.transfer_liquidity()
                    );

                    // Checking if the token belong to this liquidity pool.
                    assert_eq!(
                        liquidity_bucket.resource_address(), self.vault.resource_address(),
                        "[Debt Fund - Transfer Liquidity]: The bucket contains the wrong tokens."
                    );

                    // Checking that the bucket passed is not empty
                    assert!(
                        !liquidity_bucket.is_empty(), 
                        "[Debt Fund - Transfer Liquidity]: Can not add liquidity from an empty bucket"
                    );

                    self.vault.put(liquidity_bucket);
                }
                None => {
                    info!(
                        "[Debt Fund - Transfer Liquidity]: This Debt Fund has not funded any loan opportunities yet."
                    );
                }
            }
        }

        fn withdraw(
            &mut self,
            amount: Decimal
        ) -> Bucket 
        {            
            // Getting the vault of that resource and checking if there is enough liquidity to perform the withdraw.
            assert!(
                self.vault.amount() >= amount,
                "[Debt Fund - Withdraw]: Not enough liquidity available for the withdraw."
            );

            return self.vault.take(amount);
        }

        /// Removes the percentage of the liquidity owed to this liquidity provider.
        /// 
        /// This method is used to calculate the amount of tokens owed to the liquidity provider and take them out of
        /// the liquidity pool and return them to the liquidity provider. If the liquidity provider wishes to only take
        /// out a portion of their liquidity instead of their total liquidity they can provide a `tracking_tokens` 
        /// bucket that does not contain all of their tracking tokens (example: if they want to withdraw 50% of their
        /// liquidity, they can put 50% of their tracking tokens into the `tracking_tokens` bucket.). When the liquidity
        /// provider is given the tokens that they are owed, the tracking tokens are burned.
        /// 
        /// This method performs a number of checks before liquidity removed from the pool:
        /// 
        /// * **Check 1:** Checks to ensure that the tracking tokens passed do indeed belong to this liquidity pool.
        /// 
        /// # Arguments:
        /// 
        /// * `tracking_tokens` (Bucket) - A bucket of the tracking tokens that the liquidity provider wishes to 
        /// exchange for their share of the liquidity.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A Bucket of the share of the liquidity provider of the first token.
        /// * `Bucket` - A Bucket of the share of the liquidity provider of the second token.
        pub fn remove_liquidity(
            &mut self,
            tracking_tokens: Bucket
        ) -> Bucket
        {
            // Checking the resource address of the tracking tokens passed to ensure that they do indeed belong to this
            // liquidity pool.
            assert_eq!(
                tracking_tokens.resource_address(), self.tracking_token_address,
                "[Debt Fund - Remove Liquidity]: The tracking tokens given do not belong to this liquidity pool."
            );

            // Calculating the percentage ownership that the tracking tokens amount corresponds to
            let tracking_tokens_manager: &ResourceManager = borrow_resource_manager!(self.tracking_token_address);
            let percentage: Decimal = tracking_tokens.amount() / tracking_tokens_manager.total_supply();

            // Burning the tracking tokens
            self.tracking_token_admin_badge.authorize(|| {
                tracking_tokens.burn();
            });

            let bucket: Bucket = self.withdraw(self.vault.amount() * percentage);

            bucket
        }

        pub fn loan_request_list(
            &mut self,
        ) -> HashMap<NonFungibleId, BTreeSet<NonFungibleId>>
        {
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            maple_finance.loan_request_list()
        }

        pub fn instantiate_funding_locker(
            &mut self,
            fund_manager_badge: Proof,
            loan_request_nft_id: NonFungibleId,
            // Remove this?
            borrower_id: NonFungibleId,
            loan_amount: Decimal,
            asset_address: ResourceAddress,
            collateral_address: ResourceAddress,
            collateral_percent: Decimal,
            annualized_interest_rate: Decimal,
            draw_limit: Decimal,
            draw_minimum: Decimal,
            term_length: TermLength,
            origination_fee: Decimal,
        ) 
        {
            assert_eq!(fund_manager_badge.resource_address(), self.fund_manager_address,
                "[Fund Manager Dashboard]: This badge does not belong to this protocol."
            );
            
            let fund_manager_id: NonFungibleId = fund_manager_badge.non_fungible::<FundManager>().id();

            let origination_fee_charged = loan_amount * origination_fee;
            let remaining_balance = loan_amount + origination_fee;

            let payments_remaining: u64 = match term_length {
                TermLength::OneMonth => 1,
                TermLength::ThreeMonth => 3,
                TermLength::SixMonth => 6,
            };

            let loan_nft = self.loan_nft_admin_vault.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_nft_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    Loan {
                        borrower_id: borrower_id,
                        lender_id: fund_manager_id.clone(),
                        principal_loan_amount: loan_amount,
                        asset: asset_address,
                        collateral: collateral_address,
                        collateral_percent: collateral_percent,
                        annualized_interest_rate: annualized_interest_rate,
                        term_length: term_length,
                        payments_remaining: payments_remaining,
                        origination_fee: origination_fee,
                        origination_fee_charged: origination_fee_charged,
                        accrued_interest_expense: Decimal::zero(),
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

            let loan_nft_admin = self.debt_fund_admin_vault.authorize(|| 
                borrow_resource_manager!(self.loan_nft_admin_vault.resource_address()).mint(1)
            );

            let (funding_locker, funding_locker_badge): (ComponentAddress, Bucket) = FundingLocker::new(
                self.price_oracle_address,
                loan_request_nft_id.clone(), 
                self.loan_request_nft_address, 
                loan_nft, 
                loan_nft_admin
            );

            // * INSERTS FUNDING LOCKER DATA INTO COMPONENT STATE * //
            self.funding_lockers.insert(loan_nft_id.clone(), funding_locker);

            // Puts the Funding Locker Badge into component vault (Fund Manager does not receive it)
            self.funding_locker_badge_vault.insert(loan_nft_id.clone(), Vault::with_bucket(funding_locker_badge));

            // * INSERTS FUNDING LOCKER DATA TO THE GLOBAL INDEX * //
            // The Loan NFT Id is used as the HashMap key to allow easier quering from outsider perspective.
            let fund_manager_dashboard: FundManagerDashboard = self.optional_fund_manager_dashboard_address.unwrap().into();

            self.debt_fund_admin_vault.authorize(||
                fund_manager_dashboard.insert_funding_locker(loan_nft_id.clone(), funding_locker)
            );
        
            // * MODIFIES LOAN REQUEST NFT * // - IMPLEMENTED ACCESS CONTROL?
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            let resource_manager = borrow_resource_manager!(self.loan_request_nft_address);
            let mut loan_request_nft_data: LoanRequest = resource_manager.get_non_fungible_data(&loan_request_nft_id);

            loan_request_nft_data.status = RequestStatus::Modified;
            loan_request_nft_data.loan_nft_id = Some(loan_nft_id); 
            loan_request_nft_data.funding_locker_address = Some(funding_locker);
            
            maple_finance.authorize_loan_request_update(loan_request_nft_id, loan_request_nft_data);
        }

        pub fn fund_loan(
            &mut self,
            debt_fund_badge: Proof,
            amount: Decimal,
            loan_nft_id: NonFungibleId,
        )
        {
            self.assert_admin(&debt_fund_badge);

            let fund_bucket: Bucket = self.withdraw(amount);

            let funding_locker_address: ComponentAddress = *self.funding_lockers.get_mut(&loan_nft_id).unwrap();
            let funding_locker: FundingLocker = funding_locker_address.into();

            // Maybe have an error handling for the option.
            self.funding_locker_badge_vault.as_mut().unwrap().authorize(||
                    funding_locker.fund_loan(
                    fund_bucket
                )
            );
        }

        pub fn approve_draw_request(
            &mut self,
            debt_fund_badge: Proof,
            loan_nft_id: NonFungibleId,
        )
        {
            self.assert_admin(&debt_fund_badge);

            let funding_locker_address: ComponentAddress = *self.funding_lockers.get_mut(&loan_nft_id).unwrap();
            let funding_locker: FundingLocker = funding_locker_address.into();

            self.funding_locker_badge_vault.as_mut().unwrap().authorize(|| 
                funding_locker.approve_draw_request()
            );
        }

        pub fn reject_draw_request(
            &mut self,
            debt_fund_badge: Proof,
            loan_nft_id: NonFungibleId,
        )
        {
            self.assert_admin(&debt_fund_badge);

            let funding_locker_address: ComponentAddress = *self.funding_lockers.get_mut(&loan_nft_id).unwrap();
            let funding_locker: FundingLocker = funding_locker_address.into();

            self.funding_locker_badge_vault.as_mut().unwrap().authorize(||
                funding_locker.reject_draw_request()
            );
        }

        pub fn update_loan(
            &mut self,
            debt_fund_badge: Proof,
            loan_nft_id: NonFungibleId,
        )
        {
            self.assert_admin(&debt_fund_badge);

            let optional_funding_locker_address: Option<&ComponentAddress> = self.funding_lockers.get(&loan_nft_id);


            match optional_funding_locker_address {
                Some(funding_locker_address) => {
                    let funding_locker: FundingLocker = *funding_locker_address.into();

                    funding_locker_badge_vault.authorize(||
                        funding_locker.update_loan()
                    );
                }
                None => {
                    info!(
                        "[Debt Fund - Update Loan]: Funding Locker"
                    )
                }
            }

        }

        pub fn deposit_access_badge(
            &mut self,
            access_badge: Bucket
        )
        {
            self.access_badge_vault = Some(Vault::with_bucket(access_badge));
        }

        pub fn transfer_fees(
            &mut self,
            debt_fund_badge: Proof,
            loan_nft_id: NonFungibleId,
        )
        {
            self.assert_admin(&debt_fund_badge);

            let funding_locker_address: ComponentAddress = *self.funding_lockers.get_mut(&loan_nft_id).unwrap();
            let funding_locker: FundingLocker = funding_locker_address.into();

            let fee_bucket: Bucket = self.funding_locker_badge_vault.as_mut().unwrap().authorize(|| 
                funding_locker.transfer_fees()
            );

            if self.fee_vault.contains_key(&fee_bucket.resource_address()) {
                self.fee_vault.get_mut(&fee_bucket.resource_address()).unwrap().put(fee_bucket);
            } else {
                self.fee_vault.insert(fee_bucket.resource_address(), Vault::with_bucket(fee_bucket));
            }
            
        }

        pub fn claim_fees(
            &mut self,
            tracking_tokens: Proof,
        ) -> Vec<Bucket>
        {
            // Checking the resource address of the tracking tokens passed to ensure that they do indeed belong to this
            // liquidity pool.
            assert_eq!(
                tracking_tokens.resource_address(), self.tracking_token_address,
                "[Debt Fund - Claim Fees]: The tracking tokens given do not belong to this liquidity pool."
            );

            // Calculating the percentage ownership that the tracking tokens amount corresponds to
            let tracking_tokens_manager: &ResourceManager = borrow_resource_manager!(self.tracking_token_address);
            let percentage: Decimal = tracking_tokens.amount() / tracking_tokens_manager.total_supply();

            let mut vec_bucket: Vec<Bucket> = Vec::new();

            let fee_vaults = self.fee_vault.iter_mut();
            for (_fee_resource, fee_vault) in fee_vaults {
                // Ideally we want to have percentage of value v.s. quantity.
                let amount: Decimal = fee_vault.amount() * percentage;

                let fee_bucket: Bucket = fee_vault.take(amount);

                vec_bucket.push(fee_bucket);
            }

            vec_bucket
        }

        fn assert_admin(
            &self,
            debt_fund_badge: &Proof,
        )
        {
            assert_eq!(
                debt_fund_badge.resource_address(), self.debt_fund_badge_address,
                "[Debt Fund - Assert Admin]: Unauthorized Access."
            );
        }
    }
}