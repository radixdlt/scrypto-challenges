use scrypto::prelude::*;

// TODO Make this Parent Component for creation of member tokens
// TODO finish NFT DAO implementation
// #[derive(NonFungibleData)]
// pub struct DAO {
//   pub name: String,
//   pub desc: String,
//   pub num_shares: u32,
// }

blueprint! { 
 struct DAO {
  dao_token_vault: Vault,
 }

 impl DAO {
   pub fn instantiate_dao(dao_name: String, description: String, num_shares: u32) -> ComponentAddress {
    // possible enhancement Mint NFT defining the organization
    // define initial DAO owned assets
    let total_shares = num_shares.to_string();
// Simple v1 Create Single token representing DAO
    let dao_bucket: Bucket = ResourceBuilder::new_fungible()
         .metadata("dao_name", dao_name)
         .metadata("description", description)
         .metadata("num_shares", total_shares)
         .initial_supply(1);
    

    // define initial dao ownership structure

    // define initial operators roles & compensation
    // % of Dao Earnings

    // define member earnings structure

    // define voting levels ie common_vote, delgate_vote, founders_vote, etc.
    
     Self {
      dao_token_vault: Vault::with_bucket(dao_bucket),
     }
     .instantiate()
     .globalize()
   }
 }
}
