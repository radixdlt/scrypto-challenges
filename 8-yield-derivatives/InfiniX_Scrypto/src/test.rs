use scrypto::prelude::*;

// Import the SecurityGovernance contract
use crate::security::SecurityGovernance;

#[blueprint]
mod parametric_insurance {
    // Define the struct for the ParametricInsurance contract
    struct ParametricInsurance {
        // Define the parameters for the insurance contract
        insured_domain: String,
        premium_amount: u64,
        payout_amount: u64,
        is_claimed: bool,
        // Add a field for SecurityGovernance contract
        security_governance: SecurityGovernance,
    }

    // Implement methods for the ParametricInsurance contract
    impl ParametricInsurance {
        // Constructor function to create a new instance of ParametricInsurance
        pub fn new(
            insured_domain: String,
            premium_amount: u64,
            payout_amount: u64,
            security_governance: SecurityGovernance,
        ) -> Global<ParametricInsurance> {
            Self {
                insured_domain,
                premium_amount,
                payout_amount,
                is_claimed: false, // Initially, no claim has been made
                security_governance, // Initialize SecurityGovernance contract
            }
            .instantiate() // Instantiate the contract
            .prepare_to_globalize(OwnerRole::None) // Prepare for globalization
            .globalize() // Globalize the contract
        }

        // Function to check if the insurance contract has been claimed
        pub fn is_claimed(&self) -> bool {
            self.is_claimed
        }

        // Function to make a claim on the insurance contract
        pub fn make_claim(&mut self) -> Result<Bucket, String> {
            // Check if the claim can be made based on security governance rules
            // For example, you might check if the caller is allowed to make a claim
            // using a function from the SecurityGovernance contract
            if self.security_governance.can_make_claim() {
                if self.is_claimed {
                    Err("Claim already made".to_string())
                } else {
                    // Set is_claimed to true to prevent multiple claims
                    self.is_claimed = true;
                    // Create a payout bucket containing the payout_amount
                    let payout_bucket: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                        .metadata(metadata! {
                            init {
                                "name" => "InsurancePayout", locked;
                                "description" => "Payout for insurance claim", locked;
                            }
                        })
                        .mint_initial_supply(self.payout_amount)
                        .into();
                    Ok(payout_bucket)
                }
            } else {
                Err("Not authorized to make a claim".to_string())
            }
        }

        // Function to cancel the insurance contract
        pub fn cancel_contract(&mut self) {
            // Clear the insured_domain to prevent future claims
            self.insured_domain.clear();
            // Set is_claimed to true to prevent future claims
            self.is_claimed = true;
        }
    }
}
