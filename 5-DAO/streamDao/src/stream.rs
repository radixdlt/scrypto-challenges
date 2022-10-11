use scrypto::prelude::*;

use crate::{
    dao::*,
    structs::{channel::Channel, creator::Creator, membership::Membership},
};

// duration of a subscription
const END_SUBSCRIBE: u64 = 10;

// period between claiming reward
const CLAIM_REWARDS: u64 = 2;

blueprint! {

    struct Streamdao{

      // where the data for each channel is stored.
      channels: HashMap<String, Channel>,

      // every channel has a DAO component
      daos: HashMap<String, DaoComponent>,

      admin_badge: Vault,

      admin_badge_address: ResourceAddress,

      mint_membership_address: ResourceAddress,

      mint_creator_address: ResourceAddress,

      subscription_price: Decimal,

      amount_rewards_subscription: Decimal,

      create_channel_price: Decimal,

      amount_rewards_creating_channel: Decimal,

      // donation that the content creator receives from its subscribers
      donations_channel_received: HashMap<NonFungibleId, Vault>,

      platform_fee:Decimal,

      collect_xrd: Vault,
    }

    impl Streamdao{
      pub fn instantiate_streamdao(
        subscription_price: Decimal,
        create_channel_price: Decimal,
        amount_rewards_subscription: Decimal,
        amount_rewards_creating_channel: Decimal,
        platform_fee: Decimal,
    ) -> ComponentAddress {

        Streamdao::assert_price(subscription_price);
        Streamdao::assert_price(create_channel_price);

        Streamdao::assert_reward(amount_rewards_subscription);
        Streamdao::assert_reward(amount_rewards_creating_channel);

        Streamdao::assert_fee(platform_fee);

        let admin_badge = ResourceBuilder::new_fungible()
          .divisibility(DIVISIBILITY_NONE)
          .metadata("name", "Streamdao Admin Badge")
          .initial_supply(1);

        let admin_rule:AccessRule = rule!(require(admin_badge.resource_address()));

        let member_address = ResourceBuilder::new_non_fungible()
          .metadata("name", "Membership")
          .metadata("description", "NFT key that gives access to unlock a variety of services and rewards")
          .burnable(admin_rule.clone(), LOCKED)
          .mintable(admin_rule.clone(), LOCKED)
          .updateable_non_fungible_data(admin_rule.clone(), LOCKED)
          .no_initial_supply();

        let creator_address = ResourceBuilder::new_non_fungible()
          .metadata("name", "Creator")
          .metadata("description", "NFT representing a creator")
          .burnable(admin_rule.clone(), LOCKED)
          .mintable(admin_rule.clone(), LOCKED)
          .updateable_non_fungible_data(admin_rule.clone(), LOCKED)
          .no_initial_supply();

        let auth = AccessRules::new()
          .method("update_subscription_price", admin_rule.clone())
          .method("update_create_channel_price", admin_rule.clone())
          .method("update_amount_rewards_subscription", admin_rule.clone())
          .method("update_amount_rewards_creating_channel", admin_rule.clone())
          .method("resolve_proposal", admin_rule.clone())
          .method("update_platform_fee", admin_rule)
          .default(rule!(allow_all));

        let mut streamdao = Self {
          channels: HashMap::new(),
          mint_creator_address : creator_address,
          mint_membership_address: member_address,
          admin_badge_address: admin_badge.resource_address(),
          admin_badge: Vault::with_bucket(admin_badge),
          collect_xrd: Vault::new(RADIX_TOKEN),
          create_channel_price,
          subscription_price,
          amount_rewards_subscription,
          amount_rewards_creating_channel,
          platform_fee,
          donations_channel_received: HashMap::new(),
          daos: HashMap::new(),
        }.instantiate();

        streamdao.add_access_check(auth);

        streamdao.globalize()
      }

      // create a new user NFT that can be used to create a new channel or to subscribe to a channel
      pub fn new_membership(&self, name:String) -> Bucket {
        let member_resource_manager: &mut ResourceManager = borrow_resource_manager!(self.mint_membership_address);

        let membership = self.admin_badge.authorize(|| member_resource_manager.mint_non_fungible(
          &NonFungibleId::random(),
          Membership {
            name,
            channels: HashMap::new(),
            vote_power: HashMap::new(),
          }
        ));

        membership
      }

    /// This method is used to create a new channel.
    /// This method performs a few checks:
    /// * **Check 1:** Checks that the bucket passed contains enough amount to create a channel and pay the fee.
    /// * **Check 2:** Checks that if only one user is sent.
    /// # Arguments:
    /// * `payment` (Bucket) - The bucket that contains the XRD to pay to create a channel and pay the fee.
    /// * `membership` (Bucket) - The bucket represents the user who is creating the channel.
    /// * `name` (String) - channel name
    /// This method does not return:
    /// * `rewards_create_channel` (Bucket) - The bucket with rewards for creating a channel,
    /// * `creator` (Bucket) - The bucket represented creator,
    /// * `payment` - (Bucket) - Qhat's left of the payment,
    /// * `membership` (Bucket) - The bucket that was used to create the channel

      pub fn new_channel(
        &mut self,
        mut payment: Bucket,
        membership: Bucket,
        name: String,
      ) -> (Bucket, Bucket, Bucket, Bucket) {
        assert!(
          payment.amount() >= self.create_channel_price + self.platform_fee,
          "[NEW CHANNEL] Insufficient amount to create a channel."
        );

        assert!(
          membership.amount() == dec!("1"),
          "[NEW CHANNEL] Only one user is allowed."
        );

        self.collect_xrd.put(payment.take(self.create_channel_price));

        let rewards = ResourceBuilder::new_fungible()
          .metadata("name", name.to_uppercase())
          .metadata("description", "Rewards are given to members and allow you to submit and vote on channel DAO proposals.")
          .mintable(rule!(require(self.admin_badge_address)), LOCKED)
          .burnable(rule!(require(self.admin_badge_address)), LOCKED)
          .no_initial_supply();

        let creator = self.admin_badge.authorize(||{
          borrow_resource_manager!(self.mint_creator_address).mint_non_fungible(
            &NonFungibleId::random(),
            Creator {
              power_vote: dec!("1")
            }
          )
        });

        let channel = Channel {
          name,
          channel_id: Runtime::generate_uuid().to_string(),
          create_epoch: Runtime::current_epoch(),
          members: BTreeSet::new(),
          rewards_address: rewards,
          creator_id: creator.non_fungible_id(),
        };

        self.donations_channel_received.insert(
          creator.non_fungible_id(),
          Vault::new(RADIX_TOKEN)
        );

        let rewards_create_channel = self.admin_badge.authorize(|| {
          borrow_resource_manager!(rewards).mint(self.amount_rewards_creating_channel)
        });

        let dao = DaoComponent::new();

        self.daos.insert(channel.channel_id.clone(), dao.into());

        self.channels.insert(channel.channel_id.clone(), channel);

        (rewards_create_channel, creator, payment, membership)
      }

      /// This method is used to subscribe in channel.
      /// This method performs a few checks:
      /// * **Check 1:** Checks that if only one user is sent.
      /// * **Check 2:** Checks that the bucket passed contains enough amount to subscribe a channel and pay the fee.
      /// # Arguments:
      /// * `payment` (Bucket) - The bucket that contains the XRD to pay to create a channel and pay the fee.
      /// * `membership` (Bucket) - The bucket represents the user who is subscribe in the channel.
      /// * `channel_id` (String) - ID the channel
      /// This method does not return:
      /// * `rewards` (Bucket) - The bucket with rewards for subscribe a channel,
      /// * `payment` - (Bucket) - Qhat's left of the payment,
      /// * `membership` (Bucket) - The bucket that was used to create the channel

      pub fn subscribe(
        &mut self,
        mut payment:Bucket,
        membership:Bucket,
        channel_id:String,
      ) -> (Bucket, Bucket, Bucket){

        assert!(
          membership.amount() == dec!("1"),
          "[SUBSCRIBE] Only one user is allowed."
        );

        assert!(
          payment.amount() >= self.subscription_price + self.platform_fee,
          "[SUBSCRIBE] Insufficient amount to subscribe to the channel."
        );

        self.collect_xrd.put(payment.take(self.platform_fee));

        let channel = self.channels.get_mut(&channel_id).unwrap();

        let membership_id = membership.non_fungible_id();
        let mut membership_data = membership.non_fungible::<Membership>().data();

        // Parte do valor da incrição de um novo usuario é destinado ao donate do canal
        self.donations_channel_received.get_mut(
          &channel.creator_id).unwrap().put(payment.take(self.subscription_price - self.platform_fee));

        // recompensas por se escrever no canal
        let rewards_subscribe = self.admin_badge.authorize(|| {
          borrow_resource_manager!(channel.rewards_address).mint(self.amount_rewards_subscription)
        });

        let start_subscribe = Runtime::current_epoch();
        let end_subscribe = start_subscribe + END_SUBSCRIBE;
        membership_data.channels.insert(channel.channel_id.clone(), (start_subscribe, end_subscribe, start_subscribe));

        membership_data.vote_power.insert(channel.rewards_address, dec!("1"));

        // Inserir id do usuario no canal
        channel.members.insert(membership_id);

        self.admin_badge.authorize(|| membership.non_fungible().update_data(membership_data));

        (membership, payment, rewards_subscribe)
      }

      /// This method is used to resubscribe in channel.
      /// This method performs a few checks:
      /// * **Check 1:** Checks that if only one user is sent.
      /// * **Check 2:** Checks that the bucket passed contains enough amount to resubscribe a channel and pay the fee.
      //  * **Check 2:** Check if the user is already subscribed to the channel.
      /// # Arguments:
      /// * `payment` (Bucket) - The bucket that contains the XRD to pay to create a channel and pay the fee.
      /// * `membership` (Bucket) - The bucket represents the user who is resubscribe in the channel.
      /// * `channel_id` (String) - ID the channel
      /// This method does not return:
      /// * `rewards` (Bucket) - The bucket with rewards for resubscribe a channel,
      /// * `payment` - (Bucket) - Qhat's left of the payment,
      /// * `membership` (Bucket) - The bucket that was used to create the channel

      pub fn resubscribe(
        &mut self,
        mut payment:Bucket,
        membership:Bucket,
        channel_id:String,
      ) -> (Bucket, Bucket, Bucket){
        assert!(
          membership.amount() == dec!("1"),
          "[RESUBSCRIBE] Only one user is allowed."
        );

        assert!(
          payment.amount() >= self.subscription_price + self.platform_fee,
          "[RESUBSCRIBE] Insufficient amount to subscribe to the channel."
        );

        let channel = self.channels.get_mut(&channel_id).unwrap();

        let membership_id = membership.non_fungible_id();
        let mut membership_data = membership.non_fungible::<Membership>().data();

        let old_power_vote = membership_data.vote_power.get(&channel.rewards_address).unwrap().clone();
        let start_sub = membership_data.channels.get(&channel_id).unwrap().0;
        let old_end_sub = membership_data.channels.get(&channel_id).unwrap().1;
        let last_claim_rewards = membership_data.channels.get(&channel_id).unwrap().2;

        assert!(
          channel.members.contains(&membership_id),
          "[RESUBSCRIBE] This membership does not subscribe to this channel."
        );

        assert!(
          old_end_sub < Runtime::current_epoch(),
          "[RESUBSCRIBE] Subscription is already active."
        );

        self.collect_xrd.put(payment.take(self.platform_fee));

        // rewards for subscribing to the channel
        let rewards_subscribe = self.admin_badge.authorize(|| {
          borrow_resource_manager!(channel.rewards_address).mint(self.amount_rewards_subscription)
        });

        // with each resubscribe the user's voting power is updated
        let (new_power_vote, new_end_sub) = Streamdao::update_power_vote(
          old_power_vote,
          start_sub,
          old_end_sub,
        );

        *membership_data.vote_power.get_mut(&channel.rewards_address).unwrap() = new_power_vote;
        *membership_data.channels.get_mut(&channel_id).unwrap() = (start_sub, new_end_sub, last_claim_rewards);

         self.donations_channel_received.get_mut(&channel.creator_id).unwrap().put(payment.take(self.subscription_price - self.platform_fee));

        self.admin_badge.authorize(|| membership.non_fungible().update_data(membership_data));

        (membership, payment, rewards_subscribe)
      }

    /// This method is used to create a new proposal
    /// This method performs a few checks:
    /// * **Check 1:** Checks if the rewards used for are from the channel.
    /// * **Check 2:** Check if the user is subscribed to this channel.

      pub fn new_proposal(
        &mut self,
        mut channel_rewards:Bucket,
        author: Bucket,
        channel_id:String,
        choices: Vec<String>,
        ipfs_link: String,
        start: u64,
        end: u64,
      ) -> (Bucket, Bucket, Bucket){

        let channel = self.channels.get(&channel_id).unwrap();

        channel_rewards.create_proof().validate_proof(
          ProofValidationMode::ValidateResourceAddress(channel.rewards_address)
        ).expect("[NEW PROPOSAL] Reward is not valid for this channel.").drop();

        author.create_proof().validate_proof(
          ProofValidationMode::ValidateContainsNonFungibles(self.mint_membership_address, channel.members.clone())
        ).expect("[NEW PROPOSAL] Only subscribers can make proposals.").drop();

        channel_rewards.create_proof().validate_proof(
          ProofValidationMode::ValidateContainsAmount(channel.rewards_address, dec!("15"))
        ).expect("[NEW PROPOSAL] You need to have a minimum of 15 rewards in order to submit a proposal.").drop();

        self.admin_badge.authorize(||{
          borrow_resource_manager!(channel_rewards.resource_address()).burn(channel_rewards.take(dec!("15")));
        });

        let proposal = self.daos[&channel_id].create_proposal(choices, ipfs_link, author.non_fungible_id(), start, end);

        (channel_rewards, author, proposal)
      }

      pub fn submit_proposal(
        &self,
        proposal: Bucket,
        membership: Bucket,
        channel_id: String
      )  -> Bucket{

        let channel = self.channels.get(&channel_id).unwrap();

        membership.create_proof().validate_proof(
          ProofValidationMode::ValidateContainsNonFungibles(self.mint_membership_address, channel.members.clone())
        ).expect("[NEW PROPOSAL] Only subscribers can submit proposal.").drop();

        self.daos[&channel_id].deposit_proposal(proposal);

        membership
      }

      pub fn vote_proposal(
        &mut self,
        membership: Bucket,
        vote_proof: Proof,
        channel_id: String,
        proposal_id: NonFungibleId,
        mut choice: (String, Decimal)
      ) -> Bucket {

        let  channel = self.channels.get(&channel_id).unwrap();

        vote_proof.validate_proof(ProofValidationMode::ValidateContainsAmount(
          channel.rewards_address,
          choice.1
        )).expect("[VOTE PROPOSAL]: You do not have this amount of rewards");

        choice.1 *= membership.non_fungible::<Membership>().data().vote_power.get(&channel.rewards_address).unwrap().clone();

        self.daos[&channel_id].vote_proposal(proposal_id, membership.non_fungible_id(), choice);

        membership
      }

      pub fn claim_rewards(
        &mut self,
        channel_id:String,
        membership:Bucket,
      ) -> (Bucket, Bucket) {

        assert!(
          membership.amount() == dec!("1"),
          "[CLAIM REWARDS] Only one user is allowed."
        );

        let  channel = self.channels.get(&channel_id).unwrap();

        assert!(
          channel.members.contains(&membership.non_fungible_id()),
          "[CLAIM REWARDS] This membership does not subscribe to this channel."
        );

        let mut membership_data = membership.non_fungible::<Membership>().data();

        assert!(
          Runtime::current_epoch().abs_diff(membership_data.channels.get(&channel_id).unwrap().2) > CLAIM_REWARDS,
          "[CLAIM REWARDS] You can only claim rewards every 2 epochs."
        );

        let rewards:Bucket = self.mint_rewards(dec!("10"), channel.rewards_address);

        membership_data.channels.get_mut(&channel_id).unwrap().2 = Runtime::current_epoch();

        self.admin_badge.authorize(|| membership.non_fungible().update_data(membership_data));

        (membership, rewards)
      }

      pub fn resolve_proposal(
        &self,
        creator: Bucket,
        channel_id:String,
        proposal_id:NonFungibleId
      ) -> Bucket {

        let  channel = self.channels.get(&channel_id).unwrap();

        creator.create_proof().validate_proof(
            ProofValidationMode::ValidateResourceAddress(self.mint_creator_address)
        ).expect("[RESOLVE PROPOSAL] - Resource type not allowed.").drop();

       assert!(
        creator.non_fungible_id() == channel.creator_id,
        "[RESOLVE PROPOSAL] - Is not the creator of this channel."
       );

        self.admin_badge.authorize(|| self.daos[&channel_id].resolve_proposal(proposal_id));

        creator
      }

      fn mint_rewards(&self, amount: Decimal, resource: ResourceAddress) -> Bucket {
        let rewards_resource_manage: &mut ResourceManager = borrow_resource_manager!(resource);

        let rewards: Bucket = self.admin_badge.authorize(|| rewards_resource_manage.mint(amount));

        rewards
      }

      fn update_power_vote(old_pv:Decimal, start_subscribe:u64, end_subscribe:u64) -> (Decimal, u64){
        let new_end_subscribe = Runtime::current_epoch() + END_SUBSCRIBE;

        let old_sub:Decimal = (end_subscribe - start_subscribe).into();
        let new_sub:Decimal = (new_end_subscribe - end_subscribe).into();
        let mut_pv = old_pv + dec!("0.01");

        let teste = (old_sub * mut_pv + new_sub)/(old_sub+new_sub);
        let teste2 = old_pv - teste;

        let new_pv = old_pv + teste2.abs();

        (new_pv, new_end_subscribe)
      }

      pub fn update_subscription_price(&mut self, new_price:Decimal){
        Streamdao::assert_price(new_price);

        self.subscription_price = new_price;
      }

      pub fn update_create_channel_price(&mut self, new_price:Decimal){
        Streamdao::assert_price(new_price);

        self.create_channel_price = new_price;
      }

      pub fn update_amount_rewards_subscription(&mut self, new_reward_amount:Decimal){
        Streamdao::assert_price(new_reward_amount);

        self.amount_rewards_subscription = new_reward_amount;
      }

      pub fn update_amount_rewards_creating_channel(&mut self, new_reward_amount:Decimal){
        Streamdao::assert_price(new_reward_amount);

        self. amount_rewards_creating_channel = new_reward_amount;
      }

      pub fn update_platform_fee(&mut self, new_fee:Decimal){
        Streamdao::assert_fee(new_fee);

        self.platform_fee = new_fee;
      }

      pub fn subscription_price(&self) -> Decimal {
        self.subscription_price
      }

      pub fn create_channel_price(&self) -> Decimal {
        self.create_channel_price
      }

      pub fn amount_rewards_subscription(&self) -> Decimal {
        self.amount_rewards_subscription
      }

      pub fn amount_rewards_creating_channel(&self) -> Decimal {
        self.amount_rewards_creating_channel
      }

      pub fn platform_fee(&self) -> Decimal {
        self.platform_fee
      }

      fn assert_price(price: Decimal) {
        assert!(
            !price.is_negative(),
            "[PRICE] Price cannot be negative."
        );
      }

      fn assert_reward(reward: Decimal) {
        assert!(
            !reward.is_negative(),
            "[REWARD] Reward cannot be negative."
        );
      }

      fn assert_fee(fee: Decimal) {
        assert!(
            !fee.is_negative(),
            "[FEE] Fee cannot be negative."
        );
      }

      /////////////////////// [FOR TESTING] //////////////////////

      // pub fn new_membership_set_id(&self, membership_id: u64, name:String) -> Bucket {
      //   let member_resource_manager: &mut ResourceManager = borrow_resource_manager!(self.mint_membership_address);

      //   let membership = self.admin_badge.authorize(|| member_resource_manager.mint_non_fungible(
      //     &NonFungibleId::from_u64(membership_id),
      //     Membership {
      //       name,
      //       channels: HashMap::new(),
      //       vote_power: HashMap::new(),
      //     }
      //   ));

      //   membership
      // }

      // pub fn new_channel_set_id(
      //   &mut self,
      //   mut payment: Bucket,
      //   membership: Bucket,
      //   channel_id: String,
      //   name: String,
      // ) -> (Bucket, Bucket, Bucket, Bucket) {
      //   assert!(
      //     payment.amount() >= self.create_channel_price + self.platform_fee,
      //     "[NEW CHANNEL] Insufficient amount to create a channel."
      //   );

      //   assert!(
      //     membership.amount() == dec!("1"),
      //     "[NEW CHANNEL] Only one user is allowed."
      //   );

      //   self.collect_xrd.put(payment.take(self.create_channel_price));

      //   let rewards = ResourceBuilder::new_fungible()
      //     .metadata("name", name.to_uppercase())
      //     .metadata("description", "Rewards are given to members and allow you to submit and vote on channel DAO proposals.")
      //     .mintable(rule!(require(self.admin_badge_address)), LOCKED)
      //     .burnable(rule!(require(self.admin_badge_address)), LOCKED)
      //     .no_initial_supply();

      //   let creator = self.admin_badge.authorize(||{
      //     borrow_resource_manager!(self.mint_creator_address).mint_non_fungible(
      //       &NonFungibleId::random(),
      //       Creator {
      //         power_vote: dec!("1")
      //       }
      //     )
      //   });

      //   let channel = Channel {
      //     name,
      //     channel_id: channel_id,
      //     create_epoch: Runtime::current_epoch(),
      //     members: BTreeSet::new(),
      //     rewards_address: rewards,
      //     creator_id: creator.non_fungible_id(),
      //   };

      //   self.donations_channel_received.insert(
      //     creator.non_fungible_id(),
      //     Vault::new(RADIX_TOKEN)
      //   );

      //   let rewards_create_channel = self.admin_badge.authorize(|| {
      //     borrow_resource_manager!(rewards).mint(self.amount_rewards_creating_channel)
      //   });

      //   let dao = DaoComponent::new();

      //   self.daos.insert(channel.channel_id.clone(), dao.into());

      //   self.channels.insert(channel.channel_id.clone(), channel);

      //   (rewards_create_channel, creator, payment, membership)
      // }

      // pub fn new_proposal_set_id(
      //   &mut self,
      //   mut channel_rewards:Bucket,
      //   proposal_id: u64,
      //   author: Bucket,
      //   channel_id:String,
      //   choices: Vec<String>,
      //   ipfs_link: String,
      //   start: u64,
      //   end: u64,
      // ) -> (Bucket, Bucket, Bucket){

      //   let channel = self.channels.get(&channel_id).unwrap();

      //   channel_rewards.create_proof().validate_proof(
      //     ProofValidationMode::ValidateResourceAddress(channel.rewards_address)
      //   ).expect("[NEW PROPOSAL] Reward is not valid for this channel.").drop();

      //   author.create_proof().validate_proof(
      //     ProofValidationMode::ValidateContainsNonFungibles(self.mint_membership_address, channel.members.clone())
      //   ).expect("[NEW PROPOSAL] Only subscribers can make proposals.").drop();

      //   channel_rewards.create_proof().validate_proof(
      //     ProofValidationMode::ValidateContainsAmount(channel.rewards_address, dec!("15"))
      //   ).expect("[NEW PROPOSAL] You need to have a minimum of 15 rewards in order to submit a proposal.").drop();

      //   self.admin_badge.authorize(||{
      //     borrow_resource_manager!(channel_rewards.resource_address()).burn(channel_rewards.take(dec!("10")));
      //   });

      //   let proposal = self.daos[&channel_id].create_proposal_set_id(proposal_id, choices, ipfs_link, author.non_fungible_id(), start, end);

      //   (channel_rewards, author, proposal)
      // }
    }
}
