use scrypto::prelude::*;
use crate::paircontract::*;


blueprint!{
    struct MescaLend {
        pair_contracts: HashMap<(ResourceAddress, ResourceAddress), PairContract>,
    }

    impl MescaLend {
        pub fn new() -> ComponentAddress {
            return Self {
                pair_contracts: HashMap::new(), 
            }
            .instantiate()
            .globalize();
        }

        pub fn pair_exists(
            &self,
            asset: ResourceAddress,
            collateral: ResourceAddress,
        ) -> bool {
            // Checking if pair contract with these parameters exist in the hashmap of pair contracts
            return self.pair_contracts.contains_key(&(asset, collateral));
        }

        pub fn assert_pair_doesnt_exist(
            &self,
            asset: ResourceAddress,
            collateral: ResourceAddress,
        ) {
            assert!(
                !self.pair_exists(asset, collateral), 
                "A lending pool with the given address pair already exists."
            );
        }

        pub fn new_pair_contract(
            &mut self,
            asset: ResourceAddress,
            collateral: ResourceAddress,
        ) {
            
            // Checking if a pair contract already exists for these two tokens
            self.assert_pair_doesnt_exist(
                asset, collateral
            );

            let addresses: (ResourceAddress,ResourceAddress) = (asset,collateral);

            let pair_contract: PairContract = PairContract::new(asset, collateral).into();

            // Adding the liquidity pool to the hashmap of all liquidity pools
            self.pair_contracts.insert(addresses,pair_contract.into());


        
            }


        }


}
    



