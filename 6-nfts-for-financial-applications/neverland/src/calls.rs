use scrypto::prelude::*;
use crate::data_square::*;

// Swap tokens on an external DEX
pub fn swap_fx(
    sum: Decimal, 
    fx: ResourceAddress, 
    dex: ComponentAddress, 
    tkn_bckt: Bucket
) -> Bucket {
    let method = "buy_token_sell_exact_token".to_string(); 
    let args = args![sum,fx,tkn_bckt];

    borrow_component!(dex).call::<Bucket>(&method, args)
} 

// Test external marketplace buy allowance for royalties computation.
pub fn out_currency(ext_mrkt: ComponentAddress, bdg_bckt_ref: Proof) -> ResourceAddress {
    let method = "out_currency".to_string(); 
    let args = args![bdg_bckt_ref];
                
    borrow_component!(ext_mrkt).call::<ResourceAddress>(&method, args)
}   

// Call "tkn_stock" of an external cloned marketplace component
pub fn tkn_stock(
    royalty: Bucket,
    tkn_vault: ComponentAddress
) -> Decimal {
    let method = "tkn_stock".to_string(); 
    let arg = args![royalty];
        
    borrow_component!(tkn_vault).call::<Decimal>(&method, arg)
}

// Put $TKN token academy share in Academy Vault Component 
pub fn token_lock(academy_bckt: Bucket, academy_vault: ComponentAddress){
    let method = "tkn_lock".to_string(); 
    let arg = args![academy_bckt];
    borrow_component!(academy_vault).call::<Decimal>(&method, arg);
} 

// Retrieve AssetNFT price
pub fn get_nft_price(sale_nr: u128, mrkt_addr: ComponentAddress) -> Decimal {
    let method = "get_nft_price".to_string(); 
    let args = args![scrypto_encode(&sale_nr)];

    borrow_component!(mrkt_addr).call::<Decimal>(&method, args)
}

// Get token amount from external DEX
pub fn get_token_sell_amount(
    amnt: Decimal, 
    dex: ComponentAddress, 
    fx_in: ResourceAddress, 
    fx_out: ResourceAddress
) -> Decimal {
    let method = "get_token_sell_amount_becsc".to_string(); 
    let args = args![
        scrypto_encode(&amnt),
        scrypto_encode(&fx_in),
        scrypto_encode(&fx_out)
    ];   

    borrow_component!(dex).call::<Decimal>(&method, args)
}

// Call "buy_nft_ext" of an external cloned marketplace component
pub fn buy_nft_ext(
    sale_nr: u128, 
    mrkt: ComponentAddress,
    bckt: Bucket, 
    caller_bdg_bckt_ref: Proof,
    user_sbt: Proof
) -> (Vec<Bucket>,Bucket) {
    let method = "buy_nft_ext".to_string(); 
    let args = args![sale_nr,mrkt,bckt,caller_bdg_bckt_ref,user_sbt];     
                
    borrow_component!(mrkt).call::<(Vec<Bucket>,Bucket)>(&method, args)
}

// Call "buy_ticket_ext" of an external cloned marketplace component
pub fn buy_ticket_ext(
    sale_nr: u128, 
    mrkt: ComponentAddress, 
    bckt: Bucket, 
    sum: u8,
    caller_bdg_bckt_ref: Proof
) -> (Bucket,Bucket) {
    let method = "buy_ticket_ext".to_string(); 
    let args = args![sale_nr,mrkt,bckt,sum,caller_bdg_bckt_ref];                
                 
    borrow_component!(mrkt).call::<(Bucket,Bucket)>(&method, args)
}

// Call "place_bid_ext" of an external cloned marketplace component
pub fn place_bid_ext(
    sale_nr: u128, 
    mrkt: ComponentAddress, 
    bckt: Bucket, 
    bidder_badge: Bucket,
    bid: Decimal,
    bid_bond: Decimal,
    caller_bdg_bckt_ref: Proof
) -> (Bucket,Bucket,Bucket) {
    let method = "place_bid_ext".to_string(); 
    let args = args![sale_nr,mrkt,bckt,bidder_badge,bid,bid_bond,caller_bdg_bckt_ref];                
            
    borrow_component!(mrkt).call::<(Bucket,Bucket,Bucket)>(&method, args)
}

// Call "buy_prop_ext" of an external cloned marketplace component
pub fn buy_prop_ext(
    sale_nr: u128, 
    mrkt: ComponentAddress, 
    bckt: Bucket, 
    proposal: Decimal,
    academyline: u64,
    caller_bdg_bckt_ref: Proof
) -> (Bucket,Bucket) {
    let method = "buy_proposal_ext".to_string(); 
    let args = args![sale_nr,mrkt,bckt,proposal,academyline,caller_bdg_bckt_ref];                
            
    borrow_component!(mrkt).call::<(Bucket,Bucket)>(&method, args)
}

// Call "pay_winner_bid" of an external cloned marketplace component
pub fn pay_winner_bid(mrkt: ComponentAddress, bckt: Bucket, bdg: Bucket, user_sbt: Proof) -> (Vec<Bucket>,Bucket) {
    let method = "pay_winner_bid".to_string(); 
    let args = args![bckt,bdg,user_sbt];        

    borrow_component!(mrkt).call::<(Vec<Bucket>,Bucket)>(&method, args)
}

// Call "reclaim_bid_bond" of an external cloned marketplace component
pub fn reclaim_bid_bond(mrkt: ComponentAddress, bidder_badge: Bucket) -> Vec<Bucket> {
    let method = "reclaim_bid_bond".to_string(); 
    let args = args![bidder_badge];                
            
    borrow_component!(mrkt).call::<Vec<Bucket>>(&method, args)
}

// Call "reclaim_winner_ticket" of an external cloned marketplace component
pub fn reclaim_winner_ticket(mrkt: ComponentAddress, ticket_badge: Bucket, user_sbt: Proof) -> Vec<Bucket> {
    let method = "reclaim_winner_ticket".to_string(); 
    let args = args![ticket_badge,user_sbt];                

    borrow_component!(mrkt).call::<Vec<Bucket>>(&method, args)
}

// Reclaim abuy proposal on an external cloned AssetSquare component
pub fn reclaim_buy_proposal(mrkt: ComponentAddress, ex_badge: Bucket, user_sbt:Proof) -> Vec<Bucket> {
    let method = "reclaim_buy_proposal".to_string(); 
    let args = args![ex_badge,user_sbt];

    borrow_component!(mrkt).call::<Vec<Bucket>>(&method, args)
}

// List AssetNFT addresses on an external cloned AssetSquare component
pub fn list_address(mrkt: ComponentAddress, nft_addr: ResourceAddress) -> NftVec {
    let method = "list_address".to_string(); 
    let args = args![scrypto_encode(&nft_addr)];
         
    borrow_component!(mrkt).call::<NftVec>(&method, args) 
}

// Reset values of an external cloned AssetSquare component
pub fn reset_asset_square_values(
    comp_fee: Decimal, 
    comp_royalty: Decimal,
    tkn_address: ResourceAddress,
    comp_square_comp: ComponentAddress,
    comp_vault_comp: ComponentAddress,
    comp_badge_proof: Proof,
    ext_square: ComponentAddress
) -> bool {
    let method = "set_asset_square_values".to_string(); 
    let args = args![
        comp_fee,
        comp_royalty,
        tkn_address,
        comp_square_comp,
        comp_vault_comp,
        comp_badge_proof
    ];                
        
    borrow_component!(ext_square).call::<bool>(&method, args)
}

