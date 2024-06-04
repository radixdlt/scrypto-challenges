//! # Overview of utility functions
//!
//! ## assert_pair
//!
//! [assert_pair()][assert_pair]
//! Utility check for asserting an unavailable pair
//!
//! ## assert_resource
//!
//! [assert_resource()][assert_resource]
//! Utility check for asserting if an incorrect resource address has been given in
//!
//! ## check_maturity
//!
//! [check_maturity()][check_maturity]
//! Checks whether maturity date has been reached.
//!
//! ## epoch_max_length_checks
//!
//! [epoch_max_length_checks()][epoch_max_length_checks]
//! Utility check for verifying max length of the tokenize in epochs
//! 
//! ## lend_amount_checks
//!
//! [lend_amount_checks()][lend_amount_checks]
//! Utility check for verifying min and max limits
//!
//! ## lend_checks_time_based
//!
//! [lend_checks_time_based()][lend_checks_time_based]
//! Utility check for verifying if previous liquidity supply has been closed yet
//!
//! ## take_back_checks
//!
//! [take_back_checks()][take_back_checks]
//! Utility check for verifying the minimum amount of withdraw
//!
//! ## calculate_interest
//!
//! [calculate_interest()][calculate_interest]
//! Calculates interest on the amount supplied for the specified number of epochs and at the specified percentage
//!
//! ## calculate_interest
//!
//! [calculate_interest()][calculate_interest]
//! Calculates interest
//! 

use scrypto::prelude::*;
use scrypto_avltree::AvlTree;
use scrypto_math::*;

// Define the Reward enum
#[derive(Debug)]
pub enum Reward {
    Fixed,
    TimeBased,
}


/// Utility check for asserting an unavailable pair
/// 
/// Arguments:
/// - `message`: The message to be returned
///
pub fn assert_pair(message: String){
    assert!(
        dec!("1") == dec!("0"),
        "Unavailable pair! {}",message
    );
}


/// Utility check for asserting if an incorrect resource address has been given in
/// 
/// Arguments:
/// - `res_addr`: The resource address received
/// - `expect_res_addr`: The expected resource address 
///
pub fn assert_resource(res_addr: &ResourceAddress, expect_res_addr: &ResourceAddress){
    assert!(res_addr == expect_res_addr, "Incorrect resource passed in for interacting with the component!");
}

/// Utility check for verifying if previous liquidity supply has been closed yet 
/// 
/// Arguments:
/// - `amount_supplied`: Amount 
///
pub fn lend_checks_time_based(amount_supplied: Decimal){
    assert!(
        amount_supplied == dec!("0"),
        "No loan accepted if previous is not closed yet!"
    );
}

/// Utility check for verifying min and max limits 
/// 
/// Arguments:
/// - `num_xrds`: Amount 
/// - `min`: Minimum amount of tokens accepted
/// - `max`: Max amount of tokens accepted
///
pub fn lend_amount_checks(num_xrds: Decimal, min: Decimal, max: Decimal){
    assert!(
        num_xrds <= max,
        "No loan approved over {} at this time!", max
    );
    assert!(
        num_xrds >= min,
        "No loan approved below {} at this time!", min
    );
}

/// Utility check for verifying the minimum amount of withdraw
/// 
/// Arguments:
/// - `allowed_amount`: Minimum amount withdrawable 
/// - `amount_to_be_returned`: Amount returned
///
pub fn take_back_checks(allowed_amount: Decimal, amount_to_be_returned: &Decimal){
    info!("Minimum amount : {:?} ", allowed_amount);  
    assert!(
        amount_to_be_returned >= &allowed_amount,
        "You cannot get back less than 20% of your loan!"
    );
}

/// Utility check for verifying max length of the tokenize in epochs
/// 
/// Arguments:
/// - `tokenize_epoch_max_allowed_length`: Max allowed lenght of the tokenize blocking period 
/// - `tokenize_epochs_requested`: Requested lenght of the tokenize blocking period
///
pub fn epoch_max_length_checks(
    tokenize_epoch_max_allowed_length: Decimal, tokenize_epochs_requested: Decimal){
    info!("Max length of the tokenize in epochs : {:?} ", tokenize_epoch_max_allowed_length);  
    assert!(
        tokenize_epoch_max_allowed_length >= tokenize_epochs_requested,
        "You cannot tokenize for more than epochs : {:?} ", tokenize_epoch_max_allowed_length
    );
}

/// Utility check for verifying min length of the tokenize in epochs
/// 
/// Arguments:
/// - `tokenize_epochs_requested`: Min allowed lenght of the tokenize blocking period 
///
pub fn epoch_min(tokenize_epochs_requested: Decimal){
    info!("Min length of the tokenize in epochs : 1 ");  
    assert!(
        tokenize_epochs_requested >= dec!(1),
        "You have to tokenize for at least 1 epoch instead of : {:?} ", tokenize_epochs_requested
    );
}


