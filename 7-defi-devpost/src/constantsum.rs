use scrypto::radix_engine_interface::time::*;
use scrypto::prelude::*;
  
  // importing a radiswap method that needs to be used in this blueprint
external_component! {
    RadiswapComponentTarget {
        // Imported method
        fn swap(&mut self, input_tokens: Bucket) -> Bucket;
    }
}

#[blueprint]
mod amm_implementation {
    struct ConstantSumAmm {
        // Vault for storing token_a
        token_a_vault: Vault,
        // Vault for storing token_b
        token_b_vault: Vault,
        // Vault for storing collateral claim token a
        cct_a: Vault,
        // Vault for storing collateral claim token a
        cct_b: Vault,
        // Vault for storing bonded_token per second
        bonded_token_vault: Vault,
        // Maturity of the pool
        duration: i64,
        // Vault for storing LP admin badge
        admin_badge_vault: Vault,
        // Vault where lend badge is stored
        lend_vault: Vault,
        // Vault where token_a from options are stored
        option_token_a_vault: Vault,
        // Vault where token_b from options are stored
        option_token_b_vault: Vault,
        // resource address of LP token 
        lp_resource_address: ResourceAddress,
        //strike rate of the pool [Check docs to understand strike rate]
        strike_rate: Decimal,
        //constant_product of the pool
        constant_product: Decimal,
        // Interest to be recieved from the liquidity pool
        interest: Decimal,
        // Component Address of Radiswap
        amm_address: ComponentAddress,
        fee: Decimal
    }

    impl ConstantSumAmm {
        // Locking a token and minting collateral claim tokens and bond tokens
        pub fn locking_liquidity(token_a: Bucket, token_a_name: String, token_a_symbol: String, token_b: Bucket,
        token_b_name: String, token_b_symbol: String, duration: i64, required_interest: Decimal, strike_rate: Decimal, 
        lp_name: String, lp_symbol: String, fee: Decimal, amm_address: ComponentAddress) -> (ComponentAddress, Bucket, Bucket, Bucket) {   
            // Checking whether the ratio in which tokens are provided are correct 
            assert!(token_a.amount() / token_b.amount() == dec!(1) / strike_rate, "Tokens provided in the wrong ratio");

            // Mint badge is used for doing privilaged actions like minting and burning cctokens, bonded tokens, LP tokens, etc
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("Name", "LP Mint Badge")
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);
            // Mint badge access rule
            let admin_badge_rule: AccessRule = rule!(require(admin_badge.resource_address()));

            // Badge
            let lend_address: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("Name", "Lend Badge")
                .metadata("Usage", "Verifying a lender")
                .mintable(admin_badge_rule.clone(), LOCKED)
                .burnable(admin_badge_rule.clone(), LOCKED)
                .divisibility(DIVISIBILITY_NONE)
                .create_with_no_initial_supply();

            // Resource address of LP token
            let lp_resource_address = ResourceBuilder::new_fungible()
                .metadata("Name", lp_name)
                .metadata("Symbol", lp_symbol)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .mintable(admin_badge_rule.clone(), LOCKED)
                .burnable(admin_badge_rule.clone(), LOCKED)
                .create_with_no_initial_supply();

            // Collateral-claim-token of token_a
            let cctoken_a: Bucket = ResourceBuilder::new_fungible()
                .metadata("Name", token_a_name)
                .metadata("Symbol", token_a_symbol)
                .mintable(admin_badge_rule.clone(), LOCKED)
                .burnable(admin_badge_rule.clone(), LOCKED)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .mint_initial_supply(token_a.amount());
    
            // Collateral-claim-token of token_b
            let cctoken_b: Bucket = ResourceBuilder::new_fungible()
                .metadata("Name", token_b_name)
                .metadata("Symbol", token_b_symbol)
                .mintable(admin_badge_rule.clone(), LOCKED)
                .burnable(admin_badge_rule.clone(), LOCKED)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .mint_initial_supply(token_b.amount());
                
            // Bonded token    
            let bonded_token: Bucket = ResourceBuilder::new_fungible()
                .metadata("Name", "Bond Token")
                .metadata("Symbol", "BT")
                .mintable(admin_badge_rule.clone(), LOCKED)
                .burnable(admin_badge_rule.clone(), LOCKED)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .mint_initial_supply(required_interest * (cctoken_a.amount() + (cctoken_b.amount() * strike_rate)));

