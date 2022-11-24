use scrypto::prelude::*;
use crate::hub_data::*;

blueprint! {
    struct LandData{
        // Protocol Minter Badge resource address
    	minter_badge: Vault,
        // Vault to stock SBT Updater Badge
        sbt_updater_badge: Vault,
        // Protocol Owner Badge resource address
    	owner_badge: ResourceAddress,
        // User SBT Resource Address.
    	user_sbt: ResourceAddress,
        // Map to store an external Component address and relative Caller Badge resource address allowing it 
        // to invoke protocol methods through component call.
        // Component within map are authorized in read mode only.
    	caller_map_read: HashMap<ComponentAddress,ResourceAddress>,
        // Map to store an external Component address and relative Caller Badge resource address allowing it 
        // to invoke protocol methods through component call.
        // Component within map are authorized in write mode.
        caller_map_write: HashMap<ComponentAddress,ResourceAddress>,
        // Map relating registered user SBT identificative references with related assets identificative 
        // references and data
    	user_map: HashMap<(ResourceAddress,NonFungibleId),Vec<(ResourceAddress,NonFungibleId,AssetNFT)>>,
        // Map relating external user SBT identificative references with related assets identificative 
        // references and data
        ext_user_map: HashMap<(ResourceAddress,NonFungibleId),Vec<(ResourceAddress,NonFungibleId,AssetNFT)>>,
        // Map relating external user SBT identificative references with related assets identificative 
        // references and data for transfer of ownership purpose 
        ext_re_map: HashMap<(ResourceAddress,NonFungibleId),Vec<(ResourceAddress,NonFungibleId,AssetNFT)>>

    }

    impl LandData {
        pub fn new(
            land_name: String, 
            updater_badge_number: Decimal
    ) -> (ComponentAddress,Bucket) {
        	let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " LandData MinterBadge ")
                .initial_supply(Decimal::one());

            let updater_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " LandData UpdaterBadge ")
                .initial_supply(updater_badge_number);

            let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " LandData OwnerBadge ")
                .initial_supply(Decimal::one());

        	let user_sbt = ResourceBuilder::new_non_fungible()
                .metadata("name", land_name + " LandData UserSBT")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(updater_badge.resource_address())), LOCKED)                
                .restrict_withdraw(AccessRule::DenyAll, LOCKED)
                .no_initial_supply();

            let access_rules = AccessRules::new()
            	.method("mint_caller_badge", rule!(require(owner_badge.resource_address())))   
                .method("transfer_updater_badge", rule!(require(owner_badge.resource_address())))
                .default(rule!(allow_all));


            let mut land_data: LandDataComponent = Self {                
                minter_badge: Vault::with_bucket(minter_badge),
                sbt_updater_badge: Vault::with_bucket(updater_badge),
                owner_badge: owner_badge.resource_address(),
                user_sbt: user_sbt,
                caller_map_read: HashMap::new(),
                caller_map_write: HashMap::new(),
                user_map: HashMap::new(),
                ext_user_map: HashMap::new(),
                ext_re_map: HashMap::new()
            }
            .instantiate();
            land_data.add_access_check(access_rules);
            
            (land_data.globalize(),owner_badge)
        }

            // Mint one SBT updater Badge to an external specified Component to allow data update.
        pub fn transfer_updater_badge(&mut self, cmp_addr: ComponentAddress) {
            let bckt = self.sbt_updater_badge.take(Decimal::one());
            let method = "stock_sbt_updater_badge".to_string(); 
            let args = args![bckt];

            borrow_component!(cmp_addr).call::<()>(&method, args)
        }

        // Mint a Caller Badge to allow called from an external Component to call methods
        pub fn mint_caller_badge(&mut self, cmp_addr: ComponentAddress, flag: u8) -> Bucket {
            let caller_badge = self.build_badge(" LandData_Caller_Component_Badge".to_string());
            match flag {                        
                0 => self.caller_map_read.insert(cmp_addr,caller_badge),
                _ => self.caller_map_write.insert(cmp_addr,caller_badge)
            };
            info!(" Caller Component address added: {} ", cmp_addr);
                
            self.minter_badge
                .authorize(|| { borrow_resource_manager!(caller_badge).mint(Decimal::one()) })
        }

        // Retrieve asset data providing related owner SBT identification data
        pub fn query_by_owner_id(
            &mut self, 
            re_owner_buyer_res_addr: ResourceAddress, 
            re_owner_buyer_id: NonFungibleId,
        ) {
            let mut data_vec: Vec<(ResourceAddress,NonFungibleId,AssetNFT)> = Vec::new();
            match self.user_map.get_mut(&(re_owner_buyer_res_addr,re_owner_buyer_id.clone())) {
                Some(v) => {
                    for tuple in v {
                        data_vec.push(tuple.clone());
                    }
                }
                _ => {
                    info!(" DataLand User unfounded! ");
                    std::process::abort()
                }
            }
            self.info_map(data_vec);
        }

        // Retrieve asset data providing related owner SBT identification data 
        pub fn ext_query_by_owner_id(
            &mut self, 
            re_owner_buyer_res_addr: ResourceAddress, 
            re_owner_buyer_id: NonFungibleId,
        ) {
            let mut data_vec: Vec<(ResourceAddress,NonFungibleId,AssetNFT)> = Vec::new();
            match self.ext_user_map.get(&(re_owner_buyer_res_addr,re_owner_buyer_id.clone())) {
                Some(v) => {
                    for tuple in v {
                        data_vec.push(tuple.clone());
                    }
                }
                _ => {
                    info!(" DataLand User unfounded! ");
                    std::process::abort()
                }
            }
            self.info_map(data_vec);
        }

        // Method callable to register asset's ownership within protocol's related map, 
        // providing related data as SBT resource address and ID of owner and asset as well as
        // his data. 
        // Method callable by authorized external protocols once provided Caller Badge proof
        pub fn register_property(
            &mut self, 
            re_owner_buyer_res_addr: ResourceAddress, 
            re_owner_buyer_id: NonFungibleId,
            re_res_addr: ResourceAddress, 
            re_id: NonFungibleId,
            re_data_str: Vec<String>,
            re_data_val: Vec<u8>,
            caller_cmp_addr: ComponentAddress,
            auth_ref: Proof
        ) {
            let re_data = self.data_assembler(re_data_str, re_data_val);

            // Verify if Upgrade Caller Component is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.caller_map_write.get(&caller_cmp_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!(" Upgrade Badge authorization failed! ");       
                    std::process::abort()     
                } 
            } 

            match self.user_map.get_mut(&(re_owner_buyer_res_addr,re_owner_buyer_id.clone())) {
                Some(v) => v.push((re_res_addr,re_id,re_data)),
                _ => {
                    info!(" DataLand User unfounded! ");
                    std::process::abort()
                }
            }
        }

        // Method callable to update asset's ownership within protocol's related map, 
        // providing related data as SBT resource address and ID of seller, buyer and asset. 
        // Method callable by authorized external protocols once provided Caller Badge proof
        pub fn transfer_property(
            &mut self, 
            re_owner_seller_res_addr: ResourceAddress, 
            re_owner_seller_id: NonFungibleId,
            re_owner_buyer_res_addr: ResourceAddress, 
            re_owner_buyer_id: NonFungibleId,
            re_res_addr: ResourceAddress, 
            re_id: NonFungibleId,
            re_data_str: Vec<String>,
            re_data_val: Vec<u8>,
            caller_cmp_addr: ComponentAddress,
            auth_ref: Proof
        ) {
            let re_data = self.data_assembler(re_data_str, re_data_val);

            // Verify if Caller Badge is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.caller_map_write.get(&caller_cmp_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!(" Upgrade Badge authorization failed !");       
                    std::process::abort()     
                } 
            } 

            match self.user_map.get_mut(&(re_owner_seller_res_addr,re_owner_seller_id)) {
                Some(v) => v.retain(|x| x.0 != re_res_addr && x.1 != re_id), 
                _ => std::process::abort()
            }

            if re_owner_buyer_res_addr == self.user_sbt {
                match self.user_map.get_mut(&(re_owner_buyer_res_addr,re_owner_buyer_id)) {
                    Some(v) => v.push((re_res_addr,re_id,re_data)),
                    _ => std::process::abort()
                }
            } else {
                match self.ext_user_map.get_mut(&(re_owner_buyer_res_addr,re_owner_buyer_id)) {
                    Some(v) => v.push((re_res_addr,re_id,re_data)),
                    _ => std::process::abort()
                }
            }
        }

        // Method callable to update asset's ownership within protocol's related map, 
        // providing related data as SBT resource address and ID of seller, buyer and asset. 
        // Method callable by authorized external protocols once provided Caller Badge proof
        pub fn ext_transfer_property(
            &mut self, 
            re_owner_seller_res_addr: ResourceAddress, 
            re_owner_seller_id: NonFungibleId,
            re_owner_buyer_res_addr: ResourceAddress, 
            re_owner_buyer_id: NonFungibleId,
            re_res_addr: ResourceAddress, 
            re_id: NonFungibleId,
            re_data_str: Vec<String>,
            re_data_val: Vec<u8>,
            caller_cmp_addr: ComponentAddress,
            auth_ref: Proof
        ) {
            let re_data = self.data_assembler(re_data_str, re_data_val);

            // Verify if Caller Badge is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.caller_map_write.get(&caller_cmp_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!(" Upgrade Badge authorization failed! ");       
                    std::process::abort()     
                } 
            } 

            match self.user_map.get_mut(&(re_owner_seller_res_addr,re_owner_seller_id)) {
                Some(v) => v.retain(|x| x.0 != re_res_addr && x.1 != re_id), 
                _ => std::process::abort()
            }

            if self.ext_user_map.contains_key(&(re_owner_buyer_res_addr,re_owner_buyer_id.clone())) {
                match self.ext_re_map.get_mut(&(re_owner_buyer_res_addr,re_owner_buyer_id.clone())) {
                    Some(v) => v.push((re_res_addr,re_id,re_data)),
                    _ => std::process::abort()
                }
            } else {
                let mut v = Vec::new();
                v.push((re_res_addr,re_id,re_data));
                self.ext_re_map.insert((re_owner_buyer_res_addr,re_owner_buyer_id),v);
            }
        }

        // Method callable to update asset's ownership within protocol's related map, 
        // providing related data as SBT resource address and ID of seller, buyer and asset. 
        // Method callable by authorized external protocols once provided Caller Badge proof
        pub fn trans_property(
            &mut self, 
            re_owner_seller_res_addr: ResourceAddress, 
            re_owner_seller_id: NonFungibleId,
            re_owner_buyer_res_addr: ResourceAddress, 
            re_owner_buyer_id: NonFungibleId,
            re_res_addr: ResourceAddress, 
            re_id: NonFungibleId,
            re_data_str: Vec<String>,
            re_data_val: Vec<u8>,
            caller_cmp_addr: ComponentAddress,
            auth_ref: Proof
        ) {
            let re_data = self.data_assembler(re_data_str, re_data_val);

            // Verify if Caller Badge is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.caller_map_write.get(&caller_cmp_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!(" Upgrade Badge authorization failed !");       
                    std::process::abort()     
                } 
            } 

            if re_owner_seller_res_addr == self.user_sbt {
                match self.user_map.get_mut(&(re_owner_seller_res_addr,re_owner_seller_id.clone())) {
                    Some(v) => v.retain(|x| x.0 != re_res_addr && x.1 != re_id), 
                    _ => std::process::abort()
                }
            } else {
                match self.ext_user_map.get_mut(&(re_owner_seller_res_addr,re_owner_seller_id.clone())) {
                    Some(v) => v.retain(|x| x.0 != re_res_addr && x.1 != re_id), 
                    _ => {
                        let vec: Vec<(ResourceAddress,NonFungibleId,AssetNFT)> = Vec::new();
                        self.ext_user_map.insert((re_owner_seller_res_addr,re_owner_seller_id.clone()),vec);
                    }
                }
            }

            if re_owner_buyer_res_addr == self.user_sbt {
                match self.user_map.get_mut(&(re_owner_buyer_res_addr,re_owner_buyer_id)) {
                    Some(v) => v.push((re_res_addr,re_id,re_data)),
                    _ => std::process::abort()
                }
            } else {
                if self.ext_user_map.contains_key(&(re_owner_buyer_res_addr,re_owner_buyer_id.clone())) {
                    match self.ext_user_map.get_mut(&(re_owner_buyer_res_addr,re_owner_buyer_id.clone())) {
                        Some(v) => v.push((re_res_addr,re_id,re_data)),
                        _ => std::process::abort()
                    }
                } else {
                    let mut v = Vec::new();
                    v.push((re_res_addr,re_id,re_data));
                    self.ext_user_map.insert((re_owner_buyer_res_addr,re_owner_buyer_id),v);
                }


            }
        }

        // Method callable to verify, providing related data as SBT resource address and ID,
        // if a user is registered within protocol's map.
        // Method callable by authorized external protocols once provided Caller Badge proof
        pub fn verify_user(
            &mut self, 
            re_owner_res_addr: ResourceAddress, 
            re_owner_id: NonFungibleId, 
            caller_cmp_addr: ComponentAddress,
            flag: u8,
            auth_ref: Proof
        ) -> bool {             
            // Verify if Upgrade Caller Component is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match flag {
                0 => match self.caller_map_read.get(&caller_cmp_addr){
                    Some(addr) => assert!(*addr == auth_ref.resource_address()),
                    None => {
                        info!(" Caller Badge authorization failed! ");       
                        std::process::abort()     
                    } 
                }, 
                _ => match self.caller_map_write.get(&caller_cmp_addr){
                    Some(addr) => assert!(*addr == auth_ref.resource_address()),
                    None => {
                        info!(" Caller Badge authorization failed! ");       
                        std::process::abort()     
                    } 
                }
            }

            let mut founded = false;
            if re_owner_res_addr == self.user_sbt {
                match self.user_map.get_mut(&(re_owner_res_addr,re_owner_id)) {
                    Some(_v) => founded = true,
                    None => {
                        info!(" User unfound! ");       
                        std::process::abort()     
                    } 
                }
            }

            founded 
        }

        // Method callable to register a new Neveverland environment's user.
        // An unwithdrawable SBT is minted and returned to caller's account while his data 
        // references are stored within related protocol map. 
        pub fn register_user(&mut self, user_id: String) -> Bucket {
            let vec: Vec<(ResourceAddress,NonFungibleId,AssetNFT)> = Vec::new();
            let vec_degree: Vec<(ResourceAddress,NonFungibleId,DegreeNFT)> = Vec::new();
            let mut value: Vec<(String,Decimal)> = Vec::new();
            value.push(("".to_string(),Decimal::zero()));
            let sbt_key = NonFungibleId::random(); 

            let usr_sbt = UserSBT {
                cmp_data_address: Runtime::actor().as_component().0,
                data: user_id,
                assets: vec.clone(),
                credits: vec.clone(),
                loans: vec.clone(),
                real_estate_properties: vec.clone(),
                rental_properties: vec.clone(),
                educational_degrees: vec_degree,
                values: value
            }; 

            self.user_map.insert((self.user_sbt,sbt_key.clone()),vec);
            info!(" User SBT address added: {} ", self.user_sbt);
            info!(" User SBT id: {} ", sbt_key.clone());
         
            self.minter_badge.authorize(|| { 
                borrow_resource_manager!(self.user_sbt).mint_non_fungible(&sbt_key,usr_sbt)
            })                   
        }

            // Build a Caller Component Badge Resource 
            fn build_badge(&mut self, name: String) -> ResourceAddress {
                ResourceBuilder::new_fungible()
                .metadata("name", format!("{}",name))
                .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .no_initial_supply()
            }

            // insert provided input data into a AssetNFT structure 
            fn data_assembler(&mut self, re_data_str: Vec<String>, re_data_val: Vec<u8>) -> AssetNFT {
                AssetNFT {
                    uri : re_data_str[0].clone(),
                    data_1 : re_data_str[1].clone(),
                    data_2 : re_data_str[2].clone(),
                    data_3 : re_data_str[3].clone(),
                    data_4 : re_data_str[4].clone(),
                    value_1: re_data_val[0].clone(),
                    value_2: re_data_val[1].clone(),
                    value_3: re_data_val[2].clone(),
                    linked_assets: Vec::new()
                }
            }

            // Display input data
            fn info_map(&mut self, v: Vec<(ResourceAddress,NonFungibleId,AssetNFT)>) {
                for re in v.into_iter() {
                    info!(" Real Estate NFT resource address: {} ",re.0);
                    info!(" Real Estate NFT id: {} ",re.1);
                    info!(" Real Estate NFT data: {:?} ",re.2);
                }
            }
    }
}