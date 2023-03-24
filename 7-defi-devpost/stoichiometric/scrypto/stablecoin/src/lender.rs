use scrypto::{blueprint, external_component};

external_component! {
    OracleComponent {
        fn get_twap_since(&self, token: ResourceAddress, timestamp: i64) -> Decimal;
        fn new_observation(&mut self, token: ResourceAddress);
    }
}

#[blueprint]
mod lender {
    use crate::constants::SECONDS_PER_DAY;
    use crate::loan::Loan;

    pub struct Lender {
        collateral: Vault,
        loan_to_value: Decimal,
        interest_rate: Decimal,
        liquidation_threshold: Decimal,
        liquidation_penalty: Decimal,
        oracle: ComponentAddress,
    }

    impl Lender {
        pub fn new(
            collateral_address: ResourceAddress,
            loan_to_value: Decimal,
            interest_rate: Decimal,
            liquidation_threshold: Decimal,
            liquidation_penalty: Decimal,
            oracle: ComponentAddress,
        ) -> LenderComponent {
            assert!(
                loan_to_value.is_positive() && loan_to_value < Decimal::ONE,
                "LTV should be such that 0<LTV<1"
            );
            assert!(
                interest_rate.is_positive() && interest_rate < Decimal::ONE,
                "The daily interest rate should be such that 0<DIR<1"
            );
            assert!(
                liquidation_threshold > Decimal::ONE,
                "The liquidation threshold should be greater than one"
            );
            assert!(
                liquidation_threshold * loan_to_value < Decimal::ONE,
                "The LTV-liquidation threshold product should be smaller than one"
            );
            assert!(
                liquidation_penalty.is_positive(),
                "The liquidation incentive should be positive"
            );

            Self {
                collateral: Vault::new(collateral_address),
                loan_to_value,
                interest_rate,
                liquidation_threshold,
                liquidation_penalty,
                oracle,
            }
            .instantiate()
        }

        pub fn take_loan(&mut self, collateral: Bucket, amount_to_loan: Decimal) -> Loan {
            let price = self.get_oracle_price();

            let collateral_needed = amount_to_loan / (self.loan_to_value * price);

            assert!(
                collateral.amount() >= collateral_needed,
                "You need to provide at least {} tokens to loan {}",
                collateral_needed,
                amount_to_loan
            );

            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let loan = Loan::from(
                collateral.resource_address(),
                collateral.amount(),
                amount_to_loan,
                current_time,
                self.interest_rate,
            );
            self.collateral.put(collateral);
            loan
        }

        pub fn repay_loan(&mut self, repayment: Decimal, loan: Loan) -> (Decimal, Bucket) {
            let interests = self.compute_interests(&loan);
            assert!(
                repayment >= loan.amount_lent + interests,
                "You need to provide {} stablecoins to repay your loan",
                loan.amount_lent + interests
            );

            let collateral = self.collateral.take(loan.collateral_amount);

            (interests, collateral)
        }

        pub fn add_collateral(&mut self, collateral: Bucket, mut loan: Loan) -> Loan {
            assert!(
                loan.collateral_token == collateral.resource_address(),
                "Please provide the right tokens to add as collateral"
            );

            loan.collateral_amount = loan.collateral_amount + collateral.amount();
            self.collateral.put(collateral);
            loan
        }

        pub fn remove_collateral(&mut self, amount: Decimal, mut loan: Loan) -> (Loan, Bucket) {
            let new_collateral_amount = loan.collateral_amount - amount;
            let collateral_price = self.get_oracle_price();
            let interests = self.compute_interests(&loan);

            // Check that after removing collateral, the (collateral value)/(loan value) is still
            // greater than the liquidation threshold

            assert!(
                new_collateral_amount * collateral_price / (loan.amount_lent + interests)
                    >= self.liquidation_threshold,
                "Cannot remove {} because it would make the loan liquidatable",
                amount
            );

            loan.collateral_amount = new_collateral_amount;
            (loan, self.collateral.take(amount))
        }