            // Checking whether the minted collateral claim and bond tokens are in correct ratio
            assert!(bonded_token.amount() / cctoken_a.amount() + cctoken_b.amount() == required_interest, "Token ratios are wrong");

            // Interest to be recieved from liquidity pool
            let interest: Decimal = dec!(0);

            // Constant product of the AMM
            let constant_product: Decimal = dec!(0);
    
            // Maturity of the liquidity pool, after maturity the transaction will fail
            assert!(Clock::current_time_is_at_or_before(Instant::new(duration), TimePrecision::Minute), "Maturity of the pool is over");
    

            let amm_implementation = Self {
                option_token_a_vault: Vault::new(token_a.resource_address()),
                option_token_b_vault: Vault::new(token_b.resource_address()),
                token_a_vault: Vault::with_bucket(token_a),
                token_b_vault: Vault::with_bucket(token_b),
                cct_a: Vault::new(cctoken_a.resource_address()),
                cct_b: Vault::new(cctoken_b.resource_address()),
                bonded_token_vault: Vault::new(bonded_token.resource_address()),
                duration,
                lp_resource_address,
                admin_badge_vault: Vault::with_bucket(admin_badge),
                lend_vault: Vault::new(lend_address),
                strike_rate,
                interest,
                amm_address,
                constant_product,
                fee
            }
            .instantiate()
            .globalize();
            
            // Returning Component Address, collateral claim tokens and bonded tokens minted
            return (amm_implementation, cctoken_a, cctoken_b, bonded_token)
        }

