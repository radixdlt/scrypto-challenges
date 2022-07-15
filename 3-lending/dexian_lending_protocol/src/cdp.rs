use scrypto::prelude::*;

// use assetstate::*;

#[derive(NonFungibleData)]
pub struct CollateralDebtPosition{
    pub borrow_token: ResourceAddress,
    pub collateral_token: ResourceAddress,
    
    #[scrypto(mutable)]
    pub total_borrow: Decimal,
    #[scrypto(mutable)]
    pub total_repay: Decimal,
    
    #[scrypto(mutable)]
    pub normalized_borrow: Decimal,
    #[scrypto(mutable)]
    pub collateral_amount: Decimal,
    #[scrypto(mutable)]
    pub borrow_amount: Decimal,
    #[scrypto(mutable)]
    pub last_update_epoch: u64
}

blueprint! {

    struct CollateralManager{
        debt_cdps: HashMap<ResourceAddress, HashSet<NonFungibleId>>,
        collateral_cdps: HashMap<ResourceAddress, HashSet<NonFungibleId>>,
    }

    impl CollateralManager{

        pub fn new() -> ComponentAddress{
            Self{
                debt_cdps: HashMap::new(),
                collateral_cdps: HashMap::new()
            }.instantiate()
            .globalize()
        }

        pub fn entry(&mut self, nft_id: NonFungibleId, debt_token: ResourceAddress, collateral_token: ResourceAddress ){
            // CollateralManager::insert_cdp(self.debt_cdps, debt_token, nft_id);
            // CollateralManager::insert_cdp(self.collateral_cdps, collateral_token, nft_id.clone());
            if self.debt_cdps.contains_key(&debt_token){
                let id_set = self.debt_cdps.get_mut(&debt_token).unwrap();
                id_set.insert(nft_id.clone());
            }
            else{
                let mut id_set = HashSet::new();
                id_set.insert(nft_id.clone());
                self.debt_cdps.insert(debt_token, id_set);
            }

            if self.collateral_cdps.contains_key(&collateral_token){
                let id_set = self.collateral_cdps.get_mut(&collateral_token).unwrap();
                id_set.insert(nft_id.clone());
            }
            else{
                let mut id_set = HashSet::new();
                id_set.insert(nft_id.clone());
                self.collateral_cdps.insert(collateral_token, id_set);
            }
        }

        pub fn exit(&mut self, nft_id: NonFungibleId, debt_token: ResourceAddress, collateral_token: ResourceAddress){
            assert!(self.debt_cdps.contains_key(&debt_token), "the debt token not exist!");
            assert!(self.collateral_cdps.contains_key(&collateral_token), "the collateral token not exist!");
            
            let id_set = self.debt_cdps.get_mut(&debt_token).unwrap();
            assert!(id_set.contains(&nft_id), "the debt nft_id not exists!");
            id_set.remove(&nft_id);
            
            let id_set2 = self.collateral_cdps.get_mut(&collateral_token).unwrap();
            assert!(id_set2.contains(&nft_id), "the collateral nft_id not exists!");
            id_set2.remove(&nft_id);
        }

        // pub fn evaluation(cdp_data: &CollateralDebtPosition, collatera_state: &AssetState, debt_state: &AssetState) -> (Decimal, Decimal){
        //     let collateral_token = cdp_data.collateral_token;
        //     let borrow_token = cdp_data.borrow_token;
        //     let normalized_borrow = cdp_data.normalized_borrow;
        //     let normalized_collateral = cdp_data.collateral_amount;
        //     let last_update_epoch = cdp_data.last_update_epoch;


        // }

        // fn insert_cdp(mut cdps:HashMap<ResourceAddress, HashSet<NonFungibleId>>, token_addr: ResourceAddress, nft_id:NonFungibleId){
        //     if cdps.contains_key(&token_addr){
        //         let id_set = cdps.get_mut(&token_addr).unwrap();
        //         id_set.insert(nft_id);
        //     }
        //     else{
        //         let mut id_set = HashSet::new();
        //         id_set.insert(nft_id);
        //         cdps.insert(token_addr, id_set);
        //     }
        // }
    }

}