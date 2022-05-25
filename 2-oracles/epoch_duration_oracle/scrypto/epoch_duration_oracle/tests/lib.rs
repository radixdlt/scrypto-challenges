use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn can_tick_from_scratch() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, oracle_owner) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    let oracle_creation = TransactionBuilder::new()
        .call_function(package, "EpochDurationOracle", "new", args!())
        .call_method_with_all_resources(oracle_owner, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_receipt = executor
        .validate_and_execute(&oracle_creation)
        .unwrap_or_else(|err| {
            println!("err - {:?}", err);
            panic!()
        });
    println!("Oracle creation receipt - {:?}", oracle_receipt);
    assert!(oracle_receipt.result.is_ok());

    println!(
        "Oracle component address - {:?}",
        oracle_receipt.new_component_addresses
    );
    let oracle = oracle_receipt.new_component_addresses.get(0).unwrap();

    println!(
        "Oracle resource addresses - {:?}",
        oracle_receipt.new_resource_addresses
    );
    let oracle_badge_address = oracle_receipt.new_resource_addresses.get(0).unwrap();
    println!("Oracle badge resource address - {:?}", oracle_badge_address);

    let oracle_tick = TransactionBuilder::new()
        .create_proof_from_account_by_amount(dec!(1), *oracle_badge_address, oracle_owner)
        .call_method(*oracle, "tick", args!(1000u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_ticked = executor.validate_and_execute(&oracle_tick).unwrap();
    println!("{:?}\n", oracle_ticked);
    assert!(oracle_ticked.result.is_ok());
    let ticked_epoch = oracle_ticked.outputs.get(1).unwrap().to_string();
    assert!(ticked_epoch == "0u64");
}

#[test]
fn can_tick_from_scratch_and_switch_epoch() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, oracle_owner) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    let oracle_creation = TransactionBuilder::new()
        .call_function(package, "EpochDurationOracle", "new", args!())
        .call_method_with_all_resources(oracle_owner, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_receipt = executor
        .validate_and_execute(&oracle_creation)
        .unwrap_or_else(|err| {
            println!("err - {:?}", err);
            panic!()
        });
    println!("Oracle creation receipt - {:?}", oracle_receipt);
    assert!(oracle_receipt.result.is_ok());

    println!(
        "Oracle component address - {:?}",
        oracle_receipt.new_component_addresses
    );
    let oracle = oracle_receipt.new_component_addresses.get(0).unwrap();

    println!(
        "Oracle resource addresses - {:?}",
        oracle_receipt.new_resource_addresses
    );
    let oracle_badge_address = oracle_receipt.new_resource_addresses.get(0).unwrap();
    println!("Oracle badge resource address - {:?}", oracle_badge_address);

    let oracle_tick = TransactionBuilder::new()
        .create_proof_from_account_by_amount(dec!(1), *oracle_badge_address, oracle_owner)
        .call_method(*oracle, "tick", args!(1000u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_ticked = executor.validate_and_execute(&oracle_tick).unwrap();
    println!("{:?}\n", oracle_ticked);
    assert!(oracle_ticked.result.is_ok());
    let ticked_epoch = oracle_ticked.outputs.get(1).unwrap().to_string();
    assert!(ticked_epoch == "0u64");

    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "1000u64");

    executor.substate_store_mut().set_epoch(1);

    let oracle_tick = TransactionBuilder::new()
        .create_proof_from_account_by_amount(dec!(1), *oracle_badge_address, oracle_owner)
        .call_method(*oracle, "tick", args!(1000u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_ticked = executor.validate_and_execute(&oracle_tick).unwrap();
    println!("{:?}\n", oracle_ticked);
    assert!(oracle_ticked.result.is_ok());
    let ticked_epoch = oracle_ticked.outputs.get(1).unwrap().to_string();
    assert!(ticked_epoch == "1u64");

    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "2000u64");

    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "0u64");
}

#[test]
fn can_tick_with_bootstrap() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, oracle_owner) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    let oracle_creation = TransactionBuilder::new()
        .call_function(
            package,
            "EpochDurationOracle",
            "new_with_bootstrap",
            args!(631u64, 1653431602254u64),
        )
        .call_method_with_all_resources(oracle_owner, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_receipt = executor
        .validate_and_execute(&oracle_creation)
        .unwrap_or_else(|err| {
            println!("err - {:?}", err);
            panic!()
        });
    println!("Oracle creation receipt - {:?}", oracle_receipt);
    assert!(oracle_receipt.result.is_ok());

    println!(
        "Oracle component address - {:?}",
        oracle_receipt.new_component_addresses
    );
    let oracle = oracle_receipt.new_component_addresses.get(0).unwrap();

    println!(
        "Oracle resource addresses - {:?}",
        oracle_receipt.new_resource_addresses
    );
    let oracle_badge_address = oracle_receipt.new_resource_addresses.get(0).unwrap();
    println!("Oracle badge resource address - {:?}", oracle_badge_address);

    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "1653431602254u64");

    let oracle_tick = TransactionBuilder::new()
        .create_proof_from_account_by_amount(dec!(1), *oracle_badge_address, oracle_owner)
        .call_method(*oracle, "tick", args!(1000u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_ticked = executor.validate_and_execute(&oracle_tick).unwrap();
    println!("{:?}\n", oracle_ticked);
    assert!(oracle_ticked.result.is_ok());
    let ticked_epoch = oracle_ticked.outputs.get(1).unwrap().to_string();
    assert!(ticked_epoch == "631u64");

    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "1653431603254u64");
}
