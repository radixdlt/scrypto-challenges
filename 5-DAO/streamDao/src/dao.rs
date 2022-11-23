use crate::structs::proposal::{Proposal, Status};
use scrypto::prelude::*;

const MINIMUM_VOTING_TIME: u64 = 20u64;

blueprint! {

  struct Dao {
    proposals: Vault,

    proposals_address: ResourceAddress,

    proposals_ids: HashSet<NonFungibleId>,

    votes: HashMap<ResourceAddress, Vault>,

    admin_badge: Vault,

    admin_badge_address: ResourceAddress,
  }

  impl Dao {

    pub fn new() -> DaoComponent {

      let admin_badge = ResourceBuilder::new_fungible()
        .divisibility(DIVISIBILITY_NONE)
        .metadata("name", "Dao Admin Badge")
        .initial_supply(1);

      let admin_rule: AccessRule = rule!(require(admin_badge.resource_address()));

      let proposals_address = ResourceBuilder::new_non_fungible()
        .metadata("name", "Proposal")
        .metadata("description", "NFT representing a proposal")
        .burnable(admin_rule.clone(), LOCKED)
        .mintable(admin_rule.clone(), LOCKED)
        .updateable_non_fungible_data(admin_rule.clone(), LOCKED)
        .no_initial_supply();

      Self {
        proposals: Vault::new(proposals_address),
        proposals_address,
        proposals_ids: HashSet::new(),
        votes: HashMap::new(),
        admin_badge_address: admin_badge.resource_address(),
        admin_badge: Vault::with_bucket(admin_badge),
      }.instantiate()

    }

    pub fn create_proposal(
      &mut self,
      choices: Vec<String>,
      ipfs_link: String,
      from: NonFungibleId,
      start: u64,
      end: u64,
     ) -> Bucket {

      assert!(
        start < end,
        "[CREATE PROPOSAL] End time cannot be less than start time."
      );

      assert!(
        start.abs_diff(end) >= MINIMUM_VOTING_TIME,
        "[CREATE PROPOSAL] Proposal does not have a minimum voting time."
      );

      let mut choices_address: HashMap<String, ResourceAddress> = HashMap::new();

      for choice in &choices {
        let choice_address = ResourceBuilder::new_fungible()
        .divisibility(DIVISIBILITY_NONE)
        .metadata("choice", choice.to_uppercase())
        .metadata("description", "This resource that represents a choice")
        .mintable(rule!(require(self.admin_badge_address)), LOCKED)
        .burnable(rule!(require(self.admin_badge_address)), LOCKED)
        .restrict_withdraw(rule!(require(self.admin_badge_address)), LOCKED)
        .restrict_deposit(rule!(require(self.admin_badge_address)), LOCKED)
        .no_initial_supply();

        choices_address.insert(choice.clone(), choice_address);

        self.votes.insert(choice_address, Vault::new(choice_address));
      }

      let proposal_data = Proposal {
        choices: choices_address,
        end,
        from,
        start,
        ipfs_link,
        timestamp: Runtime::current_epoch(),
        voters: HashMap::new(),
        status: Status::Pending,
        winner:("".to_string(), dec!("0")),
      };

      let proposal = self.admin_badge.authorize(||{
        borrow_resource_manager!(self.proposals_address).mint_non_fungible(
          &NonFungibleId::random(),
          proposal_data
        )
      });

      self.proposals_ids.insert(proposal.non_fungible_id());

      proposal
    }

    pub fn deposit_proposal(&mut self, proposal:Bucket) {

      assert!(
        proposal.amount() == dec!("1"),
        "[DEPOSIT PROPOSAL] Only one proposal is allowed."
      );

      proposal.create_proof().validate_proof(
        ProofValidationMode::ValidateResourceAddress(self.proposals_address)
      ).expect("[DEPOSIT PROPOSAL] This proposal is not valid.").drop();

      let mut proposal_data:Proposal = proposal.non_fungible::<Proposal>().data();

      assert!(
        Runtime::current_epoch().abs_diff(proposal_data.end) >= MINIMUM_VOTING_TIME,
        "[DEPOSIT PROPOSAL] The proposal voting time is no longer valid."
      );

      proposal_data.status = Status::Active;

      self.admin_badge.authorize(|| {
        proposal.non_fungible().update_data(proposal_data)
      });

      self.proposals.put(proposal);

    }

    pub fn vote_proposal(
      &mut self,
      proposal_id: NonFungibleId,
      voter_id: NonFungibleId,
      choice: (String, Decimal),
    ){

      assert!(
        self.proposals_ids.contains(&proposal_id),
        "[ERROR VOTE PROPOSAL] - This proposal does not exist."
      );

      let proposal = self.proposals.take_non_fungible(&proposal_id);

      let mut proposal_data = proposal.non_fungible::<Proposal>().data();

      assert!(
        proposal_data.status == Status::Active,
        "[ERROR VOTE PROPOSAL] - This proposal does not active."
      );

      assert!(
        proposal_data.choices.contains_key(&choice.0),
        "[ERROR VOTE PROPOSAL] - This choice does not exist."
      );

      assert!(
        proposal_data.start <= Runtime::current_epoch(),
        "[ERROR VOTE PROPOSAL] - This proposal does not voting started."
      );

      assert!(
        proposal_data.end > Runtime::current_epoch(),
        "[ERROR VOTE PROPOSAL] - Voting period for this proposal finalized."
      );

      assert!(
        !(proposal_data.voters.contains_key(&voter_id)),
        "[ERROR VOTE PROPOSAL] - You already voted for this proposal."
      );

      let choice_address = proposal_data.choices.get(&choice.0).unwrap().clone();

      let vote = self.admin_badge.authorize(|| {
        borrow_resource_manager!(choice_address).mint(choice.1)
      });

      self.admin_badge.authorize(||self.votes.get_mut(&choice_address).unwrap().put(vote));

      proposal_data.voters.insert(voter_id, choice);

      self.admin_badge.authorize(||{
       proposal.non_fungible().update_data(proposal_data);
      });

      self.proposals.put(proposal);
    }

    pub fn resolve_proposal(&mut self, proposal_id:NonFungibleId) {
      assert!(
        self.proposals_ids.contains(&proposal_id),
        "[RESOLVE PROPOSAL] - This proposal does not exist."
      );

      let propasal = self.proposals.take_non_fungible(&proposal_id);
      let mut proposal_data = propasal.non_fungible::<Proposal>().data();

      assert!(
        proposal_data.end < Runtime::current_epoch(),
        "[RESOLVE PROPOSAL] - This proposal is still up for a vote."
      );

      let mut winner:(String, Decimal) = ("".to_string(), dec!("0"));

      for (choice, choice_resource) in &proposal_data.choices {
        let vote_amount = self.votes.get(&choice_resource).unwrap().amount();
        if vote_amount > winner.1 {
          winner.0 = choice.clone();
          winner.1 = vote_amount;
        }
      }

      proposal_data.status = Status::Closed;
      proposal_data.winner = winner;

      self.admin_badge.authorize(|| propasal.non_fungible().update_data(proposal_data));
      self.proposals.put(propasal);

      self.show_proposal_data(proposal_id);
    }

    pub fn show_proposal_data(&mut self, proposal_id: NonFungibleId) {

      assert!(
        self.proposals_ids.contains(&proposal_id),
        "[SHOW PROPOSAL] - This proposal does not exist."
      );

      let propasal = self.proposals.take_non_fungible(&proposal_id);
      let proposal_data = propasal.non_fungible::<Proposal>().data();

      info!("[PROPOSAL ID]: {:?}", proposal_id);
      info!("[PROPOSAL STATUS]: {:?}", proposal_data.status);
      info!("[PROPOSAL CHOICES]: {:?}", proposal_data.choices);
      info!("[PROPOSAL FROM]: {:?}", proposal_data.from);
      info!("[PROPOSAL IPFS LINK]: {:?}", proposal_data.ipfs_link);
      info!("[PROPOSAL START]: {:?}", proposal_data.start);
      info!("[PROPOSAL END]: {:?}", proposal_data.end);
      info!("[PROPOSAL VOTERS]: {:?}", proposal_data.voters);

      for (resource, vault) in &self.votes {
        for (choice, choice_resource) in &proposal_data.choices {
          if resource == choice_resource {
            info!("[PROPOSAL VOTE] Choice: {} => Amount: {}", choice, vault.amount());
          }
        }
      }
      info!("[PROPOSAL WINNER]: {:?}", proposal_data.winner);

      self.proposals.put(propasal);
    }

    // /////////////////////// [FOR TESTING] //////////////////////

    // pub fn create_proposal_set_id(
    //   &mut self,
    //   proposal_id: u64,
    //   choices: Vec<String>,
    //   ipfs_link: String,
    //   from: NonFungibleId,
    //   start: u64,
    //   end: u64,
    //  ) -> Bucket {

    //   let mut choices_address: HashMap<String, ResourceAddress> = HashMap::new();

    //   for choice in &choices {
    //     let choice_address = ResourceBuilder::new_fungible()
    //     .divisibility(DIVISIBILITY_NONE)
    //     .metadata("choice", choice.to_uppercase())
    //     .metadata("description", "This resource that represents a choice")
    //     .mintable(rule!(require(self.admin_badge_address)), LOCKED)
    //     .burnable(rule!(require(self.admin_badge_address)), LOCKED)
    //     .restrict_withdraw(rule!(require(self.admin_badge_address)), LOCKED)
    //     .restrict_deposit(rule!(require(self.admin_badge_address)), LOCKED)
    //     .no_initial_supply();

    //     choices_address.insert(choice.clone(), choice_address);
    //     self.votes.insert(choice_address, Vault::new(choice_address));
    //   }

    //   let proposal_data = Proposal {
    //     choices: choices_address,
    //     end,
    //     from,
    //     start,
    //     ipfs_link,
    //     timestamp: Runtime::current_epoch(),
    //     voters: HashMap::new(),
    //     status: Status::Pending,
    //     winner:("".to_string(), dec!("0")),
    //   };

    //   let proposal = self.admin_badge.authorize(||{
    //     borrow_resource_manager!(self.proposals_address).mint_non_fungible(
    //       &NonFungibleId::from_u64(proposal_id),
    //       proposal_data
    //     )
    //   });

    //   self.proposals_ids.insert(proposal.non_fungible_id());

    //   proposal
    // }
  }
}
