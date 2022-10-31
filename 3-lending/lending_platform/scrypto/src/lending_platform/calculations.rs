use scrypto::prelude::*;
use crate::lending_platform::user;


fn calculate_total_collateral(
    user: &user::User,
    asset_ltv_ratios: &HashMap<ResourceAddress, Decimal>,
) -> Decimal {
    /*
    calculate_total_collateral calculates the total value a user has of each deposited
    asset type in terms of XRD, then applies an LTV modifier based on each asset type to
    calculate the total collateral a user has.

    Total Collateral =
        quantity_asset_1 * xrd_price_asset_1 * ltv_asset_1 +
        quantity_asset_2 * xrd_price_asset_2 * ltv_asset_2 +
        ...
        quantity_asset_n * xrd_price_asset_n * ltv_asset_n
     */
    let user_badge_resource_address = user.user_badge_resource_address;

    info!("[LendingPlatform][USER:{}] Calculating Total Collateral.", user_badge_resource_address);

    // Total collateral a user has across all their assets
    let mut user_collateral_sum: Decimal = 0.into();

    // Iterate over each asset and calculate the amount of collateral available from each
    for (asset_address, asset_amount) in &user.deposit_balances {

        // TODO: Pull this data from an oracle - right now assuming all assets have a 1:1 ratio with the price of radix
        // We have to do this wonky behavior to create a Decimal. This will be improved in the future
        // let cost_of_asset_in_terms_of_xrd = Decimal::from(54564)/10000;
        let cost_of_asset_in_terms_of_xrd = Decimal::from(10000) / 10000;

        let ltv = asset_ltv_ratios.get(asset_address).unwrap();

        let asset_value_in_xrd = *asset_amount * cost_of_asset_in_terms_of_xrd;
        let asset_collateral = asset_value_in_xrd * *ltv;
        user_collateral_sum += asset_collateral;

        info!(
                    "[LendingPlatform][USER:{}] Asset={}, Amount={}, Value in XRD={}, LTV={}, Collateral Amount={}",
                    user_badge_resource_address,
                    asset_address,
                    asset_amount,
                    asset_value_in_xrd,
                    ltv,
                    asset_collateral
                );
    }
    user_collateral_sum.into()
}

fn calculate_total_loan_balance(user: &user::User) -> Decimal {
    let user_badge_resource_address = user.user_badge_resource_address;

    info!("[LendingPlatform][USER:{}] Calculating Total Loan Balance.", user_badge_resource_address);

    // Total loan balance a user has across all their borrowed assets
    let mut total_loan_balance: Decimal = 0.into();

    // Iterate over each asset and sum the total loan balance
    for (asset_address, asset_amount) in &user.borrow_balances {

        // TODO: Pull this data from an oracle - right now assuming all assets have a 1:1 ratio with the price of radix
        let cost_of_asset_in_terms_of_xrd = Decimal::from(10000) / 10000;

        let loan_balance_in_terms_of_xrd = *asset_amount * cost_of_asset_in_terms_of_xrd;

        info!(
            "[LendingPlatform][USER:{}] Asset={}, Amount={}, Value in XRD={}",
            user_badge_resource_address,
            asset_address,
            asset_amount,
            loan_balance_in_terms_of_xrd
        );

        total_loan_balance += loan_balance_in_terms_of_xrd;
    }
    total_loan_balance.into()
}

pub fn calculate_available_collateral(
    user: &user::User,
    asset_ltv_ratios: &HashMap<ResourceAddress, Decimal>,
) -> Decimal {
    let user_badge_resource_address = user.user_badge_resource_address;

    let users_total_collateral = calculate_total_collateral(user, asset_ltv_ratios);
    let users_loan_balance = calculate_total_loan_balance(user);
    let available_collateral = users_total_collateral - users_loan_balance;
    info!(
        "[LendingPlatform][USER:{}] Total collateral of {} XRD. Current loan balance of \
        {} XRD. Available collateral of {} XRD.",
        user_badge_resource_address,
        users_total_collateral,
        users_loan_balance,
        available_collateral
    );
    available_collateral
}