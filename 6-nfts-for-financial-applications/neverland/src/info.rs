use scrypto::prelude::*;
use crate::data_square::*;

// UI console info display

pub fn amount(amount: Decimal){
    info!( " amount : {} ",amount);
}

pub fn collect_payment(accrued_tokens: Decimal, amount: Decimal){
    info!(" Accrued tokens {} ", accrued_tokens);   
    info!(" NFT accrued selling amount {} ", amount);
}

pub fn requested_amount(amount: Decimal){
    info!(" Requested amount: {} ", amount);
}

pub fn display_rest(rest: Decimal){
    info!(" Rest {} ", rest);
}

pub fn dex_output_amount(amount: Decimal){
    info!(" Asset Dex external currency output amount: {} ", amount);
}

pub fn net_gain(rest: Decimal){
    info!(" NFT sell net gain {} ",rest);
}    

pub fn net_fee(sum_one: Decimal, protocol_royalty: Decimal){
    info!(" marketplace net fee {} ",sum_one);
    info!(" royalty {} ",protocol_royalty);
}

pub fn royalty(protocol_fee: Decimal){
    info!(" royalty {} ",protocol_fee);
}

pub fn royalty_placed(amount: Decimal){
    info!(" $TKN royalty placed in AssetSquare vault {} ",amount);
}

pub fn instance_number(nmbr: u128){
    info!(" =====================================================================================");
    info!(" Instance number: {} ",nmbr);
    info!(" =====================================================================================");
}

pub fn bid_bond(bid_bond: Decimal){
    info!(" Bid bond: {} ",bid_bond);
}

pub fn new_price(price: Decimal){
    info!(" New price set @{} $TKN ",price);
}

pub fn higher_amount(amount: Decimal){
    info!(" Pls provide an amount higher then {}",amount);
}

pub fn unauthorized(){
    info!(" Auction is live or badge is unauthorized ");
}

pub fn time_unreached(ex_end: u64){
    info!(" Endtime unreached! {} ",ex_end);
}

pub fn nft_reserve_amount(nft_addr: ResourceAddress, total_nft: usize){
    info!(" {} nft reserve amount is {} ", nft_addr, total_nft);
}

pub fn unfound(flag: u8) {
    match flag {
        0 => info!(" Found no NFT in stock! "),
        1 => info!(" Could not find NFT in stock !"),
        2 => info!(" Unfound badge correspondence within map! "),
        3 => info!(" Unfound correspondence "),
        7 => info!(" Unable to stock NFT "),
        _ => ()
    }
    if flag != 0 {
        std::process::abort()
    }
}

pub fn unfound_bckt(flag: u8) -> Bucket {
    match flag {
        4 => info!(" NFT not in stock! "),
        5 => info!(" Caller Badge not in stock! "),
        6 => info!(" Unavailable amount "),
        _ => ()
    }
    std::process::abort()
}

pub fn values(
    protocol_fee: Decimal, 
    protocol_royalty: Decimal, 
    tkn_currency: ResourceAddress,
    protocol_square: ComponentAddress,
    tkn_vault: ComponentAddress
){
    info!(" =====================================================================================");
    info!(" AssetSquare fee set to {}% ", protocol_fee);
    info!(" Asset royalty fee set to {}% ", protocol_royalty);
    info!(" TKN currency {} ", tkn_currency);
    info!(" AssetSquare component address set to {} ", protocol_square);
    info!(" TKN Vault component address set to {} ", tkn_vault);
    info!(" =====================================================================================");
}

pub fn academy_values(academy_vault: ComponentAddress, academy_share: Decimal) {
    info!(" =====================================================================================");
    info!(" Academy Vault Component Address set to {} ", academy_vault);
    info!(" Academy share fee set to {}% ", academy_share);
    info!(" =====================================================================================");
}

pub fn protocol_fee(fee: Decimal) {
    info!(" Protocol fee set to {}% ", fee);
}

pub fn deadlines(auction_dl: u64, last_bid_dl: u64, buy_proposal_dl: u64) {
    info!(" =====================================================================================");
    info!(" Auction deadline set to {} ", auction_dl);
    info!(" Auction last bid deadline set to {} ", last_bid_dl);
    info!(" Buy proposal deadline set to {} ", buy_proposal_dl);
    info!(" =====================================================================================");
}

pub fn view_ext_sbt_map(key: ResourceAddress, value: ComponentAddress) {                                                             
    info!("======================================================================================");
    info!("caller badge: {}",key);
    info!("ext comp addr: {}",value);        
}

