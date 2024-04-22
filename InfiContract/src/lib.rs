use scrypto::prelude::*;

#[blueprint]
mod parametric_insurance {
    // Define the struct for the ParametricInsurance contract
    struct ParametricInsurance {
        // Define the parameters for the insurance contract
        insured_domain: String,
        premium_amount: u64,
        payout_amount: u64,
        is_claimed: bool,
        risk_level: u8, // Use u8 to represent risk levels (0 for Low, 1 for Medium, 2 for High)
    }

    // Implement methods for the ParametricInsurance contract
    impl ParametricInsurance {
        // Constructor function to create a new instance of ParametricInsurance
        pub fn new(insured_domain: String, premium_amount: u64, payout_amount: u64, risk_level: u8) -> Global<ParametricInsurance> {
            Self {
                insured_domain,
                premium_amount,
                payout_amount,
                is_claimed: false, // Initially, no claim has been made
                risk_level,
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
            if self.is_claimed {
                return Err("Claim already made".to_string());
            }

            // Adjust claim algorithm based on risk level
            let payout: u64 = match self.risk_level {
                0 => self.payout_amount * 20 / 100, // Low risk level
                1 => self.payout_amount * 50 / 100, // Medium risk level
                2 => self.payout_amount * 80 / 100, // High risk level
                _ => return Err("Invalid risk level".to_string()), // Handle invalid risk level
            };

            // Create a payout bucket containing the calculated payout
            let payout_bucket: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "InsurancePayout", locked;
                        "description" => "Payout for insurance claim", locked;
                    }
                })
                .mint_initial_supply(payout)
                .into();

            // Set is_claimed to true to prevent multiple claims
            self.is_claimed = true;
            Ok(payout_bucket)
        }

        // Function to cancel the insurance contract
        pub fn cancel_contract(&mut self) {
            // Clear the insured_domain to prevent future claims
            self.insured_domain.clear();
            // Set is_claimed to true to prevent future claims
            self.is_claimed = true;
        }

        // Function to trigger insurance payout based on predefined conditions
        pub fn trigger_payout(&mut self, condition_met: bool) -> Result<Bucket, String> {
            if condition_met {
                // Trigger the insurance payout
                self.make_claim()
            } else {
                Err("Condition not met for payout".to_string())
            }
        }

        // Function to monitor liquidity and trigger insurance payout if a significant drop is detected
        pub fn monitor_liquidity(&mut self, liquidity_drop_threshold: u64, time_frame_hours: u64, current_liquidity: u64) -> Result<Bucket, String> {
            // Calculate the previous liquidity (for example, by querying an oracle)
            let previous_liquidity = 100000; // Example previous liquidity value

            // Calculate the percentage drop in liquidity
            let percentage_drop = ((previous_liquidity - current_liquidity) * 100) / previous_liquidity;

            // Check if the drop exceeds the threshold within the specified time frame
            if percentage_drop >= liquidity_drop_threshold && time_frame_hours <= 24 {
                // Trigger the insurance payout
                self.make_claim()
            } else {
                Err("No significant drop in liquidity detected".to_string())
            }
        }

        // Function to monitor market volatility and trigger insurance payout if volatility exceeds threshold
        pub fn monitor_market_volatility(&mut self, volatility_threshold: u64, current_volatility: u64) -> Result<Bucket, String> {
            if current_volatility > volatility_threshold {
                // Trigger the insurance payout
                self.make_claim()
            } else {
                Err("Volatility is within acceptable range".to_string())
            }
        }

        // Function to update the premium amount of the insurance contract
        pub fn update_premium_amount(&mut self, new_premium_amount: u64) {
            self.premium_amount = new_premium_amount;
        }

        // Function to update the payout amount of the insurance contract
        pub fn update_payout_amount(&mut self, new_payout_amount: u64) {
            self.payout_amount = new_payout_amount;
        }

        // Function to check the current premium amount of the insurance contract
        pub fn get_premium_amount(&self) -> u64 {
            self.premium_amount
        }

        // Function to check the current payout amount of the insurance contract
        pub fn get_payout_amount(&self) -> u64 {
            self.payout_amount
        }

        // Function to check the insured domain of the insurance contract
        pub fn get_insured_domain(&self) -> String {
            self.insured_domain.clone()
        }

        // Function to automate claim processing if conditions are met
        pub fn automate_claim_processing(&mut self, auto_claim_condition: bool) -> Result<Bucket, String> {
            if auto_claim_condition {
                self.make_claim()
            } else {
                Err("Automatic claim processing condition not met".to_string())
            }
        }
    }
}
