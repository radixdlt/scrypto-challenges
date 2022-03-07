use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

use hareswap::api::*;

// TODO LIST
// * put hareswap address or maker address into partial or matched order...some combo of all those
// * expire time on orders
// * some example on selling the order to someone else, these would be similar to options
// * CLI (based on needs from test below)

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
fn test_scenerio1() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let key = executor.new_public_key();
//    let account = executor.new_account(key);
    let package = executor.publish_package(include_code!("hareswap")).unwrap();

    // 0.0: taker
    // new public key
    // create regular account
    // mint and deposit T tokens

    // 0.1: taker: create taker_auth token to stop front-running, like "approval" but better
    // mint simple auth token -- this may be an optional step if we can test with a virtual badge instead.

    // 0.0: maker
    // new public key (for signing txs)
    // create regular account
    // mint and deposit M tokens

    // 0.1: maker: account setup:
    // mint account auth (for custodial account, but keep one in regular account too to move money out of CustodialAccount)
    // instantiate CustodialAccount with auth_requirement matching newly minted account auth
    // transfer M tokens from regular account to custodial

    // above likely the same for all tests

    // 0.2: Maker setup:
    // new public key (for transporter)
    // instantiate "simple" Maker with auto-generated callback_auth (by passing empty callback_auth Bucket) using account info from previous step

    // off-ledger

    // 1: taker: build_partial_order
    // simulate send taker -> maker : PartialOrder
    // 2: maker: match_fungible_order_simple & sign
    // simulate send maker -> taker : SignedOrder
    // 3: taker: decide they "like" the terms, ie. the provided taker_contents, and also good to double check the PartialOrder matches the one I sent

    // 4-A: taker:  OPTION 1 - simple execution
    // method name baked in and includes unlikely name to avoid collision attacks, could bake in hareswap component too as a passthru instead of adding it to the order
    // setup manifest:
        // METHOD_CALL account "withdraw" <taker_amount> T
        // TAKE_ALL_FROM_WORKTOP T Bucket("T_for_order")
        // (optional, instead of virtual badge) get auth_ref
        // METHOD_CALL SignedOrder.maker_address, "hareswap_execute_order", encoded(SignedOrder), Bucket("T_for_order"), auth_ref_or_vitual_bdage_ref
        // TAKE_ALL_FROM_WORKTOP M Bucket("from_maker")
        // ASSERT_WORKTOP_CONTAINS <expected_from_maker> <maker_resource>
        // CALL_METHOOD_WITH_ALL_RESOURCES account "deposit_badge"

    // OR

    // 4-B: taker:  OPTION 2 - advanced execution -- grab a flash loan for T collateralized with XRD? -- WIP
    // setup manifest:
        // METHOD_CALL account "withdraw" some_amount XRD
        // TAKE_ALL_FROM_WORKTOP XRD Bucket("loan_of_xrd")
        // METHOD_CALL loan_component "flash_loan" Bucket("loan_of_xrd") T
        // TAKE_FROM_WORKTOP SignedOrder.taker_contents.amount T Bucket("T_for_order")
        // - same as before
        // (optional, instead of virtual badge) get auth_ref
        // METHOD_CALL SignedOrder.maker_address, "hareswap_execute_order", encoded(SignedOrder), Bucket("T_for_order"), auth_ref_or_vitual_bdage_ref
        // TAKE_ALL_FROM_WORKTOP M Bucket("from_maker")
        // ASSERT_WORKTOP_CONTAINS <expected_from_maker> <maker_resource>
        // -- repay loan
        // ++ 
        // -- finish
        // CALL_METHOOD_WITH_ALL_RESOURCES account "deposit_badge"

    // 4-C: taker OPTION 3 - advanced execution - grab multiple RFQs and route them myself
        // TODO

    // on-ledger

    // 5: submit manifest


}


#[test]
fn test_publish() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let key = executor.new_public_key();
    let _account = executor.new_account(key);
    let _package = executor.publish_package(include_code!("hareswap")).unwrap();
}
