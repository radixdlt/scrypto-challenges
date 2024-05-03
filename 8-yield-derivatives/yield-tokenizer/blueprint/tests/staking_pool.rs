use radix_engine_interface::prelude::*;
use scrypto::*;
use scrypto_test::prelude::*;
use yield_tokenizer::staking_pool::test_bindings::StakingPool;

pub fn arrange() -> Result<
    (
        TestEnvironment,
        StakingPool,
        Vec<IntegerNonFungibleLocalId>,
        Bucket,
        Bucket,
    ),
    RuntimeError,
> {
    let mut env = TestEnvironment::new();

    env.enable_transaction_runtime_module();

    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    let lsu_bucket = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(18)
        .mint_initial_supply(1_000_000, &mut env)?;

    let sxrd_bucket = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(18)
        .mint_initial_supply(1_000_000, &mut env)?;

    let staking_pool = StakingPool::instantiate(
        lsu_bucket.resource_address(&mut env)?,
        sxrd_bucket.resource_address(&mut env)?,
        package_address,
        &mut env,
    )?;

    let mut ids: Vec<IntegerNonFungibleLocalId> = Vec::new();

    for i in 0..6 {
        ids.push(IntegerNonFungibleLocalId::new(i as u64));
    }

    Ok((env, staking_pool, ids, lsu_bucket, sxrd_bucket))
}

#[test]
pub fn asset_pool_base() -> Result<(), RuntimeError> {
    let (mut env, mut staking_pool, ids, lsu, sxrd) = arrange()?;

    let id_0: NonFungibleLocalId = ids[0].clone().into();
    let id_1: NonFungibleLocalId = ids[1].clone().into();
    let id_2: NonFungibleLocalId = ids[2].clone().into();
    let id_3: NonFungibleLocalId = ids[3].clone().into();
    let id_4: NonFungibleLocalId = ids[4].clone().into();
    let id_5: NonFungibleLocalId = ids[5].clone().into();

    // Epoch 0

    // Contribution 0 (User 0)

    staking_pool.contribute(
        id_0.clone(),
        FungibleBucket(lsu.take(dec!(100), &mut env)?),
        &mut env,
    )?;

    // Distribution 0

    staking_pool.distribute(FungibleBucket(sxrd.take(dec!(100), &mut env)?), &mut env)?;

    // Contribution 1 (User 1)

    staking_pool.contribute(
        id_1.clone(),
        FungibleBucket(lsu.take(dec!(100), &mut env)?),
        &mut env,
    )?;

    // Distribution 1

    staking_pool.distribute(FungibleBucket(sxrd.take(dec!(100), &mut env)?), &mut env)?;

    // Contribution 2 (User 2)

    staking_pool.contribute(
        id_2.clone(),
        FungibleBucket(lsu.take(dec!(200), &mut env)?),
        &mut env,
    )?;

    // Withdraw 1

    let lsu_withdraw = staking_pool.withdraw(dec!(200), &mut env)?;
    assert_eq!(lsu_withdraw.0.amount(&mut env)?, dec!(200));

    // Distribution 2

    staking_pool.distribute(FungibleBucket(sxrd.take(dec!(100), &mut env)?), &mut env)?;

    // Withdraw 2 (Empty the pool to trigger a new epoch)

    let lsu_withdraw = staking_pool.withdraw(dec!(200), &mut env)?;
    assert_eq!(lsu_withdraw.0.amount(&mut env)?, dec!(200));

    // Epoch 1

    // Contribution 3 (User 3)

    staking_pool.contribute(
        id_3.clone(),
        FungibleBucket(lsu.take(dec!(100), &mut env)?),
        &mut env,
    )?;

    // Distribution 3

    staking_pool.distribute(FungibleBucket(sxrd.take(dec!(100), &mut env)?), &mut env)?;

    // Contribution 4 (User 4)

    staking_pool.contribute(
        id_4.clone(),
        FungibleBucket(lsu.take(dec!(100), &mut env)?),
        &mut env,
    )?;

    // Distribution 4

    staking_pool.distribute(FungibleBucket(sxrd.take(dec!(100), &mut env)?), &mut env)?;

    // Contribution 5 (User 5)

    staking_pool.contribute(
        id_5.clone(),
        FungibleBucket(lsu.take(dec!(200), &mut env)?),
        &mut env,
    )?;

    // Distribution 5

    staking_pool.distribute(FungibleBucket(sxrd.take(dec!(100), &mut env)?), &mut env)?;

    // Redemptions Epoch 0

    // All user have no more LSU as the pool had been emptied by a withdraw

    let (id_0_lsu, id_0_sxrd) = staking_pool.redeem(id_0.clone(), &mut env)?;
    assert_eq!(id_0_lsu.0.amount(&mut env)?, dec!(0));
    assert_eq!(id_0_sxrd.0.amount(&mut env)?, dec!(175)); // 100 (100% of dist 0) + 50 (50% of dist 1) + 25 (25% of dist 2)

    let (id_1_lsu, id_1_sxrd) = staking_pool.redeem(id_1.clone(), &mut env)?;
    assert_eq!(id_1_lsu.0.amount(&mut env)?, dec!(0));
    assert_eq!(id_1_sxrd.0.amount(&mut env)?, dec!(75)); // 50 (50% of dist 1) + 25 (25% of dist 2)

    let (id_2_lsu, id_2_sxrd) = staking_pool.redeem(id_2.clone(), &mut env)?;
    assert_eq!(id_2_lsu.0.amount(&mut env)?, dec!(0));
    assert_eq!(id_2_sxrd.0.amount(&mut env)?, dec!(50)); // 50 (50% of dist 2)

    // Redemptions Epoch 1

    // No withdrawn of LSU so all contributors keep their LSU

    let (id_3_lsu, id_3_sxrd) = staking_pool.redeem(id_3.clone(), &mut env)?;
    assert_eq!(id_3_lsu.0.amount(&mut env)?, dec!(100));
    assert_eq!(id_3_sxrd.0.amount(&mut env)?, dec!(175)); // 100 (100% of dist 3) + 50 (50% of dist 4) + 25 (25% of dist 5)

    let (id_4_lsu, id_4_sxrd) = staking_pool.redeem(id_4.clone(), &mut env)?;
    assert_eq!(id_4_lsu.0.amount(&mut env)?, dec!(100));
    assert_eq!(id_4_sxrd.0.amount(&mut env)?, dec!(75)); // 50 (50% of dist 4) + 25 (25% of dist 5)

    let (id_5_lsu, id_5_sxrd) = staking_pool.redeem(id_5.clone(), &mut env)?;
    assert_eq!(id_5_lsu.0.amount(&mut env)?, dec!(200));
    assert_eq!(id_5_sxrd.0.amount(&mut env)?, dec!(50)); // 50 (50% of dist 5)

    // No more contributions
    // Withdraw and distribute should fail

    // Withdraw on empty pool

    // let withdraw_result = staking_pool.withdraw(dec!(200), &mut env);
    // match withdraw_result {
    //     Ok(lsu) => {
    //         println!("{}", lsu.0.amount(&mut env)?);
    //         assert!(false, "Should have failed");
    //     }
    //     Err(_) => assert!(true),
    // }

    // distribute on empty pool

    let distribute_result =
        staking_pool.distribute(FungibleBucket(sxrd.take(dec!(100), &mut env)?), &mut env);

    match distribute_result {
        Ok(_) => assert!(false, "Should have failed"),
        Err(_) => assert!(true),
    }

    Ok(())
}
