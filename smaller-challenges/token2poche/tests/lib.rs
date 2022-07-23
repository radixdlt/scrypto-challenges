use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn test_token2poche() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (admin_pk, admin_sk, admin_account) = executor.new_account();
    let (random_pk, random_sk, random_account) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();


    // Test the `new` function.
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "Token2Poche", "new", args![dec!(1000)])
        .build(executor.get_nonce([admin_pk]))
        .sign([&admin_sk]);
    let receipt1 = executor.validate_and_execute(&transaction1).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());

    let component = receipt1.new_component_addresses[0];
    let admin_badge = receipt1.new_resource_addresses[0];

    // Test the buy function with random account.
    let transaction5 = TransactionBuilder::new()
        .call_method(component, "buy", args![dec!(2)])
        .call_method_with_all_resources(random_account, "deposit_batch")
        .build(executor.get_nonce([random_pk]))
        .sign([&random_sk]);
    let receipt5 = executor.validate_and_execute(&transaction5).unwrap();
    println!("{:?}\n", receipt5);
    assert!(receipt5.result.is_ok());


    // Test the `change_price` function with admin_account`.
    let transaction2 = TransactionBuilder::new()
        .call_method(component, "change_price", args![dec!(1)])
        .call_method_with_all_resources(admin_account, "deposit_batch")
        .build(executor.get_nonce([admin_pk]))
        .sign([&admin_sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());

    // Test the `change_price` function with the random account.
    let failed_transaction1 = TransactionBuilder::new()
        .call_method(component, "change_price", args![dec!(5)])
        .call_method_with_all_resources(random_account, "deposit_batch")
        .build(executor.get_nonce([random_pk]))
        .sign([&random_sk]);
    let failed_receipt1 = executor.validate_and_execute(&failed_transaction1).unwrap();
    println!("{:?}\n", failed_receipt1);
    assert!(failed_receipt1.result.is_err());

    // Test the withdraw function with admin account.
    let transaction3 = TransactionBuilder::new()
        .call_method(component, "withdraw_funds", args![])
        .call_method_with_all_resources(admin_account, "deposit_batch")
        .build(executor.get_nonce([admin_pk]))
        .sign([&admin_sk]);
    let receipt3 = executor.validate_and_execute(&transaction3).unwrap();
    println!("{:?}\n", receipt3);
    assert!(receipt3.result.is_ok());

    // Test the withdraw function with random account.
    let failed_transaction2 = TransactionBuilder::new()
        .call_method(component, "withdraw_funds", args![])
        .call_method_with_all_resources(random_account, "deposit_batch")
        .build(executor.get_nonce([random_pk]))
        .sign([&random_sk]);
    let failed_receipt2 = executor.validate_and_execute(&failed_transaction2).unwrap();
    println!("{:?}\n", failed_receipt2);
    assert!(failed_receipt2.result.is_err());
}
