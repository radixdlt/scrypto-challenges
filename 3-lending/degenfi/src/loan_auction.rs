use scrypto::prelude::*;
use crate::user_management::*;
use crate::collateral_pool::*;
use crate::structs::{Loan, AuctionAuth};

blueprint! {
    /// The loan auction design has not been fully thought out yet so there are certainly a lot of outstanding questions to consider. 
    /// Nonetheless, the design requires the seller of the loan NFT to instantitate the `LoanAuction` blueprint to deposit their loan NFT. 
    /// The seller needs to determine the conditions of the sale (how much the minimum collateral is requested by the seller). The buyer is 
    /// then allowed to withdraw the loan NFT from the vault, but a transient token is minted along the way the must satisfy the conditions of 
    /// the sale before the loan NFT can be retrieved.
    /// The `LoanAuction` blueprint serves as a way for users to deposit their loan NFT to put up for sale. It contains four vaults: 
    /// 1. The vault where collateral will be deposited to be claimed by the seller of the NFT.
    /// 2. The vault where the NFT will be contained.
    /// 3. The vault of where the transient token badge that has authority to mint/burn/update transient tokens.
    /// 4. The vault that containse the access badge to allow components to do permissioned calls.
    //The `LoanAuction` blueprint has methods that faciliates the loan NFT transaction and the change of ownership of that loan NFT.
    
    struct LoanAuction {
        access_badge_vault: Vault,
        loan_nft_vault: Vault,
        collateral_vault: Vault,
        //Flash loan admin badge
        transient_token_auth_vault: Vault,
        // Flash loan resource address
        transient_token_address: ResourceAddress,
        collateral_requested: Decimal,
        loan_id: NonFungibleId,
        user_id: NonFungibleId,
        user_sbt_address: ResourceAddress,
        user_management_address: Option<ComponentAddress>,
    }

    impl LoanAuction {
        pub fn new(
            user_id: NonFungibleId,
            user_sbt_address: ResourceAddress,
            loan_nft: Bucket,
            collateral_requested: Decimal,
            access_badge: Bucket,
            user_management_address: ComponentAddress,
        ) -> ComponentAddress
        {
            let access_rules: AccessRules = AccessRules::new()
            .method("auction_repay", rule!(require(access_badge.resource_address())))
            .method("redeem_auction_collateral", rule!(require(access_badge.resource_address())))
            .default(rule!(allow_all));

            // Creates badge to authorizie to mint/burn flash loan
            let transient_token_token = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin authority for BasicFlashLoan")
                .metadata("symbol", "FLT")
                .metadata("description", "Admin authority to mint/burn flash loan tokens")
                .initial_supply(1);

            // Define a "transient" resource which can never be deposited once created, only burned
            let transient_token_address = ResourceBuilder::new_non_fungible()
                .metadata(
                    "name",
                    "Promise token for BasicFlashLoan - must be returned to be burned!",
                )
                .mintable(rule!(require(transient_token_token.resource_address())), LOCKED)
                .burnable(rule!(require(transient_token_token.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(transient_token_token.resource_address())), LOCKED)
                .restrict_deposit(rule!(deny_all), LOCKED)
                .no_initial_supply();

            let loan_id = loan_nft.non_fungible::<Loan>().id();
            let loan_address = loan_nft.resource_address();
            let resource_manager = borrow_resource_manager!(loan_address);
            let loan_data: Loan = resource_manager.get_non_fungible_data(&loan_id);
            let collateral_address = loan_data.collateral;

            return Self {
                access_badge_vault: Vault::with_bucket(access_badge),
                loan_nft_vault: Vault::with_bucket(loan_nft),
                collateral_vault: Vault::new(collateral_address),
                transient_token_auth_vault: Vault::with_bucket(transient_token_token),
                transient_token_address: transient_token_address,
                collateral_requested: collateral_requested,
                loan_id: loan_id,
                user_id: user_id,
                user_sbt_address: user_sbt_address,
                user_management_address: Some(user_management_address),
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize()
        }

        /// Allows a prospective buyer to purchase the loan NFT.
        ///
        /// This method is used to allow buyers to purchase the NFT contained within this vault.
        /// The logic contained within this method is mainly to set the conditions for the transient token
        /// that must be met before the transaction completes. It also faciliate the change of the loan NFT
        /// ownership.
        /// 
        /// This method does not perform any checks. 
        /// 
        /// # Arguments:
        /// 
        /// * `buyer_user_id` (NonFungibleId) - The SBT NonFungibleId of the buyer.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - Returns a bucket with the transient token.
        /// * `Bucket` - Returns the loan NFT to the buyer.
        pub fn withdraw_loan_nft(
            &mut self,
            buyer_user_id: NonFungibleId,
        ) -> (Bucket, Bucket)
        {
            // Retrieve loan data to be inserted to the transient token.
            let loan_id = self.loan_nft_vault.non_fungible::<Loan>();
            // Retrieve amount due.
            let amount_due = loan_id.data().remaining_balance;
            // Retrieve the amount requested.
            let collateral_requested = self.collateral_requested;
            // Retrieve collateral resource address.
            let collateral_address = loan_id.data().collateral;
            // Mint transient token.
            let transient_token = self.transient_token_auth_vault.authorize(|| {
                borrow_resource_manager!(self.transient_token_address)
                .mint_non_fungible(
                    &NonFungibleId::random(),
                    AuctionAuth {
                        amount_due: amount_due,
                        collateral_due: collateral_requested,
                        collateral_address: collateral_address,
                    },
                )
            });

            // Take loan NFT out of the vault.
            let loan_nft = self.loan_nft_vault.take_non_fungible(&self.loan_id);
            // Retrieve the resource address of the loan NFT.
            let loan_address = loan_nft.resource_address();
            // Retrieve loan NFT data.
            let mut loan_data = loan_nft.non_fungible::<Loan>().data();

            // Retrieve the resource address of the borrowed asset.
            let token_address = loan_data.asset;
            // Retrieve NFT ID
            let loan_id = loan_nft.non_fungible::<Loan>().id();
            // Retrieve User Management Component
            let user_management: UserManagement = self.user_management_address.unwrap().into();

            // Logic to change ownership of loan NFT from the old owner to the new owner.
            loan_data.owner = buyer_user_id.clone();
            // Retrieve resource manager.
            let resource_manager = borrow_resource_manager!(loan_address);
            // Authorize update of the loan NFT.
            self.access_badge_vault.authorize(||
            resource_manager.update_non_fungible_data(&loan_id, loan_data));
            // Authorize update that the loan is now inserted into buyer's SBT data.
            self.access_badge_vault.authorize(|| 
            user_management.insert_loan(buyer_user_id, token_address, loan_id.clone()));

            (loan_nft, transient_token)
        }

        /// Allows the buyer to return the requested collateral by the seller.
        /// 
        /// This method performs a number of checks before before the method is performed:
        /// 
        /// * **Check 1:** Checks that the transient token belongs to this blueprint.
        /// 
        /// * **Check 2:** Checks that the loan balance has been paid off.
        /// 
        /// * **Check 3:** Checks that the correct collateral amount has been returned.
        /// 
        /// * **Check 4:** Checks that the correct resource address has been passed.
        /// 
        /// # Arguments:
        /// 
        /// * `collateral` (Bucket) - The bucket that contains the collateral amount.
        /// * `transient_token` (Bucket) - The bucket that contains the transient token.
        /// 
        /// # Returns:
        /// 
        /// This method does not return any assets.
        pub fn return_collateral(
            &mut self,
            collateral: Bucket,
            transient_token: Bucket,
        )
        {
            let transient_data = transient_token.non_fungible::<AuctionAuth>().data();
            let amount_due = transient_data.amount_due;

            assert_eq!(transient_token.resource_address(), self.transient_token_address,
            "The transient token passed is not correct.");

            assert_eq!(amount_due, Decimal::zero(), 
            "Must pay off the loan balance. Amount due: {:?}", 
            amount_due);

            let collateral_requested = transient_data.collateral_due;

            assert_eq!(collateral.amount(), collateral_requested, 
            "Must fulfill the requested collateral amount. The requested amount is: {:?}", 
            collateral_requested);

            let collateral_address = transient_data.collateral_address;

            assert_eq!(collateral.resource_address(), collateral_address,
            "Must pass the correct collateral resource. The correct collateral resource is {:?}",
            collateral_address);

            self.collateral_vault.put(collateral);

            self.transient_token_auth_vault.authorize(||
            transient_token.burn());
        }

        /// Allows the seller to claim the collateral owed. 
        /// 
        /// This method performs a number of checks before before the method is performed:
        /// 
        /// * **Check 1:** Checks that the caller is authorized to claim the collateral
        /// 
        /// # Arguments:
        /// 
        /// * `user_id` (NonFungibleId) - The NonFungibleId to identify the seller.
        /// 
        /// # Returns:
        /// 
        /// `Bucket` - The bucket that contains the collateral.
        pub fn claim_collateral(
            &mut self,
            user_id: NonFungibleId,
        ) -> Bucket
        {
            assert_eq!(user_id, self.user_id,
            "You are unauthorize to access this vault.");

            let claim_amount = self.collateral_vault.take_all();

            claim_amount
        }

        /// Authorizes the update of the transient token.
        pub fn authorize_update(
            &mut self,
            transient_token_id: NonFungibleId,
            transient_token_data: AuctionAuth,
        )
        {
            let resource_manager = borrow_resource_manager!(self.transient_token_address);
            self.transient_token_auth_vault.authorize(|| 
                resource_manager.update_non_fungible_data(&transient_token_id, transient_token_data));

        }

        /// Allows the buyer to redeem the seller's collateral.
        /// 
        /// This method performs a number of checks before before the method is performed:
        /// 
        /// * **Check 1:** Checks that correct resource has been passed.
        /// 
        /// # Arguments:
        /// 
        /// * `collateral_pool` (CollateralPool) - The CollateralPool component for the method to call on.
        /// * `collateral_address` (ResourceAddress) - The resource address of the collateral.
        /// * `redeem_amount` (Decimal) - The amount of collateral to be redeemed.
        /// 
        /// # Returns:
        /// 
        /// `Bucket` - The bucket that contains the collateral.
        pub fn redeem_auction_collateral(
            &mut self,
            collateral_pool: CollateralPool,
            collateral_address: ResourceAddress,
            redeem_amount: Decimal,
        ) -> Bucket
        {
            assert_eq!(collateral_address,  self.collateral_vault.resource_address(), 
                "Wrong collateral address."
            );
            let collateral_pool: CollateralPool = collateral_pool.into();
            let original_owner_user_id = self.user_id.clone();
            let collateral = self.access_badge_vault.authorize(|| 
                collateral_pool.redeem(
                    original_owner_user_id, 
                    collateral_address, 
                    redeem_amount
                )
            );

            collateral
        }

        /// Retrieves the seller's NonFungibleId.
        pub fn get_owner_original_user_id(
            &self,
        ) -> NonFungibleId
        {
            return self.user_id.clone()
        }
    }
}

