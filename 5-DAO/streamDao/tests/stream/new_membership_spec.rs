use radix_engine::ledger::*;
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_membership_user() {
    let mut store: TypedInMemorySubstateStore = TypedInMemorySubstateStore::with_bootstrap();

    let mut test_runner: TestRunner<TypedInMemorySubstateStore> = TestRunner::new(true, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

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

    let component = test_runner
        .execute_manifest(instantiate_manifest, vec![public_key.into()])
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];

    let new_user_manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "lock_fee", args!(dec!("10")))
        .call_method(component, "new_membership", args!("Dan".to_string()))
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();

    let recept = test_runner.execute_manifest(new_user_manifest, vec![public_key.into()]);
    println!("{:?\n}", recept);

    recept.expect_commit_success();
}
