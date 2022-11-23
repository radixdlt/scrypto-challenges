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

#[derive(NonFungibleData, Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct Mode {
    pub instance_nmbr: u128,
    pub mrkt_addr: ComponentAddress,
    pub listing_mode: u8
}

#[derive(NonFungibleData, Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct TicketID {
    pub instance_nmbr: u128,
    pub mrkt_addr: ComponentAddress,
    pub key: NonFungibleId,
    pub v: Vec<u128>
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct Tab {
    pub tuple:(
        (ResourceAddress,u128),
        (
            Vec<(ResourceAddress,NonFungibleId,AssetNFT)>,
            (u8,Decimal),
            (Decimal,Decimal,u64,Decimal,u64,u8,u128)
        )
    )
}
#[allow(dead_code)]
impl Tab {
    pub fn new() -> Tab {
        let nft_zero = AssetNFT {
            uri : "".to_string(),
            data_1 : "".to_string(),
            data_2 : "".to_string(),
            data_3 : "".to_string(),
            data_4 : "".to_string(),
            value_1: 0,
            value_2: 0,
            value_3: 0,
            linked_assets: Vec::new()
        };
        let mut nft_vec : Vec<(ResourceAddress,NonFungibleId,AssetNFT)> = Vec::new();
        nft_vec.push((ResourceAddress::from(RADIX_TOKEN),NonFungibleId::from_u64(0),nft_zero));
        let tup_one = (ResourceAddress::from(RADIX_TOKEN), 0);
        let tup_two = (
            nft_vec,
            (0,Decimal::zero()),
            (Decimal::zero(),Decimal::zero(),0,Decimal::zero(),0,0,0)
        );
        let tuple = (tup_one,tup_two);

        Tab { tuple }
    }
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct Tkn {
    pub fee: Decimal,
    pub royalty: Decimal,
    pub dex: ComponentAddress,
    pub currency: ResourceAddress,
    pub square: ComponentAddress,
    pub badge: ResourceAddress,
    pub oracle: ComponentAddress,
    pub vault: ComponentAddress,
    pub academy_vault: ComponentAddress,
    pub academy_share: Decimal,
    pub auction_dl: u64,
    pub last_bid_dl: u64,
    pub buy_prop_dl: u64
}
#[allow(dead_code)]
impl Tkn {
    pub fn new(
        comp_fee: Decimal,
        comp_royalty: Decimal,
        comp_dex_address: ComponentAddress,
        abc_currency: ResourceAddress
    ) -> Self {
        let fee = comp_fee;                      
        let royalty = comp_royalty;            
        let dex = comp_dex_address;                   
        let currency = abc_currency;                             
        let square = comp_dex_address;   
        let badge = abc_currency;    
        let oracle = comp_dex_address;         
        let vault = comp_dex_address;            
        let academy_vault = comp_dex_address;     
        let academy_share = Decimal::zero();               
        let auction_dl = 5000;                             
        let last_bid_dl = 5;
        let buy_prop_dl = 5000;                        

        Self {
            fee,
            royalty,
            dex,
            currency,
            square,
            badge,
            oracle,
            vault,
            academy_vault,
            academy_share,
            auction_dl,
            last_bid_dl,
            buy_prop_dl
        }
    }
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct CheckTuple {
    pub t: (u8,Decimal,Decimal,u64,Decimal,u64,u8,Decimal,u128,Decimal)
}
impl CheckTuple {
    pub fn new() -> CheckTuple {
        let t = (0,dec!("0"),dec!("0"),0,dec!("0"),0,0,dec!("0"),0,dec!("0"));

        CheckTuple { t } 
    }
}  

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct ExtMrkt {
    pub tuple: (ComponentAddress,ResourceAddress,Decimal,ResourceAddress,bool,ResourceAddress)
}
impl ExtMrkt {
    pub fn new( comp_addr: ComponentAddress ) -> ExtMrkt {
        let zero = ResourceAddress::from(RADIX_TOKEN);
        let tuple = (comp_addr,zero,dec!("0"),zero,false,zero);

        ExtMrkt { tuple }
    }
}   

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct CheckMeta {
    pub m: (HashMap<ResourceAddress,Vec<NonFungibleId>>,Vec<(ResourceAddress,NonFungibleId)>,Decimal,u128,u128,u8)
}
    
#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct NftMatch {
    pub n: (bool,Vec<(ResourceAddress,NonFungibleId)>,Decimal)
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct BuyPropTuple {
    pub tuple: (ResourceAddress,Decimal,u64,u8,ResourceAddress)
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct AuctionTuple {
    pub tuple: (ResourceAddress,Decimal,u64,u8,Decimal,ResourceAddress)
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct RaffleTuple {
    pub tuple: (u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct BuyPropBdgMap {
    pub map: HashMap<u128,Vec<(ResourceAddress,Decimal,u64,u8,ResourceAddress)>>
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct AuctionBdgMap {
    pub map: HashMap<u128,Vec<(ResourceAddress,Decimal,u64,u8,Decimal,ResourceAddress)>>
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct RaffleBdgMap {
    pub map: HashMap<u128,Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)>>
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct NftVec {
    pub nft_vec_map: Vec<(ResourceAddress,Vec<(u128,NonFungibleId,Decimal,bool)>)>
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct ExtMrktVec {
    pub map: Vec<(ComponentAddress,ResourceAddress,Decimal,ResourceAddress,ResourceAddress)>
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct ExtMetaMap {
    pub map: HashMap<ResourceAddress,(Decimal,Vec<(NonFungibleId,NonFungibleId,Decimal,u128)>)>
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct RstData {
    pub t: (ResourceAddress,u128,u64,u64,u8,Decimal,u64,Decimal,Decimal)
}
