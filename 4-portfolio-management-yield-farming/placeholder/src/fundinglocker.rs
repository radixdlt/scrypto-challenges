use scrypto::prelude::*;
use crate::structs::*;
use crate::debt_fund::*;
use crate::price_oracle::*;

blueprint! {
    struct FundingLocker {
        /// The ResourceAddress of the Fund Manager that has authority over this Funding Locker. 
        funding_locker_admin_address: ResourceAddress,
        /// The Vault the contains the loan proceeds.
        loan_proceeds_vault: Vault,
        draw_limit: Decimal,
        draw_minimum: Decimal,
        /// The Vault the contains the loan repayments.
        loan_repay_vault: Vault,
        /// Grace period allowed for the borrower before penalty is incurred.
        grace_period: u64,
        /// The Vault that contains the Loan NFT. The Loan NFT will be withdrawn by the borrower
        /// after the borrower satisfies the collateralization requirement. 
        loan_nft_vault: Vault,
        loan_request_nft_id: NonFungibleId,
        loan_request_nft_address: ResourceAddress,
        /// The Vault the contains the Borrower's collateral.
        collateral_vault: Vault,
        /// The Vault that contains the Loan NFT Admin badge to allow the component to
        /// update loan information.
        loan_nft_admin: Vault,
        loan_nft_id: NonFungibleId,
        loan_nft_address: ResourceAddress,
        loan_proceed_status: Status,
        draw_request_vault: Vault,
        draw_vault: Vault,
        fee_vault: Vault,
        debt_fund_address: Option<ComponentAddress>,
        access_badge_address: ResourceAddress,
        price_oracle_address: ComponentAddress,
    } 

    impl FundingLocker {

        pub fn new(
            // The lender will receive a resource to manage this locker vs. inputting the lender nft and id.
            // This way the lender can transfer ownership.
            price_oracle_address: ComponentAddress,
            loan_request_nft_id: NonFungibleId,
            loan_request_nft_address: ResourceAddress,
            loan_nft: Bucket,
            loan_nft_admin: Bucket,
        ) -> (ComponentAddress, Bucket) 
        {
            let funding_locker_admin: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .metadata("symbol", "AB")
                .metadata("description", "Admin authorithing over the Funding Locker.")
                .initial_supply(1);

            let access_badge_address: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Access Badge")
                .metadata("symbol", "AB")
                .metadata("description", "Provides access to authorized method calls be the lending pool.")
                .no_initial_supply();

            // NFT description for Pool Delegates
            let draw_request_nft_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Draw Request NFT")
                .metadata("symbol", "DR_NFT")
                .metadata("description", "Draw requests from the Borrower")
                .burnable(rule!(require(funding_locker_admin.resource_address())), LOCKED)
                .no_initial_supply();
            
            let loan_nft_data = loan_nft.non_fungible::<Loan>().data();
            let draw_limit: Decimal = loan_nft_data.draw_limit;
            let draw_minimum: Decimal = loan_nft_data.draw_minimum;
            let loan_asset_address = loan_nft_data.asset;
            let loan_collateral_address = loan_nft_data.collateral;
            let loan_nft_id = loan_nft.non_fungible::<Loan>().id();
            let loan_nft_address = loan_nft.resource_address();

            let funding_locker: ComponentAddress = Self {
                funding_locker_admin_address: funding_locker_admin.resource_address(),
                loan_repay_vault: Vault::new(loan_asset_address),
                loan_proceeds_vault: Vault::new(loan_asset_address),
                draw_limit: draw_limit,
                draw_minimum: draw_minimum,
                grace_period: 10,
                loan_nft_vault: Vault::with_bucket(loan_nft),
                loan_request_nft_id: loan_request_nft_id,
                loan_request_nft_address: loan_request_nft_address,
                collateral_vault: Vault::new(loan_collateral_address),
                loan_nft_admin: Vault::with_bucket(loan_nft_admin),
                loan_nft_id: loan_nft_id,
                loan_nft_address: loan_nft_address,
                loan_proceed_status: Status::Unfunded,
                draw_request_vault: Vault::new(draw_request_nft_address),
                draw_vault: Vault::new(loan_asset_address),
                fee_vault: Vault::new(loan_asset_address),
                debt_fund_address: None,
                access_badge_address: access_badge_address,
                price_oracle_address: price_oracle_address,
            }

            .instantiate()
            .globalize();

            return (funding_locker, funding_locker_admin);
        }

        fn get_resource_manager(
            &self) -> Loan
        {
            let resource_manager = borrow_resource_manager!(self.loan_nft_address);
            let loan_nft_data: Loan = resource_manager.get_non_fungible_data(&self.loan_nft_id); 
            return loan_nft_data
        }

        fn authorize_update(
            &mut self,
            loan_nft_data: Loan
        )
        {
            let resource_manager = borrow_resource_manager!(self.loan_nft_address);
            self.loan_nft_admin.authorize(|| 
                resource_manager.update_non_fungible_data(&self.loan_nft_id, loan_nft_data)
            );
        }

        /// Allows collateral to be deposited to this component.
        /// 
        /// This method is used so that the respective Borrower associated with this loan can deposit the collateral required.
        /// A Proof of the Loan Request NFT is passed by the respective Borrower Dashboard that proves it owns the Loan Request NFT.
        /// 
        /// This method performs a few checks before collateral is deposited.
        /// 
        /// * **Check 1:** - Checks that the Proof of the Loan Request NFT is associated with the loan request NFT address
        /// that the Pool Delegate used when instantiating the Funding Locker.
        /// 
        /// * **Check 2:** - Checks that the collateral deposited is the correct collateral associated with the loan.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_request_nft_proof` (Proof) - The Proof of the Loan Request NFT.
        /// * `collateral` (Bucket) - The Bucket that contains the collateral.
        /// 
        /// # Returns:
        /// 
        /// * `Option<Bucket>` - The Bucket that contains the Loan NFT if the collateral ratio was met or none if the 
        /// collateral ratio was not met.
        pub fn deposit_collateral(
            &mut self,
            loan_request_nft_proof: Proof,
            collateral: Bucket
        ) ->  Option<Bucket>
        {
            assert_eq!(loan_request_nft_proof.resource_address(), self.loan_request_nft_address,
                "[Funding Locker]: Incorrect Proof."
            );

            assert_eq!(collateral.resource_address(), self.collateral_vault.resource_address(),
                "[Funding Lcoker]: Incorrect collateral deposited."
            );

            self.collateral_vault.put(collateral);

            let collateral_amount = self.collateral_vault.amount();

            let mut loan_nft_data = self.get_resource_manager();

            let principal_loan_amount = loan_nft_data.principal_loan_amount;
            let collateral_percent = loan_nft_data.collateral_percent;

            if (principal_loan_amount / collateral_amount) >= collateral_percent {
                loan_nft_data.loan_status = Status::ReadyToFund;

                // Authorize logic
                self.authorize_update(loan_nft_data);

                let return_loan_nft = Some(self.loan_nft_vault.take_non_fungible(&self.loan_nft_id));

                info!("[Funding Locker]: Collateralization requirement met!");
                info!("[Funding Locker]: You've received a Loan NFT. Use this Loan NFT to access the Funding Locker.");
                info!("[Funding Locker]: The resource address of your Loan NFT is: {:?}", self.loan_nft_vault.resource_address());

                return_loan_nft

            } else {

                let return_loan_nft = None;

                return_loan_nft
            }
        }

        pub fn fund_loan(
            &mut self,
            funding_locker_admin: Proof,
            debt_fund_address: ComponentAddress,
            amount: Bucket,
        ) -> Bucket
        {
            assert_eq!(
                funding_locker_admin.resource_address(), self.funding_locker_admin_address,
                "[Funding Locker - Funding Loan]: Incorrect Proof passed."
            );

            assert_eq!(
                amount.resource_address(), self.loan_proceeds_vault.resource_address(),
                "[Funding Locker - Funding Loan]: Must fund with the correct tokens."
            );

            // ** LOGIC TO CHECK IF THE LOAN HAS BEEN FULLY FUNDED ** //
            let loan_nft_data: Loan = self.get_resource_manager();
            let principal_loan_amount: Decimal = loan_nft_data.principal_loan_amount;

            let mut return_over_funded: Bucket = Bucket::new(amount.resource_address());

            self.loan_proceeds_vault.put(amount);

            // If loan is under funded -> Provide info on how much left needs to be funded.
            if self.loan_proceeds_vault.amount() < principal_loan_amount {
                info!(
                    "[Funding Locker - Loan Funding]: The full loan proceeds has not yet been met."
                );

                info!(
                    "[Funding Locker - Loan Funding]: Additional {:?} of {:?} need to be funded.",
                    (principal_loan_amount - self.loan_proceeds_vault.amount()), 
                    self.loan_proceeds_vault.amount() 
                );

            // If loan is over funded -> Return overfunded amount.
            } else if self.loan_proceeds_vault.amount() > principal_loan_amount {

                let over_funded_amount: Decimal = self.loan_proceeds_vault.amount() - principal_loan_amount;

                info!(
                    "[Funding Locker - Loan Funding]: You have overfunded by: {:?}",
                    over_funded_amount
                );

                return_over_funded.put(self.loan_proceeds_vault.take(over_funded_amount));

            // If loan is fully funded -> Loan can now be drawn.
            } else {
                info!(
                    "[Funding Locker - Loan Funding]: The Funding Locker vault is fully funded.
                    The Borrower may now be allowed to draw."
                );

                self.loan_proceed_status = Status::Funded;
            };

            self.debt_fund_address = Some(debt_fund_address);

            // Access Rule imposed.
            let access_badge: Bucket = borrow_resource_manager!(self.access_badge_address).mint(1);

            let debt_fund: DebtFund = debt_fund_address.into();

            debt_fund.deposit_access_badge(access_badge);

            return_over_funded
        }

        pub fn draw_request(
            &mut self,
            loan_nft_badge: Proof,
            amount: Decimal,
        ) 
        {
            assert_eq!(
                loan_nft_badge.resource_address(), self.loan_nft_vault.resource_address(),
                "[Funding Locker - Draw Request]: Incorrect Loan NFT badge provided."
            );

            assert_eq!(
                self.loan_proceed_status, Status::Funded,
                "[Funding Locker - Draw Loan]: Loan is not ready to be drawn yet."
            );

            assert!(
                amount <= self.draw_limit,
                "[Funding Locker - Draw Request]: Draw request exceeds the draw limit."
            );

            assert!(
                amount >= self.draw_minimum,
                "[Funding Locker - Draw Request]: Draw request must exceed the draw minimum."
            );

            let resource_manager: &ResourceManager = borrow_resource_manager!(
                self.draw_request_vault.resource_address()
            );
            let draw_request_nft: Bucket = resource_manager.mint_non_fungible(
                &NonFungibleId::random(),
                DrawRequest {
                    amount: amount,
                }
            );

            self.draw_request_vault.put(draw_request_nft);

            assert!(
                self.draw_request_vault.amount() <= Decimal::one(),
                "[Funding Locker - Draw Request]: Can only request draw one at a time."
            );
        }

        pub fn approve_draw_request(
            &mut self,
            funding_locker_admin: Proof,
        )
        {
            assert_eq!(
                funding_locker_admin.resource_address(), self.funding_locker_admin_address,
                "[Funding Locker - Funding Loan]: Incorrect Proof passed."
            );

            let draw_request: Bucket = self.draw_request_vault.take_all();

            let draw_request_nft_data = draw_request.non_fungible::<DrawRequest>().data();
            let amount: Decimal = draw_request_nft_data.amount;

            let loan_proceeds: Bucket = self.loan_proceeds_vault.take(amount);

            self.draw_vault.put(loan_proceeds);
        }

        pub fn receive_draw(
            &mut self,
            loan_nft_badge: Proof,
        ) -> Bucket
        {
            assert_eq!(
                loan_nft_badge.resource_address(), self.loan_nft_vault.resource_address(),
                "[Funding Locker - Draw Request]: Incorrect Loan NFT badge provided."
            );

            let mut draw_bucket: Bucket = self.draw_vault.take_all();

            // Collateralization calculation logic (maybe think about this last)
            let mut loan_nft_data = self.get_resource_manager();
            let origination_fee_charged: Decimal = loan_nft_data.origination_fee_charged;
            loan_nft_data.remaining_balance += draw_bucket.amount();
            loan_nft_data.last_draw = Runtime::current_epoch();
            self.authorize_update(loan_nft_data);

            let origination_fee_bucket: Bucket = draw_bucket.take(origination_fee_charged);

            self.fee_vault.put(origination_fee_bucket);

            draw_bucket
        }

        pub fn update_loan(
            &mut self,
            funding_locker_admin: Proof,
        )
        {
            assert_eq!(
                funding_locker_admin.resource_address(), self.funding_locker_admin_address,
                "[Funding Locker - Funding Loan]: Incorrect Proof passed."
            );

            let mut loan_nft_data: Loan = self.get_resource_manager();
            let interest_rate: Decimal = loan_nft_data.annualized_interest_rate;
            let remaining_balance: Decimal = loan_nft_data.remaining_balance;
            let last_draw: u64 = loan_nft_data.last_draw;
            let current_epoch = Runtime::current_epoch();
            let time_lapse: u64 = current_epoch - last_draw; 
            let interest_expense: Decimal = remaining_balance * interest_rate * time_lapse;
            loan_nft_data.accrued_interest_expense += interest_expense;

            self.authorize_update(loan_nft_data);
        }

        pub fn make_payment(
            &mut self,
            loan_nft_badge: Proof,
            mut repay_amount: Bucket
        ) -> Option<Bucket>
        {
            assert_eq!(
                loan_nft_badge.resource_address(), self.loan_nft_vault.resource_address(),
                "[Funding Locker - Loan Payment]: Incorrect Loan NFT badge provided."
            );

            assert_eq!(
                repay_amount.resource_address(), self.loan_repay_vault.resource_address(),
                "[Funding Locker - Loan Payment]: Incorrect payment provided."
            );

            // Loan interest rate calculation logic. 
            let mut loan_nft_data: Loan = loan_nft_badge.non_fungible().data();
            let payments_remaining = loan_nft_data.payments_remaining;
            let total_balance = loan_nft_data.accrued_interest_expense + loan_nft_data.remaining_balance;

            assert!(
                payments_remaining > 0,
                "[Funding Locker - Payment]: You have no payments remaining." 
            );

            let mut overpaid_bucket: Bucket = Bucket::new(repay_amount.resource_address());

            if payments_remaining == 1 {

                assert!(
                    repay_amount.amount() >= total_balance,
                    "[Funding Locker - Payment]: Must pay off the entire remaining balance and interest expense owed." 
                );

                // Overpaid logic
                if repay_amount.amount() > total_balance {

                    let overpaid: Decimal = repay_amount.amount() - total_balance;

                    info!(
                        "[Funding Locker - Payment]: You have overpaid your total balance owed by: {:?}. Returning amount...", 
                        overpaid
                    );
    
                    overpaid_bucket.put(repay_amount.take(overpaid));

                    // Updating Loan NFT Data
                    loan_nft_data.remaining_balance = Decimal::zero();
                    loan_nft_data.accrued_interest_expense = Decimal::zero();
                    loan_nft_data.payments_remaining = 0;
                    loan_nft_data.loan_status = Status::PaidOff;

                    self.authorize_update(loan_nft_data);

                    self.fee_vault.put(repay_amount);

                    return Some(overpaid_bucket)

                } else {

                    return None
                }
                
            } else {


                if loan_nft_data.accrued_interest_expense > Decimal::zero() {
                    info!(
                        "[Funding Locker - Payment]: You have an accrued interest expense balance of {:?} remaining.",
                        loan_nft_data.accrued_interest_expense
                    );
                } else {
                    info!(
                        "[Funding Locker - Payment]: Thank you for paying off this month's interest expense balance."
                    );
                }

                // Overpaid logic.
                if repay_amount.amount() > loan_nft_data.accrued_interest_expense {

                    let overpaid: Decimal = repay_amount.amount() - loan_nft_data.accrued_interest_expense;
                    info!(
                        "[Funding Locker - Payment]: You have overpaid your interest expense owed by : {:?}. Returning amount...", 
                        overpaid
                    );
    
                    overpaid_bucket.put(repay_amount.take(overpaid));

                    loan_nft_data.accrued_interest_expense -= repay_amount.amount();
                    loan_nft_data.payments_remaining -= 1;

                    self.authorize_update(loan_nft_data);

                    self.fee_vault.put(repay_amount);

                    return Some(overpaid_bucket)

                } else {
                    
                    return None

                }
            }

            // Loan stats logic.
            
        }

        /// Think about Access Rules.
        /// Note that Debt Fund component at this point already has an access badge.
        pub fn claim_fees(
            &mut self,
            access_badge: Proof,
            percentage: Decimal,
        ) -> Bucket
        {
            assert_eq!(
                access_badge.resource_address(), self.access_badge_address,
                "[Funding Locker - Claim Fees]: Unauthorized Access."
            );

            let amount: Decimal = self.fee_vault.amount() * percentage;
            
            let fee_bucket: Bucket = self.fee_vault.take(amount);

            fee_bucket
        }

        pub fn transfer_liquidity(
            &mut self,
            access_badge: Proof,
        ) -> Bucket
        {
            assert_eq!(
                access_badge.resource_address(), self.access_badge_address,
                "[Funding Locker - Claim Fees]: Unauthorized Access."
            );

            let loan_nft_data: Loan = self.get_resource_manager();
            
            let loan_status = loan_nft_data.loan_status;

            assert_eq!(
                loan_status, Status::PaidOff,
                "[Funding Locker - Redeem Liquidity]: Unauthorized liquidity redemption. The loan is still active."
            );

            let liquidity_bucket: Bucket = self.loan_repay_vault.take_all();

            liquidity_bucket
        }

        pub fn redeem_collateral(
            &mut self,
            loan_nft_badge: Proof,
        ) -> Bucket
        {
            assert_eq!(
                loan_nft_badge.resource_address(), self.loan_nft_vault.resource_address(),
                "[Funding Locker - Loan Payment]: Incorrect Loan NFT badge provided."
            );

            let loan_nft_data: Loan = loan_nft_badge.non_fungible().data();

            let loan_status = loan_nft_data.loan_status;


            assert_eq!(
                loan_status, Status::PaidOff,
                "[Funding Locker - Redeem Collateral]: Unauthorized collateral redemption. The loan is still active."
            );

            let collateral_bucket: Bucket = self.collateral_vault.take_all();

            collateral_bucket
        }

        // pub fn close_loan(
        //     &mut self,
        //     lender: Proof,) -> Bucket
        // {
        //     let funds = self.loan_repay_vault.take_all();
        //     funds
        // }
            
        fn check_collateralization(
            &self
        ) -> bool
        {
            let price_oracle: PriceOracle = self.price_oracle_address.into();
            let loan_nft_data: Loan = self.get_resource_manager();

            let loan_asset: ResourceAddress = loan_nft_data.asset;
            let loan_balance: Decimal = loan_nft_data.remaining_balance;
            let asset_price: Decimal = price_oracle.get_price(loan_asset);
            let loan_balance_value: Decimal = loan_balance * asset_price;

            let collateral_asset: ResourceAddress = self.collateral_vault.resource_address();
            let collateral_balance: Decimal = self.collateral_vault.amount();
            let collateral_price: Decimal = price_oracle.get_price(collateral_asset);
            let collateral_balance_value: Decimal = collateral_balance * collateral_price;

            let current_loan_collateralization: Decimal = loan_balance_value / collateral_balance_value;
            let loan_collateralization_requirement: Decimal = loan_nft_data.collateral_percent;

            if current_loan_collateralization < loan_collateralization_requirement {
                return true
            } else {
                return false
            }
        }

        pub fn liquidate(
            &mut self,
            funding_locker_admin: Proof,
        )
        {
            assert_eq!(
                funding_locker_admin.resource_address(), self.funding_locker_admin_address,
                "[Funding Locker - Liquidation]: Incorrect Proof passed."
            );

            let collateralization_met: bool = self.check_collateralization();

            assert_eq!(
                collateralization_met, true,
                "[Funding Locker - Liquidation]: This loan's collateralization is sufficient and cannot be liquidated."
            );

            let liquidate_amount: Bucket = self.collateral_vault.take_all();

            self.loan_repay_vault.put(liquidate_amount);
        }
    }
}