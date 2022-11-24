use scrypto::prelude::*;

blueprint! {
    struct Pitia {
        // Minter Badge vault
        minter_badge: Vault,
        // Owner Badge resource address
        owner_badge: ResourceAddress,
        // Maps to store external Component address and relative Caller Badge resource 
        // address allowing it to invoke "get_number" method through external component call  
        nft_badge_map_getnmbr: HashMap<ComponentAddress,Vec<ResourceAddress>>,
        // Maps to store external Component address and relative Caller Badge resource 
        // address allowing it to invoke "get_code" method through external component call
        nft_badge_map_getcode: HashMap<ComponentAddress,Vec<ResourceAddress>>,
        // Maps to store external Component address and relative Caller Badge resource 
        // address allowing it to invoke "get_url" method through external component call
        nft_badge_map_geturl: HashMap<ComponentAddress,Vec<ResourceAddress>>,
        // Maps to store external Component address and relative Caller Badge resource 
        // address allowing it to invoke "get_bypass" method through external component call
        nft_badge_map_bypass: HashMap<ComponentAddress,Vec<ResourceAddress>>,
        // Map to store and retrieve NFT Data mint codes 
        nft_buyers_map: HashMap<u128,String>,
        // Map to store and retrieve NFT Data mint codes
        nft_user_map: HashMap<String,Vec<u128>>,
        // Map to store and retrieve NFT Data mint codes
        nft_buyers_map_url: HashMap<(u128,String),String>,
        // Map to store and retrieve NFT Data mint codes
        nft_user_map_url: HashMap<String,Vec<(u128,String)>>,
        switch_id_detect: String,
        // Index variable
        index: usize
    }

    impl Pitia {
        pub fn new() -> (ComponentAddress,Bucket) {
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
                .method("insert_nft_buyer_data", rule!(require(owner_badge.resource_address())))
                .method("insert_nft_buyer_url", rule!(require(owner_badge.resource_address())))
                .method("info_map_getcode", rule!(require(owner_badge.resource_address())))         
                .method("info_map_geturl", rule!(require(owner_badge.resource_address())))    
                .method("info_user_map_getcode", rule!(require(owner_badge.resource_address())))         
                .method("info_user_map_geturl", rule!(require(owner_badge.resource_address())))   
                .method("info_badge_map_getnmbr", rule!(require(owner_badge.resource_address())))
                .method("info_badge_map_getcode", rule!(require(owner_badge.resource_address())))
                .method("info_badge_map_geturl", rule!(require(owner_badge.resource_address())))
                .method("info_badge_map_bypass", rule!(require(owner_badge.resource_address())))
                
                .default(rule!(allow_all)); 
        
            let mut pitia: PitiaComponent = Self {
                minter_badge: Vault::with_bucket(minter_badge),
                owner_badge: owner_badge.resource_address(),
                nft_badge_map_getnmbr: HashMap::new(),
                nft_badge_map_getcode: HashMap::new(),
                nft_badge_map_geturl: HashMap::new(),
                nft_badge_map_bypass: HashMap::new(),
                nft_buyers_map: HashMap::new(),
                nft_user_map: HashMap::new(),
                nft_buyers_map_url: HashMap::new(),
                nft_user_map_url: HashMap::new(),
                switch_id_detect: "".to_string(),
                index: 0
            }
            .instantiate();
            pitia.add_access_check(access_rules);

            (pitia.globalize(),owner_badge)
        }    

        // Return a random seed u128 number upon badge verification.
        pub fn get_number(
            &mut self, 
            caller_cmp_addr: ComponentAddress,
            _id: String, 
            url: String, 
            auth: Proof 
        ) -> Vec<(u128,String)> {
            let auth: ValidatedProof = auth.unsafe_skip_proof_validation();
            let mut found = false;
            match self.nft_badge_map_getnmbr.get(&caller_cmp_addr){       
                Some(v) => {  
                    for addr in v {
                        if *addr == auth.resource_address() {
                            found = true;
                        }
                    }
                }
                None => {
                    info!("[get_number]:Badge authorization failed ! ");       
                    std::process::abort()     
                } 
            } 
            assert!(found,"[get_number]:Badge authorization failed ! ");

            let data = Runtime::generate_uuid();
            info!(" data {} ",data);
            let mut v: Vec<(u128,String)> = Vec::new();
            v.push((data,url));
            
            v
        }   

        // Retrieve the NFT production codes relate to a specific buyer id upon badge verification
        // This Fn is called by RadishFarm searching for data with purpose of mint a NFT. 
        pub fn get_code(
            &mut self, 
            caller_cmp_addr: ComponentAddress,
            id: String,
            url: String, 
            auth: Proof 
        ) -> Vec<(u128,String)> {  
            let auth: ValidatedProof = auth.unsafe_skip_proof_validation();
            let mut found = false;
            match self.nft_badge_map_getcode.get(&caller_cmp_addr){       
                Some(v) => {  
                    for addr in v {
                        if *addr == auth.resource_address() {
                            found = true;
                        }
                    }
                }
                None => {
                    info!("[get_code]:Badge authorization failed ! ");       
                    std::process::abort()     
                } 
            } 
            assert!(found,"[get_code]:Badge authorization failed ! ");

            // Verify if buyer id has changed, in the case reset the vector index.
            if self.switch_id_detect != id {
                self.index = 0;
                self.switch_id_detect = id.clone();
            }
        
            let mut v: Vec<u128> = Vec::new();

            // Verify if a second map relating buyer id and a vector containing a list
            // of his NFT production codes exists otherwise create it. 
            match self.nft_user_map.get(&id) {
                Some(_v) => info!(" NFT user address {} found ",id),        
                _ => { 
                    for (key,value) in self.nft_buyers_map.iter() { 
                        if value == &id {
                            v.push(*key);
                        }    
                    };
                    assert!(v.len() > 0, "[get_code]:No ID found within related map ");
                    self.nft_user_map.insert(id.clone(),v.clone());
                    info!(" NFT user address {} map created ",id);
                }                
            }       

            // Extract NFT production codes list vector relate to a buyer id and retrieve
            // from it a NFT production code using an index.
            let w = self.nft_user_map.get(&id).unwrap();
            let data = *w.get(self.index).unwrap_or(&u128::MAX); 
            if data == u128::MAX {
               self.index = 0;
               info!("[get_code]:Wrong amount mint attempt ");
               std::process::abort() 
            }  
            self.index += 1;

            // Advice all NFT production codes has been extracted.
            if self.index == w.len() {
                info!("[get_code]:All NFT production codes has been extracted !");
            }
            info!("[get_code]:data {} ",data);
            let mut z: Vec<(u128,String)> = Vec::new();
            z.push((data,url));
            
            z
        }

        // Retrieve the NFT production codes & NFT urls relate to a specific buyer id upon badge verification.       
        pub fn get_url(
            &mut self, 
            caller_cmp_addr: ComponentAddress,
            id: String, 
            _url: String, 
            auth: Proof 
        ) -> Vec<(u128,String)> { 
            let auth: ValidatedProof = auth.unsafe_skip_proof_validation(); 
            let mut found = false;
            match self.nft_badge_map_geturl.get(&caller_cmp_addr){       
                Some(v) => {  
                    for addr in v {
                        if *addr == auth.resource_address() {
                            found = true;
                        }
                    }
                }
                None => {
                    info!("[get_url]:Badge authorization failed ! ");       
                    std::process::abort()     
                } 
            } 
            assert!(found,"[get_url]:Badge authorization failed ! ");     

            // Verify if buyer id has changed, in the case reset the vector index.
            if self.switch_id_detect != id {
                self.index = 0;
                self.switch_id_detect = id.clone();
            }
        
            let mut v: Vec<(u128,String)> = Vec::new();

            // Verify if a second map relating buyer id and a vector containing a list
            // of his NFT production codes exists otherwise create it. 
            match self.nft_user_map_url.get(&id) {
                Some(_v) => info!(" NFT user address {} found ",id),        
                _ => { 
                    for (key,value) in self.nft_buyers_map_url.iter() { 
                        if value == &id {
                            v.push(key.clone());
                        }    
                    };
                    assert!(v.len() > 0, "[get_url]:No ID found within related map ");
                    self.nft_user_map_url.insert(id.clone(),v.clone());
                    info!(" NFT user address {} map created ",id);
                }                
            }       

            // Extract NFT production codes list vector relate to a buyer id and retrieve
            // from it a NFT production code using an index.
            let w = self.nft_user_map_url.get(&id).unwrap();
            let (data,url) = w.get(self.index).unwrap_or(&(u128::MAX,"".to_string())).clone(); 
            if data == u128::MAX && url == "".to_string() {
               info!(" Wrong amount mint attempt ");
               std::process::abort() 
            }  
            self.index += 1;
            
            // Advice all NFT production codes has been extracted.
            if self.index == w.len() {
                info!(" All NFT production codes has been extracted !");
            }
            v.clear();
            v.push((data,url));
            
            v
        }

        // Convert input string to number NFT production code and return with input url upon badge verification.
        pub fn get_bypass(
            &mut self, 
            caller_cmp_addr: ComponentAddress,
            id: String, 
            url: String, 
            auth: Proof 
        ) -> Vec<(u128,String)> {
            let auth: ValidatedProof = auth.unsafe_skip_proof_validation();
            let mut found = false;
            match self.nft_badge_map_bypass.get(&caller_cmp_addr){       
                Some(v) => {  
                    for addr in v {
                        if *addr == auth.resource_address() {
                            found = true;
                        }
                    }
                }
                None => {
                    info!("[get_bypass]:Badge authorization failed ! ");       
                    std::process::abort()     
                } 
            } 
            assert!(found,"[get_bypass]:Badge authorization failed ! ");     

            let mut v: Vec<(u128,String)> = Vec::new();
            v.push((id.parse::<u128>().unwrap(),url));
            
            v
        }

        // Admin tools

            // Mint a Badge to transfer to another Component allowing the latter to call
            // Oracle Methods. Protocol Owner Badge required.
        pub fn nft_badge_mint(&mut self, caller_cmp_addr: ComponentAddress, flag: u8) -> Bucket {
            let mut v: Vec<ResourceAddress> = Vec::new();
            let nft_badge_res_def = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " Pitia Caller Badge ")
                .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .no_initial_supply();

            v.push(nft_badge_res_def);
            match flag {
                0 => self.nft_badge_map_getnmbr.entry(caller_cmp_addr)                      // random mint mode
                                               .and_modify(|z| z.push(nft_badge_res_def))
                                               .or_insert(v),             
                1 => self.nft_badge_map_getcode.entry(caller_cmp_addr)                      // order mint mode (seed)
                                               .and_modify(|z| z.push(nft_badge_res_def))
                                               .or_insert(v),       
                2 => self.nft_badge_map_geturl.entry(caller_cmp_addr)                       // order mint mode (seed + url)
                                              .and_modify(|z| z.push(nft_badge_res_def))
                                              .or_insert(v), 
                _ => self.nft_badge_map_bypass.entry(caller_cmp_addr)                       // order mint mode (bypass)
                                              .and_modify(|z| z.push(nft_badge_res_def))
                                              .or_insert(v) 
            };
            info!(" Component address stored: {} ", caller_cmp_addr);
                
            return self.minter_badge
                .authorize(|| { borrow_resource_manager!(nft_badge_res_def).mint(1) })
        }

            // Insert NFT buyer id and related NFT production code. Protocol Owner Badge required.
        pub fn insert_nft_buyer_data(&mut self, id: String, nft_data_code: u128){
            self.nft_buyers_map.insert(nft_data_code,id);
        }

            // Insert NFT buyer id and related NFT production code. Protocol Owner Badge required.
        pub fn insert_nft_buyer_url(
            &mut self, 
            id: String, 
            nft_data_code: u128,
            nft_url: String
        ){
            self.nft_buyers_map_url.insert((nft_data_code,nft_url),id);
        }

            // Retrieve a tuple (NFT production code, buyer id) from Oracle HashMap. 
            // Protocol Owner Badge required.
        pub fn info_map_getcode(&self) -> HashMap<u128,String> {                                                 
            for (a,b) in self.nft_buyers_map.iter() {   
                info!(" id: {}, code: {} ",a,b);
            }

            self.nft_buyers_map.clone()
        } 

            // Retrieve a tuple (NFT production code, Url, buyer id) from Oracle HashMap.
            // Protocol Owner Badge required.
        pub fn info_map_geturl(&self) -> HashMap<(u128,String),String> {                                                  
            for (key,val) in self.nft_buyers_map_url.iter() {   
                    info!(" code: {}, url: {}, id: {} ", key.0, key.1, val);
            }

            self.nft_buyers_map_url.clone()
        }

            // Retrieve a tuple (buyer id, NFT production code) from Oracle HashMap.
            // Protocol Owner Badge required.
        pub fn info_user_map_getcode(&self) -> HashMap<String,Vec<u128>> {                                               
            for (a,b) in self.nft_user_map.iter() {   
                info!(" id: {}, code: {:?} ",a,b);
            }

            self.nft_user_map.clone()
        } 

            // Retrieve a tuple (buyer id NFT production code, Url) from Oracle HashMap.
            // Protocol Owner Badge required.
        pub fn info_user_map_geturl(&self) -> HashMap<String,Vec<(u128,String)>> {                                                
            for (key,v) in self.nft_user_map_url.iter() {
                for val in v {   
                    info!(" id: {}, code: {}, url: {} ", key, val.0, val.1);
                }
            }

            self.nft_user_map_url.clone()
        }

            // Retrieve inserted External Component address & related Badge Resource address from Oracle HashMap.
            // Protocol Owner Badge required.
        pub fn info_badge_map_getnmbr(&self) -> HashMap<ComponentAddress,Vec<ResourceAddress>> {                                                
            for (key,v) in self.nft_badge_map_getnmbr.iter() {
                for val in v {   
                    info!(" Map: getnmbr. Component: {}, Badge: {} ", key, val);
                }
            }

            self.nft_badge_map_getnmbr.clone()
        } 

            // Retrieve inserted External Component address & related Badge Resource address from Oracle HashMap
            // Protocol Owner Badge required.
        pub fn info_badge_map_getcode(&self) -> HashMap<ComponentAddress,Vec<ResourceAddress>> {                                                
            for (key,v) in self.nft_badge_map_getcode.iter() {
                for val in v {   
                    info!(" Map: getcode. Component: {}, Badge: {} ", key, val);
                }
            }

            self.nft_badge_map_getcode.clone()
        }

            // Retrieve inserted External Component address & related Badge Resource address from Oracle HashMap
            // Protocol Owner Badge required.
        pub fn info_badge_map_geturl(&self) -> HashMap<ComponentAddress,Vec<ResourceAddress>> {                                                
            for (key,v) in self.nft_badge_map_geturl.iter() {
                for val in v {   
                    info!(" Map: geturl. Component: {}, Badge: {} ", key, val);
                }
            }

            self.nft_badge_map_geturl.clone()
        }

            // Retrieve inserted External Component address & related Badge Resource address from Oracle HashMap
            // Protocol Owner Badge required.
        pub fn info_badge_map_bypass(&self) -> HashMap<ComponentAddress,Vec<ResourceAddress>> {                                                
            for (key,v) in self.nft_badge_map_bypass.iter() {
                for val in v {   
                    info!(" Map: bypass. Component: {}, Badge: {} ", key, val);
                }
            }

            self.nft_badge_map_bypass.clone()
        }
    }
}