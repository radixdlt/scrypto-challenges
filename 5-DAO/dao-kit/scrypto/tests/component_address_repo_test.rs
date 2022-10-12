use radix_engine::ledger::*;
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::{ManifestBuilder};
use dao_kit::component_address_repo::ComponentAddressLookup;


#[test]
fn test_component_address_repo() {
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);
    let (public_key, _private_key, account_component) = test_runner.new_account();
    let package_address = test_runner.compile_and_publish(this_package!());

    // Instantiate the repo
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_function(package_address, "ComponentAddressRepo", "instantiate_global", args!())
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);
    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
    let repo_component: ComponentAddress = receipt.output(1);

    // Choose some address to do the testing with
    let some_address = account_component;

    // Insert that address into the repo
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(repo_component, "create_lookup", args!(some_address))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);
    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
    let some_lookup: ComponentAddressLookup = receipt.output(1);

    // Take the ID that has been returned from the repo and use it to look up the component address
    // that was just inserted
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(repo_component, "lookup_address", args!(some_lookup))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);
    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
    let some_address_looked_up: ComponentAddress = receipt.output(1);
    assert_eq!(some_address_looked_up, some_address);

    // Insert the same component address again and assert that the same ID as before is returned,
    // i.e. the component address is only stored once
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(repo_component, "create_lookup", args!(some_address))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);
    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
    let some_lookup_2: ComponentAddressLookup = receipt.output(1);
    assert_eq!(some_lookup_2, some_lookup);
}
