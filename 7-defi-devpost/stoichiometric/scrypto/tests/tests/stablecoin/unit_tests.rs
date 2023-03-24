use scrypto::prelude::{dec, Decimal, Instant};
use sqrt::error::Error;
use std::collections::HashMap;
use stoichiometric_tests::dumb_oracle::utils::set_oracle_price;
use stoichiometric_tests::stablecoin::issuer_state::LenderState;
use stoichiometric_tests::stablecoin::sqrt_implem::IssuerMethods;
use stoichiometric_tests::stablecoin::utils::{
    assert_current_has_loan, assert_current_has_no_loan_id, instantiate, new_default_lender,
};
use stoichiometric_tests::utils::{ADMIN_BADGE_NAME, STABLECOIN_NAME};

#[test]
fn test_instantiate() {
    let (test_env, _) = instantiate();
    assert_eq!(
        test_env.amount_owned_by_current(ADMIN_BADGE_NAME),
        Decimal::ONE
    )
}

#[test]
fn test_new_lender() {
    let (mut test_env, mut issuer_state) = instantiate();
    new_default_lender(&mut test_env, "btc");

    issuer_state.update();

    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        Decimal::ZERO,
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);

    issuer_state.assert_state_is(&HashMap::new(), &lenders, 0, 0);
}

#[test]
fn test_multiple_new_lenders() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env.create_fixed_supply_token("eth", dec!(1000));
    new_default_lender(&mut test_env, "eth");
    issuer_state.update();

    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        Decimal::ZERO,
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    let eth_lender = LenderState::from(
        Decimal::ZERO,
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    lenders.insert(test_env.get_resource("eth").clone(), eth_lender);

    issuer_state.assert_state_is(&HashMap::new(), &lenders, 0, 0);
}

#[test]
fn test_new_lender_already_exists_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env
        .call_method(IssuerMethods::NewLender(
            "btc".to_string(),
            Decimal::ONE,
            Decimal::ONE,
            Decimal::ONE,
            Decimal::ONE,
            "issuer_comp".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "There is already a lender for the given token".to_string(),
        ))
        .run();
}

#[test]
fn test_new_lender_ltv_negative_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env
        .call_method(IssuerMethods::NewLender(
            "radix".to_string(),
            dec!("-2"),
            Decimal::ONE,
            Decimal::ONE,
            Decimal::ONE,
            "issuer_comp".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "LTV should be such that 0<LTV<1".to_string(),
        ))
        .run();
}

#[test]
fn test_new_lender_ltv_bigger_than_one_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env
        .call_method(IssuerMethods::NewLender(
            "radix".to_string(),
            dec!(2),
            Decimal::ONE,
            Decimal::ONE,
            Decimal::ONE,
            "issuer_comp".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "LTV should be such that 0<LTV<1".to_string(),
        ))
        .run();
}

#[test]
fn test_new_lender_dir_negative_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env
        .call_method(IssuerMethods::NewLender(
            "radix".to_string(),
            dec!("0.7"),
            dec!("-1"),
            Decimal::ONE,
            Decimal::ONE,
            "issuer_comp".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "The daily interest rate should be such that 0<DIR<1".to_string(),
        ))
        .run();
}

#[test]
fn test_new_lender_dir_bigger_than_one_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env
        .call_method(IssuerMethods::NewLender(
            "radix".to_string(),
            dec!("0.7"),
            dec!("2"),
            Decimal::ONE,
            Decimal::ONE,
            "issuer_comp".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "The daily interest rate should be such that 0<DIR<1".to_string(),
        ))
        .run();
}

#[test]
fn test_new_lender_liquidation_threshold_smaller_than_one_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env
        .call_method(IssuerMethods::NewLender(
            "radix".to_string(),
            dec!("0.7"),
            dec!("0.7"),
            dec!("0.9"),
            Decimal::ONE,
            "issuer_comp".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "The liquidation threshold should be greater than one".to_string(),
        ))
        .run();
}

