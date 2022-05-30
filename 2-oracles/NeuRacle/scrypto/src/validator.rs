//! [Validator] is the blueprint to create new validator for NeuRacle ecosystem.
//! User can stake to, or unstake, withdraw from validators through this blueprint.

use scrypto::prelude::*;
use crate::utilities::*;

#[derive(NonFungibleData)]
pub struct Staker {
    /// Validator component address
    pub address: String,
    pub validator_name: String,
    #[scrypto(mutable)]
    /// Unstaking amount
    pub unstaking: Decimal,
    #[scrypto(mutable)]
    /// End time of current unstaking
    pub end: u64,
    #[scrypto(mutable)]
    /// Current available for withdrawal
    pub unstaked: Decimal
}

blueprint! {

    struct Validator {

        /// Store staker info with their staked amount
        staker: HashMap<NonFungibleId, Decimal>,
        name: String,
        /// Staker has to pay fee % for validator in each successful data validating round
        fee: Decimal,
        /// Store unstaking, unstaked amount
        unstake_vault: Vault,
        staked_vault: Vault,
        fee_vault: Vault,
        controller_badge: Vault,
        staker_badge: ResourceAddress,
        medium_token: ResourceAddress,
        /// Waiting time when start unstaking
        unstake_delay: u64,
        /// Store new datas on-chain
        datas: BTreeMap<String, String>,
        /// Keep track of round status
        round_start: bool,
        /// Keep track of validator status
        active: bool

    }

    impl Validator {
        
        pub fn new(medium_token: ResourceAddress, badge: ResourceAddress, neura_controller_badge: ResourceAddress, name: String, fee: Decimal, unstake_delay: u64) -> ComponentAddress {

            assert_fee(fee);

            let controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Validator Controller Badge")
                .initial_supply(dec!("1"));

            let staker_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "Validator staker Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("change_fee", rule!(require(badge)))
                .method("withdraw_fee", rule!(require(badge)))
                .method("update_data", rule!(require(badge)))
                .method("round_start", rule!(require(neura_controller_badge)))
                .method("get_datas", rule!(require(neura_controller_badge)))
                .method("mint", rule!(require(neura_controller_badge)))
                .method("burn", rule!(require(neura_controller_badge)))
                .default(rule!(allow_all));

            let component = Self {
                staker: HashMap::new(),
                name: name.clone(),
                fee: fee / dec!("100"),
                unstake_vault: Vault::new(medium_token),
                staked_vault: Vault::new(medium_token),
                fee_vault: Vault::new(medium_token),
                controller_badge: Vault::with_bucket(controller_badge),
                staker_badge: staker_badge,
                medium_token: medium_token,
                unstake_delay: unstake_delay,
                datas: BTreeMap::new(),
                round_start: false,
                active: false
                }
                .instantiate()
                .add_access_check(rules)
                .globalize();
    
            info!("{} Validator Address: {}", name.clone(), component);
            info!("{} Validator Staker Badge: {}", name, staker_badge);
            return component
            
        }

        pub fn stake(&mut self, bucket: Bucket) -> Bucket {

            assert_resource(bucket.resource_address(), self.medium_token, bucket.amount(), Decimal::zero());

            let user_id: NonFungibleId = NonFungibleId::random();

            let validator: String = Runtime::actor().component_address().unwrap().to_string();

            let badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.staker_badge)
                .mint_non_fungible(&user_id, Staker{
                    address: validator,
                    validator_name: self.name.clone(),
                    unstaking: Decimal::zero(),
                    end: 0,
                    unstaked: Decimal::zero()
                })
            });

            info!("You have staked {} NAR to {} validator", bucket.amount(), self.name);

            self.staker.insert(user_id, bucket.amount());

            self.staked_vault.put(bucket);

            return badge

        }
    
        pub fn add_stake(&mut self, bucket: Bucket, identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));
            assert_resource(bucket.resource_address(), self.medium_token, bucket.amount(), dec!("0"));

            let id = identity.non_fungible::<Staker>().id();

            if let Some(staked_amount) = self.staker.get_mut(&id) {
                *staked_amount += bucket.amount();
            }

            info!("You have added stake {} NAR to {} validator", bucket.amount(), self.name);

            self.staked_vault.put(bucket);
            
            return identity

        }

        pub fn show_my_stake_amount(&mut self, identity: Bucket) -> Bucket {
            
            let id = identity.non_fungible::<Staker>().id();

            info!("You have staked {} NAR to {} validator", self.staker[&id],self.name);
            
            return identity

        }

        pub fn get_current_staked_value(&self) -> Decimal {
            self.staked_vault.amount()
        }

        pub fn unstake(&mut self, amount: Decimal, mut identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));
            
            let mut data: Staker = identity.non_fungible().data();

            let id = identity.non_fungible::<Staker>().id();

            let current = Runtime::current_epoch();

            if current >= data.end {
 
                data.unstaked += data.unstaking;
                data.unstaking = Decimal::zero();
                data.end = 0;
            
            }

            assert!(
                data.unstaking == Decimal::zero(),
                "You must wait or stop unstaking before start unstake again"
            );

            assert!(
                amount <= self.staker[&id], 
                "Not enough amount for unstake."
            );

            if let Some(staked_amount) = self.staker.get_mut(&id) {
                *staked_amount -= amount;
            } 

            let end = current + self.unstake_delay;
            
            data.unstaking = amount;
            data.end = end;

            self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));

            info!("Unstaking {} NAR, estimated done in epoch {}", amount, end);

            self.unstake_vault.put(self.staked_vault.take(amount));

            return identity
            
        }

        pub fn show_unstake_record(&self, identity: Bucket) -> Bucket {
            let data: Staker = identity.non_fungible().data();
            info!("You are currently unstaking {}, estimated done in epoch {}", data.unstaking, data.end);
            return identity
        }


        pub fn stop_unstake(&mut self, mut identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));

            let mut data: Staker = identity.non_fungible().data();

            let id = identity.non_fungible::<Staker>().id();

            let current = Runtime::current_epoch();

            if current >= data.end {
 
                data.unstaked += data.unstaking;
                data.unstaking = Decimal::zero();
                data.end = 0;
                
            }

            assert!(
                data.unstaking != Decimal::zero(),
                "You currently don't have token unstaking"
            );

            if let Some(staked_amount) = self.staker.get_mut(&id) {
                *staked_amount += data.unstaking;
            }

            self.staked_vault.put(self.unstake_vault.take(data.unstaking));

            data.unstaking = Decimal::zero();
            data.end = 0;

            self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));

            info!("You have stop unstake all your current unstaking amount.");

            return identity
        }

        pub fn withdraw(&mut self, amount: Decimal, mut identity: Bucket) -> (Bucket, Bucket) {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));

            let mut data: Staker = identity.non_fungible().data();

            let current = Runtime::current_epoch();

            if current >= data.end {
 
                data.unstaked += data.unstaking;
                data.unstaking = Decimal::zero();
                data.end = 0;

            }

            assert!(
                amount <= data.unstaked,
                "Not enough unstaked amount for withdrawal"
            );

            data.unstaked -= amount;

            self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));

            info!("You have withdrawed {} NAR token.", amount);

            return (self.unstake_vault.take(amount), identity)
            
        }

        /// Validator can only update data on round start.
        pub fn update_data(&mut self, datas: BTreeMap<String, String>) {

            assert!(
                self.round_start == true,
                "The round haven't started, you can't update data yet"
            );
            
            self.datas = datas;
            self.active = true;
            self.round_start = false

        }

        pub fn round_start(&mut self) {
            self.active = false;
            self.round_start = true
        }

        pub fn get_status(&self) -> bool {
            self.active
        }

        pub fn get_data(&self) -> BTreeMap<String, String> {
            self.datas.clone()
        }

        /// Reward method for trustful validator
        pub fn mint(&mut self, rate: Decimal) {

            let amount = self.staked_vault.amount();

            let fee = rate * self.fee;

            let staker_rate = rate * (dec!("1") - self.fee);

            let reward = amount * rate;

            let mut bucket = borrow_resource_manager!(self.medium_token)
                .mint(reward);

            self.fee_vault.put(bucket.take(fee));

            for val in self.staker.values_mut() {
                *val *= dec!("1") + staker_rate
            };

            self.staked_vault.put(bucket);

            info!("Your node is rewarded with {}%, keep up the good work!", rate*dec!("100"))

        }

        /// Punished method for untruthful validator
        pub fn burn(&mut self, rate: Decimal) {

            let amount = self.staked_vault.amount();

            let punish = amount * rate;

            let bucket = self.staked_vault.take(punish);

            borrow_resource_manager!(self.medium_token).burn(bucket);
    
            for val in self.staker.values_mut() {
                *val *= dec!("1") - rate
            };

            info!("Your node is punished for {}%, don't slacking around!", rate*dec!("100"))

        }

        pub fn change_fee(&mut self, fee: Decimal) {
            assert_fee(fee);
            self.fee = fee
        }

        pub fn withdraw_fee(&mut self) -> Bucket {
            self.fee_vault.take_all()
        }
    }
}
