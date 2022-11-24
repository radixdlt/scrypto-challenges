use scrypto::prelude::*;
use crate::data_farm::*;

// Assets NFT Auction Component 
blueprint! {
    struct NeverlandAuction {
        // Vault to stock NFT Asset Farm Caller Badge used to perfom external Component calls
        nft_farm_badge_vault: HashMap<ComponentAddress,Vault>,
        // Vault to stock SBT Updater Badge
        sbt_updater_badge: Vault,
        // SBT Updater Badge resource address
        sbt_updater_badge_addr: ResourceAddress,
        // User SBT Resource Address.
        user_sbt: ResourceAddress,
        // Asset NFT farm component address
        nft_farm_comp: ComponentAddress,
        // NFT data oracle component address 
        pitia_comp: ComponentAddress,        
        // NFT data component address
        nft_data_comp: ComponentAddress,
        // Auction owner_badge resource address
        owner_badge: ResourceAddress,
        // Minter badge vault
        minter_badge: Vault,
        // Auction deposited bid bonds vault
        bid_bonds: Vault,
        // Auction payment vault
        payment: Vault,
        // Vector with land assets listed in auction ordered by auction instance progressive number, 
        // auction time deadline, bid bond, payment deadline,land parcel, asset amount, 
        // asset surface in square meters, url link, mint code id
        land_assets_vec: Vec<(u128,u64,Decimal,u64,String,u32,u8,String,String)>,
        // Auction instance number
        instance_number: u128,
        // Protocol accepted currency resource address 
        currency: ResourceAddress,
        // Asset NFT farm component mint price 
        mint_price: Decimal,
        // Hashmap with bidders data
        bidders_map: HashMap<ResourceAddress, Bidder>,
        // Hashmap with auction instance number as key, highest bid, reserve price, auction payment 
        // honoured flag, auction payment withdrawed flag, bidder badges resource addresses list as 
        // values
        auction_map: HashMap<u128,(Decimal,Decimal,bool,bool,Vec<ResourceAddress>)>,
    }

    impl NeverlandAuction {
        pub fn new(
            land_auction_name: String,                  // Protocol instance name printed on minted badges
            sbt_updater_badge_addr: ResourceAddress,    // SBT updater badge resource address 
            land_data_owner_badge: ResourceAddress,     // Neverland Data owner badge badge resource address
            user_sbt: ResourceAddress,                  // User SBT resource address
            nft_farm_comp: ComponentAddress,            // NFT Farm component address
            pitia_comp: ComponentAddress,               // NFT Data Oracle component address    
            nft_data_comp: ComponentAddress,            // NFT Data component address 
            currency: ResourceAddress,                  // Protocol currency resource address
        ) -> (ComponentAddress, Bucket) {
            let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", land_auction_name.clone() + " MinterBadge ")
                .initial_supply(Decimal::one());

            let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", land_auction_name.clone() + " OwnerBadge ")
                .initial_supply(Decimal::one());

            let access_rules = AccessRules::new()   
                .method("stock_sbt_updater_badge", rule!(require(land_data_owner_badge)))
                .method("stock_nft_farm_badge", rule!(require(owner_badge.resource_address())))              
                .method("new_land_auction", rule!(require(owner_badge.resource_address())))
                .method("claim_payment", rule!(require(owner_badge.resource_address())))
                .default(rule!(allow_all)); 

            let mut neverland_auction: NeverlandAuctionComponent = Self {
                nft_farm_badge_vault: HashMap::new(),
                sbt_updater_badge: Vault::new(sbt_updater_badge_addr),
                sbt_updater_badge_addr,
                user_sbt,
                nft_farm_comp,
                pitia_comp,        
                nft_data_comp,
                owner_badge: owner_badge.resource_address(),
                minter_badge: Vault::with_bucket(minter_badge),
                bid_bonds: Vault::new(currency.clone()),
                payment: Vault::new(currency.clone()),
                land_assets_vec: Vec::new(),
                instance_number: 1,
                currency,
                mint_price: Decimal::zero(),
                bidders_map: HashMap::new(),
                auction_map: HashMap::new(),
            }
            .instantiate();
            neverland_auction.add_access_check(access_rules);

            (neverland_auction.globalize(),owner_badge)
        }

        // Stock SBT updater badge to update users SBT data when a Land Asset NFT auction is won   
        // and payment provided. Callable by Land Data protocol owner badge only.
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

        // Stock NFT farm caller badge to call Farm external component and specify other data like
        // Land NFT mint price, NFT Data Oracle component address, NFT Data component address.
        // Callable by protocol Owner Badge only.
        pub fn stock_nft_farm_badge(
            &mut self, 
            mint_price: Decimal,
            pitia_comp: ComponentAddress,        
            nft_data_comp: ComponentAddress,
            nft_farm_comp: ComponentAddress, 
            nft_farm_badge: Bucket
        ) {
            self.mint_price = mint_price;
            self.nft_farm_comp = nft_farm_comp;
            self.pitia_comp = pitia_comp;        
            self.nft_data_comp = nft_data_comp;
            
            let v = self.nft_farm_badge_vault
                .entry(self.nft_farm_comp)
                .or_insert(Vault::new(nft_farm_badge.resource_address()));

            v.put(nft_farm_badge);
        }

        // Init new auction instancies. Callable by protocol Onwer Badge only.  
        // Vector with land assets listed in auction ordered by auction instance progressive number.
        // 
        // "land_assets" vector tuple parameters: land parcel, asset amount, asset surface in
        //  square meters, url link, mint code id
        pub fn new_land_auction(
            &mut self, 
            land_assets: Vec<(String,u32,u8,String,String)>, 
            duration: u64, 
            reserve_price: Decimal, 
            bid_bond: Decimal,
            pay_frame: u64
        ) {
            assert!(bid_bond <= reserve_price/dec!("3"), " Bid bond amount too high! ");
            assert!(duration >= 5000, " Increase duration! ");
            assert!(pay_frame >= 1000, " Increase payment deadline! ");

            let endtime = Runtime::current_epoch()+duration;
            let mut land_assets_vec: Vec<(u128,u64,Decimal,u64,String,u32,u8,String,String)> = Vec::new();

            let bidder_vec_zero: Vec<ResourceAddress> = Vec::new();

            for t in land_assets {
                land_assets_vec
                    .push((self.instance_number,endtime,bid_bond,pay_frame,t.0,t.1,t.2,t.3,t.4));
                self.auction_map.insert(
                    self.instance_number,
                    (dec!("0"),reserve_price,false,false,bidder_vec_zero.clone())
                );
                self.instance_number += 1;
            }

            self.land_assets_vec.append(&mut land_assets_vec);
        }

        // Retrieve auction istancies list
        pub fn auction_list(&mut self) {
            for (nr,endtime,bond,pay_frame,parcel,amount,surface,url,_id) in &self.land_assets_vec {
                if endtime >= &Runtime::current_epoch() { 
                    info!("======================================================================");
                    info!(
                        " Auction instance number: {} 
                        Auction deadline: {} 
                        Auction bid bond: {}
                        Auction payment deadline: {}
                        Land Asset Parcel: {} 
                        Land Asset NFT amount: {}
                        Land Asset surface: {}
                        Land Asset linked URL: {}",
                        nr,
                        endtime,
                        bond,
                        pay_frame+endtime,
                        parcel,
                        amount,
                        surface,
                        url
                    );
                }
            }
        }

        // Register a user in a specified auction instance once provided required bid bond and SBT 
        // proof  
        pub fn register(
            &mut self, 
            instance_nr: u128, 
            mut bid_bond: Bucket, 
            user_sbt: Proof
        ) -> (Bucket,Bucket) {
            // Check provided SBT
            let user_sbt: ValidatedProof = user_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[register]: Invalid SBT proof provided!");
            let user_sbt_id = user_sbt.non_fungible::<UserSBT>().id(); 

            // Check provided bid bond currency correspondence
            assert!(bid_bond.resource_address() == self.currency, "[register]:Wrong currency");

            // Check auction instance number correspondence and verify auction instance is open
            let mut found = false;             
            for data in self.land_assets_vec.iter_mut() {
                if data.0 == instance_nr {
                    assert!(Runtime::current_epoch() <= data.1, "[register]:Auction time expired!");
                    assert!(bid_bond.amount() >= data.2, "[register]:Insufficient bid bond");
                    self.bid_bonds.put(bid_bond.take(data.2));
                    found = true;
                    break;  
                }
            }
            assert!(found, "[register]:Auction instance number correspondence check failed!");

            // Mint a bidder badge
            let bidder_bdg: ResourceAddress = ResourceBuilder::new_fungible()
                    .metadata("name","BidderBadge")
                    .metadata("SBT_id", format!("{}", user_sbt_id))
                    .metadata("SBT_res_addr", format!("{}", self.user_sbt))
                    .metadata("instance", format!("{}", instance_nr))
                    .metadata(
                        "NeverlandAuction", 
                        format!("{}", Runtime::actor().as_component().0.to_string())
                    )
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply();
                             
            let bidder_badge =  self.minter_badge.authorize(|| { 
                borrow_resource_manager!(bidder_bdg).mint(Decimal::one()) 
            });

            // Insert new bidder data within related map 
            let new_bidder = Bidder { 
                instance_nr: instance_nr, 
                bid: Decimal::zero(), 
                bid_bond_reclaimed: false,
                asset_collected: false
            }; 

            self.bidders_map.insert(bidder_bdg,new_bidder.clone());

            match self.auction_map.get_mut(&instance_nr) {
                Some(tup) => {
                    tup.4.push(bidder_badge.resource_address())
                }
                None => {
                    info!("[register]: Auction instance unfound within bidders map!");
                    std::process::abort()
                }
            }

            (bidder_badge,bid_bond)
        }

        // Place a bid on relative auction instance number specified within required bidder badge 
        pub fn place_bid(&mut self, bid: Decimal, bidder_badge: Proof) {            
            // Check the bidder badge and get the bidder data
            let (mut bidder,bidder_addr) = self.get_bidder(bidder_badge);

            // Check if auction istance is open
            let mut found = false;
            for tup in &self.land_assets_vec {
                if tup.0 == bidder.instance_nr {
                    assert!(Runtime::current_epoch() <= tup.1, "[place_bid]:Auction closed");
                    found = true;
                    break;
                }
            }
            assert!(found, "[place_bid]:Auction instance number correspondence check failed!");

            // Check if bid amount is acceptable and update highest bid value amount within relative
            // map 
            match self.auction_map.get_mut(&bidder.instance_nr) {
                Some(tup) => {
                    assert!(bid != dec!("0"), "[place_bid]:No bid provided!");
                    assert_eq!(
                        bid > tup.0, 
                        bid >= tup.1,
                        "[place_bid]:Bid amount results too low!");
                    tup.0 = bid;
                }
                None => {
                    info!("[place_bid]: Auction instance unfound within auction map!");
                    std::process::abort()
                }
            }

            // Save bid overwriting related Hashmap
            bidder.bid = bid;
            self.bidders_map.insert(bidder_addr, bidder);
        }

        // Method to mint and claim a Land Asset NFT once won related auction instance.
        // Required a payment to honour the auction, winner badge proof as well as user SBT proof to
        // assign ownership of minted Land Asset NFT. 
        pub fn claim_land_asset(
            &mut self, 
            mut payment: Bucket, 
            bidder_badge: Proof, 
            user_sbt: Proof
        ) -> (Bucket,Vec<Bucket>) {
            // Check provided SBT
            let user_sbt: ValidatedProof = user_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[claim_land_asset]: Invalid SBT proof provided!");

            // Check the bidder badge and get bidder data
            let (mut bidder,bidder_addr) = self.get_bidder(bidder_badge); 

            // Check bidder badge and user SBT ID correspondence
            let user_sbt_id = user_sbt.non_fungible::<UserSBT>().id();
            let sbt_id = borrow_resource_manager!(bidder_addr.clone())
                            .metadata()["SBT_id"].clone();
            assert!(
                sbt_id == user_sbt_id.to_string(),
                "[claim_land_asset]: SBT ID mismatch detected!"
            );      
            
            // Check if auction istance is still open, payment timeframe results expired. 
            // Retrieve auction bid bond deposited amount as well as other data helpful to 
            // call NFT Farm external component to mint a Land asset NFT resource.
            let (mut bid_bond, mut amount, mut asset_surface) = (dec!(0), 0, 0); 
            let (mut url, mut mint_code) = ("".to_string(), "".to_string()); 
            let mut found = false;
            for tup in &self.land_assets_vec {
                if tup.0 == bidder.instance_nr {
                    assert!(Runtime::current_epoch() > tup.1, "[claim_land_asset]:Auction open!");
                    assert!(
                        Runtime::current_epoch() <= tup.1+tup.3, 
                        "[claim_land_asset]:Payment deadline overtaken!"
                    );
                    (bid_bond, amount, asset_surface) = (tup.2, tup.5, tup.6);
                    (url, mint_code) = (tup.7.clone(), tup.8.clone());
                    found = true;
                    break;
                }
            }
            assert!(found, "[claim_land_asset]: Auction instance unfound!");

            // Check within related map if bidder won relative auction instance and update payment
            // state setting switching relative bool var value
            match self.auction_map.get_mut(&bidder.instance_nr) {
                Some(tup) => {
                    assert!(bidder.bid == tup.0,"[claim_land_asset]:Loser bid detected!");
                    tup.2 = true;
                }
                None => {
                    info!("[claim_land_asset]: Auction instance unfound within auction map!");
                    std::process::abort()
                }
            }

            // Verify Land Asset NFT ain't already been claimed and update bidder badge data 
            assert!(!bidder.asset_collected, "[claim_land_asset]: Land Asset NFT already claimed!");
            bidder.asset_collected = true;
            self.bidders_map.insert(bidder_addr, bidder.clone());

            // Check payment amount and currency
            assert_eq!(
                payment.resource_address() == self.currency, 
                payment.amount() >= bidder.bid-bid_bond, 
                "[claim_land_asset]: Incorrect payment amount or wrong currency detected!"
            );

            // Put payment in respective vault. Transfer deposited bid bond to payment vault.
            self.payment.put(payment.take(bidder.bid-bid_bond-self.mint_price));
            self.payment.put(self.bid_bonds.take(bid_bond));

            // Perform a call to NFT Farm external component to mint a Land asset NFT resource.  
            let (payment,land_asset_nft_vec) = 
                self.mint_land_asset(mint_code, payment, amount, asset_surface, url); 

            // Update data within user's provided SBT storing land asset NFT data
            for land_asset_nft in &land_asset_nft_vec {
                let land_asset_addr = land_asset_nft.resource_address();
                let land_asset_id = land_asset_nft.non_fungible::<AssetNFT>().id();
                let land_asset_data: AssetNFT = land_asset_nft.non_fungible().data();

                let land_owner_sbt_id = user_sbt.non_fungible::<UserSBT>().id();
                let mut land_owner_data_sbt: UserSBT = user_sbt.non_fungible().data();

                land_owner_data_sbt.real_estate_properties
                    .push((land_asset_addr,land_asset_id,land_asset_data));

                self.sbt_updater_badge.authorize(|| {
                    borrow_resource_manager!(user_sbt.resource_address())
                        .update_non_fungible_data(&land_owner_sbt_id, land_owner_data_sbt)
                }); 
            }

            (payment,land_asset_nft_vec)
        }

        // Reclaim deposited bid bond method for auction loser bidders. 
        pub fn reclaim_bid_bond(&mut self, bidder_badge: Proof) -> Bucket {            
            // Check bidder badge and get bidder data
            let (mut bidder,bidder_addr) = self.get_bidder(bidder_badge);

            // Check if auction is closed and retrieve bid bound deposited value
            let mut bid_bond = dec!(0);
            let mut found = false;
            for tup in &self.land_assets_vec {
                if tup.0 == bidder.instance_nr {
                    assert!(Runtime::current_epoch() > tup.1, "[reclaim_bid_bond]:Auction open!");
                    bid_bond = tup.2;
                    found = true;
                    break;
                } 
            }
            assert!(found, "[reclaim_bid_bond]: Auction instance unfound!");

            // Check within related map if bidder won relative auction instance  
            match self.auction_map.get_mut(&bidder.instance_nr) {
                Some(tup) => assert!(bidder.bid != tup.0,"[reclaim_bid_bond]:Winner bid detected!"),
                None => {
                    info!("[claim_land_asset]: Auction instance unfound within auction map!");
                    std::process::abort()
                }
            }

            // Check if the bidder has not yet reclaimed the bid bond
            assert!(!bidder.bid_bond_reclaimed, "Bid bond already claimed");

            // Save that the bidder reclaimed the bid bond
            bidder.bid_bond_reclaimed = true;
            self.bidders_map.insert(bidder_addr, bidder);

            self.bid_bonds.take(bid_bond)
        }
        
        // Collect payment if auction instance has been honoured otherwise collect bid bond 
        // deposited by winner bidder is payment deadline has been overtaken or collect an empty
        // bucket if auction instance gone deserted. 
        // Callable by protocol Owner Badge only.
        pub fn claim_payment(&mut self, instance_number: u128) -> Bucket {                          
            // Check if auction is closed, payment deadline wasn't overtaken and retrieve bid bound 
            // deposited value
            let mut bid_bond = dec!(0);
            let mut deadline_overtaken = false;
            let mut found = false;
            for tup in &self.land_assets_vec {
                if tup.0 == instance_number {
                    assert!(Runtime::current_epoch() > tup.1, "[claim_payment]:Auction open!");
                    if Runtime::current_epoch() > tup.1+tup.3 {
                        deadline_overtaken = true;
                    }
                    bid_bond = tup.2;
                    found = true;
                    break;
                } 
            }
            assert!(found, "[claim_payment]: Auction instance unfound!");                          

            // Check within related map if auction instance payment has already been claimed, if 
            // same payment has previously been made and update inherent value, retrieve also 
            // winner bid amount  
            let mut winner_bid = dec!(0);
            let mut payment_received = false;
            let mut collect_penalty = false;  
            match self.auction_map.get_mut(&instance_number) {
                Some(tup) => {
                    assert!(tup.0 != dec!("0"),"[claim_payment]:Auction went deserted!");
                    assert!(!tup.3,"[claim_payment]:Payment already claimed!");
                    if tup.2 {
                        winner_bid = tup.0;
                        tup.3 = true;
                        payment_received = true; 
                    } else {
                        collect_penalty = true;
                    }
                }
                None => {
                    info!("[claim_payment]: Auction instance unfound within auction map!");
                    std::process::abort()
                }
            }                                                                                      

            // Determine kind of resource and relative amount are going to be collected
            if deadline_overtaken && collect_penalty {                                              
                self.payment.take(bid_bond)
            } else if !deadline_overtaken && payment_received {                                     
                self.payment.take(winner_bid-self.mint_price)
            } else if deadline_overtaken && payment_received {                                      
                self.payment.take(winner_bid-self.mint_price)
            } else {
                info!("[claim_payment]:Payment deadline not yet passed nor payment been received!");
                Bucket::new(RADIX_TOKEN)
            }
        }

        // Verify provided bidder badge proof and retrieve his own data
        fn get_bidder(&self, bidder_badge: Proof) -> (Bidder,ResourceAddress) {
            let bidder_badge: ValidatedProof = bidder_badge.unsafe_skip_proof_validation();          

            assert!(bidder_badge.amount() > Decimal::zero(), "No bidder badge presented");
            let bidder = self.bidders_map.get(&bidder_badge.resource_address());
            assert!(bidder.is_some(), "Incorrect bidder badge");

            (bidder.unwrap().clone(),bidder_badge.resource_address())
        }

        // Method callable to mint a Land Asset NFT performing a call to NFT Farm external Component
        fn mint_land_asset(
            &mut self, 
            mint_code_id: String,
            payment: Bucket,
            amount: u32,
            asset_surface: u8,
            url: String
        ) -> (Bucket, Vec<Bucket>) {
            // Take related caller badge from vault and create a proof of it 
            let nft_farm_badge_bckt = self.nft_farm_bdg_take();
            let nft_farm_bdg_ref = nft_farm_badge_bckt.create_proof(); 

            info!("mint_code_id: {}",mint_code_id.clone()); 

            let method = "mint_asset_nft".to_string(); 
            let pitia_method = "get_code".to_string();
            let method_x = "asset_data_one".to_string();
            let method_y = "asset_data_two".to_string();
            let method_z = "asset_data_three".to_string();
            let method_w = "asset_data_for".to_string();

            let args = args![
                amount,
                payment,
                mint_code_id,
                url,
                asset_surface,
                self.pitia_comp,
                pitia_method,
                self.nft_data_comp,
                method_x,
                method_y,
                method_z,
                method_w,               
                nft_farm_bdg_ref
            ]; 

            let (payment,land_asset_nft_vec) = 
                borrow_component!(self.nft_farm_comp).call::<(Bucket, Vec<Bucket>)>(&method, args);

            self.nft_farm_badge_bckt_put(nft_farm_badge_bckt);

            (payment,land_asset_nft_vec)
        }

        // Put NFT Farm Badge back in Vault.
        fn nft_farm_badge_bckt_put(&mut self, nft_farm_badge_bckt: Bucket){
            let v = self.nft_farm_badge_vault.get_mut(&self.nft_farm_comp).unwrap();
            v.put(nft_farm_badge_bckt);
        }

        // Take NFT Farm Badge from related Vault to call an external NFT Farm Component.
        fn nft_farm_bdg_take(&mut self) -> Bucket {
            match self.nft_farm_badge_vault.get_mut(&self.nft_farm_comp) {
                Some(vault) => vault.take(Decimal::one()),
                None => {
                    info!(" [nft_farm_bdg_take] NFT Farm Badge not in stock! ");
                    std::process::abort()
                }
            }
        } 
    }
}