        // Method to be called when spot_price is lesser than strike_rate, here you deposit cctoken_a and bonded_token   
        pub fn deposit_liquidity_a(&mut self, cctoken_a: Bucket, bonded_token: Bucket, strike_price: Decimal, duration: i64) -> Bucket {
            // Checking whether duration and strike rate provided are correct
            assert!(duration == self.duration, "Maturity provided is wrong");
            assert!(strike_price == self.strike_rate, "Wrong strike rate");
        
            // Checking whether collateral claim tokens and bond tokens provided are correct 
            assert!(cctoken_a.resource_address() == self.cct_a.resource_address() || bonded_token.resource_address() ==
            self.bonded_token_vault.resource_address(), "Wrong collateral claim or bond token provided");

            // Checking whether collateral claim tokens and bond tokens provided are empty 
            assert!(!cctoken_a.is_empty() && !bonded_token.is_empty(), "Empty tokens provided");

            // Checking whether collateral claim tokens and bond tokens are provided in the correct ratio
            assert!(bonded_token.amount() / cctoken_a.amount() == self.interest, "Ratio of the tokens provided are wrong");

            // Setting up the interest rate
            self.interest += (bonded_token.amount() / Decimal::from(duration)) / cctoken_a.amount();

            // Adding current constant product to the actual constant product
            let delta_constant_product = cctoken_a.amount() * (bonded_token.amount() / Decimal::from(duration));
            self.constant_product += delta_constant_product;

            // Minting LP tokens
            let lp_token: Bucket = self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.lp_resource_address).mint(delta_constant_product.powi(1/2))
            });

            // Sending collateral claim tokens and bonded tokens to the liquidity pool
            self.cct_a.put(cctoken_a);
            self.bonded_token_vault.put(bonded_token);

            // Returning LP tokens for the user to withdraw
            return lp_token;
        }

        // Method to be called when spot_price is greater than strike_rate, here you deposit cctoken_b and bonded_token
        pub fn deposit_liquidity_b(&mut self, cctoken_b: Bucket, bonded_token: Bucket, 
        strike_price: Decimal, duration: i64) -> Bucket {
            // Checking whether maturity provided is wrong
            assert!(duration == self.duration, "Maturity provided is wrong");

            // Checking whether strike price is wrong
            assert!(strike_price == self.strike_rate, "Wrong strike rate");
            
            // Checking whether tokens provided are correct
            assert!(cctoken_b.resource_address() == self.cct_b.resource_address() || bonded_token.resource_address() ==
            self.bonded_token_vault.resource_address(), "Wrong collateral claim or bond token provided");
    
            assert!(!cctoken_b.is_empty() && !bonded_token.is_empty(), "Empty tokens provided");
            
            // Checking the ratio of the tokens provided
            assert!(bonded_token.amount() / (cctoken_b.amount() / strike_price) == self.interest, "Ratio of the tokens provided are wrong");

            // Making changes in interest rate because of the changes in liquidity pool
            self.interest += (bonded_token.amount() / Decimal::from(duration)) / (cctoken_b.amount() / strike_price);
    
            // Making changes in constant product because of the changes in liquidity pool
            let delta_constant_product = (cctoken_b.amount() / strike_price) * (bonded_token.amount() / Decimal::from(duration));
            self.constant_product += delta_constant_product;
    
            // Liquidity Pool token
             let lp_token: Bucket = self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.lp_resource_address).mint(delta_constant_product.powi(1/2))
            });

            // Putting collateral claim tokens and bonded tokens to the liquidity pool
            self.cct_b.put(cctoken_b);
            self.bonded_token_vault.put(bonded_token);
    
            return lp_token;
        }

         
        // This method to withdraw liquidity
        pub fn withdraw_liquidity(&mut self, lp_token: Bucket, strike_price: Decimal, duration: i64) -> (Bucket, Bucket, Bucket)  {
            // Checking whether the provided tokens are empty or wrong
            assert!(!lp_token.is_empty(), "No LP tokens provided");
            assert!(lp_token.resource_address() == self.lp_resource_address, "Wrong LP token provided");

            // Checking whether strike rate or duration is wrong
            assert!(duration == self.duration, "Maturity provided is wrong");
            assert!(strike_price == self.strike_rate, "Wrong strike rate");

            let lp_manager = borrow_resource_manager!(self.lp_resource_address);

            // Share of the total liquidity pool supply
            let share = lp_token.amount() / lp_manager.total_supply();

            // Calculating and updating the constant product of liquidity pool
            self.constant_product = ((self.cct_a.amount() - (self.cct_a.amount() * share)) +
            ((self.cct_b.amount() / strike_price) - ((self.cct_b.amount() / strike_price) * share))) *
            ((self.bonded_token_vault.amount() / Decimal::from(duration)) - (self.bonded_token_vault.amount() / Decimal::from(duration)) * share);

            // Calculating and updating the interest rate of liquidity pool
            self.interest = ((self.bonded_token_vault.amount() / Decimal::from(duration)) - (self.bonded_token_vault.amount() / Decimal::from(duration)) * share) /
            (self.cct_a.amount() - (self.cct_a.amount() * share)) + ((self.cct_b.amount() / strike_price) - ((self.cct_b.amount() / strike_price) * share));

            // Burning LP tokens
            self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.lp_resource_address).burn(lp_token)
            });

            // Giving back the collateral claim tokens and bonded tokens from the liquidity pool
            (
                self.cct_a.take(self.cct_a.amount() * share),
                self.cct_b.take(self.cct_b.amount() * share),
                self.bonded_token_vault.take(self.bonded_token_vault.amount() * share)
            )
        }  

        // This is a transaction that is done by arbitraguers when there is more than one type of token in the pool 
        pub fn rebalance_transaction(&mut self, collateral: Bucket) -> Bucket {
            assert!(!collateral.is_empty(), "No tokens provided");

            assert!(collateral.resource_address() == self.token_a_vault.resource_address() || 
            collateral.resource_address() == self.token_b_vault.resource_address(), "Wrong token provided");

            // When strike price is lesser than market price
            if collateral.resource_address() == self.token_b_vault.resource_address() {
                // Calculating the amount of tokens to be withdrawn
                let withdraw = self.cct_a.take(collateral.amount() / self.strike_rate);

                // This variable takes a collateral and mint its collateral-claim-token and converts it to a token by calling the convert_option method
                let convert = self.convert_option(collateral, withdraw);

                // Tokens getting swapped
                let mut amm_component = RadiswapComponentTarget::at(self.amm_address);
                let token_b = amm_component.swap(convert.0);

                // Putting collateral claim token to liquidity pool
                self.cct_b.put(convert.1);

                // returning the asset
                return token_b
            }
            // When strike price is bigger than market price
            else {
                // Calculating the amount of tokens to be withdrawn
                let withdraw = self.cct_b.take(collateral.amount() * self.strike_rate);

                // This variable takes a collateral and mint its collateral-claim-token and converts it to a token by calling the convert_option method
                let convert = self.convert_option(collateral, withdraw);

                // Tokens getting swapped
                let mut amm_component = RadiswapComponentTarget::at(self.amm_address);
                let token_a = amm_component.swap(convert.0);

                // Putting collateral claim token to liquidity pool
                self.cct_a.put(convert.1);

                // returning the asset
                return token_a
            }
        }

        // This method takes a collateral and mint its collateral token and converts it to a token
        pub fn convert_option(&mut self, lock_token: Bucket, cctoken: Bucket) -> (Bucket, Bucket) {
            // Checking lock token and collateral claim token are correctly provided
            assert!((lock_token.resource_address() == self.token_a_vault.resource_address() && cctoken.resource_address() ==
            self.cct_b.resource_address()) || (lock_token.resource_address() == self.token_b_vault.resource_address() && 
            cctoken.resource_address() == self.cct_a.resource_address()) , "Provided collateral or collateral claim token is wrong");

            // If collateral claim token is cctoken_a
            if lock_token.resource_address() == self.token_a_vault.resource_address() {

                // Collateral claim token(token_a) is minted
                let collateral_claim_token =  self.admin_badge_vault.authorize(|| {
                    borrow_resource_manager!(self.cct_a.resource_address()).mint(lock_token.amount())
                });

                // Collateral claim token(token_b) is burned
                self.admin_badge_vault.authorize(|| {
                    borrow_resource_manager!(self.cct_b.resource_address()).burn(cctoken)
                });

                // Calculating token that should be recieved to user
                let output_token = self.token_b_vault.take(lock_token.amount() * self.strike_rate);

                // Locking the collateral
                self.option_token_a_vault.put(lock_token);

                // Returning token of collateral-claim-token provided and collateral-claim-token of the token provided
                (output_token, collateral_claim_token)
            }
            // If collateral claim token is cctoken_a
            else {
                // Collateral claim token(token_b) is minted
                let collateral_claim_token = self.admin_badge_vault.authorize(|| {
                    borrow_resource_manager!(self.cct_b.resource_address()).mint(lock_token.amount())
                });

                // Collateral claim token(token_a) is burned
                self.admin_badge_vault.authorize(|| {
                    borrow_resource_manager!(self.cct_a.resource_address()).burn(cctoken)
                });

                // Calculating token that should be recieved to user
                let output_token = self.token_a_vault.take(lock_token.amount() / self.strike_rate);

                // Locking the collateral
                self.option_token_b_vault.put(lock_token);

                // Returning token of collateral-claim-token provided and collateral-claim-token of the token provided
                (output_token, collateral_claim_token)
            }
            
        }
        

        // To lock token_a and mint collateral-claim-token-a bonded-token
        fn option_a_deposit(&mut self, lock_token: Bucket) -> (Bucket, Bucket) {
            // Checking whether token provided is correct
            assert!(lock_token.resource_address() == self.token_a_vault.resource_address(), "Wrong token provided");

            // Minting collateral-claim-token-a
            let cctoken_a: Bucket = self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.cct_a.resource_address()).mint(lock_token.amount())
            });
            
            // Minting bonded token a
            let bonded_token_a: Bucket = self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.bonded_token_vault.resource_address()).mint(lock_token.amount())
            });

            // Locking the token to the option vault
            self.option_token_a_vault.put(lock_token);

            // returning collateral claim token and bonded token
            return (cctoken_a, bonded_token_a);
        }

