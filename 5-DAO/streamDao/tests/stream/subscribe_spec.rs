use radix_engine::ledger::*;
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_subscribe() {
    let mut store: TypedInMemorySubstateStore = TypedInMemorySubstateStore::with_bootstrap();

    let mut test_runner: TestRunner<TypedInMemorySubstateStore> = TestRunner::new(true, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();
    let (public_key_2, _private_key_2, account_component_2) = test_runner.new_account();

    let package_address = test_runner.compile_and_publish(this_package!());

    let subscription_price: Decimal = dec!("20");
    let create_channel_price: Decimal = dec!("50");
    let amount_rewards_subscription: Decimal = dec!("15");
    let amount_rewards_creating_channel: Decimal = dec!("15");
    let platform_fee: Decimal = dec!("5");

    let instantiate_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .call_function(
            package_address,
            "Streamdao",
            "instantiate_streamdao",
            args!(
                subscription_price,
                create_channel_price,
                amount_rewards_subscription,
                amount_rewards_creating_channel,
                platform_fee
            ),
        )
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

    let mint_membership_address = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[1];

    let membership_id_1: u64 = 95u64;

    let new_user_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_1, "Dan".to_string()),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_user_manifest, vec![public_key.into()]);

    let mut membership_1_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();

    membership_1_bt_id.insert(NonFungibleId::from_u64(membership_id_1));

    let channel_id: String = String::from("1e234ewerwer56789ewrwe");

    let new_channel_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .withdraw_from_account_by_amount(dec!("100"), RADIX_TOKEN, account_component)
        .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |build, xrd_bucket_id| {
            build
                .withdraw_from_account_by_ids(
                    &membership_1_bt_id,
                    mint_membership_address,
                    account_component,
                )
                .take_from_worktop_by_ids(
                    &membership_1_bt_id,
                    mint_membership_address,
                    |build, user_nft_bucket_id| {
                        build.call_method(
                            component,
                            "new_channel_set_id",
                            args!(
                                Bucket(xrd_bucket_id),
                                Bucket(user_nft_bucket_id),
                                channel_id,
                                "fuserleer".to_string()
                            ),
                        )
                    },
                )
        })
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_channel_manifest, vec![public_key.into()]);

    let membership_id_2: u64 = 30u64;

    let new_membership_2_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("10")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_2, "Chico".to_string()),
        )
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_membership_2_manifest, vec![public_key_2.into()]);

    let mut membership_2_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();
    membership_2_bt_id.insert(NonFungibleId::from_u64(membership_id_2));

    let subscribe_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("10")))
        .withdraw_from_account_by_amount(dec!("100"), RADIX_TOKEN, account_component_2)
        .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |build, xrd_bucket_id| {
            build
                .withdraw_from_account_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    account_component_2,
                )
                .take_from_worktop_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    |build, membership_bucket_id| {
                        build.call_method(
                            component,
                            "subscribe",
                            args!(
                                Bucket(xrd_bucket_id),
                                Bucket(membership_bucket_id),
                                channel_id
                            ),
                        )
                    },
                )
        })
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    let recept = test_runner.execute_manifest(subscribe_manifest, vec![public_key_2.into()]);

    println!("{:?\n}", recept);
    recept.expect_commit_success();
}

#[test]
fn test_resubscribe() {
    let mut store: TypedInMemorySubstateStore = TypedInMemorySubstateStore::with_bootstrap();

    let mut test_runner: TestRunner<TypedInMemorySubstateStore> = TestRunner::new(true, &mut store);

    test_runner.set_current_epoch(10);

    let (public_key, _private_key, account_component) = test_runner.new_account();
    let (public_key_2, _private_key_2, account_component_2) = test_runner.new_account();

    let package_address = test_runner.compile_and_publish(this_package!());

    let subscription_price: Decimal = dec!("20");
    let create_channel_price: Decimal = dec!("50");
    let amount_rewards_subscription: Decimal = dec!("15");
    let amount_rewards_creating_channel: Decimal = dec!("15");
    let platform_fee: Decimal = dec!("5");

    let instantiate_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .call_function(
            package_address,
            "Streamdao",
            "instantiate_streamdao",
            args!(
                subscription_price,
                create_channel_price,
                amount_rewards_subscription,
                amount_rewards_creating_channel,
                platform_fee
            ),
        )
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

    let mint_membership_address = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[1];

    let membership_id_1: u64 = 95u64;

    let new_user_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_1, "Dan".to_string()),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_user_manifest, vec![public_key.into()]);

    let mut membership_1_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();

    membership_1_bt_id.insert(NonFungibleId::from_u64(membership_id_1));

    let channel_id: String = String::from("2314134n3214l32n14134k");

    let new_channel_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .withdraw_from_account_by_amount(dec!("100"), RADIX_TOKEN, account_component)
        .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |build, xrd_bucket_id| {
            build
                .withdraw_from_account_by_ids(
                    &membership_1_bt_id,
                    mint_membership_address,
                    account_component,
                )
                .take_from_worktop_by_ids(
                    &membership_1_bt_id,
                    mint_membership_address,
                    |build, user_nft_bucket_id| {
                        build.call_method(
                            component,
                            "new_channel_set_id",
                            args!(
                                Bucket(xrd_bucket_id),
                                Bucket(user_nft_bucket_id),
                                channel_id,
                                "fuserleer".to_string()
                            ),
                        )
                    },
                )
        })
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_channel_manifest, vec![public_key.into()]);

    let membership_id_2: u64 = 30u64;

    let new_membership_2_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("10")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_2, "Chico".to_string()),
        )
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_membership_2_manifest, vec![public_key_2.into()]);

    let mut membership_2_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();
    membership_2_bt_id.insert(NonFungibleId::from_u64(membership_id_2));

    test_runner.set_current_epoch(30);

    let subscribe_manifest_1 = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("10")))
        .withdraw_from_account_by_amount(dec!("100"), RADIX_TOKEN, account_component_2)
        .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |build, xrd_bucket_id| {
            build
                .withdraw_from_account_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    account_component_2,
                )
                .take_from_worktop_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    |build, membership_bucket_id| {
                        build.call_method(
                            component,
                            "subscribe",
                            args!(
                                Bucket(xrd_bucket_id),
                                Bucket(membership_bucket_id),
                                channel_id
                            ),
                        )
                    },
                )
        })
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(subscribe_manifest_1, vec![public_key_2.into()]);

    test_runner.set_current_epoch(40);

    let resubscribe_manifest_1 = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("10")))
        .withdraw_from_account_by_amount(dec!("100"), RADIX_TOKEN, account_component_2)
        .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |build, xrd_bucket_id| {
            build
                .withdraw_from_account_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    account_component_2,
                )
                .take_from_worktop_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    |build, membership_bucket_id| {
                        build.call_method(
                            component,
                            "resubscribe",
                            args!(
                                Bucket(xrd_bucket_id),
                                Bucket(membership_bucket_id),
                                channel_id
                            ),
                        )
                    },
                )
        })
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.set_current_epoch(100);
    let recept = test_runner.execute_manifest(resubscribe_manifest_1, vec![public_key_2.into()]);

    println!("{:?\n}", recept);
    recept.expect_commit_success();
}

