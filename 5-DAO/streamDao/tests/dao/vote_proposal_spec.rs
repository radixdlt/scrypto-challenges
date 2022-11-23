use radix_engine::ledger::*;
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_vote_proposal() {
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

    let instantiation_receipt =
        test_runner.execute_manifest(instantiate_manifest, vec![public_key.into()]);

    let component = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];

    let proposal_resource_address = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[1];

    let proposal_id: u64 = 20u64;
    let mut proposal_bt_1: BTreeSet<NonFungibleId> = BTreeSet::new();
    proposal_bt_1.insert(NonFungibleId::from_u64(proposal_id));

    let membership_id_1: u64 = 14u64;

    let deposit_proposal_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("100")))
        .call_method(
            component,
            "create_proposal_set_id",
            args!(
                proposal_id,
                vec!["DAN-COIN", "COIN-DAN", "DAN-DAN"],
                String::from("https://streamdaotest/ipfs/a5XPj3N75k4ZUNcFLGngSj72kYZBH"),
                NonFungibleId::from_u64(100u64),
                20u64,
                50u64
            ),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .withdraw_from_account_by_ids(&proposal_bt_1, proposal_resource_address, account_component)
        .take_from_worktop_by_ids(
            &proposal_bt_1,
            proposal_resource_address,
            |build, proposal| {
                build.call_method(component, "deposit_proposal", args!(Bucket(proposal)))
            },
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .call_method(
            component,
            "vote_proposal",
            args!(
                NonFungibleId::from_u64(proposal_id),
                NonFungibleId::from_u64(membership_id_1),
                ("DAN-COIN".to_string(), dec!("10"))
            ),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.set_current_epoch(22);
    let receipt = test_runner.execute_manifest(deposit_proposal_manifest, vec![public_key.into()]);

    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
}

#[test]
fn test_error_already_voted() {
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

    let instantiation_receipt =
        test_runner.execute_manifest(instantiate_manifest, vec![public_key.into()]);

    let component = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];

    let proposal_resource_address = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[1];

    let proposal_id: u64 = 20u64;
    let mut proposal_bt_1: BTreeSet<NonFungibleId> = BTreeSet::new();
    proposal_bt_1.insert(NonFungibleId::from_u64(proposal_id));

    let membership_id_1: u64 = 14u64;

    let vote_proposal_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("100")))
        .call_method(
            component,
            "create_proposal_set_id",
            args!(
                proposal_id,
                vec!["DAN-COIN", "COIN-DAN", "DAN-DAN"],
                String::from("https://streamdaotest/ipfs/a5XPj3N75k4ZUNcFLGngSj72kYZBH"),
                NonFungibleId::from_u64(100u64),
                20u64,
                50u64
            ),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .withdraw_from_account_by_ids(&proposal_bt_1, proposal_resource_address, account_component)
        .take_from_worktop_by_ids(
            &proposal_bt_1,
            proposal_resource_address,
            |build, proposal| {
                build.call_method(component, "deposit_proposal", args!(Bucket(proposal)))
            },
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .call_method(
            component,
            "vote_proposal",
            args!(
                NonFungibleId::from_u64(proposal_id),
                NonFungibleId::from_u64(membership_id_1),
                ("DAN-COIN".to_string(), dec!("10"))
            ),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .call_method(
            component,
            "vote_proposal",
            args!(
                NonFungibleId::from_u64(proposal_id),
                NonFungibleId::from_u64(membership_id_1),
                ("COIN-DAN".to_string(), dec!("20"))
            ),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.set_current_epoch(22);
    let receipt = test_runner.execute_manifest(vote_proposal_manifest, vec![public_key.into()]);

    println!("{:?}\n", receipt);
    receipt.expect_commit_failure();
}

#[test]
fn test_error_vote_proposal_choice_does_not_exist() {
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

    let instantiation_receipt =
        test_runner.execute_manifest(instantiate_manifest, vec![public_key.into()]);

    let component = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];

    let proposal_resource_address = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[1];

    let proposal_id: u64 = 20u64;
    let mut proposal_bt_1: BTreeSet<NonFungibleId> = BTreeSet::new();
    proposal_bt_1.insert(NonFungibleId::from_u64(proposal_id));

    let membership_id_1: u64 = 14u64;

    let deposit_proposal_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("100")))
        .call_method(
            component,
            "create_proposal_set_id",
            args!(
                proposal_id,
                vec!["DAN-COIN", "COIN-DAN", "DAN-DAN"],
                String::from("https://streamdaotest/ipfs/a5XPj3N75k4ZUNcFLGngSj72kYZBH"),
                NonFungibleId::from_u64(100u64),
                20u64,
                50u64
            ),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .withdraw_from_account_by_ids(&proposal_bt_1, proposal_resource_address, account_component)
        .take_from_worktop_by_ids(
            &proposal_bt_1,
            proposal_resource_address,
            |build, proposal| {
                build.call_method(component, "deposit_proposal", args!(Bucket(proposal)))
            },
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .call_method(
            component,
            "vote_proposal",
            args!(
                NonFungibleId::from_u64(proposal_id),
                NonFungibleId::from_u64(membership_id_1),
                ("DAN".to_string(), dec!("10"))
            ),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.set_current_epoch(22);
    let receipt = test_runner.execute_manifest(deposit_proposal_manifest, vec![public_key.into()]);

    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
}
