use scrypto::prelude::*;
use crate::data_square::*;
use crate::info::*;

blueprint! {
    struct Maps {        
        // Buy Proposal Badge Hashmap with NFT Address & key, Badge Address,amount,deadline,status flag,Caller Badge.
        buy_prop_badge_map: HashMap<u128,Vec<(ResourceAddress,Decimal,u64,u8,ResourceAddress)>>,

        // Auction Badge Hashmap with NFT Address & key, Badge Address,amount,deadline,status flag,bid bond,Caller Badge.
        auction_badge_map: HashMap<u128,Vec<(ResourceAddress,Decimal,u64,u8,Decimal,ResourceAddress)>>,

        // Raffle Badge Hashmap with NFT Address & key,ticket ID,Badge Address,jackpot,deadline,status flag,Caller Badge.
        raffle_badge_map: HashMap<u128,Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)>>,
    }

    #[allow(dead_code)]
    impl Maps {
        pub fn new() -> MapsComponent {
            Self {
                buy_prop_badge_map: HashMap::new(),
                auction_badge_map: HashMap::new(),
                raffle_badge_map: HashMap::new()
            }
            .instantiate()
        }

        pub fn insert_buy_prop_map(
            &mut self, 
            sale_nr: u128, 
            mut v: Vec<(ResourceAddress,Decimal,u64,u8,ResourceAddress)>
        ){
            self.buy_prop_badge_map.entry(sale_nr)                      
                                .and_modify(|z| z.append(&mut v))
                                .or_insert(v); 
        } 

        pub fn insert_auction_map(
            &mut self, 
            sale_nr: u128, 
            mut v: Vec<(ResourceAddress,Decimal,u64,u8,Decimal,ResourceAddress)>
        ){
            self.auction_badge_map.entry(sale_nr)                      
                                .and_modify(|z| z.append(&mut v))
                                .or_insert(v);
        } 

        pub fn insert_raffle_map(
            &mut self, 
            sale_nr: u128, 
            mut v: Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)>
        ){
            self.raffle_badge_map.entry(sale_nr)                      
                                .and_modify(|z| z.append(&mut v))
                                .or_insert(v); 
        } 
        
        pub fn badge_in(&mut self, nmbr: u128, bdg: ResourceAddress, j: u8) -> bool {  
            let mut founded = false; 
            match j {
                0 => {  
                    for (key,v) in self.buy_prop_badge_map.iter() { 
                        for value in v {
                            if *key == nmbr && value.0 == bdg {
                                founded = true;
                                break;
                            } 
                        } 
                    }
                }  
                1 => { 
                    for (key,v) in self.auction_badge_map.iter() {  
                        for value in v {
                            if *key == nmbr && value.0 == bdg {
                                founded = true;
                                break;
                            } 
                        } 
                    }
                }  
                _ => {  
                    for (key,v) in self.raffle_badge_map.iter() {
                        for value in v {
                            if *key == nmbr && value.1 == bdg {
                                founded = true;
                                break;
                            } 
                        }        
                    }
                }
            }
            founded
        }

            ///raffle

            // Update past raffle bids in related map
        pub fn update_raffle_map(&mut self, nmbr: u128) {
            for (key,v) in self.raffle_badge_map.iter_mut() {
                for mut value in v {
                    if *key == nmbr {
                        match value.4 {
                            0 => value.4 = 2,
                            2 => (),
                            _ => value.4 = 4
                        }
                    } 
                }        
            }     
        } 

        pub fn erase_map_entry(&mut self, nmbr: u128, a: u64, b: u8) {
            for (key,v) in self.raffle_badge_map.iter_mut() {
                if *key == nmbr {
                    match b {
                        0 => v.retain(|x| x.3 != a),
                        _ => ()
                    }
                } 
            }
        } 

        pub fn collect_jackpot(&mut self, nmbr: u128) -> (ResourceAddress,ResourceAddress,usize) {
            let mut caller_badge_addr = ResourceAddress::Normal([0; 26]); 
            let mut new_badge = ResourceAddress::Normal([0; 26]);          
            let mut flag = 0;
            for (key,v) in self.raffle_badge_map.iter_mut() {
                for mut value in v {
                    if *key == nmbr {                   
                        caller_badge_addr = value.5;    
                        match value.4 {
                            1 => { 
                                new_badge = value.1;    
                                value.4 = 3;
                                flag = 1;               
                                break;
                            }
                            3 => { 
                                flag = 3;
                                break;
                            }
                            4 => flag = 2,
                            _ => ()
                        } 
                    } 
                }        
            }

            (caller_badge_addr,new_badge,flag)
        }

        pub fn buy_ticket_ext(&mut self, sale_nr: u128, ttl: Decimal, end: u64, new_end: bool) { 
            for (key,v) in self.raffle_badge_map.iter_mut() {
                for mut value in v {
                    if *key == sale_nr && value.3 >= end {  
                        value.2 = ttl;                  
                        if new_end {                        
                            value.3 += 1; 
                        }
                    } 
                }        
            } 
        }

        pub fn reclaim_ticket(
            &mut self, 
            nmbr: u128, 
            bdg_addr: ResourceAddress,
            prc: Decimal
        ) -> (usize,Decimal,(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)) {
            let zero_addr = ResourceAddress::Normal([0; 26]); 
            let mut tup = (0,zero_addr,dec!("0"),0,0,zero_addr);
            let mut wave = 0;
            let mut sum_back = Decimal::zero();

            for (key,v) in self.raffle_badge_map.iter_mut() {
                for (ticket_id,badge_address,jackpot,endtime,flag,caller_bdg) in v.clone(){
                    if *key == nmbr && bdg_addr == badge_address {
                        //assert!( endtime < Runtime::current_epoch(), " Raffle still alive "); // Check be4 erase it
                        match flag {
                            0 => {                                      
                                    sum_back += prc; 
                                    v.retain(|x| x.1 != badge_address);                       
                            }
                            1 => {  
                                    tup = (ticket_id,badge_address,jackpot,endtime,3,caller_bdg);
                                    v.retain(|x| x.4 == 4 || x.4 == 2);
                                    wave = 1;
                                    break;
                            }
                            2 => {                                      
                                    sum_back += prc; 
                                    v.retain(|x| x.1 != badge_address);                     
                            }                                    
                            _ => {
                                    v.retain(|x| x.4 == 4 || x.4 == 2);
                                    wave = 1;
                                    break;
                            }
                        }
                    }
                }
            }

            (wave,sum_back,tup)
        }

        pub fn raffle_map_insert(
            &mut self, 
            nmbr: u128, 
            v: Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)>
        ) {
            self.raffle_badge_map.insert(nmbr,v);
        }

        pub fn raffle_winner(
            &mut self, 
            nmbr: u128 
        ) -> Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)> {
            let mut w: Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)> = Vec::new();
            for (key,v) in self.raffle_badge_map.iter() {
                for value in v {
                    if *key == nmbr && value.4 == 0 {
                        w.push(*(value));
                    } 
                }        
            } 

            w
        }

        pub fn update_raffle_winner(&mut self, id: u128, bdg: ResourceAddress){
            for (_key,v) in self.raffle_badge_map.iter_mut() {        
                for mut value in v {
                    if value.0 == id && value.1 == bdg {
                        value.4 = 1;
                        break;
                    }
                }
            }
        }

        pub fn raffle_badge_map(&mut self, sale_nr: u128, badge: ResourceAddress) -> RaffleBdgMap {
            let mut out_map = HashMap::new();
            for (key,v) in self.raffle_badge_map.clone().into_iter() {
                for val in v.clone() {
                    if badge != ResourceAddress::Normal([0; 26]) { 
                        if key == sale_nr && val.1 == badge {
                            raffle_badge_map(RaffleTuple { tuple: val });
                        } 
                    } else {
                        if key == sale_nr {
                            raffle_badge_map(RaffleTuple { tuple: val });
                        } 
                    }
                }
                out_map.insert(key,v);        
            }

            RaffleBdgMap { map:out_map }
        }

            ///auction

            // Update past auction bids in related map
        pub fn update_auction_map(&mut self, nmbr: u128){
            for (key,v) in self.auction_badge_map.iter_mut() { 
                for mut value in v {
                    if *key == nmbr {
                        match value.3 {
                            1 => value.3 = 3,
                            3 => value.3 = 3,
                            _ => value.3 = 2
                        }
                    }                           
                }       
            } 
        } 

        pub fn collect_auction_payment(&mut self, nmbr: u128) -> ResourceAddress {
            let mut caller_badge_addr = ResourceAddress::Normal([0; 26]);                
            let mut wave = false;
            let mut j = 0; 
            let v = self.auction_badge_map.get_mut(&nmbr).unwrap();
                for value in v.clone() {
                    info!(" accrued_amount {} ",value.1);
                    if value.3 == 1 || value.3 == 3 {
                        caller_badge_addr = value.5;                  
                        wave = true; 
                        v.remove(j);            
                        break;
                    }  
                    j += 1;
                }
            assert!(wave," Correspondence unfounded! ");

            caller_badge_addr
        }

            // Update current winning bid in related map and check if bidder badge is present  
        pub fn place_bid(
            &mut self, 
            nmbr: u128, 
            bdr_addr: ResourceAddress, 
            s: u8, 
            bid: Decimal, 
            new_end: bool,
            mut wave: bool
        ) -> bool { 
            for (key,v) in self.auction_badge_map.iter_mut() { 
                for mut value in v {
                    if *key == nmbr && value.3 < 2 {
                        if value.0 != bdr_addr && Runtime::current_epoch() <= value.2 {
                            value.3 = 0;
                        } else {    
                            if s == 1 {
                                value.3 = 1;
                            }
                            value.1 = bid;
                            wave = false;   // User provided a bidder badge already minted
                        }
                        if new_end {
                            value.2 += 1; 
                        }
                    } 
                } 
            } 

            wave
        }

        pub fn pay_winner_bid(&mut self, nmbr: u128, bidder_badge_addr: ResourceAddress) {
            let mut wave = false;
            for (key,v) in self.auction_badge_map.iter_mut() { 
                for value in v { 
                    if *key == nmbr && value.0 == bidder_badge_addr && value.3 == 1 {
                        wave = true;
                        break;
                    }
                }        
            } 
            assert!(wave," Check Badge ");
        }

        pub fn reclaim_bond(
            &mut self, 
            nmbr: u128, 
            bidder_bdg: ResourceAddress,
            auction_dl: u64
        ) -> (Decimal,bool,bool,bool) {
            let mut answer = false;
            let mut winner_flag = false;
            let mut burn_badge_flag = false;
            let mut bid_bond = dec!("0");
            for (key,v) in self.auction_badge_map.iter_mut() {         
                for value in v.clone() {
                    if *key == nmbr && value.0 == bidder_bdg && value.2 < Runtime::current_epoch() { 
                        if value.3 != 1 && value.3 != 3 {
                            bid_bond = value.4;
                            info!(" Bid bond {} ",bid_bond);
                            answer = true;
                            v.retain(|x| x.0 != bidder_bdg);                       
                            break;  
                        } else if value.2+auction_dl >= Runtime::current_epoch() {
                            info!(
                                " You won the Auction. Payment Deadline: {} Bid: {} $TKN ",
                                value.2+auction_dl, value.1
                            );
                            winner_flag = true;
                            break;
                        } else {
                            info!(" You won the Auction. Payment Deadline outdated ");
                            v.retain(|x| x.0 != bidder_bdg);
                            winner_flag = true;   
                            burn_badge_flag = true;                    
                            break;
                        }        
                    } 
                }
            } 

            (bid_bond,answer,winner_flag,burn_badge_flag)
        }

        pub fn auction_badge_map(&mut self, sale_nr: u128) -> AuctionBdgMap { 
            let mut out_map = HashMap::new();
            for (key,v) in self.auction_badge_map.clone().into_iter() {  
                for val in v.clone() {
                    if key == sale_nr {
                        auction_badge_map(AuctionTuple { tuple: val });
                    } 
                } 
                out_map.insert(key,v);     
            }

            AuctionBdgMap { map:out_map }        
        } 

            ///buy_prop

        pub fn collect_buy_proposal(
            &mut self, 
            nmbr: u128, 
            amount: Decimal
        ) -> (ResourceAddress,ResourceAddress,Decimal) {
            let mut caller_badge_addr = ResourceAddress::Normal([0; 26]); 
            let mut new_badge = ResourceAddress::Normal([0; 26]);            
            let mut max_value = amount;
            let mut wave: bool = false; 
            for (key,v) in self.buy_prop_badge_map.iter_mut() { 
                for value in v {
                    if *key == nmbr && value.3 == 1 {
                        max_value = value.1;
                        new_badge = value.0;
                        caller_badge_addr = value.4; 
                        value.3 = 2;                 
                        wave = true; 
                        break;
                    } 
                } 
            }
            assert!(wave," Correspondence unfounded! "); 
            assert!(max_value >= amount," Buy proposal received is lower then inserted amount! ");

            (caller_badge_addr,new_badge,max_value)
        }

        pub fn buy_proposal_ext(&mut self, nmbr: u128){
            for (key,v) in self.buy_prop_badge_map.iter_mut() { 
                for mut value in v {
                    if *key == nmbr {
                        value.3 = 0;
                    } 
                } 
            }
        }

        pub fn reclaim_prop(&mut self, nmbr: u128, bdg_addr: ResourceAddress) -> (u8,Decimal,u64) {
            let mut ex_flag = 3;
            let mut ex_amnt = dec!("0");
            let mut ex_endtime = u64::MAX;
            for (key,v) in self.buy_prop_badge_map.iter_mut() { 
                for (badge_address,amnt,endtime,flag,_caller_bdg) in v.clone() {
                    if *key == nmbr && bdg_addr == badge_address {
                        ex_flag = flag;
                        ex_amnt = amnt;
                        ex_endtime = endtime;
                        break;
                    }
                }      
            }

            (ex_flag,ex_amnt,ex_endtime) 
        }

        pub fn remove_prop(&mut self, nmbr: u128, bdg_addr: ResourceAddress) {
            let mut wave = false;
            for (key,v) in self.buy_prop_badge_map.iter_mut() { 
                let mut i = 0;
                for value in v.clone() {
                    if *key == nmbr && value.0 == bdg_addr {              
                        wave = true; 
                        v.remove(i);    
                        break;
                    }
                    i += 1;
                } 
            }
            assert!(wave," Correspondence unfounded! ");
        }

        pub fn buy_prop_badge_map(&mut self, sale_nr: u128) -> BuyPropBdgMap { 
            let mut out_map = HashMap::new();
            for (key,v) in self.buy_prop_badge_map.clone().into_iter() {
                for val in v.clone() {
                    if key == sale_nr {
                        buy_prop_badge_map(BuyPropTuple { tuple: val });
                    } 
                }
                out_map.insert(key,v);           
            } 

            BuyPropBdgMap { map:out_map }  
        } 
    }
}
