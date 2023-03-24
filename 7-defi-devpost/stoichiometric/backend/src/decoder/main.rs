use std::env;
use std::num::ParseIntError;
use radix_engine_interface::address::Bech32Encoder;
use radix_engine_interface::api::types::{Decoder};
use radix_engine_interface::data::ScryptoDecoder;
use radix_engine_interface::math::Decimal;
use radix_engine_interface::model::{ResourceAddress, NonFungibleLocalId};
use radix_engine_interface::node::NetworkDefinition;
use scrypto::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).unwrap().parse::<u8>().unwrap()
    {
        1 => { //Number to decode loans
            let mutable_hex = args.get(2).unwrap();
            let immutable_hex = args.get(3).unwrap();

            decode_loan(immutable_hex, mutable_hex);
        }
        2 => { //Number to decode positions
            let mutable_hex = args.get(2).unwrap();
            let immutable_hex = args.get(3).unwrap();

            decode_position(immutable_hex, mutable_hex);
        }
        3 => { //Number to decode voter cards

            let mutable_hex = args.get(2).unwrap();

            decode_voter_card(mutable_hex);
        }

        4 => {
            let immutable_hex = args.get(2).unwrap();
            decode_proposal_receipt(immutable_hex);
        }

        _=> { panic!("No decode option for this number") }
    }
}

pub fn decode_loan(immutable_hex: &String, mutable_hex: &String) {

    let mutable_vec_bytes = decode_hex(mutable_hex).expect("The input string could not be parsed correctly");
    let mutable_bytes = mutable_vec_bytes.as_slice();
    let (collateral_amount, amount_lent): (Decimal, Decimal) = ScryptoDecoder::new(mutable_bytes).decode_payload(92).unwrap();

    let immutable_vec_bytes = decode_hex(immutable_hex).expect("The input string could not be parsed correctly");
    let immutable_bytes = immutable_vec_bytes.as_slice();
    let (collateral_token_tmp, loan_date, loan_to_value, interest_rate): (ResourceAddress, i64, Decimal, Decimal) = ScryptoDecoder::new(immutable_bytes).decode_payload(92).unwrap();

    let bech = Bech32Encoder::new(&NetworkDefinition::nebunet());
    let collateral_token = bech.encode_resource_address_to_string(&collateral_token_tmp);

    println!("{} {} {} {} {} {}", collateral_token, collateral_amount, amount_lent, loan_date, loan_to_value, interest_rate);
}

pub fn decode_position(immutable_hex: &String, mutable_hex: &String) {

    let to_decode = String::from(&mutable_hex[14..]);
    let step_positions_len = 204;
    let nb_positions = to_decode.len() / step_positions_len;

    let mut step_positions_string = String::new();

    for i in 0..nb_positions
    {
        let start = i*step_positions_len;
        let end = (i+1)*step_positions_len;
        let step_position = String::from(&to_decode[start..end]);
        let step = u16::from_str_radix(&step_position[..4], 16).unwrap();

        let liquidity = decode_decimal_hex(&step_position[6..72]);
        let last_stable_fees_per_liq = decode_decimal_hex(&step_position[72..138]);
        let last_other_fees_per_liq = decode_decimal_hex(&step_position[138..204]);

        step_positions_string = format!("{}{} {} {} {}@", step_positions_string, step, liquidity, last_stable_fees_per_liq, last_other_fees_per_liq);
    }

    step_positions_string.pop();

    let immutable_data_fixed = &immutable_hex[6..];
    let immutable_vec_bytes = decode_hex(&format!("5c{}",immutable_data_fixed)).expect("The input string could not be parsed correctly");
    let immutable_bytes = immutable_vec_bytes.as_slice();
    let other_token_address: ResourceAddress = ScryptoDecoder::new(immutable_bytes).decode_payload(92).unwrap();
    let bech = Bech32Encoder::new(&NetworkDefinition::nebunet());
    let other_token = bech.encode_resource_address_to_string(&other_token_address);

    println!("{}@{}", other_token, step_positions_string);
}

pub fn decode_voter_card(mutable_hex: &String) {

    let mutable_vec_bytes = decode_hex(mutable_hex).expect("The input string could not be parsed correctly");
    let mutable_bytes = mutable_vec_bytes.as_slice();

    let (voting_power, stablecoins_locked, positions_locked_ids, last_proposal_voted_id, proposals_voted): (Decimal, Decimal, Vec<NonFungibleLocalId>, u64, HashSet<u64>) = ScryptoDecoder::new(mutable_bytes).decode_payload(92).unwrap();

    let mut positions_ids_string = String::new();
    for id in positions_locked_ids {
        positions_ids_string = format!("{}{} ", positions_ids_string, id);
    }
    positions_ids_string.pop();

    let mut proposals_voted_string = String::new();
    for id in proposals_voted {
        proposals_voted_string = format!("{}{} ", proposals_voted_string, id);
    }
    proposals_voted_string.pop();

    println!("{}@{}@{}@{}@{}", voting_power, stablecoins_locked, positions_ids_string, last_proposal_voted_id, proposals_voted_string);
}

pub fn decode_proposal_receipt(immutable_hex: &String){
    let immutable_data_fixed = &immutable_hex[6..];
    let proposal_id_bytes = decode_hex(&format!("5c{}", immutable_data_fixed)).expect("The input string could not be parsed");

    let proposal_id: u64 = ScryptoDecoder::new(proposal_id_bytes.as_slice()).decode_payload(92).unwrap();

    println!("{}", proposal_id);

}

fn decode_decimal_hex(decimal: &str) -> Decimal {
    let decimal = format!("5c{}", decimal);
    ScryptoDecoder::new(decode_hex(&decimal).unwrap().as_slice()).decode_payload(92).unwrap()
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
