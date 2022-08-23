use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct LoanDue {
    pub amount_due: Decimal,
}

blueprint! {
    struct FlashLoan {
        loan_vault: Vault,
        auth_vault: Vault,
        transient_resource_address: ResourceAddress,
    }

    impl FlashLoan {
        pub fn new(initial_liquidity: Bucket) -> ComponentAddress {

            let auth_token = ResourceBuilder::new_fungible()
                .metadata("name", "Flash Loan Auth")
                .initial_supply(1);

            // Define a "transient" resource which can never be deposited once created.
            let address = ResourceBuilder::new_non_fungible()
                .metadata(
                    "name",
                    "Promise token for BasicFlashLoan - must be returned to be burned!",
                )
                .mintable(rule!(require(auth_token.resource_address())), LOCKED)
                .burnable(rule!(require(auth_token.resource_address())), LOCKED)
                .restrict_deposit(rule!(deny_all), LOCKED)
                .no_initial_supply();

            Self {
                loan_vault: Vault::with_bucket(initial_liquidity),
                auth_vault: Vault::with_bucket(auth_token),
                transient_resource_address: address,
            }
            .instantiate()
            .globalize()
        }

        pub fn take_loan(&mut self, loan_amount: Decimal) -> (Bucket, Bucket) {
            assert!(
                loan_amount <= self.loan_vault.amount(),
                "Not enough liquidity to supply this loan!"
            );
            let amount_due = loan_amount * dec!("1.001");

            let loan_terms = self.auth_vault.authorize(|| {
                borrow_resource_manager!(self.transient_resource_address).mint_non_fungible(
                    &NonFungibleId::random(),
                    LoanDue {
                        amount_due: amount_due,
                    },
                )
            });
            (self.loan_vault.take(loan_amount), loan_terms)
        }

        pub fn repay_loan(&mut self, loan_repayment: Bucket, loan_terms: Bucket) {
            assert!(
                loan_terms.resource_address() == self.transient_resource_address,
                "Incorrect resource passed in for loan terms"
            );

            let terms: LoanDue = loan_terms.non_fungible().data();
            assert!(
                loan_repayment.amount() >= terms.amount_due,
                "Insufficient repayment given for your loan!"
            );

            self.loan_vault.put(loan_repayment);
            self.auth_vault.authorize(|| loan_terms.burn());
        }
    }
}