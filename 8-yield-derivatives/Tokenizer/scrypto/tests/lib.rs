use tokenizer::tokenizer::{test_bindings::*};
use scrypto::*;
use scrypto_test::prelude::*;
use scrypto::prelude::FungibleBucket;


#[test]
fn tokenizer_supply_tokenize_swap_success_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    println!("tokenizer_supply_tokenize_swap_success_test: {:?} ", package_address); 
    
    // Act
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    

    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = bucket2.resource_address(&mut env)?;

    // Act
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";

    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token1, package_address, &mut env,)?;

    // Act
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        /* Kernel modules are reset just after this point. */
    });

    // Act
    let userdata_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    assert_eq!(userdata_nft.0.amount(&mut env)?, dec!("1"));
    println!("Nft: {:?} ", userdata_nft);  

    // Act
    let (liquid_bucket, userdata_nft) = tokenizerdapp.supply(bucket1, userdata_nft, XRD, &mut env)?;

    // Assert
    let amount = liquid_bucket.amount(&mut env)?;
    assert_eq!(amount, dec!("100"));

    env.set_current_epoch(Epoch::of(100));
    // Act
    let (tokenized_bucket, userdata_nft) = tokenizerdapp.tokenize_yield(liquid_bucket, dec!(10000), userdata_nft, XRD,&mut env)?;
    assert_eq!(userdata_nft.0.amount(&mut env)?, dec!("1"));
    assert_eq!(tokenized_bucket.0.amount(&mut env)?, amount);

    env.set_current_epoch(Epoch::of(11001));
    // Act
    let (liquid_bucket, userdata_nft) = tokenizerdapp.redeem_from_pt(tokenized_bucket, userdata_nft, XRD,&mut env)?;
    assert_eq!(userdata_nft.0.amount(&mut env)?, dec!("1"));
    assert_eq!(liquid_bucket.amount(&mut env)?, amount);
    println!("liquid_bucket from reedem: {:?} ", liquid_bucket.amount(&mut env)?);  

    // Act
    let (liquid_bucket, userdata_nft) = tokenizerdapp.claim_yield(userdata_nft, XRD,&mut env)?;
    println!("liquid_bucket from claim: {:?} ", liquid_bucket.amount(&mut env)?);  
    assert_eq!(userdata_nft.0.amount(&mut env)?, dec!("1"));
    assert_eq!(liquid_bucket.amount(&mut env)?, dec!(0.95583));

    Ok(())
}


#[test]
fn tokenizer_supply_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    // Act
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);

    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = bucket2.resource_address(&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";

    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token1, package_address, &mut env,)?;

    // Act
    let user_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    assert_eq!(user_nft.0.amount(&mut env)?, dec!("1"));
    info!("Nft: {:?} ", _nft_bucket);  

    // // Act
    let (liquid_bucket, _nft_bucket) = tokenizerdapp.supply(bucket1, user_nft, XRD, &mut env)?;

    // Assert
    assert_eq!(liquid_bucket.amount(&mut env)?, dec!("100"));
    Ok(())
}



#[test]
fn tokenizer_takes_back_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();

    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    // Act
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = bucket2.resource_address(&mut env)?;

    // Act
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";

    let (mut tokenizerdapp, _admin_badge, _owner_badge) = Tokenizer::instantiate(
        reward, symbol, reward_type.to_owned(), XRD, token1, package_address, &mut env,)?;
    
    // Act
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        /* Kernel modules are reset just after this point. */
    });
    // Act
    let user_nft = tokenizerdapp.register(&mut env)?;
    // Act
    let (liquid_bucket, received_nft) = tokenizerdapp.supply(bucket1, user_nft, XRD, &mut env)?;

    // Verify that the received buckets amount matches the expected amount
    // Assert
    assert_eq!(received_nft.0.amount(&mut env)?, dec!("1"));
    assert_eq!(liquid_bucket.amount(&mut env)?, dec!("100"));

    info!("Nft: {:?} ", _nft_bucket);  

    env.set_current_epoch(Epoch::of(10000));
    // Act
    let (xrd_bucket, nft_option) = tokenizerdapp.takes_back(liquid_bucket, received_nft,XRD, &mut env)?;

    match nft_option {
        Some(nft) => {
            // Verify that the nft has been correctly burned
            assert_eq!(nft.0.amount(&mut env)?, dec!("1"));
            // Verify that the reward has been received
            assert_eq!(xrd_bucket.amount(&mut env)?, dec!("100.47668"));
        }
        None => {
            // Verify that the reward has been received
            assert_eq!(xrd_bucket.amount(&mut env)?, dec!("100.47668"));
        }
    }

    Ok(())
}



