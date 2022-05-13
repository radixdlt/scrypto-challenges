use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn vrf_verify_success() {
    // // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, account) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    // Test the `new` function.
    // When calling new, provide the public key of the off-chain oracle for the contract to store and use when verifying proofs from the off-chain oracle.
    let transaction1 = TransactionBuilder::new()
        .call_function(
            package,
            "VrfOracleContract",
            "new",
            args!["0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6"],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt1 = executor
        .validate_and_execute(&transaction1)
        .unwrap_or_else(|err| {
            println!("err - {:?}", err);
            panic!()
        });
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());

    println!(
        "receipt1 new resource addresses - {:?}",
        receipt1.new_resource_addresses
    );
    let nft_receipt_addr = receipt1.new_resource_addresses.get(2).unwrap();
    println!("nft resource addr - {:?}", nft_receipt_addr);

    // Start a request_randomess with 5 XRD payment.
    let vrf_component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .withdraw_from_account_by_amount(Decimal(5), RADIX_TOKEN, account)
        .take_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
            builder.call_method(
                vrf_component,
                "request_randomness_by_counter",
                args![scrypto::resource::Bucket(bucket_id)],
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
    let mut ids = BTreeSet::new();
    ids.insert(NonFungibleId::from_u64(0));

    // Calculate VRF proof off-chain, then call fullfill_randomness with vrf proof.
    let transaction3 = TransactionBuilder::new()
        .call_method(
            vrf_component,
            "fullfill_randomness_request",
            args!["0000000000000000", "02c964e837f153a67f51b87354796c9f1c8ca2436a6568e26f9d740d305a554c8e99eb6802c6b541355b3b9b20a89fb9d384c0bc32603e4e5e1f92bb41b88a3548ef8a0eb0aefbb85918c5ca386f1ffe34"]
        )
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt3 = executor.validate_and_execute(&transaction3).unwrap();
    println!("{:?}\n", receipt3);
    assert!(receipt3.result.is_ok());

    let transaction4 = TransactionBuilder::new()
        .withdraw_from_account_by_ids(&ids, *nft_receipt_addr, account)
        .take_from_worktop_by_ids(&ids, *nft_receipt_addr, |builder, bucket_id| {
            builder.call_method(
                vrf_component,
                "fetch_randomness",
                args![scrypto::resource::Bucket(bucket_id)],
            )
        })
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt4 = executor.validate_and_execute(&transaction4).unwrap();
    println!("{:?}\n", receipt4);
    assert!(receipt4.result.is_ok());
}

#[test]
fn vrf_verify_fail() {
    // // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, account) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    // // Test the `new` function.
    let transaction1 = TransactionBuilder::new()
        .call_function(
            package,
            "VrfOracleContract",
            "new",
            args!["0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6"],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt1 = executor
        .validate_and_execute(&transaction1)
        .unwrap_or_else(|err| {
            println!("err - {:?}", err);
            panic!()
        });
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());

    println!(
        "receipt1 new resource addresses - {:?}",
        receipt1.new_resource_addresses
    );
    let nft_receipt_addr = receipt1.new_resource_addresses.get(2).unwrap();
    println!("nft resource addr - {:?}", nft_receipt_addr);

    // Start a request_randomess with 5 XRD payment.
    let vrf_component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .withdraw_from_account_by_amount(Decimal(5), RADIX_TOKEN, account)
        .take_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
            builder.call_method(
                vrf_component,
                "request_randomness_by_counter",
                args![scrypto::resource::Bucket(bucket_id)],
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
    let mut ids = BTreeSet::new();
    ids.insert(NonFungibleId::from_u64(0));

    // Calculate VRF proof off-chain, then call fullfill_randomness with vrf proof.
    let transaction3 = TransactionBuilder::new()
        .call_method(
            vrf_component,
            "fullfill_randomness_request",
            // Edit first byte from 02 to 03 to generate verify failure
            args!["0000000000000000", "03c964e837f153a67f51b87354796c9f1c8ca2436a6568e26f9d740d305a554c8e99eb6802c6b541355b3b9b20a89fb9d384c0bc32603e4e5e1f92bb41b88a3548ef8a0eb0aefbb85918c5ca386f1ffe34"]
        )
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt3 = executor.validate_and_execute(&transaction3).unwrap();
    println!("{:?}\n", receipt3);
    assert!(receipt3.result.is_err());
}
