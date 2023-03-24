use radix_engine::ledger::*;
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_new_proposal() {
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
        .call_method(account_component, "lock_fee", args!(dec!("100")))
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
    let mut membership_1_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();
    membership_1_bt_id.insert(NonFungibleId::from_u64(membership_id_1));

    let channel_id: String = String::from("234823375365432420349");

    let new_channel_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("100")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_1, String::from("Dan")),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
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
                                String::from("fuserleer")
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

    let instantiation_channel_receipt =
        test_runner.execute_manifest(new_channel_manifest, vec![public_key.into()]);

    let channel_rewards_address = instantiation_channel_receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[0];

    let membership_id_2: u64 = 30u64;
    let mut membership_2_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();
    membership_2_bt_id.insert(NonFungibleId::from_u64(membership_id_2));

    let new_membership_2_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("100")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_2, String::from("Piers")),
        )
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
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
        .withdraw_from_account_by_amount(dec!("15"), channel_rewards_address, account_component_2)
        .take_from_worktop_by_amount(
            dec!("15"),
            channel_rewards_address,
            |build, channel_rewards_bucket_id| {
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
                                "new_proposal",
                                args!(
                                    Bucket(channel_rewards_bucket_id),
                                    Bucket(membership_bucket_id),
                                    channel_id,
                                    vec!["DAN-COIN", "COIN-DAN", "DAN-DAN"],
                                    String::from(
                                        "https://streamdaotest/ipfs/a5XPj3N75k4ZUNcFLGngSj72kYZBH"
                                    ),
                                    20u64,
                                    50u64
                                ),
                            )
                        },
                    )
            },
        )
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    let recept = test_runner.execute_manifest(new_membership_2_manifest, vec![public_key_2.into()]);

    println!("{:?\n}", recept);
    recept.expect_commit_success();
}

#[test]
fn test_error_new_proposal_insufficient_amount_rewards() {
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
        .call_method(account_component, "lock_fee", args!(dec!("100")))
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
    let mut membership_1_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();
    membership_1_bt_id.insert(NonFungibleId::from_u64(membership_id_1));

    let channel_id: String = String::from("234823375365432420349");

    let new_channel_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("100")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_1, String::from("Dan")),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
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
                                String::from("fuserleer")
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

    let instantiation_channel_receipt =
        test_runner.execute_manifest(new_channel_manifest, vec![public_key.into()]);

    let channel_rewards_address = instantiation_channel_receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[0];

    let membership_id_2: u64 = 30u64;
    let mut membership_2_bt_id: BTreeSet<NonFungibleId> = BTreeSet::new();
    membership_2_bt_id.insert(NonFungibleId::from_u64(membership_id_2));

    let new_membership_2_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component_2, "lock_fee", args!(dec!("100")))
        .call_method(
            component,
            "new_membership_set_id",
            args!(membership_id_2, String::from("Piers")),
        )
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
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
        .withdraw_from_account_by_amount(dec!("10"), channel_rewards_address, account_component_2)
        .take_from_worktop_by_amount(
            dec!("10"),
            channel_rewards_address,
            |build, channel_rewards_bucket_id| {
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
                                "new_proposal",
                                args!(
                                    Bucket(channel_rewards_bucket_id),
                                    Bucket(membership_bucket_id),
                                    channel_id,
                                    vec!["DAN-COIN", "COIN-DAN", "DAN-DAN"],
                                    String::from(
                                        "https://streamdaotest/ipfs/a5XPj3N75k4ZUNcFLGngSj72kYZBH"
                                    ),
                                    20u64,
                                    50u64
                                ),
                            )
                        },
                    )
            },
        )
        .call_method(
            account_component_2,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    let recept = test_runner.execute_manifest(new_membership_2_manifest, vec![public_key_2.into()]);

    println!("{:?\n}", recept);
    recept.expect_commit_failure();
}