#[test]
fn test_new_lender_liquidation_threshold_ltv_product_greater_than_one_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env
        .call_method(IssuerMethods::NewLender(
            "radix".to_string(),
            dec!("0.7"),
            dec!("0.7"),
            dec!("2"),
            Decimal::ONE,
            "issuer_comp".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "The LTV-liquidation threshold product should be smaller than one".to_string(),
        ))
        .run();
}

#[test]
fn test_new_lender_liquidation_incentive_non_positive_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    test_env
        .call_method(IssuerMethods::NewLender(
            "radix".to_string(),
            dec!("0.7"),
            dec!("0.7"),
            dec!("1.3"),
            dec!("-0.5"),
            "issuer_comp".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "The liquidation incentive should be positive".to_string(),
        ))
        .run();
}

#[test]
fn test_take_loan() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // The default btc lender has an LTV of 0.7 so for a bitcoin at 20000$, we should be able to loan 42k$ for 3 bitcoins
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(42000),
        ))
        .run();

    issuer_state.update();

    // Check that the issuer is in the right state
    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        dec!(3),
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    issuer_state.assert_state_is(&HashMap::new(), &lenders, 1, 0);

    // Check that the account has the right amount of stablecoins and the right NFR
    assert_eq!(
        test_env.amount_owned_by_current(STABLECOIN_NAME),
        dec!(42000)
    );
    assert_current_has_loan(
        &test_env,
        "#0#",
        "btc",
        dec!(3),
        dec!(42000),
        0,
        dec!("0.0001"),
    );
}

#[test]
fn test_take_loan_not_enough_collateral_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // The default btc lender has an LTV of 0.7 so for a bitcoin at 20000$, we should be able to loan 42k$ for 3 bitcoins
    // Therefore the method call should fail
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(42001),
        ))
        .should_panic(Error::AssertFailed(format!(
            "You need to provide at least {} tokens to loan {}",
            dec!("3.000071428571428571"),
            dec!(42001)
        )))
        .run();
}

#[test]
fn test_repay_single_loan() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // Take loan
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(30000),
        ))
        .run();

    // Change time
    let new_time = Instant::new(0).add_days(31).unwrap();
    test_env.set_current_time(new_time);

    // Take another loan to repay first loan
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(1),
            dec!(10000),
        ))
        .run();

    // Repay loan with interests
    let interest_paid = dec!(93);
    test_env
        .call_method(IssuerMethods::RepayLoans(
            dec!(30000) + interest_paid.clone(),
            vec!["#0#".to_string()],
        ))
        .run();

    // Check that the issuer has the right state
    issuer_state.update();
    let mut lenders = HashMap::new();
    // After repaying the loan, the only collateral left is from the second loan
    let btc_lender = LenderState::from(
        Decimal::ONE,
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    let mut reserves = HashMap::new();
    reserves.insert(
        test_env.get_resource(STABLECOIN_NAME).clone(),
        interest_paid,
    );
    issuer_state.assert_state_is(&reserves, &lenders, 2, 0);

    assert_current_has_no_loan_id(&test_env, "#0#");
    assert_current_has_loan(
        &test_env,
        "#1#",
        "btc",
        dec!(1),
        dec!(10000),
        2678400,
        dec!("0.0001"),
    );
}

#[test]
fn test_repay_single_loan_not_enough_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // Take loan
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(30000),
        ))
        .run();

    // Change time
    let new_time = Instant::new(0).add_days(31).unwrap();
    test_env.set_current_time(new_time);

    // Take another loan to repay first loan
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(1),
            dec!(10000),
        ))
        .run();

    // Repay loan with not enough interests
    test_env
        .call_method(IssuerMethods::RepayLoans(
            dec!(30000) + dec!(92),
            vec!["#0#".to_string()],
        ))
        .should_panic(Error::AssertFailed(
            "You need to provide 30093 stablecoins to repay your loan".to_string(),
        ))
        .run();
}