        pub fn liquidate(
            &mut self,
            stabelcoin_input: Decimal,
            mut loan: Loan,
        ) -> (Decimal, Bucket, Option<Bucket>, Loan) {
            // First check that the loan can indeed be liquidated
            let accrued_interests = self.compute_interests(&loan);
            let total_lent = loan.amount_lent + accrued_interests;
            let collateral_price = self.get_oracle_price();
            let collateralization_ratio = loan.collateral_amount * collateral_price / total_lent;

            assert!(
                collateralization_ratio <= self.liquidation_threshold,
                "Cannot liquidate this loan: the collateralization ratio is {} >= {}",
                collateralization_ratio,
                self.liquidation_threshold
            );

            // If the previous assert worked, then it means that the loan can be partially or fully liquidated

            // In the case where the collateralization ratio is smaller than 1, we liquidate everything
            // Note that there is not necessarily bad debt because without counting interests, the ratio could be > 1
            if collateralization_ratio < Decimal::ONE {
                // In this case, we fully liquidate the loan and only take the interests that can be
                // paid

                assert!(
                    stabelcoin_input >= loan.amount_lent,
                    "Please provide at least {} SUSD to liquidate this loan",
                    loan.amount_lent
                );

                // We only claim 10% of the interests that can be claimed
                let real_interests = dec!("0.1")
                    * accrued_interests
                        .min(collateral_price * loan.collateral_amount - loan.amount_lent);
                let stablecoin_interest = real_interests / collateral_price;
                let liquidator_amount = loan.collateral_amount - stablecoin_interest;

                let liquidator_share = self.collateral.take(liquidator_amount);
                let protocol_share = self.collateral.take(stablecoin_interest);
                let stablecoins_needed = loan.amount_lent;

                loan.amount_lent = Decimal::ZERO;
                loan.collateral_amount = Decimal::ZERO;

                (
                    stablecoins_needed,
                    liquidator_share,
                    Some(protocol_share),
                    loan,
                )
            } else {
                //In the other case, we compute the maximum amount that can be liquidated
                let virtual_collateral =
                    (Decimal::ONE - self.liquidation_penalty) * loan.collateral_amount;
                let virtual_collateralization_ratio =
                    virtual_collateral * collateral_price / total_lent;
                let max_input = total_lent
                    * (Decimal::ONE - virtual_collateralization_ratio / self.liquidation_threshold)
                        .sqrt()
                        .unwrap();

                let actual_input = max_input.min(stabelcoin_input);
                let collateral_output =
                    loan.collateral_amount * actual_input / (total_lent + actual_input);

                let new_amount_lent = loan.amount_lent - actual_input;
                let new_collateral_amount = loan.collateral_amount - collateral_output;

                loan.amount_lent = new_amount_lent;
                loan.collateral_amount = new_collateral_amount;

                let liquidator_bucket = self.collateral.take(collateral_output);
                (actual_input, liquidator_bucket, None, loan)
            }
        }

        pub fn clear_bad_debt(&mut self, mut loan: Loan) -> (Decimal, Bucket, Loan) {
            // Check that there is indeed bad debt
            let collateral_price = self.get_oracle_price();
            let collateral_value = loan.collateral_amount * collateral_price;

            assert!(
                collateral_value < loan.amount_lent,
                "There is no bad debt to clear!"
            );

            // If there is bad debt, fully liquidate the loan
            let collateral = self.collateral.take(loan.collateral_amount);
            let amount_to_clear = loan.amount_lent;
            loan.collateral_amount = Decimal::ZERO;
            loan.amount_lent = Decimal::ZERO;

            (amount_to_clear, collateral, loan)
        }

        pub fn change_parameters(
            &mut self,
            loan_to_value: Decimal,
            interest_rate: Decimal,
            liquidation_threshold: Decimal,
            liquidation_penalty: Decimal,
        ) {
            self.loan_to_value = loan_to_value;
            self.interest_rate = interest_rate;
            self.liquidation_threshold = liquidation_threshold;
            self.liquidation_penalty = liquidation_penalty;
        }

        pub fn change_oracle(&mut self, oracle: ComponentAddress) {
            self.oracle = oracle;
        }

        pub fn get_state(&self) -> Vec<Decimal> {
            vec![
                self.collateral.amount(),
                self.loan_to_value,
                self.interest_rate,
                self.liquidation_threshold,
                self.liquidation_penalty,
            ]
        }

        fn get_oracle_price(&self) -> Decimal {
            let mut oracle = OracleComponent::at(self.oracle);

            // Look at the TWAP with as much data as possible
            let price = oracle.get_twap_since(self.collateral.resource_address(), 0);

            // Make a new observation to have more input to the oracle for future requests
            oracle.new_observation(self.collateral.resource_address());

            price
        }

        fn compute_interests(&self, loan: &Loan) -> Decimal {
            let current_time = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let loan_days_duration = Decimal::from(current_time - loan.loan_date) / SECONDS_PER_DAY;

            loan.amount_lent * loan_days_duration * loan.interest_rate
        }
    }
}
