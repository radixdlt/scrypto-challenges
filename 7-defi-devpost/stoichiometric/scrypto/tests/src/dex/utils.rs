use crate::dex::pool_state::PoolState;
use crate::dex::sqrt_implem::{RouterBlueprint, RouterMethods};
use crate::utils::{run_command, ADMIN_BADGE_NAME, POSITION_NAME, STABLECOIN_NAME};
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::math::Decimal;
use scrypto::prelude::dec;
use sqrt::manifest_call::ManifestCall;
use sqrt::method::Arg::{
    AccountAddressArg, ComponentAddressArg, DecimalArg, ResourceAddressArg, StringArg, U16,
};
use sqrt::package::Package;
use sqrt::test_environment::TestEnvironment;
use std::collections::HashMap;
use std::process::Command;

pub fn instantiate() -> TestEnvironment {
    let mut test_env = TestEnvironment::new();
    let router_blueprint = Box::new(RouterBlueprint {});
    let mut router_package = Package::new("../dex");
    router_package.add_blueprint("router_bp", router_blueprint);
    test_env.publish_package("router", router_package);
    test_env.create_fixed_supply_token("usd", dec!(10000000));
    test_env.create_fixed_supply_token("btc", dec!(10000000));
    test_env.create_fixed_supply_token(ADMIN_BADGE_NAME, Decimal::ONE);
    test_env.new_component(
        "router_comp",
        "router_bp",
        vec![
            ResourceAddressArg(ADMIN_BADGE_NAME.to_string()),
            ResourceAddressArg("usd".to_string()),
        ],
    );
    test_env
}

pub fn create_pool(
    test_env: &mut TestEnvironment,
    other: &str,
    initial_rate: Decimal,
    min_rate: Decimal,
    max_rate: Decimal,
) -> PoolState {
    test_env
        .call_method(RouterMethods::CreatePool(
            other.to_string(),
            initial_rate,
            min_rate,
            max_rate,
        ))
        .run();

    let mut pool_state: PoolState = PoolState::from(String::new(), String::new());

    let router_address = test_env.get_component("router_comp").unwrap();
    let other_address = test_env.get_resource(other).clone();

    let output = run_command(Command::new("resim").arg("show").arg(router_address));

    lazy_static! {
        static ref POOLS_LIST_RE: Regex =
            Regex::new(r#"Map<ResourceAddress, Tuple>\((.*), Tuple\(Own"#).unwrap();
    }

    let pools_list_cap = &POOLS_LIST_RE
        .captures(&output)
        .expect("Could not find pools list");
    let pools_list = &pools_list_cap[1];

    lazy_static! {
        static ref POOLS_RE: Regex = Regex::new(r#"ResourceAddress\("(\w*)"\)"#).unwrap();
    }

    for cap in POOLS_RE.captures_iter(pools_list) {
        let resource = String::from(&cap[1]);
        if resource == other_address {
            pool_state = PoolState::from(router_address.to_string(), other_address);
            break;
        }
    }

    pool_state.update();
    pool_state
}

pub fn add_liquidity_at_step<'a>(
    test_env: &'a mut TestEnvironment,
    amount_stable: Decimal,
    token_b: &'a str,
    amount_b: Decimal,
    step: u16,
    position_id: Option<String>,
) -> ManifestCall<'a> {
    let mut env_args = Vec::new();
    env_args.push((
        "caller_address".to_string(),
        AccountAddressArg(test_env.get_current_account_name().to_string()),
    ));
    env_args.push((
        "component_address".to_string(),
        ComponentAddressArg(test_env.get_current_component_name().unwrap().to_string()),
    ));
    env_args.push((
        "token_a_address".to_string(),
        ResourceAddressArg(STABLECOIN_NAME.to_string()),
    ));
    env_args.push(("token_a_amount".to_string(), DecimalArg(amount_stable)));
    env_args.push((
        "token_b_address".to_string(),
        ResourceAddressArg(token_b.to_string()),
    ));
    env_args.push(("token_b_amount".to_string(), DecimalArg(amount_b)));

    env_args.push(("step".to_string(), U16(step)));

    let manifest_name = match position_id {
        None => "add_liquidity_at_step_no_pos",
        Some(id) => {
            env_args.push((
                "position_address".to_string(),
                ResourceAddressArg(POSITION_NAME.to_string()),
            ));
            env_args.push(("position_id".to_string(), StringArg(id)));
            "add_liquidity_at_step_with_pos"
        }
    };

    test_env.call_custom_manifest(manifest_name, env_args)
}

pub fn add_liquidity_at_steps<'a>(
    test_env: &'a mut TestEnvironment,
    amount_stable: Decimal,
    other: &'a str,
    amount_other: Decimal,
    steps: Vec<(u16, Decimal, Decimal)>,
    position_id: Option<String>,
) -> ManifestCall<'a> {
    let mut env_args = Vec::new();
    env_args.push((
        "caller_address".to_string(),
        AccountAddressArg(test_env.get_current_account_name().to_string()),
    ));
    env_args.push((
        "component_address".to_string(),
        ComponentAddressArg(test_env.get_current_component_name().unwrap().to_string()),
    ));
    env_args.push((
        "stablecoin_address".to_string(),
        ResourceAddressArg("usd".to_string()),
    ));
    env_args.push(("stablecoin_amount".to_string(), DecimalArg(amount_stable)));
    env_args.push((
        "other_token".to_string(),
        ResourceAddressArg(other.to_string()),
    ));
    env_args.push(("other_token_amount".to_string(), DecimalArg(amount_other)));

    let mut step_string = String::new();

    for (step, stable_amount, other_amount) in steps {
        let new_string = format!(
            "Tuple({}u16, Decimal(\"{}\"), Decimal(\"{}\"))",
            step, stable_amount, other_amount
        );
        step_string = format!("{}{} ,", step_string, new_string);
    }
    step_string.pop();
    step_string.pop();

    env_args.push(("steps_string".to_string(), StringArg(step_string)));

    let manifest_name = match position_id {
        None => "add_liquidity_at_steps_no_pos",
        Some(id) => {
            env_args.push((
                "position_address".to_string(),
                ResourceAddressArg(POSITION_NAME.to_string()),
            ));
            env_args.push(("position_id".to_string(), StringArg(id)));
            "add_liquidity_at_steps_with_pos"
        }
    };

    test_env.call_custom_manifest(manifest_name, env_args)
}