/* 
        COULDNT USE THIS FUNCTION BECAUSE OF LIMITED TIME

        // Option to withdraw the locked token by providing the cctoken and bonded token
        fn option_a_withdraw(&mut self, unlock_token: ResourceAddress, cctoken_a: Bucket, bonded_token: Bucket) -> Bucket {
            // Checking whether the provided cctokens and bonded tokens are correct 
            assert!(cctoken_a.resource_address() == self.cct_a.resource_address() &&
            bonded_token.resource_address() == self.bonded_token_vault.resource_address(), "Wrong tokens provided");

            // Checking whether the provided resource address of token to be unlocked is correct
            assert!(unlock_token == self.token_a_vault.resource_address(), "Wrong token provided");
            
            // Taking back the locked token from option vault
            let output_token: Bucket = self.option_token_a_vault.take(cctoken_a.amount());

            // Burning the collateral claim token
            self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.cct_a.resource_address()).burn(cctoken_a)
            });

            // Burning the bonded token
            self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.bonded_token_vault.resource_address()).burn(bonded_token)
            });
            
            // Returning the locked token to the user
            return output_token;
        }
*/

        // To lock token_b and mint collateral-claim-tokens and bonded tokens
        fn option_b_deposit(&mut self, lock_token: Bucket) -> (Bucket, Bucket) {
            // Checking whether resource address of lock_token is correct
            assert!(lock_token.resource_address() == self.token_b_vault.resource_address(), "Wrong token provided");
            
            // Amount of lock_token
            let lock_token_amount = lock_token.amount();

            // Transferring the token to lock in the option vault
            self.option_token_b_vault.put(lock_token);

            // Minting collateral claim tokens of lock_token
            let cctoken_b: Bucket = self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.cct_b.resource_address()).mint(lock_token_amount)
            });

            // Minting bonded tokens of lock_token
            let bonded_token_b: Bucket = self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.bonded_token_vault.resource_address()).mint(lock_token_amount / self.strike_rate)
            });

            // returning collateral-claim-token and bonded token
            return (cctoken_b, bonded_token_b);
        } 
