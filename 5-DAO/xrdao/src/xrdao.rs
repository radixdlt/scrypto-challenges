/// **This submission does not need to be judged due to being incomplete***
/// 
/// 
use scrypto::prelude::*;
use crate::xrdaousers::*;
use crate::xrdaoproposal::*;
use crate::radiswap::*;

blueprint! {
    struct Xrdao { 
        // resources address of admin badge given to instantiator of XRDao
        admin_badge: ResourceAddress,
        // minting authority badge held in component to allow component to mint/burn
        xrdao_access_badge: Vault, 
        // Resource address of XRDao token
        xrdao_address: ResourceAddress, 
        // Resource address of REP token
        rep_address: ResourceAddress, 
        // holds lp tokens from radiswap after purchasing protocol owned liquidity
        xrdao_radiswap_component: Option<ComponentAddress>,
        // vault for LP tokens owned by XRDao protocol
        lp_token_vault: Option<Vault>, 
        // Vault to store the XRD collected in exchange for XRDao token
        collected_xrd: Vault, 
        // Vault to store all XRD from collected fees
        xrd_fees_collected: Vault, 
        // Stores xrdaouser component address to allow access to change user SBT
        xrdaouser_address: ComponentAddress,
        // Fee associated with buying/minting XRDao [updatable by governance]
        buy_fee: Decimal, 
        // Fee associated with selling/burning XRDao [updatable by governance]
        sell_fee: Decimal,
        // Percentage of fees that go towards buying protocol owned liquidity [updatable by governance]
        percent_reward_to_lp: Decimal,
        // Stores acceptable altcoin investments for platform users [updatable by governance]
        investment_pools: HashMap<ResourceAddress, Vault>,
        // Stores NFT ID and Vault for each user for purposes of claiming rewards
        user_xrd_vaults: HashMap<NonFungibleId, Vault>,
        // Stores NFT ID and Vault for each user for storing reputation
        user_rep_vaults: HashMap<NonFungibleId, Vault>,
        // Stores last epoch rewards were distributed
        last_reward_epoch: u64,
    }

    impl Xrdao {
        pub fn instantiate_xrdao() -> ComponentAddress {
            
            // Create an admin badge that will be returned to the caller of instantiate_xrdao
            // This badge allows authority for calling admin restricted methods within this component
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "XRDao admin badge")
                .initial_supply(1);

            // Create a access badge that will be kept inside this component to be able to:
            // mint and burn Xrdao token, access to methods from xrdaouser & xrdaoproposal components
            let xrdao_access_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "XRDao Access Badge")
                .initial_supply(1);
            
            // Create xrdao token and store address
            // This token is never held inside the component
            let xrdao_address: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "XRDao")
                .metadata("symbol", "XRDAO")
                .mintable(rule!(require(xrdao_access_badge.resource_address())), LOCKED)
                .burnable(rule!(require(xrdao_access_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Create rep token and store address
            // This token is never held inside the component
            let rep_address: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "Reputation")
                .metadata("symbol", "REP")
                .mintable(rule!(require(xrdao_access_badge.resource_address())), LOCKED)
                .burnable(rule!(require(xrdao_access_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();

            let mut component = Self {
                // Admin badge resource address
                admin_badge: admin_badge.resource_address(), 
                // Vault to store access badge
                xrdao_access_badge: Vault::with_bucket(xrdao_access_badge), 
                // XRDao token address
                xrdao_address: xrdao_address.resource_address(), 
                // rep token address
                rep_address: rep_address.resource_address(), 
                // Stores LP component address from radiswap
                xrdao_radiswap_component: None,  
                // Stores LP tokens for protocol owned liquidity
                lp_token_vault: None, 
                // Collected in exchange for xrd
                collected_xrd: Vault::new(RADIX_TOKEN), 
                // Vault to store collected platform fees
                xrd_fees_collected: Vault::new(RADIX_TOKEN), 
                // Stores new xrdaouser component address
                xrdaouser_address: XrdaoUser::new(access_badge.resource_address()),
                // starts at .5% fee to buy
                buy_fee: dec(".005"), 
                // starts at .5% fee to sell
                sell_fee: dec(".005"),  
                // starts at 50% of fees collected buy protocol liquidity
                percent_reward_to_lp: dec(".5"), 
                // requires governance vote to add pools
                investment_pools: None, 
                // Maps users SBT ID to Vault to hold xrd rewards
                user_xrd_vaults: HashMap::new(), 
                // Maps users SBT ID to Vault to hold rep rewards
                user_rep_vaults: HashMap::new(), 
                // sets last reward to epoch 0 to start
                last_reward_epoch: 0,
                
            }
            
            component.instantiate();
            let access_rules = AccessRules::new()
                .method("set_xrdao_radiswap_component_address", rule!(require(admin_badge.resource_address())));
                .default(rule!(allow_all));
            component.add_access_check(access_rules);
            (component.globalize(), admin_badge)
        }
        
        // Sets the component address for xrdao to buy protocol owned liquidity
        // admin sets this address after instantiation and liquidity pool is started
        pub fn set_xrdao_radiswap_component_address(&mut self, radiswap: ComponentAddress) {
            self.xrdao_radiswap_component = radiswap;
        }

        // Method call to create a new user for XRDao protocol
        // Calls method by same name from xrdaousers to create the actual NFT
        pub fn new_user(&mut self, account_address: ComponentAddress, handle: String) -> Bucket {
            let xrdao_user: XrdaoUser = self.xrdaouser_address.into();
            let new_user_sbt: Bucket = xrdao_user.new_user(account_address, handle);
            // creates user reward vault
            let user_xrd_vault: Vault = Vault::new(RADIX_TOKEN);
            // creates user reward vault
            let user_rep_vault: Vault = Vault::new(rep_address);
            // finds sbt_id
            let user_id: NonFungibleId = new_user_sbt.non_fungible::<User>().id();
            // Stores sbt_id & xrd vault in xrdao component
            self.user_xrd_vaults.insert(user_id, user_xrd_vault);
            // Stores sbt_id & rep vault in xrdao component
            self.user_rep_vaults.insert(user_id, user_rep_vault);
            // Returns NFT to user
            new_user_sbt 
        }

        //takes xrd_deposit from user, deposits xrd into collected_xrd vault, mints xrdao and gives to user
        pub fn mint_xrdao(&mut self, xrd_deposit: Bucket, sbt_id: Proof) -> Bucket {
            
            // take fee in put tokens in fee vault
            let total_fee: Decimal = xrd_deposit.amount() * self.buy_fee; // Calculate fees
            let fee_collected: Bucket = xrd_deposit.take(total_fee);
            self.xrd_fees_collected.put(fee_collected); // puts the collected fee in vault***********************

            // Take remainder and send back to user
            
            // test how much is actually in here? has fee been successfully taken?
            info!("amount to mint is {:?}", xrd_deposit.amount()) // ***should be 99.5

            let xrdao_to_mint: Decimal = xrd_deposit.amount();
            self.collected_xrd.put(xrd_deposit); // puts xrd in vault *****************************************

            // Mint xrdao according to how much xrd was deposited
            let xrdao_resource_manager: &mut Resourcemanager = borrow_resource_manager!(self.xrdao_address);
            let xrdao = self.xrdao_access_badge
                .authorize(|| xrdao_resource_manager.mint(xrdao_to_mint));
            
            // Calling the xrdaouser componenet to have access to decrease xrdao balance on SBT, and use sbt_address
            let xrdao_user: XrdaoUser = self.xrdaouser_address.into();
            self.xrdao_access_badge.authorize(||
                xrdao_user.inc_xrdao_balance(&sbt_id, &xrdao_to_mint)); // Increase xrdao balance on user's SBT
            
            xrdao // Returns xrdao tokens to user
        }

        // takes xrdao from user, burns the xrdao, returns XRD to user minus sell fee
        pub fn burn_xrdao(&mut self, xrdao_deposit: Bucket, sbt_id: Proof) -> Bucket {
            
            // take fee in put tokens in fee vault
            let total_fee: Decimal = xrdao_deposit.amount() * self.buy_fee; // Calculate fees
            self.xrd_fees_collected.put(self.collected_xrd.take(total_fee)); // puts the collected fee in vault
            
            // Take remainder and send back to user
            let xrd_to_return: Decimal = xrdao_deposit.empty();
            let xrd = self.collected_xrd.take(xrd_to_return);
            
            // burn xrdao and return xrd according to how much xrdao was deposited
            let xrdao_resource_manager: &mut Resourcemanager = borrow_resource_manager!(self.xrdao_address);
            let xrdao_tokens = self.xrdao_access_badge
                .authorize(|| xrdao_resource_manager.burn(xrdao_deposit));
  
            // Calling the xrdaouser componenet to have access to decrease xrdao balance on SBT, and use sbt_address
            let xrdao_user: XrdaoUser = self.xrdaouser_address.into(); 
            // update xrdao user SBT with a decrease in xrdao balance
            self.xrdao_access_badge.authorize(||
                xrdao_user.dec_xrdao_balance(&sbt_id, &xrdao_deposit.amount())); 

            xrd // Returns xrd tokens to user
        }

        // Method call to authorize and increase user rep balance held in component rep vault
        // Calls method by same name from xrdaousers to update balance in SBT data
        pub fn mint_rep(&mut self, user_id: Proof, amount: Bucket) { 
            let sbt_id: NonFungibleId = user_id.non_fungible::<User>().id();
            let user_vault = self.user_rep_vaults.get(&user_id);
            user_vault.put(self.xrdao_access_badge.authorize(|| (amount.amount().mint())));
            let xrdao_user: XrdaoUser = self.xrdaouser_address.into();
            xrdao_user.inc_rep_balance(sbt_id, amount.amount());
        }
        
        // Method call to authorize and decrease user xrdao balance in component rep vault
        // Calls method by same name from xrdaousers to update balance in SBT data
        pub fn burn_rep(&mut self, user_id: Proof, amount: Bucket) {
            let sbt_id: NonFungibleId = user_id.non_fungible::<User>().id();
            let user_vault = self.user_rep_vaults.get(&user_id);
            self.xrdao_access_badge.authorize(|| (user_vault.take(amount)).burn());  
            let xrdao_user: XrdaoUser = self.xrdaouser_address.into();
            xrdao_user.dec_rep_balance(sbt_id, amount.amount());  
        }

        // takes entire xrd_fees_collected vault and distributes xrd to vxrd holders, vxrd loans
        // anyone can call this to distrubte rewards after 50 epochs
        pub fn distribute_rewards(&mut self) -> () {
            let xrdao_user: XrdaoUser = self.xrdaouser_address.into(); 
            xrdao_user.reputation_decay()
            let mut current_epoch = runtime::current_epoch();
            let mut next_payment = last_reward_epoch + 50;
            if current epoch >= next_payment {
                // Take all xrd to distrubte into a bucket
                let total_rewards: Bucket = self.xrd_fees_collected.empty();
                // Take out the amount of rewards dedicated to buying LP
                let rewards_for_lp: Bucket = total_rewards.take(total_rewards.amount() * percent_reward_to_lp);
                // buy LP with these funds
                buy_xrdao_lp(rewards_for_lp); 
                // empty all available rewards into a bucket
                let distribution_rewards: Bucket = total_rewards.empty();
                // Presidents = top 1% users by rep get 3% fees
                let president_rewards = .03 * distribution_rewards;
                // Vice Presidents = top 10% users by rep get 18% of fees
                let vice_president_rewards = .18 * distribution_rewards;
                // Senators = top 30% of users by rep get 30% of fees
                let senator_rewards = .30 * distribution_rewards;
                // Governers = top 50% of users by rep get 50% of fees
                let governor_rewards = .20 * distribution_rewards;
                // Citizen = top 90% of users by rep get 50% of fees
                let citizen_rewards = .29 * distribution_rewards;
                // Plebs dont earn fees (bottom 10%)

                let xrdao_user: XrdaoUser = self.xrdaouser_address.into(); 
                let (total_users, vec_rep_leaderboard) = self.XrdaoUser.sort_rep_leaderboard();
                let ( )

                // need to divide each ranks fees up by number of people contained in each rank
                // how do I count the number of ranks easily?





            } else {
                println!("Next rewards available at epoch {}", next_payment)
            }

            // let president_users = round_down((total_users +1)*.01);
            // let vice_president_users = round_down((total_users +1)*.10) - president_users;
            // let senator_users = round_down((total_users +1)*.20) - vice_president_users;
            // let governor_users = senatorround_down((total_users +1)*.50) - senator_users;
            // let citizen_users = round_down((total_users +1)*.90) - governer_users;

            // let presidents = &vec_rep_leaderboard[0..president_users]
            // let vice_presidents = &vec_rep_leaderboard[president_users + 1..vice_president_users]
            // let senators = &vec_rep_leaderboard[vice_president_users + 1..senator_users]
            // let governors = &vec_rep_leaderboard[senator_users + 1..governor_users]
            // let citizens = &vec_rep_leaderboard[governor_users + 1..citizen_users]
            
            //find account addresses linked to those SBTs and send 3% of rewards/president users
            let vicepresident_users = &vec_rep_leaderboard[president_users..vicepresident_users];
            
            for (sbt_id, sbt_data) in self.user_record {
                let rank = 

                match rank in sbt_data {
                    Rank::President => 
                    Rank::VicePresident =>
                    Rank::Senator =>
                    Rank::Governor =>
                    Rank::Citizen =>
                    Rank::Pleb =>
                }
            }
            if self.facilitator.is_some()
                && self.facilitator_fee > Decimal::zero()
                && !self.facilitator_rewards.contains_key(&loan_token)
            {
                self.facilitator_rewards.insert(loan_token,
                                                Vault::new(loan_token));
            }
        }
        
        /// Allows a user to request to collect all xrd rewards earned from the platform that
        /// is stored in each user's individual vault inside the xrdao component
        pub fn collect_individual_rewards(&mut self, user_id: Proof) -> (Bucket) {
            // assert user_id proof has an nft resource address that matches sbt address
            // assert reward balance is > 0
            let sbt_id: NonFungibleId = user_id.non_fungible::<User>().id();
            let user_vault = self.user_rep_vaults.get(&user_id);
            let rewards = user_vault.empty();
            // returns all collected xrd rewards to user
            rewards
        }    

        // Buy xrdao-xrd liquidity and deposit LP tokens to vault in this component

        // make sure no dangeling resources or leftovers
        fn buy_xrdao_lp(&mut self, xrd_tokens: Bucket) -> () {
            let radiswap: Radiswap = self.xrdao_radiswap_component.into(); 
            let half = xrd_tokens.take(xrd_tokens.amount()/dec!("2"));
            let (xrdao_tokens, swap_remainder) = radiswap.swap(half);
            self.collected_xrd_fees.put(swap_remainder); // put any remainder back in vault
            //verify what resource remainder is to put in correct vault ******************************
            let (lp_tokens, lp_remainder) = radiswap.add_liquidity(xrd_tokens.empty(), xrdao_tokens);
            self.lp_token_vault.put(lp_tokens);
            self.collected_xrd_fees.put(lp_remainder); // put any remainder back in vault
        }





    }
}

