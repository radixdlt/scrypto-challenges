use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

use hareswap::api::*;

// taker does this
fn build_partial_order(maker_requirement: BucketRequirement, taker_resource: ResourceDef, taker_auth: BucketRequirement) -> PartialOrder {
    PartialOrder {
        maker_requirement,
        taker_resource,
        taker_auth,
    }
}

// maker does this
fn match_fungible_order_simple(partial_order: PartialOrder, maker_address: Address, amount: Decimal) -> MatchedOrder {
    let taker_contents = BucketContents::Fungible(amount); // i'll need this much
    let maker_callback = Callback::CallMethod { // in a bucket passed to this callback
        component_address: maker_address,
        method: "handle_order_default_callback".to_owned(),
        args: vec![], // no bound custom args
    };
    MatchedOrder {
        partial_order,
        taker_contents,
        maker_callback,
    }
}

// maker does this
fn sign_order(matched_order: MatchedOrder, private_key: &[u8]) -> SignedOrder {
    let serialized = scrypto_encode(&matched_order);
    let signature = sign(&serialized, private_key);
    SignedOrder {
        order: matched_order,
        signature,
    }
}


#[test]
fn test_hello() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(include_code!("hareswap")).unwrap();

    // Test the `instantiate_hello` function.
    let transaction1 = TransactionBuilder::new(&executor)
        .call_function(package, "Hello", "instantiate_hello", vec![], None)
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt1 = executor.run(transaction1).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());

    // Test the `free_token` method.
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(component, "free_token", vec![], Some(account))
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}