pub fn buy_prop_badge_map(bpm: BuyPropTuple){
    info!(" =====================================================================================");
    info!(
        "   Buyer Badge: 
        {}
            Proposal:        {} $TKN 
            End Time:        {} Epoch
            Higher proposal: {} $TKN 
            Caller Badge:       
        {} ",
        bpm.tuple.0,bpm.tuple.1,bpm.tuple.2,bpm.tuple.3,bpm.tuple.4
    );
    info!(" =====================================================================================");
}

pub fn auction_badge_map(abm: AuctionTuple){
    info!(" =====================================================================================");
    info!(
        "   Bidder Badge: 
        {}
            Bid:        {} $TKN 
            End Time:   {} Epoch
            Status:     {} 
            Bid Bond:   {} $TKN 
            Caller Badge:       
        {} ",
        abm.tuple.0,abm.tuple.1,abm.tuple.2,abm.tuple.3,abm.tuple.4,abm.tuple.5
    );
    info!(" =====================================================================================");
}

pub fn raffle_badge_map(rbm: RaffleTuple){
    info!(" =====================================================================================");
    info!(
        "   Ticket ID: {} 
            Ticket Badge: 
        {}
            Jackpot:        {} $TKN 
            End Time:       {} Epoch     
            Status:         {} 
            Caller Badge:       
        {} ",
        rbm.tuple.0,rbm.tuple.1,rbm.tuple.2,rbm.tuple.3,rbm.tuple.4,rbm.tuple.5
    );
    info!(" =====================================================================================");
}

pub fn ext_mrkt(
    mrkt: ComponentAddress, 
    bdg: ResourceAddress, 
    fee: Decimal, 
    fx: ResourceAddress, 
    bdg_addr: ResourceAddress
){
    info!(" =====================================================================================");
    info!(" Ext Marketplace address: {}",mrkt);
    info!(" Ext badge address: {}",bdg);
    info!(" Ext Marketplace fee: {}",fee);
    info!(" Ext Marketplace currency: {}",fx);
    info!(" Caller badge address: {}",bdg_addr);
}

pub fn meta_map(addr: ResourceAddress, s: Decimal, id: NonFungibleId, mid: NonFungibleId, nr: u128){
    info!(" =====================================================================================");
    info!(" NFT address: {} ", addr);
    info!(" sum: {} ",s);
    info!(" NFT key: {} ",id);
    info!(" metaNFT key: {} ",mid);
    info!(" instance number: {} ",nr);
}

pub fn meta(addr: ResourceAddress, meta_addr: ResourceAddress){
    info!(" NFT address: {} ",addr);
    info!(" metaNFT address: {} ",meta_addr);
}

pub fn stock(
    amnt: Decimal, 
    addr: ResourceAddress, 
    e: String, 
    s: String, 
    nr: String, 
    key: NonFungibleId, 
    data: AssetNFT, 
    price: Decimal
){
    info!(" Added {} NFT, {} ResAddress {} Ecosystem {} Series {} Number ",amnt,addr,e,s,nr);
    info!(" UUID: {} Data: {} {} @{} $TKN ",key,data.data_1,data.data_2,price); 
}

pub fn nft_mint(protocol_uuid: NonFungibleId, protocol_addr: ResourceAddress){
    info!(" Asset_NFT_id {} ",protocol_uuid);
    info!(" Asset_NFT_res_addr {} ",protocol_addr);
    info!(" =====================================================================================");
}

pub fn meta_mint(meta_uuid: NonFungibleId, meta_addr: ResourceAddress){
    info!(" meta_NFT_id {} ",meta_uuid);
    info!(" meta_NFT_res_addr {} ",meta_addr);
    info!(" =====================================================================================");
}

pub fn instance(nmbr: u128){
    info!(" =====================================================================================");
    info!(" Instance number: {}",nmbr);
}

pub fn winner(id: u128, bdg: ResourceAddress){
    info!(" -------------------------------------------------------------------------------------");
    info!(" Winner ID: {}", id);
    info!(" Winner Badge: {}", bdg);
    info!(" -------------------------------------------------------------------------------------");
}

pub fn list_nft(
    nft_addr: ResourceAddress, 
    v: Vec<(u128,NonFungibleId,Decimal,bool)>, 
    addr: ResourceAddress
){
    for data in v.clone() { 
        if data.3 {
            info!(
                " Resource Address {} 
                Instance number {} 
                NFT ID: {}  @ {} ${} ", 
                nft_addr, 
                data.0.to_string(), 
                data.1.to_string(), 
                data.2.to_string(),
                borrow_resource_manager!(addr).metadata()["symbol"].clone()
            );
        }
    } 
}