/*      
        COULDNT USE THIS FUNCTION BECAUSE OF LIMITED TIME

        fn option_b_withdraw(&mut self, unlock_token: ResourceAddress, cctoken_b: Bucket, bonded_token: Bucket) -> Bucket {
            assert!(cctoken_b.resource_address() == self.cct_b.resource_address() &&
            bonded_token.resource_address() == self.bonded_token_vault.resource_address(), "Wrong tokens provided");
            assert!(unlock_token == self.token_b_vault.resource_address(), "Wrong token provided");
            
            let output_token: Bucket = self.option_token_b_vault.take(cctoken_b.amount());

            self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.cct_b.resource_address()).burn(cctoken_b)
            });

            self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.bonded_token_vault.resource_address()).burn(bonded_token)
            });
            
            return output_token;
        } 
*/
        // When strike rate is lesser than market price
        pub fn lend_a(&mut self, mut lend_token: Bucket) -> (Bucket, Bucket, Bucket) {
            // Checking whether any tokens are provided 
            assert!(!lend_token.is_empty(), "No tokens provided");

            // Taking the fees and putting it into the liquidity pool
            self.cct_b.put(lend_token.take(lend_token.amount() * self.fee));

            // Checking whether provided token is correct
            assert!(lend_token.resource_address() == self.token_b_vault.resource_address(), "Swap the token");

            // Lending Badge
            let badge = self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.lend_vault.resource_address()).mint(1)
            });

            // Depositing token_b to mint its collateral claim token and bonded token
            let returns = self.option_b_deposit(lend_token);
            
            // Calculating interest amount of bonded token per second to be recieved
            let bond_token_per_minute = (self.bonded_token_vault.amount() / Decimal::from(self.duration)) -
            (self.constant_product / ((self.cct_b.amount() / self.strike_rate) + returns.1.amount()));
                                    
            // Calculating total amount of interest bonded token to be recieved                        
            let bond_token = bond_token_per_minute * Decimal::from(self.duration);

            // Sending collateral claim token to the pool
            self.cct_b.put(returns.0);

            // Taking the total amount of interest bonded token to be recieved from the bonded token pool  
            let required_bond_token = self.bonded_token_vault.take(bond_token);

            // Returning minted bonded token + interest bonded token
            (returns.1, required_bond_token, badge)
        }

        // When strike rate is bigger than market price
        pub fn lend_b(&mut self, mut lend_token: Bucket) -> (Bucket, Bucket, Bucket) {
            // Checking whether any tokens are provided 
            assert!(!lend_token.is_empty(), "No tokens provided");

            // Checking whether provided token is correct 
            assert!(lend_token.resource_address() == self.token_a_vault.resource_address(), "Swap the token");

            // Taking the fees and putting it into the liquidity pool
            self.cct_a.put(lend_token.take(lend_token.amount() * self.fee));

            // Lending Badge
            let badge = self.admin_badge_vault.authorize(|| {
                borrow_resource_manager!(self.lend_vault.resource_address()).mint(1)
            });

            // Depositing token_a to mint its collateral claim token and bonded token
            let returns = self.option_a_deposit(lend_token);

            // Calculating interest amount of bonded token per second to be recieved
            let bond_token_per_minute = (self.bonded_token_vault.amount() / Decimal::from(self.duration)) -
            (self.constant_product / (self.cct_a.amount() + returns.1.amount()));

            // Calculating total amount of interest bonded token to be recieved                        
            let bond_token = bond_token_per_minute * Decimal::from(self.duration);

            // Sending collateral claim token to the pool
            self.cct_a.put(returns.0);

            // Taking the total amount of interest bonded token to be recieved from the bonded token pool  
            let required_bond_token = self.bonded_token_vault.take(bond_token);
            
            // Returning minted bonded token + interest bonded token
            (returns.1, required_bond_token, badge)
        }
        
        // When strike rate is lesser than market price
        pub fn borrow_a(&mut self, mut collateral: Bucket) -> (Bucket, Bucket, Bucket) {
            // Checking the resource address of the collateral
            assert!(collateral.resource_address() == self.token_a_vault.resource_address() ||
            collateral.resource_address() == self.token_b_vault.resource_address(), "Wrong token provided");

            // Checking whether provided token is correct 
            if collateral.resource_address() == self.token_a_vault.resource_address() {
                // value of token_b in the pool
                let y: Decimal = self.cct_b.amount() / self.strike_rate;

                // Amount of bonded token per second
                let z: Decimal = self.bonded_token_vault.amount() / Decimal::from(self.duration);

                // collateral value
                let delta_y: Decimal = collateral.amount() / self.strike_rate;

                // Equation to find bonded_token_per_minute
                let bonded_token_per_minute: Decimal = (self.constant_product / (y - delta_y)) - z;
                let bond_token: Decimal = bonded_token_per_minute * Decimal::from(self.duration);

                // Taking the fees for borrowing and putting it into liquidity pool
                self.token_a_vault.put(collateral.take(collateral.amount() * self.fee));

                // Taking bond token worth of collateral(token_a)
                let first_batch: Bucket = collateral.take(bond_token);

                // Remaining collateral amount
                let second_batch: Bucket = collateral.take(collateral.amount() - first_batch.amount());

                // Depositing bond token worth of token_a and minting collateral-claim-token and bonded token
                let returns = self.option_a_deposit(first_batch);

                // Depositing the bonded token recieved from the line above to the liquidity pool and swapping it for collateral-claim-token(token_b)
                self.bonded_token_vault.put(returns.1);
                let cc_token =  self.cct_b.take(self.strike_rate * delta_y);

                // Converting remaining collateral amount(token_a) and collateral-claim-token_b to token_b and collateral-claim-token of token_a
                let convert = self.convert_option(second_batch, cc_token);

                // Returning borrowed amount of token(token_b), collateral-claim-token(token_a) and amount of interest to be paid
                // worth of collateral-claim-token(token_a) 
                return (convert.0, convert.1, returns.0);
            }
            else {                        
                // value of token_b in the pool
                let y: Decimal = self.cct_b.amount() / self.strike_rate;

                // Amount of bonded token per second
                let z: Decimal = self.bonded_token_vault.amount() / Decimal::from(self.duration);

                // collateral value
                let delta_y: Decimal = collateral.amount() / self.strike_rate;

                // Equation to find bonded_token_per_minute
                let bonded_token_per_minute: Decimal = (self.constant_product / (y - delta_y)) - z;
                let bond_token: Decimal = bonded_token_per_minute * Decimal::from(self.duration);

                // Taking the fees for borrowing and putting it into liquidity pool
                self.token_b_vault.put(collateral.take(collateral.amount() * self.fee));

                // Taking bond token worth of collateral(token_a)
                let first_batch: Bucket = collateral.take(bond_token * self.strike_rate);

                // Remaining collateral amount
                let second_batch: Bucket = collateral.take(collateral.amount() - first_batch.amount());

                // Depositing bond token worth of token_b and minting collateral-claim-token and bonded token
                let returns = self.option_b_deposit(first_batch);

                // Depositing the bonded token recieved from the line above to the liquidity pool and swapping it for collateral-claim-token(token_b)
                self.bonded_token_vault.put(returns.1);
                let cc_token =  self.cct_b.take(collateral.amount());

                // Swapping token_b for token_a through radiswap
                let mut amm_component = RadiswapComponentTarget::at(self.amm_address);
                let token_a = amm_component.swap(second_batch);

                // Returning borrowed amount of token(token_a), collateral-claim-token(token_b) and amount of interest to be paid
                // worth of collateral-claim-token(token_b)
                return (token_a, returns.0, cc_token);
            }            
        }    
            // When strike rate is greater than market price
        pub fn borrow_b(&mut self, mut collateral: Bucket) -> (Bucket, Bucket, Bucket) {
            // Checking the resource address of the collateral
            assert!(collateral.resource_address() == self.token_a_vault.resource_address() ||
            collateral.resource_address() == self.token_b_vault.resource_address(), "Wrong token provided");

            // Checking whether provided token is correct 
            if collateral.resource_address() == self.token_a_vault.resource_address() {
                // Equation to find bonded_token_per_minute
                let bonded_token_per_minute: Decimal = (self.constant_product / (self.cct_a.amount() - collateral.amount())) - self.bonded_token_vault.amount();
                let bond_token: Decimal = bonded_token_per_minute * Decimal::from(self.duration);

                // Taking the fees for borrowing and putting it into liquidity pool
                self.token_a_vault.put(collateral.take(collateral.amount() * self.fee));
    
                // Taking bond token worth of token_b
                let first_batch: Bucket = collateral.take(bond_token);
    
                // Remaining collateral amount
                let second_batch: Bucket = collateral.take(collateral.amount() - first_batch.amount());

                // Depositing bond token worth of token_a and minting collateral-claim-token and bonded token
                let returns = self.option_a_deposit(first_batch);

                // Depositing the bonded token recieved from the line above to the liquidity pool and swapping it for collateral-claim-token(token_a)
                self.bonded_token_vault.put(returns.1);
                let cc_token =  self.cct_a.take(collateral.amount());

                // Swapping token_a for token_b through radiswap
                let mut amm_component = RadiswapComponentTarget::at(self.amm_address);
                let token_a = amm_component.swap(second_batch);

                // Returning borrowed amount of token(token_a), collateral-claim-token(token_b) and amount of interest to be paid
                // worth of collateral-claim-token(token_b) 
                return (token_a, returns.0, cc_token);
            }
            else {
                // value of token_b in the pool
                let x: Decimal = self.cct_a.amount();

                // Amount of bonded token per second
                let z: Decimal = self.bonded_token_vault.amount() / Decimal::from(self.duration);

                // collateral value
                let delta_x: Decimal = collateral.amount() / self.strike_rate;

                // Equation to find bonded_token_per_minute
                let bonded_token_per_minute: Decimal = (self.constant_product / (x - delta_x)) - z;
                let bond_token: Decimal = bonded_token_per_minute * Decimal::from(self.duration);

                // Taking the fees for borrowing and putting it into liquidity pool
                self.token_b_vault.put(collateral.take(collateral.amount() * self.fee));

                // Taking bond token worth of collateral(token_b)
                let first_batch: Bucket = collateral.take(bond_token * self.strike_rate);

                // Remaining collateral amount
                let second_batch: Bucket = collateral.take(collateral.amount() - first_batch.amount());

                // Depositing bond token worth of token_a and minting collateral-claim-token and bonded token
                let returns = self.option_a_deposit(first_batch);

                // Depositing the bonded token recieved from the line above to the liquidity pool and swapping it for collateral-claim-token(token_b)
                self.bonded_token_vault.put(returns.1);
                let cc_token =  self.cct_a.take(delta_x);

                // Converting remaining collateral amount(token_b) and collateral-claim-token_b to token_b and collateral-claim-token of token_a
                let convert = self.convert_option(second_batch, cc_token);

                // Returning borrowed amount of token(token_a), collateral-claim-token(token_b) and amount of interest to be paid
                // worth of collateral-claim-token(token_b) 
                return (convert.0, convert.1, returns.0);
            }
        }
    }        
    

}