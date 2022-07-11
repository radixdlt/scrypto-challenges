mod assetstate;
use scrypto::prelude::*;

use assetstate::*;

blueprint! {
    struct LendingPool {
        //Status of each asset in the lending pool
       states: LazyMap<ResourceAddress, AssetState>,
       origin_asset_map: LazyMap<ResourceAddress, ResourceAddress>,
       // Cash of each asset in the lending pool
       vaults: LazyMap<ResourceAddress, Vault>,

       def_insurance_ratio: Decimal,
       
       // lending pool admin badge.
       admin_badge: ResourceAddress,
       
       minter: Vault,

    }

    impl LendingPool {
        
        pub fn instantiate_asset_pool(def_insurance_ratio: Decimal) -> (ComponentAddress, Bucket) {
            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("1"));

            let minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("1"));
            
            let rules = AccessRules::new()
                .method("new_pool", rule!(require(admin_badge.resource_address())))
                // .method("withdraw_fees", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            // Instantiate a LendingPool component
            let component = LendingPool {
                states: LazyMap::new(),
                origin_asset_map: LazyMap::new(),
                vaults: LazyMap::new(),
                minter: Vault::with_bucket(minter),
                admin_badge: admin_badge.resource_address(),
                def_insurance_ratio
            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            (component, admin_badge)
        }

        
        pub fn new_pool(&mut self, asset_address: ResourceAddress, _insurance_ratio: Decimal) -> ResourceAddress  {
            let res_mgr = borrow_resource_manager!(asset_address);
            let origin_symbol = res_mgr.metadata()["symbol"].clone();
            let supply_token = ResourceBuilder::new_fungible()
                .metadata("symbol", origin_symbol)
                .mintable(rule!(require(self.minter.resource_address())), LOCKED)
                .burnable(rule!(require(self.minter.resource_address())), LOCKED)
                .no_initial_supply();
            let mut insurance_ratio = self.def_insurance_ratio;
            if _insurance_ratio > Decimal::ZERO {
                insurance_ratio = _insurance_ratio;
            }
            
            let asset_state = AssetState {
                supply_index: Decimal::ONE,
                borrow_index: Decimal::ONE,
                borrow_interest_rate: Decimal::ZERO,
                supply_interest_rate: Decimal::ZERO,
                insurance_balance: Decimal::ZERO,
                token: supply_token,
                normalized_total_borrow: Decimal::ZERO,
                last_update_epoch: Runtime::current_epoch(),
                insurance_ratio
            };

            self.states.insert(asset_address, asset_state);
            self.vaults.insert(asset_address, Vault::new());
            self.origin_asset_map.insert(supply_token, asset_address);
            supply_token
        }

        pub fn supply(&mut self, deposit_asset: Bucket) -> Bucket {
            let asset_address = deposit_asset.resource_address();
            // let res_mgr = borrow_resource_manager!();
            assert!(self.states.contains_key(asset_address) && self., "There is no pool of funds corresponding to the assets!");
            let asset_state = self.states.get(asset_address);
            asset_state.update_index();

            let amount = deposit_asset.amount();
            let vault = self.vaults.get(asset_address);
            vault.put(deposit_asset.take_all());

            


        }
    }
}