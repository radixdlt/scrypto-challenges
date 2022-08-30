use scrypto::prelude::*;
use crate::farmers_market::*;
use crate::fundinglocker::*;
use crate::structs::*;


blueprint! {
    /// This is a struct used to define the BorrowerDashboard. The BorrowerDashboard is accessible only to Borrowers.
    /// It is used to allow Borrowers to request loans and interface with their loans. This component is designed
    /// such that there is no need to have multiple components created for each Borrower. The way that is done is
    /// by relying on the _______ to maintain a global record of all the loans. Borrowers will pass their badge
    /// or a proof of their badge to allow the components to identify what loan is associeted to theirs.
    struct BorrowerDashboard {
        /// The ResourceAddress of the Borrower. Only Borrowers can access this Component.
        borrower_admin_address: ResourceAddress,
        /// Borrower Admin Badge to allow this component to change Borrower NFT data.
        borrower_admin_vault: Vault,
        /// Loan Request Admin Badge to allow this component to change Loan Request NFT data.
        loan_request_nft_admin: Vault,
        /// The ResourceAddress of the Loan Request NFT so that the data can be viewed.
        loan_request_nft_address: ResourceAddress,
        /// The ComponentAddress of the Global Index to make method calls.
        farmers_market_global_address: ComponentAddress,
    }

    impl BorrowerDashboard {

        pub fn new(
            farmers_market_global_address: ComponentAddress,
            borrower_admin: Bucket,
            borrower_admin_address: ResourceAddress,
            loan_request_nft_admin: Bucket,
            loan_request_nft_address: ResourceAddress,
        ) -> ComponentAddress
        {

            return Self {
                borrower_admin_address: borrower_admin_address,
                borrower_admin_vault: Vault::with_bucket(borrower_admin),
                loan_request_nft_admin: Vault::with_bucket(loan_request_nft_admin),
                loan_request_nft_address: loan_request_nft_address,
                farmers_market_global_address: farmers_market_global_address,
            }
            .instantiate()
            .globalize();
        }

        /// This method is used to retrieve the NFT data of a selected NFT. 
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `nft_id` (&NonFungibleId) - The NonFungibleId of the NFT data to retrieve.
        /// * `borrower_badge` (BorrowerBadges) - The Enum that matches and retrieves the ResourceAddress of the NFT.
        /// 
        /// # Returns: 
        /// 
        /// * `BorrowerBadgeContainer` - The NFT data of the chosen NFT.
        fn get_resource_manager(
            &self,
            nft_id: &NonFungibleId,
            borrower_badge: BorrowerBadges
        ) -> BorrowerBadgeContainer
        {
            let badge_address = match borrower_badge {
                BorrowerBadges::LoanRequestNFT => self.loan_request_nft_address,
                BorrowerBadges::Borrower => self.borrower_admin_address,
            };

            let resource_manager = borrow_resource_manager!(badge_address);

            match borrower_badge {
                BorrowerBadges::LoanRequestNFT => {
                    let loan_request_nft_data: LoanRequest = resource_manager.get_non_fungible_data(&nft_id);
                    return BorrowerBadgeContainer::LoanRequestNFTContainer(loan_request_nft_data)
                }
                BorrowerBadges::Borrower => {
                    let borrower_nft_data: Borrower = resource_manager.get_non_fungible_data(&nft_id);
                    return BorrowerBadgeContainer::BorrowerContainer(borrower_nft_data)
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
        /// * `badge` (BorrowerBadges) - The Enum of the badge that matches and retrieves the ResourceAddress of the NFT. 
        /// * `nft_data` (BorrowerBadgeContainer) - The Enum that matches and retrieves the NFT data of the NFT.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        fn authorize_update(
            &self,
            nft_id: &NonFungibleId,
            badge: BorrowerBadges,
            nft_data: BorrowerBadgeContainer
        )
        {
            let badge_address = match badge {
                BorrowerBadges::LoanRequestNFT => self.loan_request_nft_address,
                BorrowerBadges::Borrower => self.borrower_admin_address,
            };

            let resource_manager = borrow_resource_manager!(badge_address);
            
            match nft_data {
                BorrowerBadgeContainer::LoanRequestNFTContainer(loan_request) => {
                    self.loan_request_nft_admin.authorize(|| 
                        resource_manager.update_non_fungible_data(nft_id, loan_request)
                    );
                }
                BorrowerBadgeContainer::BorrowerContainer(borrower) => {
                    self.borrower_admin_vault.authorize(|| 
                        resource_manager.update_non_fungible_data(nft_id, borrower)
                    );
                }
            }
        }

        // Implement check to make sure borrower belongs to this protocol. Does it matter to check ID if resource address checks out?
        pub fn request_new_loan(
            &mut self,
            borrower_badge: Bucket,
            token_address: ResourceAddress,
            loan_amount: Decimal,
            collateral_address: ResourceAddress,
            collateral_percent: Decimal,
            term_length: TermLength,
            annualized_interest_rate: Decimal,
        ) -> Bucket
        {
            let borrower_proof: Proof = borrower_badge.create_proof();
            let borrower_id: NonFungibleId = borrower_badge.non_fungible::<Borrower>().id();

            assert_eq!(borrower_badge.resource_address(), self.borrower_admin_address, 
                "[Borrower Dashboard]: Incorrect Proof passed."
            );

            let loan_request_nft = self.loan_request_nft_admin.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.loan_request_nft_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    LoanRequest {
                        asset: token_address,
                        loan_amount: loan_amount,
                        collateral_address: collateral_address,
                        collateral_percent: collateral_percent,
                        term_length: term_length,
                        annualized_interest_rate: annualized_interest_rate,
                        borrower: borrower_id.clone(),
                        status: RequestStatus::Pending,
                        loan_nft_id: None,
                        funding_locker_address: None,
                    },
                )
            });

            let loan_request_nft_id: NonFungibleId = loan_request_nft.non_fungible::<LoanRequest>().id();

            let borrower_badge_container = self.get_resource_manager(&borrower_id, BorrowerBadges::Borrower);

            match borrower_badge_container {
                BorrowerBadgeContainer::BorrowerContainer(mut borrower_nft_data) => {
                    borrower_nft_data.loan_requests.insert(loan_request_nft_id.clone());
                
                    self.authorize_update(
                        &borrower_id, 
                        BorrowerBadges::Borrower, 
                        BorrowerBadgeContainer::BorrowerContainer(borrower_nft_data)
                    );
                }
                _ => {}
            }

            info!("[Borrower Dashboard]: The NFT ID of this loan request is: {:?}", 
                loan_request_nft_id
            );

            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();
            farmers_market.deposit_loan_requests(borrower_proof, loan_request_nft);

            borrower_badge
        }

        pub fn view_loan_request(
            &self,
            loan_request_id: NonFungibleId
        )
        {
            let badge_container = self.get_resource_manager(&loan_request_id, BorrowerBadges::LoanRequestNFT);

            match badge_container {
            
                BorrowerBadgeContainer::LoanRequestNFTContainer(loan_request_nft_data) => {
                    info!("[Borrower Dashboard - Loan Request Info]: Asset: {:?}", loan_request_nft_data.asset);
                    info!("[Borrower Dashboard - Loan Request Info]: Loan Amount: {:?}", loan_request_nft_data.loan_amount);
                    info!("[Borrower Dashboard - Loan Request Info]: Collateral Address: {:?}", loan_request_nft_data.collateral_address);
                    info!("[Borrower Dashboard - Loan Request Info]: Collateral Percent: {:?}", loan_request_nft_data.collateral_percent);
                    info!("[Borrower Dashboard - Loan Request Info]: Term Length: {:?}", loan_request_nft_data.term_length);
                    info!("[Borrower Dashboard - Loan Request Info]: Annualized Interest Rate: {:?}", loan_request_nft_data.annualized_interest_rate);
                    info!("[Borrower Dashboard - Loan Request Info]: Borrower: {:?}", loan_request_nft_data.borrower);
                    info!("[Borrower Dashboard - Loan Request Info]: Status: {:?}", loan_request_nft_data.status);
                    info!("[Borrower Dashboard - Loan Request Info]: Loan NFT ID: {:?}", loan_request_nft_data.loan_nft_id);
                    info!("[Borrower Dashboard - Loan Request Info]: Funding Locker Component Address: {:?}", loan_request_nft_data.funding_locker_address);
                }
                _ => {},
            };
        }

        /// Anyone can essentially view the loan request, but using Proof, since I imagine that's easier from the end user to provide.
        pub fn view_approved_loan_requests(
            &self,
            borrower_badge: Proof,
        ) -> BTreeSet<NonFungibleId>
        {
            let borrower_id: NonFungibleId = borrower_badge.non_fungible::<Borrower>().id();
            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();
            let loan_request_vault: BTreeSet<NonFungibleId> = farmers_market.view_loan_requests(borrower_id);
            let loan_requests = loan_request_vault.iter();
            let mut approved_loans: BTreeSet<NonFungibleId> = BTreeSet::new();
            for loan_id in loan_requests {
                let borrower_badge_container: BorrowerBadgeContainer = self.get_resource_manager(
                    loan_id, 
                    BorrowerBadges::LoanRequestNFT
                );

                match borrower_badge_container {
                    BorrowerBadgeContainer::LoanRequestNFTContainer(loan_request_nft_data) => {
                        if loan_request_nft_data.loan_nft_id.is_some() == true {
                            approved_loans.insert(loan_id.clone());
                        } else {
                            info!(
                                "[Borrower Dashboard - Approved Loan Requests]: No loan requests have been approved."
                            );
                        }
                    }
                    _ => {}
                }
            }

            return approved_loans
        }
        
        /// Retrieves the Loan Request NFT ID to retrieve the correct Funding Locker ComponentAddress
        /// to allow Borrower to deposit collateral into.
        /// 
        /// This method is used to establish a connection between the Borrower side and Pool Delegate side via
        /// the Loan Request NFT. The Pool Delegate instantiates the Funding Locker blueprint with the associated
        /// loan request NFT and mints their proposed loan terms via Loan NFT. Once successfully instantiated,
        /// the Loan Request NFT will be provided with the ComponentAddress of the Funding Locker for the Borrower
        /// to deposit collateral to. When the collateral meets the collateral ratio required, the Borrower receives
        /// the Loan NFT to which the Borrower will use to draw from the loan and meet payment obligations.
        /// 
        /// This method performs a single check.
        /// 
        /// * **Check 1:** Checks that the Proof belongs to the Borrower in control of this Borrower Dashboard.
        /// 
        /// # Arguments:
        /// 
        /// * `borrower_badge` (Proof) - The Proof of the Borrower Admin Badge that belongs to this Borrower Dashboard.
        /// * `loan_request_nft_id` (NonFungibleId) - The NonFungibleId of the Loan Request NFT that was approved.
        /// * `collateral` (Bucket) - The Bucket that contains the collateral deposit.
        /// 
        /// # Returns:
        /// 
        /// * `Option<Bucket>` - The Bucket that contains the Loan NFT if the collateral ratio was met or none if the 
        /// collateral ratio was not met. 
        pub fn deposit_collateral(
            &mut self,
            borrower_badge: Proof,
            loan_request_nft_id: NonFungibleId,
            collateral: Bucket
        ) -> Option<Bucket>
        {
            assert_eq!(borrower_badge.resource_address(), self.borrower_admin_address,
                "[Borrower Dashboard]: Incorrect borrower."
            );

            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();
            // Retrieves loan request NFT and creates Proof so can deposit collateral to Funding Locker
            let loan_request_nft: Bucket = self.borrower_admin_vault.authorize(|| 
                farmers_market.retrieve_loan_request_nft(loan_request_nft_id.clone())
            );

            let loan_request_nft_proof: Proof = loan_request_nft.create_proof();

            let badge_container: BorrowerBadgeContainer = self.get_resource_manager(&loan_request_nft_id, BorrowerBadges::LoanRequestNFT);

            match badge_container {
                BorrowerBadgeContainer::LoanRequestNFTContainer(loan_request_nft_data) => {
                    let funding_locker_address: Option<ComponentAddress> = loan_request_nft_data.funding_locker_address;
                    if funding_locker_address.is_some() {
                        let funding_locker: FundingLocker = funding_locker_address.unwrap().into();
                        let option_bucket: Option<Bucket> = funding_locker.deposit_collateral(loan_request_nft_proof, collateral);

                        match option_bucket {
                            Some(bucket) => {
                                let optional_loan_nft_bucket: Option<Bucket> = Some(bucket);

                                let borrower_id: NonFungibleId = borrower_badge.non_fungible::<Borrower>().id();

                                let borrower_badge_container = self.get_resource_manager(&borrower_id, BorrowerBadges::Borrower);

                                match borrower_badge_container {
                                    BorrowerBadgeContainer::BorrowerContainer(mut borrower_nft_data) => {
                                        borrower_nft_data.loan_requests.remove(&loan_request_nft_id);
                                        borrower_nft_data.loans.insert(
                                            optional_loan_nft_bucket
                                            .as_ref()
                                            .unwrap()
                                            .non_fungible::<Loan>()
                                            .id()
                                        );
                                    
                                        self.authorize_update(
                                            &borrower_id, 
                                            BorrowerBadges::Borrower, 
                                            BorrowerBadgeContainer::BorrowerContainer(borrower_nft_data)
                                        );
                                    }
                                    _ => {}
                                }

                                self.loan_request_nft_admin.authorize(|| loan_request_nft.burn());

                                optional_loan_nft_bucket
                            }

                            None => {

                                self.borrower_admin_vault.authorize(|| 
                                    farmers_market.return_loan_request_nft(loan_request_nft)
                                );

                                return None 
                            }
                        }
                    } else {

                        info!(
                            "[Borrower Dashboard - Deposit Collateral]: This loan has not yet been approved."
                        );

                        return None
                    }
                }

                BorrowerBadgeContainer::BorrowerContainer(_borrower_nft_data) => { return None }
            }
        }

        /// This method allows Borrowers to request loan draws from the Fund Manager.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the FundingLocker
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_nft_badge` (Bucket) - The Bucket that contains the Loan NFT.
        /// * `amount` (Decimal) - Amount of the loan requested to draw.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the Loan NFT.
        pub fn draw_request(
            &self,
            loan_nft_badge: Bucket,
            amount: Decimal,
        ) -> Bucket
        {
            let loan_nft_id: NonFungibleId = loan_nft_badge.non_fungible::<Loan>().id();
            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();
            let funding_locker_address: ComponentAddress = farmers_market.retrieve_funding_locker_address(loan_nft_id);
            let funding_locker: FundingLocker = funding_locker_address.into();
            let loan_nft_proof: Proof = loan_nft_badge.create_proof();
            funding_locker.draw_request(loan_nft_proof, amount);
            
            loan_nft_badge
        }

        /// This method allows Borrowers to retrieve their loan draw.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the FundingLocker
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `loan_nft_badge` (Bucket) - The Bucket that contains the Loan NFT.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the Loan NFT.
        /// * `Bucket` - The Bucket that contains the loan draw.
        pub fn receive_draw(
            &self,
            loan_nft_badge: Bucket,
        ) -> (Bucket, Bucket)
        {
            let loan_nft_id: NonFungibleId = loan_nft_badge.non_fungible::<Loan>().id();
            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();
            let funding_locker_address: ComponentAddress = farmers_market.retrieve_funding_locker_address(loan_nft_id);
            let funding_locker: FundingLocker = funding_locker_address.into();
            let loan_nft_proof: Proof = loan_nft_badge.create_proof();
            let draw_bucket: Bucket = funding_locker.receive_draw(loan_nft_proof);

            (loan_nft_badge, draw_bucket)
        }

        /// This method allows Borrowers to make payments on their loan.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the FundingLocker
        /// component.
        /// 
        /// # Arguments: 
        /// 
        /// * `loan_nft_badge` (Bucket) - The Bucket that contains the Loan NFT.
        /// * `repay_amount` (Bucket) - The Bucket of the repayments.
        /// 
        /// # Returns:
        /// 
        /// * `(Option<Bucket>, Bucket)` - The Option Bucket that contains the overpayment if the Borrower overpaid on the loan.
        /// and the Bucket that contains the Loan NFT.
        pub fn make_payment(
            &self,
            loan_nft_badge: Bucket,
            repay_amount: Bucket,
        ) -> (Option<Bucket>, Bucket)
        {
            let loan_nft_id: NonFungibleId = loan_nft_badge.non_fungible::<Loan>().id();
            let farmers_market: FarmersMarket = self.farmers_market_global_address.into();
            let funding_locker_address: ComponentAddress = farmers_market.retrieve_funding_locker_address(loan_nft_id);
            let funding_locker: FundingLocker = funding_locker_address.into();
            let loan_nft_proof: Proof = loan_nft_badge.create_proof();
            let option_over_repayment: Option<Bucket> = funding_locker.make_payment(loan_nft_proof, repay_amount);

            (option_over_repayment, loan_nft_badge)
        }

        /// This method allows the Borrower view all the loan request they have made.
        /// 
        /// # Checks:
        /// 
        /// * **Check 1:** - Checks that the Proof provided is the Borrower.
        /// 
        /// # Arguments:
        /// 
        /// * `borrower_badge` (Proof) - The Proof of the Borrower Badge.
        /// 
        /// # Returns:
        /// 
        /// * `BTreeSet<NonFungibleId>` - The BTreeSet of the NFT ID of the loan requests.
        pub fn view_loans(
            &mut self,
            borrower_badge: Proof,
        ) -> BTreeSet<NonFungibleId>
        {
            assert_eq!(borrower_badge.resource_address(), self.borrower_admin_address,
                "[Borrower Dashboard]: Incorrect borrower."
            );

            let borrower_nft_data: Borrower = borrower_badge.non_fungible().data();

            return borrower_nft_data.loans
        }

        pub fn view_loan(
            &self,
            loan_nft_badge: Proof,
        )
        {
            let loan_nft_data: Loan = loan_nft_badge.non_fungible().data();
            let borrower_id: NonFungibleId = loan_nft_data.borrower_id;
            let lender_id: NonFungibleId = loan_nft_data.lender_id;
            let principal_loan_amount: Decimal = loan_nft_data.principal_loan_amount;
            let asset_address: ResourceAddress = loan_nft_data.asset;
            let collateral_address: ResourceAddress = loan_nft_data.collateral;
            let collateral_percent: Decimal = loan_nft_data.collateral_percent;
            let annualized_interest_rate: Decimal = loan_nft_data.annualized_interest_rate;
            let term_length: TermLength = loan_nft_data.term_length;
            let payments_remaining: u64 = loan_nft_data.payments_remaining;
            let origination_fee: Decimal = loan_nft_data.origination_fee;
            let origination_fee_charged: Decimal = loan_nft_data.origination_fee_charged;
            let accrued_interest_expense: Decimal = loan_nft_data.accrued_interest_expense;
            let remaining_balance: Decimal = loan_nft_data.remaining_balance;
            let draw_limit: Decimal = loan_nft_data.draw_limit;
            let draw_minimum: Decimal = loan_nft_data.draw_minimum;
            let last_draw: u64 = loan_nft_data.last_draw;
            let collateral_amount: Decimal = loan_nft_data.collateral_amount;
            let loan_status: Status = loan_nft_data.loan_status;

            info!("[Borrower Dashboard - View Loan] - The Borrower ID is: {:?}", borrower_id);
            info!("[Borrower Dashboard - View Loan] - The Lender ID is: {:?}", lender_id);
            info!("[Borrower Dashboard - View Loan] - The principal loan amount is: {:?}", principal_loan_amount);
            info!("[Borrower Dashboard - View Loan] - Asset borrowed: {:?}", asset_address);
            info!("[Borrower Dashboard - View Loan] - Collateral borrowed: {:?}", collateral_address);
            info!("[Borrower Dashboard - View Loan] - The collateral percent: {:?}", collateral_percent);
            info!("[Borrower Dashboard - View Loan] - Annualized Interest Rate: {:?}", annualized_interest_rate);
            info!("[Borrower Dashboard - View Loan] - Term Length: {:?}", term_length);
            info!("[Borrower Dashboard - View Loan] - Payments Remaining: {:?}", payments_remaining);
            info!("[Borrower Dashboard - View Loan] - Origination Fee: {:?}", origination_fee);
            info!("[Borrower Dashboard - View Loan] - Origination Fee Charged: {:?}", origination_fee_charged);
            info!("[Borrower Dashboard - View Loan] - Accrued Interest Expense: {:?}", accrued_interest_expense);
            info!("[Borrower Dashboard - View Loan] - Remaining Balance: {:?}", remaining_balance);
            info!("[Borrower Dashboard - View Loan] - Draw Limit: {:?}", draw_limit);
            info!("[Borrower Dashboard - View Loan] - Draw Minimum: {:?}", draw_minimum);
            info!("[Borrower Dashboard - View Loan] - Last Draw: {:?}", last_draw);
            info!("[Borrower Dashboard - View Loan] - Collateral Amount: {:?}", collateral_amount);
            info!("[Borrower Dashboard - View Loan] - Loan Status: {:?}", loan_status);

        }
    }
}