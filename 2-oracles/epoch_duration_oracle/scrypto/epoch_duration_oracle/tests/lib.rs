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
    // println!("Oracle creation receipt - {:?}", oracle_receipt);
    assert!(oracle_receipt.result.is_ok());

    // println!(
    //     "Oracle component address - {:?}",
    //     oracle_receipt.new_component_addresses
    // );
    let oracle = oracle_receipt.new_component_addresses.get(0).unwrap();

    // println!(
    //     "Oracle resource addresses - {:?}",
    //     oracle_receipt.new_resource_addresses
    // );
    let oracle_badge_address = oracle_receipt.new_resource_addresses.get(0).unwrap();
    // println!("Oracle badge resource address - {:?}", oracle_badge_address);

    // I am just instantiated
    // I should have 0 millis elapsed in epoch 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "0u64");

    // I should have 0 millis elapsed since epoch 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "0u64");

    // I should not find epochs greater than 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_err());
    let (_, error_message) = millis_in_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // I should not be able to not find elapsed time since epochs greater than 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_err());
    let (_, error_message) = millis_since_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // I should be able to tick 1000 millis in the current epoch (0)
    let oracle_tick = TransactionBuilder::new()
        .create_proof_from_account_by_amount(dec!(1), *oracle_badge_address, oracle_owner)
        .call_method(*oracle, "tick", args!(1000u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_ticked = executor.validate_and_execute(&oracle_tick).unwrap();
    // println!("{:?}\n", oracle_ticked);
    assert!(oracle_ticked.result.is_ok());
    let ticked_epoch = oracle_ticked.outputs.get(1).unwrap().to_string();
    assert!(ticked_epoch == "0u64");

    // I should then be able to retrieve the 1000 millis on epoch 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "1000u64");

    // And I should have 1000 millis elapsed since epoch 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "1000u64");

    // But still should not then be able to retrieve a greater epochs than 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_err());
    let (_, error_message) = millis_in_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // And should still not find elapsed time since epochs greater than 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_err());
    let (_, error_message) = millis_since_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // An epoch happens on ledger
    executor.substate_store_mut().set_epoch(1);
    // Epoch is now 1 on ledger but still 0 on oracle

    // I should then be able to retrieve the 1000 millis on epoch 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "1000u64");

    // And I should have 1000 millis elapsed since epoch 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "1000u64");

    // But still should not then be able to retrieve a greater epochs than 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_err());
    let (_, error_message) = millis_in_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // And should still not find elapsed time since epochs greater than 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_err());
    let (_, error_message) = millis_since_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // I should be able to tick 1000 millis in the current epoch (0)
    let oracle_tick = TransactionBuilder::new()
        .create_proof_from_account_by_amount(dec!(1), *oracle_badge_address, oracle_owner)
        .call_method(*oracle, "tick", args!(1000u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    // And this should switch oracle state to epoch 1
    let oracle_ticked = executor.validate_and_execute(&oracle_tick).unwrap();
    // println!("{:?}\n", oracle_ticked);
    assert!(oracle_ticked.result.is_ok());
    let ticked_epoch = oracle_ticked.outputs.get(1).unwrap().to_string();
    assert!(ticked_epoch == "1u64");

    // I should then be able to retrieve the 2000 millis on epoch 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "2000u64");

    // And 0 millis on epoch 1
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "0u64");

    // And I should have 2000 millis elapsed since epoch 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "2000u64");

    // And 0 millis elapsed since epoch 1
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "0u64");

    // But still should not then be able to retrieve a greater epochs than 1
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(2u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_err());
    let (_, error_message) = millis_in_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // And should still not find elapsed time since epochs greater than 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(2u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_err());
    let (_, error_message) = millis_since_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // An epoch is missed on oracle but happens on ledger
    executor.substate_store_mut().set_epoch(3);
    // Epoch is now 3 on ledger but still 1 on oracle

    // I should be able to tick 1000 millis in the current epoch (1)
    let oracle_tick = TransactionBuilder::new()
        .create_proof_from_account_by_amount(dec!(1), *oracle_badge_address, oracle_owner)
        .call_method(*oracle, "tick", args!(1000u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    // And this should switch oracle state to epoch 3
    let oracle_ticked = executor.validate_and_execute(&oracle_tick).unwrap();
    // println!("{:?}\n", oracle_ticked);
    assert!(oracle_ticked.result.is_ok());
    let ticked_epoch = oracle_ticked.outputs.get(1).unwrap().to_string();
    assert!(ticked_epoch == "3u64");

    // I should then be able to retrieve the 2000 millis on epoch 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "2000u64");

    // I should then be able to retrieve the 1000 millis on epoch 1
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "1000u64");

    // 0 millis on epoch 2
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(2u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    let (level, error_message) = millis_in_epoch_receipt.logs.get(1).unwrap();
    assert!(millis_in_epoch == "0u64");
    assert!(error_message.contains("Epoch was not registered on the oracle, sorry for the inconvenience. We are returning 0 and suggest you call millis_since_epoch or contact an administrator if you absolutely need this epoch duration."));
    matches!(level, Level::Warn);

    // And 0 millis on epoch 3
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(3u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "0u64");

    // And I should have 3000 millis elapsed since epoch 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "3000u64");

    // 1000 millis elapsed since epoch 1
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(1u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "1000u64");

    // 0 millis elapsed since epoch 2
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(2u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "0u64");

    // And 0 millis elapsed since epoch 3
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(3u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "0u64");

    // But still should not then be able to retrieve a greater epochs than 3
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(4u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_err());
    let (_, error_message) = millis_in_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));

    // And should still not find elapsed time since epochs greater than 3
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(4u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_err());
    let (_, error_message) = millis_since_epoch_receipt.logs.get(0).unwrap();
    assert!(error_message
        .contains("The requested epoch has not yet happened or was not yet registered on ledger."));
}

#[test]
fn can_tick_with_bootstrap() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, oracle_owner) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    executor.substate_store_mut().set_epoch(631);

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
    // println!("Oracle creation receipt - {:?}", oracle_receipt);
    assert!(oracle_receipt.result.is_ok());

    // println!(
    //     "Oracle component address - {:?}",
    //     oracle_receipt.new_component_addresses
    // );
    let oracle = oracle_receipt.new_component_addresses.get(0).unwrap();

    // println!(
    //     "Oracle resource addresses - {:?}",
    //     oracle_receipt.new_resource_addresses
    // );
    let oracle_badge_address = oracle_receipt.new_resource_addresses.get(0).unwrap();
    // println!("Oracle badge resource address - {:?}", oracle_badge_address);

    // I should not be able to request epoch 0
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    let (level, error_message) = millis_in_epoch_receipt.logs.get(1).unwrap();
    assert!(millis_in_epoch == "0u64");
    assert!(error_message.contains("Epoch was not registered on the oracle, sorry for the inconvenience. We are returning 0 and suggest you call millis_since_epoch or contact an administrator if you absolutely need this epoch duration."));
    matches!(level, Level::Warn);

    // But should be able to request elapsed since epoch 0
    let get_millis_since_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_since_epoch", args!(0u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_since_epoch_receipt = executor
        .validate_and_execute(&get_millis_since_epoch)
        .unwrap();
    // println!("{:?}\n", millis_since_epoch_receipt);
    assert!(millis_since_epoch_receipt.result.is_ok());
    let millis_since_epoch = millis_since_epoch_receipt
        .outputs
        .get(0)
        .unwrap()
        .to_string();
    assert!(millis_since_epoch == "1653431602254u64");

    // And request duration of epoch 631
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(631u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "1653431602254u64");

    // I should be able to tick 1000 millis in epoch 631
    let oracle_tick = TransactionBuilder::new()
        .create_proof_from_account_by_amount(dec!(1), *oracle_badge_address, oracle_owner)
        .call_method(*oracle, "tick", args!(1000u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let oracle_ticked = executor.validate_and_execute(&oracle_tick).unwrap();
    // println!("{:?}\n", oracle_ticked);
    assert!(oracle_ticked.result.is_ok());
    let ticked_epoch = oracle_ticked.outputs.get(1).unwrap().to_string();
    assert!(ticked_epoch == "631u64");

    // And check that ticker increased in that epoch
    let get_millis_in_epoch = TransactionBuilder::new()
        .call_method(*oracle, "millis_in_epoch", args!(631u64))
        .build(executor.get_nonce([pk]))
        .sign([&sk]);

    let millis_in_epoch_receipt = executor.validate_and_execute(&get_millis_in_epoch).unwrap();
    // println!("{:?}\n", millis_in_epoch_receipt);
    assert!(millis_in_epoch_receipt.result.is_ok());
    let millis_in_epoch = millis_in_epoch_receipt.outputs.get(0).unwrap().to_string();
    assert!(millis_in_epoch == "1653431603254u64");
}
