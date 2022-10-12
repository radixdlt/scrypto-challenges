use radix_engine::ledger::*;
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_instantiate_dao() {
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(true, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    let package_address = test_runner.compile_and_publish(this_package!());

    let instantiate_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("100")))
        .call_function(package_address, "Dao", "new", args!())
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    let receipt = test_runner.execute_manifest(instantiate_manifest, vec![public_key.into()]);

    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
}
