use scrypto::prelude::*;
use std::collections::HashMap;

#[blueprint]
mod scryptlend {
    struct Scryptlend {
        loan_name: String,
        lenders: HashMap<String, Decimal>,
        collected_xrd: Vault,
        loan_amount: Decimal,
        /// The interest rate of deposits, per epoch
        deposit_interest_rate: Decimal,
        /// The (stable) interest rate of loans, per epoch
        borrow_interest_rate: Decimal,
        /// min collateral ratio to be maintained
        min_collateral_ratio: Decimal,
        //Loan tenure in months
        time_period: Decimal,
    }

    impl Scryptlend {
        pub fn new(
            loan_amount: Decimal,
            loan_name: String,
            borrow_interest_rate: Decimal,
            deposit_interest_rate: Decimal,
            min_collateral_ratio: Decimal,
            time_period: Decimal,
        ) -> (ComponentAddress, Bucket) {
            //Creates loan seeker
            let seeker_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Loan Seeker Badge")
                .metadata("symbol", "LSeeker")
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            //seeker has access to "withdraw" the funds
            let access_rules: AccessRules = AccessRules::new()
                .method(
                    "withdraw",
                    rule!(require(seeker_badge.resource_address())),
                    LOCKED,
                )
                .default(rule!(allow_all), LOCKED);

            let mut loan_component: ScryptlendComponent = Self {
                loan_name: loan_name,
                lenders: HashMap::default(),
                collected_xrd: Vault::new(RADIX_TOKEN),
                loan_amount: loan_amount,
                borrow_interest_rate: borrow_interest_rate,
                deposit_interest_rate: deposit_interest_rate,
                min_collateral_ratio: min_collateral_ratio,
                time_period: time_period,
            }
            .instantiate();

            loan_component.add_access_check(access_rules);
            let loan_component_address: ComponentAddress = loan_component.globalize();
            return (loan_component_address, seeker_badge);
        }

        pub fn lend(&mut self, funds: Bucket, lender: String) -> Decimal {
            let new_loan_amount = funds.amount();
            self.collected_xrd.put(funds);
            self.lenders.insert(lender, new_loan_amount);
            self.collected_xrd.amount()
        }

        pub fn withdraw(&mut self) -> Bucket {
            if self.collected_xrd.amount() >= self.loan_amount {
                self.collected_xrd.take_all()
            } else {
                error!("LOAN AMOUNT NOT MET");
                self.collected_xrd.take(0)
            }
        }

        /// Returns the deposit interest rate per epoch
        pub fn set_deposit_interest_rate(&mut self, rate: Decimal) {
            self.deposit_interest_rate = rate;
        }

        /// Returns the borrow interest rate per epoch
        pub fn set_borrow_interest_rate(&mut self, rate: Decimal) {
            self.borrow_interest_rate = rate;
        }
    }
}
