use scrypto::prelude::*;
use crate::data_square::*;
use crate::info::*;

blueprint! {
    struct DemoTools {  
        // Vault to stock SBT Updater Badge
        sbt_updater_badge: Vault,
        // SBT Updater Badge resource address
        sbt_updater_badge_addr: ResourceAddress,
        // User SBT Resource Address.
        user_sbt: ResourceAddress, 
		// A vault that holds the mint badge
        demo_tools_nft_minter_badge: Vault,
        // Resource definition of Asset NFT series one
        demo_tools_nft_one: ResourceAddress,   
        // Resource definition of Asset NFT series two
        demo_tools_nft_two: ResourceAddress          
    }

    #[allow(dead_code)]
    impl DemoTools {
        pub fn new(
            sbt_updater_badge_addr: ResourceAddress,    // Land Data protocol's SBT updater Badge resource address
            user_sbt: ResourceAddress                   // Land Data protocol's registered users SBT resource address
        ) -> ComponentAddress {
        	// Create a Protocol Minter Badge resource
            let demo_tools_nft_minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("Name", "Asset NFT Minter Badge")
                .initial_supply(1);
            // Create an NFT resource with mutable supply    
            let demo_tools_nft_one = ResourceBuilder::new_non_fungible()
                .metadata("Ecosystem", "Neverland")
                .metadata("Series", "Alpha")
                .metadata("Number", "1".to_string())
                .mintable(rule!(require(demo_tools_nft_minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(demo_tools_nft_minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(demo_tools_nft_minter_badge.resource_address())), LOCKED)
                .no_initial_supply();
            // Create an NFT resource with mutable supply     
            let demo_tools_nft_two = ResourceBuilder::new_non_fungible()
                .metadata("Ecosystem", "Mahoroba")
                .metadata("Series", "Beta")
                .metadata("Number", "1".to_string())
                .mintable(rule!(require(demo_tools_nft_minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(demo_tools_nft_minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(demo_tools_nft_minter_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let access_rules = AccessRules::new()
                .default(rule!(allow_all));

            let mut demo_tools: DemoToolsComponent = Self {
                sbt_updater_badge: Vault::new(sbt_updater_badge_addr),
                sbt_updater_badge_addr,
                user_sbt,
                demo_tools_nft_minter_badge: Vault::with_bucket(demo_tools_nft_minter_badge),
                demo_tools_nft_one,
                demo_tools_nft_two
            }
            .instantiate();
            demo_tools.add_access_check(access_rules);

            demo_tools.globalize()
        }

        // Stock "LandData UpdaterBadge" to update users SBT data when requested by protocol  
        pub fn stock_sbt_updater_badge(&mut self, sbt_updater_badge: Bucket) {
            assert!(
                sbt_updater_badge.resource_address() == self.sbt_updater_badge_addr,
                "[stock_sbt_updater_badge]:Wrong Badge provided! "
            );
            assert!(sbt_updater_badge.amount() == dec!("1"),"[stock_sbt_updater_badge]:Just one! ");
            assert!(
                self.sbt_updater_badge.is_empty(),
                "[stock_sbt_updater_badge]:Updater Badge already present! "
            );
            self.sbt_updater_badge.put(sbt_updater_badge);
        }

        // Mint one Asset NFT one
        pub fn nft_mint_one(&mut self, parcel: String) -> Bucket {      // Test purpose only  
        	let nft_key = NonFungibleId::random();          
            let out_url = "https://gistcdn.githack.com/alanci17/4010e8a0db866b7abb36ef89cca9b701/raw/5b305f012ca90cb3324a6ed7ccf484600ceb31f8/9991.svg".to_string();
            let tab = "\" \n \"".to_string();
            let svg_1 = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"400\"><circle cx=\"200\" cy=\"400\" r=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"pink\" />".to_string();
            let svg_2 = "<circle cx=\"0\" cy=\"0\" r=\"150\" stroke=\"black\" stroke-width=\"5\" fill=\"pink\" />".to_string();
            let svg_3 = "<circle cx=\"350\" cy=\"350\" r=\"175\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />".to_string();
            let svg_4 = "<circle cx=\"350\" cy=\"100\" r=\"175\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />".to_string();
            let svg_5 = "<circle cx=\"300\" cy=\"300\" r=\"150\" stroke=\"black\" stroke-width=\"5\" fill=\"green\" />".to_string();
            let svg_6 = "<circle cx=\"250\" cy=\"250\" r=\"125\" stroke=\"black\" stroke-width=\"5\" fill=\"red\" />".to_string();
            let svg_7 = "<circle cx=\"200\" cy=\"200\" r=\"100\" stroke=\"black\" stroke-width=\"5\" fill=\"palevioletred\" />".to_string();
            let svg_8 = "<circle cx=\"150\" cy=\"150\" r=\"75\" stroke=\"black\" stroke-width=\"5\" fill=\"turquoise\" /></svg>".to_string();
            let svg = tab + &svg_1 + &svg_2 + &svg_3 + &svg_4 + &svg_5 + &svg_6 + &svg_7 + &svg_8;
            let str_1 = " Neverland Property Certificate ".to_string();
            let str_2 = " Alpha City ".to_string();
            let str_3 = " Beta District ".to_string();
            let str_4 = "parcel".to_string() + &parcel;

            let new_nft = AssetNFT {
                uri: out_url.to_owned() + &svg,
                data_1: str_1,
                data_2: str_2,
                data_3: str_3,
                data_4: str_4,
                value_1: 0,
                value_2: 125,
                value_3: 0,
                linked_assets: Vec::new()
            };

            nft_mint(nft_key.clone(),self.demo_tools_nft_one);
        
            self.demo_tools_nft_minter_badge.authorize(|| { 
                borrow_resource_manager!(self.demo_tools_nft_one).mint_non_fungible(&nft_key,new_nft)
            }) 
        }

        // Mint one Asset NFT 
        pub fn nft_mint_two(&mut self, parcel: String) -> Bucket {      // Test purpose only
            let nft_key = NonFungibleId::random();          
            let out_url = "https://gistcdn.githack.com/alanci17/4010e8a0db866b7abb36ef89cca9b701/raw/5b305f012ca90cb3324a6ed7ccf484600ceb31f8/9991.svg".to_string();
            let tab = "\" \n \"".to_string();
            let svg_1 = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"400\"><circle cx=\"200\" cy=\"400\" r=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"pink\" />".to_string();
            let svg_2 = "<circle cx=\"0\" cy=\"0\" r=\"150\" stroke=\"black\" stroke-width=\"5\" fill=\"pink\" />".to_string();
            let svg_3 = "<circle cx=\"350\" cy=\"350\" r=\"175\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />".to_string();
            let svg_4 = "<circle cx=\"350\" cy=\"100\" r=\"175\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />".to_string();
            let svg_5 = "<circle cx=\"300\" cy=\"300\" r=\"150\" stroke=\"black\" stroke-width=\"5\" fill=\"green\" />".to_string();
            let svg_6 = "<circle cx=\"250\" cy=\"250\" r=\"125\" stroke=\"black\" stroke-width=\"5\" fill=\"red\" />".to_string();
            let svg_7 = "<circle cx=\"200\" cy=\"200\" r=\"100\" stroke=\"black\" stroke-width=\"5\" fill=\"palevioletred\" />".to_string();
            let svg_8 = "<circle cx=\"150\" cy=\"150\" r=\"75\" stroke=\"black\" stroke-width=\"5\" fill=\"turquoise\" /></svg>".to_string();
            let svg = tab + &svg_1 + &svg_2 + &svg_3 + &svg_4 + &svg_5 + &svg_6 + &svg_7 + &svg_8;
            let str_1 = " Mahoroba Property Certificate ".to_string();
            let str_2 = " Beta City ".to_string();
            let str_3 = " Gamma District ".to_string();
            let str_4 = "parcel".to_string() + &parcel;

            let new_nft = AssetNFT {
                uri: out_url.to_owned() + &svg,
                data_1: str_1,
                data_2: str_2,
                data_3: str_3,
                data_4: str_4,
                value_1: 0,
                value_2: 125,
                value_3: 0,
                linked_assets: Vec::new()
            };

            nft_mint(nft_key.clone(),self.demo_tools_nft_two);
        
            self.demo_tools_nft_minter_badge.authorize(|| { 
                borrow_resource_manager!(self.demo_tools_nft_two).mint_non_fungible(&nft_key,new_nft)
            }) 
        }

        pub fn insert_property(                                                 // Test purpose only
            &mut self,
            land_asset_nft: Bucket, 
            land_owner_sbt: Proof
        ) -> Bucket {
            let land_owner_sbt: ValidatedProof = land_owner_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[insert_property]: Invalid proof provided!");        

            let land_asset_addr = land_asset_nft.resource_address();
            let land_asset_id = land_asset_nft.non_fungible::<AssetNFT>().id();
            let land_asset_data: AssetNFT = land_asset_nft.non_fungible().data();

            let land_owner_sbt_id = land_owner_sbt.non_fungible::<UserSBT>().id();
            let mut land_owner_data_sbt: UserSBT = land_owner_sbt.non_fungible().data();

            land_owner_data_sbt.real_estate_properties
                .push((land_asset_addr,land_asset_id,land_asset_data));

            self.sbt_updater_badge.authorize(|| {
                borrow_resource_manager!(land_owner_sbt.resource_address())
                    .update_non_fungible_data(&land_owner_sbt_id, land_owner_data_sbt)
            }); 

            land_asset_nft
        }
    }
}    