pub fn picked(addr: ResourceAddress, key: NonFungibleId){
    info!(" =====================================================================================");
    info!(" NFT collected ");
    info!(" resource address: {} ",addr);
    info!(" key: {} ",key);
    info!(" =====================================================================================");
}

pub fn settings(fee: Decimal, tkn: Tkn) -> Tkn {
    info!(" =====================================================================================");
    info!(" Marketplace fee set to {}% ", fee);
    info!(" AssetSquare fee set to {}% ", tkn.fee);
    info!(" Asset royalty fee set to {}% ", tkn.royalty);
    info!(" TKN token {} ", tkn.currency);
    info!(" AssetSquare address set to {} ", tkn.square);
    info!(" Asset Badge address set to {} ", tkn.badge);
    info!(" Asset Oracle address set to {} ", tkn.oracle);
    info!(" Asset Vault address set to {} ", tkn.vault);
    info!(" Academy Vault address set to {} ", tkn.academy_vault);
    info!(" Academy Share set to {}% ", tkn.academy_share);
    info!(" Auction deadline set to {} ", tkn.auction_dl);
    info!(" Auction last bid deadline set to {} ", tkn.last_bid_dl);
    info!(" Buy proposal deadline set to {} ", tkn.buy_prop_dl);
    info!(" =====================================================================================");

    tkn
}

pub fn tkn_gains(tkn_output: Decimal) {
    info!(" Protocol accrued gain {} $TKN ", tkn_output);
} 

pub fn position(tab: Tab, auction_dl: u64) {        
    let key = tab.tuple.0;
    let value = tab.tuple.1;
    let v = value.0.get(0).unwrap();

    info!(" =====================================================================================");
    info!(" NFT: {} ",&v.0);
    info!(" NFT key: {} ",&v.1);
    info!("--------------------------------------------------------------------------------------");
    match value.1.0 {    
        0 => info!(
            " NFT on Sell
            Instance number : {}
            Price: {} $TKN 
            Buy proposal: {} $TKN 
            Deadline: {}",
            key.1,value.2.0,value.2.1,value.2.2
        ),       
        1 => info!(
            " NFT Sold.
            Instance number : {} 
            Accrued profit: {} $TKN ",
            key.1,value.1.1
        ),
        2 => info!(
            " Buy proposal Accepted. 
            Instance number : {}
            Payed amount: {} $TKN ",
            key.1,value.2.1
        ),
        12 => info!(" NFT withdrawn from sale "),
        3 => info!(
            " NFT on Auction.
            Instance number : {} 
            Reserve price: {} $TKN 
            Highest bid: {} $TKN
            Deadline: {}
            Bid bond: {}
            Last minute bid war deadline: {} ",
            key.1,value.2.0,value.2.1,value.2.2,value.2.3,value.2.4
        ), 
        4 => info!(
            " Auction ended.
            Instance number : {} 
            Reserve price: {} $TKN
            Winning bid: {} $TKN
            Payment deadline: {} 
            Bid bond: {}",
            key.1,value.2.0,value.2.1,value.2.2+auction_dl,value.2.3
        ),
        5 => info!(
            " Auction ended. Payment deadline outdated.
            Instance number : {} 
            Reserve price: {} $TKN
            Winning bid: {} $TKN
            Bid bond penalty: {} $TKN
            To claim penalty start a new auction or unstock item ",
            key.1,value.2.0,value.2.1,value.2.3
        ),
        6 => info!(
            " Auction honored & payment withdrawable.
            Instance number : {} 
            Reserve price: {} $TKN
            Accrued amount: {} $TKN ",
            key.1,value.2.0,value.2.1
        ),
        7 => info!(
            " Auction ended. Reserve price unmatched.
            Instance number : {}
            Reserve price: {} $TKN
            Higher bid: {} $TKN
            deadline: {} 
            Start a new auction or a new raffle or unstock item ",
            key.1,value.2.0,value.2.1,value.2.2
        ),
        8 => info!(
            " NFT on Raffle.
            Instance number : {} 
            Reserve price: {} $TKN
            Jackpot: {} 
            Ticket price: {} $TKN 
            Deadline: {}
            Tickets sold: {}
            Last minute tickets fomo deadline: {} ",
            key.1,value.2.0,value.2.1*value.2.5,value.2.1,value.2.2,value.2.5,value.2.4
        ), 
        9 => info!(
            " Raffle ended. 
            Instance number : {}
            Reserve price: {} $TKN 
            Jackpot: {}
            Ticket price: {} $TKN 
            Deadline: {}
            Tickets sold: {}
            Winner ticket: {} ",
            key.1,value.2.0,value.2.1*value.2.5,value.2.1,value.2.2,value.2.5,value.2.4
        ),
        10 => info!(
            " Raffle ended. Reserve price unmatched.
            Instance number : {}
            Reserve price: {} $TKN 
            Jackpot: {} 
            Deadline: {}
            Start a new raffle or a new auction or unstock item ",
            key.1,value.2.0,value.2.1*value.2.5,value.2.2
        ),  
        11 => info!(
            " Raffle succesfully concluded.
            Instance number : {} 
            Jackpot: {} $TKN redeemed
            Deadline: {}
            Winning Ticket: {}
            Please claim won Nft ",
            key.1,value.2.1*value.2.5,value.2.2,value.2.6
        ),
        _ => std::process::abort()                
    }    
    info!("--------------------------------------------------------------------------------------");                
    info!(" {} ",&v.2.data_1);
    info!(" {} ",&v.2.data_2);
    info!(" {} ",&v.2.data_3);
    info!(" {} ",&v.2.data_4);
    info!(" value_1: {} ",v.2.value_1);
    info!(" value_2: {} ",v.2.value_2);
    info!(" value_3: {} ",v.2.value_3);
    info!(" =====================================================================================");        
}

