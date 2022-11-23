// Author: NellySayon
// Blueprint: JYields
// Purpose: creates a user and delivers methods to handle the investement
//
// Note! Some of the methods choosing the investment should actually be moved to another blueprint
// to be more modular and provide the possibility to exchange that part with another blueprint
// (with e.g. other fees or other strategies)



use scrypto::prelude::*;
use std::ops::Mul;

use crate::data::{User, LevelChoice, UserPref, Investment, InvestmentType};
use crate::validators::instantiate_validators;
use crate::dexes::instantiate_dex;

blueprint! {
    struct JYields {
        user_data: User,
        xrd_wallet: Vault, // vault containing the XRD amount the user has giving to the Yield platform
        fee_wallet: Vault,  // vault with the fees the user has to pay
        juicy_wallet: Vault, // vault for the Juice Token the user receives
        investment_data: Vec<Investment> // this vector stores all investments that were made
    }

    impl JYields {    
        // -----------------------------------------------------------------------------                     
        // function to create a user account with the given user name,
        // a bucket of first JUICE token 
        // and default_values for preferences and investment
        // -----------------------------------------------------------------------------
        pub fn create_user(name: String, juice_bucket: Bucket) -> ComponentAddress {
            // create a kind of user ID
            let user_nft = ResourceBuilder::new_non_fungible()
            .metadata("username", &name)
            .no_initial_supply();
               
            Self {
                user_data : User {
                    user_name: name,
                    user_address: user_nft,
                    preferences: UserPref { // set preferences to default
                        risk_level: LevelChoice::Low,
                        contribution_level: LevelChoice::Low,
                        wants_lottery: false,
                        wants_charity: false,
                        wants_airdrops: false
                    },                
                },
                xrd_wallet: Vault::new(RADIX_TOKEN),
                fee_wallet: Vault::new(RADIX_TOKEN),
                juicy_wallet: Vault::with_bucket(juice_bucket),
                investment_data: Vec::new()                
            }
            .instantiate()                 
            .globalize()
        }

        // -----------------------------------------------------------------------------
        // set user preferences for the given user
        // -----------------------------------------------------------------------------
        pub fn set_preferences(&mut self, risk_level: LevelChoice, contribution_level: LevelChoice, 
                               wants_lottery: bool, wants_charity: bool, wants_airdrops: bool) {
            self.user_data.preferences.risk_level = risk_level;
            self.user_data.preferences.contribution_level = contribution_level;
            self.user_data.preferences.wants_lottery = wants_lottery;
            self.user_data.preferences.wants_charity = wants_charity;        
            self.user_data.preferences.wants_airdrops = wants_airdrops;
        }

        // -----------------------------------------------------------------------------
        // either sets or increases the deposit for the user
        // -----------------------------------------------------------------------------
        pub fn set_deposit(&mut self, new_deposit: Bucket) {
            self.xrd_wallet.put(new_deposit);            
        }

        // -----------------------------------------------------------------------------
        // check the risk tolerance of the user and make (a proposal for) an investment
        // possible result: Staking, Lending, Arbitrage
        // -----------------------------------------------------------------------------
        pub fn make_proposal(&mut self, inv_amount: Decimal) -> Option<Bucket>{
            // check if there is enough XRD in our wallet
            assert!((self.xrd_wallet.amount()>=inv_amount), "Not enough XRD in your wallet. Please deposit more or change the investment amount.");
            
            let mut do_arbitrage = true; 
            let mut do_lending = true;            

            // Check the risk level the user has selected to define the investment type
            // the other values are meant to be used to select a specific investment
            match self.user_data.preferences.risk_level {
                LevelChoice::Low => do_arbitrage = false,
                LevelChoice::Medium => do_lending = false,
                LevelChoice::High => {} // basically all three can be taken
            }

            // then we need to check which of the selected investment types gives the best return on investment
            // so some magic happens and gives us these current max values
            let staking_apy = dec!("9.75");
            let lending_apy = dec!("7.3");
            let arbitrage_apy = dec!("5.4");
            
            // select the investment type here
            let inv_type: InvestmentType;

            // staking is default anyway, so just check the others
            if do_lending && (lending_apy > arbitrage_apy || do_arbitrage == false) && lending_apy > staking_apy{
                inv_type = InvestmentType::Lending;
            }else if do_arbitrage && arbitrage_apy > staking_apy{
                inv_type = InvestmentType::Arbitrage;
            }else{
                inv_type = InvestmentType::Staking;
            }
            
            // Note! Here the user would actually need to confirm the proposal or 
            // better even choose from a selection of proposals
      
            // then we would actually perform the investment transaction           
            match inv_type {
                InvestmentType::Staking => Some(self.perform_stake(inv_amount)),
                InvestmentType::Lending => Some(self.perform_lending(inv_amount)),                
                InvestmentType::Arbitrage => {self.perform_arbitrage(inv_amount, "DPH".to_string());
                                              None},
                InvestmentType::Lottery => None, // lottery cannot be done via proposal
                InvestmentType::Betting => None, // betting cannot be done via proposal               
            }       
        }

        // -----------------------------------------------------------------------------        
        // takes the investment amount and the fee in %
        // to reduce the fee as XRD from the investment amount
        // -----------------------------------------------------------------------------
        fn reduce_fee(&mut self, inv_amount: Decimal, fee: Decimal) -> Decimal {
            // reduce the investment value with the fee
            let calc_fee = inv_amount.mul(fee);
            // move the fee to the other vault
            self.fee_wallet.put(self.xrd_wallet.take(calc_fee));
            // set the new investment balance     
            let new_amount = inv_amount - calc_fee;     
            
            info!("Investment amount: {:?} XRD", new_amount);
            info!("Investment fee: {:?} XRD", calc_fee);
            
            new_amount
        }

        // -----------------------------------------------------------------------------        
        // creates a ticket for the Juicy Yields own lottery
        // one lottery ticket has a fixed price of 100 XRD 
        // 5 XRD from that are taken as fee
        // Note! You can only buy one ticket at a time, but you can call this method 
        // several times
        // -----------------------------------------------------------------------------
        pub fn buy_lottery(&mut self) -> Bucket{
            // check if there is enough XRD in our vault
            assert!((self.xrd_wallet.amount()>=dec!("100")), "Not enough XRD in your wallet. Please deposit more or change the investment amount.");
           
            let new_amount = self.reduce_fee(dec!("100"), dec!("0.05"));

            self.investment_data.push(Investment{
                inv_address: "Juicy Yields".to_string(),                        
                inv_type: InvestmentType::Lottery,
                inv_value: new_amount,
               }
            );

            info!("One lottery ticket created with 95 XRD at Juicy Yields");

            self.xrd_wallet.take(dec!("95"))
        }

        // -----------------------------------------------------------------------------        
        // select a validator for staking based on user preferences
        // -----------------------------------------------------------------------------        
        pub fn perform_stake(&mut self, inv_amount: Decimal) -> Bucket{
            // check if there is enough XRD in the user's vault
            assert!((self.xrd_wallet.amount()>=inv_amount), "Not enough XRD in your wallet. Please deposit more or change the investment amount.");

            //get all validators
            let v_all = instantiate_validators();
            
            let mut v_pre_select = Vec::new();            

            let mut select = 0;

            let mut val_found = false;

            // check if there is a validator that fulfills the given preferences of the user
            for i in 0..v_all.len(){
                // when the user has set wants_xxx to false, it still does not harm if the 
                // validator does that anyway. It's considered as "I don't care". 
                if (v_all[i].does_lottery == self.user_data.preferences.wants_lottery || self.user_data.preferences.wants_lottery == false) && 
                   (v_all[i].does_charity == self.user_data.preferences.wants_charity || self.user_data.preferences.wants_charity == false) &&
                   (v_all[i].does_airdrops == self.user_data.preferences.wants_airdrops || self.user_data.preferences.wants_airdrops == false) {
                    
                    let min_uptime: Decimal;
                    // continue checking the uptime based on the chosen risk level
                    match self.user_data.preferences.risk_level{
                        LevelChoice::Low => min_uptime = dec!("99.8"),
                        LevelChoice::Medium => min_uptime = dec!("95"),
                        LevelChoice::High => min_uptime = dec!("80"),
                    } 
                    // does the validator fulfill the minimum uptime?
                    if v_all[i].uptime >= min_uptime {
                        // just remember the index of the validator in the original vector
                        v_pre_select.push(i);                    
                    }
                }
            }

            info!("Number of preselected validators: {:?}", v_pre_select.len());
            // if at least 2 validators are left continue to check the highest APY
            if v_pre_select.len() >=2 {
                for i in 0..v_pre_select.len(){
                    if v_pre_select.len()> i+1{
                        if v_all[i+1].apy > v_all[i].apy{
                            select = i+1;
                        }                        
                    }
                }                
                val_found = true;            
            }else if v_pre_select.len() == 1 {
                val_found = true;
                select = v_pre_select[0];
            }

            // so do we have any good validator?
            if val_found == true{
                info!("Index of selected validator: {:?}", select);
                // get all data from that selected validator
                let _sel_val = v_all.get(select);   

                // for Staking we take 2.5 % fee
                let new_amount = self.reduce_fee(inv_amount, dec!("0.025"));

                info!("Investment performed: Staking with {:?} XRD", new_amount);

                // store the investement data
                self.investment_data.push(Investment{
                        inv_address: "Best Validator found".to_string(),
                        inv_value: new_amount,
                        inv_type: InvestmentType::Staking,
                    }
                );            
                // we reduce the amount in the users xrd wallet here to simulate the staking
                // usually this would need to be sent to the validator instead
                self.xrd_wallet.take(new_amount)
            } else{
                info!("No appropriate validator found");
                self.xrd_wallet.take(0)
            }            
        }
        
        // -----------------------------------------------------------------------------        
        // this would select the perfect lending opportunity and just do it 
        // it only covery the part where the user provides xrd to another user and 
        // would receive interest for that
        // -----------------------------------------------------------------------------
        pub fn perform_lending(&mut self, inv_amount: Decimal) -> Bucket{
            // check if there is enough XRD in our vault
            assert!((self.xrd_wallet.amount()>=inv_amount), "Not enough XRD in your wallet. Please deposit more or change the investment amount.");

            // magic happens to select the best lending opportunity...
            let inv_address = "Best lending platform found".to_string();

            // for lending we take 1,5 % fee
            let new_amount = self.reduce_fee(inv_amount, dec!("0.015"));

            info!("Investment performed: Lending with {:?} XRD at {:?}", new_amount, inv_address);

            // store the investement data
            self.investment_data.push(Investment{
                    inv_address: inv_address,
                    inv_value: new_amount,
                    inv_type: InvestmentType::Lending,
                }
            );            
            // we reduce the amount in the user's xrd wallet here to simulate the lending
            // usually this would need to be sent to a connected lending platform instead
            // and we would also need to ensure that the user receives back the interest
            self.xrd_wallet.take(new_amount)
        }

        // -----------------------------------------------------------------------------        
        // this would select the perfect betting opportunity and just do it 
        // -----------------------------------------------------------------------------
        pub fn perform_bet(&mut self, inv_amount: Decimal) -> Bucket{
            // check if there is enough XRD in our vault
            assert!((self.xrd_wallet.amount()>=inv_amount), "Not enough XRD in your wallet. Please deposit more or change the investment amount.");
            
            // magic happens to select the best bet...
            let inv_address = "Delphibets".to_string();

            // for betting we take 6 % fee
            let new_amount = self.reduce_fee(inv_amount, dec!("0.06"));

            info!("Investment performed: Betting with {:?} XRD at {:?}", new_amount, inv_address);

            // store the investement data
            self.investment_data.push(Investment{
                    inv_address: inv_address,
                    inv_value: new_amount,
                    inv_type: InvestmentType::Betting,
                }
            );            
            // we reduce the amount in the user's xrd wallet here to simulate the betting
            // usually this would need to be sent to a connected betting platform instead
            // furthermore we would of course need to tell them which exact bet we want to 
            // take part of
            self.xrd_wallet.take(new_amount)
        }

        
        // -----------------------------------------------------------------------------        
        // This would select the perfect arbitrage opportunity and just do it 
        // Arbitrage would either work on one single dex because of price differences 
        // in different pools e.g. Pool A: 1 XRD = 0.43 OCI; Pool B 1 OCI = 67.97 FOTON; 
        // Pool C 1 FOTON = 0.05 XRD
        // OR by different prizes on different dexes what we are simulating here
        // -----------------------------------------------------------------------------
        pub fn perform_arbitrage(&mut self, inv_amount: Decimal, token: String){
            // check if there is enough XRD in our vault
            assert!((self.xrd_wallet.amount()>=inv_amount), "Not enough XRD in your wallet. Please deposit more or change the investment amount.");
            
            // get our list of dexes and token
            let all_dex = instantiate_dex();

            let mut values = Vec::new();
            let mut swap_fees = Vec::new();
            
            let mut arb_amount = dec!("0.0");

            let mut dex_high = 0;
            let mut dex_low = 0;

            // check if there is a dex with the wished token
            for i in 0..all_dex.len(){
                if all_dex[i].token == token{
                    values.push (all_dex[i].value);
                    swap_fees.push (all_dex[i].swap_fee);
                }
            }

            // is there at least two dexes to compare?
            if values.len() >=2 {
                // check for the highest and lowest value and remember their index
                for i in 0..values.len(){
                    if values.len()> i+1{
                        if values[i+1] > values[i]{
                            dex_high = i+1;
                        }
                        else if values[i+1] < values[i]{
                            dex_low = i+1;
                        }
                    }
                }                            
            }

            // did we find different dexes?
            if dex_low != dex_high{                                
                //do the fees still make the swapping a good profit?
                arb_amount = (inv_amount * (values[dex_high]-swap_fees[dex_high])) - (inv_amount * (values[dex_low]-swap_fees[dex_low]));
            }

            // minimum profit for an arbitrage is considerred as 10 XRD
            if arb_amount >= dec!("10") {
            
                // so here we would perform the actual arbitrage:
                // 1. send bucket of XRD to dex_low, swap XRD to token
                // 2. send token to dex_high, swap it to XRD
                // 3. put xrd back to user's vault
                
                // for arbitrage we take 5 % fee (on profit only)
                self.reduce_fee(arb_amount, dec!("0.05"));

                info!("Arbitrage performed: with {:?} XRD for token {:?}. Earned {:?} XRD", inv_amount, token, arb_amount);

                // store the investement data
                self.investment_data.push(Investment{
                        inv_address: "".to_string(),
                        inv_value: inv_amount,
                        inv_type: InvestmentType::Arbitrage,
                    }
                );
            }
            else {
                info!("No Arbitrage possibility found.");
            };
        }

        // -----------------------------------------------------------------------------        
        // takes all the XRDs that were paid as fee and returns them as a Bucket
        // -----------------------------------------------------------------------------        
        pub fn collect_fee(&mut self) -> Bucket{
            self.fee_wallet.take_all()
        }

        // -----------------------------------------------------------------------------        
        // deposit $JUICE rewards to the user vault based on invested XRD
        // -----------------------------------------------------------------------------        
        pub fn drop_juice(&mut self, juice: Bucket){
            self.juicy_wallet.put(juice);
        }

        // -----------------------------------------------------------------------------        
        // temporarily takes all the Juice tokens to enable the juice component
        // to check the amount and pay the xrd rewards
        // -----------------------------------------------------------------------------        
        pub fn check_juice_amount(&mut self) -> Bucket{
            self.juicy_wallet.take_all()
        }

    }
}