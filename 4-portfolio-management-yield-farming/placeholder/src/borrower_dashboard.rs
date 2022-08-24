use scrypto::prelude::*;
use crate::maple_finance_global::*;
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
        maple_finance_global_address: ComponentAddress,
    }

    impl BorrowerDashboard {

        pub fn new(
            maple_finance_global_address: ComponentAddress,
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
                maple_finance_global_address: maple_finance_global_address,
            }
            .instantiate()
            .globalize();
        }

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
                "[Borrower Dashboard: Incorrect Proof passed."
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

            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            maple_finance.deposit_loan_requests(borrower_proof, loan_request_nft);

            borrower_badge
        }

        /// Broadcast loan requests from this borrower.
        /// 
        /// This method is used to broadcast loan request from the borrower. It takes the NFT IDs of the loan
        /// request NFTs inside the loan_requests vault and returns the HashMap of the ResourceAddress of the loan
        /// request NFT and the BTreeSet of the NonFungibleIds of each loan request. This method gets picked up
        /// by the MapleFinance blueprint where other blueprints can view that data. This method is the start to
        /// allow the borrower logic and lender logic to marry up.
        // pub fn broadcast_loan_requests(
        //     &mut self) -> HashMap<ResourceAddress, BTreeSet<NonFungibleId>>
        // {
        //     let mut loan_requests: HashMap<ResourceAddress, BTreeSet<NonFungibleId>> = HashMap::new();
        //     loan_requests.insert(
        //         self.loan_request_nft_address, 
        //         self.loan_requests.non_fungible_ids().clone()
        //     );
        //     return loan_requests 
        // }

        /// Iterates over the loan request vault to find loans that have been approved and inserts approved loans
        /// to the funding_lockers HashMap.
        /// 
        /// This method is used to find loans that have been approved by a Pool Delegate. The Pool Delegate modifies the
        /// loan request NFT to fill in the ComponentAddress of the Funding Locker (where loans are funded and can be borrowed from)
        /// to which this method iterates and find any loan request NFTs that have been filled. If they are filled, then it will be
        /// inserted to the funding_lockers HashMap so that the Borrower has access to the blueprint where the loans are funded.
        // fn seek_approved_loans(
        //     &mut self, 
        // )
        // {
        //     let loan_request_vault: BTreeSet<NonFungibleId> = self.loan_requests.non_fungible_ids();
        //     let loan_requests = loan_request_vault.iter();
        //     for loan_id in loan_requests {
        //         let loan_request_nft_data = self.get_resource_manager(loan_id);
        //         if loan_request_nft_data.loan_nft_id.is_some() == true {
        //             let loan_nft_id = loan_request_nft_data.loan_nft_id.unwrap();
        //             let funding_locker_address = loan_request_nft_data.funding_locker_address.unwrap();
        //             self.funding_lockers.insert(loan_nft_id, funding_locker_address);
        //         }
        //     }
        // }

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
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            let loan_request_vault: BTreeSet<NonFungibleId> = maple_finance.view_loan_requests(borrower_id);
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
            borrower_badge: Bucket,
            loan_request_nft_id: NonFungibleId,
            collateral: Bucket
        ) -> (Bucket, Option<Bucket>)
        {
            assert_eq!(borrower_badge.resource_address(), self.borrower_admin_address,
                "[Borrower Dashboard]: Incorrect borrower."
            );

            // Creates Borrower Badge Proof so can call method from Global Index and retrieve loan request NFT.
            let borrower_proof: Proof = borrower_badge.create_proof();
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            // Retrieves loan request NFT and creates Proof so can deposit collateral to Funding Locker
            let loan_request_nft: Bucket = maple_finance.retrieve_loan_request_nft(borrower_proof, loan_request_nft_id.clone());
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
                                let return_bucket: Option<Bucket> = Some(bucket);

                                self.loan_request_nft_admin.authorize(|| loan_request_nft.burn());

                                (borrower_badge, return_bucket)
                            }
                            None => (borrower_badge, None)
                        }
                    } else {
                        (borrower_badge, None)
                    }
                }
                BorrowerBadgeContainer::BorrowerContainer(_borrower_nft_data) => { (borrower_badge, None) }
            }

            // let loan_nft_id: NonFungibleId = loan_request_nft_data.loan_nft_id.unwrap();
            // let optional_funding_locker: Option<&ComponentAddress> = self.funding_lockers.get(&loan_nft_id);
            // match optional_funding_locker {
            //     Some (_funding_locker) => {
            //         let funding_locker_address: ComponentAddress = *optional_funding_locker.unwrap();
            //         let funding_locker: FundingLocker = funding_locker_address.into();
            //         let loan_request_nft: Bucket = self.loan_requests.take_non_fungible(&loan_request_nft_id);
            //         let loan_request_nft_proof: Proof = loan_request_nft.create_proof();
            //         let option_bucket: Option<Bucket> = funding_locker.deposit_collateral(loan_request_nft_proof, collateral);
            //         match option_bucket {
            //             Some (bucket) => {
            //                 let return_bucket = Some(bucket);

            //                 self.loan_request_nft_admin.authorize(|| loan_request_nft.burn());

            //                 return_bucket
            //             }
            //             None => None
            //         }
            //     }
            //     None => None
            // }
        }

        pub fn draw_request(
            &self,
            loan_nft_badge: Bucket,
            amount: Decimal,
        ) -> Bucket
        {
            let loan_nft_id: NonFungibleId = loan_nft_badge.non_fungible::<Loan>().id();
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            let funding_locker_address: ComponentAddress = maple_finance.retrieve_funding_locker_address(loan_nft_id);
            let funding_locker: FundingLocker = funding_locker_address.into();
            let loan_nft_proof: Proof = loan_nft_badge.create_proof();
            funding_locker.draw_request(loan_nft_proof, amount);
            
            loan_nft_badge
        }

        pub fn receive_draw(
            &self,
            loan_nft_badge: Bucket,
        ) -> (Bucket, Bucket)
        {
            let loan_nft_id: NonFungibleId = loan_nft_badge.non_fungible::<Loan>().id();
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            let funding_locker_address: ComponentAddress = maple_finance.retrieve_funding_locker_address(loan_nft_id);
            let funding_locker: FundingLocker = funding_locker_address.into();
            let loan_nft_proof: Proof = loan_nft_badge.create_proof();
            let draw_bucket: Bucket = funding_locker.receive_draw(loan_nft_proof);

            (loan_nft_badge, draw_bucket)
        }

        pub fn make_payment(
            &self,
            loan_nft_badge: Bucket,
            repay_amount: Bucket,
        ) -> Bucket
        {
            let loan_nft_id: NonFungibleId = loan_nft_badge.non_fungible::<Loan>().id();
            let maple_finance: MapleFinance = self.maple_finance_global_address.into();
            let funding_locker_address: ComponentAddress = maple_finance.retrieve_funding_locker_address(loan_nft_id);
            let funding_locker: FundingLocker = funding_locker_address.into();
            let loan_nft_proof: Proof = loan_nft_badge.create_proof();
            funding_locker.make_payment(loan_nft_proof, repay_amount);

            loan_nft_badge
        }
    }
}