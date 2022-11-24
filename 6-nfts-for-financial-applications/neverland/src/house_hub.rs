use scrypto::prelude::*;
use crate::hub_data::*;

blueprint! {
    struct HouseHub {
        // Vault to stock NFT Asset Farm Caller Badge used to perfom external Component calls
        nft_farm_badge_vault: HashMap<ComponentAddress,(Vault,Vault)>,
        // NFT Asset Farm component address 
        nft_farm_comp: ComponentAddress,
        // NFT Data Oracle component address for merge Land Asset NFT method purpose
        pitia_addr: ComponentAddress,        
        // NFT Data component address for merge Land Asset NFT method purpose
        nft_data_addr: ComponentAddress,
        // Map with external component address and relative badge resource address allowed to call 
        // methods within protocol
        caller_map: HashMap<ComponentAddress,ResourceAddress>,
        // Contribution vault to store contributions, fees, gains 
        contribution_vault: Vault,
        // Vault to stock SBT Updater Badge
        sbt_updater_badge: Vault,
        // SBT Updater Badge resource address
        sbt_updater_badge_addr: ResourceAddress,
        // Map with SBT resource address & ID, architect badge resource address, ID and degree data
        arch_badge_map: HashMap<(ResourceAddress,NonFungibleId),(ResourceAddress,NonFungibleId,DegreeNFT)>,
        // Map with Architect Badge resource address & ID, house project NFT resource address, ID and
        // Asset NFT data 
        house_project_map: HashMap<(ResourceAddress,NonFungibleId),Vec<(ResourceAddress,NonFungibleId,AssetNFT)>>,
        // List of house project realized by architects within protocol. 
        house_project_listing: Vec<(ResourceAddress,NonFungibleId,Decimal)>,
        // House project payment vault with project resource address and id as key value.
        project_payment_vault: HashMap<(ResourceAddress,NonFungibleId),Vault>,
        // Building Contract NFT vault with building contract resource address and id as key value.
        building_contract_vault: HashMap<(ResourceAddress,NonFungibleId),Vault>,
        // Vector of vaults with building contract payments, land NFT vaults & project NFT vaults &  
        // building property NFT, able to handle different instances within same Building Contract.
        deposit_vaults: HashMap<(ResourceAddress,NonFungibleId),Vec<(Vault,Vault,Vault,Vault)>>,
        // List of build calls
        build_call_listing: Vec<(ResourceAddress,NonFungibleId,Decimal)>,
        // Build Call Vector with Building Contract resource address & id, contract data, amount & 
        // duration, penalty contructor flag, paymet penalty client flag. 
        build_call_vec: Vec<(ResourceAddress,NonFungibleId,BuildingContract,Decimal,u64,bool,bool)>,
        // Map with merge NFT mint data ID code to merge two different Land Asset NFT into a single NFT 
        merge_data: HashMap<String,String>,
        // Minter Badge vault
        minter_badge: Vault,
        // Protocol Owner Badge resource address
        owner_badge: ResourceAddress,
        // Architect Badge resource address
        arch_badge: ResourceAddress,
        // House Project NFT resource address 
        house_project: ResourceAddress,
        // Building Contract NFT resource address
        building_contract: ResourceAddress,
        // Building Property NFT resource address
        building_property: ResourceAddress,
        // User SBT Resource Address.
        user_sbt: ResourceAddress,
        // Buildable Land NFT Resource Address.
        land_nft: ResourceAddress,
        // Claimed contribution amount.
        contribution_claimed: Decimal,
        // Currency accepted by Protocol.
        currency: ResourceAddress,
        // Building Contract payment deadline
        payment_deadline: u64
    }
   
    impl HouseHub {
        pub fn new(
            house_hub_name: String,                     // Protocol name instance printed on minted Badge & NFTs                     
            sbt_updater_badge_addr: ResourceAddress,    // Land Data protocol's SBT updater Badge resource address
            land_data_owner_badge: ResourceAddress,     // Land Data protocol's Owner Badge resource address
            user_sbt: ResourceAddress,                  // Land Data protocol's registered users SBT resource address
            land_nft: ResourceAddress,                  // Land Asset NFT resource address
            currency: ResourceAddress,                  // Protocol accepted currency resource address
            nft_farm_comp: ComponentAddress,            // Land Asset Farm component address
            pitia_addr: ComponentAddress,               // Pitia NFT data oracle component address
            nft_data_addr: ComponentAddress,            // Neverland Merge Nft Data component address
            payment_deadline: u64                       // Building Project payment deadline 
        ) -> (ComponentAddress,Bucket) {
            let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", house_hub_name.clone() + " MinterBadge ")
                .initial_supply(Decimal::one());

            let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", house_hub_name.clone() + " OwnerBadge ")
                .initial_supply(Decimal::one());

            let arch_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", house_hub_name.clone() + " ArchBadge ")        
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minter_badge.resource_address())), LOCKED)                
                .no_initial_supply();

            let house_project = ResourceBuilder::new_non_fungible()
                .metadata("name", house_hub_name.clone() + " House Project ")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)                                   
                .burnable(rule!(require(minter_badge.resource_address())), LOCKED)                       
                .updateable_non_fungible_data(rule!(require(arch_badge)), LOCKED)
                .no_initial_supply();

            let building_contract = ResourceBuilder::new_non_fungible()
                .metadata("name", house_hub_name.clone() + " Building Contract ")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minter_badge.resource_address())), LOCKED)  
                .updateable_non_fungible_data(rule!(require(minter_badge.resource_address())), LOCKED)              
                .no_initial_supply();

            let building_property = ResourceBuilder::new_non_fungible()
                .metadata("name", house_hub_name + " Building Property ")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minter_badge.resource_address())), LOCKED)  
                .updateable_non_fungible_data(rule!(require(minter_badge.resource_address())), LOCKED)              
                .no_initial_supply();

            let access_rules = AccessRules::new()   
                .method("stock_sbt_updater_badge", rule!(require(land_data_owner_badge)))
                .method("stock_nft_farm_badge", rule!(require(owner_badge.resource_address())))                
                .method("mint_caller_badge", rule!(require(owner_badge.resource_address())))   
                .method("claim_contribution", rule!(require(owner_badge.resource_address())))
                .method("insert_merge_data", rule!(require(owner_badge.resource_address())))      
                .default(rule!(allow_all)); 

            let mut house_hub: HouseHubComponent = Self {
                nft_farm_badge_vault: HashMap::new(),
                nft_farm_comp,
                pitia_addr,  
                nft_data_addr,
                caller_map: HashMap::new(),
                contribution_vault: Vault::new(currency.clone()),
                sbt_updater_badge: Vault::new(sbt_updater_badge_addr),
                sbt_updater_badge_addr,
                arch_badge_map: HashMap::new(),
                house_project_map: HashMap::new(),
                house_project_listing: Vec::new(),
                project_payment_vault: HashMap::new(),
                building_contract_vault: HashMap::new(),
                deposit_vaults: HashMap::new(),
                build_call_listing: Vec::new(),
                build_call_vec: Vec::new(),
                merge_data: HashMap::new(),
                minter_badge: Vault::with_bucket(minter_badge),
                owner_badge: owner_badge.resource_address(),
                arch_badge,
                house_project,
                building_contract,
                building_property,
                user_sbt,
                land_nft,
                contribution_claimed: Decimal::zero(),
                currency,
                payment_deadline
            }
            .instantiate();
            house_hub.add_access_check(access_rules);

            (house_hub.globalize(),owner_badge)
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

        // Stock one AssetFarm Ugrade NFT Badge and one AssetFarm Merge NFT Badge as well as relevant  
        // components adresses allowing protocol to perform external component calls when 
        // specifically required by his methods
        pub fn stock_nft_farm_badge(
            &mut self, 
            pitia_addr: ComponentAddress,           // Pitia NFT data oracle component address
            nft_data_addr: ComponentAddress,        // Neverland Merge Nft Data component address
            nft_farm_comp: ComponentAddress,        // Asset Farm component address
            ugrade_nft_farm_badge: Bucket,          // Ugrade NFT Asset Farm Badge 
            merge_nft_farm_badge: Bucket            // Merge NFT Asset Farm Badge 
        ) {
            self.nft_farm_comp = nft_farm_comp;
            self.pitia_addr = pitia_addr;        
            self.nft_data_addr = nft_data_addr;                                                     
            
            let (upgrade_vault,merge_vault) = self.nft_farm_badge_vault
                .entry(self.nft_farm_comp)
                .or_insert((
                    Vault::new(ugrade_nft_farm_badge.resource_address()),
                    Vault::new(merge_nft_farm_badge.resource_address())
                ));                                                                                 

            upgrade_vault.put(ugrade_nft_farm_badge);
            merge_vault.put(merge_nft_farm_badge);
        }

        // Mint a Caller Badge to allow called from an external Component to call methods
        pub fn mint_caller_badge(&mut self, cmp_addr: ComponentAddress) -> Bucket {
            let caller_badge = self.build_badge(" ProAcademy_Caller_Badge".to_string());
            self.caller_map.insert(cmp_addr,caller_badge);
            info!(" Caller Component address added: {} ", cmp_addr);
                
            self.minter_badge
                .authorize(|| { borrow_resource_manager!(caller_badge).mint(Decimal::one()) })
        }

        // Mint a Architect Badge to allow an external user to call methods
        pub fn mint_arch_badge(&mut self, arch_sbt: Proof) -> Bucket { 
            // check architect SBT proof
            let arch_sbt: ValidatedProof = arch_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[mint_arch_badge]: Invalid proof provided!"); 

            let arch_sbt_id = arch_sbt.non_fungible::<UserSBT>().id(); 
            let data_sbt: UserSBT = arch_sbt.non_fungible().data();

            let key = NonFungibleId::random(); 
            let mut data_vec = DegreeZero::new();
            let mut data = data_vec.degree_zero.pop().unwrap();

            let title_one = "Master Architecture".to_string();
            let title_two = "Bachelor Architecture".to_string();

            let mut found = false;
            for tuple in data_sbt.educational_degrees.clone() {                                   
                for title in &tuple.2.degree_name {
                    if *title == title_one || *title == title_two {
                        data.uri = tuple.2.uri;
                        data.pro_academy_address = tuple.2.pro_academy_address;
                        data.user_sbt_address = tuple.2.user_sbt_address;
                        data.user_sbt_id = tuple.2.user_sbt_id;
                        data.user_name = tuple.2.user_name;
                        data.degree_name = tuple.2.degree_name;
                        data.mint_date =  tuple.2.mint_date;
                        data.teaching_subject = tuple.2.teaching_subject;
                        data.grade_point_avg = tuple.2.grade_point_avg;
                        data.cum_laude = tuple.2.cum_laude;
                        found = true;
                        break;
                    }
                }
            }                                                                                     
            assert!(found,"[mint_arch_badge]: Invalid Degree detected!");

            match self.arch_badge_map.get(&(arch_sbt.resource_address(),arch_sbt_id.clone())) {
                Some(_tuple) => {
                    info!("[mint_arch_badge]: Architect already registered! ");
                    std::process::abort()
                }
                None => {
                    self.arch_badge_map
                        .insert(
                            (arch_sbt.resource_address(),arch_sbt_id.clone()),
                            (self.arch_badge,key.clone(),data.clone())
                        );
                    info!(" Architect Badge address: {} ", self.arch_badge);
                    info!(" Architect Badge id: {} ", key.clone());
                }
            }
         
            self.minter_badge.authorize(|| { 
                borrow_resource_manager!(self.arch_badge).mint_non_fungible(&key,data)
            }) 
        }

        // Insert NFT data ID codes to retrieve within "merge_properties" method and identify
        // merge NFT data to mint by external NFT Asset Farm.
        // Only protocol owner can succesfully call.   
        pub fn insert_merge_data(&mut self, merge_data: Vec<(String,String)>) {
            for data in merge_data {
                self.merge_data.insert(data.0,data.1);
            }
        }

        // Claim accrued contribution function whom only protocol owner can succesfully call.
        pub fn claim_contribution(&mut self, amount: Decimal) -> Bucket {       
            let bckt_output: Bucket = self.contribution_vault.take(amount);
            self.contribution_claimed += bckt_output.amount();
            info!(" HouseHub accrued contribution claimed {} ", bckt_output.amount());
            info!(
                " HouseHub total accrued contribution claimed {} ${} ", 
                self.contribution_claimed, 
                self.currency
            );

            bckt_output
        }

        // Method callable by a registered architect, once provided his Arch Badge, to submit a new
        // house prject within protocol.
        pub fn submit_house_project(
            &mut self, 
            url: String,                                    // URL pointing at house project data
            svg_data: Vec<String>,                          // svg data vector. 
            data_1: String,                                 // AssetNFT first data string field 
            data_2: String,                                 // AssetNFT second data string field 
            data_3: String,                                 // AssetNFT third data string field
            data_4: String,                                 // AssetNFT forth data string field
            value_2: u8,                                    // Building square meters surface
            value_3: u8,                                    // Building energetic class
            price: Decimal,                                 // house project price
            arch_badge: Proof                               // Arch Badge proof
        ) {
            // check architect SBT proof
            let arch_badge: ValidatedProof = arch_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.arch_badge,
                    dec!("1"),
                ))
                .expect("[mint_house_project]: Invalid proof provided");
            let arch_badge_id = arch_badge.non_fungible::<DegreeNFT>().id();

            // svg data assembler loop
            let mut svg = "\" \n \"".to_string(); 
            let mut i = 0;
            loop {
                svg += &svg_data[i];
                if i < svg_data.len() {
                    break;
                }
                i += 1;
            }
            let title_string = "House Building Project ".to_string();
            let key = NonFungibleId::random();
            let linked_assets_vec: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();

            // populate AssetNFT structure with relative data
            let project_data = AssetNFT {
                uri: url.to_owned() + &svg,
                data_1: title_string.to_owned() + &data_1,
                data_2: data_2,
                data_3: data_3,
                data_4: data_4,
                value_1: 0,
                value_2: value_2,
                value_3: value_3,
                linked_assets: linked_assets_vec
            };

            let mut v: Vec<(ResourceAddress,NonFungibleId,AssetNFT)> = Vec::new();
            v.push((self.house_project,key.clone(),project_data.clone()));
            self.house_project_map.insert((self.arch_badge,arch_badge_id.clone()),v);

            // assign a vault to collect house project future sale accrued amount  
            if !self.project_payment_vault
                .contains_key(&(arch_badge.resource_address(),arch_badge_id.clone())) {
                    self.project_payment_vault.entry((arch_badge.resource_address(),arch_badge_id))
                        .or_insert(Vault::new(self.currency));
            }

            // add provided house project to related listing map
            self.house_project_listing.push((self.house_project,key,price));  
        }

        // Retrieve list of all house projects submitted by architects within protocol. 
        pub fn house_project_list(&mut self) {
            for (key,value) in self.house_project_map.iter() {
                info!(" =========================================================================");
                info!(" Architect Badge resource address: {} ", key.0);
                info!(" Architect Badge id: {} ", key.1);
                for tuple in value {
                    info!(" =====================================================================");
                    info!(" House Project NFT resource address: {} ", tuple.0);
                    info!(" House Project NFT id: {} ", tuple.1.clone());
                    info!(" House Project URI: {:?} ", tuple.2.uri);
                    info!(" Building description: {} ", tuple.2.data_1);
                    info!(" Levels description: {} ", tuple.2.data_2);
                    info!(" Rooms description: {} ", tuple.2.data_3);
                    info!(" Installations description: {} ", tuple.2.data_4);
                    info!(" Number of realized buildings: {} ", tuple.2.value_1);
                    info!(" Building square meters surface: {} ", tuple.2.value_2);
                    info!(" Building energetic class: {} ", tuple.2.value_3);
                    for tup in &self.house_project_listing {
                        if tuple.0 == tup.0 && tuple.1 == tup.1 {
                            let name = borrow_resource_manager!(self.currency)
                                .metadata()["name"].clone();
                            let symbol = borrow_resource_manager!(self.currency)
                                .metadata()["symbol"].clone();
                            info!(" House Project price: {} ", tup.2);
                            info!(" Name: {} Symbol: {} ", name, symbol);
                        }
                    }
                }
            }   
        }

        // Method callable by a Land AssetNFT owner, authenticated by provided SBT proof,
        // wishing to purchase a house project made & listed by an architect on protocol,
        // aiming to use it as reference to build a house within his land property.
        // Method mint a house project Asset NFT and return it in a payment exchange. 
        pub fn buy_house_project(
            &mut self,
            house_project_addr: ResourceAddress,            // house project resource address
            house_project_id: NonFungibleId,                // house project ID
            mut payment: Bucket,                            // payment
            land_owner_sbt: Proof                           // land owner SBT proof
        ) -> (Bucket,Bucket) {
            // check land owner SBT proof
            let land_owner_sbt: ValidatedProof = land_owner_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[buy_house_project]: Invalid proof provided!");        
            
            let land_owner_data_sbt: UserSBT = land_owner_sbt.non_fungible().data();
            assert!(
                !land_owner_data_sbt.real_estate_properties.is_empty(),
                "[buy_house_project]: Real Estate Property unfound!"
            );

            // verify house project request is acceptable
            let mut price = dec!("0");
            let mut found = false;
            for tuple in &self.house_project_listing {
                if tuple.0 == house_project_addr && tuple.1 == house_project_id.clone() {
                    price = tuple.2;
                    found = true;
                    break;
                }
            }
            assert!(found,"[buy_house_project]: House project unfound!");
            assert!(payment.amount() >= price,"[buy_house_project]: Payment amount too short!");

            // retrieve requested house project data
            found = false;
            let mut data_vec = AssetZero::new();
            let (mut arch_addr,mut arch_id) = (house_project_addr,house_project_id.clone());
            let mut project_data = data_vec.asset_zero.pop().unwrap();
            for (key,value) in self.house_project_map.iter() {
                for tuple in value {
                    if tuple.0 == house_project_addr && tuple.1 == house_project_id.clone() {
                        project_data = tuple.2.clone();
                        arch_addr = key.0;
                        arch_id = key.1.clone();
                        found = true;
                        break;
                    }
                }
            }
            assert!(found,"[buy_house_project]: House project data unfound!");

            // collect house project payment
            match self.project_payment_vault.get_mut(&(arch_addr,arch_id)) {
                Some(vault) => vault.put(payment.take(price)),
                None => {
                    info!("[buy_house_project]: Payment vault unfound! ");
                    std::process::abort()
                }
            }

            info!(" House Project NFT address: {} ", self.house_project);
            info!(" House Project NFT id: {} ", house_project_id.clone());
         
            // mint a house project AssetNFT and return it  
            (
              self.minter_badge.authorize(|| { 
                    borrow_resource_manager!(self.house_project)
                        .mint_non_fungible(&house_project_id,project_data)
              }),
                payment
            )
        }

        // Method callable by architect, authorized through Arch Badge identification, wishing to
        // know sum of fpayments accrued by house project's selling activity. 
        pub fn ask_accrued_amount(&mut self, arch_badge: Proof) -> Decimal {
            // check architect Badge proof
            let arch_badge: ValidatedProof = arch_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.arch_badge,
                    dec!("1"),
                ))
                .expect("[ask_accrued_amount]: Invalid proof provided");
            let arch_id = arch_badge.non_fungible::<DegreeNFT>().id();

            // check payment amount collected within vault
            let amount: Decimal;
            match self.project_payment_vault.get_mut(&(arch_badge.resource_address(),arch_id)) {
                Some(vault) => amount = vault.amount(),
                None => {
                    info!("[ask_accrued_amount]: Vault unfound! ");
                    std::process::abort()
                }
            }

            amount
        }

        // Method callable by architect, authorized through Arch Badge identification, wishing to
        // withdrawal payments accrued by house project's selling, He needs to specify the 
        // withdrawal amount also.  
        pub fn collect_project_payment(&mut self, amount: Decimal, arch_badge: Proof) -> Bucket {
            // check architect Badge proof
            let arch_badge: ValidatedProof = arch_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.arch_badge,
                    dec!("1"),
                ))
                .expect("[ask_accrued_amount]: Invalid proof provided");

            // collect requested payment amount from vault
            let arch_id = arch_badge.non_fungible::<DegreeNFT>().id();
            match self.project_payment_vault.get_mut(&(arch_badge.resource_address(),arch_id)) {
                Some(vault) => vault.take(amount),
                None => {
                    info!(" Payment vault unfound! ");
                    std::process::abort()
                }
            }
        }

        // Build a call to find a General Contractor wishing to build up the house in land owner 
        // property following architect's house project. Client needs to specify contract amount,
        // duration, contract's URL pointer, deposit an amount in protocol's currency, deposit 
        // House Project NFT as well as Land Property AssetNFT and authenticate himself through SBT 
        //proof to testify land property ownership's correspondence. 
        pub fn build_call(
            &mut self,
            contract_amount: Decimal,                       // contract value
            duration: u64,                                  // contract duration
            url: String,                                    // contract's URL pointer
            mut deposit: Bucket,                            // client's deposit Bucket 
            house_project: Bucket,                          // house project AssetNFT
            land_asset: Bucket,                             // land property AssetNFT
            land_owner_sbt: Proof                           // land owner SBT proof
        ) -> Bucket {
            // check land owner SBT proof
            let land_owner_sbt: ValidatedProof = land_owner_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[build_call]: Invalid proof provided!");  
            let land_owner_sbt_id = land_owner_sbt.non_fungible::<UserSBT>().id();     

            assert!(
                land_asset.resource_address() == self.land_nft,
                "[build_call]: Land Asset unacceptable!"
            );

            // check provided house project AssetNFT
            let mut building_surface = 0;
            let mut found = false;
            let house_project_id = house_project.non_fungible::<AssetNFT>().id();
            assert_eq!(
                house_project.resource_address(),
                self.house_project,
                "[build_call]: House Project unacceptable!"
            );

            for (_key,value) in self.house_project_map.iter() {
                for tup in value {
                    if tup.1 == house_project_id.clone() { 
                        building_surface = tup.2.value_2;
                        found = true; 
                        break;
                    }
                }
            }
            assert!(found,"[build_call]: House Project correspondence undetected!");

            // check provided land property AssetNFT
            let land_owner_data_sbt: UserSBT = land_owner_sbt.non_fungible().data();
            let land_id = land_asset.non_fungible::<AssetNFT>().id();
            found = false;
            for assets in land_owner_data_sbt.real_estate_properties {
                if land_asset.resource_address() == assets.0 && land_id.clone() == assets.1 {
                    if assets.2.value_2 == 0 || building_surface+assets.2.value_3 < assets.2.value_2/2 {
                        found = true;
                    } else {
                        assert!(found,"[build_call]: Insufficient building surface available!");
                    }
                    break;
                }
            }
            assert!(found,"[build_call]: Land Asset correspondence undetected!");
            assert!(
                deposit.amount() >= contract_amount*dec!("10")/dec!("100"),
                "[build_call]: Insufficient deposit amount!"
            );

            let key = NonFungibleId::random();

            // populate BuildingContract structure with relative data
            let data = BuildingContract {
                url: url.to_string(),
                house_hub_address: Runtime::actor().as_component().0,
                land_owner_sbt_address: land_owner_sbt.resource_address(),
                land_owner_sbt_id: land_owner_sbt_id.clone(),
                contractor_sbt_address: ResourceAddress::from(RADIX_TOKEN),
                contractor_sbt_id: NonFungibleId::from_u64(0),
                land_property_nft: land_asset.resource_address(),
                land_property_nft_id: land_id.clone(),
                house_project_nft: house_project.resource_address(),
                house_project_nft_id: house_project_id.clone(),
                property_building_nft: ResourceAddress::from(RADIX_TOKEN),
                property_building_nft_id: NonFungibleId::from_u64(0),
                building_surface: building_surface,
                contract_amount: contract_amount,
                deadline: 0,
                executed: false,
                approved: false
            };

            // assign vaults to store provided NFTs and future payment and building property NFT.
            let vec_vaults: Vec<(Vault,Vault,Vault,Vault)> = Vec::new();
            let mut amount_vault = Vault::new(self.currency); 
            let mut land_vault = Vault::new(self.land_nft);
            let mut project_vault = Vault::new(self.house_project);
            let property_vault =  Vault::new(self.building_property);
            
            let deposit_vaults_vec = self.deposit_vaults
                .entry((self.building_contract,key.clone()))
                .or_insert(vec_vaults);

            // put provided NFTs in related vaults
            amount_vault.put(deposit.take(contract_amount*dec!("10")/dec!("100")));
            land_vault.put(land_asset);
            project_vault.put(house_project);
            deposit_vaults_vec.push((amount_vault,land_vault,project_vault,property_vault));
            
            self.build_call_vec
                .push((self.building_contract,key.clone(),data,contract_amount,duration,false,false));
            self.build_call_listing.push((self.building_contract,key.clone(),contract_amount));

            deposit
        }

        // Method callable by a General Contractor wishing to build up the house project.
        // He needs to: specify which build call he's subscribing, provide a bond in protocol 
        // currency, provide his study Degree data within SBT proof 
        pub fn subscribe_build_call(
            &mut self,
            contract_address: ResourceAddress,              // building contract resource address
            contract_id: NonFungibleId,                     // building contract ID
            mut bond: Bucket,                               // bond Bucket
            contractor_sbt: Proof                           // general contractor SBT proof
        ) -> Bucket {
            // check general contractor SBT proof
            let contractor_sbt: ValidatedProof = contractor_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[subscribe_build_call]: Invalid proof provided!");
            let contractor_sbt_id = contractor_sbt.non_fungible::<UserSBT>().id();

            assert!(
                bond.resource_address() == self.currency,
                "[subscribe_build_call]: Wrong currency provided!"
            );

            // Verify professional qualification through Degree title check.
            let data_sbt: UserSBT = contractor_sbt.non_fungible().data();
            let title_one = "General Construction".to_string();

            let mut found = false;
            for tuple in data_sbt.educational_degrees.clone() {                                   
                for title in tuple.2.degree_name {
                    if title == title_one {
                        found = true;
                        break;
                    }
                }
            }                                                                                     
            assert!(found,"[subscribe_build_call]: Invalid Degree detected!");

            // retrieve and update building contract data 
            found = false;
            for value in self.build_call_vec.iter_mut() {
                if value.0 == contract_address && value.1 == contract_id {
                    assert!(bond.amount() >= value.3*3/100,
                    "[subscribe_build_call]: Bond amount less then 3%!"
                    );
                    value.2.contractor_sbt_address = contractor_sbt.resource_address();
                    value.2.contractor_sbt_id = contractor_sbt_id;
                    value.2.deadline = Runtime::current_epoch() + value.4;
                    value.2.property_building_nft = self.building_property;
                    value.2.property_building_nft_id = NonFungibleId::random();

                    // mint Building Contract NFT
                    let building_contract = self.minter_badge.authorize(|| { 
                        borrow_resource_manager!(self.building_contract)
                            .mint_non_fungible(&value.1.clone(),value.2.clone())
                    });

                    // put Building Contract NFT in vault
                    let building_contract_vault = 
                        self.building_contract_vault.entry((value.0,value.1.clone()))
                            .or_insert(Vault::new(self.building_contract));
                    building_contract_vault.put(building_contract);

                    // collect bond 
                    match self.deposit_vaults.get_mut(&(value.0,value.1.clone())) {
                        Some(vaults_vec) => {
                            for vault in vaults_vec {
                                vault.0.put(bond.take(value.3*3/100));
                            }
                        }
                        None => {
                            info!("[subscribe_build_call]: Deposit vault unfound!");
                            std::process::abort()
                        }
                    }
                   
                    found = true;  
                    break;
                }
            }
            assert!(found,"[subscribe_build_call]: Contract correspondence undetected!");

            bond
        }

        // Method callable by General Contractor, identificated by SBT, once built the house.
        // He needs to specify contract resource address & ID as well as an URL index point and/or   
        // some svg data contract's inherent.
        pub fn contractor_delivery(
            &mut self,
            contract_address: ResourceAddress,              // building contract resource address
            contract_id: NonFungibleId,                     // building contract ID
            url: String,                                    // contract pointer URL
            svg_data: Vec<String>,                          // svg data contract's inherent
            contractor_sbt: Proof                           // general contractor SBT proof
        ) {
            // check general contractor SBT proof
            let contractor_sbt: ValidatedProof = contractor_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[contractor_delivery]: Invalid proof provided!");
            let contractor_sbt_id = contractor_sbt.non_fungible::<UserSBT>().id();

            // svg data assembler loop
            let mut svg = "\" \n \"".to_string();
            let mut i = 0;
            loop {
                svg += &svg_data[i];
                if i < svg_data.len() {
                    break;
                }
                i += 1;
            }

            // check Building Contract data 
            let house_project_id: NonFungibleId;
            let mut found = false;
            for value in self.build_call_vec.iter_mut() {
                if value.0 == contract_address && value.1 == contract_id.clone() {
                    assert_eq!(
                        value.2.contractor_sbt_address == contractor_sbt.resource_address(),
                        value.2.contractor_sbt_id == contractor_sbt_id.clone(),
                        "[contractor_delivery]: Contractor correspondence unfound!"
                    );
                    assert!(
                        !value.2.executed,
                        "[contractor_delivery]: Building contract already delivered!"
                    );

                    // set Building Contract NFT related data value as executed 
                    match self.building_contract_vault.get_mut(&(value.0,value.1.clone())) {
                        Some(vault) => {
                            let contract = vault.take_non_fungible(&value.1.clone());
                            let mut data: BuildingContract = contract.non_fungible().data();
                            assert_eq!(
                                value.2.executed,
                                data.executed,
                                "[contractor_delivery]: Contract data mismatch!"
                            );
                            house_project_id = value.2.house_project_nft_id.clone();
                            value.2.executed = true;
                            data.executed = true;

                            // update Building Contract NFT data
                            self.minter_badge.authorize(|| {
                                borrow_resource_manager!(self.building_contract)
                                    .update_non_fungible_data(&value.1, data)
                            });

                            vault.put(contract);
                        }
                        None => {
                            info!("[contractor_delivery]: Building contract vault unfound!");
                            std::process::abort()
                        }
                    }

                    // Mint a Real Estate Building Property Ownership Certificate NFT. Data is mostly 
                    // transferred from House Project NFT.
                    match self.deposit_vaults.get_mut(&(contract_address,contract_id)) {         
                        Some(vaults_vec) => {
                            let project_nft = vaults_vec[0].2.take_non_fungible(&house_project_id.clone());
                            let mut project_data: AssetNFT = project_nft.non_fungible().data();
                            project_data.value_1 = 1;

                            let mut linked_assets: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();
                            linked_assets.push((project_nft.resource_address(),house_project_id));

                            let title_string = "Real Estate: Bulding Property Certificate".to_string();

                            // populate AssetNFT structure with relative data
                            let building_data = AssetNFT {
                                uri: url.to_owned() + &svg + &project_data.uri.clone(),
                                data_1: title_string.to_owned(),
                                data_2: project_data.data_2.clone(),
                                data_3: project_data.data_3.clone(),
                                data_4: project_data.data_4.clone(),
                                value_1: project_data.value_1.clone(),
                                value_2: project_data.value_2.clone(),
                                value_3: project_data.value_3.clone(),
                                linked_assets: linked_assets
                            };

                            // mint a Building Property AssetNFT
                            let building_nft = self.minter_badge.authorize(|| { 
                                borrow_resource_manager!(self.building_property)
                                    .mint_non_fungible(
                                        &value.2.property_building_nft_id.clone(),
                                        building_data
                                    )
                            });

                            // put NFTs in vaults
                            vaults_vec[0].2.put(project_nft);
                            vaults_vec[0].3.put(building_nft);

                            info!(
                                " Building Property NFT address: {} ", 
                                value.2.property_building_nft
                            );
                            info!(
                                " Building Property NFT id: {} ", 
                                value.2.property_building_nft_id
                            );

                            }
                        None => {
                            info!("[collect_contract_payment]: Deposit vault unfound!");
                            std::process::abort()
                        }
                    }                                                                           

                    // check if delivery date has been overtaked
                    if Runtime::current_epoch() > value.2.deadline {
                        value.5 = true;
                        info!(" Delivery date overtaken! Penalty applied.");
                    }
                    found = true;
                    break;
                }
            }
            assert!(found,"[contractor_delivery]: Contract correspondence undetected!");
        }

        // Method callable to ckeck data on BuildingContract NFT once provided relative contract
        // address and relative contract ID
        pub fn inspect_building_contract(
            &mut self,
            contract_address: ResourceAddress,                 // building contract resource address
            contract_id: NonFungibleId                         // building contract ID
        ) {
            match self.building_contract_vault.get_mut(&(contract_address,contract_id.clone())) {
                Some(vault) => {
                    let contract = vault.take_non_fungible(&contract_id.clone());
                    let data: BuildingContract = contract.non_fungible().data();
                    info!("url: {} ",data.url);
                    info!("house hub address : {} ",data.house_hub_address);
                    info!("land owner sbt_address : {} ",data.land_owner_sbt_address);
                    info!("land owner sbt id : {} ",data.land_owner_sbt_id);
                    info!("contractor sbt address : {} ",data.contractor_sbt_address);
                    info!("contractor sbt id : {} ",data.contractor_sbt_id);
                    info!("land property nft: {} ",data.land_property_nft);
                    info!("land property nft_id: {} ",data.land_property_nft_id);
                    info!("house project nft: {} ",data.house_project_nft);
                    info!("house project nft id: {} ",data.house_project_nft_id);
                    info!("property building nft: {} ",data.property_building_nft);
                    info!("property building nft_id: {} ",data.property_building_nft_id);
                    info!("building surface: {} ",data.building_surface);
                    info!("contract amount: {} ",data.contract_amount);
                    info!("deadline: {} ",data.deadline);
                    info!("executed: {} ",data.executed);
                    info!("approved: {} ",data.approved);
                    vault.put(contract);
                }
                None => info!(" Building Contract unfound ")
            }
        }

        // Method callable by client to approve building contract. After passed required security data 
        // checking, contract payment is collected, contract data modify and contract set to "approved"
        // state. Land Property NFT and Building Property NFT are returned to client once his SBT data
        // hasbeen updated to include the new property built.  
        pub fn approve_contract(
            &mut self,
            contract_address: ResourceAddress,              // building contract resource address
            contract_id: NonFungibleId,                     // building contract ID
            mut payment: Bucket,                            // payment bucket
            land_owner_sbt: Proof                           // land owner SBT proof
        ) -> (Bucket,Bucket,Bucket){
            // check land owner SBT proof
            let land_owner_sbt: ValidatedProof = land_owner_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[build_call]: Invalid proof provided!");  
            let land_owner_sbt_id = land_owner_sbt.non_fungible::<UserSBT>().id();     

            let mut land_owner_data_sbt: UserSBT = land_owner_sbt.non_fungible().data();
            let mut land_id = NonFungibleId::from_u64(0);
            let mut land_id_vec: Vec<NonFungibleId> = Vec::new();
            let mut found = false;
            for asset in &land_owner_data_sbt.real_estate_properties {
                if self.land_nft == asset.0 {
                    land_id_vec.push(asset.1.clone());
                    found = true;
                }
            }
            assert!(found,"[approve_contract]:Land Asset resource address correspondence unfound!");

            let mut building_property = ResourceAddress::from(RADIX_TOKEN);
            let mut building_id = NonFungibleId::from_u64(0);
            let mut building_surface = 0;

            // check provided resources and verify Building Contract hasn't been already approved
            let mut found = false;
            let mut amount = Decimal::zero();
            for value in self.build_call_vec.iter_mut() {
                if value.0 == contract_address && value.1 == contract_id.clone() {
                    assert!( 
                        value.2.land_owner_sbt_address == land_owner_sbt.resource_address(),
                        "[approve_contract]: Land owner SBT address mismatch!"
                    );
                    assert!( 
                        value.2.land_owner_sbt_id == land_owner_sbt_id.clone(),
                        "[approve_contract]: Land owner SBT ID mismatch!"
                    );
                    assert!( 
                        value.2.land_property_nft == self.land_nft,
                        "[approve_contract]: Land property NFT address mismatch!"
                    );
                    land_id = value.2.land_property_nft_id.clone();
                    assert!(
                        land_id_vec.contains(&land_id.clone()),
                        "[approve_contract]:Land Asset ID correspondence unfound!"
                    );
                    assert!(
                        !value.2.approved,
                        "[approve_contract]: Contract already approved!"
                    );
                    amount = value.3; 

                    if Runtime::current_epoch() > value.2.deadline+self.payment_deadline {          
                        value.6 = true;
                        info!("[approve_contract]: Payment deadline overtaken! Penalty applied.");
                        if value.5 {
                            amount = amount*dec!("90")/dec!("100"); 
                            assert!(
                                payment.amount() >= amount*dec!("90")/dec!("100"),
                                "[approve_contract]: Insufficient payment amount! Penalty required."
                            );
                        } else {
                            amount = amount*dec!("93")/dec!("100");
                            assert!(
                                payment.amount() >= amount*dec!("93")/dec!("100"),
                                "[approve_contract]: Insufficient payment amount! Penalty required."
                            );
                        }
                    } else {
                        if value.5 {
                            amount = amount*dec!("87")/dec!("100");
                            assert!(
                                payment.amount() >= amount*dec!("87")/dec!("100"),
                                "[approve_contract]: Insufficient payment amount!"
                            );
                        } else {
                            amount = amount*dec!("90")/dec!("100");
                            assert!(
                                payment.amount() >= amount*dec!("90")/dec!("100"),
                                "[approve_contract]: Insufficient payment amount!"
                            );
                        }
                    }
                    value.2.approved = true;

                    // set Building Contract NFT data related value asapproved
                    match self.building_contract_vault.get_mut(&(value.0,value.1.clone())) {
                        Some(vault) => {
                            let contract = vault.take_non_fungible(&value.1.clone());
                            let mut data: BuildingContract = contract.non_fungible().data();
                            assert!(
                                !data.approved,
                                "[approve_contract]: Contract data mismatch (approved)"
                            );
                            data.approved = true;
                            assert_eq!(
                                land_id.clone(),
                                data.land_property_nft_id,
                                "[approve_contract]: Contract data mismatch (land_property_nft_id)"
                            );
                            building_surface = data.building_surface;

                            // update Building Contract NFT data
                            self.minter_badge.authorize(|| {
                                borrow_resource_manager!(self.building_contract)
                                    .update_non_fungible_data(&value.1, data)
                            });

                            vault.put(contract);
                        }
                        None => {
                            info!("[approve_contract]: Building contract vault unfound!");
                            std::process::abort()
                        }
                    }
                    building_property = value.2.property_building_nft;
                    building_id = value.2.property_building_nft_id.clone();
                    found = true;
                    break;
                }
            }
            assert!(found,"[approve_contract]: Contract correspondence undetected!");

            // put Contract Building payment in vault
            // take Land Property AssetNFT and Building Property AssetNFT from vaults
            let (land_property_nft,building_property_nft): (Bucket,Bucket);
            match self.deposit_vaults.get_mut(&(contract_address,contract_id)) {
                Some(vaults_vec) => { 
                    vaults_vec[0].0.put(payment.take(amount)); 
                    land_property_nft = vaults_vec[0].1.take_non_fungible(&land_id.clone());    
                    building_property_nft = vaults_vec[0].3.take_non_fungible(&building_id.clone()); 
                } 
                None => {
                    info!("[approve_contract]: Deposit vault unfound!");
                    std::process::abort()
                }
            }

            let mut linked_assets: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();
            linked_assets.push((building_property,building_id.clone()));

            // upgrade provided Land Property AssetNFT to include Building Property AssetNFT data
            let (payment,land_property_nft) = self.update_land_property_nft(
                payment,
                land_property_nft,
                linked_assets.clone(),
                building_surface
            );                                                                                    

            // Update data on provided land owner SBT: add building property NFT resource address 
            // and ID. 
            let lenght = land_owner_data_sbt.real_estate_properties.len();
            let mut real_estate_properties_vec = land_owner_data_sbt.real_estate_properties.clone();
            let mut index = 0;
            loop {
                let mut tup = real_estate_properties_vec.pop().unwrap();
                if self.land_nft == tup.0 && land_id.clone() == tup.1 { 
                    tup.2.value_1 += 1;
                    tup.2.value_2 += building_surface;
                    tup.2.linked_assets.append(&mut linked_assets.clone());
                    land_owner_data_sbt.real_estate_properties
                        .retain(|x| x.1 != tup.1.clone());
                    land_owner_data_sbt.real_estate_properties.push(tup);
                    break;
                }
                index += 1;
                if index == lenght {
                    break;
                }
            }

            self.sbt_updater_badge.authorize(|| {
                borrow_resource_manager!(land_owner_sbt.resource_address())
                    .update_non_fungible_data(&land_owner_sbt_id, land_owner_data_sbt)
            });       
                                                                            
            (payment,land_property_nft,building_property_nft) 
        }

        // Method callable to ckeck data on provided Asset NFT like Land Property or Building
        // Property
        pub fn check_asset_nft(&mut self, asset_nft: Bucket) -> Bucket {
            let data: AssetNFT = asset_nft.non_fungible().data();
            info!(" data_1: {} ", data.data_1);
            info!(" data_2: {} ", data.data_2);
            info!(" data_3: {} ", data.data_3);
            info!(" data_4: {} ", data.data_4);
            info!(" value_1: {} ", data.value_1);
            info!(" value_2: {} ", data.value_2);
            info!(" value_3: {} ", data.value_3);
            for tup in data.linked_assets {
                info!(
                    " linked_assets resource address: 
                    {} ", 
                    tup.0
                );
                info!(
                    " linked_assets ID: 
                    {} ", 
                    tup.1
                );
            }

            asset_nft
        }   

        // Collect building contract payment once checked the latter has been approved by client,
        // other data checking succesfully performed and update Contractor SBT data stating 
        // relevant contract has been correctly executed 
        pub fn collect_contract_payment(
            &mut self,
            contract_address: ResourceAddress,                 // building contract resource address
            contract_id: NonFungibleId,                        // building contract ID
            contractor_sbt: Proof                              // general contractor SBT proof 
        ) -> Bucket {
            // check general contractor SBT proof
            let contractor_sbt: ValidatedProof = contractor_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[collect_contract_payment]: Invalid proof provided!");
            let contractor_sbt_id = contractor_sbt.non_fungible::<UserSBT>().id();

            // Verify Building Contract correspondence and approved status within related list
            let mut found = false;
            let mut penalty = false;
            let mut amount = Decimal::zero();
            for value in self.build_call_vec.iter_mut() {
                if value.0 == contract_address && value.1 == contract_id.clone() {
                    assert_eq!(
                        value.2.contractor_sbt_address == contractor_sbt.resource_address(),
                        value.2.contractor_sbt_id == contractor_sbt_id.clone(),
                        "[collect_contract_payment]: Contractor correspondence unfound!"
                    );
                    assert!(
                        value.2.approved,
                        "[collect_contract_payment]: Deal unapproved!"
                    );
                    penalty = value.5;

                    // Verify Building Contract NFT datat correspondence
                    match self.building_contract_vault.get_mut(&(value.0,value.1.clone())) {
                        Some(vault) => {
                            let contract = vault.take_non_fungible(&value.1.clone());
                            let data: BuildingContract = contract.non_fungible().data();
                            assert_eq!(
                                value.2.approved,
                                data.approved,
                                "[collect_contract_payment]: Contract data mismatch!"
                            );
                            amount = data.contract_amount;
                            vault.put(contract);
                        }
                        None => {
                            info!("[collect_contract_payment]: Building contract vault unfound!");
                            std::process::abort()
                        }
                    }
                    assert_eq!(
                        value.3,
                        amount,
                        "[collect_contract_payment]: Amounts uncorrespondence detected!"
                    );
                    found = true;
                    break;
                }
            }
            assert!(found,"[collect_contract_payment]: Contract correspondence undetected!");


            // Update General Contractor SBT data by inserting ID and value of executed contract as
            // positive feedback for future references.  
            let info = "Contract ".to_string()+&contract_id.clone().to_string()+" Executed";

            let mut data_sbt: UserSBT = contractor_sbt.non_fungible().data();
            data_sbt.values.push((info,amount));

            self.sbt_updater_badge.authorize(|| {
                borrow_resource_manager!(contractor_sbt.resource_address())
                    .update_non_fungible_data(&contractor_sbt_id, data_sbt)
            });

            // take payment from vault and return it
            match self.deposit_vaults.get_mut(&(contract_address,contract_id)) {
                Some(vaults_vec) => {
                    if !penalty {
                        vaults_vec[0].0.take_all()
                    } else {
                       vaults_vec[0].0.take(amount) 
                    }
                }
                None => {
                    info!("[collect_contract_payment]: Deposit vault unfound!");
                    std::process::abort()
                }
            }
        }

        // Retrieve a list of all build calls made within protocol
        pub fn build_call_list(&mut self) {
            for value in self.build_call_vec.iter() {
                info!(" =========================================================================");
                info!(" House Hub address: {} ", value.2.house_hub_address);
                info!(" Building Contract resource address: {} ", value.0);
                info!(" Building Contract id: {} ", value.1);                
                info!(" =========================================================================");
                info!(" Building Contract URL: {} ", value.2.url);
                info!(" Land owner SBT id: {} ", value.2.land_owner_sbt_id);
                if value.2.contractor_sbt_address == ResourceAddress::from(RADIX_TOKEN) {
                    info!(" Contractor SBT address: None ");
                } else {
                    info!(" Contractor SBT address: {} ", value.2.contractor_sbt_address);
                }
                if value.2.contractor_sbt_id == NonFungibleId::from_u64(0) {
                    info!(" Contractor SBT id: None ");
                } else {
                    info!(" Contractor SBT id: {} ", value.2.contractor_sbt_id);
                }
                info!(" Land property NFT: {} ", value.2.land_property_nft);
                info!(" Land property NFT id: {} ", value.2.land_property_nft_id);
                info!(" House project NFT: {} ", value.2.house_project_nft);
                info!(" House project NFT id : {} ", value.2.house_project_nft_id);
                info!(" Building surface: {} ", value.2.building_surface);
                info!(" Contract amount: {} ", value.2.contract_amount);

                if value.2.deadline == 0 {
                    info!(" Deadline: None ");
                } else {
                    info!(" Deadline: {} ", value.2.deadline);
                }
                info!(" Executed: {} ", value.2.executed);
                let name = borrow_resource_manager!(self.currency)
                                .metadata()["name"].clone();
                let symbol = borrow_resource_manager!(self.currency)
                                .metadata()["symbol"].clone();
                info!(" Building Contract value: {} ${} {}", value.3, symbol, name); 
                if !value.5 {
                    info!(" Penalty: None ");
                } else {
                    info!(" Penalty: {} ", value.5);
                }  
            }   
        }

        // Method callable to merge two adjacent Land Property Asset NFTs, both registered within
        // same SBT owner. 
        pub fn merge_properties(
            &mut self, 
            url: String,                            // URL pointing to merged property data
            land_property_nft_one: Bucket,          // land property AssetNFT number one
            land_property_nft_two: Bucket,          // land property AssetNFT number two
            payment: Bucket,                        // Payment Bucket
            land_owner_sbt: Proof                   // Neverland land owner SBT as proof
        ) -> (Bucket,Bucket) {
            // check land owner SBT proof
            let land_owner_sbt: ValidatedProof = land_owner_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[merge_properties]: Invalid proof provided!");  

            // Collect NFT and SBT data and IDs
            let nft_one_addr = land_property_nft_one.resource_address();
            let nft_two_addr = land_property_nft_two.resource_address();
            let nft_one_data: AssetNFT = land_property_nft_one.non_fungible().data();
            let nft_two_data: AssetNFT = land_property_nft_two.non_fungible().data();
            let nft_one_id = land_property_nft_one.non_fungible::<AssetNFT>().id();
            let nft_two_id = land_property_nft_two.non_fungible::<AssetNFT>().id();
            
            info!("[merge_properties]: parcel_one: {}",nft_one_data.data_4.clone());
            info!("[merge_properties]: parcel_two: {}",nft_two_data.data_4.clone());
            // Retrieve land property parcels position: derive numbers from strings deleting chars.
            let parcel_one = nft_one_data.data_4.chars().skip_while(|c| !c.is_digit(10)).collect::<String>();
            let parcel_two = nft_two_data.data_4.chars().skip_while(|c| !c.is_digit(10)).collect::<String>();
            
            // Get mint code ID form related map to retrieve Land asset NFT mint data within relative
            // external Merge NFT Data component  
            let mint_key_id = parcel_one.clone() + &parcel_two.clone();
            info!("[merge_properties]: Mint code ID {}",mint_key_id.clone());
            
            // Retrieve mint code ID from relative merge data map
            let mint_code_id: String;
            match self.merge_data.get(&mint_key_id) {
                Some(value) => mint_code_id = value.clone(),
                None => {
                    info!("[merge_properties]: Mint code ID unfound within related map!");
                    std::process::abort()
                }
            }

            // Convert to numeric data
            let parsed_one = parcel_one.trim().parse::<u32>().unwrap();
            let parsed_two = parcel_two.trim().parse::<u32>().unwrap();             

            // Demux numeric data to retrieve land parcels coordinates.
            let (pos_one_x,pos_one_y) = self.process_data(parsed_one);
            let (pos_two_x,pos_two_y) = self.process_data(parsed_two);

            // Verify land property parcels are contiguous.
            if pos_one_x == pos_two_x {
                if pos_one_y > pos_two_y {
                    assert!(pos_one_y-pos_two_y == 1,"[merge_properties]:Land parcels uncontiguos");
                } else if pos_two_y > pos_one_y {
                    assert!(pos_two_y-pos_one_y == 1,"[merge_properties]:Land parcels uncontiguos");
                } else {
                    info!("[merge_properties]:Land parcels position unrecognized");
                    std::process::abort()
                }
            } else if pos_one_y == pos_two_y {
                if pos_one_x > pos_two_x {
                    assert!(pos_one_x-pos_two_x == 1,"[merge_properties]:Land parcels uncontiguos");
                } else if pos_two_x > pos_one_x {
                    assert!(pos_two_x-pos_one_x == 1,"[merge_properties]:Land parcels uncontiguos");
                } else {
                    info!("[merge_properties]:Land parcels position unrecognized");
                    std::process::abort()
                }
            } else {
                info!("[merge_properties]:Land parcels uncontiguos");
                std::process::abort()
            }

            // Setup data to perform NFT merge calling an external NFT Farm Component whom previously
            // minted related NFT assets resources.  
            let nft_farm_badge_bckt = self.nft_farm_bdg_take(1);
            let nft_farm_bdg_ref = nft_farm_badge_bckt.create_proof();  

            let method = "merge_asset_nft".to_string(); 
            let pitia_method = "get_code".to_string();
            let method_x = "asset_data_one".to_string();
            let method_y = "asset_data_two".to_string();
            let method_z = "asset_data_three".to_string();
            let method_w = "asset_data_for".to_string();

            let args = args![
                land_property_nft_one,
                land_property_nft_two,
                payment,
                mint_code_id,
                url,
                self.pitia_addr,
                pitia_method,
                self.nft_data_addr,
                method_x,
                method_y,
                method_z,
                method_w,               
                nft_farm_bdg_ref
            ]; 

            let (payment,land_merged_nft) = 
                borrow_component!(self.nft_farm_comp).call::<(Bucket,Bucket)>(&method, args);

            self.nft_farm_badge_bckt_put(nft_farm_badge_bckt,1);

            // Retrieve merged NFT data and ID to update merging data on provided land owner SBT.
            let merged_nft_id = land_merged_nft.non_fungible::<AssetNFT>().id();     
            let merged_nft_data: AssetNFT = land_merged_nft.non_fungible().data();

            // Retrieve SBT data to update merging data on provided land owner SBT.  
            let land_owner_sbt_id = land_owner_sbt.non_fungible::<UserSBT>().id();     
            let mut land_owner_data_sbt: UserSBT = land_owner_sbt.non_fungible().data();

            // Remove old NFTs data from provided land owner SBT. 
            let mut i = 0;
            for asset in land_owner_data_sbt.real_estate_properties.clone() {
                if asset.0 == nft_one_addr && asset.1 == nft_one_id {
                    land_owner_data_sbt.real_estate_properties.remove(i);
                    break;
                }
                i += 1;
            } 
            i = 0;
            for asset in land_owner_data_sbt.real_estate_properties.clone() {
                if asset.0 == nft_two_addr && asset.1 == nft_two_id {
                    land_owner_data_sbt.real_estate_properties.remove(i);
                    break;
                }
                i += 1;
            } 

            // Add merged NFT data to provided land owner SBT.
            land_owner_data_sbt.real_estate_properties
                .push((land_merged_nft.resource_address(),merged_nft_id,merged_nft_data));

            self.sbt_updater_badge.authorize(|| {
                borrow_resource_manager!(land_owner_sbt.resource_address())
                    .update_non_fungible_data(&land_owner_sbt_id, land_owner_data_sbt)
            });

            (payment,land_merged_nft)
        }

        // Retrieve property assets list within land owner SBT data 
        pub fn ask_property_sbt(&mut self, land_owner_sbt: Proof) {
            // check land owner SBT proof
            let land_owner_sbt: ValidatedProof = land_owner_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[ask_property_sbt]: Invalid proof provided");

            let id = land_owner_sbt.non_fungible::<AssetNFT>().id();
            let data: UserSBT = land_owner_sbt.non_fungible().data();

            info!(" Land Owner SBT NFT resource address: {} ",land_owner_sbt.resource_address());
            info!(" Land Owner SBT NFT id: {} ",id);
            for tuple in data.real_estate_properties.clone() {
                info!(" ========================================================================="); 
                info!(" Real estate property NFT resource address: {} ",tuple.0);
                info!(" Real estate property NFT id: {} ",tuple.1);
                info!(" Real estate property NFT data: {:?} ",tuple.2);
            }
        }

        // Retrieve property assets list within land owner SBT data 
        pub fn ask_property_sbt_ext(&mut self, land_owner_sbt: Proof) {
            let land_owner_sbt: ValidatedProof = land_owner_sbt.unsafe_skip_proof_validation();
            assert!(
                borrow_resource_manager!(land_owner_sbt.resource_address()).resource_type()
                    == ResourceType::NonFungible,
                "[ask_property_sbt]: Invalid fungible proof provided"
            );
            assert!(
                land_owner_sbt.amount() == dec!("1"),
                "[ask_property_sbt]: Invalid number of proof provided"
            );
            let id = land_owner_sbt.non_fungible::<AssetNFT>().id();
            let data: UserSBT = land_owner_sbt.non_fungible().data();

            info!(" Land Owner SBT NFT resource address: {} ",land_owner_sbt.resource_address());
            info!(" Land Owner SBT NFT id: {} ",id);
            for tuple in data.real_estate_properties.clone() {
                info!(" ========================================================================="); 
                info!(" Real estate property NFT resource address: {} ",tuple.0);
                info!(" Real estate property NFT id: {} ",tuple.1);
                info!(" Real estate property NFT data: {:?} ",tuple.2);
            }
        }

            // Build a Caller Component Badge Resource to allow method's calls from an external 
            // Component.
            fn build_badge(&mut self, name: String) -> ResourceAddress {
                ResourceBuilder::new_fungible()
                .metadata("name", format!("{}",name))
                .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .no_initial_supply()
            }

            // Perform a call to the external Farm Component that previously minted the Land Property
            // Asset NFT to update his data once a related Building Construction NFT has been minted.  
            fn update_land_property_nft(
                &mut self, 
                payment: Bucket,                            
                land_property_nft: Bucket, 
                linked_assets: Vec<(ResourceAddress,NonFungibleId)>,
                building_surface: u8
            ) -> (Bucket,Bucket) {  
                let nft_farm_badge_bckt = self.nft_farm_bdg_take(0);
                let nft_farm_bdg_ref = nft_farm_badge_bckt.create_proof();  

                let method = "upgrade_asset_nft".to_string(); 
                let args = args![
                    payment,
                    land_property_nft,
                    linked_assets,
                    building_surface,
                    nft_farm_bdg_ref
                ]; 

                let (a,b) =
                    borrow_component!(self.nft_farm_comp).call::<(Bucket,Bucket)>(&method, args);
            
                self.nft_farm_badge_bckt_put(nft_farm_badge_bckt,0);
            
                (a,b)
            }

            // Put NFT Farm Badge back in Vault.
            fn nft_farm_badge_bckt_put(&mut self, nft_farm_badge_bckt: Bucket, flag: u8){
                let (vu,vm) = self.nft_farm_badge_vault.get_mut(&self.nft_farm_comp).unwrap();
                match flag {
                    0 => vu.put(nft_farm_badge_bckt),
                    _ => vm.put(nft_farm_badge_bckt)
                }
            }

            // Take NFT Farm Badge from related Vault to call an external NFT Farm Component.
            fn nft_farm_bdg_take(&mut self, flag: u8) -> Bucket {
                match self.nft_farm_badge_vault.get_mut(&self.nft_farm_comp) {
                    Some((vu,vm)) => {
                        match flag {
                            0 => vu.take(Decimal::one()),
                            _ => vm.take(Decimal::one())
                        }
                    }
                    None => {
                        info!(" [nft_farm_bdg_take] NFT Farm Badge not in stock! ");
                        std::process::abort()
                    }
                }
            } 

            // Numeric data extractor Fn.
            fn extract_code( seed: u32, exp: u32) -> (u32,u32) {
                let float_x = seed/10_u32.pow(exp);
                let data_x = u32::try_from(float_x).unwrap();
                let mask_x = data_x*10_u32.pow(exp);

                (data_x,seed-mask_x)
            } 

            // Demux input data Fn.  
            fn process_data(&mut self, seed: u32) -> (u32,u32) {
                let (_data_zero,num_g) = HouseHub::extract_code(seed,4);
                let (data_one,_num_h) = HouseHub::extract_code(num_g,2);
                let data_two = num_g-data_one*10_u32.pow(2);

                (data_one,data_two)
            } 
    }
}