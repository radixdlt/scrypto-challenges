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
    let transaction_trading_app = TransactionBuilder::new()
        .call_function(package, "TradingApp", "create_market", args)
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt_trading_app = executor.validate_and_execute(&transaction_trading_app).unwrap();
    println!("Package Trading created {:?} \n", receipt_trading_app);
    assert!(receipt_trading_app.result.is_ok());
    let trading_component = receipt_trading_app.new_component_addresses.get(0).unwrap();
    println!("Trading Component Address  {:} ", trading_component);

    // Creating a new blueprint LendingApp 
    let _args_lending = args![RADIX_TOKEN, dec!(1000), dec!(7), dec!(10)];
    let transaction_lending_app = TransactionBuilder::new()
        .withdraw_from_account(RADIX_TOKEN, account)
        .take_from_worktop_by_amount(dec!("1000"), RADIX_TOKEN, |builder, bucket_id| {
            builder.call_function(package, "LendingApp", "instantiate_pool", args![Bucket(bucket_id),dec!(1000), dec!(10), dec!(7)])
        })    
        .build(executor.get_nonce([_beneficiary_public_key]))
        .sign([&sk]);
    let receipt_lending_app = executor.validate_and_execute(&transaction_lending_app).unwrap();
    println!("Package Lending created {:?} \n", receipt_lending_app);
    // assert!(receipt2.result.is_ok());
    let lending_component = receipt_lending_app.new_component_addresses.get(0).unwrap();
    println!("Lending Component Address  {} ", lending_component);
    let lending_badge = receipt_lending_app.new_resource_addresses.get(0).unwrap();
    println!("Lending Admin Badge  {} ", lending_badge);
    let lending_nft_resource_address = receipt_lending_app.new_resource_addresses.get(1).unwrap();
    println!("Lending NFT  {} ", lending_nft_resource_address);
    let borrow_nft = receipt_lending_app.new_resource_addresses.get(2).unwrap();
    println!("Borrowing NFT {} ", borrow_nft);
    let lnd_token = receipt_lending_app.new_resource_addresses.get(3).unwrap();
    println!("LND Token {} ", lnd_token);    

    // Creating a new blueprint PortfolioApp 
    let args_portfolio = args![RADIX_TOKEN, btc_resource_address, *lending_component, *trading_component, *lending_nft_resource_address, *borrow_nft, *lnd_token];
    let transaction_portfolio_app = TransactionBuilder::new()
        .call_function(package, "Portfolio", "new", args_portfolio)
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt_portfolio_app = executor.validate_and_execute(&transaction_portfolio_app).unwrap();
    println!("Package Portfolio created {:?} \n", receipt_portfolio_app);
    assert!(receipt_portfolio_app.result.is_ok());
    let portfolio_component = receipt_portfolio_app.new_component_addresses.get(0).unwrap();    
    println!("Package Component Address  {:?} \n", portfolio_component);

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
    let fund_token1_transaction = TransactionBuilder::new()
    .withdraw_from_account(RADIX_TOKEN, account)
    .take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |builder, bucket_id| {
        builder.call_method(*trading_component, "fund_token1", args![Bucket(bucket_id)])
    })
    .call_method_with_all_resources(account, "deposit_batch")
    .build(executor.get_nonce([_beneficiary_public_key]))
    .sign([&sk]);
    let register_fund_token1 = executor.validate_and_execute(&fund_token1_transaction).unwrap();
    println!("Method fund1 executed {:?} \n", register_fund_token1);
    assert!(register_fund_token1.result.is_ok());

    // Test the `fund_token2` method.
    let fund_token2_transaction = TransactionBuilder::new()
    .withdraw_from_account(btc_resource_address, _beneficiary_address)
    .take_from_worktop_by_amount(dec!("10"), btc_resource_address, |builder, bucket_id| {
        builder.call_method(*trading_component, "fund_token2", args![Bucket(bucket_id)])
    })
    .call_method_with_all_resources(_beneficiary_address, "deposit_batch")
    .build(executor.get_nonce([_beneficiary_public_key]))
    .sign([&_beneficiary_private_key]);
    let register_fund_token2 = executor.validate_and_execute(&fund_token2_transaction).unwrap();
    println!("Method fund2 executed {:?} \n", register_fund_token2);
    assert!(register_fund_token2.result.is_ok());

    
    // Test the `current_price` method
    let transaction_current_price = TransactionBuilder::new()
        .call_method(*trading_component, "current_price", args![btc_resource_address,eth_resource_address])
        // .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([_beneficiary_public_key]))
        .sign([&_beneficiary_private_key]);
    let current_price_receipt = executor.validate_and_execute(&transaction_current_price).unwrap();
    println!("Method current_price executed {:?}\n", current_price_receipt);
    assert!(current_price_receipt.result.is_ok());

    // Test the `fund_portfolio` method
    let fund_portfolio_transaction = TransactionBuilder::new()
    .withdraw_from_account(RADIX_TOKEN, _beneficiary_address)
    .take_from_worktop_by_amount(dec!("10000"), RADIX_TOKEN, |builder, bucket_id| {
        builder.call_method(*portfolio_component, "fund_portfolio", args![Bucket(bucket_id)])
    })
    .call_method_with_all_resources(_beneficiary_address, "deposit_batch")
    .build(executor.get_nonce([_beneficiary_public_key]))
    .sign([&_beneficiary_private_key]);
    let fund_portfolio_register = executor.validate_and_execute(&fund_portfolio_transaction).unwrap();
    println!("Method fund_portfolio executed {:?} \n", fund_portfolio_register);
    assert!(fund_portfolio_register.result.is_ok());

    // Test the `buy` method
    let component = receipt_trading_app.new_component_addresses[0];
    let transaction_buy = TransactionBuilder::new()
        .call_method(*portfolio_component, "buy", args![dec!(100), _beneficiary_address,btc_resource_address])
        // .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([_beneficiary_public_key]))
        .sign([&_beneficiary_private_key]);
    let buy_receipt = executor.validate_and_execute(&transaction_buy).unwrap();
    println!("Method buy executed {:?}\n", buy_receipt);
    assert!(buy_receipt.result.is_ok());

    // Test the `position` method
    let component = receipt_trading_app.new_component_addresses[0];
    let transaction_position = TransactionBuilder::new()
        .call_method(*portfolio_component, "position", args![])
        // .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([_beneficiary_public_key]))
        .sign([&_beneficiary_private_key]);
    let position_receipt = executor.validate_and_execute(&transaction_position).unwrap();
    println!("Method 'position' executed {:?}\n", position_receipt);
    assert!(position_receipt.result.is_ok());
    let log_message = &position_receipt.logs.get(0).unwrap().1;
    println!("Position Id needed for closing the position {:?}\n", log_message);

    // let losing_position: Vec<u128>  = position_receipt.into();


//     CALL_METHOD ComponentAddress("${account}") "withdraw_by_amount" Decimal("80") ResourceAddress("${xrd}");
// TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("80") ResourceAddress("${xrd}") Bucket("bucket1");
// CALL_METHOD ComponentAddress("${account}") "create_proof_by_amount" Decimal("1") ResourceAddress("${lend_nft}");
// POP_FROM_AUTH_ZONE Proof("proof1");
// CALL_METHOD ComponentAddress("${component}") "lend_money" Bucket("bucket1") Proof("proof1");
// CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("${account}") "deposit_batch";
}
