//! It's pretty hard write complex intergration tests, and they are slow to complie/link
//! So the integration testing is happening externally utilizing resim and hare cli and 
//! a bunch of shell scripts
use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

use hareswap::api::*;

#[test]
fn test_publish() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let key = executor.new_public_key();
    let _account = executor.new_account(key);
    let _package = executor.publish_package(include_code!("hareswap")).unwrap();
}
