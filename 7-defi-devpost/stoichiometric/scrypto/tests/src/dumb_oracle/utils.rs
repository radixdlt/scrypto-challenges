use crate::dumb_oracle::dumb_oracle_sqrt::{DumbOracleBlueprint, DumbOracleMethods};
use scrypto::prelude::Decimal;
use sqrt::package::Package;
use sqrt::test_environment::TestEnvironment;

pub fn instantiate_oracle(test_env: &mut TestEnvironment) {
    let oracle_blueprint = Box::new(DumbOracleBlueprint {});
    let mut oracle_package = Package::new("src/dumb_oracle/package");
    oracle_package.add_blueprint("oracle_bp", oracle_blueprint);
    test_env.publish_package("oracle", oracle_package);
}

pub fn new_oracle(test_env: &mut TestEnvironment, token: &str) -> String {
    let oracle_component_name = format!("{}_component", token);

    match test_env.get_current_component_name() {
        None => {
            test_env.set_current_package("oracle");
            test_env.new_component(&oracle_component_name, "oracle_bp", vec![]);
        }
        Some(name) => {
            let component_name = String::from(name);
            let package_name = String::from(test_env.get_current_package_name().unwrap());
            test_env.set_current_package("oracle");
            test_env.new_component(&oracle_component_name, "oracle_bp", vec![]);
            test_env.set_current_package(&package_name);
            test_env.set_current_component(&component_name);
        }
    }

    oracle_component_name
}

pub fn set_oracle_price(test_env: &mut TestEnvironment, token: &str, new_price: Decimal) {
    let oracle_component_name = format!("{}_component", token);

    match test_env.get_current_component_name() {
        None => {
            test_env.set_current_package("oracle");
            test_env.set_current_component(&oracle_component_name);
            test_env
                .call_method(DumbOracleMethods::SetPrice(new_price))
                .run();
        }
        Some(name) => {
            let component_name = String::from(name);
            let package_name = String::from(test_env.get_current_package_name().unwrap());
            test_env.set_current_package("oracle");
            test_env.set_current_component(&oracle_component_name);
            test_env
                .call_method(DumbOracleMethods::SetPrice(new_price))
                .run();

            test_env.set_current_package(&package_name);
            test_env.set_current_component(&component_name);
        }
    }
}