/// calculate interest
/// 
/// Arguments:
/// - `reward_type`: Reward type for suppliers (Fixed/TimeBased) 
/// - `reward_fixed`: Used only when the reward type is Fixed (Not available in this blueprint)
/// - `start_lending_epoch`: Start epoch of the liquidity supply
/// - `amount_to_be_returned`: Amount to be returned to the account
/// - `interest_for_suppliers`: Binary tree that will need to be scrolled to extract the interest rate changes along the epochs
///
/// Returns:
/// - `total_amount`: Total amount of the reward
/// 
pub fn calculate_interests(
    reward_type: &String, 
    reward_fixed: &Decimal, 
    start_lending_epoch: u64, 
    amount_to_be_returned: &Decimal, 
    interest_for_suppliers: &AvlTree<Decimal, Decimal>) -> Decimal {

    // Dereference the Decimal values
    let amount = *amount_to_be_returned;
    let fixed = *reward_fixed;
    let current_epoch = Runtime::current_epoch().number(); 
        
    //calculate interest to be repaied with specified reward type 
    return match Reward::from_str(reward_type) {
        Ok(reward) => {
            match reward {
                Reward::Fixed => {
                    info!("Handle Fixed reward logic here");
                    amount*fixed/100
                }
                Reward::TimeBased => {
                    info!("Handle TimeBased logic here from epoch {} to epoch {} applied to capital {}" , start_lending_epoch, current_epoch, amount_to_be_returned);
    
                    // Use fold to calculate the total interest
                    let total_amount = interest_for_suppliers
                        .range(Decimal::from(start_lending_epoch)..Decimal::from(current_epoch))
                        .fold((dec!(0), Decimal::from(start_lending_epoch), dec!(0)), |(total, first_epoch, _last_value), (key, value, _next_key)| {
                            let internal_length = key - first_epoch;
                            info!("epoch: {}, interest %: {}, length of the period: {}", key, value, internal_length);
                            let accumulated_interest =
                                calculate_interest(Decimal::from(internal_length), value, amount);
                            info!("Adding accumulated_interest {} for the period, totalling {} from epoch {} until epoch {} ", accumulated_interest, total + accumulated_interest, first_epoch, key);
                            (total + accumulated_interest, key, value)
                        });
    
                    // Need to add the last run from first_epoch to current epoch
                    let last = current_epoch - total_amount.1;
                    let accumulated_interest =
                        calculate_interest(Decimal::from(last), total_amount.2, amount);
                    info!("Adding accumulated_interest {} for the last period, totalling {} from epoch {} until epoch {} ", accumulated_interest, total_amount.0 + accumulated_interest, total_amount.1, current_epoch);
    
                    total_amount.0 + accumulated_interest
                }
            }
        }
        Err(()) => {
            println!("Invalid reward string");
            // Handle invalid input here
            dec!(0)
        }
    };
}


/// Calculates interest on the amount supplied for the specified number of epochs and at the specified percentage
/// 
/// Arguments:
/// - `epochs`: Number of epochs
/// - `percentage`: Percentage to be applied
/// - `capital`: Amount of tokens supplied
/// 
/// Returns:
/// - `rounded`: Amount of the reward
/// 
pub fn calculate_interest(epochs: Decimal, percentage: Decimal, capital: Decimal) -> Decimal {
    // Calculate daily rate
    let daily_rate = percentage / dec!(100) / dec!(105120);

    // Assuming interest is calculated daily
    let compound_factor = (dec!(1) + daily_rate).pow(epochs);
    let interest = capital * (compound_factor.unwrap() - dec!(1));
    let rounded = interest.checked_round(5, RoundingMode::ToNearestMidpointToEven);

    rounded.unwrap()
}



/// Checks whether maturity date has been reached.
/// 
/// Arguments:
/// - `maturity_date`: Maturity date expressed in epochs
///
pub fn check_maturity(maturity_date: Decimal) -> bool {    
    let maturity = maturity_date.to_string().parse::<u64>().unwrap();
    let current_epoch = Runtime::current_epoch().number();
    info!("Maturity  {} should be before current epoch {}", maturity, current_epoch);   
    maturity > current_epoch
}


/// Implement the FromStr trait for parsing strings into Reward enum variants
/// 
/// Arguments:
/// - `allowed_amount`: Minimum amount withdrawable 
/// - `amount_to_be_returned`: Amount returned
///
impl FromStr for Reward {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fixed" => Ok(Reward::Fixed),
            "timebased" => Ok(Reward::TimeBased),
            _ => Err(()),
        }
    }
}