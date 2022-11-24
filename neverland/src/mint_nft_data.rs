use scrypto::prelude::*;

// Asset NFT Mint Data Component 
blueprint! {
    struct NeverlandMintNftData {
        minter_badge: Vault,
        owner_badge: ResourceAddress,
        nft_badge_map: HashMap<ComponentAddress,ResourceAddress>,
        component_addr: ComponentAddress,
        svg_data_map: HashMap<String,Vec<(u128,String)>>
    }

    impl NeverlandMintNftData {
        pub fn new() -> (ComponentAddress,Bucket){
            let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " MinterBadge ")
                .initial_supply(Decimal::one());

            let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " OwnerBadge ")
                .initial_supply(Decimal::one()); 

            let access_rules = AccessRules::new()
                .method("nft_badge_mint", rule!(require(owner_badge.resource_address())))
                .method("insert_svg_data", rule!(require(owner_badge.resource_address())))
                .method("check_svg_data", rule!(require(owner_badge.resource_address())))
                .default(rule!(allow_all));     

            let mut neverland_nft_data: NeverlandMintNftDataComponent = Self {
                minter_badge: Vault::with_bucket(minter_badge),
                owner_badge: owner_badge.resource_address(),
                nft_badge_map: HashMap::new(),
                component_addr:  ComponentAddress::Normal([0; 26]),
                svg_data_map: HashMap::new()
            }
            .instantiate();
            neverland_nft_data.add_access_check(access_rules);

            (neverland_nft_data.globalize(),owner_badge)
        }
        
            // Mint a caller badge to authorize an external component to invoke methods.
            // Owner_badge required  
        pub fn nft_badge_mint(&mut self, component_addr: ComponentAddress) -> Bucket {
            let nft_badge_res_def = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "nft_badge_evo")
                .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .no_initial_supply();
                              
            self.nft_badge_map.insert(component_addr,nft_badge_res_def);
            self.component_addr = component_addr;
            info!(" Component address: {} ", self.component_addr);
                
            self.minter_badge
                .authorize(|| { borrow_resource_manager!(nft_badge_res_def).mint(1) })
        }

            // Insert NFT trait name and related production code & svg data
            // Owner_badge required
        pub fn insert_svg_data(&mut self, trait_data: String, mut svg_data_vec: Vec<(u128,String)>){
            match self.svg_data_map.get_mut(&trait_data) {    
                Some(v) => v.append(&mut svg_data_vec),
                _ => { 
                    self.svg_data_map.insert(trait_data,svg_data_vec);         
                }
            }
        }

            // Check inserted svg data 
            // Owner_badge required
        pub fn check_svg_data(&mut self) -> HashMap<String,Vec<(u128,String)>> {
            for (key,val) in self.svg_data_map.iter() {
                info!(" key {} ",key);
                for values in val {
                    info!(" u128 {} ",values.0);
                    info!(" string {} ",values.1);
                }
            }

            self.svg_data_map.clone()
        }

            // Extract svg data
            fn extract_svg(&mut self, str_name: String, seed: u128) -> String {
                let mut svg_str = "".to_string();
                match self.svg_data_map.get(&str_name) {
                    Some(v) => {
                        for (pointer,svg_data) in v {
                                if *pointer == seed {
                                    svg_str = svg_data.to_string();
                                    break;
                                }
                            }
                        }                        
                    None => ()
                }
                svg_str
            }

            // Extract code
            fn extract_code( seed: u128, exp: u32) -> (u128,u128) {
                let float_x = seed/10_u128.pow(exp);
                let data_x = u128::try_from(float_x).unwrap();
                let mask_x = data_x*10_u128.pow(exp);

                (data_x,seed-mask_x)
            } 

            // Demux input data Fn  
            fn process_data(&mut self, seed: u128) -> (u128,u128,u128,u128){
                let (data_e,num_f) = NeverlandMintNftData::extract_code(seed,6);
                let (data_f,num_g) = NeverlandMintNftData::extract_code(num_f,4);
                let (data_g,_num_h) = NeverlandMintNftData::extract_code(num_g,2);
                let data_h = num_g-data_g*10_u128.pow(2);

                (data_e, data_f, data_g, data_h)
            }  
 
            // Collect data for NFT mint purpose. Asset NFT data field number one
        pub fn asset_data_one(&mut self, seed: String, auth: Proof) -> (String,String) {    
            let auth: ValidatedProof = auth.unsafe_skip_proof_validation();               
            match self.nft_badge_map.get(&self.component_addr){
                Some(addr) => assert!(*addr == auth.resource_address()),
                None => {
                    info!("[asset_data_one]:NFT Badge authorization failed !");       
                    std::process::abort()     
                } 
            } 
            
            // Demux input data
            let (e,_f,_g,_h) = self.process_data(seed.parse::<u128>().unwrap());

            // Match return value
            let se = match e {
                0 => "Neverland Property Certificate".to_string(),   
                1 => "Neverland Lend Certificate".to_string(), 
                2 => "Neverland Borrow Certificate".to_string(),
                3 => "Neverland Rental Certificate".to_string(),
                _ => "".to_string()
            };

            let str_e = "type".to_string();

            (     
                " asset type ".to_string() + &se,
                self.extract_svg(str_e,e)
            )
        }

            // Collect data for NFT mint purpose. Asset NFT data field number two 
        pub fn asset_data_two(&mut self, seed: String, auth: Proof) -> (String,String) {     
            let auth: ValidatedProof = auth.unsafe_skip_proof_validation();               
            match self.nft_badge_map.get(&self.component_addr){
                Some(addr) => assert!(*addr == auth.resource_address()),
                None => {
                    info!("[asset_data_two]:NFT Badge authorization failed !");       
                    std::process::abort()     
                } 
            } 
            
            // Demux input data
            let (_e,f,_g,_h) = self.process_data(seed.parse::<u128>().unwrap());

            // Match return value
            let sf = match f {
                0 => "North".to_string(),   1 => "Center".to_string(),    2 => "South".to_string(),                         
                _ => "".to_string()
            };

            let str_f = "region".to_string();
        
            (
                " Region ".to_string() + &sf,
                self.extract_svg(str_f,f)
            )
        }

            // Collect data for NFT mint purpose. Asset NFT data field number three
        pub fn asset_data_three(&mut self, seed: String, auth: Proof) -> (String,String) { 
            let auth: ValidatedProof = auth.unsafe_skip_proof_validation();              
            match self.nft_badge_map.get(&self.component_addr){
                Some(addr) => assert!(*addr == auth.resource_address()),
                None => {
                    info!("[asset_data_three]:NFT Badge authorization failed !");       
                    std::process::abort()     
                } 
            } 
            
            // Demux input data
            let (_e,_f,g,_h) = self.process_data(seed.parse::<u128>().unwrap());
             
            // Match return value
            let sg = match g {
                0 => "NorthWest".to_string(),   1 => "North".to_string(),    2 => "NorthEast".to_string(),                   
                3 => "West".to_string(), 4 => "Center".to_string(),  5 => "East".to_string(),
                6 => "SouthWest".to_string(),    7 => "South".to_string(), 8 => "SouthEast".to_string(),            
                _ => "".to_string()
            };

            let str_g = "district".to_string();        
        
            (
                " District ".to_string() + &sg, 
                self.extract_svg(str_g,g)
            )
        }

            // Collect data for NFT mint purpose. Asset NFT data field number for
        pub fn asset_data_for(&mut self, seed: String, auth: Proof) -> (String,String) {
            let auth: ValidatedProof = auth.unsafe_skip_proof_validation();                    
            match self.nft_badge_map.get(&self.component_addr){
                Some(addr) => assert!(*addr == auth.resource_address()),
                None => {
                    info!("[asset_data_for]:NFT Badge authorization failed !");       
                    std::process::abort()     
                } 
            } 
            
            // Demux input data
            let (_e,_f,_g,h) = self.process_data(seed.parse::<u128>().unwrap());

            // Match return value
            let sh = match h {
                0 => "0101".to_string(),    1 => "0102".to_string(),  2 => "0103".to_string(),      
                3 => "0201".to_string(),    4 => "0202".to_string(), 5 => "0203".to_string(),    
                6 => "0301".to_string(),    7 => "0302".to_string(),  8 => "0303".to_string(),      
                _ => "".to_string()
            };
            let str_h = "parcel".to_string();        
            
            (
                " Parcel ".to_string() + &sh,
                self.extract_svg(str_h,h)  
            )
        }     
    }
}