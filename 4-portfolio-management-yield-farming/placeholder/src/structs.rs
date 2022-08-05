use scrypto::prelude::*;

#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub struct User {
    pub funds_managed: HashMap<String, ComponentAddress>,
    pub funds_invested: HashMap<String, ComponentAddress>,
}

#[derive(Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub enum FundType {
    IndexFund,
    DebtFund,
}