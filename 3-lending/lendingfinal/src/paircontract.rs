use scrypto::prelude::*;
use crate::lendingpool::*;

blueprint!{
    struct PairContract {
        asset: ResourceAddress,
        collateral: ResourceAddress,
        lending_pools: HashMap<Decimal, LendingPool>
    }

    impl PairContract {

        pub fn new(asset: ResourceAddress, collateral: ResourceAddress) -> ComponentAddress {
            return Self {
                asset: asset,
                collateral: collateral,
                lending_pools: HashMap::new()
            }
            .instantiate()
            .globalize();

        }

        pub fn pool_exists(
            &self,
            asset: ResourceAddress,
            collateral: ResourceAddress,
            maturity_time: Decimal
        ) -> bool {
            // Checking if lending pool with these parameters exist in the hashmap of lending pools or not.
            assert_eq!(asset, self.asset);
            assert_eq!(collateral, self.collateral);
            return self.lending_pools.contains_key(&maturity_time);
        }

        pub fn assert_pool_doesnt_exist(
            &self,
            asset: ResourceAddress,
            collateral: ResourceAddress,
            maturity_time: Decimal
        ) {
            assert!(
                !self.pool_exists(asset, collateral, maturity_time), 
                "A lending pool with the given address pair already exists."
            );
        }

        pub fn new_lending_pool(
            &mut self,
            asset: Bucket,
            collateral: Bucket,
            interest_rate: Decimal,
            maturity_time: Decimal,
        ) -> (Bucket, Bucket) {
            // Checking if a lending pool already exists between these two tokens
            self.assert_pool_doesnt_exist(
                asset.resource_address(), collateral.resource_address(), maturity_time
            );

            let (lending_pool, collateralized_debt_token, liquidity_tokens): (
                ComponentAddress, Bucket, Bucket
            ) = LendingPool::new(
                asset, collateral, interest_rate, maturity_time
            );

            // Adding the lending pool to the hashmap of all lending pools
            self.lending_pools.insert(maturity_time,lending_pool.into());

            // Returning the collateralized debt NFT and liquidity tokens back to the caller of this method 
            //(the initial liquidity provider).
            return (collateralized_debt_token, liquidity_tokens);
        }
    }




}
