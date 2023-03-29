use crate::dumb_oracle::utils::{instantiate_oracle, new_oracle};
use crate::stablecoin::issuer_state::IssuerState;
use crate::stablecoin::sqrt_implem::{IssuerBlueprint, IssuerMethods};
use crate::utils::{run_command, ADMIN_BADGE_NAME, STABLECOIN_NAME};
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::prelude::{dec, Decimal};
use sqrt::method::Arg::{FungibleBucketArg, ResourceAddressArg};
use sqrt::package::Package;
use sqrt::test_environment::TestEnvironment;
use std::process::Command;

pub fn instantiate() -> (TestEnvironment, IssuerState) {
    let mut test_env = TestEnvironment::new();
    test_env.create_fixed_supply_token(ADMIN_BADGE_NAME, dec!(2));

    test_env.create_fixed_supply_token("btc", dec!(10000000));
    test_env.create_mintable_token(STABLECOIN_NAME, ADMIN_BADGE_NAME);

    let issuer_blueprint = Box::new(IssuerBlueprint {});
    let mut issuer_package = Package::new("../stablecoin");
    issuer_package.add_blueprint("issuer_bp", issuer_blueprint);
    test_env.publish_package("issuer", issuer_package);
    test_env.new_component(
        "issuer_comp",
        "issuer_bp",
        vec![
            ResourceAddressArg(ADMIN_BADGE_NAME.to_string()),
            FungibleBucketArg(ADMIN_BADGE_NAME.to_string(), Decimal::ONE),
            ResourceAddressArg(STABLECOIN_NAME.to_string()),
        ],
    );

    instantiate_oracle(&mut test_env);

    let issuer_address = test_env.get_component("issuer_comp").unwrap();
    let mut issuer_state = IssuerState::from(issuer_address.to_string());
    issuer_state.update();

    (test_env, issuer_state)
}

pub fn new_default_lender(test_env: &mut TestEnvironment, token: &str) {
    let component_name = new_oracle(test_env, token);

    test_env
        .call_method(IssuerMethods::NewLender(
            token.to_string(),
            dec!("0.7"),
            dec!("0.0001"),
            dec!("1.3"),
            dec!("0.1"),
            component_name,
        ))
        .run();
}

pub fn assert_current_has_loan(
    test_env: &TestEnvironment,
    loan_id: &str,
    collateral_token: &str,
    collateral_amount: Decimal,
    amount_lent: Decimal,
    loan_date: i64,
    interest_rate: Decimal,
) {
    let current_account = test_env.get_current_account_address();
    let output = run_command(Command::new("resim").arg("show").arg(current_account));

    lazy_static! {
        static ref LOAN_RE: Regex = Regex::new(r#"NonFungible \{ id: NonFungibleLocalId\("(.*)"\), immutable_data: Tuple\(ResourceAddress\("(\w*)"\), (\w*)i64, Decimal\("([\d.]*)"\)\), mutable_data: Tuple\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\) \}"#).unwrap();
    }

    for loan_capture in LOAN_RE.captures_iter(&output) {
        if loan_id.to_string() == String::from(&loan_capture[1]) {
            let collateral_token_found = String::from(&loan_capture[2]);
            let loan_date_found = String::from(&loan_capture[3]).parse::<i64>().unwrap();
            let interest_rate_found = Decimal::from(&loan_capture[4]);
            let collateral_amount_found = Decimal::from(&loan_capture[5]);
            let amount_lent_found = Decimal::from(&loan_capture[6]);

            assert_eq!(
                test_env.get_resource(collateral_token).clone(),
                collateral_token_found
            );
            assert_eq!(collateral_amount, collateral_amount_found);
            assert_eq!(amount_lent, amount_lent_found);
            assert_eq!(loan_date, loan_date_found);
            assert_eq!(interest_rate, interest_rate_found);

            return;
        }
    }
}

pub fn assert_current_has_no_loan_id(test_env: &TestEnvironment, loan_id: &str) {
    let current_account = test_env.get_current_account_address();
    let output = run_command(Command::new("resim").arg("show").arg(current_account));

    lazy_static! {
        static ref LOAN_RE: Regex = Regex::new(r#"NonFungible \{ id: NonFungibleLocalId\("(.)*"\), immutable_data: Tuple\(ResourceAddress\("(\w*)"\), (\w*)i64, Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\), mutable_data: Tuple\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\) \}"#).unwrap();
    }

    for loan_capture in LOAN_RE.captures_iter(&output) {
        let loan_id_found = &loan_capture[1];
        assert_ne!(loan_id_found.to_string(), loan_id.to_string());
    }
}
