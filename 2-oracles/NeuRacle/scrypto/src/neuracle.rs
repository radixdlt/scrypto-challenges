//! [NeuRacle] is the core on-chain governance blueprint of NeuRacle.
//! NeuRacle's native projects or any third-party NeuRacle service user will have to fetch data through this blueprint.
//! 
//! Any token can become NeuRacle medium. However, become Neuracle medium require mint-burn authority of that token and will affect token-economic a lot.
//! Therefore, choosing inital payrate, reward, punishment rate will be a critical point.
//! 
//! Personally, I also don't recommend having these rate changable since that will come at the cost of community trust.
//! 
//! P/s: Because NeuRacle has almost the same Consensus model as Radix, Radix token can easily become NeuRacle medium by just utilize it's current decentralized consensus network.
//! This mean beside validating transaction, Radix validator can also participate on data feeding and create more utility for Radix token.

use scrypto::prelude::*;
use crate::neura_stable_coin::NStableCoin;
use crate::validator::Validator;
use crate::utilities::*;

#[derive(NonFungibleData)]
pub struct ValidatorData {
    pub name: String,
    pub location: String,
    pub website: String,
    #[scrypto(mutable)]
    /// Validator Component Address
    pub address: String

}

#[derive(NonFungibleData)]
pub struct UserData {
    #[scrypto(mutable)]
    /// Time limit of data fetching
    pub end: u64,
    /// Data source
    pub api: String,
}

