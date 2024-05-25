/*
use radix_engine_interface::prelude::*;
use scrypto::prelude::ManifestCustomValue::Address;
use scrypto::this_package;
use scrypto_test::prelude::*;
use scrypto_unit::*;
use super_iyo::test_bindings::*;

#[test]
fn test_hello() {
    // Setup the environment
    let mut test_runner = TestRunnerBuilder::new().build();

    // Create an account
    let (public_key, _private_key, account) = test_runner.new_allocated_account();

    
    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Test the `instantiate_hello` function.
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "Super",
            "new",
            manifest_args!(public_key, 1u64),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    let component = receipt.expect_commit(true).new_component_addresses()[0];


    let manifest = ManifestBuilder::new()
        .call_method(component, "deposit", manifest_args!("{}:100", XRD))
        .call_method(
            account,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
    
    // Test the `free_token` method.
    let manifest = ManifestBuilder::new()
        .call_method(component, "deposit", manifest_args!("{}:100", XRD))
        .call_method(
            account,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
}



*/

