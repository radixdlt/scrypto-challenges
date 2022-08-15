use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;
use radix_engine::model::Receipt;
use radix_engine::model::SignedTransaction;

#[test]
fn test_portfolio_app() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, account) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    info!("Starting test 1 ");
    println!("Starting test 2");

    // Creating two accounts for the two parties involved.
    let (admin_public_key, admin_private_key, admin_address): (EcdsaPublicKey, EcdsaPrivateKey, ComponentAddress) =
    executor.new_account();

    let (_beneficiary_public_key, _beneficiary_private_key, _beneficiary_address): (
        EcdsaPublicKey,
        EcdsaPrivateKey,
        ComponentAddress,
    ) = executor.new_account();

    let (_key, _sk, _account) = executor.new_account();

    println!("Admin Account created {} " , admin_address);
    println!("Beneficiary Account created {} " , _beneficiary_address);
    println!("Account created {} " , _account);

    // Creating a new token to use for the test
    let mut token_information: HashMap<String, String> = HashMap::new();
    token_information.insert("name".to_string(), "Bitcoin example".to_string());
    token_information.insert("symbol".to_string(), "BTC".to_string());

    let token_creation_tx: SignedTransaction = TransactionBuilder::new()
        .new_token_fixed(token_information, dec!("100"))
        .call_method_with_all_resources(_beneficiary_address, "deposit_batch")
        .build(executor.get_nonce([_beneficiary_public_key]))
        .sign([&_beneficiary_private_key]);
    let token_creation_receipt: Receipt = executor.validate_and_execute(&token_creation_tx).unwrap();
    let btc_resource_address: ResourceAddress = token_creation_receipt.new_resource_addresses[0];
    println!("Token1 created {} " , btc_resource_address);

    // Creating a new token to use for the test
    let mut token_information: HashMap<String, String> = HashMap::new();
    token_information.insert("name".to_string(), "Ethereum example".to_string());
    token_information.insert("symbol".to_string(), "ETH".to_string());
    let token_creation_tx: SignedTransaction = TransactionBuilder::new()
        .new_token_fixed(token_information, dec!("1000"))
        .call_method_with_all_resources(admin_address, "deposit_batch")
        .build(executor.get_nonce([admin_public_key]))
        .sign([&admin_private_key]);
    let token_creation_receipt: Receipt = executor.validate_and_execute(&token_creation_tx).unwrap();
    let eth_resource_address: ResourceAddress = token_creation_receipt.new_resource_addresses[0];
    println!("Token2 created {} " , eth_resource_address);

    // Creating a new token to use for the test
    let mut token_information: HashMap<String, String> = HashMap::new();
    token_information.insert("name".to_string(), "Radix example".to_string());
    token_information.insert("symbol".to_string(), "DRX".to_string());
    let token_creation_tx: SignedTransaction = TransactionBuilder::new()
        .new_token_fixed(token_information, dec!("100000"))
        .call_method_with_all_resources(admin_address, "deposit_batch")
        .build(executor.get_nonce([admin_public_key]))
        .sign([&admin_private_key]);
    let token_creation_receipt: Receipt = executor.validate_and_execute(&token_creation_tx).unwrap();
    let rdx_resource_address: ResourceAddress = token_creation_receipt.new_resource_addresses[0];
    println!("Token3 created {} " , rdx_resource_address);
    
    // Creating a new token to use for the test
    let mut token_information: HashMap<String, String> = HashMap::new();
    token_information.insert("name".to_string(), "Leonets example".to_string());
    token_information.insert("symbol".to_string(), "Leo".to_string());
    let token_creation_tx: SignedTransaction = TransactionBuilder::new()
        .new_token_fixed(token_information, dec!("10000"))
        .call_method_with_all_resources(admin_address, "deposit_batch")
        .build(executor.get_nonce([admin_public_key]))
        .sign([&admin_private_key]);
    let token_creation_receipt: Receipt = executor.validate_and_execute(&token_creation_tx).unwrap();
    let leo_resource_address: ResourceAddress = token_creation_receipt.new_resource_addresses[0];  
    println!("Token4 created {} " , leo_resource_address);

    // Creating a new blueprint TradingApp 
    let args = args![RADIX_TOKEN, btc_resource_address, eth_resource_address, leo_resource_address];
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "TradingApp", "create_market", args)
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt1 = executor.validate_and_execute(&transaction1).unwrap();
    println!("Package Trading created {:?} \n", receipt1);
    assert!(receipt1.result.is_ok());
    let trading = receipt1.new_component_addresses.get(0).unwrap();
    println!("Trading Component Address  {:} ", trading);

    // Creating a new blueprint LendingApp 
    let _args_lending = args![RADIX_TOKEN, dec!(1000), dec!(7), dec!(10)];
    let transaction2 = TransactionBuilder::new()
        .withdraw_from_account(RADIX_TOKEN, account)
        .take_from_worktop_by_amount(dec!("1000"), RADIX_TOKEN, |builder, bucket_id| {
            builder.call_function(package, "LendingApp", "instantiate_pool", args![Bucket(bucket_id),dec!(1000), dec!(10), dec!(7)])
        })    
        .build(executor.get_nonce([_beneficiary_public_key]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("Package Lending created {:?} \n", receipt2);
    // assert!(receipt2.result.is_ok());
    let lending = receipt2.new_component_addresses.get(0).unwrap();
    println!("Lending Component Address  {} ", lending);

    // Creating a new blueprint PortfolioTradingApp 
    // let args_lending = args![RADIX_TOKEN, dec!(1000), dec!(7), dec!(10)];
    // let transaction2 = TransactionBuilder::new()
    //     .call_function(package, "LendingApp", "instantiate_pool", args_lending)
    //     .build(executor.get_nonce([pk]))
    //     .sign([&sk]);
    // let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    // println!("Package Lending created {:?} \n", receipt1);
    // assert!(receipt2.result.is_ok());
    // let lending = receipt2.new_component_addresses.get(0).unwrap();    

    // let mut xrd_bucket = Bucket::new(rdx_resource_address);
    // println!("Bucket1 created");  
    // let mut btc_bucket = Bucket::new(btc_resource_address);
    // println!("Bucket2 created");
    // let mut eth_bucket = Bucket::new(eth_resource_address);
    // let mut leo_bucket = Bucket::new(leo_resource_address);
    // println!("Bucket4 created");
    // let args = args![xrd_bucket, btc_bucket, eth_bucket, leo_bucket];
    //let args = args![10u32, dec!("1"), 3u64];
    let _args = args![dec!("1")];

    // Test the `fund_token1` method.
    let register_transaction = TransactionBuilder::new()
    .withdraw_from_account(RADIX_TOKEN, account)
    .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |builder, bucket_id| {
        builder.call_method(*trading, "fund_token1", args![Bucket(bucket_id)])
    })
    .call_method_with_all_resources(account, "deposit_batch")
    .build(executor.get_nonce([_beneficiary_public_key]))
    .sign([&sk]);
    let register_receipt = executor.validate_and_execute(&register_transaction).unwrap();
    println!("Method fund1 executed {:?} \n", register_receipt);
    assert!(register_receipt.result.is_ok());

    // Test the `fund_token2` method.
    let register_transaction_2 = TransactionBuilder::new()
    .withdraw_from_account(btc_resource_address, _beneficiary_address)
    .take_from_worktop_by_amount(dec!("10"), btc_resource_address, |builder, bucket_id| {
        builder.call_method(*trading, "fund_token2", args![Bucket(bucket_id)])
    })
    .call_method_with_all_resources(_beneficiary_address, "deposit_batch")
    .build(executor.get_nonce([_beneficiary_public_key]))
    .sign([&_beneficiary_private_key]);
    let register_receipt_2 = executor.validate_and_execute(&register_transaction_2).unwrap();
    println!("Method fund2 executed {:?} \n", register_receipt_2);
    assert!(register_receipt_2.result.is_ok());

    
    // Test the `fund_token1` method.... not working 
    let component = receipt1.new_component_addresses[0];
    let transaction3 = TransactionBuilder::new()
        .call_method(component, "fund_token1", args![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt3 = executor.validate_and_execute(&transaction3).unwrap();
    println!("{:?}\n", receipt3);
    assert!(receipt3.result.is_ok());

//     CALL_METHOD ComponentAddress("${account}") "withdraw_by_amount" Decimal("80") ResourceAddress("${xrd}");
// TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("80") ResourceAddress("${xrd}") Bucket("bucket1");
// CALL_METHOD ComponentAddress("${account}") "create_proof_by_amount" Decimal("1") ResourceAddress("${lend_nft}");
// POP_FROM_AUTH_ZONE Proof("proof1");
// CALL_METHOD ComponentAddress("${component}") "lend_money" Bucket("bucket1") Proof("proof1");
// CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("${account}") "deposit_batch";
}