blueprint! {

    struct NeuRacle {

        /// Store NeuRacle validated datas.
        datas: BTreeMap<String, String>,
        /// Store L2 stable coins project and it's name
        stable_coins: LazyMap<ComponentAddress, String>,
        /// Store NeuRacle Validator Addresses and their vote weight (staked amount).
        validators: Vec<(ComponentAddress, Decimal)>, 
        /// The maximum validator will be choosed to validate data and mint reward per data feeding round
        validator_cap: usize, 
        neura_vault: Vault,
        /// NeuRacle component controller badge, this can also be used to control other project in NeuRacle L2
        controller_badge: Vault, 
        neura: ResourceAddress, 
        validator_badge: ResourceAddress,
        user_badge: ResourceAddress,
        /// The rate which user has to pay per data feeding round. All will be burned.
        pay_rate: Decimal, 
        /// The fee user has to pay for ecosystem when using L2 stablecoin project of NeuRacle. All will be burned.
        fee_stablecoin: Decimal, 
        /// Unstaking will be delayed for an amount of time before withdrawal.
        unstake_delay: u64, 
        /// After having a good decentralization and security, Admin can advance stage to allow anyone can become validator. Initial stage is stage 1.
        stage: u8, 
        /// The frequent of data feeding. Current Scrypto version only allow using epoch time unit, so the lowest is 1 epoch, estimated 1 hour per data update.
        round_length: u64, 
        /// The inflation percent each round. If reward_rate is 0.0015% and 1 round = 1 epoch ~ 1 hour, that's about 13.14% APY.
        reward_rate: Decimal, 
        /// Untruthful validator behavior will be punished * times per reward rate. Eg: punishment = 5, reward rate = 0.0015 > punish 0.0075% per round.
        punishment: Decimal, 
        /// NeuRacle system time, caculated by current epoch / round length.
        system_time: u64, 
        /// Keep track of the round status.
        round_start: bool, 
        /// Keep track of the active validators per round.
        active_validators: HashMap<ComponentAddress, Decimal>, 
        /// The badge to mint new NeuRacle ecosystem controller badge when new L2 project created.
        mint_controller_badge: Vault

    }

    impl NeuRacle {

        pub fn new(
            medium_token: ResourceAddress, 
            admin_badge: ResourceAddress, 
            mint_controller_badge: Bucket, 
            controller_badge: ResourceAddress, 
            validator_cap: usize, 
            round_length: u64, 
            pay_rate: Decimal, 
            fee_stablecoin: Decimal, 
            unstake_delay: u64, 
            reward_rate: Decimal, 
            punishment: Decimal) -> ComponentAddress {

            let system_time = Runtime::current_epoch() / round_length;

            assert_fee(fee_stablecoin);

            let controller_badge_new = mint_controller_badge.authorize(|| {
                borrow_resource_manager!(controller_badge)
                .mint(dec!("1"))
            });

            let validator_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "NeuRacle Validator Badge")
                .mintable(rule!(require(controller_badge)), LOCKED)
                .burnable(rule!(require(controller_badge)), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge)), LOCKED)
                .no_initial_supply();
            
                info!("Validator badge address: {}", validator_badge);

            let user_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "NeuRacle User Badge")
                .mintable(rule!(require(controller_badge)), LOCKED)
                .burnable(rule!(require(controller_badge)), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge)), LOCKED)
                .no_initial_supply();

                info!("User badge: {}", user_badge);

            let rules = AccessRules::new()
                .method("create_new_validator_node", rule!(require(admin_badge)))
                .method("advance_stage", rule!(require(admin_badge)))
                .method("set_unstake_delay", rule!(require(admin_badge)))
                .method("new_stable_coin_project", rule!(require(admin_badge)))
                .method("set_round_length", rule!(require(admin_badge)))
                .method("new_api", rule!(require(controller_badge)))
                .default(rule!(allow_all));

            let component = Self {
                datas: BTreeMap::new(),
                stable_coins: LazyMap::new(),
                validators: Vec::new(),
                validator_cap: validator_cap,
                controller_badge: Vault::with_bucket(controller_badge_new),
                neura_vault: Vault::new(medium_token),
                neura: medium_token,
                validator_badge: validator_badge,
                user_badge: user_badge,
                pay_rate: pay_rate,
                fee_stablecoin: fee_stablecoin,
                unstake_delay: unstake_delay,
                stage: 1,
                round_length: round_length,
                reward_rate: reward_rate / dec!("100"),
                punishment: punishment,
                system_time: system_time,
                round_start: false,
                active_validators: HashMap::new(),
                mint_controller_badge: Vault::with_bucket(mint_controller_badge)
                }
                .instantiate()
                .add_access_check(rules)
                .globalize();
    
            info!(
                "Component: {}", component
            );

            return component
        }

        ///At first, to prevent Sybil attack, NeuRacle also need to use DPoS mechanism and choose only Validators that has the basic requirement of network traffic and security.
        pub fn create_new_validator_node(&mut self, name: String, location: String, website: String, fee: Decimal) -> (ComponentAddress, Bucket) {

            assert!(self.stage == 1);

            let validator_id: NonFungibleId = NonFungibleId::random();

            let mut badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.validator_badge)
                .mint_non_fungible(&validator_id, ValidatorData{
                    name: name.clone(), 
                    location: location,
                    website: website,
                    address: String::default()
                })
            });

            let validator_address = Validator::new(self.neura, badge.resource_address(), self.controller_badge.resource_address(), name, fee, self.unstake_delay);

            let mut data: ValidatorData = badge.non_fungible().data();

            data.address = validator_address.to_string();
            
            self.controller_badge
                .authorize(|| badge.non_fungible().update_data(data));

            self.validators.push((validator_address, Decimal::zero()));

            return (validator_address, badge)

        }

        ///After Xi'an, when the NeuRacle system is more decentralized, anyone can become validator.
        pub fn become_new_validator(&mut self, name: String, location: String, website: String, fee: Decimal) -> (ComponentAddress, Bucket) {

            assert!(self.stage == 2);

            let validator_id: NonFungibleId = NonFungibleId::random();
            
            let mut badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.validator_badge)
                .mint_non_fungible(&validator_id, ValidatorData{
                    name: name.clone(),
                    location: location,
                    website: website,
                    address: String::default()
                })
            });

            let validator_address = Validator::new(self.neura, self.controller_badge.resource_address(), badge.resource_address(), name, fee, self.unstake_delay);

            let mut data: ValidatorData = badge.non_fungible().data();

            data.address = validator_address.to_string();

            self.controller_badge
                .authorize(|| badge.non_fungible().update_data(data));

            self.validators.push((validator_address, Decimal::zero()));

            return (validator_address, badge)

        }

        /// Anyone can become NeuRacle user with NAR token, the data source must be an accessible api or validators won't get the data
        pub fn become_new_user(&mut self, mut payment: Bucket, api: String) -> (Bucket, Bucket) {

            let amount = payment.amount();

            assert_resource(payment.resource_address(), self.neura, amount, self.pay_rate);

            let user_id: NonFungibleId = NonFungibleId::random();

            let length = (amount/self.pay_rate).floor();

            self.controller_badge.authorize(|| {
                payment.take(length * self.pay_rate).burn();
            });

            let current = Runtime::current_epoch();

            let end = current + length.to_string().parse::<u64>().unwrap() * self.round_length;

            let badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.user_badge)
                .mint_non_fungible(&user_id, UserData{
                    end: end,
                    api: api.clone()
                })
            });
            
            info!("You can access this data from now until epoch {}", end);

            if !self.datas.contains_key(&api) {

                self.datas.insert(api.clone(), String::default());
            
            }

            return (badge, payment)
            
        }

        pub fn refund_account(&mut self, mut identity: Bucket, mut payment: Bucket) -> (Bucket, Bucket) {

            let amount = payment.amount();

            assert_resource(identity.resource_address(), self.user_badge, identity.amount(), dec!("1"));
            assert_resource(payment.resource_address(), self.neura, amount, dec!("0"));

            let length = (amount/self.pay_rate).floor();

            self.controller_badge.authorize(|| {
                payment.take(length * self.pay_rate).burn();
            });

            let current = Runtime::current_epoch();

            let end = current + length.to_string().parse::<u64>().unwrap() * self.round_length;

            let mut data: UserData = identity.non_fungible().data();

            data.end = end;

            self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));
            
            info!("You can access your data until epoch {}", end);

            return (identity, payment)

        }

        pub fn get_data(&self, identity: Bucket) -> (Bucket, String) {

            assert_resource(identity.resource_address(), self.user_badge, identity.amount(), dec!("1"));
            
            let data = identity.non_fungible::<UserData>().data();

            assert!(
                (Runtime::current_epoch() <= data.end) || (data.end == 0),
                "Run out of time, you cannot access this data for now, please refund your account."
            );

            let my_data = self.datas.get(&data.api).unwrap().clone();

            return (identity, my_data)
        }

        /// This method will check the staking weight of each validator, set their round start status (so they're able to update data), and reset their active status.
        /// On stage 1, this method will also eliminate all validators that aren't on top 100 staking weight.
        /// The method can only be called after 1 "round length" of last round end time.
        /// The person who call new round will be rewarded a payrate amount.
        pub fn new_round(&mut self) -> Bucket {
            
            assert!(
                self.round_start == false,
                "Previous round haven't ended yet!"
            );

            let current = Runtime::current_epoch();

            assert!(
                current/self.round_length >= self.system_time,
                "Not time to start a new round yet!"
            );

            self.controller_badge.authorize(|| {

                self.validators.iter_mut().for_each(|(validator_address, weight)| {

                    let validator: Validator = validator_address.clone().into();

                    *weight = validator.get_current_staked_value();
                
                    validator.round_start();

                });
            });

            info!("Start voting round number {} of NeuRacle", self.system_time);

            let reward = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.neura).mint(self.pay_rate)
            });

            info!("You are rewarded {} NAR for start a NeuRacle round", reward.amount());

            self.round_start = true;

            if self.stage == 1 {

                self.validators.sort_by_key(|a| a.1);
                self.validators.reverse();
                let try_get_cap = self.validators.get(0..self.validator_cap);

                match try_get_cap {
                    Some(x) => self.active_validators = x.iter().cloned().collect(),
                    None => self.active_validators = self.validators.iter().cloned().collect()
                }
            }

            else {
                
                self.active_validators = self.validators.iter().cloned().collect();

            }

            return reward
        }

        /// This method will check on the active status of validators and can only advance if >2/3 validator is active.
        /// After that, this method will get the datas with the most weight, check if it > 2/3 vote weight,
        /// feed that on NeuRacle, reward the validators provided same datas and punish those didn't.
        /// The person who end a round will be rewarded 2 times payrate (Assuming call this method will more costly on xrd fee than start a round)
        pub fn end_round(&mut self) -> Bucket {
            
            assert!(
                self.round_start == true,
                "New round hasn't started yet!"
            );
        
            let mut val: HashMap<ComponentAddress, Decimal> = HashMap::new();

            self.active_validators.iter().for_each(|(&address, &weight)| {
    
                let validator: Validator = address.into();
                    
                if validator.get_status() {
                    
                    val.insert(address, weight);
                    
                }
            });
            
            assert!(
                val.len()*3 > self.active_validators.len()*2,
                "Not enough validator active yet!"
            );

            self.active_validators = val;

            let mut all_datas: HashMap<BTreeMap<String, String>, Decimal> = HashMap::new();
            let mut total_weight = Decimal::zero();

            self.active_validators.iter().for_each(|(&address, &weight)| {

                let validator: Validator = address.into();
                let datas = validator.get_data();
                if all_datas.contains_key(&datas) {
                    if let Some(x) = all_datas.get_mut(&datas) {
                        *x += weight;
                    }
                }
                else {
                    all_datas.insert(datas.clone(), weight);
                }
                total_weight += weight
            });

            let result = all_datas.iter().max_by_key(|entry | entry.1).unwrap();
            let data = result.0.clone();
            let weight = result.1.clone();

            if weight*dec!("3") >= total_weight*dec!("2") {
                self.datas = data;
                self.controller_badge.authorize(|| {
                    self.active_validators.iter().for_each(|(&address, _weight)| {
    
                        let validator: Validator = address.into();
                        if validator.get_data() == self.datas.clone() {validator.mint(self.reward_rate)}
                            else {validator.burn(self.reward_rate * self.punishment)}
                    })  
                })
            }

            info!("End round {} of NeuRacle", self.system_time);

            let current = Runtime::current_epoch();

            self.system_time = current/self.round_length + 1;

            self.round_start = false;

            let reward = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.neura).mint(self.pay_rate*dec!("2"))
            });

            info!("You are rewarded {} NAR for ending a NeuRacle round", reward.amount());

            return reward
            
        }

        /// A method to create NeuRacle's native stablecoin project. 
        pub fn new_stable_coin_project(&mut self, pegged_to: String, api: String) -> ComponentAddress {

            if !self.datas.contains_key(&api) {
                
                self.datas.insert(api.clone(), String::default());
            
            };

            let neuracle: ComponentAddress = Runtime::actor().component_address().unwrap();

            let user_id: NonFungibleId = NonFungibleId::random();

            let data_badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.user_badge)
                .mint_non_fungible(&user_id, UserData{
                    end: 0,
                    api: api.clone()
                })
            });

            let controller_badge =self.mint_controller_badge.authorize(|| {
                borrow_resource_manager!(self.controller_badge.resource_address())
                .mint(dec!("1"))
            });

            let stable_coin_project_address = NStableCoin::new(self.neura, pegged_to.clone(), neuracle, controller_badge, data_badge, self.fee_stablecoin);

            self.stable_coins.insert(stable_coin_project_address, pegged_to + "NStable Coin");

            return stable_coin_project_address

        }

        pub fn show_validators(&self) {
            info!("Begin show data pools, format: (validator_id: staked_weight)|| {:?}", self.validators)
        }

        pub fn show_apis(&self) {
            info!("Begin show data apis, {:?}", self.datas.keys())
        }


        pub fn show_stable_coins(&self) {
            info!("Begin show stable coin projects, format: (project_address: project_name)|| {:?}", self.stable_coins)
        }

        pub fn get_apis(&self) -> Vec<String>{
            self.datas.keys().cloned().collect()
        }

        pub fn advance_stage(&mut self) {
            assert!(self.stage == 2);
            self.stage += 1
        }

        pub fn set_unstake_delay(&mut self, new_unstake_delay: u64) {
            self.unstake_delay = new_unstake_delay
        }

        pub fn set_round_length(&mut self, new_round_length:u64) {
            self.round_length = new_round_length
        }
    }
}
