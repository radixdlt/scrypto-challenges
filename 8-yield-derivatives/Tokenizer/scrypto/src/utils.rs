use scrypto::prelude::*;
use scrypto_avltree::AvlTree;
use scrypto_math::*;
use std::env;

// Define the Reward enum
#[derive(Debug)]
pub enum Reward {
    Fixed,
    TimeBased,
}

// Function to get the NFT icon URL based on the environment
fn _get_nft_icon_url() -> String {
    match env::var("ENVIRONMENT") {
        Ok(environment) if environment == "production" => {
            env::var("NFT_ICON_URL_PROD").unwrap_or_default()
        }
        _ => {
            env::var("NFT_ICON_URL_NON_PROD").unwrap_or_default()
        }
    }
}

//utility check
pub fn assert_pair(message: String){
    assert!(
        dec!("1") == dec!("0"),
        "Unavailable pair! {}",message
    );
}


//for both supply and take_back
pub fn assert_resource(res_addr: &ResourceAddress, expect_res_addr: &ResourceAddress){
    assert!(res_addr == expect_res_addr, "Incorrect resource passed in for interacting with the component!");
}

//for supply
pub fn lend_checks_time_based(amount_supplied: Decimal){
    //no subsequent lends to remain simple
    assert!(
        amount_supplied == dec!("0"),
        "No loan accepted if previous is not closed yet!"
    );
}

pub fn lend_amount_checks(num_xrds: Decimal, min: Decimal, max: Decimal){
    assert!(
        num_xrds <= max,
        "No loan approved over 1000xrd at this time!"
    );
    assert!(
        num_xrds >= min,
        "No loan approved below 1xrd at this time!"
    );
}

//for take_back
pub fn take_back_checks(allowed_amount: Decimal, amount_to_be_returned: &Decimal){
    info!("Minimum amount : {:?} ", allowed_amount);  
    assert!(
        amount_to_be_returned >= &allowed_amount,
        "You cannot get back less than 20% of your loan!"
    );
}

//for tokenize
pub fn epoch_max_length_checks(
    borrow_epoch_max_allowed_length: Decimal, borrow_epochs_requested: Decimal){
    // Check the borrow epochs length
    info!("Max length of the borrow in epochs : {:?} ", borrow_epoch_max_allowed_length);  
    assert!(
        borrow_epoch_max_allowed_length >= borrow_epochs_requested,
        "You cannot borrow for more than epochs : {:?} ", borrow_epoch_max_allowed_length
    );
}

//for tokenize
pub fn epoch_min(borrow_epochs_requested: Decimal){
    // Check the min borrow epochs length
    info!("Min length of the borrow in epochs : 1 ");  
    assert!(
        borrow_epochs_requested >= dec!(1),
        "You have to borrow for at least 1 epoch instead of : {:?} ", borrow_epochs_requested
    );
}


//calculate interest
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


//calculate the interest for the epochs at the percentage given with the capital provided
pub fn calculate_interest(epochs: Decimal, percentage: Decimal, capital: Decimal) -> Decimal {
    // Calculate daily rate
    let daily_rate = percentage / dec!(100) / dec!(105120);

    // Assuming interest is calculated daily
    let compound_factor = (dec!(1) + daily_rate).pow(epochs);
    let interest = capital * (compound_factor.unwrap() - dec!(1));
    let rounded = interest.checked_round(5, RoundingMode::ToNearestMidpointToEven);

    rounded.unwrap()
}

// Implement the FromStr trait for parsing strings into Reward enum variants
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

// /// Checks whether maturity date has been reached.
pub fn check_maturity(maturity_date: Decimal) -> bool {    
    let maturity = maturity_date.to_string().parse::<u64>().unwrap();
    let current_epoch = Runtime::current_epoch().number();
    info!("Maturity  {} should be before current epoch {}", maturity, current_epoch);   
    maturity > current_epoch
}