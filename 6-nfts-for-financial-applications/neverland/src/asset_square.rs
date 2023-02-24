use scrypto::prelude::*;

use crate::data_square::*;
use crate::calls::*;
use crate::info::*;
use crate::maps::*;

blueprint! {
    struct AssetSquare {  
        // Map of external Caller Badge resource addresses and relative external cloned component 
        // address authorized to purchase assets within protocol. Nedeed to guarantee external
        // caller component is a cloned one so SBT property updates will be correctly executed. 
        ext_component_map: HashMap<ResourceAddress,ComponentAddress>,
        // Vault to stock SBT Updater Badge
        sbt_updater_badge: Vault,
        // SBT Updater Badge resource address
        sbt_updater_badge_addr: ResourceAddress,
        // User SBT Resource Address.
        user_sbt: ResourceAddress,       
        // tkn vault.
        collected_tkn: Vault,         
        // NFT Hashmap of vaults.                                   
        nft_vaults: HashMap<ResourceAddress,Vault>,   
        // Token vault inherent to a specific Asset NFT selling instance number
        tkn_vaults: HashMap<u128,Vault>,
        // External cloned marketplace component authentication Caller Badge vault 
        badge_vaults: HashMap<ResourceAddress,Vault>,
        // nft Hashmap with nft data, nft key, nft price. Used to return NFT availability within protocol.
        nft_map: HashMap<ResourceAddress,Vec<(u128,NonFungibleId,Decimal,bool)>>,
        // NFT Hashmap with address, total accrued selling amount, NFT & metaNFT keys, NFT accrued selling amount.
        meta_map: HashMap<ResourceAddress,(Decimal,Vec<(NonFungibleId,NonFungibleId,Decimal,u128)>)>,         
        // metanft Hashmap with NFT Address & metaNFT ResourceAddress correspondence. 
        meta: HashMap<ResourceAddress,ResourceAddress>,  
        // Map of external cloned marketplaces to ensure interoperability between cloned protocols. 
        // External marketplace component address & caller badge resource address, external marketplace fee,        
        // dex component resource address, badge resource address to call external marketplaces methods
        ext_mrkt_map: Vec<(ComponentAddress,ResourceAddress,Decimal,ResourceAddress,ResourceAddress)>,        
        // Badge to mint and burn metaCandies.                      
        minter_badge: Vault,         
        // Owner badge to determine protocol fee and collect accrued tkn fee.                                     
        owner_badge: ResourceAddress, 
        // AssetSquare protocol Badge resource amount
        asset_square_badge: ResourceAddress, 
        // Protocol currency resource address
        currency: ResourceAddress,
        // AssetSquare component address
        asset_square_comp_addr: ComponentAddress,
        // Protocol fee variable.
        fee: Decimal,
        // Maps component containing BuyerBadge maps data relative to different selling instance
        // modes: normal mode with a "buy proposition" map, auction mode and raffle mode with
        // relative maps  
        maps: MapsComponent,
        // Tkn struct containing relevant data utilized by protocol
        tkn: Tkn,
        // Asset NFT selling instance number
        instance_number: u128,
        // Asset NFT selling instance data Hashmap:
        //   Key                    = (seller Badge Address, instance number),
        //   Value.0                = (NFT resource address, NFT id, NFT data), 
        //   Value.1                = (Selling status flag, profit amount), 
        //   Value.2 (Normal mode)  = (price, buy offer amount, deadline, unused data(Decimal,u64,u8,u128)).
        //   Value.2 (Auction mode) = (reserve price, highest bid, deadline, bid bond, last minute bid deadline,
        //                             unused data(u8,u128)).
        //   Value.2 (Raffle mode)  = (reserve price, ticket price, deadline, unused data(Decimal), 
        //                             last minute bid deadline, tickets amount, winner ticket id).        
        list_map: HashMap<
            (ResourceAddress,u128),
            (
                Vec<(ResourceAddress,NonFungibleId,AssetNFT)>,
                (u8,Decimal),
                (Decimal,Decimal,u64,Decimal,u64,u8,u128)
            )
        >
    }

    impl AssetSquare {
        pub fn new(
            sbt_updater_badge_addr: ResourceAddress,    // Land Data protocol's SBT updater Badge resource address
            land_data_owner_badge: ResourceAddress,     // Land Data protocol's Owner Badge resource address
            user_sbt: ResourceAddress,                  // Land Data protocol's registered users SBT resource address
            fee: Decimal,                               // Protocol fee percentile amount
            currency: ResourceAddress,                  // Protocol currency resource address
            dex: ComponentAddress                       // DEX component address
        ) -> (ComponentAddress,Bucket,Bucket) {
            let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " MinterBadge ")
                .initial_supply(Decimal::one());

           let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " OwnerBadge ")
                .initial_supply(Decimal::one());

            let asset_square_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " AssetBadge ")
                .initial_supply(Decimal::one()); 

            let access_rules = AccessRules::new()
                .method("stock_sbt_updater_badge", rule!(require(land_data_owner_badge)))
                .method("set_comp_addr", rule!(require(owner_badge.resource_address())))
                .method("reset_asset_square_values", rule!(require(owner_badge.resource_address())))
                .method("set_fee", rule!(require(owner_badge.resource_address())))
                .method("set_deadlines", rule!(require(owner_badge.resource_address())))
                .method("claim_tkn_fee", rule!(require(owner_badge.resource_address())))
                .method("add_ext_mrkt", rule!(require(owner_badge.resource_address())))
                .method("remove_ext_mrkt", rule!(require(owner_badge.resource_address())))
                .method("view_ext_mrkt_map", rule!(require(owner_badge.resource_address())))
                .method("stock_badge", rule!(require(owner_badge.resource_address())))
                .method("modify_dex_address", rule!(require(owner_badge.resource_address())))
                .method("buy_prop_badge_map", rule!(require(owner_badge.resource_address())))
                .method("auction_badge_map", rule!(require(owner_badge.resource_address())))
                .method("raffle_badge_map", rule!(require(owner_badge.resource_address())))
                .method("meta_map_check_all", rule!(require(owner_badge.resource_address())))
                .method("meta_check_all", rule!(require(owner_badge.resource_address())))
                .method("set_academy_values", rule!(require(owner_badge.resource_address())))
                .method("insert_ext_sbt_map", rule!(require(owner_badge.resource_address())))
                .method("view_ext_sbt_map", rule!(require(owner_badge.resource_address())))
                .default(rule!(allow_all));
           
            let mut tkn_square: AssetSquareComponent = Self {
                ext_component_map: HashMap::new(),
                sbt_updater_badge: Vault::new(sbt_updater_badge_addr),
                sbt_updater_badge_addr,
                user_sbt,
                collected_tkn: Vault::new(currency.clone()),
                nft_vaults: HashMap::new(),
                tkn_vaults: HashMap::new(),
                badge_vaults: HashMap::new(),
                nft_map: HashMap::new(),
                meta_map: HashMap::new(),
                meta: HashMap::new(),
                ext_mrkt_map: Vec::new(),
                minter_badge: Vault::with_bucket(minter_badge),
                owner_badge: owner_badge.resource_address(),
                asset_square_badge: asset_square_badge.resource_address(),
                currency,
                asset_square_comp_addr: dex,
                fee, 
                maps: MapsComponent::new(),
                tkn: Tkn::new(dec!("0"),dec!("0"),dex,currency),
                instance_number: 1,
                list_map: HashMap::new()
            }
            .instantiate();
            tkn_square.add_access_check(access_rules);
            
            (tkn_square.globalize(),owner_badge,asset_square_badge)
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

        // Stock NFT function callable by an end user wishing to list a NFT on sale on protocol.
        // Args detail:
        // reserve_prc:         reserve price in auction or minimum number of tickets in raffle
        pub fn stock_nft(
            &mut self, 
            mut nft_bckt_sum: Vec<Bucket>,          // Vector of buckets of AssetNFTs to place on sale  
            badge: Bucket,                          // Seller badge proof if any
            price: Decimal,                         // selling instane price
            auction: bool,                          // auction mode flag
            raffle: bool,                           // raffle mode flag
            reserve_prc: Decimal,                   // see above 
            time: u64,                              // auction or raffle mode duration
            bid_bond: Decimal,                      // bid bond amount in auction mode
            start_prc: Decimal,                     // raffle's ticket price or auction's floor price
            land_owner_sbt: Proof                   // SBT as proof of AssetNFT's ownership
        ) -> (Vec<Bucket>,Bucket,Bucket) {
            // check AssetNFT ownership with provided SBT proof
            nft_bckt_sum = self.check_ownership(nft_bckt_sum, land_owner_sbt);
            let bdg_addr = badge.resource_address();            
            let nmbr = self.instance_number; 
            self.instance_number += 1;  
            instance_number(nmbr);                 

            let mut meta_bckt: Vec<Bucket> = Vec::new();
            let badge_evo: Bucket;
            let mut tab = Tab::new();
            let mut vec_data = tab.tuple.1.0.clone();
            let mut check_auction_map = false;
            let mut check_raffle_map = false;
            let mut j = 0;
            for nft in nft_bckt_sum.into_iter() {
                let addr = nft.resource_address();
                let key = nft.non_fungible::<AssetNFT>().id();
                let amnt = nft.amount();
                if j == 0 {
                    vec_data.clear();
                }

                assert!(
                    !auction && !raffle || auction && !raffle || !auction && raffle,
                    "[stock_nft]:Check data "
                );
                if !auction && !raffle {
                    assert!(
                        price > Decimal::zero(),
                        "[stock_nft]:New price must be a positive value "
                    );
                } else {
                    assert!(
                        price == Decimal::zero(),
                        "[stock_nft]:In selected mode NFT price must be Asset to zero "
                    );
                    assert!(time <= self.tkn.auction_dl," Check auction duration ");
                    if auction && !raffle {
                        assert!(
                            bid_bond <= reserve_prc/Decimal::from("5"),
                            "[stock_nft]:Max bid bond allowed is 20% reserve price 
                        ");
                    } else {
                        assert!(
                            start_prc <= reserve_prc/Decimal::from("100"),
                            "[stock_nft]:100 or more tickets are required within a raffle instance"
                        );
                    }
                }
                let eco = 
                    borrow_resource_manager!(nft.resource_address()).metadata()["Ecosystem"].clone();
                let series = 
                    borrow_resource_manager!(nft.resource_address()).metadata()["Series"].clone();
                let nr = 
                    borrow_resource_manager!(nft.resource_address()).metadata()["Number"].clone();
            
                // Retrieve NFT data
                let data: AssetNFT = nft.non_fungible().data();
                vec_data.push((addr,key.clone(),data.clone()));

                let mut v: Vec<(u128,NonFungibleId,Decimal,bool)> = Vec::new();
    
                if self.nft_map.contains_key(&addr) {
                    match self.nft_map.get_mut(&addr) {
                        Some(v) => {
                            v.push((nmbr,key.clone(),price,true));
                            let vault = self.nft_vaults.get_mut(&addr).unwrap();
                            vault.put(nft);
                            check_auction_map = true;
                            check_raffle_map = true;
                        }
                        _ => unfound(7)
                    }
                } else {
                    let vault = self.nft_vaults.entry(addr).or_insert(Vault::new(addr));
                    vault.put(nft);

                    v.push((nmbr,key.clone(),price,true));
                    self.nft_map.insert(addr,v);

                    let mut v_key: Vec<(NonFungibleId,NonFungibleId,Decimal,u128)> = Vec::new();
                    v_key.push((key.clone(),key.clone(),dec!("0"),nmbr));
             
                    self.meta_map.entry(addr) 
                                 .and_modify(|z| z.1.append(&mut v_key))                     
                                 .or_insert((Decimal::zero(),v_key));          

                    self.add_meta_nft(eco.clone(), series.clone(), nr.clone(), addr);
                }
                stock(amnt, addr, eco, series, nr, key.clone(), data.clone(), price);
                
                // Mint metaNFT and insert relative hashmaps values.
                let meta_nft = self.meta_mint(addr, data, key, nmbr);

                meta_bckt.push(meta_nft);
                j += 1;
            }
            self.tkn_vaults.insert(nmbr,Vault::new(self.currency));

            // Check and update same NFT past auction or past raffle data within relative maps 
            if check_auction_map && auction {   
                self.update_auction_map(nmbr);
            } else if check_raffle_map && raffle { 
                self.update_raffle_map(nmbr); 
            }

            let end = Runtime::current_epoch()+time;
            let dl =end+self.tkn.last_bid_dl;
            let mut tup_two = tab.tuple.1;
                
            if self.check_badge_list(bdg_addr) {                 
                let tup_one = (bdg_addr, nmbr);
                tup_two.0 = vec_data.clone(); 
                if !auction && !raffle {  
                    tup_two.2 = (price,dec!("0"),0,dec!("0"),0,0,0);  
                } else if !raffle {
                    tup_two.1.0 = 3;  
                    tup_two.2 = (reserve_prc,start_prc,end,bid_bond,dl,0,0);     
                } else {  
                    tup_two.1.0 = 8;
                    tup_two.2 = (reserve_prc,start_prc,end,dec!("0"),dl,0,0);
                }  
                tab = Tab { tuple:(tup_one,tup_two)};
                self.map_insert_list(tab);
                badge_evo = Bucket::new(RADIX_TOKEN);
            } else {
                // if seller hasn't provide one, mint a seller badge   
                badge_evo = self.add_badge(
                    vec_data.clone(), 
                    price,
                    auction,
                    raffle,
                    reserve_prc,
                    start_prc,
                    end,
                    bid_bond,
                    dl,
                    nmbr
                );
            }

            (meta_bckt,badge,badge_evo)
        }

        // Method callable to:
        //      1_Modify Asset NFT selling price of an open istance in Normal selling mode.
        //      2_Restart a deserted Auction or an unhounored one without performing NFT unstocking 
        //        as well as restocking operations. 
        //      3_Restart a Raffle whom necessary number of tickets results unreached without
        //        performing NFT unstocking as well as restocking operations. 
        // Seller must provide correspondent Meta Asset NFT received in exchange for NFT listed on 
        // sale and deposited within protocol.
        // A Seller Badge proof is required as well.
        //
        // Args detail:
        // reserve_prc:         reserve price in auction or minimum number of tickets in raffle
        //
        // price          auction mode => start price 
        //                raffle mode => ticket price  
        //                normal mode (modify price) => new price
        //
        // flag           0 => restart auction, 
        //                1 => restart raffle, 
        //                2 => modify price
        //
        pub fn restart_modify(
            &mut self,
            meta_nft_bckt_sum: Vec<Bucket>,         // Vector of buckets of metaAssetNFTs to return 
            badge: Proof,                           // seller badge proof if any
            reserve_prc: Decimal,                   // reserve price
            time: u64,                              // duration
            bid_bond: Decimal,                      // bid bond in auction mode
            price: Decimal,   
            flag: u8           
        ) -> (Vec<Bucket>,Bucket) {
            let mut past_bbond = dec!("0");
            let mut past_rprc = dec!("0");
            let mut past_wbid = dec!("-1");
            let mut meta_bckt_vec: Vec<Bucket> = Vec::new();                                                 
            let mut new_nft_vec: Vec<AssetNFT> = Vec::new();
            let mut i = 0;
            for _nft in meta_nft_bckt_sum.iter() {
                let new_nft: AssetNFT = meta_nft_bckt_sum.get(i).unwrap().non_fungible().data();
                new_nft_vec.push(new_nft);
                i += 1;
            }
            let nmbr: u128;
            let mut onr = 0;
            let mut nft_vec: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();
            let mut tab = Tab::new(); 
            let tup_two = tab.tuple.1;
            let badge: ValidatedProof = badge.unsafe_skip_proof_validation(); 

            if flag == 2 {  
                let mut tup = self.check_meta(meta_nft_bckt_sum,true,0,1,price);
                nmbr = tup.m.3;
                let tup_one = (badge.resource_address(), nmbr);
                nft_vec.append(&mut tup.m.1);
                let chk = self.status_check(nmbr);
                assert!(chk.t.0 == 0," Unauthorized operation ");
                tab = Tab { tuple:(tup_one,tup_two)};
                self.modify(tab,price);
                new_price(price);
            } else {       
                nmbr = self.instance_number;
                self.instance_number += 1;  
                let tup_one = (badge.resource_address(), nmbr);
                tab = Tab { tuple:(tup_one,tup_two)};
                instance_number(nmbr);
                let mut tup = self.check_meta(meta_nft_bckt_sum,true,nmbr,0,dec!("0"));
                nft_vec.append(&mut tup.m.1);
                onr = tup.m.4;
                assert!(tup.m.5 == 7 || tup.m.5 == 5 || tup.m.5 == 10," Unauthorized operation "); 
                self.tkn_vaults.insert(nmbr,Vault::new(self.currency));
                let dl = Runtime::current_epoch()+time+self.tkn.last_bid_dl;

                let bdg = badge.resource_address();
                let rd = (bdg,tup.m.4,self.tkn.auction_dl,dl,flag,reserve_prc,time,bid_bond,price);
                (past_bbond,past_rprc,past_wbid) = self.restart(tab,RstData { t: rd });
            }

            let mut j = 0;
            for (address,key) in nft_vec.clone().into_iter() {
                let new_nft = new_nft_vec.get(j).unwrap();     
                let meta_nft = self.meta_mint(address, new_nft.clone(), key.clone(), nmbr);    
                meta_bckt_vec.push(meta_nft);
                j += 1; 
            }

            if flag == 2 {
                (meta_bckt_vec,Bucket::new(RADIX_TOKEN))
            } else {
                self.update_auction_map(nmbr);
                if flag == 0 && past_bbond > dec!("0") && past_wbid >= past_rprc {
                    (meta_bckt_vec,self.tkn_collect(onr, past_bbond))
                } else {
                    (meta_bckt_vec,Bucket::new(RADIX_TOKEN))
                }
            }
        }

        // Unstock NFT function callable by an end user wishing to withdraw owned NFT from protocol.
        pub fn unstock_nft(
            &mut self, 
            mut meta_nft_bckt_sum: Vec<Bucket>,      // Vector of buckets of metaAssetNFTs to return
            user_sbt: Proof                          // seller's SBT proof
        ) -> (Vec<Bucket>,Bucket) {
            meta_nft_bckt_sum = self.sbt_update(meta_nft_bckt_sum, false, user_sbt);
            let tup = self.check_meta(meta_nft_bckt_sum,true,0,0,dec!("0"));
            let chk = self.status_check(tup.m.3);
            assert!(
                chk.t.0 == 0 || chk.t.0 == 5 || chk.t.0 == 7 || chk.t.0 == 10,
                "[unstock_nft]:Unauthorized operation "
            );           
            self.unstock_list(tup.m.3);

            // Erase relative hashmaps values from map & collect NFT 
            let mut nft_bckt: Vec<Bucket> = Vec::new();
            for (nft_address,vec_key) in tup.m.0.into_iter() {
                for key in vec_key {
                    picked(nft_address,key.clone());
                    nft_bckt.push(self.nft_take(nft_address,key,false)); 
                }
            }    

            (nft_bckt,self.bid_bond(chk.t.0,chk.t.4,chk.t.1,chk.t.2,tup.m.3))
        }

        // Method callable by NFT seller to collect payment received by NFT selling
        // providing a relative user Badge as reference.
        pub fn collect_payment(
            &mut self, 
            mut meta_nft_bckt_sum: Vec<Bucket>,     // Vector of buckets of metaAssetNFTs to return
            user_sbt: Proof                         // seller's SBT proof
        ) -> Bucket {
            meta_nft_bckt_sum = self.sbt_update(meta_nft_bckt_sum, true, user_sbt);
            let tup = self.check_meta(meta_nft_bckt_sum,false,0,0,dec!("0"));
            let chk = self.status_check(tup.m.3);        
            assert!(chk.t.0 == 1,"[collect_payment]:Unauthorized operation "); 
            let accrued_token_bckt = self.tkn_vaults.get(&tup.m.3).unwrap();
            collect_payment(accrued_token_bckt.amount(),chk.t.9);

            self.tkn_collect(tup.m.3, chk.t.9)
        }

        // Method callable by NFT seller to collect payment received by NFT selling
        // providing a relative user Badge as reference.
        // Seller accept buy proposal payment made by buyer, the latter is then able to collect his
        // NFT. If, in the meantime, a higher proposal has been made, protocol detect it and assign 
        // NFT withdrawal right accordingly. 
        pub fn collect_buy_prop_payment(
            &mut self, 
            mut meta_vec: Vec<Bucket>,              // Vector of buckets of metaAssetNFTs to return
            sum: Decimal,                           // buy proposal amount
            user_sbt: Proof                         // seller's SBT proof
        ) -> Bucket { 
            meta_vec = self.sbt_update(meta_vec, true, user_sbt);
            let tup = self.check_meta(meta_vec,false,0,0,dec!("0"));               
            let chk = self.status_check(tup.m.3);
            assert!(chk.t.0 == 0,"[collect_buy_prop_payment]:Unauthorized operation ");        

            let (caller_bdg_addr,new_badge,max_value) = self.maps.collect_buy_proposal(tup.m.3,sum);

            self.switch_badge(new_badge,0,tup.m.3,dec!("0"));
            self.switch_status(tup.m.3,0);

            // Collect $TKN from Vault and hold back Protocol fees
            let tkn_bckt = self.tkn_collect(tup.m.3, max_value);
            let (_rest,output_bckt) = self.take_fee(tup.m.3,max_value,tkn_bckt,caller_bdg_addr,0);

            output_bckt
        }

        // Collect payment of a Asset NFT selling instance in Auction mode 
        pub fn collect_auction_payment(
            &mut self, 
            mut meta_nft_bckt_sum: Vec<Bucket>,     // Vector of buckets of metaAssetNFTs to return
            user_sbt: Proof                         // seller's SBT proof
        ) -> Bucket {   
            meta_nft_bckt_sum = self.sbt_update(meta_nft_bckt_sum, true, user_sbt);                                        
            let tup = self.check_meta(meta_nft_bckt_sum,false,0,0,dec!("0"));
            let chk = self.status_check(tup.m.3);

            assert!(chk.t.0 == 6,"[collect_auction_payment]:Unauthorized operation ");    

            let caller_badge_addr = self.maps.collect_auction_payment(tup.m.3);   

            // Collect $TKN from Vault and hold back Protocol fees
            let tkn_bckt = AssetSquare::tkn_collect(self, tup.m.3, chk.t.2);
            let (_rest,output_bckt) = 
                self.take_fee(tup.m.3,chk.t.2,tkn_bckt,caller_badge_addr,0);

            output_bckt
        }

        // Collect payment of a Asset NFT selling instance Raffle mode  
        pub fn collect_raffle_jackpot(
            &mut self, 
            mut meta_nft_bckt_sum: Vec<Bucket>,     // Vector of buckets of metaAssetNFTs to return 
            user_sbt: Proof                         // seller's SBT proof
        ) -> Bucket {
            meta_nft_bckt_sum = self.sbt_update(meta_nft_bckt_sum, true, user_sbt);
            let tup = self.check_meta(meta_nft_bckt_sum,false,0,0,dec!("0"));
            let chk = self.status_check(tup.m.3);
            assert!(chk.t.0 == 9,"[collect_raffle_jackpot]:Unauthorized operation ");    
            amount(chk.t.7);

            let (caller_badge_addr,new_badge,flag) = self.maps.collect_jackpot(tup.m.3);            
            
            if flag == 3 {  
                 self.erase_map_entry(tup.m.3, chk.t.3, 0);
            } else if flag == 1 {

            // Switch Badge addresses data within user Badge HashMap & update NFT selling status
                self.switch_badge(new_badge,1,chk.t.8,dec!("0"));
            } else if flag == 2 { 
                self.erase_map_entry(tup.m.3, 0, 4);
            }

            // Collect $TKN from Vault and hold back Protocol fees
            let tkn_bckt = self.tkn_collect(tup.m.3, chk.t.7);
            let (_rest,output_bckt) = 
                self.take_fee(tup.m.3,chk.t.7,tkn_bckt,caller_badge_addr,0);
            
            output_bckt
        }

        // Given a AssetNft selling instance number, retrieve price.
        pub fn get_nft_price(&mut self, sale_nr: u128) -> Decimal { 

            self.get_nft_price_ext(sale_nr,self.tkn.dex)
        }

        // Given a AssetNft selling instance number as well as marketplace component address where  
        // AssetNFT is listed on sale, retrieve price.
        pub fn get_nft_price_ext(&mut self, sale_nr: u128, mrkt_addr: ComponentAddress) -> Decimal {  
            let matched = self.nft_match(sale_nr,false);
            if matched.n.0 {     
                matched.n.2
            } else {
                let extmrkt = self.ext_mrkt_data(mrkt_addr,ResourceAddress::from(RADIX_TOKEN));
                let amnt = get_nft_price(sale_nr,extmrkt.tuple.0);
                requested_amount(amnt);
                get_token_sell_amount(amnt,self.tkn.dex,extmrkt.tuple.3,self.currency)
            }
        }

        // Buy an AssetNFT. Method callable within external authorized cloned marketplace 
        pub fn buy_nft_ext(    
            &mut self,       
            sale_nr: u128,                          // sale instance number
            mrkt_addr: ComponentAddress,            // external marketplace component address
            tkn_bckt: Bucket,                       // payment
            bdg_ref: Proof,                         // buyer badge proof
            user_sbt: Proof                         // buyer's SBT proof
        ) -> (Vec<Bucket>,Bucket) {
            if !self.nft_match(sale_nr,false).n.0 {
                let (extmrkt,caller_badge,dex) = self.check_buy(mrkt_addr);
                let bckt = self.bckt_fx(dec!("0"),extmrkt.tuple.3,dex,tkn_bckt);
                dex_output_amount(bckt.amount().clone());
                let proof = caller_badge.create_proof();
                let no_sbt = caller_badge.create_proof();
                let (vec_bckt,tkn_bckt) = 
                    buy_nft_ext(sale_nr,extmrkt.tuple.0,bckt,proof,no_sbt);   
                self.caller_bdg_put(caller_badge);                              
                (self.check_sbt(vec_bckt,user_sbt),tkn_bckt)
            } else {

                self.buy_nft(sale_nr,tkn_bckt,bdg_ref,user_sbt)
            }
        } 

        // Buy an AssetNFT. Method callable within AssetSquare protocol
        pub fn buy_nft(
            &mut self, 
            sale_nr: u128,                              // sale instance number
            mut tkn_bckt: Bucket,                       // payment 
            bdg_ref: Proof,                             // buyer badge proof
            user_sbt: Proof                             // buyer SBT proof
        ) -> (Vec<Bucket>,Bucket) { 
            let matched = self.nft_match(sale_nr,false);
            assert_eq!(matched.n.0,true);  
            let chk = self.check_status(sale_nr,2);
            assert!(chk.t.0 == 0," NFT not on sell ");
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();
            
            let mut bdg_addr = ResourceAddress::from(RADIX_TOKEN);  
            if tkn_bckt.resource_address() != self.currency {
                tkn_bckt = self.swap_fx(matched.n.2,self.currency,self.tkn.dex,tkn_bckt);
                bdg_addr = bdg_ref.resource_address();
            } 
            requested_amount(matched.n.2);      
            assert!( matched.n.2 <= tkn_bckt.amount(), " Not enough tkn input ");
            
            let (rest,tkn_bckt) = self.take_fee(sale_nr,matched.n.2,tkn_bckt,bdg_addr,1);
            display_rest(rest);
                
            // Update NFT status within user Badge HashMap
            self.buy_nft_list(sale_nr,rest);
                
            let mut output_vec_bckt: Vec<Bucket> = Vec::new(); 
            let addr_key_map = self.check_meta_id(sale_nr);
            for (nft_address,vec_key) in addr_key_map.into_iter() {
                for key in vec_key {
                    picked(nft_address,key.clone());
                    output_vec_bckt.push(self.nft_take(nft_address, key, true));
                }
            } 

            (self.check_sbt(output_vec_bckt,user_sbt),tkn_bckt) 
        }

        // Make a buy proposal on a listed Asset NFT selling istance.
        // Method callable within external authorized cloned marketplace 
        pub fn buy_proposal_ext(
            &mut self, 
            sale_nr: u128,                              // sale instance number                              
            mrkt_addr: ComponentAddress,                // external marketplace component address
            tkn_bckt: Bucket,                           // payment
            prop: Decimal,                              // buy proposal amount
            endtime: u64,                               // buy proposal duration
            bdg_ref: Proof                              // buyer badge proof
        ) -> (Bucket,Bucket) {
            if !self.nft_match(sale_nr,false).n.0 {
                let (extmrkt,caller_badge,dex) = self.check_buy(mrkt_addr);
                let bckt = self.bckt_fx(prop,extmrkt.tuple.3,dex,tkn_bckt);
                dex_output_amount(bckt.amount().clone());
                let proof = caller_badge.create_proof();
                let (a,b) = buy_prop_ext(sale_nr,extmrkt.tuple.0,bckt,prop,endtime,proof);
                self.caller_bdg_put(caller_badge);

                (a,b)
            } else {

                self.buy_proposal(sale_nr, tkn_bckt, prop, endtime, bdg_ref)
            }          
        } 

        // Make a buy proposal on a listed Asset NFT selling istance.
        // Method callable within AssetSquare protocol
        pub fn buy_proposal(
            &mut self, 
            sale_nr: u128,                              // sale instance number
            mut tkn_bckt: Bucket,                       // payment
            prop: Decimal,                              // buy proposal amount
            endtime: u64,                               // buy proposal duration      
            bdg_ref: Proof                              // buyer badge proof
        ) -> (Bucket,Bucket) {
            assert_eq!(self.nft_match(sale_nr,false).n.0,true); 
            assert!(endtime <= self.tkn.buy_prop_dl, " Please provide a valid deadline! "); 
            let chk = self.check_status(sale_nr,2);                                         
            assert!(chk.t.0 == 0," NFT not on sell anymore ");
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();
            assert!(bdg_ref.amount() == dec!("1")," Badge proof check failed ");

            // Check if provided currency is requested one otherwise swap it
            if tkn_bckt.resource_address() != self.currency {
                tkn_bckt = self.swap_fx(prop,self.currency,self.tkn.dex,tkn_bckt);
            } 

            // Update NFT status within user Badge HashMap
            let (end,flag) = self.buy_prop_list(chk.t.8,prop,endtime);

            // Update current higher proposal in related map reseting previous proposal values.
            // Return related badge resource address if call is made by an external marketplace.
            // Return a Badge if proposal is acceptable otherwise return an empty bucket.
            let out_bckt: Bucket;
            let mut bdg_addr = ResourceAddress::from(RADIX_TOKEN);
            if flag == 1 {  
                self.maps.buy_proposal_ext(chk.t.8);
                let extmrkt = self.ext_mrkt_data(self.tkn.dex,bdg_ref.resource_address());
                if extmrkt.tuple.4 {    
                    bdg_addr = extmrkt.tuple.1; 
                } 
                out_bckt = 
                    self.add_buyer_badge(prop,dec!("0"),end,0,dec!("0"),bdg_addr,flag,chk.t.8);

                // Put amount of NFT Buy Proposal into related vault.
                self.tkn_put(sale_nr, tkn_bckt.take(prop));
            } else {
                out_bckt = Bucket::new(RADIX_TOKEN);
            }
            
            (out_bckt,tkn_bckt)      
        } 

        // Reclaim a buy proposal on a listed Asset NFT selling istance.
        // User needs to provide his Buyer Badge as well as his SBT proof if his previously made
        // buy proposal has been accepted by buyer. Otherwise he's entitled to retire his offer 
        // if one of next conditions are met:
        // An higher buy proposal has been made.
        // Buyer retired Asset NFT from sale.
        // Buy proposal deadline expired.
        // Method callable within AssetSquare protocol as well as external authorized cloned marketplace
        pub fn reclaim_buy_proposal(&mut self, ex_badge: Bucket, user_sbt: Proof) -> Vec<Bucket> { 
            let (ex_badge,nmbr,mrkt_addr) = AssetSquare::buy_bdg_data(ex_badge); 
            if self.badge_in(nmbr,ex_badge.resource_address(),0) { 
                let ex_badge_addr = ex_badge.resource_address();
                let mut output_vec_bckt: Vec<Bucket> = Vec::new();
                let (ex_flag,ex_amnt,ex_end) = self.maps.reclaim_prop(nmbr,ex_badge_addr);
                match ex_flag { 
                    0 => {  
                        self.bckt_burn(ex_badge);
                        output_vec_bckt.push(self.tkn_collect(nmbr, ex_amnt));
                    }
                    1 => {
                        if ex_end < Runtime::current_epoch() {
                            self.bckt_burn(ex_badge);
                            output_vec_bckt.push(self.tkn_collect(nmbr, ex_amnt));
                        } else {
                            time_unreached(ex_end);
                            std::process::abort()
                        }
                    }
                    2 => {  
                        // Retrieve instance number, check correspondence & take NFT from relative maps 
                        let (ex_badge,nmbr,_nft_addr) = AssetSquare::buy_bdg_data(ex_badge);
                        let addr_key_map = self.check_meta_id(nmbr);
                        for (nft_address,vec_key) in addr_key_map.into_iter() {
                            for key in vec_key {
                                picked(nft_address,key.clone());
                                output_vec_bckt.push(self.nft_take(nft_address,key,true)); 
                            }
                        } 

                        output_vec_bckt = self.check_sbt(output_vec_bckt,user_sbt);

                        // Burn provided Badge
                        self.bckt_burn(ex_badge);
                    }
                    _ => unfound(2)
                }

                // Remove related NFT buy_proposals from map once verified condition.
                self.maps.remove_prop(nmbr,ex_badge_addr);

                output_vec_bckt
            } else { 
                let (extmrkt,caller_badge,dex) = self.check_rec(mrkt_addr); 
                let no_sbt = caller_badge.create_proof();
                let mut vec_bckt = reclaim_buy_proposal(extmrkt.tuple.0,ex_badge,no_sbt); 
                self.caller_bdg_put(caller_badge);
            
                vec_bckt = self.check_sbt(vec_bckt,user_sbt);
                
                self.swap(extmrkt,vec_bckt,dex) 
            }          
        }

        // Buy a determined number of tickets on a listed Asset NFT selling istance in raffle mode.
        // Method callable within external authorized cloned marketplace
        pub fn buy_ticket_ext(
            &mut self, 
            sale_nr: u128,                              // sale instance number
            mrkt_addr: ComponentAddress,                // external marketplace component address
            tkn_bckt: Bucket,                           // payment
            sum: u8,                                    // number of tickets to buy
            bdg_ref: Proof                              // buyer badge proof
        ) -> (Bucket,Bucket) {
            if !self.nft_match(sale_nr,false).n.0 {
                let (extmrkt,caller_badge,dex) = self.check_buy(mrkt_addr);
                let bckt = self.bckt_fx(dec!("0"),extmrkt.tuple.3,dex,tkn_bckt);
                dex_output_amount(bckt.amount().clone()); 
                let proof = caller_badge.create_proof();
                let (a,b) = buy_ticket_ext(sale_nr,extmrkt.tuple.0,bckt,sum,proof);
                self.caller_bdg_put(caller_badge);

                (a,b)
            } else {

                self.buy_ticket(sale_nr, tkn_bckt, sum, bdg_ref)
            }          
        } 

        // Buy a determined number of tickets on a listed Asset NFT selling istance in raffle mode.
        // Method callable within AssetSquare protocol
        pub fn buy_ticket(
            &mut self, 
            sale_nr: u128,                                  // sale instance number
            mut tkn_bckt: Bucket,                           // payment
            sum: u8,                                        // number of tickets to buy
            bdg_ref: Proof                                  // buyer badge proof
        ) -> (Bucket,Bucket) { 
            assert_eq!(self.nft_match(sale_nr,false).n.0,true);
            let mut chk = self.check_status(sale_nr,1);
            assert!(chk.t.0 == 8," NFT not on Raffle ");
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();
            assert!(bdg_ref.amount() == dec!("1")," Badge proof check failed ");

            // Check if provided currency is requested one otherwise swap it
            if tkn_bckt.resource_address() != self.currency {
                tkn_bckt = self.swap_fx(chk.t.2*sum,self.currency,self.tkn.dex,tkn_bckt);
            }   
            let amnt = tkn_bckt.amount();  
            assert!(amnt/sum >= chk.t.2," Check $TKN amount provided "); 

            // Increase Raffle deadline by an Epoch if tickets are purchased within last valid Epoch as
            // long as auction deadline limit ain't outdated
            let mut new_end: bool = false;
            if Runtime::current_epoch() == chk.t.3 && Runtime::current_epoch() < chk.t.5 {
                if sum >= chk.t.6.wrapping_div(20) { 
                    chk.t.3 += 1;
                    new_end = true;
                } 
            } 

            // Update current raffle data in related map and check if ticket badge is present.
            // Return related badge resource address if call is made by an external marketplace.
            // Return a Badge if tickets order is acceptable otherwise return an empty bucket.
            let sum_dec = Decimal::from(sum);
            let ttl_dec = Decimal::from(self.buy_ticket_list(chk.t.8,sum,new_end));
            self.maps.buy_ticket_ext(sale_nr,ttl_dec,chk.t.3,new_end);  
            let output_bckt: Bucket;
            let mut bdg_addr = ResourceAddress::from(RADIX_TOKEN);
            let extmrkt = self.ext_mrkt_data(self.tkn.dex,bdg_ref.resource_address());
            if extmrkt.tuple.4 {    
                bdg_addr = extmrkt.tuple.1; 
            } 
            output_bckt = self.add_buyer_badge(sum_dec,ttl_dec,chk.t.3,2,chk.t.4,bdg_addr,0,chk.t.8);

            // Put amount of NFT Raffles Tickets purchase into related vault.
            AssetSquare::tkn_put(self, sale_nr, tkn_bckt.take(sum_dec*chk.t.2));
                
            (output_bckt,tkn_bckt)
        } 

        // Reclaim won Asset NFT, in raffle mode selling istance, providing required buyer badge
        // containing winner ticket ID.
        // To verify he's the raffle winner a buyer can call "ask_position" method providing his buyer 
        // badge or alternatively check "ask_instance" method if he's not the winner one.
        // If provided badgecontais only loser tickets IDs, protocol burn it.
        // Method callable within AssetSquare protocol as well as external authorized cloned marketplace
        pub fn reclaim_winner_ticket(
            &mut self, 
            ticket_badge: Bucket,                   // buyer's raffle badge  
            user_sbt: Proof                         // buyer's SBT proof
        ) -> Vec<Bucket> { 
            let (ticket_badge,nmbr,mrkt_addr) = AssetSquare::raffle_bdg_data(ticket_badge);
            if self.badge_in(nmbr,ticket_badge.resource_address(),2) {
                let chk = self.status_check(nmbr);
                assert!(chk.t.0 >= 9," Unauthorized operation ");
                let mut output_vec_bckt: Vec<Bucket> = Vec::new();

                let (wave,sum,tup) = 
                    self.maps.reclaim_ticket(nmbr,ticket_badge.resource_address(),chk.t.2);
                match wave {
                    0 => {
                        if chk.t.2*Decimal::from(chk.t.6) < chk.t.1 {
                            self.bckt_burn(ticket_badge);
                            output_vec_bckt.push(self.tkn_collect(nmbr, sum));
                        } else {
                            self.bckt_burn(ticket_badge);
                            output_vec_bckt.push(Bucket::new(RADIX_TOKEN));
                        }
                    }
                    _ => {
                        if tup.0 != 0 {
                            let mut v: Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)> = Vec::new();
                            v.push(tup);
                            self.maps.raffle_map_insert(nmbr,v);
                        }
                        let addr_key_map = self.check_meta_id(nmbr);
                        for (nft_address,vec_key) in addr_key_map.into_iter() {
                            for key in vec_key {
                                picked(nft_address,key.clone());
                                output_vec_bckt.push(self.nft_take(nft_address,key,true)); 
                            }
                        } 

                        output_vec_bckt = self.check_sbt(output_vec_bckt,user_sbt);

                        // Burn provided Badge
                        self.bckt_burn(ticket_badge);
                    }
                }

                output_vec_bckt
            } else {
                let (extmrkt,caller_badge,dex) = self.check_rec(mrkt_addr);
                let no_sbt = caller_badge.create_proof();
                let mut vec_bckt = reclaim_winner_ticket(extmrkt.tuple.0,ticket_badge,no_sbt);
                self.caller_bdg_put(caller_badge);

                if vec_bckt.get(0).unwrap().resource_address() != ResourceAddress::from(RADIX_TOKEN) {
                    vec_bckt = self.swap(extmrkt,vec_bckt,dex);
                }

                self.check_sbt(vec_bckt,user_sbt)
            }          
        }

        // Place a bid on a listed Asset NFT selling istance in auction mode.
        // Method callable within external authorized cloned marketplace
        pub fn place_bid_ext(
            &mut self, 
            sale_nr: u128,                              // sale instance number
            mrkt_addr: ComponentAddress,                // external marketplace component address
            tkn_bckt: Bucket,                           // payment
            bidder_badge: Bucket,                       // buyer bidder badge 
            bid: Decimal,                               // bid amount
            bid_bond: Decimal,                          // bid bond
            bdg_ref: Proof                              // buyer badge proof
        ) -> (Bucket,Bucket,Bucket) {
            if !self.nft_match(sale_nr,false).n.0 {
                let (extmrkt,caller_badge,dex) = self.check_buy(mrkt_addr);
                let bckt = self.bckt_fx(bid_bond,extmrkt.tuple.3,dex,tkn_bckt);
                dex_output_amount(bckt.amount().clone());
                let proof = caller_badge.create_proof();
                let (a,b,c) = place_bid_ext(sale_nr,extmrkt.tuple.0,bckt,bidder_badge,bid,bid_bond,proof);
                self.caller_bdg_put(caller_badge);

                (a,b,c)
            } else {

                self.place_bid(sale_nr,tkn_bckt,bidder_badge,bid,bid_bond,bdg_ref)
            }          
        }   
 
        // Place a bid on a listed Asset NFT selling istance in auction mode.
        // Method callable within AssetSquare protocol
        pub fn place_bid(
            &mut self, 
            sale_nr: u128,                                  // sale instance number
            mut tkn_bckt: Bucket,                           // payment
            bidder_badge: Bucket,                           // buyer bidder badge 
            bid: Decimal,                                   // bid amount
            bid_bond: Decimal,                              // bid bond
            bdg_ref: Proof                                  // buyer badge proof
        ) -> (Bucket,Bucket,Bucket) {
            assert_eq!(self.nft_match(sale_nr,false).n.0,true); 
        
            // Check if NFT is listed in auction mode & provided resources and data are valid
            let mut chk = self.check_status(sale_nr,0);
            assert!(chk.t.0 == 3," NFT not in Auction ");
            assert!(tkn_bckt.amount() >= chk.t.4," Check bid bond amount ");  
            assert!(bid > chk.t.2," An higher bid has been placed yet ");
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();
            assert!(bdg_ref.amount() == dec!("1")," Badge proof check failed ");

            // Check if provided currency is requested one otherwise swap it
            if tkn_bckt.resource_address() != self.currency {
                tkn_bckt = self.swap_fx(bid_bond,self.currency,self.tkn.dex,tkn_bckt);
            }
            
            // Increase Auction deadline by an Epoch if bid is placed within last valid Epoch as
            // long as auction deadline limit ain't outdated
            let mut new_end: bool = false;
            if Runtime::current_epoch() == chk.t.3 && Runtime::current_epoch() < chk.t.5 {
                chk.t.3 += 1;
                new_end = true; 
            } 
    
            // Update NFT status within user Badge HashMap
            let wave = self.place_bid_list(chk.t.8,bid,new_end);

            let mut s = 0;
            if bid >= chk.t.1 {
                s = 1;
            }
                  
            // Update current winning bid in related map and check if bidder badge is present 
            // Return related badge resource address if call is made by an external marketplace.
            // Return a Badge if bid is acceptable otherwise return an empty bucket. 
            let output_bckt: Bucket;
            let mut bdg_addr = ResourceAddress::from(RADIX_TOKEN);
            if self.maps.place_bid(chk.t.8,bidder_badge.resource_address(),s,bid,new_end,wave) {  
                let extmrkt = self.ext_mrkt_data(self.tkn.dex,bdg_ref.resource_address()); 
                if extmrkt.tuple.4 {    
                    bdg_addr = extmrkt.tuple.1; 
                } 
                output_bckt = 
                    self.add_buyer_badge(bid,dec!("0"),chk.t.3,1,chk.t.4,bdg_addr,s,chk.t.8);

            // Put amount of NFT Placed Bid into related vault.
                self.tkn_put(sale_nr, tkn_bckt.take(chk.t.4));
            } else {
                output_bckt = Bucket::new(RADIX_TOKEN);
            }
            
            (output_bckt,tkn_bckt,bidder_badge)  
        }

        pub fn reclaim_bid_bond(
            &mut self, 
            bidder_badge: Bucket                       // buyer bidder badge 
        ) -> Vec<Bucket> {
            let (bidder_badge,nmbr,mrkt_addr) = AssetSquare::buy_bdg_data(bidder_badge);
            if self.badge_in(nmbr,bidder_badge.resource_address(),1) {
                let mut output_vec_bckt: Vec<Bucket> = Vec::new();
                let (bid_bond,answer,winner_flag,burn_badge_flag) = 
                    self.maps.reclaim_bond(nmbr,bidder_badge.resource_address(),self.tkn.auction_dl);
                if answer { 
                    self.bckt_burn(bidder_badge);
                    output_vec_bckt.push(self.tkn_collect(nmbr, bid_bond));
                } else if !winner_flag || !burn_badge_flag {
                    if !winner_flag {
                        unauthorized();
                    }
                    output_vec_bckt.push(bidder_badge);
                } else {
                    self.bckt_burn(bidder_badge);
                    output_vec_bckt.push(Bucket::new(RADIX_TOKEN));
                }

                output_vec_bckt
            } else {
                let (extmrkt,caller_badge,dex) = self.check_rec(mrkt_addr);  
                self.caller_bdg_put(caller_badge);       
                let vec_bckt = reclaim_bid_bond(extmrkt.tuple.0,bidder_badge);

                self.swap(extmrkt,vec_bckt,dex)
            }          
        }

        pub fn pay_winner_bid(
            &mut self, 
            mut tkn_bckt: Bucket,                       // payment 
            bidder_badge: Bucket,                       // buyer bidder badge  
            user_sbt: Proof                             // buyer's SBT proof
        ) -> (Vec<Bucket>,Bucket) {
            let (bidder_badge,nmbr,mrkt_addr) = AssetSquare::buy_bdg_data(bidder_badge);
            if self.nft_match(nmbr,true).n.0 { 
                let chk = self.check_status(nmbr,1);
                assert!(chk.t.0 == 4," NFT not in Auction payment mode ");

                let rest = self.pay_win_bid_list(nmbr,self.tkn.auction_dl);

                // Check if provided currency is requested one otherwise swap it
                if tkn_bckt.resource_address() != self.currency {
                    tkn_bckt = self.swap_fx(rest,self.currency,self.tkn.dex,tkn_bckt);
                }

                // Verify if provided badge is the winner one
                self.maps.pay_winner_bid(nmbr,bidder_badge.resource_address());

                let mut output_vec_bckt: Vec<Bucket> = Vec::new();
                let addr_key_map = self.check_meta_id(nmbr);
                for (nft_address,vec_key) in addr_key_map.into_iter() {
                    for key in vec_key {
                        picked(nft_address,key.clone());
                        output_vec_bckt.push(self.nft_take(nft_address,key,true)); 
                    }
                }   

                self.bckt_burn(bidder_badge);
                self.tkn_put(nmbr, tkn_bckt.take(rest));

                (self.check_sbt(output_vec_bckt,user_sbt),tkn_bckt)
            } else {
                let (extmrkt,caller_badge,dex) = self.check_rec(mrkt_addr);
                let no_sbt = caller_badge.create_proof();
                let bckt = self.bckt_fx(dec!("0"),extmrkt.tuple.3,dex,tkn_bckt);
                dex_output_amount(bckt.amount().clone());
                let (vec_bckt,tkn_bckt) = 
                    pay_winner_bid(extmrkt.tuple.0,bckt,bidder_badge,no_sbt);
                self.caller_bdg_put(caller_badge);

                (self.check_sbt(vec_bckt,user_sbt),tkn_bckt)
            }          
        }

        // Retrieve nft provider position providing a relative userBadge as reference.
        pub fn ask_position(&mut self, badge: Proof) -> Vec<Tab> {
            let badge: ValidatedProof = badge.unsafe_skip_proof_validation();
            let output_vector = self.update_state(0, badge.resource_address());
            for tab in output_vector.clone() {
                position(tab.clone(), self.tkn.auction_dl);
            }
 
            output_vector
        }

        // Retrieve nft selling status providing a relative instance number as reference.
        pub fn ask_instance(&mut self, sale_nr: u128) -> Vec<Tab> { 
            let mut output_vector = self.update_state(sale_nr, ResourceAddress::from(RADIX_TOKEN));  
            for mut tab in output_vector.clone() {
                    if tab.tuple.1.1.0 == 3 || tab.tuple.1.1.0 == 8 {
                        tab.tuple.1.2.0 = dec!(0);
                    } 
                    position(tab.clone(), self.tkn.auction_dl);
                    output_vector.push(tab);
            }
 
            output_vector
        }

        // Providing a Meta AssetNFT, method performs checks to verify his existence within related
        // protocol's map. In case of positive result, instance state inherent to provided asset is
        // updated and resulting data is returned to caller. Method is intended to be call by external 
        // targeted protocols, like lending or gaming protocols whom agree to utilized Meta AssetNFT
        // as substitute of original one, chasing a logic of capital efficiency.   
        pub fn ask_meta_key(&mut self, meta_key: NonFungibleId, addr: ResourceAddress) -> Vec<Tab> {     
            let (mut founded,mut sale_nr,mut nft_addr) = (false,0,ResourceAddress::from(RADIX_TOKEN));
            for (key,value) in self.meta_map.iter() {
                for val in &value.1 {
                    if meta_key == val.1 {
                        (founded,sale_nr,nft_addr) = (true,val.3,*key);
                        break;
                    }
                }
            }
            for (key,value) in self.meta.iter() {
                if addr == *value {
                    assert_eq!(founded == true,nft_addr == *key," Wrong metaNft provided ");
                }
            }
            let mut output_vector = self.update_state(sale_nr, ResourceAddress::from(RADIX_TOKEN));  
            for mut tab in output_vector.clone() {
                    tab.tuple.1.2.0 = Decimal::zero();
                    position(tab.clone(), self.tkn.auction_dl);
                    output_vector.push(tab);
            }
 
            output_vector
        }

        // Get reserve amount of a determinated nft giving his resource address.            
        pub fn get_reserve(&self, nft_addr: ResourceAddress) -> usize {
            let mut total_nft = 0;
            match self.nft_map.get(&nft_addr) {
                Some(v) => { 
                    for val in v {
                        if val.3 {
                            total_nft += 1;
                        }
                    }
                    nft_reserve_amount(nft_addr,total_nft);
                },
                None => unfound(1)
            }

           total_nft 
        }

        // Get protocol's nft menu.                                                         
        pub fn list_nft(&self) -> NftVec {
            let mut vec = Vec::new();
            for (nft_addr,v) in self.nft_map.clone().into_iter() {
                list_nft(nft_addr, v.clone(), self.collected_tkn.resource_address());
                vec.push((nft_addr,v)); 
            }   

            NftVec { nft_vec_map:vec } 
        }

        // Get external marketplace's list of sellable NFTs.                                
        pub fn list_nft_ext(&mut self, nft_addr: ResourceAddress) -> Vec<NftVec> {
            let mut output_vec: Vec<NftVec> = Vec::new();
            for (ext_square,_ext_bdg,_ext_fee,_ext_fx,_bdg) in self.ext_mrkt_map.iter() {
                let nft_vec_map = list_address(*ext_square,nft_addr);
                output_vec.push(nft_vec_map);
            }

            output_vec                    
        }

        // Get protocol's list of NFTs sharing same resource address.                       
        pub fn list_address(&self, nft_addr: ResourceAddress) -> NftVec {
            let mut vec = Vec::new();
            if self.nft_map.contains_key(&nft_addr) {
                match self.nft_map.get(&nft_addr) {
                    Some(v) => {  
                        list_nft(nft_addr, v.clone(), self.collected_tkn.resource_address());
                        vec.push((nft_addr,v.clone()));
                    }
                    None => unfound(0)
                }    
            }

            NftVec {nft_vec_map:vec.clone()}
        }

        // Get external marketplace's list of NFTs sharing same resource address.
        pub fn list_address_ext(&mut self, mrkt: ComponentAddress, nft: ResourceAddress) -> NftVec {
            let extmrkt = self.ext_mrkt_data(mrkt,ResourceAddress::from(RADIX_TOKEN));
            
            list_address(extmrkt.tuple.0,nft)                       
        }

        // Providing a Buyer Badge, method retrieve his data. 
        pub fn buy_badge_data_utils(&mut self, nft: Bucket) -> (Bucket,Mode) {        
            buy_badge_data_test(nft)
        }  

        // Providing a Raffle Buyer Badge, method retrieve his data.
        pub fn raffle_badge_data_utils(&mut self, nft: Bucket) -> (Bucket,TicketID) {        
            raffle_badge_data_test(nft)
        }

        // Providing an AssetNFT, method retrieve his data.
        pub fn nft_data_utils(&mut self, nft: Bucket) -> (Bucket,AssetNFT) {
            nft_data(nft)
        }

        // Providing an AssetNFT and his ID, method retrieve his data.
        pub fn nft_data_key_utils(&mut self, key: NonFungibleId , nft: Bucket) -> (Bucket,AssetNFT) {        
            nft_data_key(key.clone(), nft)
        }

        // Retrieve protocol settings.
        pub fn ask_setting(&mut self) -> Tkn {
            
            settings(self.fee,self.tkn.clone())
        }

        // Retrieve amount of tokens gained by protocol and deposited within his internal vault.
        pub fn ask_tkn_gain(&mut self) -> Decimal {              
            let tkn_output = self.collected_tkn.amount();
            tkn_gains(tkn_output);
            
            tkn_output
        }

        // Retrieve external marketplace currency to perform reverse swap
        pub fn out_currency(&mut self, bdg_ref: Proof) -> ResourceAddress { 
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();               
            let extmrkt = self.ext_mrkt_data(self.tkn.dex,bdg_ref.resource_address());
            assert!(extmrkt.tuple.4," NFT correspondence unfounded ");
            
            extmrkt.tuple.3
        } 

        // ======================================================
        // Admin only extra tools callable by protocol owner only
        // ======================================================

        // Modify external DEX resource address.
        // Callable by protocol owner only.
        pub fn modify_dex_address(&mut self, new_dex_address: ComponentAddress) {      
            self.tkn.dex = new_dex_address;     
        }

        // Add an external marketplace address & fee related to an NFT resource address 
        // to list on sell there too. Mint a Caller Badge to send to that marketplace and relate 
        // it to other data. Callable by protocol owner only.        
        pub fn add_ext_mrkt(
            &mut self, 
            ext_square: ComponentAddress, 
            ext_fee: Decimal,
            ext_fx: ResourceAddress
        ) -> Bucket {
            let (mut founded,mut bdg_addr) = (false,ResourceAddress::from(RADIX_TOKEN));
            let mut badge = bdg_addr;
            for val in self.ext_mrkt_map.iter() {    
                if ext_square == val.0 {
                    (bdg_addr,badge,founded) = (val.1,val.4,true);
                    break;
                }    
            }
            if ext_square != self.tkn.square {
                if founded && bdg_addr != ResourceAddress::from(RADIX_TOKEN) {
                    self.ext_mrkt_map.retain(|x| x.0 != ext_square);
                    self.ext_mrkt_map.push((ext_square,bdg_addr,ext_fee,ext_fx,badge));
                    Bucket::new(RADIX_TOKEN)
                } else {
                    let caller_bdg_res_def = ResourceBuilder::new_fungible()
                        .divisibility(DIVISIBILITY_NONE)
                        .metadata("name","CallerBadge")
                        .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                        .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                        .no_initial_supply(); 

                    if founded && bdg_addr == ResourceAddress::from(RADIX_TOKEN) {
                        self.ext_mrkt_map.retain(|x| x.0 != ext_square);  
                    }

                    self.ext_mrkt_map.push((ext_square,caller_bdg_res_def,ext_fee,ext_fx,badge));
                    let caller_badge = self.minter_badge
                        .authorize(|| { borrow_resource_manager!(caller_bdg_res_def).mint(dec!("1")) 
                    });

                    self.ext_component_map.insert(caller_badge.resource_address(),ext_square);
                    
                    caller_badge
                }
            } else {
                if founded {
                    self.ext_mrkt_map.retain(|x| x.0 != self.tkn.square);
                }
            
                self.ext_mrkt_map.push((
                    self.tkn.square,
                    self.asset_square_badge,
                    self.tkn.fee,
                    self.tkn.currency,badge
                ));
                Bucket::new(RADIX_TOKEN)
            }
        }  

        // Remove external marketplace allowance related a specified NFT resource address. 
        pub fn remove_ext_mrkt(&mut self, ext_square: ComponentAddress) {
            let mut tkn_badge = ResourceAddress::from(RADIX_TOKEN);
            let mut caller_badge = ResourceAddress::from(RADIX_TOKEN);
            let zero = tkn_badge;
            let (mut i,mut found) = (0,false);
            for val in self.ext_mrkt_map.iter_mut() {
                if val.0 == ext_square {
                    (tkn_badge,found,caller_badge) = (val.4,true,val.1);
                    break;
                }    
                i += 1;
            }
            if found {
                self.ext_mrkt_map.remove(i);
                self.ext_component_map.remove(&caller_badge);
                if self.asset_square_comp_addr == self.tkn.square {
                    self.ext_mrkt_map.push((ext_square,zero,dec!("0"),zero,tkn_badge));
                }
            }
            found = false;
            for (key,value) in self.ext_component_map.iter_mut() {
                if *value == ext_square {
                    tkn_badge = *key;
                    found = true;
                    break;
                }
            }
            if found {
                self.ext_component_map.remove(&tkn_badge);
            }
        }

        // Stock External Marketplace badge in relative Vaults Hashmap: AssetSquare Badge for 
        // AssetSquare or CallerBadge for others Markeplace Components.
        pub fn stock_badge(&mut self, ext_square: ComponentAddress, caller_badge: Bucket){
            let mut founded = false;
            for val in self.ext_mrkt_map.iter_mut() {
                if val.0 == ext_square { 
                    val.4 = caller_badge.resource_address();
                    founded = true;
                    break;
                }    
            }
            if !founded { 
                let zero = ResourceAddress::from(RADIX_TOKEN);
                self.ext_mrkt_map
                    .push((ext_square,zero,dec!("0"),zero,caller_badge.resource_address()));
            }
            let vault = self.badge_vaults.entry(caller_badge.resource_address())
                .or_insert(Vault::new(caller_badge.resource_address()));
            vault.put(caller_badge);
        }

        // Init AssetSquare settings in case of external cloned Marketplace Component implementation 
        pub fn set_asset_square_values(                                                                  
            &mut self, 
            asset_square_fee: Decimal, 
            asset_square_royalty: Decimal,
            tkn_address: ResourceAddress,
            asset_square_address: ComponentAddress,
            tkn_vault: ComponentAddress,
            asset_square_bdg: Proof
        ) -> bool {
            asset_square_bdg
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.asset_square_badge,
                    dec!("1"))
                )
                .expect(" [asset_square_square::set_asset_square_values]: Unauthorized ");
            assert!(
                asset_square_royalty >= dec!("0") && asset_square_royalty <= dec!("1"),
                " delta fee 0:1 "
            );            
            
            self.tkn.badge = self.asset_square_badge;
            self.tkn.fee = asset_square_fee;
            self.tkn.royalty = asset_square_royalty;
            self.tkn.currency = tkn_address;
            self.tkn.square = asset_square_address;
            self.tkn.vault = tkn_vault;
            values(
                asset_square_fee,
                asset_square_royalty,
                tkn_address,
                asset_square_address,
                tkn_vault
            );

            self.ext_component_map.insert(self.tkn.badge,self.tkn.square);
        
            true
        } 

        // Reset AssetSquare data values within an external authorized cloned protocol
        pub fn reset_asset_square_values(
            &mut self, 
            fee: Decimal, 
            royalty: Decimal,
            tkn: ResourceAddress,
            asset_square_mrkt: ComponentAddress,
            tkn_vault: ComponentAddress,
            asset_square_badge_addr: ResourceAddress,
            ext_mrkt: ComponentAddress
        ) {
            let badge = self.caller_bdg_take(asset_square_badge_addr);
            assert!(
                reset_asset_square_values(
                    fee,
                    royalty,
                    tkn,
                    asset_square_mrkt,
                    tkn_vault,
                    badge.create_proof(),
                    ext_mrkt
                ),
                " Unable to reset values "
            );
            self.caller_bdg_put(badge);
        }

        // Set Academy Vault Component address & academy share.
        pub fn set_academy_values(
            &mut self, 
            academy_vault: ComponentAddress, 
            academy_share: Decimal
        ) {      
            assert!(academy_share <= Decimal::from(100)," Max allowed value is 100 ");
            (self.tkn.academy_vault,self.tkn.academy_share) = (academy_vault,academy_share);
            academy_values(self.tkn.academy_vault,self.tkn.academy_share);
        }

        // Set protocol fee function whom only protocol owner can succesfully call.
        pub fn set_fee(&mut self, prtcl_fee: Decimal) {
            assert!(prtcl_fee >= dec!("0") && prtcl_fee <= dec!("10")," delta fee 0:10 ");
            self.fee = prtcl_fee;
            protocol_fee(self.fee);
        }

        // Set auction deadline, buy proposal deadline and last bid deadline aka the number of 
        // epochs an auction or a raffle instances are extended whenever a bid or a ticket sell of a
        // determinated amount has occurred within ultimate available epoch.
        pub fn set_deadlines(&mut self, auction: u64, last_bid_deadline: u64, buy_proposal: u64) {
            assert!(last_bid_deadline <= auction/10," Please lower last bid deadline ");
            self.tkn.auction_dl = auction;    
            self.tkn.last_bid_dl = last_bid_deadline;
            self.tkn.buy_prop_dl = buy_proposal;
            deadlines(self.tkn.auction_dl,self.tkn.last_bid_dl,self.tkn.buy_prop_dl);
        }

        // Set AssetSquare component address
        pub fn set_comp_addr(&mut self){
            self.asset_square_comp_addr = Runtime::actor().as_component().0;
        } 

        // Given a certain AssetNFT normal mode selling instance number, method retrieve related data
        // from buy proposition NFT badge map 
        pub fn buy_prop_badge_map(&mut self, sale_nr: u128) -> BuyPropBdgMap { 
            
            self.maps.buy_prop_badge_map(sale_nr)
        } 

        // Given a certain AssetNFT auction mode selling instance number, method retrieve related data
        // from auction NFT badge map 
        pub fn auction_badge_map(&mut self, sale_nr: u128) -> AuctionBdgMap { 

            self.maps.auction_badge_map(sale_nr)   
        } 

        // Given a certain AssetNFT raffle mode selling instance number, method retrieve related data
        // from raffle NFT badge map 
        pub fn raffle_badge_map(&mut self, sale_nr: u128, badge: ResourceAddress) -> RaffleBdgMap {                                                         
            
            self.maps.raffle_badge_map(sale_nr,badge)
        }

        // View authorized external marketplaces.
        pub fn view_ext_mrkt_map(&self) -> ExtMrktVec {
            let mut op_map = Vec::new();
            for val in self.ext_mrkt_map.clone().into_iter() {
                ext_mrkt(val.0,val.1,val.2,val.3,val.4);
                op_map.push(val);
            }
            return ExtMrktVec { map:op_map };
        }

        // Method returs data from correspondence map between NFT & metaNFT. address, total accrued 
        // selling amount, NFT & metaNFT keys, NFT accrued selling amount.
        pub fn meta_map_check_all(&mut self) -> ExtMetaMap {                    
            let mut op_map = HashMap::new();
            for (nft_addr,(sum,v_key)) in self.meta_map.clone().into_iter() {
                for tup in v_key.clone() {
                    meta_map(nft_addr,sum,tup.0,tup.1,tup.3);
                }
                op_map.insert(nft_addr,(sum,v_key));
            }

            ExtMetaMap { map:op_map }
        }

        // Method returs data from map with NFT Address & metaNFT ResourceAddress correspondence.
        pub fn meta_check_all(&mut self) -> Vec<(ResourceAddress,ResourceAddress)> { 
            let mut output_vec: Vec<(ResourceAddress,ResourceAddress)> = Vec::new();
            for (nft_addr,meta_rd) in self.meta.iter() {
                meta(*nft_addr,*meta_rd);
                output_vec.push((*nft_addr,*meta_rd));
            }

            output_vec
        }

        // Display external cloned marketplace components addresses authorized to buy and sell 
        // AssetNFT having an implemented system to succesfully update user's SBT data. 
        pub fn view_ext_sbt_map(&mut self) {                                                             
            for (key,value) in self.ext_component_map.iter() {
                view_ext_sbt_map(*key, *value);
            }           
        }

        // Manually insert external cloned marketplace components addresses authorized to buy and 
        // sell AssetNFT having an implemented system to succesfully update user's SBT data.
        // Only testing purpose method as components data is automatically stored whenever an
        // external marketlace is added within "add_ext_mrkt" method and "set_asset_square_values" 
        // method for Neverland AssetSquare. 
        pub fn insert_ext_sbt_map(&mut self, key: ResourceAddress, value: ComponentAddress) { 
            self.ext_component_map.insert(key,value);     
        }

        // Claim accrued tkn fee function whom only protocol owner can succesfully call.
        pub fn claim_tkn_fee(&mut self, amount: Decimal) -> Bucket {       
            
            self.collected_tkn.take(amount)
        }

        // ==================================================
        // Internal methods
        // ==================================================

        // Insert new Asset NFT selling instance data within Asset NFT selling instance data Hashmap
        // whenever a new selling instance is started
        fn map_insert_list(&mut self, tab: Tab) {
            let tup_one = tab.tuple.0;
            let tup_two = tab.tuple.1;
            self.list_map.insert(tup_one,tup_two);
        }

        // Retrieve instance number from related map invoked by seller's methods.
        // Chech made within Asset NFT selling instance data Hashmap.
        fn check_status_list(&mut self, nmbr: u128, flag: u8) {
            for (key,value) in self.list_map.iter() {
                if key.1 == nmbr {
                    match flag {
                        2 => if value.1.0 == 0 {
                            break;
                        }
                        _ => ()
                    }
                } 
            }
        }

        // Switch Badge addresses data between buyer/seller whenever a buy proposal has been accepted
        // or a raffle jackpot has been collected within Asset NFT selling instance data Hashmap.
        fn switch_badge_list(&mut self, bdg: ResourceAddress, flag: u8, nmbr: u128, gain: Decimal){
            let tab = Tab::new();
            let mut tup_two = tab.tuple.1;
            let mut founded = false;
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr {
                    value.1.1 = gain;
                    match flag {
                        1 => value.1.0 = 11,
                        _ => value.1.0 = 2
                    }
                    tup_two = value.clone();
                    founded = true;
                    break;
                }
            }
            assert!(founded," Correspondence unfounded ");
            let tup_one = (bdg,nmbr);
            self.list_map.insert(tup_one,tup_two);
        } 

        // Check a badge existence within Asset NFT selling instance data Hashmap.
        fn check_badge_list(&mut self, bdg_addr: ResourceAddress) -> bool {                   
            for (key,_value) in self.list_map.iter() {
                if key.0 == bdg_addr {
                    return true;
                }    
            }     
            false
        } 

        // Retrieve raffle NFT winner within Asset NFT selling instance data Hashmap.
        fn raffle_winner_list(&mut self, nmbr: u128, val: u128) {
            for (key,value) in self.list_map.iter_mut() { 
                if key.1 == nmbr {
                    value.2.6 = val;
                }
            }
        }

        // Update NFT selling instance status within Asset NFT selling instance data Hashmap.
        // Method needs to be internally call by any other method able to modify selling instance 
        // conditions to reflect NFT selling instance actual status in terms of time and triggering 
        // his state transition at occurence.     
        fn update_list(
            &mut self, 
            nr: u128, 
            bdg: ResourceAddress, 
            dl: u64
        ) -> (Vec<Tab>,Vec<u128>,Vec<u128>,u8) { 
            let mut v: Vec<Tab> = Vec::new();
            let mut nmbr_vec: Vec<u128> = Vec::new();   
            let mut switch_vec: Vec<u128> = Vec::new(); 
            let mut wave = false;       
            let mut tckt = 0;                      
            for (key,value) in self.list_map.iter_mut() { 
                if key.1 == nr && bdg == ResourceAddress::from(RADIX_TOKEN) || nr == 0 && bdg == key.0 {  
                    wave = true;
                    let now = Runtime::current_epoch();
                    let tckt_prc_dec = Decimal::from(value.2.5);
                    if now > value.2.2 {
                        match value.1.0 {
                            0 => {                                                    // NFT on Sell
                                value.2.1 = dec!("0");
                                value.2.2 = 0;
                            }
                            3 => {                                                 // NFT on Auction
                                if value.2.1 >= value.2.0 {
                                    value.1.0 = 4;
                                } else {
                                    value.1.0 = 7;
                                }
                                switch_vec.push(key.1); 
                                if now > value.2.2+dl && value.2.1 >= value.2.0 {   
                                    value.1.0 = 5;
                                }
                            }
                            4 => if now > value.2.2+dl && value.2.1 >= value.2.0 {
                                    value.1.0 = 5;
                                } 
                            8 => if value.2.1*tckt_prc_dec >= value.2.0 {           // NFT on Raffle
                                    value.1.0 = 9;
                                    tckt = value.2.5;
                                    nmbr_vec.push(key.1);
                                } else {
                                    value.1.0 = 10;
                                    switch_vec.push(key.1);
                                }
                            _ => ()
                        }
                    }
                    let tup_one = (key.0,key.1);
                    let tup_two = (
                        value.0.clone(),
                        (value.1.0,value.1.1),
                        (value.2.0,value.2.1,value.2.2,value.2.3,value.2.4,value.2.5,value.2.6)
                    );
                    let tab = Tab { tuple:(tup_one,tup_two)};  
                    v.push(tab);                                  
                    match nr { 
                        0 => (),
                        _ => break
                    } 
                } 
            } 
            assert!(wave," Correspondence unfounded! ");
            (v,nmbr_vec,switch_vec,tckt) 
        }

        // Update NFT selling instance status within Asset NFT selling instance data Hashmap.
        fn buy_nft_list(&mut self, sale_nr: u128, rest: Decimal) {
            let mut wave = false;
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == sale_nr {    
                    value.1.0 = 1;
                    value.1.1 = rest;
                    wave = true;
                    break;
                }
            }
            assert!(wave," NFT correspondence unfounded! ");
        } 

        // Update NFT selling instance status within Asset NFT selling instance data Hashmap.
        fn buy_ticket_list(&mut self, nmbr: u128, sum: u8, new_end: bool) -> u8 {
            let mut total_tckt = 0;
            let mut wave: bool = false;
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr && Runtime::current_epoch() <= value.2.2 {
                    value.2.5 += sum;
                    total_tckt = value.2.5;
                    wave = true;
                    if new_end { 
                        value.2.2 += 1;
                    } 
                    break;                        
                }
            } 
            assert!(wave," Unable to Update NFT status ");

            total_tckt
        }

        // Update NFT selling instance status within Asset NFT selling instance data Hashmap. 
        // Asset NFT selling instance in normal mode
        fn buy_prop_list(&mut self, nmbr: u128, prop: Decimal, endtime: u64) -> (u64,u8) {
            let mut founded = false;
            let end = Runtime::current_epoch()+endtime;         
            let mut flag = 0;                                  
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr {
                    if prop > value.2.1 {
                        value.2.1 = prop; 
                        value.2.2 = end;
                        flag = 1;
                    } else {
                        higher_amount(value.2.1);
                        break;
                    }  
                founded = true;
                break;
                }
            } 
            assert!(founded," Unable to update values ");

            (end,flag)
        }

        // Update NFT selling instance status within Asset NFT selling instance data Hashmap. 
        // Asset NFT selling instance in auction mode
        fn place_bid_list(&mut self, nmbr: u128, bid: Decimal, new_end: bool) -> bool {
            let mut wave: bool = false;
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr && Runtime::current_epoch() <= value.2.2 {
                    if bid > value.2.1 {
                        value.2.1 = bid;
                            if new_end {
                                value.2.2 += 1;
                            } 
                        wave = true;
                    } else {
                        higher_amount(value.2.1);
                        break;
                    }  
                    break;
                }
            } 
            assert!(wave," Unable to Update NFT status ");

            wave
        }

        // Update NFT selling instance status within Asset NFT selling instance data Hashmap. 
        // Asset NFT selling instance retired, "unstock_nft" method called by NFT seller.
        fn unstock_list(&mut self, nr: u128) {
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nr {
                    value.1.0 = 12;
                    break; 
                }      
            }
        }

        // Update NFT selling instance status within Asset NFT selling instance data Hashmap. 
        // Asset NFT selling instance in auction mode
        fn pay_win_bid_list(&mut self, nmbr: u128, end: u64) -> Decimal {
            let mut rest = dec!("0");
            let mut wave = false;
            let now = Runtime::current_epoch();
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr {
                    assert!(value.2.1 >= value.2.0," Reserve price unmatched ");
                    if now >= value.2.2+1 && now <= value.2.2+end && value.1.1 == dec!("0") {
                        rest = value.2.1-value.2.3;
                        value.1.0 = 6;
                        value.1.1 = value.2.1;
                        wave = true;
                        break;
                    }
                }    
            }
            assert!(wave," Check NFT data ");

            rest
        }

        // Internal function that modify price variable value within "list_map".
        // Utilized whenever a seller needs to modify sale price of an NFT selling
        // instance in normal mode.
        fn modify(&mut self, tab: Tab, price: Decimal) {
            let tup_one = tab.tuple.0;
            let tuple_two = self.list_map.get_mut(&tup_one).unwrap();
            tuple_two.2 = (price,dec!("0"),0,dec!("0"),0,0,0);
        }

        // Internal function that modify values within "list_map".
        // Utilized whenever a seller needs to restart an NFT selling
        // instance in auction or raffle mode.
        fn restart(&mut self, tab: Tab, rd: RstData) -> (Decimal,Decimal,Decimal) {
            let mut past_bid_bond = dec!("0");
            let mut past_rsv_prc = dec!("0");
            let mut past_win_bid = dec!("-1");
            let mut tup_two = tab.tuple.1;
            match self.list_map.get(&(rd.t.0,rd.t.1)){
                Some(value) => {
                    if rd.t.4 == 0 {
                        if Runtime::current_epoch() > value.2.2+rd.t.2 && value.2.1 >= value.2.0 ||
                            Runtime::current_epoch() > value.2.2 && value.2.1 < value.2.0 {
                                past_bid_bond = value.2.3;
                                past_rsv_prc = value.2.0;
                                past_win_bid = value.2.1;
                        }
                        tup_two.1 = (3,Decimal::zero()); 
                        tup_two.2 = 
                            (rd.t.5,rd.t.8,Runtime::current_epoch()+rd.t.6,rd.t.7,rd.t.3,0,0);
                    } else {
                        tup_two.1 = (8,Decimal::zero());  
                        tup_two.2 = 
                            (rd.t.5,rd.t.8,Runtime::current_epoch()+rd.t.6,rd.t.7,rd.t.3,rd.t.4,0);
                    }
                    tup_two.0 = value.0.clone();                   
                }
                _ => unfound(3)
            }
            self.list_map.insert(tab.tuple.0,tup_two);

            (past_bid_bond,past_rsv_prc,past_win_bid)
        }

        // Remove Asset NFT ownership from seller SBT withdrawing accrued payment
        fn sbt_update(
            &mut self,
            meta_nft_bckt_sum: Vec<Bucket>,
            update_flag: bool,
            land_owner_sbt: Proof
        ) -> Vec<Bucket>  {
            let land_owner_sbt: ValidatedProof = land_owner_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[sbt_update]: Invalid proof provided!");  
            let land_owner_sbt_id = land_owner_sbt.non_fungible::<UserSBT>().id();     
            let mut land_owner_data_sbt: UserSBT = land_owner_sbt.non_fungible().data();

            let mut linked_assets: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();

            // Collect resource address & ID of NFT within input vector
            let mut i = 0;
            for _bckt in meta_nft_bckt_sum.iter() {
                let meta_data = meta_nft_bckt_sum.get(i).unwrap().non_fungible::<AssetNFT>().data();
                linked_assets.append(&mut meta_data.linked_assets.clone());
                i += 1;
            }
            assert!(i == meta_nft_bckt_sum.len(),"[sbt_update]: linked asset issue detected!");    
    
            // Remove Asset NFT ownership from seller user:
            // remove NFT resource address & ID from "linked_assets" data field within provided SBT
            i = 0;
            let mut count = 0;
            for tup in land_owner_data_sbt.real_estate_properties.clone() {
                for data in linked_assets.clone() {
                    if tup.0 == data.0 && tup.1 == data.1 {
                        land_owner_data_sbt.real_estate_properties.remove(i);
                        count += 1;
                        i -= 1;
                    }
                }
                i += 1;
            }
            assert!(linked_assets.len() == count,"[sbt_update]: Land Property check failed!");

            // Update SBT data
            if update_flag {
                self.sbt_updater_badge.authorize(|| {
                    borrow_resource_manager!(land_owner_sbt.resource_address())
                        .update_non_fungible_data(&land_owner_sbt_id, land_owner_data_sbt)
                });
            }

            meta_nft_bckt_sum
        }

        // Verify Asset NFT ownership from seller SBT before sale listing.
        fn check_ownership(
            &mut self,
            nft_bckt_sum: Vec<Bucket>,
            land_owner_sbt: Proof
        ) -> Vec<Bucket>  {
            let land_owner_sbt: ValidatedProof = land_owner_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[sbt_update]: Invalid proof provided!");       
            let land_owner_data_sbt: UserSBT = land_owner_sbt.non_fungible().data();
            let mut linked_assets: Vec<(ResourceAddress,NonFungibleId,AssetNFT)> = Vec::new(); 

            // Collect resource address, ID & data of NFT within input vector 
            let mut i = 0;
            for bckt in nft_bckt_sum.iter() {
                let nft_id = nft_bckt_sum.get(i).unwrap().non_fungible::<AssetNFT>().id();
                let nft_data = nft_bckt_sum.get(i).unwrap().non_fungible::<AssetNFT>().data();
                linked_assets.push((bckt.resource_address(),nft_id,nft_data));
                i += 1;
            }

            // Land Property Asset NFT ownership check
            i = 0;
            for tup in land_owner_data_sbt.real_estate_properties.clone() {
                for data in linked_assets.clone() {
                    if tup.0 == data.0 && tup.1 == data.1 {
                        i += 1;
                    }
                }
            }
            assert!(i == nft_bckt_sum.len(),"[check_ownership]:Asset ownership undetected!");

            nft_bckt_sum
        }

        // Add Asset NFT ownership into provided SBT if buying occurred within protocol 
        // otherwise check if it occured on an authorized external cloned protocol through 
        // proof resource address verification.
        fn check_sbt(
            &mut self,
            output_vec_bckt: Vec<Bucket>,
            user_sbt: Proof
        ) -> Vec<Bucket> {
            let user_sbt: ValidatedProof = user_sbt.unsafe_skip_proof_validation();
            assert!(user_sbt.amount() == dec!("1"),"[check_sbt]:Single SBT proof required!");
            
            // Check SBT proof resource address    
            let mut found = false;
            if user_sbt.resource_address() == self.user_sbt {    
                let user_sbt_id = user_sbt.non_fungible::<UserSBT>().id();     
                let mut user_data_sbt: UserSBT = user_sbt.non_fungible().data();
                let mut linked_assets: Vec<(ResourceAddress,NonFungibleId,AssetNFT)> = Vec::new(); 

                // Collect resource address, ID & data of NFT within input vector
                let mut i = 0;
                for bckt in output_vec_bckt.iter() {
                    let nft_id = output_vec_bckt.get(i).unwrap().non_fungible::<AssetNFT>().id();
                    let nft_data = output_vec_bckt.get(i).unwrap().non_fungible::<AssetNFT>().data();
                    linked_assets.push((bckt.resource_address(),nft_id,nft_data));
                    i += 1;
                }

                // Land Property Asset NFT anti money laundering check
                let mut wash = false;
                for tup in user_data_sbt.real_estate_properties.clone() {
                    for data in linked_assets.clone() {
                        if tup.0 == data.0 && tup.1 == data.1 {
                            wash = true;
                        }
                    }
                }
                assert!(!wash,"[check_sbt]:Land Property AML check failed!");

                user_data_sbt.real_estate_properties.append(&mut linked_assets);   

                // Update SBT data
                self.sbt_updater_badge.authorize(|| {
                    borrow_resource_manager!(user_sbt.resource_address())
                        .update_non_fungible_data(&user_sbt_id, user_data_sbt)
                });

                found = true;
            } 
            assert!(
                found || self.ext_component_map.get(&user_sbt.resource_address()).is_some(),
                "[check_sbt]:Unauthorized SBT resource detected!"
            );

            output_vec_bckt
        }

            // Retrieve external marketplace values 
            fn ext_mrkt_data(&mut self, cmp: ComponentAddress, bdg: ResourceAddress) -> ExtMrkt {   
                let mut extmrkt = ExtMrkt::new(self.tkn.dex);
                for val in self.ext_mrkt_map.iter() {
                    if val.1 == bdg && cmp == self.tkn.dex {
                        extmrkt.tuple.0 = val.0;
                        extmrkt.tuple.1 = val.1;
                        extmrkt.tuple.2 = val.2;
                        extmrkt.tuple.3 = val.3;
                        extmrkt.tuple.4 = true;
                        extmrkt.tuple.5 = val.4;
                        break;
                    } else if val.0 == cmp && bdg == ResourceAddress::from(RADIX_TOKEN) {
                        extmrkt.tuple.0 = val.0;    
                        extmrkt.tuple.3 = val.3;    
                        extmrkt.tuple.5 = val.4;
                        break;
                    }    
                }

                extmrkt
            }

            // Take Caller Badge from Vault relate to an external Marketplace resource address.
            fn caller_bdg_take(&mut self, ext_badge: ResourceAddress) -> Bucket {
                match self.badge_vaults.get_mut(&ext_badge) {
                    Some(vault) => vault.take(Decimal::one()),
                    None => {
                        info!(" Caller Badge not in stock! ");
                        std::process::abort()
                    }
                }
            } 

            // Test external marketplace buy allowance for royalties computation.
            fn out_fx(&self, ext_mrkt: ComponentAddress, bdg_bckt_ref: Proof) -> ResourceAddress {
                out_currency(ext_mrkt,bdg_bckt_ref)
            } 

            // If required by conditions swap tokens on an external DEX, otherwise return immutate 
            // input resources.
            fn bckt_fx(
                &mut self, 
                min: Decimal, 
                ad: ResourceAddress, 
                d: ComponentAddress, 
                b: Bucket
            ) -> Bucket {
                if ad != b.resource_address().clone() {
                    self.swap_fx(min, ad, d, b)
                } else {
                    b
                }
            }

            // If required by conditions swap tokens on an external DEX, otherwise return immutate 
            // input resources.
            fn swap(
                &mut self, 
                em: ExtMrkt, 
                mut vb: Vec<Bucket>, 
                dex: ComponentAddress
            ) -> Vec<Bucket> {
                if vb.get(0).unwrap().resource_address() == em.tuple.3 {    
                    let bckt = self.swap_fx(dec!("0"), self.currency, dex, vb.pop().unwrap());            
                    vb.push(bckt);
                }

                vb
            }

            // Swap tokens on an external DEX
            fn swap_fx(
                &self, 
                sum: Decimal, 
                fx: ResourceAddress, 
                dex: ComponentAddress, 
                tkn: Bucket
            ) -> Bucket { 
                swap_fx(sum,fx,dex,tkn)
            }

            // Put Caller Badge in Vault relate to an external Marketplace resource address.
            fn caller_bdg_put(&mut self, caller_badge: Bucket){
                let v = self.badge_vaults.get_mut(&caller_badge.resource_address()).unwrap();
                v.put(caller_badge);
            }

            // Internal method invoked to take a specific Caller Badge from related component 
            // vaults map and return it with other relevant data like external marketplace 
            // component address & currency. Used to perform buy/selling AssetNFT operations via
            // external component call. 
            fn check_buy(
                &mut self, 
                mrkt_addr: ComponentAddress
            ) -> (ExtMrkt,Bucket,ComponentAddress) {
                let extmrkt = self.ext_mrkt_data(mrkt_addr,ResourceAddress::from(RADIX_TOKEN));
                let caller_badge = self.caller_bdg_take(extmrkt.tuple.5);
                let bdg_bckt_ref = caller_badge.create_proof();
                let output_currency = self.out_fx(extmrkt.tuple.0,bdg_bckt_ref);
                assert!(self.currency == output_currency," External marketplace unauthorized ");

                (extmrkt,caller_badge,self.tkn.dex.clone())
            }

            // Internal method invoked to take a specific Caller Badge from related component 
            // vaults map and return it with other relevant data like external marketplace 
            // component address & currency. Used to perform buy/selling AssetNFT operations via
            // external component call. 
            fn check_rec(
                &mut self, 
                mrkt_addr: ComponentAddress
            ) -> (ExtMrkt,Bucket,ComponentAddress) {          
                let extmrkt = self.ext_mrkt_data(mrkt_addr,ResourceAddress::from(RADIX_TOKEN));
                let caller_badge = self.caller_bdg_take(extmrkt.tuple.5);

                (extmrkt,caller_badge,self.tkn.dex.clone())
            }


            // Verify exact correspondence between provided metaAssetNFT and related AssetNFT 
            // and returns a tuple with useful data to succesfully perform operations like
            // unstock unsold AssetNFT, modify instance prices, restart auction and raffle instances,
            // collect ordinary sale payments, buy proposals, auction payments, raffle jackpots. 
            fn check_meta( 
                &mut self,
                meta_nft_bckt_sum: Vec<Bucket>,
                flag: bool,
                new_nmbr: u128,
                price_flag: u8,
                new_price: Decimal  
            ) -> CheckMeta {
                let mut vmk: Vec<NonFungibleId> = Vec::new();
                let mut addr_metakey_map: HashMap<ResourceAddress,Vec<NonFungibleId>> = HashMap::new();
                let (mut esc,mut key_tuple_bool,mut ix,mut jx) = (false,false,0,0);
                for (addr,meta_addr) in self.meta.iter() {
                    for _bckt in meta_nft_bckt_sum.iter() {
                        if meta_addr == &meta_nft_bckt_sum.get(jx).unwrap().resource_address() {
                            let meta_key = 
                                meta_nft_bckt_sum.get(jx).unwrap().non_fungible::<AssetNFT>().id();
                            match addr_metakey_map.get_mut(&addr) {    
                                Some(v) => { 
                                    v.push(meta_key);
                                }
                                _ => { 
                                    vmk.clear();
                                    vmk.push(meta_key);
                                    addr_metakey_map.insert(*addr,vmk.clone());
                                }
                            }                    
                            ix += 1;
                        }
                        jx += 1;
                        if ix == meta_nft_bckt_sum.len() {
                            esc = true;
                            break;
                        }
                        if jx == meta_nft_bckt_sum.len() {
                           jx = 0; 
                        }

                    }
                    if esc {
                        key_tuple_bool = true;
                        break;
                    }
                }  
                assert!(key_tuple_bool," Key correspondence unfounded ");

                for meta_nft_burn in meta_nft_bckt_sum {
                    self.bckt_burn(meta_nft_burn);
                }  

                let (mut amount,mut number,mut old_nmbr,mut switch) = (dec!(0),0,0,false);
                let mut vec_new: Vec<(NonFungibleId,NonFungibleId,Decimal,u128)> = Vec::new();
                let mut chk = CheckTuple::new();
                let (mut esc,mut key_tuple_bool,mut i,mut iy) = (false,false,0,0);
                let mut vec_nmbr: Vec<u128> = Vec::new();
                for (nft_addr,meta_nft_key_vec) in addr_metakey_map.iter_mut() {
                    let (_sum,v_key) = self.meta_map.get_mut(&nft_addr).unwrap();
                   
                    for _meta_nft_key in meta_nft_key_vec.clone() {  
                        let mut j = 0;
                        for mut tuple in v_key.clone() {
                            if tuple.1 == meta_nft_key_vec[i] { 
                                if number != 0 && new_nmbr == 0 {
                                    assert!(tuple.3 == number," Mixed instances detected ");
                                } 
                                vec_nmbr.push(tuple.3);
                                if new_nmbr != 0 {
                                    old_nmbr = tuple.3;
                                    tuple.3 = new_nmbr;
                                }
                                number = tuple.3;
                                if !flag && tuple.2 == Decimal::zero() { 
                                    tuple.2 = Decimal::one();  
                                    let tup = tuple.clone();
                                    vec_new.push(tup);          
                                    switch = true;         
                                }
                                v_key.remove(j);
                                meta_nft_key_vec[i] = tuple.0.clone();  
                                i += 1;
                                iy += 1;                                                 
                            } else {
                                j += 1;
                            }
                            if i == meta_nft_key_vec.len() {
                                if iy == ix {
                                    (amount,key_tuple_bool,esc) = (tuple.2,true,true);
                                    break;
                                } 
                                i = 0;
                            }
                        }
                        if switch {
                            for tuple in &vec_new {
                                v_key.push(tuple.clone());
                            }
                            vec_new.clear();
                        }
                        if esc {    
                            break;
                        }
                    }           

                    if esc {
                        if new_nmbr != 0 {
                            assert!(
                                vec_nmbr.iter().all(|x| *x == old_nmbr),
                                " Mixed instances detected "
                            );
                            chk = self.status_check(old_nmbr);
                        }
                        break;
                    }
                } 
                assert!(key_tuple_bool," Key correspondence unfounded ");
                
                let mut nft_vec: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();
                for (nft_addr,meta_nft_key_vec) in addr_metakey_map.clone().into_iter() { 
                    for keys in meta_nft_key_vec {
                        nft_vec.push((nft_addr,keys.clone()));
                    }
                }

                if new_nmbr != 0 || new_price != Decimal::zero() {
                    for (nft_addr,key) in nft_vec.clone() {
                        match self.nft_map.get_mut(&nft_addr) {
                            Some(v) => {
                               for mut data in v { 
                                    if data.1 == key.clone() {
                                        if price_flag != 0 {
                                            data.2 = new_price;                 // modify_nft_price method
                                        } else {
                                            (data.0,data.3) = (number,true);    // restart_auction restart_raffle methods
                                        }
                                    }                               
                                }                        
                            }  
                            None => { 
                                info!(" Unfounded correspondence ");
                                std::process::abort()
                            }
                        }
                    }
                }
                
                let output_tuple = (addr_metakey_map,nft_vec,amount,number,old_nmbr,chk.t.0);
                CheckMeta { m: output_tuple }
            }

            // Given a AssetNFT selling instance number, method retrieve resource addresses and related ID
            // of AssetNFTs associated with given instance.
            fn check_meta_id(
                &mut self,
                number: u128,
            ) -> HashMap<ResourceAddress,Vec<NonFungibleId>> {
                let (mut key_tuple_bool,mut switch) = (false,false);
                let mut vk: Vec<NonFungibleId> = Vec::new();
                let mut addr_key_map: HashMap<ResourceAddress,Vec<NonFungibleId>> = HashMap::new();
                let mut vec_new: Vec<(NonFungibleId,NonFungibleId,Decimal,u128)> = Vec::new();
                
                for (addr,(_a,v_key)) in self.meta_map.iter_mut() { 
                    let mut i = 0;                    
                    for mut tuple in v_key.clone() {
                        if number == tuple.3 {
                            match addr_key_map.get_mut(&addr) {
                                Some(v) => v.push(tuple.0.clone()),
                                _ => { 
                                    vk.clear();
                                    vk.push(tuple.0.clone());
                                    addr_key_map.insert(*addr,vk.clone());
                                }
                            } 
                            if tuple.2 != dec!("1") {
                               (tuple.2,switch) = (dec!("1"),true);
                               vec_new.push(tuple);       
                            }
                            v_key.remove(i);
                            key_tuple_bool = true;
                        } else {
                            vec_new.push(tuple);
                            i += 1;
                        }
                    }
                    if switch {
                        v_key.clear();
                        v_key.append(&mut vec_new);
                    }
                    if !key_tuple_bool {
                        vec_new.clear();
                    }
                }
                assert!(key_tuple_bool," Key correspondence unfounded ");
                    
                addr_key_map
            } 

            // Retrieve instance number from related map invoked by seller's methods. 
            fn check_status(&mut self, nmbr: u128, flag: u8) -> CheckTuple { 
                self.check_status_list(nmbr,flag);                         
                self.status_check(nmbr)                                     
            }  

            // Check Auction or Raffle mode by related NFT and export related data. 
            fn status_check(&mut self, nmbr: u128) -> CheckTuple { 
                let mut tup = CheckTuple::new();   
                let output_vector = self.update_state(nmbr, ResourceAddress::from(RADIX_TOKEN));           
                for tab in output_vector.clone() { 
                    let (_key,val) = tab.tuple;
                    let amnt = val.2.1*val.2.5;
                    match val.1.0 {
                        3 ... 7 => tup.t = (val.1.0,val.2.0,val.2.1,val.2.2,val.2.3,0,0,amnt,nmbr,val.1.1),
                        8 ... 12 => tup.t = (val.1.0,val.2.0,val.2.1,val.2.2,dec!("0"),val.2.4,val.2.5,amnt,nmbr,val.1.1),
                        _ => tup.t = (val.1.0,dec!("0"),dec!("0"),0,dec!("0"),0,0,amnt,nmbr,val.1.1)
                    }
                }

                CheckTuple { t: tup.t }
            }

            // Update NFT selling instance status within Asset NFT selling instance data Hashmap.
            // Method needs to be internally call by any other method able to modify selling instance 
            // conditions to reflect NFT selling instance actual status in terms of time and triggering 
            // his state transition at occurence.
            fn update_state(&mut self, nr: u128, bdg: ResourceAddress) -> Vec<Tab> { 
                let mut output_vec: Vec<Tab> = Vec::new();
                let (v,nmbr_vec,switch_vec,tckt) = self.update_list(nr,bdg,self.tkn.auction_dl);

                if switch_vec.len() > 0 { 
                    for sale_number in switch_vec { 
                        self.switch_status(sale_number,0); 
                    }
                }

                if nmbr_vec.len() > 0 { 
                    for mut tab in v {
                        for sale_number in nmbr_vec.clone() {
                            if tab.tuple.0.1 == sale_number {
                                tab.tuple.1.2.6 = self.raffle_winner(sale_number,tckt);
                                self.switch_status(sale_number,0); 
                            }
                        }
                        output_vec.push(tab);
                    }

                    output_vec
                } else { 
                    
                    v
                }
            }

            // Retrieve data stored within a provided Buyer NFT Badge
            fn buy_bdg_data(nft: Bucket) -> (Bucket,u128,ComponentAddress) {
                let nft_data: Mode = nft.non_fungible().data();
                (nft,nft_data.instance_nmbr,nft_data.mrkt_addr)
            }
            
            // Retrieve data stored within a provided Raffle Buyer NFT Badge 
            fn raffle_bdg_data(nft: Bucket) -> (Bucket,u128,ComponentAddress) {
                let nft_data: TicketID = nft.non_fungible().data();
                (nft,nft_data.instance_nmbr,nft_data.mrkt_addr)
            }

            // Switch Badge addresses data between buyer/seller whenever a buy proposal has been accepted
            // or a raffle jackpot has been collected 
            fn switch_badge(&mut self, bdg: ResourceAddress, flag: u8, nmbr: u128, profit: Decimal){
                self.switch_badge_list(bdg,flag,nmbr,profit);
            }

            // Determine Asset NFT buying/selling transaction amounts, send correspetives into
            // protocol's vault, swap and send correspetives to related external cloned protocols,
            // send an academy share to Academy component vault
            fn take_fee(
                &mut self,
                sale_nr: u128,
                tkn_amnt: Decimal, 
                mut tkn_bckt: Bucket,
                bdg_addr: ResourceAddress,
                flag: u8
            ) -> (Decimal,Bucket) {
                let h = dec!("100");
                let rest: Decimal;
                if self.currency != self.tkn.currency {
                    let asset_square_bckt: Bucket;
                    rest = tkn_amnt-tkn_amnt*self.fee/h;
                    net_gain(rest);

                    // NFT sold on External Marketplace by another External Marketplace 
                    if bdg_addr != self.asset_square_badge && bdg_addr != ResourceAddress::from(RADIX_TOKEN) {
                        let extmrkt = self.ext_mrkt_data(self.tkn.dex,bdg_addr);
                        assert!(extmrkt.tuple.4," NFT correspondence unfounded 2 ");
                        
                        self.collected_tkn.put(tkn_bckt.take(tkn_amnt*(self.fee-extmrkt.tuple.2-self.tkn.royalty)/h));
                        let ext_fee_bckt = tkn_bckt.take(tkn_amnt*extmrkt.tuple.2/h);
                        asset_square_bckt = tkn_bckt.take(tkn_amnt*self.tkn.royalty/h);                        
                        let royalty = 
                            self.swap_fx(dec!("0"),extmrkt.tuple.3,self.tkn.dex,ext_fee_bckt);
                        let amnt = tkn_stock(royalty,extmrkt.tuple.0);
                        info!(" Fee placed in external Marketplace Vault {} ",amnt);

                    // NFT sold by External Marketplace 
                    } else if bdg_addr == ResourceAddress::from(RADIX_TOKEN) {                       
                        self.collected_tkn
                            .put(tkn_bckt.take(tkn_amnt*(self.fee-self.tkn.royalty)/h));
                        asset_square_bckt = tkn_bckt.take(tkn_amnt*self.tkn.royalty/h);

                        let sum_one = tkn_amnt*(self.fee-self.tkn.royalty)/h;
                        let asset_square_royalty = tkn_amnt*self.tkn.royalty/h;
                        net_fee(sum_one,asset_square_royalty);
                        
                    // NFT sold on External Marketplace by AssetSquare 
                    } else {                                                                
                        assert!(bdg_addr == self.asset_square_badge," AssetSquare Selling Tx ");                   
                        self.collected_tkn.put(tkn_bckt.take(tkn_amnt*(self.fee-self.tkn.fee)/h));
                        asset_square_bckt = tkn_bckt.take(tkn_amnt*self.tkn.fee/h);  

                        let asset_square_fee = tkn_amnt*self.tkn.fee/h;
                        royalty(asset_square_fee);
                    }
                    let royalty = 
                        self.swap_fx(dec!("0"),self.tkn.currency,self.tkn.dex,asset_square_bckt);
                    let amount = tkn_stock(royalty,self.tkn.vault);
                    royalty_placed(amount);

                // NFT sold on AssetSquare
                } else {
                    let mut asset_square_fee = tkn_amnt*self.tkn.fee/h;
                    if self.tkn.academy_share > dec!("0") {    
                        let academy_fee = (tkn_amnt*self.tkn.fee/h)*self.tkn.academy_share/h;
                        asset_square_fee -= academy_fee;
                        self.tkn_lock(tkn_bckt.take(academy_fee));
                    }

                    self.collected_tkn.put(tkn_bckt.take(asset_square_fee));
                    rest = tkn_amnt-tkn_amnt*self.tkn.fee/h;
                    net_gain(rest);
                }

                // Put proceeds from NFT sale into seller related vault.
                match flag {
                    1 => self.tkn_put(sale_nr, tkn_bckt.take(rest)),
                    _ => ()
                }

                (rest,tkn_bckt)
            } 

            // Send $TKN token academy share in Academy Vault Component 
            fn tkn_lock(&mut self, academy_bckt: Bucket){
                token_lock(academy_bckt,self.tkn.academy_vault);
            } 

            // Collect bid bond in case of auction payment deadline ovetaken 
            fn bid_bond(&mut self, s: u8, bb: Decimal, rp: Decimal, m: Decimal, n: u128) -> Bucket {
                let mut collected_bid_bond: bool = false;
                if s == 5 && bb > dec!("0") {
                    bid_bond(bb);
                    collected_bid_bond = true;
                }   
                if collected_bid_bond && m >= rp {   
                    self.tkn_collect(n, bb)
                } else {
                    Bucket::new(RADIX_TOKEN)
                }
            } 

            // Put protocol's currency collected tokens in vault
            fn tkn_put(&mut self, sale_nr: u128, bckt: Bucket){
                match self.tkn_vaults.get_mut(&sale_nr) {
                    Some(vault) => vault.put(bckt),
                    None => std::process::abort()
                }
            } 

            // Take protocol's currency collected tokens from vault
            fn tkn_collect(&mut self, sale_nr: u128, amnt: Decimal) -> Bucket {
                let output_tkn: Bucket;
                match self.tkn_vaults.get_mut(&sale_nr) {
                    Some(vault) => output_tkn = vault.take(amnt),
                    None => output_tkn = unfound_bckt(6)
                }
            
                output_tkn
            }

            // Update past auction bids in related map
            fn update_auction_map(&mut self, nmbr: u128){
                self.maps.update_auction_map(nmbr);
            } 

            // Update past raffle bids in related map
            fn update_raffle_map(&mut self, nmbr: u128){
                self.maps.update_raffle_map(nmbr); 
            } 

            // Change NFT sale status switching related flag. 
            fn switch_status(&mut self, nmbr: u128, flag: usize){
                let mut answer = false;
                for (_key,v) in self.nft_map.iter_mut() { 
                    for val in v { 
                        if val.0 == nmbr { 
                            match flag {
                                1 => val.3 = true, 
                                _ => val.3 = false
                            }
                            answer = true;
                        }
                    }
                }
                assert!(answer," Sale instance unfounded ! ");
            } 

            // Pick up a raffle winner of a determinated Asset NFT selling instance
            fn raffle_winner(&mut self, nmbr: u128, tickets: u8) -> u128 { 
                let tckts = usize::from(tickets); 
                let hash = Runtime::transaction_hash().to_string();
                let seed = &hash[0..5];
                let result = usize::from_str_radix(&seed, 16).unwrap();
                let index_value = result % tckts;
                instance(nmbr);

                let w = self.maps.raffle_winner(nmbr);
                let val = w.get(index_value).unwrap(); 
                winner(val.0,val.1);
                self.maps.update_raffle_winner(val.0,val.1);   
                self.raffle_winner_list(nmbr,val.0);

                val.0
            } 

            // Verify requested NFT is present within AssetSquare listing or not. 
            fn nft_match(&mut self, nmbr: u128, flag: bool) -> NftMatch {         
                let (mut answer,mut price) = (false,dec!("0"));
                let mut nft_vec: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();
                for (key,v) in self.nft_map.iter_mut() { 
                    for val in v.clone() { 
                        if val.0 == nmbr && val.3 || val.0 == nmbr && flag { 
                            nft_vec.push((*key,val.1));
                            (answer,price) = (true,val.2);
                        }
                    }
                }
                if !answer { 
                    nft_vec.push((ResourceAddress::from(RADIX_TOKEN),NonFungibleId::from_u64(0u64)));
                }
                let output_tuple = (answer,nft_vec,price);
                
                NftMatch { n: output_tuple}
            } 

            // Burn a bucket
            fn bckt_burn(&mut self, bckt: Bucket) {
                self.minter_badge.authorize(|| {bckt.burn()});
            }

            // Take buyed NFT from nft vault and erase data in relative hashmap.
            fn nft_take(&mut self, addr: ResourceAddress, key: NonFungibleId, flag: bool) -> Bucket {
                self.erase_from_map(addr, key.clone());

                // Update NFT sell profit amount
                if flag {
                    let (a,v) = self.meta_map.get_mut(&addr).unwrap();
                    for value in v {
                        if value.0 == key.clone() {
                            *a += value.2;
                        }
                    }
                }

                match self.nft_vaults.get_mut(&addr) {
                    Some(vault) => vault.take_non_fungible(&key),
                    None => {
                        info!(" NFT not in stock! ");
                        std::process::abort()
                    }
                }
            }

            // erase NFT data from map
            fn erase_from_map(&mut self, nft_addr: ResourceAddress, nft_key: NonFungibleId){
                let v = self.nft_map.get_mut(&nft_addr).unwrap();
                let mut i = 0;
                for data in v.clone() { 
                    if i < v.clone().len() && data.1 == nft_key {
                        v.remove(i);
                    }    
                    i += 1;                     
                }  
            }  

            // Ckeck buyer badge correspondence within related maps.
            fn badge_in(&mut self, nmbr: u128, bdg: ResourceAddress, j: u8) -> bool {  
                
                self.maps.badge_in(nmbr,bdg,j)
            }

            // Method invoked to erase a map entry within raffle badge map
            fn erase_map_entry(&mut self, nmbr: u128, a: u64, b: u8) {

                self.maps.erase_map_entry(nmbr,a,b)
            } 

            // Mint a Seller Badge following a NFT stock event.
            fn add_badge(
                &mut self,  
                vec_data: Vec<(ResourceAddress,NonFungibleId,AssetNFT)>,
                price: Decimal, 
                auction: bool,
                raffle: bool,
                reserve_prc: Decimal,
                start_prc: Decimal,
                end: u64,
                bid_bond: Decimal,
                dl: u64,             
                nmbr: u128
            ) -> Bucket {
                let user_badge: ResourceAddress = ResourceBuilder::new_fungible()
                    .divisibility(DIVISIBILITY_NONE)
                    .metadata("name", " AssetSquare User Badge ")
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply();

                let mut tab = Tab::new();
                let mut tup_two = tab.tuple.1;
                tup_two.0 = vec_data.clone();
                let tup_one = (user_badge, nmbr);

                if !auction && !raffle {  
                    tup_two.2.0 = price;
                } else if !raffle {
                    tup_two.1.0 = 3;
                    tup_two.2 = (reserve_prc,start_prc,end,bid_bond,dl,0,0);
                } else {
                    tup_two.1.0 = 8;   
                    tup_two.2 = (reserve_prc,start_prc,end,dec!("0"),dl,0,0);
                } 

                tab = Tab { tuple:(tup_one,tup_two)};
                self.map_insert_list(tab);
                self.minter_badge
                    .authorize(|| { borrow_resource_manager!(user_badge).mint(Decimal::one()) })
            }

            // Mint a Badge and populate a hashmap following a Buy Proposal or an Auction Bid event
            // or a tickets purchase within a NFT raffle selling instance.
            fn add_buyer_badge(
                &mut self, 
                amnt: Decimal,
                sum: Decimal, 
                end: u64,
                mode: u8,       
                bid_bond: Decimal,
                badge_addr: ResourceAddress,    
                status: u8,
                sale_nr: u128
            ) -> Bucket {   
                let buy_bdg: ResourceAddress = ResourceBuilder::new_non_fungible()
                    .metadata("name","BuyerBadge")
                    .metadata("instance", format!("{}", sale_nr))
                    .metadata("marketplace", format!("{}", self.asset_square_comp_addr))
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply(); 

                let key = NonFungibleId::from_bytes(sale_nr.to_be_bytes().to_vec());
                let data = Mode { 
                    instance_nmbr: sale_nr,
                    mrkt_addr: self.asset_square_comp_addr, 
                    listing_mode: mode
                };
                match mode {
                    0 => {
                        let mut v: Vec<(ResourceAddress,Decimal,u64,u8,ResourceAddress)> = Vec::new();
                        v.push((buy_bdg, amnt, end, 1, badge_addr));
                        self.maps.insert_buy_prop_map(sale_nr,v);
        
                        self.minter_badge.authorize(|| { 
                            borrow_resource_manager!(buy_bdg).mint_non_fungible(&key, data)
                        })     
                    }
                    1 => {
                        let mut v: Vec<(ResourceAddress,Decimal,u64,u8,Decimal,ResourceAddress)> = Vec::new();  
                        v.push((buy_bdg, amnt, end, status, bid_bond, badge_addr));
                        self.maps.insert_auction_map(sale_nr,v);          

                        self.minter_badge.authorize(|| { 
                            borrow_resource_manager!(buy_bdg).mint_non_fungible(&key, data)
                        })
                    }
                    _ => {
                        let mut vec_id = self.gen_ticket_id(amnt);
                        let id = TicketID {
                            instance_nmbr: sale_nr,
                            mrkt_addr: self.asset_square_comp_addr, 
                            key: key.clone(),
                            v: vec_id.clone()
                        };
                        let mut v: Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)> = Vec::new();
                        for tckt_id in vec_id.iter_mut() {
                            v.push((*tckt_id,buy_bdg, sum, end, 0, badge_addr));
                        }
                        self.maps.insert_raffle_map(sale_nr,v);
                        
                        self.minter_badge.authorize(|| { 
                            borrow_resource_manager!(buy_bdg).mint_non_fungible(&key, id)
                        })   
                    }
                }
            }

            // Generate raffle tickets ID 
            fn gen_ticket_id(&self, amnt: Decimal) -> Vec<u128> {
                let mut i = dec!("0");
                let mut vec_id = Vec::new();
                loop {
                    let tckt_id = u128::from(Runtime::generate_uuid());
                    vec_id.push(tckt_id);
                    i += 1;
                    if i == amnt {
                        break;
                    }
                }

                vec_id
            }   

            // Create a new meta Asset NFT resource linked to a new Asset NFT resource provided to
            // protocol  
            fn add_meta_nft(
                &mut self, 
                eco: String,
                series: String, 
                number: String, 
                address: ResourceAddress
            ){
                assert!(!self.meta.contains_key(&address)," meta nft already exist ");

                let meta_res_def: ResourceAddress = ResourceBuilder::new_non_fungible()
                    .metadata("Ecosystem", format!(" m-{}",eco))
                    .metadata("Series", format!(" m-{}",series))
                    .metadata("Number", format!(" m-{}",number))
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply(); 

                self.meta.insert(address.clone(),meta_res_def);
            }

            // Mint a metanft amount relative to an amount of NFT provided to protocol 
            fn meta_mint(
                &mut self,  
                nft_address: ResourceAddress, 
                mut new_nft: AssetNFT,
                nft_key: NonFungibleId,
                nmbr: u128
            ) -> Bucket {             
                let meta_res_def = self.meta.get(&nft_address).unwrap().clone();               
                let meta_nft_key = NonFungibleId::random();
                meta_mint(meta_nft_key.clone(),meta_res_def);
                match self.meta_map.get_mut(&nft_address) { 
                    Some((_a,v_key)) => {
                        let mut i: usize = 0;
                        for keys in v_key.clone() { 
                            if nft_key == keys.0 && nft_key == keys.1 && nmbr == keys.3 { 
                                v_key.remove(i);
                                break;
                            }     
                            i += 1;
                        } 
                        v_key.push((nft_key.clone(),meta_nft_key.clone(),Decimal::zero(),nmbr));  
                    }    
                    None => std::process::abort()                  
                };

                self.meta.insert(nft_address.clone(),meta_res_def.clone());

                new_nft.linked_assets.push((nft_address,nft_key.clone()));
        
                self.minter_badge.authorize(|| { 
                    borrow_resource_manager!(meta_res_def).mint_non_fungible(&meta_nft_key,new_nft)
                }) 
            }
    }
}

