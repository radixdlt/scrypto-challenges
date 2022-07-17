use scrypto::prelude::*;
use crate::user_management::*;
use crate::structs::{Loan, AuctionAuth};

blueprint! {
    struct LoanAuction {
        access_badge_vault: Vault,
        loan_nft_vault: Vault,
        collateral_vault: Vault,
        //Flash loan admin badge
        flash_loan_auth_vault: Vault,
        // Flash loan resource address
        flash_loan_address: ResourceAddress,
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
            // Creates badge to authorizie to mint/burn flash loan
            let flash_loan_token = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin authority for BasicFlashLoan")
                .metadata("symbol", "FLT")
                .metadata("description", "Admin authority to mint/burn flash loan tokens")
                .initial_supply(1);

            let list_of_resource = vec![access_badge.resource_address(), flash_loan_token.resource_address()];

            // Define a "transient" resource which can never be deposited once created, only burned
            let flash_loan_address = ResourceBuilder::new_non_fungible()
                .metadata(
                    "name",
                    "Promise token for BasicFlashLoan - must be returned to be burned!",
                )
                .mintable(rule!(require(flash_loan_token.resource_address())), LOCKED)
                .burnable(rule!(require(flash_loan_token.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require_any_of(list_of_resource)), LOCKED)
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
                flash_loan_auth_vault: Vault::with_bucket(flash_loan_token),
                flash_loan_address: flash_loan_address,
                collateral_requested: collateral_requested,
                loan_id: loan_id,
                user_id: user_id,
                user_sbt_address: user_sbt_address,
                user_management_address: Some(user_management_address),
            }
            .instantiate()
            .globalize()
        }

        pub fn withdraw_loan_nft(
            &mut self,
            user_id: NonFungibleId,
        ) -> (Bucket, Bucket)
        {
            let loan_id = self.loan_nft_vault.non_fungible::<Loan>();
            let amount_due = loan_id.data().remaining_balance;
            let collateral_requested = self.collateral_requested;
            let collateral_address = loan_id.data().collateral;
            let transient_token = self.flash_loan_auth_vault.authorize(|| {
                borrow_resource_manager!(self.flash_loan_address)
                .mint_non_fungible(
                    &NonFungibleId::random(),
                    AuctionAuth {
                        amount_due: amount_due,
                        collateral_due: collateral_requested,
                        collateral_address: collateral_address,
                    },
                )
            });

            let loan_nft = self.loan_nft_vault.take_non_fungible(&self.loan_id);

            let loan_address = loan_nft.resource_address();

            let mut loan_data = loan_nft.non_fungible::<Loan>().data();

            let remaining_balance = loan_data.remaining_balance;

            let token_address = loan_data.asset;

            let loan_id = loan_nft.non_fungible::<Loan>().id();

            let user_management: UserManagement = self.user_management_address.unwrap().into();

            // Logic to change ownership
            loan_data.owner = user_id.clone();

            let resource_manager = borrow_resource_manager!(loan_address);

            self.access_badge_vault.authorize(||
            resource_manager.update_non_fungible_data(&loan_id, loan_data));

            self.access_badge_vault.authorize(|| 
            user_management.insert_loan(user_id, token_address, loan_id.clone()));

            // Close the loan of previous owner.
            self.change_sbt_ownership(token_address, loan_id, remaining_balance);

            (loan_nft, transient_token)
        }

        fn change_sbt_ownership(
            &mut self,
            token_address: ResourceAddress,
            loan_id: NonFungibleId,
            amount: Decimal,
        )
        {
            let user_management: UserManagement = self.user_management_address.unwrap().into();
            self.access_badge_vault.authorize(|| 
            user_management.close_loan(self.user_id.clone(), token_address, loan_id));
            self.access_badge_vault.authorize(||
            user_management.decrease_borrow_balance(self.user_id.clone(), token_address, amount));
        }

        pub fn return_collateral(
            &mut self,
            collateral: Bucket,
            transient_token: Bucket,
        )
        {
            let transient_data = transient_token.non_fungible::<AuctionAuth>().data();
            let amount_due = transient_data.amount_due;

            assert_eq!(transient_token.resource_address(), self.flash_loan_address,
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

            self.flash_loan_auth_vault.authorize(||
            transient_token.burn());
        }

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
    }

}