#[test]
fn tokenizer_supply_and_tokenize_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    println!("tokenizer_supply_and_tokenize_test: {:?} ", package_address); 
    
    // Act
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = bucket2.resource_address(&mut env)?;

    // Act
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";

    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token1, package_address, &mut env,)?;

    // Act
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        /* Kernel modules are reset just after this point. */
    });

    // Act
    let user_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    assert_eq!(user_nft.0.amount(&mut env)?, dec!("1"));
    println!("Nft: {:?} ", user_nft);  

    // Act
    let (liquid_bucket, nft_bucket) = tokenizerdapp.supply(bucket1, user_nft, XRD, &mut env)?;

    // Assert
    let amount = liquid_bucket.amount(&mut env)?;
    assert_eq!(amount, dec!("100"));

    env.set_current_epoch(Epoch::of(100));
    // Act
    let (tokenized_bucket, userdata_nft) = tokenizerdapp.tokenize_yield(liquid_bucket, dec!(10000), nft_bucket,XRD, &mut env)?;
    assert_eq!(userdata_nft.0.amount(&mut env)?, dec!("1"));
    assert_eq!(tokenized_bucket.0.amount(&mut env)?, amount);

    env.set_current_epoch(Epoch::of(9999));
    // Act
    let (_liquid_bucket, _userdata_nft) = tokenizerdapp.redeem_from_pt(tokenized_bucket, userdata_nft, XRD,&mut env)?;


    Ok(())
}



#[test]
fn tokenizer_supply_and_tokenize_success_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    println!("tokenizer_supply_and_tokenize_success_test: {:?} ", package_address); 
    
    // Act
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = bucket2.resource_address(&mut env)?;

    // Act
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";

    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token1, package_address, &mut env,)?;

    // Act
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        /* Kernel modules are reset just after this point. */
    });

    // Act
    let user_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    assert_eq!(user_nft.0.amount(&mut env)?, dec!("1"));
    println!("Nft: {:?} ", user_nft);  

    // Act
    let (liquid_bucket, nft_bucket) = tokenizerdapp.supply(bucket1, user_nft, XRD, &mut env)?;

    // Assert
    let amount = liquid_bucket.amount(&mut env)?;
    assert_eq!(amount, dec!("100"));

    env.set_current_epoch(Epoch::of(100));

    // Act
    let (tokenized_bucket, userdata_nft) = tokenizerdapp.tokenize_yield(liquid_bucket, dec!(10000), nft_bucket,XRD, &mut env)?;
    assert_eq!(userdata_nft.0.amount(&mut env)?, dec!("1"));
    assert_eq!(tokenized_bucket.0.amount(&mut env)?, amount);

    env.set_current_epoch(Epoch::of(11001));
    // Act
    let (liquid_bucket, userdata_nft) = tokenizerdapp.redeem_from_pt(tokenized_bucket, userdata_nft, XRD,&mut env)?;
    assert_eq!(userdata_nft.0.amount(&mut env)?, dec!("1"));
    assert_eq!(liquid_bucket.amount(&mut env)?, amount);
    println!("liquid_bucket from reedem: {:?} ", liquid_bucket.amount(&mut env)?);  

    // Act
    let (liquid_bucket, userdata_nft) = tokenizerdapp.claim_yield(userdata_nft, XRD,&mut env)?;
    println!("liquid_bucket from claim: {:?} ", liquid_bucket.amount(&mut env)?);  
    assert_eq!(userdata_nft.0.amount(&mut env)?, dec!("1"));
    assert_eq!(liquid_bucket.amount(&mut env)?, dec!(0.95583));

    Ok(())
}


