use crate::user;
use scrypto::prelude::*;

fn calculate_total_collateral(
    user: &user::User,
    asset_ltv_ratios: &HashMap<ResourceAddress, Decimal>,
) -> Decimal {

    let user_badge_resource_address = user.user_badge_resource_address;

    info!(
        "`{:?}` calculating total collateral.",
        user_badge_resource_address
    );

    // Total collateral a user has across all their assets
    let mut user_collateral_sum: Decimal = 0.into();

    // Iterate over each asset and calculate the amount of collateral available from each
    for (asset_address, asset_amount) in &user.deposit_balances {

        let cost_of_asset_in_terms_of_xrd = Decimal::from(10000) / 10000;

        let ltv = asset_ltv_ratios.get(asset_address).unwrap();

        let asset_value_in_xrd = asset_amount.balance * cost_of_asset_in_terms_of_xrd;
        let asset_collateral = asset_value_in_xrd * *ltv;
        user_collateral_sum += asset_collateral;

        info!(
                    "[USER:`{:?}`] Asset=`{:?}`, Amount=`{:?}`, Value in XRD=`{:?}`, LTV=`{:?}`, Collateral Amount=`{:?}`",
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

    info!(
        "[USER:`{:?}`] Calculating Total Loan Balance.",
        user_badge_resource_address
    );

    // Total loan balance a user has across all their borrowed assets
    let mut total_loan_balance: Decimal = 0.into();

    // Iterate over each asset and sum the total loan balance
    for (asset_address, asset_amount) in &user.borrow_balances {
        // TODO: Pull this data from an oracle - right now assuming all assets have a 1:1 ratio with the price of radix
        let cost_of_asset_in_terms_of_xrd = Decimal::from(10000) / 10000;

        let loan_balance_in_terms_of_xrd = asset_amount.balance * cost_of_asset_in_terms_of_xrd;

        info!(
            "[USER:`{:?}`] Asset=`{:?}`, Amount=`{:?}`, Value in XRD=`{:?}`",
            user_badge_resource_address, asset_address, asset_amount, loan_balance_in_terms_of_xrd
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
        "[USER:`{:?}`] Total collateral of `{:?}` XRD. Current loan balance of \
        `{:?}` XRD. Available collateral of `{:?}` XRD.",
        user_badge_resource_address,
        users_total_collateral,
        users_loan_balance,
        available_collateral
    );
    available_collateral
}

pub fn calculate_interest_amount(user: &user::User, asset_address: ResourceAddress, interest_rate: Decimal) -> Decimal {
    match user.deposit_balances.get(&asset_address) {
        Some(current_balance) => {
            interest_rate * current_balance.deposit_time_elapsed()
            // current_ballance * interest_rate * time_elapsed = interest
        }
        None => Decimal::zero(),
    }
}

pub fn get_utilisation(deposit_balance: Decimal, borrow_balance: Decimal) -> Decimal {
    let utilisation = borrow_balance / deposit_balance * 100;
    utilisation
}

pub fn calculate_borrow_rate(
    multiplier: Decimal,
    base_multiplier: Decimal,
    base: Decimal,
    kink: Decimal,
    utilisation: Decimal,
) -> Decimal {
    if utilisation > Decimal::from("0") {
        if utilisation < kink {
            let borrow_rate = multiplier * utilisation + base;
            borrow_rate
        } else {
            let borrow_rate = multiplier * kink
                + base_multiplier * (utilisation - kink)
                + base * (utilisation - kink);
            borrow_rate
        }
    } else {
        Decimal::from("0")
    }
}

// pub fn get_deposit_rate(reserve_factor: Decimal, utilisation: Decimal, borrow_rate:Decimal) -> Decimal {
    pub fn calculate_deposit_rate(borrow_rate:Decimal) -> Decimal {
    let deposit_rate = borrow_rate;
    // This will be the calculation when platform fees are implemented
    //let deposit_rate = borrow_rate * utilisation * (Decimal::from("1") - reserve_factor) / 100;
    deposit_rate
}

pub fn calculate_sr_tokens_to_mint(deposit: Decimal, exchange_rate: Decimal) -> Decimal {
    let tokens_to_mint = deposit / exchange_rate;
    tokens_to_mint
}

pub fn calculate_sr_token_exchange_rate(
    deposit: Decimal,
    sr_token_total_supply: Decimal,
    reserve_factor: Decimal,
) -> Decimal {
    let exchange_rate = (deposit - reserve_factor) / sr_token_total_supply;
    exchange_rate
}

// r = Interest rate percentage per epoch
// r/100 = Interest rate per epoch decimal
pub fn calculate_new_rate(rate: Decimal) -> Decimal {

    let et = Decimal::from("60");

    let r = rate / ((Decimal::from("24") * Decimal::from("60") * Decimal::from("365")) / et);
    info!("r is {}.", r);
    // Decimal
    r/100
}