pub fn add_liquidity<'a>(
    test_env: &'a mut TestEnvironment,
    amount_stable: Decimal,
    token_b: &'a str,
    amount_b: Decimal,
    rate: Decimal,
    position_id: Option<String>,
) -> ManifestCall<'a> {
    let mut env_args = Vec::new();
    env_args.push((
        "caller_address".to_string(),
        AccountAddressArg(test_env.get_current_account_name().to_string()),
    ));
    env_args.push((
        "component_address".to_string(),
        ComponentAddressArg(test_env.get_current_component_name().unwrap().to_string()),
    ));
    env_args.push((
        "token_a_address".to_string(),
        ResourceAddressArg("usd".to_string()),
    ));
    env_args.push(("token_a_amount".to_string(), DecimalArg(amount_stable)));
    env_args.push((
        "token_b_address".to_string(),
        ResourceAddressArg(token_b.to_string()),
    ));
    env_args.push(("token_b_amount".to_string(), DecimalArg(amount_b)));
    env_args.push(("rate".to_string(), DecimalArg(rate)));

    let manifest_name = match position_id {
        None => "add_liquidity_no_pos",
        Some(id) => {
            env_args.push((
                "position_address".to_string(),
                ResourceAddressArg(POSITION_NAME.to_string()),
            ));
            env_args.push(("position_id".to_string(), StringArg(id)));
            "add_liquidity_with_pos"
        }
    };

    test_env.call_custom_manifest(manifest_name, env_args)
}

pub fn assert_current_position(
    test_env: &TestEnvironment,
    token: &str,
    step_positions: &HashMap<u16, (Decimal, Decimal, Decimal)>,
) {
    let output = run_command(
        Command::new("resim")
            .arg("show")
            .arg(test_env.get_current_account_address()),
    );

    lazy_static! {
        static ref POSITIONS_RE: Regex = Regex::new(r#"NonFungible \{ id: NonFungibleLocalId\("(.*)"\), immutable_data: Tuple\(ResourceAddress\("(\w*)"\)\), mutable_data: Tuple\(Map<U16, Tuple>\((.*)\) \}"#).unwrap();
    }

    let mut position_found = false;
    for position_cap in POSITIONS_RE.captures_iter(&output) {
        let token_address = String::from(&position_cap[2]);
        let token = test_env.get_resource(token).clone();
        if token_address == token {
            position_found = true;
            assert_step_positions(&position_cap[3], step_positions);
        }
    }

    assert!(position_found);
}

fn assert_step_positions(
    output_str: &str,
    step_positions: &HashMap<u16, (Decimal, Decimal, Decimal)>,
) {
    lazy_static! {
        static ref STEP_POSITION_RE: Regex = Regex::new(r#"(\w*)u16, Tuple\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)"#).unwrap();
    }

    let mut new_hashmap = HashMap::new();

    for step_position_cap in STEP_POSITION_RE.captures_iter(output_str) {
        let step_id: u16 = String::from(&step_position_cap[1]).parse::<u16>().unwrap();
        let liquidity = Decimal::from(&step_position_cap[2]);
        let last_stable_fees_per_liq = Decimal::from(&step_position_cap[3]);
        let last_other_fees_per_liq = Decimal::from(&step_position_cap[4]);
        new_hashmap.insert(
            step_id,
            (liquidity, last_stable_fees_per_liq, last_other_fees_per_liq),
        );
    }

    assert!(
        new_hashmap.len() == step_positions.len()
            && new_hashmap.keys().all(|k| step_positions.contains_key(k))
    );

    for (key, value) in new_hashmap {
        let value_2 = step_positions.get(&key).unwrap();
        assert_eq!(value, *value_2);
    }
}

pub fn assert_no_positions(test_env: &TestEnvironment, token: &str) {
    let output = run_command(
        Command::new("resim")
            .arg("show")
            .arg(test_env.get_current_account_address()),
    );

    lazy_static! {
        static ref POSITIONS_RE: Regex = Regex::new(r#"NonFungible \{ id: NonFungibleLocalId\("(.*)"\), immutable_data: Tuple\(ResourceAddress\("(\w*)"\)\), mutable_data: Tuple\(Map<U16, Tuple>\((.*)\) \}"#).unwrap();
    }

    let mut position_found = false;
    for position_cap in POSITIONS_RE.captures_iter(&output) {
        let token_address = String::from(&position_cap[2]);
        let token = test_env.get_resource(token).clone();
        if token_address == token {
            position_found = true;
            break;
        }
    }

    assert!(!position_found);
}
