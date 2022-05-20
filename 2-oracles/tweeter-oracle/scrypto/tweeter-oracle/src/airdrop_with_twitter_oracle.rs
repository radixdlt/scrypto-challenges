use scrypto::prelude::*;
use crate::tweeter_oracle::*; 

#[derive(NonFungibleData)]
pub struct AirdropWithTweeterOracleData {
    token_type: ResourceAddress,
    #[scrypto(mutable)]
    is_collected: bool,
}

blueprint! {

    //This component allows airdrop automation. A certain number of tasks are defined by the creators of the airdrop component
    //Follow 1 and/or more accounts, like a tweet and/or more tweets and/retweet one or more tweets
    //Users register for the airdrop via the Register method by specifying their tweeter account and receive in return a non-fungible token to claim the amount of the airdrop when possible
    //At the stage of finalizing the airdrop methode finalize_airdrop the Tweeter_oracle component is used to verify that all tasks have been carried out by subscribers.
    //
    struct AirdropWithTweeterOracle {
        admin_badge: ResourceAddress,
        tokens: Vault,
        participant_badge_address: ResourceAddress,
        minter_badge_vault: Vault,
        airdrop_participants : HashMap<NonFungibleId, String>,
        participants_tweeter_account : HashSet<String>,
        accounts_to_follow :Vec<String>, 
        tweets_to_retweet :Vec<String> , 
        tweets_to_like : Vec<String>,
        tweeter_oracle : TweeterOracle,
        recipients : HashSet<NonFungibleId>,
        amount_per_recipient : Decimal 
    }

    impl AirdropWithTweeterOracle {
        pub fn new( token_type: ResourceAddress,
                    accounts_to_follow :Vec<String>, 
                    tweets_to_retweet :Vec<String> , 
                    tweets_to_like : Vec<String>,
                    tweeter_oracle_component_address : ComponentAddress ) -> (ComponentAddress, Bucket) {

            assert!(accounts_to_follow.len() > 0 
                 || tweets_to_retweet.len() > 0
                || tweets_to_like.len() > 0 , "you must give at leat 1 condition for the airdrop"); 

            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(Decimal::one());

            let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "minter badge")
                .initial_supply(Decimal::one());

            let participant_badge_address = ResourceBuilder::new_non_fungible()
                .metadata("name", "participant badge")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minter_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let access_rules = AccessRules::new()
                .method("finalize_airdrop", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            let component = Self {
                admin_badge: admin_badge.resource_address(),
                tokens: Vault::new(token_type),
                participant_badge_address,
                minter_badge_vault: Vault::with_bucket(minter_badge),
                airdrop_participants : HashMap::new(),
                participants_tweeter_account : HashSet::new(),
                accounts_to_follow : accounts_to_follow,
                tweets_to_retweet : tweets_to_retweet,
                tweets_to_like : tweets_to_like,
                tweeter_oracle : tweeter_oracle_component_address.into(),
                recipients : HashSet::new(),
                amount_per_recipient : Decimal::zero()
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            return (component, admin_badge);
        }

        pub fn register(&mut self, tweeter_account_name: String, participant: ComponentAddress) {
            
            assert!(!self.participants_tweeter_account.contains(&tweeter_account_name),"already registered to this airdrop");
            let id = NonFungibleId::random();

            self.airdrop_participants.insert(id.clone(), tweeter_account_name.to_string()); 
            self.participants_tweeter_account.insert(tweeter_account_name);

            let participant_badge = self.minter_badge_vault.authorize(|| {
                borrow_resource_manager!(self.participant_badge_address).mint_non_fungible(
                    &id,
                    AirdropWithTweeterOracleData {
                        token_type: self.tokens.resource_address(),
                        is_collected: false,
                    },
                )
            });

            borrow_component!(participant).call::<()>("deposit", args![participant_badge]);
        }

        pub fn finalize_airdrop(&mut self, 
                                     mut tokens: Bucket)  {
            
            assert!(
                     tokens.amount() > Decimal::zero(),
                     "tokens quantity cannot be 0"
                    );
            
            assert_eq!(
                        tokens.resource_address(),
                        self.tokens.resource_address(),
                        "token address must match"
            );

            assert!(
                    self.amount_per_recipient == Decimal::zero(), 
                   "The airdrop is already finalize"
                   );

            //find partcipants who made all taks
            for nft_id in self.airdrop_participants.keys() {
                // check eligibility
                let tweeter_account = self.airdrop_participants.get(&nft_id).unwrap().clone();
                if self.can_receive_airdrop(tweeter_account) {
                    self.recipients.insert(nft_id.clone());
                }
            }

            assert!(
                     self.recipients.len() > 0 , 
                     "there is no recipient for the airdrop"
                   );

            // Calculate the amount of tokens each recipient can receive
            let amount_per_recipient = tokens.amount() / Decimal::from(self.recipients.len() as i128);
            self.amount_per_recipient = amount_per_recipient; 

            self.tokens.put(tokens);
        }
        
        pub fn withdraw(&mut self, auth: Proof) -> Bucket {
           
            assert!(self.amount_per_recipient > Decimal::zero() , "impossible withdraw : the airdrop is in progress"); 
            assert_eq!(auth.resource_address(), self.participant_badge_address, "Invalid Badge Provided");
            assert_eq!(auth.amount(), dec!("1"), "Invalid Badge Provided");
            let nft_id = auth.non_fungible::<AirdropWithTweeterOracleData>().id(); 
            assert!(self.recipients.contains(&nft_id), "you cannot receive the airdrop");

            let mut nft_data = auth.non_fungible::<AirdropWithTweeterOracleData>().data();
            assert!(!nft_data.is_collected, "withdraw already done");
            nft_data.is_collected = true;
            let amount = self.amount_per_recipient;
            self.minter_badge_vault.authorize({|| {
                auth.non_fungible().update_data(nft_data);
                }
            });
            info!("withdraw_token : {}", amount);
            return self.tokens.take(amount);
        }

        fn can_receive_airdrop(&self, participant_tweeter_account : String) -> bool {

             let is_follower = self.accounts_to_follow.len() == 0 || self.accounts_to_follow.clone().into_iter()
                    .all(|x| self.tweeter_oracle.is_account_follower(x, participant_tweeter_account.to_string()));

             let is_liker = self.tweets_to_like.len() == 0 || self.tweets_to_like.clone().into_iter()
                    .all(|x| self.tweeter_oracle.is_tweet_liker(x,participant_tweeter_account.to_string()));

             let is_retweeter = self.tweets_to_retweet.len() == 0 || self.tweets_to_retweet.clone().into_iter()
                   .all(|x| self.tweeter_oracle.is_tweet_retweeter(x ,participant_tweeter_account.to_string()));

             return is_follower && is_liker && is_retweeter; 
        }
    }
}