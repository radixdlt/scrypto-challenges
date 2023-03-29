use radix_engine_interface::model::FromPublicKey;
use radix_engine::types::*;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_perp() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().without_trace().build();

    // Create an account
    let (public_key, _private_key, account_component) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    let manifest1 = ManifestBuilder::new()
        .lock_fee(account_component, 10.into())
        .call_method(account_component, 
            "withdraw_by_amount", 
            args!(dec!(100), RADIX_TOKEN))
        .take_from_worktop(
            RADIX_TOKEN, 
            |builder, bucket| {
                builder.call_function(
                    package_address, 
                    "CupPerp", 
                    "instantiate_pair", 
                    args!(dec!("1000"), bucket))
        })
        .call_method(
            account_component,
            "deposit_batch",
            args!(ManifestExpression::EntireWorktop),
        )
        .build();

    let component = 
        test_runner.execute_manifest(
            manifest1, 
            vec![NonFungibleGlobalId::from_public_key(&public_key)])
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];

    let manifest2 = ManifestBuilder::new()
        .lock_fee(account_component, 10.into())
        .call_method(account_component, 
            "withdraw_by_amount", 
            args!(dec!(5), RADIX_TOKEN))
        .take_from_worktop_by_amount(
            5.into(),
            RADIX_TOKEN,
            |builder, bucket| {
                builder.call_method(
                    component,
                    "deposit",
                    args!(true, bucket)
                )
            })
        .call_method(
            account_component,
            "deposit_batch",
            args!(ManifestExpression::EntireWorktop)
        )
        .build();
    
    test_runner.execute_manifest(
        manifest2, 
        vec![NonFungibleGlobalId::from_public_key(&public_key)])
        .expect_commit();

    let manifest3 = ManifestBuilder::new()
        .lock_fee(account_component, 10.into())
        .call_method(component, "set_oracle", args!(dec!(1030)))
        .call_method(component, "update", args!())
        .call_method(component, "show_cups", args!())
        .build();

    let receipt = test_runner.execute_manifest(
        manifest3, 
        vec![NonFungibleGlobalId::from_public_key(&public_key)]);

    // apparently .output indexes starting from 1 into outputs
    // of ALL function calls, not just ones that return outputs
    let (up, down): (Decimal, Decimal) = receipt.output(3);
    receipt.expect_commit();
}