#[test]
fn test_error_subscription_already_active() {
    let mut store: TypedInMemorySubstateStore = TypedInMemorySubstateStore::with_bootstrap();

    let mut test_runner: TestRunner<TypedInMemorySubstateStore> = TestRunner::new(true, &mut store);

    test_runner.set_current_epoch(10);

    let (public_key, _private_key, account_component) = test_runner.new_account();
    let (public_key_2, _private_key_2, account_component_2) = test_runner.new_account();

    let package_address = test_runner.compile_and_publish(this_package!());

    let subscription_price: Decimal = dec!("20");
    let create_channel_price: Decimal = dec!("50");
    let amount_rewards_subscription: Decimal = dec!("15");
    let amount_rewards_creating_channel: Decimal = dec!("15");
    let platform_fee: Decimal = dec!("5");

    let instantiate_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .call_function(
            package_address,
            "Streamdao",
            "instantiate_streamdao",
            args!(
                subscription_price,
                create_channel_price,
                amount_rewards_subscription,
                amount_rewards_creating_channel,
                platform_fee
            ),
        )
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

    let mint_membership_address = instantiation_receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[1];

    let membership_id_1: u64 = 95u64;

    let new_user_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_1, "Dan".to_string()),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_user_manifest, vec![public_key.into()]);

    let mut membership_1_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();

    membership_1_bt_id.insert(NonFungibleId::from_u64(membership_id_1));

    let channel_id: String = String::from("2314134n3214l32n14134k");

    let new_channel_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .withdraw_from_account_by_amount(dec!("100"), RADIX_TOKEN, account_component)
        .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |build, xrd_bucket_id| {
            build
                .withdraw_from_account_by_ids(
                    &membership_1_bt_id,
                    mint_membership_address,
                    account_component,
                )
                .take_from_worktop_by_ids(
                    &membership_1_bt_id,
                    mint_membership_address,
                    |build, user_nft_bucket_id| {
                        build.call_method(
                            component,
                            "new_channel_set_id",
                            args!(
                                Bucket(xrd_bucket_id),
                                Bucket(user_nft_bucket_id),
                                channel_id,
                                "fuserleer".to_string()
                            ),
                        )
                    },
                )
        })
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_channel_manifest, vec![public_key.into()]);

    let membership_id_2: u64 = 30u64;

    let new_membership_2_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("10")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_2, "Chico".to_string()),
        )
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(new_membership_2_manifest, vec![public_key_2.into()]);

    let mut membership_2_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();
    membership_2_bt_id.insert(NonFungibleId::from_u64(membership_id_2));

    test_runner.set_current_epoch(30);

    let subscribe_manifest_1 = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("10")))
        .withdraw_from_account_by_amount(dec!("100"), RADIX_TOKEN, account_component_2)
        .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |build, xrd_bucket_id| {
            build
                .withdraw_from_account_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    account_component_2,
                )
                .take_from_worktop_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    |build, membership_bucket_id| {
                        build.call_method(
                            component,
                            "subscribe",
                            args!(
                                Bucket(xrd_bucket_id),
                                Bucket(membership_bucket_id),
                                channel_id
                            ),
                        )
                    },
                )
        })
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    test_runner.execute_manifest(subscribe_manifest_1, vec![public_key_2.into()]);

    test_runner.set_current_epoch(40);

    let resubscribe_manifest_1 = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("10")))
        .withdraw_from_account_by_amount(dec!("100"), RADIX_TOKEN, account_component_2)
        .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |build, xrd_bucket_id| {
            build
                .withdraw_from_account_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    account_component_2,
                )
                .take_from_worktop_by_ids(
                    &membership_2_bt_id,
                    mint_membership_address,
                    |build, membership_bucket_id| {
                        build.call_method(
                            component,
                            "resubscribe",
                            args!(
                                Bucket(xrd_bucket_id),
                                Bucket(membership_bucket_id),
                                channel_id
                            ),
                        )
                    },
                )
        })
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    let recept = test_runner.execute_manifest(resubscribe_manifest_1, vec![public_key_2.into()]);

    println!("{:?\n}", recept);
    recept.expect_commit_failure();
}
