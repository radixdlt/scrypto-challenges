use scrypto::prelude::*;

#[derive(NonFungibleData, Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct UserSBT {
    pub cmp_data_address: ComponentAddress,
    pub data: String,
    #[scrypto(mutable)]
    pub assets: Vec<(ResourceAddress,NonFungibleId,AssetNFT)>,
    #[scrypto(mutable)]
    pub credits: Vec<(ResourceAddress,NonFungibleId,AssetNFT)>,
    #[scrypto(mutable)]
    pub loans: Vec<(ResourceAddress,NonFungibleId,AssetNFT)>,
    #[scrypto(mutable)]
    pub real_estate_properties: Vec<(ResourceAddress,NonFungibleId,AssetNFT)>,
    #[scrypto(mutable)]
    pub rental_properties: Vec<(ResourceAddress,NonFungibleId,AssetNFT)>,
    #[scrypto(mutable)]
    pub educational_degrees: Vec<(ResourceAddress,NonFungibleId,DegreeNFT)>,
    #[scrypto(mutable)]
    pub values: Vec<(String,Decimal)>
} 

// Asset NFT Stucture
#[derive(NonFungibleData, Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct AssetNFT {
    pub uri: String,
    pub data_1: String,
    pub data_2: String,
    pub data_3: String,
    pub data_4: String,
    #[scrypto(mutable)]
    pub value_1: u8,
    #[scrypto(mutable)]
    pub value_2: u8,
    #[scrypto(mutable)]
    pub value_3: u8,
    #[scrypto(mutable)]
    pub linked_assets: Vec<(ResourceAddress,NonFungibleId)>
} 

#[derive(NonFungibleData, Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct DegreeNFT {
    pub uri: String,
    pub pro_academy_address: ComponentAddress,
    pub user_sbt_address: ResourceAddress,
    pub user_sbt_id: NonFungibleId,
    pub user_name: String,
    pub degree_name: Vec<String>,
    pub mint_date: u64,
    pub teaching_subject: Vec<String>,
    pub grade_point_avg: u8,
    pub cum_laude: bool
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct Tkn {
    pub tkn_currency: ResourceAddress,         
    pub series_component_addr: ComponentAddress,
    pub mint_component_addr: ComponentAddress,
    pub merge_component_addr: ComponentAddress,
    pub upgrade_component_addr: ComponentAddress,
    pub asset_xrd_main_vault: ComponentAddress,
    pub asset_tkn_main_vault: ComponentAddress,
    pub asset_dex_address: ComponentAddress,
    pub academy_comp: ComponentAddress 
}
impl Tkn {
    pub fn new(comp_addr: ComponentAddress, currency: ResourceAddress) -> Self {
        Self {
            tkn_currency: currency,
            series_component_addr: comp_addr,   
            mint_component_addr: comp_addr,
            merge_component_addr: comp_addr,
            upgrade_component_addr: comp_addr,
            asset_xrd_main_vault: comp_addr, 
            asset_tkn_main_vault: comp_addr, 
            asset_dex_address: comp_addr,
            academy_comp: comp_addr
        }
    }
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct Bidder {
    pub instance_nr: u128,
    pub bid: Decimal,
    pub bid_bond_reclaimed: bool,
    pub asset_collected: bool
}


