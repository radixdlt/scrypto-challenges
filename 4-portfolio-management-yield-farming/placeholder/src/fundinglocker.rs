use scrypto::prelude::*;
use crate::structs::*;

blueprint! {
    struct FundingLocker {
        loan_funding_vault: Vault,
        loan_repay_vault: Vault,
        grace_period: u64,
        loan_nft_vault: Vault,
        loan_request_nft_id: NonFungibleId,
        loan_request_nft_address: ResourceAddress,
        lender_id: NonFungibleId,
        lender_address: ResourceAddress,
        collateral_vault: Vault,
        loan_nft_admin: Vault,
        loan_nft_id: NonFungibleId,
        loan_nft_address: ResourceAddress,
    }

    impl FundingLocker {

        pub fn new(
            loan_request_nft_id: NonFungibleId,
            loan_request_nft_address: ResourceAddress,
            loan_nft: Bucket,
            loan_nft_admin: Bucket,
        ) -> ComponentAddress 
        {
            let loan_nft_data = loan_nft.non_fungible::<Loan>().data();
            let lender_id = loan_nft_data.lender_id;
            let lender_address = loan_nft_data.lender_address;
            let loan_asset_address = loan_nft_data.asset;
            let loan_collateral_address = loan_nft_data.collateral;
            let loan_nft_id = loan_nft.non_fungible::<Loan>().id();
            let loan_nft_address = loan_nft.resource_address();

            return Self {
                loan_repay_vault: Vault::new(loan_asset_address),
                loan_funding_vault: Vault::new(loan_asset_address),
                grace_period: 10,
                loan_nft_vault: Vault::with_bucket(loan_nft),
                loan_request_nft_id: loan_request_nft_id,
                loan_request_nft_address: loan_request_nft_address,
                lender_id: lender_id,
                lender_address: lender_address,
                collateral_vault: Vault::new(loan_collateral_address),
                loan_nft_admin: Vault::with_bucket(loan_nft_admin),
                loan_nft_id: loan_nft_id,
                loan_nft_address: loan_nft_address,
            }

            .instantiate()
            .globalize();
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
            loan_nft_data: Loan)
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
            collateral: Bucket) ->  Option<Bucket>
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

        // pub fn draw_loan(
        //     &mut self) -> Bucket 
        // {
        //     let draw: Bucket = self.loan_funding_vault.take(1);
        //     draw
        // }

        // pub fn make_payment(
        //     &mut self,
        //     borrower: Proof,
        //     repay_amount: Bucket) 
        // {
        //     self.loan_repay_vault.put(repay_amount);
        // }

        // pub fn replenish(
        //     &mut self,
        //     lender: Proof,
        //     replenish_amount: Bucket)
        // {
        //     self.loan_funding_vault.put(replenish_amount);
        // }

        // pub fn close_loan(
        //     &mut self,
        //     lender: Proof,) -> Bucket
        // {
        //     let funds = self.loan_repay_vault.take_all();
        //     funds
        // }
    
    }
}