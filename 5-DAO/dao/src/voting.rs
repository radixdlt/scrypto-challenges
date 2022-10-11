// Primary blueprint purpose
// Validate Proof of authorized caller ie founders_badge or operators_badge to create ballots &
// create vote_tokens with total supply of num_votes param or num_shares(total supply of member tokens)

// accept votes from callers with proof of member_tokens
// deposit vote_tokens into vault corresponding to vote_option @ vote_tokens * number_member_tokens held


use scrypto::prelude::*;

blueprint! { 
 struct Voting {
    admin_badge_vault: Vault,
    // vaults collection for constructing ballot vaults
    vaults: HashMap<ResourceAddress, Vault>,
    vote_token_vault: Vault,
    total_shares: u128,
 }

 impl Voting {
     // TODO implement access rules to accept founders or operators badge for auth
   pub fn instantiate_voting(auth_badge: Proof, total_shares: u128) -> ComponentAddress {

     let admin_badge: Bucket = ResourceBuilder::new_fungible()
     .divisibility(DIVISIBILITY_NONE)
     .metadata("name", "Admin Badge")
     .metadata("description", "An admin badge used for internal functionality of creating vote tokens & ballots.")
     .initial_supply(dec!("1"));

     let vote_token: Bucket = ResourceBuilder::new_fungible()
          .divisibility(DIVISIBILITY_NONE)
          .metadata("name", "Vote Token")
          .metadata("symbol", "Vote")
          .initial_supply(total_shares);
     

      Self {
     admin_badge_vault: Vault::with_bucket(admin_badge),
     vaults: HashMap::new(),
     vote_token_vault: Vault::with_bucket(vote_token),
     total_shares: total_shares,
     }
     .instantiate()
     .globalize()
   }

  //  general DAO ballot mechanism for initiatives voted on my all members 1share = 1vote
   pub fn create_ballot(&mut self, ballot_options: HashMap<String,String>) {
     // iterate over ballot_options and create a vault/toke pair for each option
    //  ballot_options structure -> key:option_name,value:options_descritption
  }

// require proof of voters badge --> include num_member_tokens for weighted votes/delegate voters
   pub fn operators_vote(ballot_name: String, vote: String, num_votes: u32) {
     // general purpose voting mechanism for internal initiatives voted on by operators only
    // collect votes
    // deposit signed ballot w/num_votes * vote_token into vault with corresponding ballot_name/vote

    // report results
   }

  //  Special mechanism for election of voter delegates and DAO operator positions
   pub fn create_election() {
    // construct ballots
    // create vault to collect votes
    }

   pub fn tally_votes(ballot_id: ResourceAddress) {
     // get list of vaults assaciated with ballot_id
     // count num of tokens for each ballot vault
     // evaluate ballot vaults to determine ranked results
     // report results
   }

   pub fn simple_nft_vote() {
    // construct simple list ballot + config num of options that can be selected
    // mint nft with ballot selections and deposit into nft_proposals_vault
    // voter must present proof of required badge, ie. members badge, delegate badge or other acceptable badge.    
   }

 }
}
