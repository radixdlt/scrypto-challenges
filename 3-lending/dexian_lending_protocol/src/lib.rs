mod assetstate;
mod definterestmodel;
mod stableinterestmodel;
mod cdp;

use scrypto::prelude::*;

use assetstate::*;
use cdp::*;


blueprint! {
    struct LendingPool {
        // asset price oracle
        oracle_addr: ComponentAddress,
        //Status of each asset in the lending pool
       states: HashMap<ResourceAddress, AssetState>,
       // address map for supply token(K) and deposit asset(V)
       origin_asset_map: HashMap<ResourceAddress, ResourceAddress>,
       // vault for each collateral asset(supply token)
       collateral_vaults: HashMap<ResourceAddress, Vault>,
       // Cash of each asset in the lending pool
       vaults: HashMap<ResourceAddress, Vault>,
       // CDP token for each loan asset. <loan_asset, CDP token address>
       cdp_nfts: HashMap<ResourceAddress, ResourceAddress>,
       // CDP id counter
       cdp_id_counter: u64,
       // lending pool admin badge.
       admin_badge: ResourceAddress,
       // minter
       minter: Vault,

    }

    impl LendingPool {
        
        pub fn instantiate_asset_pool(oracle_addr: ComponentAddress) -> (ComponentAddress, Bucket) {
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
                states: HashMap::new(),
                origin_asset_map: HashMap::new(),
                collateral_vaults: HashMap::new(),
                vaults: HashMap::new(),
                cdp_nfts: HashMap::new(),
                cdp_id_counter: 0u64,
                minter: Vault::with_bucket(minter),
                admin_badge: admin_badge.resource_address(),
                oracle_addr
            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            (component, admin_badge)
        }

        
        pub fn new_pool(&mut self, asset_address: ResourceAddress, 
            ltv: Decimal,
            liquidation_threshold: Decimal,
            liquidation_bonus: Decimal,
            insurance_ratio: Decimal, 
            interest_model: ComponentAddress) -> ResourceAddress  {
            let res_mgr = borrow_resource_manager!(asset_address);
            // TODO: concat string + "dx"
            let origin_symbol = res_mgr.metadata()["symbol"].clone();
            let supply_token = ResourceBuilder::new_fungible()
                .metadata("symbol", format!("dx{}", origin_symbol))
                .metadata("name", format!("DeXian Lending LP token({}) ", origin_symbol))
                .mintable(rule!(require(self.minter.resource_address())), LOCKED)
                .burnable(rule!(require(self.minter.resource_address())), LOCKED)
                .no_initial_supply();

            if ltv > Decimal::ZERO {
                let cdp_token = ResourceBuilder::new_non_fungible()
                    .metadata("symbol", "CDP")
                    .metadata("name", format!("DeXian CDP({})", origin_symbol))
                    .mintable(rule!(require(self.minter.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter.resource_address())), LOCKED)
                    .no_initial_supply();
                self.cdp_nfts.insert(asset_address, cdp_token);
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
                ltv,
                liquidation_threshold,
                liquidation_bonus,
                insurance_ratio,
                interest_model
            };

            self.states.insert(asset_address, asset_state);
            self.origin_asset_map.insert(supply_token, asset_address);
            self.vaults.insert(asset_address, Vault::new(asset_address));
            supply_token
        }

        pub fn supply(&mut self, deposit_asset: Bucket) -> Bucket {
            let asset_address = deposit_asset.resource_address();
            // let res_mgr = borrow_resource_manager!();
            assert!(self.states.contains_key(&asset_address) && self.vaults.contains_key(&asset_address), "There is no pool of funds corresponding to the assets!");
            let asset_state = self.states.get_mut(&asset_address).unwrap();
            
            asset_state.update_index();

            let amount = deposit_asset.amount();
            let vault = self.vaults.get_mut(&asset_address).unwrap();
            vault.put(deposit_asset);

            let normalized_amount = LendingPool::floor(amount / asset_state.supply_index);
            
            let supply_token = self.minter.authorize(|| {
                let supply_res_mgr: &ResourceManager = borrow_resource_manager!(asset_state.token);
                supply_res_mgr.mint(normalized_amount)
            });

            asset_state.update_interest_rate();
            //TODO: log
            supply_token
        }

        pub fn withdraw(&mut self, supply_token: Bucket) -> Bucket {
            let token_address = supply_token.resource_address();
            assert!(self.origin_asset_map.contains_key(&token_address), "unsupported the token!");
            let amount = supply_token.amount();
            let asset_address = self.origin_asset_map.get(&token_address).unwrap();
            let asset_state = self.states.get_mut(&asset_address).unwrap();

            asset_state.update_index();

            let normalized_amount = LendingPool::floor(amount * asset_state.supply_index);
            //TODO: check borrow and collateral debt position
            //check cash balance and amount
            self.minter.authorize(|| {
                let supply_res_mgr: &ResourceManager = borrow_resource_manager!(asset_state.token);
                supply_res_mgr.burn(supply_token);
            });
            let vault = self.vaults.get_mut(&asset_address).unwrap();
            let asset_bucket = vault.take(normalized_amount);
            asset_state.update_interest_rate();
            //TODO: log
            asset_bucket
        }

        pub fn borrow(&mut self, supply_token: Bucket, borrow_token: ResourceAddress, amount: Decimal) -> (Bucket, Bucket){
            assert!(self.states.contains_key(&borrow_token), "unsupported the borrow token!");
            let token_address = supply_token.resource_address();
            assert!(self.origin_asset_map.contains_key(&token_address), "unsupported the collateral token!");
            
            let collateral_addr = self.origin_asset_map.get(&token_address).unwrap();
            debug!("borrow supply_token {}, collateral_addr {}, ", token_address, collateral_addr);
            let collateral_state = self.states.get_mut(&collateral_addr).unwrap();
            assert!(collateral_state.ltv > Decimal::ZERO, "Then token is not colleteral asset!");

            collateral_state.update_index();

            let supply_amount = supply_token.amount();
            let deposit_amount = LendingPool::floor(supply_token.amount() * collateral_state.supply_index);
            let max_loan_amount = self.get_max_loan_amount(collateral_addr, deposit_amount, collateral_state.ltv, borrow_token);
            let borrow_amount = [max_loan_amount, amount].iter().min();

            let collateral_vault = self.collateral_vaults.get_mut(&collateral_addr).unwrap();
            collateral_vault.put(supply_token);

            
            let borrow_asset_state = self.states.get_mut(&borrow_token).unwrap();
            borrow_asset_state.update_index();
            
            let borrow_normalized_amount = LendingPool::ceil(borrow_amount / borrow_asset_state.borrow_index);
            borrow_asset_state.normalized_total_borrow += borrow_normalized_amount;
            borrow_asset_state.update_interest_rate();

            let borrow_vault = self.vaults.get_mut(&borrow_token).unwrap();
            let borrow_bucket = borrow_vault.take(borrow_amount);

            let data = CollateralDebtPosition{
                collateral_token: collateral_addr.clone(),
                total_borrow: borrow_amount,
                total_repay: Decimal::ZERO,
                normalized_borrow: borrow_normalized_amount,
                collateral_amount: supply_amount,
                borrow_amount: borrow_amount,
                last_update_epoch: Runtime::current_epoch(),
                borrow_token
            };
            let cdp_nft_res_addr = self.cdp_nfts.get(&borrow_token).unwrap();
            let cdp = self.minter.authorize(|| {
                self.cdp_id_counter += 1;
                let cdp_res_mgr: &ResourceManager = borrow_resource_manager!(*cdp_nft_res_addr);
                cdp_res_mgr.mint_non_fungible(&NonFungibleId::from_u64(self.cdp_id_counter), data)
            });
            (borrow_bucket, cdp)
        }

        fn get_max_loan_amount(&self, deposit_asset: ResourceAddress, deposit_amount: Decimal, ltv: Decimal, borrow_asset: ResourceAddress) -> Decimal{
            deposit_amount * self.get_asset_price(deposit_asset) * ltv / self.get_asset_price(borrow_asset)
        }

        fn get_asset_price(&self, asset_addr: ResourceAddress) -> Decimal{
            let component: &Component = borrow_component!(self.oracle_addr);
            component.call::<Decimal>("get_price_quote_in_xrd", args![asset_addr])
        }

        fn ceil(dec: Decimal) -> Decimal{
            dec.round(18u8, RoundingMode::TowardsPositiveInfinity)
        }

        fn floor(dec: Decimal) -> Decimal{
            dec.round(18u8, RoundingMode::TowardsNegativeInfinity)
        }
    }
}
