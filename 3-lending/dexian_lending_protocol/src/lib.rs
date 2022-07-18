mod assetstate;
mod definterestmodel;
mod stableinterestmodel;
mod cdp;
mod oracle;

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
        // CDP token define
        cdp_res_addr: ResourceAddress,
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
            
            let cdp_res_addr = ResourceBuilder::new_non_fungible()
                .metadata("symbol", "CDP")
                .metadata("name", "DeXian CDP Token")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minter.resource_address())), LOCKED)
                .no_initial_supply();
            
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
                cdp_id_counter: 0u64,
                minter: Vault::with_bucket(minter),
                admin_badge: admin_badge.resource_address(),
                cdp_res_addr,
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

            let origin_symbol = res_mgr.metadata()["symbol"].clone();
            let dx_token = ResourceBuilder::new_fungible()
                .metadata("symbol", format!("dx{}", origin_symbol))
                .metadata("name", format!("DeXian Lending LP token({}) ", origin_symbol))
                .mintable(rule!(require(self.minter.resource_address())), LOCKED)
                .burnable(rule!(require(self.minter.resource_address())), LOCKED)
                .no_initial_supply();
            
            let asset_state = AssetState {
                supply_index: Decimal::ONE,
                borrow_index: Decimal::ONE,
                borrow_interest_rate: Decimal::ZERO,
                supply_interest_rate: Decimal::ZERO,
                insurance_balance: Decimal::ZERO,
                token: dx_token,
                normalized_total_borrow: Decimal::ZERO,
                last_update_epoch: Runtime::current_epoch(),
                ltv,
                liquidation_threshold,
                liquidation_bonus,
                insurance_ratio,
                interest_model
            };

            self.states.insert(asset_address, asset_state);
            self.origin_asset_map.insert(dx_token, asset_address);
            self.vaults.insert(asset_address, Vault::new(asset_address));
            dx_token
        }

        pub fn supply(&mut self, deposit_asset: Bucket) -> Bucket {
            let asset_address = deposit_asset.resource_address();
            assert!(self.states.contains_key(&asset_address) && self.vaults.contains_key(&asset_address), "There is no pool of funds corresponding to the assets!");
            let asset_state = self.states.get_mut(&asset_address).unwrap();
            
            debug!("before update_index, asset_address{} indexes:{},{}", asset_address, asset_state.borrow_index, asset_state.supply_index);
            asset_state.update_index();
            debug!("after update_index, asset_address{} indexes:{},{}", asset_address, asset_state.borrow_index, asset_state.supply_index);

            let amount = deposit_asset.amount();
            let vault = self.vaults.get_mut(&asset_address).unwrap();
            vault.put(deposit_asset);

            let normalized_amount = LendingPool::floor(amount / asset_state.supply_index);
            
            let dx_token_bucket = self.minter.authorize(|| {
                let supply_res_mgr: &ResourceManager = borrow_resource_manager!(asset_state.token);    
                supply_res_mgr.mint(normalized_amount)
            });

            asset_state.update_interest_rate();
            debug!("{}, supply:{}, borrow:{}, rate:{},{}", asset_address, asset_state.get_total_normalized_supply(), asset_state.normalized_total_borrow, asset_state.borrow_interest_rate, asset_state.supply_interest_rate);

            dx_token_bucket
        }

        pub fn withdraw(&mut self, dx_bucket: Bucket) -> Bucket {
            let dx_address = dx_bucket.resource_address();
            assert!(self.origin_asset_map.contains_key(&dx_address), "unsupported the token!");
            let amount = dx_bucket.amount();
            let asset_address = self.origin_asset_map.get(&dx_address).unwrap();
            let asset_state = self.states.get_mut(&asset_address).unwrap();

            debug!("before update_index, asset_address{} indexes:{},{}", asset_address, asset_state.borrow_index, asset_state.supply_index);
            asset_state.update_index();
            debug!("after update_index, asset_address{} indexes:{},{}", asset_address, asset_state.borrow_index, asset_state.supply_index);

            let normalized_amount = LendingPool::floor(amount * asset_state.supply_index);
            self.minter.authorize(|| {
                let supply_res_mgr: &ResourceManager = borrow_resource_manager!(asset_state.token);
                supply_res_mgr.burn(dx_bucket);
            });
            let vault = self.vaults.get_mut(&asset_address).unwrap();
            let asset_bucket = vault.take(normalized_amount);
            asset_state.update_interest_rate();
            debug!("{}, supply:{}, borrow:{}, rate:{},{}", asset_address, asset_state.get_total_normalized_supply(), asset_state.normalized_total_borrow, asset_state.borrow_interest_rate, asset_state.supply_interest_rate);
            asset_bucket
        }

        pub fn borrow(&mut self, dx_bucket: Bucket, borrow_token: ResourceAddress, mut amount: Decimal) -> (Bucket, Bucket){
            assert!(self.states.contains_key(&borrow_token), "unsupported the borrow token!");
            let dx_address = dx_bucket.resource_address();
            assert!(self.origin_asset_map.contains_key(&dx_address), "unsupported the collateral token!");
            
            let collateral_addr = self.origin_asset_map.get(&dx_address).unwrap();
            debug!("borrow dx_bucket {}, collateral_addr {}, ", dx_address, collateral_addr);
            let collateral_state = self.states.get_mut(collateral_addr).unwrap();
            assert!(collateral_state.ltv > Decimal::ZERO, "Then token is not colleteral asset!");
            
            collateral_state.update_index();
            
            let supply_index = collateral_state.supply_index;
            let ltv = collateral_state.ltv;
            let supply_amount = dx_bucket.amount();

            let deposit_amount = LendingPool::floor(supply_amount * supply_index);
            let max_loan_amount = self.get_max_loan_amount(collateral_addr.clone(), deposit_amount, ltv, borrow_token);
            debug!("max loan amount {}, supply_amount:{} deposit_amount:{}, amount:{}", max_loan_amount, supply_amount, deposit_amount, amount);
            if amount > max_loan_amount {
                amount = max_loan_amount;
            }

            if self.collateral_vaults.contains_key(&dx_address){
                let collateral_vault = self.collateral_vaults.get_mut(&dx_address).unwrap();
                collateral_vault.put(dx_bucket);
            }
            else{
                let vault = Vault::with_bucket(dx_bucket);
                self.collateral_vaults.insert(dx_address, vault);
            }

            
            let borrow_asset_state = self.states.get_mut(&borrow_token).unwrap();
            borrow_asset_state.update_index();
            
            let borrow_normalized_amount = LendingPool::ceil(amount / borrow_asset_state.borrow_index);
            borrow_asset_state.normalized_total_borrow += borrow_normalized_amount;
            borrow_asset_state.update_interest_rate();
            debug!("{}, supply:{}, borrow:{}, rate:{},{}", borrow_token, borrow_asset_state.get_total_normalized_supply(), borrow_asset_state.normalized_total_borrow, borrow_asset_state.borrow_interest_rate, borrow_asset_state.supply_interest_rate);

            let borrow_vault = self.vaults.get_mut(&borrow_token).unwrap();
            let borrow_bucket = borrow_vault.take(amount);

            let data = CollateralDebtPosition{
                collateral_token: dx_address.clone(),
                total_borrow: amount,
                total_repay: Decimal::ZERO,
                normalized_borrow: borrow_normalized_amount,
                collateral_amount: supply_amount,
                borrow_amount: amount,
                last_update_epoch: Runtime::current_epoch(),
                borrow_token
            };

            let cdp = self.minter.authorize(|| {
                self.cdp_id_counter += 1;
                let cdp_res_mgr: &ResourceManager = borrow_resource_manager!(self.cdp_res_addr);
                cdp_res_mgr.mint_non_fungible(&NonFungibleId::from_u64(self.cdp_id_counter), data)
            });
            (borrow_bucket, cdp)
        }

        pub fn repay(&mut self, mut repay_token: Bucket, cdp: Bucket) -> (Bucket, Bucket, Option<Bucket>) {
            assert!(
                cdp.amount() == dec!("1"),
                "We can only handle one CDP each time!"
            );

            let cdp_id = cdp.non_fungible::<CollateralDebtPosition>().id();
            let mut cdp_data: CollateralDebtPosition = cdp.non_fungible().data();
            let borrow_token = cdp_data.borrow_token;
            assert!(borrow_token == repay_token.resource_address(), "Must return borrowed coin.");

            let borrow_state = self.states.get_mut(&borrow_token).unwrap();
            debug!("before update_index, borrow normalized:{} total_borrow_normailized:{} indexes:{},{}", cdp_data.normalized_borrow, borrow_state.normalized_total_borrow, borrow_state.supply_index, borrow_state.borrow_index);
            borrow_state.update_index();
            debug!("after update_index, borrow normalized:{} total_borrow_normailized:{} indexes:{},{}", cdp_data.normalized_borrow, borrow_state.normalized_total_borrow, borrow_state.supply_index, borrow_state.borrow_index);
            let borrow_index = borrow_state.borrow_index;
            assert!(borrow_index > Decimal::ZERO, "borrow index error! {}", borrow_index);
            let mut normalized_amount = LendingPool::floor(repay_token.amount() / borrow_index);
            let mut repay_amount = repay_token.amount();
            

            let mut collateral_bucket: Option<Bucket> = None;
            if cdp_data.normalized_borrow <= normalized_amount {
                // repayAmount <= amount
                // because ⌈⌊a/b⌋*b⌉ <= a
                repay_amount = LendingPool::ceil(cdp_data.normalized_borrow * borrow_index);
                normalized_amount = cdp_data.normalized_borrow;

                let dx_address = cdp_data.collateral_token;
                let collateral_vault = self.collateral_vaults.get_mut(&dx_address).unwrap();
                collateral_bucket = Some(collateral_vault.take(cdp_data.collateral_amount));
                
                cdp_data.collateral_amount = Decimal::ZERO;
                
                // self.minter.authorize(|| {
                //     cdp.burn();
                // });
                // return (repay_token, collateral_bucket);
            }
            debug!("repay_bucket:{}, normalized_amount:{}, normalized_borrow:{}, repay_amount:{}", repay_amount, normalized_amount, cdp_data.normalized_borrow, repay_amount);
            let borrow_vault = self.vaults.get_mut(&borrow_token).unwrap();
            borrow_vault.put(repay_token.take(repay_amount));

            cdp_data.total_repay += repay_amount;
            cdp_data.normalized_borrow -= normalized_amount;
            cdp_data.last_update_epoch = Runtime::current_epoch();
            borrow_state.normalized_total_borrow -= normalized_amount;

            borrow_state.update_interest_rate();
            
            self.minter.authorize(|| {
                let cdp_res_mgr: &ResourceManager = borrow_resource_manager!(cdp.resource_address());
                cdp_res_mgr.update_non_fungible_data(&cdp_id, cdp_data);
            });

            (repay_token, cdp, collateral_bucket)
            
        }

        pub fn liquidation(&mut self, mut debt_bucket: Bucket, cdp_id: u64) -> Bucket{
            let (collateral, debt, collateral_in_xrd, debt_in_xrd, collateral_price, _) = self.get_cdp_digest(cdp_id);
            assert!(debt == debt_bucket.resource_address(), "The CDP can not support the repay by the bucket!");
            let collateral_state = self.states.get_mut(&collateral).unwrap();
            assert!(collateral_state.liquidation_threshold >= debt_in_xrd / collateral_in_xrd, "The CDP can not be liquidation yet, the timing too early!");
            collateral_state.update_index();
            let liquidation_bonus = collateral_state.liquidation_bonus;
            let collateral_supply_index = collateral_state.supply_index;

            let debt_state = self.states.get_mut(&debt).unwrap();
            debt_state.update_index();
            debug!("before update_index, borrow in xrd:{} total_borrow_normailized:{} indexes:{},{}", debt_in_xrd, debt_state.normalized_total_borrow, debt_state.supply_index, debt_state.borrow_index);
            debt_state.update_index();
            debug!("after update_index, borrow in xrd:{} total_borrow_normailized:{} indexes:{},{}", debt_in_xrd, debt_state.normalized_total_borrow, debt_state.supply_index, debt_state.borrow_index);
            let borrow_index = debt_state.borrow_index;
            assert!(borrow_index > Decimal::ZERO, "borrow index error! {}", borrow_index);
            let mut normalized_amount = LendingPool::floor(debt_bucket.amount() / borrow_index);

            let mut cdp_data: CollateralDebtPosition = borrow_resource_manager!(self.cdp_res_addr).get_non_fungible_data(&NonFungibleId::from_u64(cdp_id));
            assert!(cdp_data.normalized_borrow <= normalized_amount,  "Underpayment of value of debt!");
            // repayAmount <= amount
            // because ⌈⌊a/b⌋*b⌉ <= a
            let repay_amount = LendingPool::ceil(cdp_data.normalized_borrow * borrow_index);
            normalized_amount = cdp_data.normalized_borrow;

            let normalized_collateral = debt_in_xrd / collateral_price * (Decimal::ONE - liquidation_bonus) / collateral_supply_index;
            assert!(cdp_data.collateral_amount > normalized_collateral, "take collateral too many!");
            
            let dx_address = cdp_data.collateral_token;
            let collateral_vault = self.collateral_vaults.get_mut(&dx_address).unwrap();
            let collateral_bucket = collateral_vault.take(normalized_collateral);
            
            cdp_data.collateral_amount -=  normalized_collateral;
            cdp_data.normalized_borrow = Decimal::ZERO;

            debug!("repay_bucket:{}, normalized_amount:{}, normalized_borrow:{}, repay_amount:{}", repay_amount, normalized_amount, cdp_data.normalized_borrow, repay_amount);
            let borrow_vault = self.vaults.get_mut(&debt).unwrap();
            borrow_vault.put(debt_bucket.take(repay_amount));
            debt_state.normalized_total_borrow -= repay_amount;

            debt_state.update_interest_rate();

            self.minter.authorize(|| {
                let cdp_res_mgr: &ResourceManager = borrow_resource_manager!(self.cdp_res_addr);
                cdp_res_mgr.update_non_fungible_data(&NonFungibleId::from_u64(cdp_id)   , cdp_data);
            });

            collateral_bucket
        } 

        pub fn get_cdp_digest(&self, cdp_id: u64) -> (ResourceAddress, ResourceAddress, Decimal, Decimal, Decimal, Decimal){
            let cdp: CollateralDebtPosition = borrow_resource_manager!(self.cdp_res_addr).get_non_fungible_data(&NonFungibleId::from_u64(cdp_id));
            let borrow_token = cdp.borrow_token;
            let collateral_token = cdp.collateral_token;
            let deposit_asset_addr = self.origin_asset_map.get(&collateral_token).unwrap();
            let collateral_state = self.states.get(&deposit_asset_addr).unwrap();
            let debt_state = self.states.get(&borrow_token).unwrap();
            

            let deposit_asset_price = self.get_asset_price(deposit_asset_addr.clone());
            let debet_asset_price = self.get_asset_price(borrow_token.clone());
            let (collateral_supply_index, _)= collateral_state.get_current_index();
            let (_, debet_borrow_index) = debt_state.get_current_index();
            

            // return {
            //     "collateral_token": cdp.collateral_token,
            //     "borrow_token": cdp.borrow_token,
            //     "debt_in_xrd": LendingPool::ceil(cdp.normalized_borrow * debet_borrow_index * debet_asset_price),
            //     "collateral_in_xrd": LendingPool::floor(cdp.normalized_collateral * collateral_supply_index * deposit_asset_price)
            //     "debet_asset_price": debet_asset_price,
            //     "collateral_asset_price": deposit_asset_price
            // };
            (
                cdp.borrow_token,
                cdp.collateral_token, 
                LendingPool::ceil(cdp.normalized_borrow * debet_borrow_index * debet_asset_price),
                LendingPool::floor(cdp.collateral_amount * collateral_supply_index * deposit_asset_price),
                debet_asset_price,
                deposit_asset_price
            )

        }

        pub fn get_current_index(&self, asset_addr: ResourceAddress) -> (Decimal, Decimal){
            assert!(self.states.contains_key(&asset_addr), "unknown asset!");
            self.states.get(&asset_addr).unwrap().get_current_index()
        }

        pub fn get_interest_rate(&self, asset_addr: ResourceAddress) -> (Decimal, Decimal){
            assert!(self.states.contains_key(&asset_addr), "unknown asset!");
            self.states.get(&asset_addr).unwrap().get_interest_rates(Decimal::ZERO)
        }

        pub fn get_asset_price(&self, asset_addr: ResourceAddress) -> Decimal{
            let component: &Component = borrow_component!(self.oracle_addr);
            component.call::<Decimal>("get_price_quote_in_xrd", args![asset_addr])
        }

        fn get_max_loan_amount(&self, deposit_asset: ResourceAddress, deposit_amount: Decimal, ltv: Decimal, borrow_asset: ResourceAddress) -> Decimal{
            deposit_amount * self.get_asset_price(deposit_asset) * ltv / self.get_asset_price(borrow_asset)
        }

        fn ceil(dec: Decimal) -> Decimal{
            dec.round(18u8, RoundingMode::TowardsPositiveInfinity)
        }

        fn floor(dec: Decimal) -> Decimal{
            dec.round(18u8, RoundingMode::TowardsNegativeInfinity)
        }
    }
}
