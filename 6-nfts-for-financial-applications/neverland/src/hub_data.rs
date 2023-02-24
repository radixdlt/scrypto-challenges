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
pub struct TestCertificate {
    pub uri: String,
    pub pro_academy_address: ComponentAddress,
    pub sbt_address: ResourceAddress,
    pub sbt_id: NonFungibleId,
    pub course_name: String,
    pub test_name: String,
    pub course_number: u32,
    pub test_number: u8,
    pub test_date: u64,
    pub test_passed: bool,
    pub score: u8 
}

#[derive(NonFungibleData, Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct Test {
    pub uri: String,
    pub pro_academy_address: ComponentAddress,
    pub user_sbt_address: ResourceAddress,
    pub user_sbt_id: NonFungibleId,
    pub course_name: String,
    pub course_number: u32,
    pub test_name: String,
    pub test_number: u8,
    pub test_date: u64,
    pub assertions: Vec<String>,
    pub answers: Vec<bool>,
    #[scrypto(mutable)]
    pub right_answers: Vec<bool>,
    #[scrypto(mutable)]
    pub test_passed: bool,
    #[scrypto(mutable)]
    pub score: u8
}

#[derive(NonFungibleData, Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct BuildingContract {
    pub url: String,
    pub house_hub_address: ComponentAddress,
    pub land_owner_sbt_address: ResourceAddress,
    pub land_owner_sbt_id: NonFungibleId,
    pub contractor_sbt_address: ResourceAddress,
    pub contractor_sbt_id: NonFungibleId,
    pub land_property_nft: ResourceAddress,
    pub land_property_nft_id: NonFungibleId,
    pub house_project_nft: ResourceAddress,
    pub house_project_nft_id: NonFungibleId,
    pub property_building_nft: ResourceAddress,         
    pub property_building_nft_id: NonFungibleId,        
    pub building_surface: u8,
    pub contract_amount: Decimal,
    pub deadline: u64,
    #[scrypto(mutable)]
    pub executed: bool,
    #[scrypto(mutable)]
    pub approved: bool
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

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct AssetZero {
    pub asset_zero: Vec<AssetNFT>
}
impl AssetZero {
    pub fn new() -> AssetZero {
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
        let mut asset_zero : Vec<AssetNFT> = Vec::new();
        asset_zero.push(nft_zero);
        
        AssetZero { asset_zero }
    }
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct DegreeZero {
    pub degree_zero: Vec<DegreeNFT>
}
impl DegreeZero {
    pub fn new() -> DegreeZero {
        let vec_string : Vec<String> = Vec::new();
        let degree_nft_zero = DegreeNFT {
            uri : "".to_string(),
            pro_academy_address: Runtime::actor().as_component().0,
            user_sbt_address: ResourceAddress::from(RADIX_TOKEN),
            user_sbt_id: NonFungibleId::from_u64(0),
            user_name: "".to_string(),
            degree_name: vec_string.clone(),
            mint_date: 0,
            teaching_subject: vec_string,
            grade_point_avg: 0,
            cum_laude: false,
        };
        let mut degree_zero : Vec<DegreeNFT> = Vec::new();
        degree_zero.push(degree_nft_zero);
        
        DegreeZero { degree_zero }
    }
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct TestZero {
    pub test_zero: Vec<Test>
}
#[allow(dead_code)]
impl TestZero {
    pub fn new() -> TestZero {
        let assertion_vec : Vec<String> = Vec::new();
        let answer_vec : Vec<bool> = Vec::new();
        let test_nft_zero = Test {
            uri : "".to_string(),
            pro_academy_address: Runtime::actor().as_component().0,
            user_sbt_address: ResourceAddress::from(RADIX_TOKEN),
            user_sbt_id: NonFungibleId::from_u64(0),
            course_name: "".to_string(),
            course_number: 0,
            test_name: "".to_string(),
            test_number: 0,
            test_date: 0,
            assertions: assertion_vec,
            answers: answer_vec.clone(),
            right_answers: answer_vec,
            test_passed: false,
            score: 0
            
        };
        let mut test_zero : Vec<Test> = Vec::new();
        test_zero.push(test_nft_zero);
        
        TestZero { test_zero }
    }
}

#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode, sbor::Describe, PartialEq, Eq)]
pub struct Tup {
    pub tuple: (ResourceAddress,NonFungibleId,DegreeNFT)
}
#[allow(dead_code)]
impl Tup {
    pub fn new() -> Tup {
        let str_vec: Vec<String> = Vec::new();
        let degree_zero = DegreeNFT {
            uri : "".to_string(),
            pro_academy_address: Runtime::actor().as_component().0,
            user_sbt_address: ResourceAddress::from(RADIX_TOKEN),
            user_sbt_id: NonFungibleId::from_u64(0),
            user_name: "".to_string(),
            degree_name: str_vec.clone(),            
            mint_date: 0,
            teaching_subject: str_vec,
            grade_point_avg: 0, 
            cum_laude: false   
        };
       
        let tuple = (ResourceAddress::from(RADIX_TOKEN),NonFungibleId::from_u64(0),degree_zero);
        
        Tup { tuple }
    }
}