pub fn buy_badge_data_test(nft: Bucket) -> (Bucket,Mode) {    
    let nft_data: Mode = nft.non_fungible().data();
    let nmbr = nft_data.instance_nmbr;
    let addr = nft_data.mrkt_addr;
    let mode = nft_data.listing_mode;
    info!("--------------------------------------------------------------------------------------");
    info!(" Instance number: {} ",nmbr);
    info!(" Marketplace address: {} ",addr);
    match mode {
        0 => info!(" Listing mode: {}_ Normal ",mode),
        1 => info!(" Listing mode: {}_ Auction ",mode),
        _ => ()
    }   
    info!("--------------------------------------------------------------------------------------");

    (nft,nft_data)
} 

pub fn raffle_badge_data_test(nft: Bucket) -> (Bucket,TicketID) {
    let nft_data: TicketID = nft.non_fungible().data();
    let nmbr = nft_data.instance_nmbr.clone();
    let addr = nft_data.mrkt_addr.clone();
    let key = nft_data.key.clone();
    let v = nft_data.v.clone();
    info!("--------------------------------------------------------------------------------------");
    info!(" Instance number: {} ",nmbr);
    info!(" Marketplace address: {} ",addr);
    info!(" Ticket badge key: {} ",key);
    for tck_id in v.clone() {
        info!(" Ticket ID: {} ",tck_id);   
    }
    info!("--------------------------------------------------------------------------------------");

    (nft,nft_data)
}

pub fn nft_data(mut nft: Bucket) -> (Bucket,AssetNFT) {
    let mut i = 0;
    let mut nft_data: AssetNFT;
    loop {
        let nft_key_vec = nft.non_fungible_ids().into_iter().collect::<Vec<NonFungibleId>>();
        let nft_key = nft_key_vec.get(i).unwrap();
        (nft,nft_data) = retrieve_nft_data(nft_key.clone(), nft);
        i += 1;
        if Decimal::from(i) == nft.amount() {
            break;
        }
    }

    (nft,nft_data)
}

pub fn nft_data_key(nft_key: NonFungibleId , nft: Bucket) -> (Bucket,AssetNFT) {        
    retrieve_nft_data(nft_key.clone(), nft)
}

    fn retrieve_nft_data(nft_key: NonFungibleId , nft: Bucket) -> (Bucket,AssetNFT) {
        let eco = borrow_resource_manager!(nft.resource_address()).metadata()["Ecosystem"].clone();
        let series = borrow_resource_manager!(nft.resource_address()).metadata()["Series"].clone();
        let number = borrow_resource_manager!(nft.resource_address()).metadata()["Number"].clone();

        let nft_data: AssetNFT = nft.non_fungible().data();

        info!(" NFT key {} ",nft_key);
        info!(" Ecosystem: {} ",eco);
        info!(" Series: {} ",series);
        info!(" Number: {} ",number);
        info!(" data_1: {} ",nft_data.data_1);
        info!(" data_2: {} ",nft_data.data_2);
        info!(" data_3: {} ",nft_data.data_3);
        info!(" data_4: {} ",nft_data.data_4);
        info!(" value_1: {} ",nft_data.value_1);
        info!(" value_2: {} ",nft_data.value_2);
        info!(" value_3: {} ",nft_data.value_3);
        info!(" uri: {} ",nft_data.uri);
        info!("----------------------------------------------------------------------------------");

        (nft,nft_data)
    }            







