use scrypto::prelude::*;
use crate::structs::*;
use crate::fundinglocker::*;
use crate::maple_finance_global::*;

// Pool Management

blueprint! {
    struct DebtFund {
        fund_manager_address: ResourceAddress,
        fund_manager_id: NonFungibleId,
        vault: Vault,
        debt_fund_admin_address: ResourceAddress,
        /// When liquidity providers provide liquidity to the liquidity pool, they are given a number of tokens that is
        /// equivalent to the percentage ownership that they have in the liquidity pool. The tracking token is the token
        /// that the liquidity providers are given when they provide liquidity to the pool and this is the resource 
        /// address of the token.
        tracking_token_address: ResourceAddress,
        /// The tracking tokens are mutable supply tokens that may be minted and burned when liquidity is supplied or 
        /// removed from the liquidity pool. This badge is the badge that has the authority to mint and burn the tokens
        /// when need be.
        tracking_token_admin_badge: Vault,
        funding_locker_address: Option<ComponentAddress>,
        access_badge_vault: Option<Vault>,
        loan_request_nft_address: ResourceAddress,
        loan_nft_address: ResourceAddress,
        loan_nft_admin_vault: Vault,
        maple_finance_global_address: ComponentAddress,
        funding_lockers: HashMap<ResourceAddress, ComponentAddress>,
        funding_locker_admin_vault: Option<Vault>,
    }

    impl DebtFund {

        pub fn new(
            maple_finance_global_address: ComponentAddress,
            fund_manager_name: String,
            fund_manager_address: ResourceAddress,
            fund_manager_id: NonFungibleId,
            loan_request_nft_address: ResourceAddress,
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
                .metadata("symbol", "FO")
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

            let debt_fund = Self {
                maple_finance_global_address: maple_finance_global_address,
                fund_manager_address: fund_manager_address,
                fund_manager_id: fund_manager_id,
                debt_fund_admin_address: debt_fund_badge.resource_address(),
                vault: Vault::with_bucket(initial_funds),
                tracking_token_address: tracking_tokens.resource_address(),
                tracking_token_admin_badge: Vault::with_bucket(tracking_token_admin_badge),
                funding_locker_address: None,
                access_badge_vault: None,
                loan_request_nft_address: loan_request_nft_address,
                loan_nft_address: loan_nft_address,
                loan_nft_admin_vault: Vault::with_bucket(loan_nft_admin),
                funding_lockers: HashMap::new(),
                funding_locker_admin_vault: None,
            }
            .instantiate()
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

            tracking_tokens
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

            let loan_nft_admin = self.loan_nft_admin_vault.authorize(|| 
                borrow_resource_manager!(self.loan_nft_admin_vault.resource_address()).mint(1)
            );

            let (funding_locker, funding_locker_admin): (ComponentAddress, Bucket) = FundingLocker::new(
                loan_request_nft_id.clone(), 
                self.loan_request_nft_address, 
                loan_nft, 
                loan_nft_admin
            );

            // * INSERTS FUNDING LOCKER DATA INTO COMPONENT STATE * //
            self.funding_lockers.insert(funding_locker_admin.resource_address(), funding_locker);

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
            
            maple_finance.authorize_loan_request_update(loan_request_nft_id, loan_request_nft_data);

            self.funding_locker_admin_vault = Some(Vault::with_bucket(funding_locker_admin));
        }

        pub fn fund_loan(
            &mut self,
            debt_fund_badge: Proof,
            amount: Decimal,
            funding_locker_address: ComponentAddress,
            debt_fund_address: ComponentAddress,
        )
        {
            self.assert_admin(&debt_fund_badge);

            let bucket: Bucket = self.withdraw(amount);

            self.funding_locker_address = Some(funding_locker_address);
            let funding_locker: FundingLocker = funding_locker_address.into();
            funding_locker.fund_loan(debt_fund_address, bucket);
        }

        /// Think about Access Rule
        /// Perhaps fund_loan method can provide access badge to fund locker
        pub fn deposit_access_badge(
            &mut self,
            access_badge: Bucket
        )
        {
            self.access_badge_vault = Some(Vault::with_bucket(access_badge));
        }

        pub fn claim_fees(
            &mut self,
            tracking_tokens: Proof,
        ) -> Option<Bucket>
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

            let optional_funding_locker: Option<ComponentAddress> = self.funding_locker_address;
            match optional_funding_locker {
                Some(funding_locker) => {
                    let funding_locker_admin_proof: Proof = self.funding_locker_admin_vault.as_mut().unwrap().create_proof();
                    let funding_locker: FundingLocker = funding_locker.into();
                    let fee_bucket: Bucket = funding_locker.claim_fees(funding_locker_admin_proof, percentage);
                    return Some(fee_bucket)
                }
                None => {
                    info!(
                        "[Debt Fund - Claim Fees]: This Debt Fund has not funded any loan opportunities yet."
                    );

                    return None
                }
            }
        }

        fn assert_admin(
            &self,
            debt_fund_badge: &Proof,
        )
        {
            assert_eq!(
                debt_fund_badge.resource_address(), self.debt_fund_admin_address,
                "[Debt Fund - Assert Admin]: Unauthorized Access."
            );
        }
    }
}