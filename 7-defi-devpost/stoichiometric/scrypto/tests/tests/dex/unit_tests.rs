use scrypto::prelude::{dec, Decimal};
use sqrt::error::Error;
use std::collections::HashMap;
use stoichiometric_tests::dex::pool_state::StepState;
use stoichiometric_tests::dex::sqrt_implem::RouterMethods;
use stoichiometric_tests::dex::utils::{
    add_liquidity, add_liquidity_at_step, add_liquidity_at_steps, assert_current_position,
    assert_no_positions, create_pool, instantiate,
};
use stoichiometric_tests::utils::POSITION_NAME;

#[test]
fn test_create_pool() {
    let mut test_env = instantiate();

    let pool_usd_btc = create_pool(&mut test_env, "btc", dec!(20000), dec!(100), dec!(100000));

    let mut pool_states = HashMap::new();
    pool_states.insert(
        50266,
        StepState::from(
            Decimal::ZERO,
            Decimal::ZERO,
            dec!(20000),
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
        ),
    );

    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );
}

#[test]
fn test_create_multiple_pools() {
    let mut test_env = instantiate();

    let pool_usd_btc = create_pool(&mut test_env, "btc", dec!(20000), dec!(100), dec!(100000));
    let mut pool_states = HashMap::new();
    pool_states.insert(
        50266,
        StepState::from(
            Decimal::ZERO,
            Decimal::ZERO,
            dec!(20000),
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
        ),
    );

    pool_usd_btc.assert_state_is(
        dec!("1.000105411144423293"),
        50266,
        dec!(100),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );

    test_env.create_fixed_supply_token("eth", dec!(1000000));
    let pool_usd_eth = create_pool(&mut test_env, "eth", dec!(1700), dec!(10), dec!(20000));
    let mut pool_states = HashMap::new();
    pool_states.insert(
        44280,
        StepState::from(
            Decimal::ZERO,
            Decimal::ZERO,
            dec!(1700),
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
        ),
    );

    pool_usd_eth.assert_state_is(
        dec!("1.000115989063276095"),
        44280,
        dec!(10),
        pool_states,
        Decimal::ZERO,
        Decimal::ZERO,
    );
}

#[test]
fn test_create_pool_with_stablecoin_fail() {
    let mut test_env = instantiate();

    test_env
        .call_method(RouterMethods::CreatePool(
            "usd".to_string(),
            Decimal::ONE,
            dec!("0.0001"),
            dec!(2),
        ))
        .should_panic(Error::AssertFailed(
            "Two pools cannot trade the same token".to_string(),
        ))
        .run();
}

#[test]
fn test_create_pool_already_exists_fail() {
    let mut test_env = instantiate();
    create_pool(&mut test_env, "btc", dec!(20000), dec!(100), dec!(100000));

    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            dec!(20000),
            dec!(100),
            dec!(100000),
        ))
        .should_panic(Error::AssertFailed(
            "A pool trading these tokens already exists".to_string(),
        ))
        .run();
}

#[test]
fn test_create_pool_min_rate_zero_fail() {
    let mut test_env = instantiate();
    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            Decimal::ZERO,
            Decimal::ZERO,
            dec!(0),
        ))
        .should_panic(Error::AssertFailed(
            "The minimum rate should be positive".to_string(),
        ))
        .run();
}

#[test]
fn create_pool_max_rate_less_than_min_fail() {
    let mut test_env = instantiate();
    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            Decimal::ZERO,
            Decimal::ONE,
            dec!("0.5"),
        ))
        .should_panic(Error::AssertFailed(
            "The maximum rate should be greater than the minimum rate".to_string(),
        ))
        .run();
}

#[test]
fn create_pool_initial_rate_less_than_min_fail() {
    let mut test_env = instantiate();
    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            dec!("0.5"),
            Decimal::ONE,
            dec!(2),
        ))
        .should_panic(Error::AssertFailed(
            "The initial rate should be included in the given rate range".to_string(),
        ))
        .run();
}

#[test]
fn create_pool_initial_rate_greater_than_max_fail() {
    let mut test_env = instantiate();
    test_env
        .call_method(RouterMethods::CreatePool(
            "btc".to_string(),
            dec!(3),
            Decimal::ONE,
            dec!(2),
        ))
        .should_panic(Error::AssertFailed(
            "The initial rate should be included in the given rate range".to_string(),
        ))
        .run();
}