#[test]
fn test_repay_multiple_loans() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // Take multiple loans
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(30000),
        ))
        .run();
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(30000),
        ))
        .run();

    // Change time
    let new_time = Instant::new(0).add_days(31).unwrap();
    test_env.set_current_time(new_time);

    // Take another loan to repay previous loans
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(1),
            dec!(10000),
        ))
        .run();

    // Repay loan with interests
    test_env
        .call_method(IssuerMethods::RepayLoans(
            dec!(60186),
            vec!["#0#".to_string(), "#1#".to_string()],
        ))
        .run();

    // Check that the issuer has the right state
    issuer_state.update();
    let mut lenders = HashMap::new();
    // After repaying the loan, the only collateral left is from the second loan
    let btc_lender = LenderState::from(
        Decimal::ONE,
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    let mut reserves = HashMap::new();
    reserves.insert(test_env.get_resource(STABLECOIN_NAME).clone(), dec!(186));
    issuer_state.assert_state_is(&reserves, &lenders, 3, 0);

    assert_current_has_no_loan_id(&test_env, "#0#");
    assert_current_has_no_loan_id(&test_env, "#1#");
    assert_current_has_loan(
        &test_env,
        "#1#",
        "btc",
        dec!(1),
        dec!(10000),
        2678400,
        dec!("0.0001"),
    );
}

#[test]
fn test_add_collateral() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // The default btc lender has an LTV of 0.7 so for a bitcoin at 20000$, we should be able to loan 42k$ for 3 bitcoins
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(42000),
        ))
        .run();

    // We increase collateral
    test_env
        .call_method(IssuerMethods::AddCollateral(
            "btc".to_string(),
            dec!(4),
            "#0#".to_string(),
        ))
        .run();

    // Check that the issuer has the right state
    issuer_state.update();
    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        dec!(7),
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    issuer_state.assert_state_is(&HashMap::new(), &lenders, 1, 0);

    // Check that the account has the right amount of stablecoins and the right NFR
    assert_eq!(
        test_env.amount_owned_by_current(STABLECOIN_NAME),
        dec!(42000)
    );
    assert_current_has_loan(
        &test_env,
        "#0#",
        "btc",
        dec!(7),
        dec!(42000),
        0,
        dec!("0.0001"),
    );
}

#[test]
fn test_add_collateral_wrong_token_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // The default btc lender has an LTV of 0.7 so for a bitcoin at 20000$, we should be able to loan 42k$ for 3 bitcoins
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(42000),
        ))
        .run();

    // We try increasing collateral with the wrong token
    test_env.create_fixed_supply_token("eth", dec!(10));
    test_env
        .call_method(IssuerMethods::AddCollateral(
            "eth".to_string(),
            dec!(4),
            "#0#".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "Please provide the right tokens to add as collateral".to_string(),
        ))
        .run();
}

#[test]
fn test_remove_collateral() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // The default btc lender has an LTV of 0.7 so for a bitcoin at 20000$, we should be able to loan 42k$ for 3 bitcoins
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(28000),
        ))
        .run();

    // We should be able to remove 1.18 *2000btc of collateral
    test_env
        .call_method(IssuerMethods::RemoveCollateral(
            dec!("1.18"),
            "#0#".to_string(),
        ))
        .run();

    // Check that the issuer has the right state
    issuer_state.update();
    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        dec!("1.82"),
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    issuer_state.assert_state_is(&HashMap::new(), &lenders, 1, 0);

    assert_current_has_loan(
        &test_env,
        "#0#",
        "btc",
        dec!("1.82"),
        dec!(28000),
        0,
        dec!("0.0001"),
    );
}

