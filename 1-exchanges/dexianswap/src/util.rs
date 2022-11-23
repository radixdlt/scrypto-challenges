use scrypto::prelude::*;

// utility funcs
pub fn get_pool_token_pair(base_address: Address, quote_address: Address) -> String {
    let base_name = get_token_symbol(base_address);
    let quote_name = get_token_symbol(quote_address);
    return format!("LP {}-{}", base_name, quote_name);
}

pub fn get_token_symbol(address: Address) -> String{
    return match ResourceDef::from(address).metadata().get("symbol"){
        Some(s) => format!("{}", s),
        None => format!("{}", address)
    }
}