use scrypto::prelude::*;
use crate::data_farm::*;

blueprint! {
    struct AssetFarm {
        // Protocol owner badge.
        owner_badge: ResourceAddress,
        // A vault that holds the mint badge
        asset_nft_minter_badge: Vault,
        // Resource definition of Asset NFT series
        asset_nft_resource_def: ResourceAddress,
        // Resource definition of merge Asset NFT series
        merge_asset_nft_resource_def: ResourceAddress,
        // NFT Series circulation supply
        circulation_supply: u128,
        // Price of a asset NFT
        asset_nft_price: Decimal,
        // Price to perform asset NFTs merge
        asset_nft_merge_price: Decimal,
        // Price to perform asset NFTs upgrade
        asset_nft_upgrade_price: Decimal,
        // Counter for NFT ID generation
        asset_nft_id_counter: u128,
        // Counter for merged NFT ID generation
        asset_merged_nft_id_counter: u128,
        // HashMap of minted NFT
        minted_nft: HashMap<(String,String,String,String),ComponentAddress>,
        // HashMap of minted merged NFT
        minted_merged_nft: HashMap<(String,String,String,String),ComponentAddress>, 
        // Hashmap of Nft Badges to access Data Component
        nft_badge_vault: HashMap<ResourceAddress,Vault>, 
        // Hashmap to relate Data Component address with Nft Badges address
        nft_data: HashMap<ComponentAddress,ResourceAddress>,
        // Map of external component and relative badge resource addresses authorized to perform 
        // component calls and mint a NFT resource 
        mint_nft_badge_map: HashMap<ComponentAddress,ResourceAddress>,
        // Map of external component and relative badge resource addresses authorized to perform 
        // component calls and merge NFT resources 
        merge_nft_badge_map: HashMap<ComponentAddress,ResourceAddress>,
        // Map of external component and relative badge resource addresses authorized to perform 
        // component calls and upgrade a NFT resource 
        upgrade_nft_badge_map: HashMap<ComponentAddress,ResourceAddress>,
        // Vault to collect XRD payments
        collected_xrd: Vault,
        // Vault to collect non-XRD payments
        collected_tkn: Vault,
        // List of mergeable resources and relative component creator able to burn them once a merge
        // method has been called  
        mergeable_nfts_vec: Vec<(ResourceAddress,ComponentAddress)>,    
        // Non-XRD currency ResourceAddress
        currency: ResourceAddress,  
        // Contribution share to support Academy protocol
        academy_contribution_share: Decimal,
        // Bool var to specify if Buy NFT Fn is callable without Authorization
        mint_flag: bool,         
        // Bool var to specify if Merge NFT Fn is callable without Authorization
        merge_flag: bool,
        // Flag to check if NFT to mint is a merged one to correctly check data redundancy
        merge_nft_flag : bool,
        // Bool var to specify if NFTs sharing same Resource Definition are mergeable
        merge_same_nft: bool,       
        // Limit of buyable NFTs within a single Tx
        max: u32,
        // Counter of minted NFT in a single Tx 
        counter: u32,
        // Tkn struct to store and retrieve resource and component addresses used by protocol.
        tkn: Tkn
    }

    impl AssetFarm {
        pub fn new( 
            dex: ComponentAddress,                  // Dex component address
            currency: ResourceAddress,              // Nft series buying currency
            series: String,                         // Series name
            nmbr: Decimal,                          // Series number
            merge_series: String,                   // Merge series name
            merge_nmbr: Decimal,                    // Merge Series number
            supply: u128,                           // Sum of NFT to mint within series 
            buy_price: Decimal,                     // Price per NFT 
            merge_price: Decimal,                   // Price to merge NFT
            upgrade_price: Decimal,                 // Price to upgrade NFT
            mint_flag: bool,                        // True if mint NFT Fn is callable without Auth, False if not 
            merge_flag: bool,                       // True if merge NFT Fn is callable without Auth, False if not
            max: u32                                // Max NFT buyable amount within a single Tx 
        ) -> (ComponentAddress,Bucket) {
            // Create a Protocol Owner Badge resource  
            let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " OwnerBadge ")
                .initial_supply(1);
            // Create a Protocol Minter Badge resource
            let asset_nft_minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("Name", "Asset NFT Minter Badge")
                .initial_supply(1);
            // Create an NFT resource with mutable supply    
            let asset_nft_resource_def = ResourceBuilder::new_non_fungible()
                .metadata("Ecosystem", "Asset")
                .metadata("Series", series)
                .metadata("Number", nmbr.to_string())
                .mintable(rule!(require(asset_nft_minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(asset_nft_minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(asset_nft_minter_badge.resource_address())), LOCKED)
                .no_initial_supply();
            // Create a merge NFT resource with mutable supply    
            let merge_asset_nft_resource_def = ResourceBuilder::new_non_fungible()
                .metadata("Ecosystem", "Asset")
                .metadata("Series", merge_series)
                .metadata("Number", merge_nmbr.to_string())
                .mintable(rule!(require(asset_nft_minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(asset_nft_minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(asset_nft_minter_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let access_rules = AccessRules::new()
                .method("mint_nft_badge", rule!(require(owner_badge.resource_address())))
                .method("merge_nft_badge", rule!(require(owner_badge.resource_address())))
                .method("upgrade_nft_badge", rule!(require(owner_badge.resource_address())))
                .method("init_cmp_addr", rule!(require(owner_badge.resource_address())))
                .method("transfer_badge", rule!(require(owner_badge.resource_address())))
                .method("mergeable_nfts", rule!(require(owner_badge.resource_address())))
                .method("set_academy_values", rule!(require(owner_badge.resource_address())))
                .method("xrd_withdrawal", rule!(require(owner_badge.resource_address())))
                .method("tkn_withdrawal", rule!(require(owner_badge.resource_address())))
                .method("info_minted_map", rule!(require(owner_badge.resource_address())))
                .method("info_merged_map", rule!(require(owner_badge.resource_address())))
                .default(rule!(allow_all));

            let mut asset_farm: AssetFarmComponent = Self {
                owner_badge: owner_badge.resource_address(),
                asset_nft_minter_badge: Vault::with_bucket(asset_nft_minter_badge),
                asset_nft_resource_def,
                merge_asset_nft_resource_def,
                circulation_supply: supply,
                asset_nft_price: buy_price,
                asset_nft_merge_price: merge_price,
                asset_nft_upgrade_price: upgrade_price,
                asset_nft_id_counter: 0,
                asset_merged_nft_id_counter: 0,
                minted_nft: HashMap::new(),
                minted_merged_nft: HashMap::new(),
                nft_badge_vault: HashMap::new(),
                nft_data: HashMap::new(),
                mint_nft_badge_map: HashMap::new(),
                merge_nft_badge_map: HashMap::new(), 
                upgrade_nft_badge_map: HashMap::new(),
                collected_xrd: Vault::new(RADIX_TOKEN),
                collected_tkn: Vault::new(currency),
                mergeable_nfts_vec: Vec::new(),
                currency,
                academy_contribution_share: Decimal::zero(),
                mint_flag,
                merge_flag,
                merge_nft_flag : false,
                merge_same_nft: false,
                max,
                counter: 0,
                tkn: Tkn::new(dex,currency)
            }
            .instantiate();
            asset_farm.add_access_check(access_rules);

            (asset_farm.globalize(),owner_badge)
        }

            // Check TKN total amount in vault.
        pub fn check_amounts(&self) -> (Decimal,Decimal) {
            let xrd_amnt = self.collected_xrd.amount();
            let tkn_amnt = self.collected_tkn.amount();
            info!(" XRD amount in vault: {}, TKN amount in vault: {} ",xrd_amnt,tkn_amnt);

            (xrd_amnt,tkn_amnt)
        }

            // Check NFT minted and merged stats.
        pub fn check_mint_amounts(&self) -> (u32,u128,u128,u128) {
            let max_mint_tx = self.max;
            let max_supply = self.circulation_supply;
            let minted_amnt = self.asset_nft_id_counter;
            let merged_amnt = self.asset_merged_nft_id_counter; 
            info!(" 
                    Max NFT mintable per Tx: {} 
                    Max NFT series supply: {} 
                    Total NFT series minted amount: {} 
                    Total NFT series merged amount: {} 
                    ",max_mint_tx, max_supply, minted_amnt, merged_amnt
            );

            (max_mint_tx, max_supply, minted_amnt, merged_amnt)
        }

            // Retrieve NFT mint,merge or upgrade price given an imput currency 
        pub fn get_mint_price(&mut self, fx_currency: ResourceAddress, flag: u8) -> Decimal {  
            let price: Decimal;
            match flag {
                0 => price = self.asset_nft_price,      
                1 => price = self.asset_nft_merge_price,       
                _ => price = self.asset_nft_upgrade_price
            }
            if fx_currency == self.currency {
                price
            } else {    
                let method = "get_token_sell_amount_becsc".to_string(); 
                let args = args![price, self.currency, fx_currency];
                borrow_component!(self.tkn.asset_dex_address).call::<Decimal>(&method, args)
            }
        }

        // Fn Implemented to mint Asset Nft's.
        pub fn mint_asset_nft(
            &mut self, 
            amount: u32,                    // Amount of Nft to buy and mint 
            mut payment: Bucket,            // Payment provided  
            mint_code_id: String,           // Mint code to retrieve mint data from pitia oracle
            url: String,                    // NFT rendered image Url 
            asset_surface: u8,              // Asset surface in square meters
            pitia_addr: ComponentAddress,   // External Oracle Component ResourceAddress
            pitia_method: String,           // External Oracle Method to call 
            ext_addr: ComponentAddress,     // External Data Component ResourceAddress            
            method_x: String,               // External Data first Method to call
            method_y: String,               // External Data second Method to call
            method_z: String,               // External Data third Method to call
            method_w: String,               // External Data fourth Method to call
            auth_ref: Proof                 // Caller Component Authorization Badge
        ) -> (Bucket, Vec<Bucket>) {   
            // Verify there's enough Nft to mint
            assert!(
                self.asset_nft_id_counter < self.circulation_supply,
                " All supplied Asset NFTs have been minted! "
            );
            // Verify user provided enough tokens to purchase Nft
            assert!(
                payment.amount() >= self.asset_nft_price,
                " Please provide a sufficient payment amount! "
            );
            // Verify amount of Nft requested by users doesn't exceed buyable Nft number per single Tx
            assert!(
                amount <= self.max,
                " Limit of buyable Asset NFTs within a single Tx exceeded! " 
            );
            // Verify if Fn is callable with no badge or if mint caller Component is authorized
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation(); 
            if self.mint_flag != true {
                match self.mint_nft_badge_map.get(&self.tkn.mint_component_addr){
                    Some(addr) => assert!(*addr == auth_ref.resource_address()),
                    None => {
                        info!(" Mint NFT Badge authorization failed !");       
                        std::process::abort()     
                    } 
                } 
            }

            // Vector to store minted Nft
            let mut vector_nft: Vec<Bucket> = Vec::new();

            // Reset Nft counter per Tx 
            self.counter = 0;
            let flag: bool = true;
            let buy_flag: bool = true;

            // Loop to mint requested amount of Nft per single Tx
            loop {
                // Create a flag to avoid data rendundancy within minted Nfts
                let valid_nft_minted_flag = self.counter;

                // Call a Fn to populate Nft data and then mint Nft
                let new_nft = self.build_asset_nft( 
                    mint_code_id.clone(), 
                    url.clone(), 
                    asset_surface,
                    pitia_addr.clone(),
                    pitia_method.clone(), 
                    ext_addr.clone(), 
                    method_x.clone(),
                    method_y.clone(),
                    method_z.clone(),
                    method_w.clone(),
                    flag,
                    buy_flag
                );

                // Verify there's no data rendundancy within last minted Nft
                if valid_nft_minted_flag != self.counter {
                    
                    // Push fresh minted Nft into store Vector
                    vector_nft.push(new_nft);

                    // Collect payment
                    payment = self.collect_payment(payment,0);
                } else {
                    // Burn fresh minted once verified data rendundancy issues
                    self.asset_nft_minter_badge.authorize(|| {new_nft.burn()}); 
                }
                // Verify minted amounts of current Tx and general minted amounts
                if self.counter == amount || self.asset_nft_id_counter == self.circulation_supply {
                    break;
                }
            } 

            (payment, vector_nft)
        }

            // Burn a NFT asset previously minted by protocol 
        pub fn asset_nft_burn(&mut self, nft_one: Bucket) -> bool {    
                assert!( 
                    nft_one.resource_address() == self.asset_nft_resource_def,
                    " Please provide right NFT to burn! "   
                );  
                assert!(nft_one.amount() == Decimal::one()," Please provide only one NFT to burn! ");  

                self.asset_nft_minter_badge.authorize(|| {nft_one.burn()});

                return true
        } 

            // Upgrade  a NFT asset previously minted by protocol
        pub fn upgrade_asset_nft(
            &mut self, 
            mut payment: Bucket,                                        // payment 
            nft_up: Bucket,                                             // NFT to upgrade
            mut linked_assets: Vec<(ResourceAddress,NonFungibleId)>,    // linked assets data to update
            building_surface: u8,                                       // surface data to update
            auth_ref: Proof                                             // authorized badge proof     
        ) -> (Bucket,Bucket) { 
            // Verify user provided enough tokens to upgrade NFT
            assert!(
                payment.amount() >= self.asset_nft_upgrade_price,
                " Please provide a sufficient payment amount ! "
            );  
            // Verify user provided right number of NFT to upgrade
            assert!(nft_up.amount() == Decimal::one()," Please pass 1 Asset NFTs to upgrade ");

            // Verify if upgrade caller Component is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.upgrade_nft_badge_map.get(&self.tkn.upgrade_component_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!(" Upgrade NFT Badge authorization failed !");       
                    std::process::abort()     
                } 
            } 

            let nft_key = nft_up.non_fungible_ids().into_iter().collect::<Vec<NonFungibleId>>();

            // Retrieve and upgrade mutable data   
            let mut nft_mutable_data: AssetNFT = nft_up.non_fungible().data();
            nft_mutable_data.value_1 += 1;
            nft_mutable_data.value_2 += building_surface;
            nft_mutable_data.linked_assets.append(&mut linked_assets);

            // Collect payment
            payment = self.collect_payment(payment,2);
            
            self.asset_nft_minter_badge.authorize(|| {
                borrow_resource_manager!(nft_up.resource_address())
                    .update_non_fungible_data(&nft_key.get(0).unwrap(), nft_mutable_data)
            });

            (payment,nft_up)
        }

            // Merge a couple of NFT into a new one minted by protocol
            // NFT doesn't necessarly need to be minted by protocol but in this case is required to 
            // have minter protocol component address and related NFT resource address retrivable 
            // within relative map aiming to burn provided NFTs resources once minted new merged NFT  
        pub fn merge_asset_nft(
            &mut self, 
            nft_one: Bucket,                            // First Asset Nft to merge
            nft_two: Bucket,                            // Second Asset Nft to merge
            mut payment: Bucket,                        // Payment provided 
            mint_code_id: String,                       // Tx sender identifier
            url: String,                                // NFT rendered image Url  
            pitia_addr: ComponentAddress,               // External Oracle Component ResourceAddress
            pitia_method: String,                       // External Oracle Method to call 
            ext_addr: ComponentAddress,                 // External Data Component ResourceAddress            
            method_x: String,                           // External Data first Method to call
            method_y: String,                           // External Data second Method to call
            method_z: String,                           // External Data third Method to call
            method_w: String,                           // External Data fourth Method to call
            auth_ref: Proof                             // Caller Component Authorization Badge
        ) -> (Bucket,Bucket) { 
             // Verify user provided enough tokens to merge NFTs
            assert!(
                payment.amount() >= self.asset_nft_merge_price,
                " Please provide a sufficient payment amount ! "
            );  
            // Verify user provided right number of NFTs to merge
            assert!(
                nft_one.amount() == Decimal::one() && nft_two.amount() == Decimal::one(),
                " Please pass 2 Asset NFTs to merge "
            );    
            // Verify if Fn is callable with no badge or if merge caller Component is authorized 
            if self.merge_flag != true {
                let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
                match self.merge_nft_badge_map.get(&self.tkn.merge_component_addr){
                    Some(addr) => assert!(*addr == auth_ref.resource_address()),
                    None => {
                        info!(" Merge NFT Badge authorization failed !");       
                        std::process::abort()     
                    } 
                } 
            }
            // Randomly inizialize a couple mutable ComponentAddress-type var,
            // real ones, if any required (NFT minted within external farm protocols), are then 
            // retrieved from related vector map
            let mut comp_addr_one = self.tkn.asset_dex_address;
            let mut comp_addr_two = self.tkn.asset_dex_address;

            // Verify NFTs provided by user are mergeable
            for a in &self.mergeable_nfts_vec[..]{
                if a.0 == nft_one.resource_address() {
                    info!(" Correspondent NFT one res addr founded !");
                    for b in &self.mergeable_nfts_vec[..]{
                        if b.0 == nft_two.resource_address() {
                            info!(" Correspondence NFT two res addr founded !");
                            comp_addr_one = a.1;
                            comp_addr_two = b.1;
                            if !self.merge_same_nft {
                                assert!(comp_addr_one != comp_addr_two,"Please provide right NFTs"); 
                            }
                            break;
                        } 
                    };
                }      
            };    

            // Set merge NFT flag. 
            self.merge_nft_flag = true;

            // Retrieve user provided NFTs keys & data
            let mut nft_one_data: AssetNFT = nft_one.non_fungible().data();
            let mut nft_two_data: AssetNFT = nft_one.non_fungible().data();
        
            let new_value_1 = nft_one_data.value_1 + nft_two_data.value_1;
            let new_value_2 = nft_one_data.value_2 + nft_two_data.value_2;
            let new_value_3 = nft_one_data.value_3 + nft_two_data.value_3;
            nft_one_data.linked_assets.append(&mut nft_two_data.linked_assets);
            let new_linked_assets = nft_one_data.linked_assets;
                

            // Burn original NFTs calling original creator Component Burn Method 
            let burn_method: String = "asset_nft_burn".to_string();

            if nft_one.resource_address() == self.asset_nft_resource_def {
                self.asset_nft_burn(nft_one);
                info!(" comp_addr_one: {}",comp_addr_one);
            } else {
                info!(" comp_addr_one: {}",comp_addr_one);
                let _answer = borrow_component!(comp_addr_one).call::<bool>(&burn_method,args![nft_one]);
            } 
            if nft_two.resource_address() == self.asset_nft_resource_def {
                self.asset_nft_burn(nft_two);
                 info!(" comp_addr_two: {}",comp_addr_two);
            } else {
                info!(" comp_addr_two: {}",comp_addr_two);
                let _answer = borrow_component!(comp_addr_two).call::<bool>(&burn_method,args![nft_two]);
            }

            // Bucket to store minted merged NFT
            let mut merged_nft_bckt: Bucket;

            // Reset NFT counter per Tx 
            self.counter = 0;
            let buy_flag: bool = false;
            let flag: bool = true;
            let mut exit_flag: bool = false;

            // Loop to mint merged Nft 
            loop {
                // Create a flag to avoid data rendundancy within minted merged Nft
                let valid_nft_minted_flag = self.counter;

                // Call a Fn to populate Nft data and then mint Nft
                let new_nft = self.build_asset_nft(  
                    mint_code_id.clone(), 
                    url.clone(),
                    0,
                    pitia_addr.clone(),
                    pitia_method.clone(), 
                    ext_addr.clone(), 
                    method_x.clone(),
                    method_y.clone(),
                    method_z.clone(),
                    method_w.clone(),
                    flag,
                    buy_flag
                );

                let new_nft_data: AssetNFT = new_nft.non_fungible().data();
            
                // Verify there's no data rendundancy within last minted merged Nft
                if valid_nft_minted_flag != self.counter {
                    self.asset_nft_minter_badge.authorize(|| {new_nft.burn()});
                    
                    let merged_nft = AssetNFT {
                        uri: new_nft_data.uri,
                        data_1 : new_nft_data.data_1,
                        data_2 : new_nft_data.data_2,
                        data_3 : new_nft_data.data_3,
                        data_4 : new_nft_data.data_4,
                        value_1: new_value_1,
                        value_2: new_value_2,
                        value_3: new_value_3,
                        linked_assets: new_linked_assets.clone()
                    };
    
                    merged_nft_bckt = self.asset_nft_minter_badge.authorize(|| {
                        borrow_resource_manager!(self.asset_nft_resource_def).mint_non_fungible(
                            &NonFungibleId::random(),
                            merged_nft
                        )
                    });
                    
                    exit_flag = true;
                    
                    // Collect payment
                    payment = self.collect_payment(payment,1);
                } else {                    
                    // Create an empty Bucket
                    merged_nft_bckt = Bucket::new(RADIX_TOKEN);
                    // Burn fresh minted once verified data rendundancy issues
                    self.asset_nft_minter_badge.authorize(|| {new_nft.burn()});   
                }

                if exit_flag {
                    self.merge_nft_flag = false;
                    break;
                }
            } 

            (payment,merged_nft_bckt)
        }

        // Admin methods

            // Mint a Mint NFT Badge to allow a Asset Component to call "mint_asset_nft" Fn.
            // owner_badge required to call method.
        pub fn mint_nft_badge(&mut self, mint_component_addr: ComponentAddress) -> Bucket {
            let mint_nft_badge = self.build_badge("Mint_NFT_Badge_Caller_Component".to_string());                   
            self.mint_nft_badge_map.insert(mint_component_addr,mint_nft_badge);
            self.tkn.mint_component_addr = mint_component_addr;
            info!(" Component address: {} ", self.tkn.mint_component_addr);
                
            self.asset_nft_minter_badge
                .authorize(|| { borrow_resource_manager!(mint_nft_badge).mint(Decimal::one()) })
        }

            // Mint a Merge NFT Badge to allow a Asset Component to call "merge_asset_nft" Fn. 
            // owner_badge required to call method.
        pub fn merge_nft_badge(&mut self, merge_component_addr: ComponentAddress) -> Bucket {  
            let merge_nft_badge = self.build_badge("Merge_NFT_Badge_Caller_Component".to_string());
            self.merge_nft_badge_map.insert(merge_component_addr,merge_nft_badge);
            self.tkn.merge_component_addr = merge_component_addr;
            info!(" Component address: {} ", self.tkn.merge_component_addr);
                
            self.asset_nft_minter_badge
                .authorize(|| { borrow_resource_manager!(merge_nft_badge).mint(Decimal::one()) })
        }

            // Mint a Upgrade NFT Badge to allow a Asset Component to call "upgrade_asset_nft" Fn.
            // owner_badge required to call method.
        pub fn upgrade_nft_badge(&mut self, upgrade_component_addr: ComponentAddress) -> Bucket {
            let upgrade_nft_badge = self.build_badge("Upgrade_NFT_Badge_Caller_Component".to_string());
            self.upgrade_nft_badge_map.insert(upgrade_component_addr,upgrade_nft_badge);
            self.tkn.upgrade_component_addr = upgrade_component_addr;
            info!(" Component address: {} ", self.tkn.upgrade_component_addr);
                
            self.asset_nft_minter_badge
                .authorize(|| { borrow_resource_manager!(upgrade_nft_badge).mint(Decimal::one()) })
        }

            // Init AssetFarm series Component resource address and Asset Protocol Main Vaults 
            // destination addresses to transfer collected liquidity.
            // owner_badge required to call method.
        pub fn init_cmp_addr(
            &mut self,               
            asset_dex_address: ComponentAddress,
            tkn_currency: ResourceAddress
        ){
            self.tkn.series_component_addr = Runtime::actor().as_component().0;
            self.tkn.asset_dex_address = asset_dex_address;
            self.tkn.tkn_currency = tkn_currency;
            info!(" Series component address: {} ",self.tkn.series_component_addr);
            info!(" Asset XRD main vault: {} ",self.tkn.asset_xrd_main_vault);
            info!(" Asset TKN main vault: {} ",self.tkn.asset_tkn_main_vault);
            info!(" Asset dex address: {} ",self.tkn.asset_dex_address);
            info!(" TKN currency: {} ",self.tkn.tkn_currency);
        }

            // Fn Implemented to transfer Nft Badges from Data Component to AssetFarm Component
            // callable by Protocol Owner only
            // owner_badge required to call method.
        pub fn transfer_badge(&mut self, nft_data_cmp: ComponentAddress, nft_data_bdg: Bucket){
            let nft_bdg_addr = nft_data_bdg.resource_address();
            self.nft_data.insert(nft_data_cmp,nft_bdg_addr);
            let v = self.nft_badge_vault.entry(nft_bdg_addr).or_insert(Vault::new(nft_bdg_addr));
            v.put(nft_data_bdg);
        }

            // Set Asset Academy Vault Component address & fee share. 
            // owner_badge required to call method.
        pub fn set_academy_values(&mut self, academy_comp: ComponentAddress, fee_share: Decimal) {
            assert!(fee_share <= Decimal::from(100)," Max allowed value is 100 ");
            self.tkn.academy_comp = academy_comp;
            self.academy_contribution_share = fee_share;
            info!(" Academy Vault Component Address set to {} ", self.tkn.academy_comp);
            info!(" Academy share set to {}% ", self.academy_contribution_share);
        }

            // Fn specifying mergeable resources within "merge_asset_nft" Fn within a determinated 
            // Asset NFT series Component and a bool value to specify if NFTs sharing same resource 
            // definition are mergeable together. 
            // owner_badge required to call method.
        pub fn mergeable_nfts(
            &mut self, 
            nft_res_def: ResourceAddress, 
            component_addr: ComponentAddress, 
            merge_same_nft: bool
        ){
            self.mergeable_nfts_vec.push((nft_res_def,component_addr));  
            self.merge_same_nft = merge_same_nft;              
        }

            // Withdrawal TKN tokens from vault. owner_badge required to call method.
        pub fn xrd_withdrawal(&mut self, amount: Decimal) -> Bucket {
            let xrd_bckt = self.collected_xrd.take(amount);
            info!(" $XRD withdrawn amount {} ",xrd_bckt.amount());

            xrd_bckt
        }

            // Withdrawal TKN tokens from vault. owner_badge required to call method.
        pub fn tkn_withdrawal(&mut self, amount: Decimal) -> Bucket {
            let tkn_bckt = self.collected_tkn.take(amount);
            info!(" $TKN withdrawn amount {} ",tkn_bckt.amount());

            tkn_bckt
        }

            // owner_badge required to call method.
        pub fn info_minted_map(&self) -> HashMap<(String,String,String,String),ComponentAddress> { 
            for ((a,b,c,d),addr) in self.minted_nft.iter() {
                info!(" a: {}, b: {}, c: {}, d: {}, address: {} ",a,b,c,d,addr);
            }

            self.minted_nft.clone()
        }

            // owner_badge required to call method.
        pub fn info_merged_map(&self) -> HashMap<(String,String,String,String),ComponentAddress> { 
            for ((a,b,c,d),addr) in self.minted_merged_nft.iter() {
                info!(" a: {}, b: {}, c: {}, d: {}, address: {} ",a,b,c,d,addr);
            }

            self.minted_merged_nft.clone()
        }

            // Build a Badge Resource 
        fn build_badge(&mut self, name: String) -> ResourceAddress {
            ResourceBuilder::new_fungible()
            .metadata("name", format!("{}",name))
            .mintable(rule!(require(self.asset_nft_minter_badge.resource_address())), LOCKED)
            .burnable(rule!(require(self.asset_nft_minter_badge.resource_address())), LOCKED)
            .no_initial_supply()
        }

            // Swap tokens on an external DEX
        fn swap_fx(
            &self, 
            sum: Decimal, 
            fx: ResourceAddress, 
            dex: ComponentAddress, 
            bckt_in: Bucket
        )-> Bucket {
            let method = "buy_token_sell_exact_token".to_string(); 

            borrow_component!(dex).call::<Bucket>(&method, args![sum, fx, bckt_in])
        }

            // Verify currency and collect payment for Nft upgrade
        fn collect_payment(&mut self, mut payment: Bucket, flag: u8) -> Bucket {
            let price: Decimal;
            match flag {
                0 => price = self.asset_nft_price,
                1 => price = self.asset_nft_upgrade_price,
                 _ => price = self.asset_nft_merge_price
            }
            if payment.resource_address() == self.currency && self.currency != RADIX_TOKEN {
                let mut fee_amnt = Decimal::from(0);
                if payment.resource_address() == self.tkn.tkn_currency {
                    fee_amnt = price*self.academy_contribution_share/Decimal::from(100);
                    assert_eq!(fee_amnt, self.tkn_lock(payment.take(fee_amnt)));
                }
                self.collected_tkn.put(payment.take(price-fee_amnt));   
            } else if payment.resource_address() == RADIX_TOKEN {
                self.collected_xrd.put(payment.take(price));
            } else {
                payment = self.swap_fx(price,self.currency,self.tkn.asset_dex_address,payment);
            }

            payment
        }

            // Transfer $TKN token fee share in Academy Vault Component 
        fn tkn_lock(&mut self, fee_bckt: Bucket) -> Decimal {
            let method = "tkn_lock".to_string(); 

            borrow_component!(self.tkn.academy_comp).call::<Decimal>(&method, args![fee_bckt])
        }

            // Build Nft Data calling external Data Components and then mint it 
        fn build_asset_nft(
            &mut self, 
            mint_code_id: String,   
            url: String, 
            asset_surface: u8,       
            pitia_addr: ComponentAddress,
            pitia_method: String, 
            ext_addr: ComponentAddress, 
            method_x: String,
            method_y: String,
            method_z: String, 
            method_w: String, 
            mut flag: bool,
            buy_flag: bool
        ) -> Bucket {

            // Retrieve Nft Badges for External Oracle Component authorization purpose
            let pitia_bdg_addr = self.nft_data.get(&pitia_addr).unwrap().clone();
            let pitia_badge_bckt = match self.nft_badge_vault.get_mut(&pitia_bdg_addr) {
                    Some(v) => v.take(Decimal::one()),
                    None => std::process::abort()                  
            }; 
            let pitia_bdg_ref = pitia_badge_bckt.create_proof();
        
            // Call an external Oracle and retrieve a seed
            let arg = args![self.tkn.series_component_addr, mint_code_id, url, pitia_bdg_ref];
            let v = borrow_component!(pitia_addr).call::<Vec<(u128,String)>>(&pitia_method.to_string(),arg);
            let (asset_seed,out_url) = v.get(0).unwrap();
            // Put Nft Badge back in Vault
            match self.nft_badge_vault.get_mut(&pitia_bdg_addr) {
                    Some(v) => v.put(pitia_badge_bckt),
                    None => std::process::abort()                  
            };     

            // Retrieve Nft Badges for External Data Component authorization purpose
            let nft_bdg_addr = self.nft_data.get(&ext_addr).unwrap().clone();
            let nft_badge_bckt = match self.nft_badge_vault.get_mut(&nft_bdg_addr) {
                    Some(v) => v.take(Decimal::one()),
                    None => std::process::abort()                  
            }; 

            let nft_bdg_ref_a = nft_badge_bckt.create_proof();
            let nft_bdg_ref_b = nft_badge_bckt.create_proof();
            let nft_bdg_ref_c = nft_badge_bckt.create_proof();
            let nft_bdg_ref_d = nft_badge_bckt.create_proof();

            info!("asset_seed: {}",asset_seed.to_string().clone());

            // Call an external Data Component and retrieve fresh Nft data to mint it 
            let args_a = args![asset_seed.to_string(), nft_bdg_ref_a];
            let args_b = args![asset_seed.to_string(), nft_bdg_ref_b];
            let args_c = args![asset_seed.to_string(), nft_bdg_ref_c];
            let args_d = args![asset_seed.to_string(), nft_bdg_ref_d];

            let (str_1,svg_1) = borrow_component!(ext_addr).call::<(String,String)>(&method_x.to_string(), args_a);
            let (str_2,svg_2) = borrow_component!(ext_addr).call::<(String,String)>(&method_y.to_string(), args_b);
            let (str_3,svg_3) = borrow_component!(ext_addr).call::<(String,String)>(&method_z.to_string(), args_c);
            let (str_4,svg_4) = borrow_component!(ext_addr).call::<(String,String)>(&method_w.to_string(), args_d);
            let tab = "\" \n \"".to_string();
            let svg = tab + &svg_1 + &svg_2 + &svg_3 + &svg_4; 
            let linked_assets: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();  

            let new_nft = AssetNFT {
                uri: out_url.to_owned() + &svg,
                data_1: str_1,
                data_2: str_2,
                data_3: str_3,
                data_4: str_4,
                value_1: 0,
                value_2: 0,
                value_3: asset_surface,
                linked_assets: linked_assets
            };

            let x = new_nft.data_1.clone();
            let y = new_nft.data_2.clone();
            let z = new_nft.data_3.clone();
            let w = new_nft.data_4.clone();
            
            let nft_bucket : Bucket;

            // Verify NFT to mint is a regular or a merged one.
            // Verify Nft hasn't already been minted checking relative storing Hashmaps
            // and increment counters in case of negative response.
            if !self.merge_nft_flag {
                match self.minted_nft.get(&(x.clone(),y.clone(),z.clone(),w.clone())) { 
                    Some(_address) => { 
                        flag = false;
                        info!(" NFT already minted. Randomize again! ");    
                        nft_bucket = self.mint_nft(new_nft, x, y, z, w, flag, buy_flag, true);
                    }
                    _ => nft_bucket = self.mint_nft(new_nft, x, y, z, w, flag, buy_flag, true),
                }    
            } else {
                match self.minted_merged_nft.get(&(x.clone(),y.clone(),z.clone(),w.clone())) { 
                    Some(_address) => { 
                        flag = false;
                        info!(" NFT already minted. Randomize again! ");    
                        nft_bucket = self.mint_nft(new_nft, x, y, z, w, flag, buy_flag, false);
                    }
                    _ => nft_bucket = self.mint_nft(new_nft, x, y, z, w, flag, buy_flag, false),
                }    
            }

            // Put Nft Badges back in Vault
            match self.nft_badge_vault.get_mut(&nft_bdg_addr) {
                Some(v) => v.put(nft_badge_bckt),
                None => std::process::abort()                  
            };            

            nft_bucket
        }

            // Nft minter Fn
        fn mint_nft(
            &mut self, 
            new_nft: AssetNFT, 
            x: String, 
            y: String, 
            z: String, 
            w: String, 
            flag: bool,
            buy_flag: bool,
            mint_flag: bool 
        ) -> Bucket {
            let nft_id = NonFungibleId::random();
            let nft_bucket = self.asset_nft_minter_badge.authorize(|| { 
                borrow_resource_manager!(self.asset_nft_resource_def)
                    .mint_non_fungible(&nft_id.clone(), new_nft)
            });  

            if flag {
                if mint_flag {
                    info!(" NFT ID: {} ", nft_id);
                    info!(" NFT resource address: {} ", self.asset_nft_resource_def);
                }
                info!(" Mint: {}, {}, {}, {} ", x, y, z, w);
                if buy_flag {
                    if self.merge_nft_flag != true {
                        self.minted_nft.insert((x,y,z,w),self.tkn.series_component_addr);
                        self.asset_nft_id_counter += 1;
                    } else {
                        self.minted_merged_nft.insert((x,y,z,w),self.tkn.series_component_addr);
                        self.asset_merged_nft_id_counter += 1;
                    }
                }
                self.counter += 1;
            } 

            nft_bucket          
        }
    }
}