#[test]
fn test_remove_collateral_too_much_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // The default btc lender has an LTV of 0.7 so for a bitcoin at 20000$, we should be able to loan 42k$ for 3 bitcoins
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(28000),
        ))
        .run();

    // We can remove a maximum of 1.18 btc of collateral
    test_env
        .call_method(IssuerMethods::RemoveCollateral(
            dec!("1.180000000000000001"),
            "#0#".to_string(),
        ))
        .should_panic(Error::AssertFailed(
            "Cannot remove 1.180000000000000001 because it would make the loan liquidatable"
                .to_string(),
        ))
        .run();
}

#[test]
fn test_remove_collateral_after_time_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(20000));

    // The default btc lender has an LTV of 0.7 so for a bitcoin at 20000$, we should be able to loan 42k$ for 3 bitcoins
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(3),
            dec!(28000),
        ))
        .run();

    // Change time
    let new_time = Instant::new(0).add_days(31).unwrap();
    test_env.set_current_time(new_time);

    // Now removing 1.18 btc collateral does not work because of accrued interests
    test_env
        .call_method(IssuerMethods::RemoveCollateral(dec!(1), "#0#".to_string()))
        .should_panic(Error::AssertFailed(
            "Cannot remove 1.18 because it would make the loan liquidatable".to_string(),
        ));
}

#[test]
fn test_partial_liquidation() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(10000));

    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(1),
            dec!(6000),
        ))
        .run();

    // Liquidation threshold is at 1.3 hence the loan can be partially liquidated at 1*price/6000 = 1.3
    // => price = 7800
    set_oracle_price(&mut test_env, "btc", dec!(7000));

    // Take loan to liquidate first one
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(10),
            dec!(10000),
        ))
        .run();

    test_env
        .call_method(IssuerMethods::Liquidate(dec!(616), "#0#".to_string()))
        .run();

    // Check that the issuer has the right state
    issuer_state.update();
    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        dec!("10.906892382103990327"),
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);

    let reserves = HashMap::new();
    issuer_state.assert_state_is(&reserves, &lenders, 2, 0);

    assert_current_has_loan(
        &test_env,
        "#0#",
        "btc",
        dec!("0.906892382103990327"),
        dec!("5384"),
        0,
        dec!("0.0001"),
    );
    assert_current_has_loan(
        &test_env,
        "#1#",
        "btc",
        dec!(10),
        dec!(10000),
        0,
        dec!("0.0001"),
    );
}

#[test]
fn test_partial_liquidation_too_much_sent() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(10000));

    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(1),
            dec!(6000),
        ))
        .run();

    // Liquidation threshold is at 1.3 hence the loan can be partially liquidated at 1*price/6000 = 1.3
    // => liquidation price is 7800
    set_oracle_price(&mut test_env, "btc", dec!(7000));

    // Take loan to liquidate first one
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(10),
            dec!(10000),
        ))
        .run();

    // The maximum amount that we can liquidate is ~0.25 btc
    // We check that we cannot liquidate more
    test_env
        .call_method(IssuerMethods::Liquidate(dec!(10000), "#0#".to_string()))
        .run();

    // Check that the issuer has the right state
    issuer_state.update();
    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        dec!("10.695154559476600963"),
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);

    let reserves = HashMap::new();
    issuer_state.assert_state_is(&reserves, &lenders, 2, 0);

    assert_current_has_loan(
        &test_env,
        "#0#",
        "btc",
        dec!("0.695154559476600963"),
        dec!("3368.825942078912358"),
        0,
        dec!("0.0001"),
    );
    assert_current_has_loan(
        &test_env,
        "#1#",
        "btc",
        dec!(10),
        dec!(10000),
        0,
        dec!("0.0001"),
    );
}

