use scrypto::prelude::*;
use crate::{structs::*, fundinglocker::FundingLocker};

// Allows approved Pool Delegate to manage pools.

blueprint! {
    struct BorrowerDashboard {
        borrower_admin_address: ResourceAddress,
        borrower_id: NonFungibleId,
        loan_request_nft_admin: Vault,
        loan_request_nft_address: ResourceAddress,
        loan_requests: Vault,
        funding_lockers: HashMap<NonFungibleId, ComponentAddress>,
    }

    impl BorrowerDashboard {

        pub fn new(
            borrower_admin_address: ResourceAddress,
            borrower_id: NonFungibleId,
            loan_request_nft_admin: Bucket) -> ComponentAddress
        {
            // NFT description for Pool Delegates
            let loan_request_nft_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Loan Request NFT")
                .metadata("symbol", "LRNFT")
                .metadata("description", "Loan Request Terms")
                .mintable(rule!(require(loan_request_nft_admin.resource_address())), LOCKED)
                .burnable(rule!(require(loan_request_nft_admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(loan_request_nft_admin.resource_address())), LOCKED)
                .no_initial_supply();

            return Self {
                borrower_admin_address: borrower_admin_address,
                borrower_id: borrower_id,
                loan_request_nft_admin: Vault::with_bucket(loan_request_nft_admin),
                loan_request_nft_address: loan_request_nft_address,
                loan_requests: Vault::new(loan_request_nft_address),
                funding_lockers: HashMap::new(),
            }
            .instantiate()
            .globalize();
        }

        fn get_resource_manager(
            &self,
            loan_request_id: &NonFungibleId) -> LoanRequest
        {
            let resource_manager = borrow_resource_manager!(self.loan_request_nft_address);
            let loan_request_nft_data: LoanRequest = resource_manager.get_non_fungible_data(loan_request_id);
            loan_request_nft_data
        }

        pub fn request_new_loan(
            &mut self,
            borrower_proof: Proof,
            token_address: ResourceAddress,
            loan_amount: Decimal,
            collateral_address: ResourceAddress,
            collateral_percent: Decimal,
            term_length: u64,
            annualized_interest_rate: Decimal,
            payment_frequency: PaymentFrequency)
        {
            assert_eq!(borrower_proof.resource_address(), self.borrower_admin_address, 
                "[Borrower Dashboard: Incorrect Proof passed."
            );

            assert_eq!(borrower_proof.non_fungible::<Borrower>().id(), self.borrower_id,
                "[Borrower Dashboard: Incorrect user."
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
                        payment_frequency: payment_frequency,
                        borrower: borrower_proof.non_fungible::<Borrower>().id(),
                        status: RequestStatus::Pending,
                        loan_nft_id: None,
                        funding_locker_address: None,
                    },
                )
            });

            info!("[Borrower Dashboard]: The NFT ID of this loan request is: {:?}", 
                loan_request_nft.non_fungible::<LoanRequest>().id()
            );

            self.loan_requests.put(loan_request_nft);
        }

        /// Broadcast loan requests from this borrower.
        /// 
        /// This method is used to broadcast loan request from the borrower. It takes the NFT IDs of the loan
        /// request NFTs inside the loan_requests vault and returns the HashMap of the ResourceAddress of the loan
        /// request NFT and the BTreeSet of the NonFungibleIds of each loan request. This method gets picked up
        /// by the MapleFinance blueprint where other blueprints can view that data. This method is the start to
        /// allow the borrower logic and lender logic to marry up.
        pub fn broadcast_loan_requests(
            &mut self) -> HashMap<ResourceAddress, BTreeSet<NonFungibleId>>
        {
            let mut loan_requests: HashMap<ResourceAddress, BTreeSet<NonFungibleId>> = HashMap::new();
            loan_requests.insert(
                self.loan_request_nft_address, 
                self.loan_requests.non_fungible_ids().clone()
            );
            return loan_requests 
        }

        /// Iterates over the loan request vault to find loans that have been approved and inserts approved loans
        /// to the funding_lockers HashMap.
        /// 
        /// This method is used to find loans that have been approved by a Pool Delegate. The Pool Delegate modifies the
        /// loan request NFT to fill in the ComponentAddress of the Funding Locker (where loans are funded and can be borrowed from)
        /// to which this method iterates and find any loan request NFTs that have been filled. If they are filled, then it will be
        /// inserted to the funding_lockers HashMap so that the Borrower has access to the blueprint where the loans are funded.
        fn seek_approved_loans(
            &mut self, 
        )
        {
            let loan_request_vault: BTreeSet<NonFungibleId> = self.loan_requests.non_fungible_ids();
            let loan_requests = loan_request_vault.iter();
            for loan_id in loan_requests {
                let loan_request_nft_data = self.get_resource_manager(loan_id);
                if loan_request_nft_data.loan_nft_id.is_some() == true {
                    let loan_nft_id = loan_request_nft_data.loan_nft_id.unwrap();
                    let funding_locker_address = loan_request_nft_data.funding_locker_address.unwrap();
                    self.funding_lockers.insert(loan_nft_id, funding_locker_address);
                }
            }
        }

        pub fn view_approved_loans(
            &mut self) -> HashMap<NonFungibleId, ComponentAddress>
        {
            self.seek_approved_loans();
            return self.funding_lockers.clone()
        }

        pub fn view_approved_loan_requests(
            &self) -> BTreeSet<NonFungibleId>
        {
            let loan_request_vault: BTreeSet<NonFungibleId> = self.loan_requests.non_fungible_ids();
            let loan_requests = loan_request_vault.iter();
            let mut approved_loans: BTreeSet<NonFungibleId> = BTreeSet::new();
            for loan_id in loan_requests {
                let loan_request_nft_data = self.get_resource_manager(loan_id);
                if loan_request_nft_data.loan_nft_id.is_some() == true {
                    approved_loans.insert(loan_id.clone());
                }
            }

            return approved_loans
        }

        pub fn view_loan_request(
            &self,
            loan_request_id: NonFungibleId)
        {
            let loan_request_nft_data = self.get_resource_manager(&loan_request_id);

            info!("[Borrower Dashboard - Loan Request Info]: Asset: {:?}", loan_request_nft_data.asset);
            info!("[Borrower Dashboard - Loan Request Info]: Loan Amount: {:?}", loan_request_nft_data.loan_amount);
            info!("[Borrower Dashboard - Loan Request Info]: Collateral Address: {:?}", loan_request_nft_data.collateral_address);
            info!("[Borrower Dashboard - Loan Request Info]: Collateral Percent: {:?}", loan_request_nft_data.collateral_percent);
            info!("[Borrower Dashboard - Loan Request Info]: Term Length: {:?}", loan_request_nft_data.term_length);
            info!("[Borrower Dashboard - Loan Request Info]: Annualized Interest Rate: {:?}", loan_request_nft_data.annualized_interest_rate);
            info!("[Borrower Dashboard - Loan Request Info]: Payment Frequency: {:?}", loan_request_nft_data.payment_frequency);
            info!("[Borrower Dashboard - Loan Request Info]: Borrower: {:?}", loan_request_nft_data.borrower);
            info!("[Borrower Dashboard - Loan Request Info]: Status: {:?}", loan_request_nft_data.status);
            info!("[Borrower Dashboard - Loan Request Info]: Loan NFT ID: {:?}", loan_request_nft_data.loan_nft_id);
            info!("[Borrower Dashboard - Loan Request Info]: Funding Locker Component Address: {:?}", loan_request_nft_data.funding_locker_address);
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
        /// * `borrower_proof` (Proof) - The Proof of the Borrower Admin Badge that belongs to this Borrower Dashboard.
        /// * `loan_request_nft_id` (NonFungibleId) - The NonFungibleId of the Loan Request NFT that was approved.
        /// * `collateral` (Bucket) - The Bucket that contains the collateral deposit.
        /// 
        /// # Returns:
        /// 
        /// * `Option<Bucket>` - The Bucket that contains the Loan NFT if the collateral ratio was met or none if the 
        /// collateral ratio was not met. 
        pub fn deposit_collateral(
            &mut self,
            borrower_proof: Proof,
            loan_request_nft_id: NonFungibleId,
            collateral: Bucket) -> Option<Bucket>
        {
            assert_eq!(borrower_proof.resource_address(), self.borrower_admin_address,
                "[Borrower Dashboard]: Incorrect borrower."
            );

            let loan_request_nft_data = self.get_resource_manager(&loan_request_nft_id);
            let loan_nft_id: NonFungibleId = loan_request_nft_data.loan_nft_id.unwrap();
            let optional_funding_locker: Option<&ComponentAddress> = self.funding_lockers.get(&loan_nft_id);
            match optional_funding_locker {
                Some (_funding_locker) => {
                    let funding_locker_address: ComponentAddress = *optional_funding_locker.unwrap();
                    let funding_locker: FundingLocker = funding_locker_address.into();
                    let loan_request_nft: Bucket = self.loan_requests.take_non_fungible(&loan_request_nft_id);
                    let loan_request_nft_proof: Proof = loan_request_nft.create_proof();
                    let option_bucket: Option<Bucket> = funding_locker.deposit_collateral(loan_request_nft_proof, collateral);
                    match option_bucket {
                        Some (bucket) => {
                            let return_bucket = Some(bucket);

                            self.loan_request_nft_admin.authorize(|| loan_request_nft.burn());

                            return_bucket
                        }
                        None => None
                    }
                }
                None => None
            }
        }

        // pub fn make_payment(
        //     &mut self,
        //     payment_amount: Decimal)
        // {

        // }

        // pub fn draw_loan(
        //     &mut self,
        //     loan_nft: Proof,
        //     amount: Decimal,
        // ) -> Bucket
        // {

        //     let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
        //     match optional_lending_pool {
        //         Some (lending_pool) => { // If it matches it means that the lending pool exists.
        //             let return_bucket: Bucket = lending_pool.repay(user_id, loan_id, token_requested, amount);
        //             let degen_token = self.degen_token_vault.take(1);
        //             (return_bucket, degen_token)
        //         }
        //         None => { 
        //             info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
        //             let empty_bucket1: Bucket = self.access_auth_vault.take(0);
        //             let empty_bucket2: Bucket = self.access_auth_vault.take(0);
        //             (empty_bucket1, empty_bucket2)
        //         }
        //     }
        // }
    }
}