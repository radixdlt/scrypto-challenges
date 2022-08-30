use scrypto::prelude::*;
use crate::structs::*;
use crate::price_oracle::*;

blueprint! {
    struct FundingLocker {
        /// The ResourceAddress of the Fund Manager that has authority over this Funding Locker. 
        funding_locker_badge_address: ResourceAddress,
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
        loan_nft_admin_vault: Vault,
        loan_nft_id: NonFungibleId,
        loan_nft_address: ResourceAddress,
        loan_proceed_status: Status,
        draw_request_vault: Vault,
        draw_vault: Vault,
        fee_vault: Vault,
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
            let funding_locker_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Funding Locker Badge")
                .metadata("symbol", "FLB")
                .metadata("description", "Badge to access the Funding Locker.")
                .initial_supply(1);

            // NFT description for Pool Delegates
            let draw_request_nft_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Draw Request NFT")
                .metadata("symbol", "DR_NFT")
                .metadata("description", "Draw requests from the Borrower")
                .mintable(rule!(require(loan_nft_admin.resource_address())), LOCKED)
                .burnable(rule!(require(loan_nft_admin.resource_address())), LOCKED)
                .no_initial_supply();
            
            let loan_nft_data = loan_nft.non_fungible::<Loan>().data();
            let draw_limit: Decimal = loan_nft_data.draw_limit;
            let draw_minimum: Decimal = loan_nft_data.draw_minimum;
            let loan_asset_address = loan_nft_data.asset;
            let loan_collateral_address = loan_nft_data.collateral;
            let loan_nft_id = loan_nft.non_fungible::<Loan>().id();
            let loan_nft_address = loan_nft.resource_address();

            let access_rules: AccessRules = AccessRules::new()
                .method("fund_loan", rule!(require(funding_locker_badge.resource_address())))
                .method("approve_draw_request", rule!(require(funding_locker_badge.resource_address())))
                .method("reject_draw_request", rule!(require(funding_locker_badge.resource_address())))
                .method("reject_draw_request", rule!(require(funding_locker_badge.resource_address())))
                .method("update_loan", rule!(require(funding_locker_badge.resource_address())))
                .method("transfer_fees", rule!(require(funding_locker_badge.resource_address())))
                .method("liquidate", rule!(require(funding_locker_badge.resource_address())))
                .method("transfer_liquidity", rule!(require(loan_nft_admin.resource_address())))
                .default(rule!(allow_all)
            );

            let funding_locker: ComponentAddress = Self {
                funding_locker_badge_address: funding_locker_badge.resource_address(),
                loan_repay_vault: Vault::new(loan_asset_address),
                loan_proceeds_vault: Vault::new(loan_asset_address),
                draw_limit: draw_limit,
                draw_minimum: draw_minimum,
                grace_period: 10,
                loan_nft_vault: Vault::with_bucket(loan_nft),
                loan_request_nft_id: loan_request_nft_id,
                loan_request_nft_address: loan_request_nft_address,
                collateral_vault: Vault::new(loan_collateral_address),
                loan_nft_admin_vault: Vault::with_bucket(loan_nft_admin),
                loan_nft_id: loan_nft_id,
                loan_nft_address: loan_nft_address,
                loan_proceed_status: Status::Unfunded,
                draw_request_vault: Vault::new(draw_request_nft_address),
                draw_vault: Vault::new(loan_asset_address),
                fee_vault: Vault::new(loan_asset_address),
                price_oracle_address: price_oracle_address,
            }

            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            return (funding_locker, funding_locker_badge);
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
            self.loan_nft_admin_vault.authorize(|| 
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

            let mut loan_nft_data = self.get_resource_manager();

            loan_nft_data.collateral_amount += collateral.amount();

            self.collateral_vault.put(collateral);

            let collateral_amount = self.collateral_vault.amount();

            let principal_loan_amount = loan_nft_data.principal_loan_amount;
            let collateral_percent = loan_nft_data.collateral_percent;

            if ( collateral_amount / principal_loan_amount ) >= collateral_percent {
                loan_nft_data.loan_status = Status::ReadyToFund;

                // Authorize logic
                self.authorize_update(loan_nft_data);

                let return_loan_nft = Some(self.loan_nft_vault.take_non_fungible(&self.loan_nft_id));

                info!("[Funding Locker]: Collateralization requirement met!");
                info!("[Funding Locker]: You've received a Loan NFT. Use this Loan NFT to access the Funding Locker.");
                info!(
                    "[Funding Locker]: The resource address of your Loan NFT is: {:?}", 
                    self.loan_nft_vault.resource_address()
                );

                return_loan_nft

            } else {
                
                // Authorize logic
                self.authorize_update(loan_nft_data);

                info!("[Funding Locker]: Insufficient collateralization.");

                info!(
                    "[Funding Locker]: Your collateral percentage is {:?}. 
                    You must at least provide {:?} collateralization before this loan can be funded",
                    ( collateral_amount / principal_loan_amount ),
                    collateral_percent
                );

                let return_loan_nft = None;

                return_loan_nft
            }
        }

        /// This method allows the Fund Manager to fund the loan for Borrowers to draw from.
        /// 
        /// # Checks: 
        /// 
        /// * **Check 1:** - Checks that the Bucket passed is the correct tokens required to fund the loan proceeds.
        /// 
        /// # Arguments:
        /// 
        /// * `amount` (Bucket) - The Bucket that contains the loan proceeds.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket the over funded amount if the Fund Manager overfunded the loan.
        pub fn fund_loan(
            &mut self,
            amount: Bucket,
        ) -> Bucket
        {
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

            return_over_funded
        }

        /// This method allows Borrowers to request loan draws from the Fund Manager.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof of the Loan NFT is correct.
        /// * **Check 2:** - Checks whether the loan is ready to be funded. 
        /// * **Check 3:** - Checks that the draw request meets the draw minimum requirement.
        /// * **Check 4:** - Checks that the draw request does not exceed the draw limit requirement.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_nft_badge` (Proof) - The Proof of the Loan NFT.
        /// * `amount` (Decimal) - Amount of the loan requested to draw.
        /// 
        /// # Returns:
        /// 
        /// * This method does not return anything.
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


            let draw_request_nft = self.loan_nft_admin_vault.authorize(|| {
                    let resource_manager: &ResourceManager = borrow_resource_manager!(
                        self.draw_request_vault.resource_address()
                    );
                    resource_manager.mint_non_fungible(
                    &NonFungibleId::random(),
                    DrawRequest {
                        amount: amount,
                    },
                )
            });

            info!(
                "[Funding Locker - Draw Request]: You've made a draw request to the amount of {:?}, {:?}",
                amount, self.loan_proceeds_vault.resource_address()
            );
            
            self.draw_request_vault.put(draw_request_nft);

            assert!(
                self.draw_request_vault.amount() <= Decimal::one(),
                "[Funding Locker - Draw Request]: Can only request draw one at a time."
            );
        }

        pub fn view_draw_request(
            &self,
        )
        {
            let draw_request_nft_data: DrawRequest = self.draw_request_vault.non_fungible().data();

            let draw_request_amount: Decimal = draw_request_nft_data.amount;

            info!(
                "[Funding Locker - View Draw Request]: Draw request amount: {:?}",
                draw_request_amount
            );
        }

        /// This method allows the Fund Manager to approve the draw request.
        /// 
        /// # Checks: 
        /// 
        /// * **Check 1:** - Checks that there is a draw request made.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn approve_draw_request(
            &mut self,
        )
        {
            assert_eq!(
                self.draw_request_vault.amount(), Decimal::one(),
                "[Funding Locker - Draw Request Approval]: There are no draw request made."
            );

            let draw_request: Bucket = self.draw_request_vault.take_all();

            let draw_request_nft_data = draw_request.non_fungible::<DrawRequest>().data();
            let amount: Decimal = draw_request_nft_data.amount;

            info!(
                "[Funding Locker - Draw Request Approval]: Draw request {:?} of the amount {:?} has been approved!",
                draw_request.non_fungible::<DrawRequest>().id(),
                amount   
            );

            let loan_proceeds: Bucket = self.loan_proceeds_vault.take(amount);

            self.draw_vault.put(loan_proceeds);

            self.loan_nft_admin_vault.authorize(||
                draw_request.burn()
            );
        }

        /// This method allows Fund Managers to reject the draw request made by the Borrower.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that there was a draw request made.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn reject_draw_request(
            &mut self,
        )
        {
            assert_eq!(
                self.draw_request_vault.amount(), Decimal::one(),
                "[Funding Locker - Draw Request Approval]: There are no draw request made."
            );

            let draw_request: Bucket = self.draw_request_vault.take_all();

            self.loan_nft_admin_vault.authorize(||
                draw_request.burn()
            );
        }

        /// This method allows Borrowers to retrieve their loan draw.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof of the Loan NFT is correct.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_nft_badge` (Proof) - The Proof of the Loan NFT.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the loan draw.
        pub fn receive_draw(
            &mut self,
            loan_nft_badge: Proof,
        ) -> Bucket
        {
            assert_eq!(
                loan_nft_badge.resource_address(), self.loan_nft_vault.resource_address(),
                "[Funding Locker - Receive Draw]: Incorrect Loan NFT badge provided."
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
            
            info!(
                "[Funding Locker - Receive Draw]: You've received {:?} of {:?} in funding.",
                draw_bucket.amount(),
                draw_bucket.resource_address()
            );

            draw_bucket
        }

        /// This method allows the Fund Manager to update the loan with the Interest Expense calculation.
        /// 
        /// This method does not perform any
        pub fn update_loan(
            &mut self,

        )
        {
            let mut loan_nft_data: Loan = self.get_resource_manager();
            let interest_rate: Decimal = loan_nft_data.annualized_interest_rate;
            let remaining_balance: Decimal = loan_nft_data.remaining_balance;
            let last_draw: u64 = loan_nft_data.last_draw;
            let current_epoch = Runtime::current_epoch();
            let time_lapse: u64 = current_epoch - last_draw; 
            let interest_expense: Decimal = remaining_balance * interest_rate * time_lapse;
            loan_nft_data.accrued_interest_expense += interest_expense;

            info!(
                "[Funding Locker - Update Loan]: {:?} epoch has lasped since the last draw.",
                time_lapse
            );

            info!(
                "[Funding Locker - Update Loan]: {:?} in interest expense has accrued.",
                interest_expense
            );

            self.authorize_update(loan_nft_data);
        }

        /// This method allows Borrowers to make payments on their loan.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof provided is correct.
        /// * **Check 2:** - Checks that the repayment is the correct token.
        /// * **Check 3:** - Checks that the loan has actually been funded.
        /// * **Check 4:** - Checks that the minimum draw amount has been drawn.
        /// * **Check 5:** - Checks that there are still payments remaining.
        /// * **Check 6:** - If this is the last payment, checks that the amount passed
        /// pays off the entire loan balance and accrued interest expense owed.
        /// 
        /// # Arguments: 
        /// 
        /// * `loan_nft_badge` (Proof) - The Proof of the Loan NFT.
        /// * `mut repay_amount` (Bucket) - The Bucket of the repayments.
        /// 
        /// # Returns:
        /// 
        /// * `Option<Bucket>` - The Bucket that contains the overpayment if the Borrower overpaid on the loan.
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

            assert_eq!(
                self.loan_proceed_status, Status::Funded,
                "[Funding Locker - Loan Payment]: You cannot pay back a loan that has not yet funded."
            );

            // Loan interest rate calculation logic. 
            let mut loan_nft_data: Loan = loan_nft_badge.non_fungible().data();
            
            assert!(
                loan_nft_data.remaining_balance >= loan_nft_data.draw_minimum,
                "[Funding Locker - Loan Payment]: You cannot pay back a loan that has not been drawn."
            );

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
                
            } else { // If there are more than 1 payments remaining... only interest expense is owed.

                let mut loan_nft_data: Loan = loan_nft_badge.non_fungible().data();

                // If overpaid... return overpaid.
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

                } else { // Else just pay down the interest expense...

                    loan_nft_data.accrued_interest_expense -= repay_amount.amount();

                    self.fee_vault.put(repay_amount);

                    self.authorize_update(loan_nft_data);

                    let loan_nft_data: Loan = loan_nft_badge.non_fungible().data();

                    if loan_nft_data.accrued_interest_expense > Decimal::zero() {
                        info!(
                            "[Funding Locker - Payment]: You have an accrued interest expense balance of {:?} remaining.",
                            loan_nft_data.accrued_interest_expense
                        );
    
                        self.authorize_update(loan_nft_data);
    
                        return None
    
                    } else {
                        
                        info!(
                            "[Funding Locker - Payment]: Thank you for paying off this month's interest expense balance."
                        );
    
                        return None
                    }
                }
            }
        }

        /// This method allows the Fund Manager to transfer the fees accrued in this loan to the Debt Fund.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that there are fees contained in the fee vault.
        /// 
        /// This method imposes an Access Rule that requires the Loan NFT Admin Badge present.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contain the fees.
        pub fn transfer_fees(
            &mut self,
        ) -> Bucket
        {
            assert!(
                self.fee_vault.amount() > Decimal::zero(),
                "[Funding Locker - Fee Vault]: No fees have yet been collected."
            );

            let fee_bucket: Bucket = self.fee_vault.take_all();

            info!(
                "[Funding Locker - Fee Vault]: Transfering fees collected | Amount: {:?} | Token: {:?}",
                fee_bucket.amount(),
                fee_bucket.resource_address()
            );

            fee_bucket
        }

        /// This method allows the Fund Manager to transfer the liquidity provided to fund the loan back into
        /// the Debt Fund.
        /// 
        /// # Checks: 
        /// 
        /// * **Check 1:** - Checks that the loan has been paid off.
        /// 
        /// This method imposes an Access Rule that requires the Loan NFT Admin Badge present.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the liquidity.
        pub fn transfer_liquidity(
            &mut self,
        ) -> Bucket
        {

            let loan_nft_data: Loan = self.get_resource_manager();
            
            let loan_status = loan_nft_data.loan_status;

            assert_eq!(
                loan_status, Status::PaidOff,
                "[Funding Locker - Redeem Liquidity]: Unauthorized liquidity redemption. The loan is still active."
            );

            let liquidity_bucket: Bucket = self.loan_repay_vault.take_all();

            liquidity_bucket
        }

        /// This method allows the Borrower to redeem their collateral.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof of the Loan NFT is correct.
        /// * **Check 1:** - Checks that the loan has been paid off.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_nft_badge` (Proof) - The Proof of the Loan NFT.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the collateral.
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
        
        /// This method allows the component to calculate the collateralization requirement.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the loan has been funded.
        /// * **Check 2:** - Checks that the loan has been drawn.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `bool` - The bool wether the collateralization requirement has been met or not.
        fn check_collateralization(
            &self
        ) -> bool
        {
            assert_eq!(
                self.loan_proceed_status, Status::Funded,
                "[Funding Locker - Check Collateralization]: The loan must first be funded before collateralization is checked."
            );

            let price_oracle: PriceOracle = self.price_oracle_address.into();
            let loan_nft_data: Loan = self.get_resource_manager();

            assert!(
                loan_nft_data.remaining_balance >= loan_nft_data.draw_minimum,
                "[Funding Locker - Check Collateralization]: The loan must first be drawn from before collateralization is checked."
            );

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

        /// This method allows the Fund Manager to liquidate the loan if the collateralization requirement is not met.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the 
        pub fn liquidate(
            &mut self,
        )
        {
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