#[test]
fn test_partial_liquidation_after_time() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(10000));

    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(1),
            dec!(6000),
        ))
        .run();

    // Change time
    let new_time = Instant::new(0).add_days(31).unwrap();
    test_env.set_current_time(new_time);

    // Interests are at 18.6$ therefore liquidation threshold price is at 1.3*6018.6 = 7824.18
    set_oracle_price(&mut test_env, "btc", dec!(7824));

    // Take loan to liquidate first one
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(10),
            dec!(10000),
        ))
        .run();
    test_env
        .call_method(IssuerMethods::Liquidate(dec!(1), "#0#".to_string()))
        .run();

    // Check that the issuer has the right state
    issuer_state.update();
    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        dec!("10.99983387600505017"),
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    let reserves = HashMap::new();
    issuer_state.assert_state_is(&reserves, &lenders, 2, 0);

    assert_current_has_loan(
        &test_env,
        "#0#",
        "btc",
        dec!("0.99983387600505017"),
        dec!("5999"),
        0,
        dec!("0.0001"),
    );
    assert_current_has_loan(
        &test_env,
        "#1#",
        "btc",
        dec!(10),
        dec!(10000),
        2678400,
        dec!("0.0001"),
    );
}

#[test]
fn test_full_liquidation_after_time() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(10000));

    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(1),
            dec!(6000),
        ))
        .run();

    // Change time
    let new_time = Instant::new(0).add_days(31).unwrap();
    test_env.set_current_time(new_time);

    // Interests are at 18.6$ therefore, full liquidation price happens for price < 6018.6
    set_oracle_price(&mut test_env, "btc", dec!(6018));

    // Take loan to liquidate first one
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(10),
            dec!(10000),
        ))
        .run();
    test_env
        .call_method(IssuerMethods::Liquidate(dec!(10000), "#0#".to_string()))
        .run();

    // Check that the issuer has the right state
    issuer_state.update();
    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        dec!("10"),
        dec!("0.7"),
        dec!("0.0001"),
        dec!("1.3"),
        dec!("0.1"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    let mut reserves = HashMap::new();
    reserves.insert(
        test_env.get_resource("btc").clone(),
        dec!("0.000299102691924227"),
    );
    issuer_state.assert_state_is(&reserves, &lenders, 2, 0);

    assert_current_has_loan(
        &test_env,
        "#0#",
        "btc",
        dec!("0"),
        dec!("0"),
        0,
        dec!("0.0001"),
    );
    assert_current_has_loan(
        &test_env,
        "#1#",
        "btc",
        dec!(10),
        dec!(10000),
        2678400,
        dec!("0.0001"),
    );
}

#[test]
fn test_liquidate_threshold_not_hit_fails() {
    let (mut test_env, _) = instantiate();

    new_default_lender(&mut test_env, "btc");
    set_oracle_price(&mut test_env, "btc", dec!(15000));

    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(1),
            dec!(10000),
        ))
        .run();

    // Liquidation threshold is at 13000
    set_oracle_price(&mut test_env, "btc", dec!(13001));

    // Take loan to liquidate first one
    test_env
        .call_method(IssuerMethods::TakeLoan(
            "btc".to_string(),
            dec!(10),
            dec!(10000),
        ))
        .run();
    test_env
        .call_method(IssuerMethods::Liquidate(dec!(10000), "#0#".to_string()))
        .should_panic(Error::AssertFailed(
            "Cannot liquidate this loan: the collateralization ratio is 1.3001 >= 1.3".to_string(),
        ))
        .run();
}

#[test]
fn test_change_lender_parameter() {
    let (mut test_env, mut issuer_state) = instantiate();

    new_default_lender(&mut test_env, "btc");

    // Change the parameters
    test_env
        .call_method(IssuerMethods::ChangeLenderParameters(
            "btc".to_string(),
            dec!("0.5"),
            dec!("0.5"),
            dec!(2),
            dec!("0.3"),
        ))
        .run();

    issuer_state.update();
    let mut lenders = HashMap::new();
    let btc_lender = LenderState::from(
        Decimal::ZERO,
        dec!("0.5"),
        dec!("0.5"),
        dec!(2),
        dec!("0.3"),
    );
    lenders.insert(test_env.get_resource("btc").clone(), btc_lender);
    issuer_state.assert_state_is(&HashMap::new(), &lenders, 0, 0);
}
