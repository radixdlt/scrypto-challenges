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

// Example 4 from https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#appendix-A.2
#[test]
fn vrf_verify_example_4() {
    let PK = "0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6";
    let alpha = "73616d706c65"; // (ASCII "sample")
    let pi = "0331d984ca8fece9cbb9a144c0d53df3c4c7a33080c1e02ddb1a96a365394c7888782fffde7b842c38c20c08de6ec6c2e7027a97000f2c9fa4425d5c03e639fb48fde58114d755985498d7eb234cf4aed9";

    // // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, account) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    // Test the `new` function.
    // When calling new, provide the public key of the off-chain oracle for the contract to store and use when verifying proofs from the off-chain oracle.
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "VrfOracleContract", "new", args![PK])
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
                "request_randomness_with_seed",
                args![scrypto::resource::Bucket(bucket_id), alpha],
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());

    // Calculate VRF proof off-chain, then call fullfill_randomness with vrf proof.
    let transaction3 = TransactionBuilder::new()
        .call_method(
            vrf_component,
            "fullfill_randomness_request",
            args![alpha, pi],
        )
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt3 = executor.validate_and_execute(&transaction3).unwrap();
    println!("{:?}\n", receipt3);
    assert!(receipt3.result.is_ok());

    let mut ids = BTreeSet::new();
    ids.insert(NonFungibleId::from_bytes(hex::decode(alpha).unwrap()));

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

// Example 5 from https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#appendix-A.2
#[test]
fn vrf_verify_example_5() {
    let PK = "0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6";
    let alpha = "74657374"; // (ASCII "test")
    let pi = "03f814c0455d32dbc75ad3aea08c7e2db31748e12802db23640203aebf1fa8db2743aad348a3006dc1caad7da28687320740bf7dd78fe13c298867321ce3b36b79ec3093b7083ac5e4daf3465f9f43c627";

    // // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, account) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    // Test the `new` function.
    // When calling new, provide the public key of the off-chain oracle for the contract to store and use when verifying proofs from the off-chain oracle.
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "VrfOracleContract", "new", args![PK])
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
                "request_randomness_with_seed",
                args![scrypto::resource::Bucket(bucket_id), alpha],
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());

    // Calculate VRF proof off-chain, then call fullfill_randomness with vrf proof.
    let transaction3 = TransactionBuilder::new()
        .call_method(
            vrf_component,
            "fullfill_randomness_request",
            args![alpha, pi],
        )
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt3 = executor.validate_and_execute(&transaction3).unwrap();
    println!("{:?}\n", receipt3);
    assert!(receipt3.result.is_ok());

    let mut ids = BTreeSet::new();
    ids.insert(NonFungibleId::from_bytes(hex::decode(alpha).unwrap()));

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

// Example 6 from https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#appendix-A.2
#[test]
fn vrf_verify_example_6() {
    let PK = "03596375e6ce57e0f20294fc46bdfcfd19a39f8161b58695b3ec5b3d16427c274d";
    let alpha = "4578616d706c65207573696e67204543445341206b65792066726f6d20417070656e646978204c2e342e32206f6620414e53492e58392d36322d32303035"; // (ASCII "Example using ECDSA key from Appendix L.4.2 of ANSI.X9-62-2005")
    let pi = "039f8d9cdc162c89be2871cbcb1435144739431db7fab437ab7bc4e2651a9e99d5488405a11a6c7fc8defddd9e1573a563b7333aab4effe73ae9803274174c659269fd39b53e133dcd9e0d24f01288de9a";

    // // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, account) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    // Test the `new` function.
    // When calling new, provide the public key of the off-chain oracle for the contract to store and use when verifying proofs from the off-chain oracle.
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "VrfOracleContract", "new", args![PK])
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
                "request_randomness_with_seed",
                args![scrypto::resource::Bucket(bucket_id), alpha],
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());

    // Calculate VRF proof off-chain, then call fullfill_randomness with vrf proof.
    let transaction3 = TransactionBuilder::new()
        .call_method(
            vrf_component,
            "fullfill_randomness_request",
            args![alpha, pi],
        )
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt3 = executor.validate_and_execute(&transaction3).unwrap();
    println!("{:?}\n", receipt3);
    assert!(receipt3.result.is_ok());

    let mut ids = BTreeSet::new();
    ids.insert(NonFungibleId::from_bytes(hex::decode(alpha).unwrap()));

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
