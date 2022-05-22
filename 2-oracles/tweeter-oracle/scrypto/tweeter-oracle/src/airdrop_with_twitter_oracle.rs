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
        // ResourceAddress of admin_badge : this admin_badge is return to user who instanciate the component (Airdrop maker)
        admin_badge: ResourceAddress,
        // The vault that contains the tokens to distribute to the participants of the airdrop who have executed all the tasks
        tokens: Vault,
        // Badge resource address returned to people registered in the airdrop : This will allow them to make the withdrawal
        participant_badge_address: ResourceAddress,
        // minter of participant_badge vault
        minter_badge_vault: Vault,
        //Store the airdrop particpant : Tweeter Account by NonFungibleId : [{"12345678901234567890123456789012u128","cyover"},{"12345678901234567890123456788080u128","cyrolsi"}] 
        airdrop_participants : HashMap<NonFungibleId, String>,
        //Store participants tweeter_account to avoid multiple participation with the same tweeter account
        participants_tweeter_account : HashSet<String>,
        // Store tweeters accounts to follow
        accounts_to_follow :Vec<String>, 
        // Store tweets  to retweet 
        tweets_to_retweet :Vec<String> , 
        // Store tweets  to like 
        tweets_to_like : Vec<String>,
        // The oracle tweeter component which makes it possible to verify that all the tasks have been correctly executed
        tweeter_oracle : TweeterOracle,
        // Store the NonFongibleId of participants  who have completed all the tasks and who will receive the airdrop
        recipients : HashSet<NonFungibleId>,
        // Amount per recipient
        amount_per_recipient : Decimal 
    }

    impl AirdropWithTweeterOracle {
        // This function instanciate the AirdropWithTweeterOracle 
        // #Argumets 
        // * `token_type` Tokens resourceAddress to distribute
        // * `accounts_to_follow` tweeters accounts to follow 
        // * `tweets_to_retweet`  tweets to retweet
        // * `tweets_to_like`     tweets to like
        // * `tweeter_oracle_component_address` Address of TweeterOracle component
         pub fn new( token_type: ResourceAddress,
                    accounts_to_follow :Vec<String>, 
                    tweets_to_retweet :Vec<String> , 
                    tweets_to_like : Vec<String>,
                    tweeter_oracle_component_address : ComponentAddress ) -> (ComponentAddress, Bucket) {

            // Check that there is at least one task to do
            assert!(accounts_to_follow.len() > 0 
                 || tweets_to_retweet.len() > 0
                || tweets_to_like.len() > 0 , "you must give at leat 1 task for the airdrop"); 

            // create admin_badge bucket with one supply
            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(Decimal::one());

            // create a minter badge  
            let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "minter badge")
                .initial_supply(Decimal::one());

            // Create a participant badge address 
            let participant_badge_address = ResourceBuilder::new_non_fungible()
                .metadata("name", "participant badge")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minter_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // //Definition of the methods which will be accessible only to the administrator of the component 
            let access_rules = AccessRules::new()
                .method("finalize_airdrop", rule!(require(admin_badge.resource_address())))
                .method("find_and_store_airdrop_recipients", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            let tweeter_oracle : TweeterOracle =  tweeter_oracle_component_address.into();
            tweeter_oracle.add_followers_to_update(accounts_to_follow.clone()); 
            tweeter_oracle.add_likers_to_update(tweets_to_like.clone());
            tweeter_oracle.add_retweeters_to_update(tweets_to_retweet.clone());
            

            //  Instantiate AirdropWithTweeterOracle component and return it with the admin badge to caller
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
                tweeter_oracle : tweeter_oracle,
                recipients : HashSet::new(),
                amount_per_recipient : Decimal::zero()
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            return (component, admin_badge);
        }

        //This method allows you to register for an airdrop
        // #Arguments 
        // `tweeter_account_name` : tweeter account name
        pub fn register(&mut self, tweeter_account_name: String) -> Bucket {
            
            //Avoid multiple participation with the same tweeter account
            assert!(!self.participants_tweeter_account.contains(&tweeter_account_name),"already registered to this airdrop");
            
            // Check if the airdrop were already finalize
             assert!(
                self.amount_per_recipient == Decimal::zero(), 
               "The airdrop were already finalize"
               );
            // Generate NonFungibleId for participant
            let id = NonFungibleId::random();

            // Store tweeter account name by NonFungibleId 
            self.airdrop_participants.insert(id.clone(), tweeter_account_name.to_string()); 
            // Store tweeter account name
            self.participants_tweeter_account.insert(tweeter_account_name);

            // create participant badge that will allow him to make the withdrawal
            let participant_badge = self.minter_badge_vault.authorize(|| {
                borrow_resource_manager!(self.participant_badge_address).mint_non_fungible(
                    &id,
                    AirdropWithTweeterOracleData {
                        token_type: self.tokens.resource_address(),
                        is_collected: false,
                    },
                )
            });
            
            // return the participant_badge to caller
            return participant_badge;
        }

        //This find the participants that have completed the tasks and to store them
        pub fn find_and_store_airdrop_recipients(&mut self) -> usize
        {
            //find partcipants who made all tasks
            for nft_id in self.airdrop_participants.keys() {
                // check if current participant have executed all tasks
                let tweeter_account = self.airdrop_participants.get(&nft_id).unwrap().clone();
                if  !self.recipients.contains(&nft_id) && self.has_completed_all_tasks(tweeter_account)  {
                    // store the recipient Nft_id for widhraw
                    self.recipients.insert(nft_id.clone());
                }
            }
            // return the number of recipients
            return self.recipients.len();
        }

        // this method makes it possible to finalize the airdrop 
        // #Arguments
        // * `tokens` Bucket containing the tokens to distribute 
        pub fn finalize_airdrop(&mut self, 
                                     mut tokens: Bucket)  -> Bucket  {
            
            // check tokens quantity                            
            assert!(
                     tokens.amount() > Decimal::zero(),
                     "tokens quantity cannot be 0"
                    );
        
            // check token address
            assert_eq!(
                        tokens.resource_address(),
                        self.tokens.resource_address(),
                        "token address must match"
            );

            // check recipients 
            assert!(
                self.recipients.len() > 0 , 
                "there is no recipient for the airdrop"
              );

            // Check if the airdrop were already finalize
            assert!(
                    self.amount_per_recipient == Decimal::zero(), 
                   "The airdrop were already finalize"
                   );

            // check tokens quantity for NonFungible
            assert!(
                borrow_resource_manager!(tokens.resource_address()).resource_type()
                        == ResourceType::NonFungible && tokens.amount() >= Decimal::from(self.recipients.len() as i128 ) ,
                "For non-fungible tokens, a number at least equal to the number of recipients is required"
            );

            // Calculate the amount of tokens each recipient can receive
            let mut amount_per_recipient = tokens.amount() / Decimal::from(self.recipients.len() as i128);

            // Special case for NonFongible Token 
            if borrow_resource_manager!(tokens.resource_address()).resource_type() == ResourceType::NonFungible 
            {
                amount_per_recipient = Decimal::from(amount_per_recipient.round(18, RoundingMode::TowardsZero));
            }
            
            self.amount_per_recipient = amount_per_recipient; 

            // Take necessary amount from tokens bucket 
            self.tokens.put(tokens.take(amount_per_recipient * Decimal::from(self.recipients.len() as i128)));

            // return tokens  bucket to caller
            return tokens;
        }
        
        //This method allows recipients to withdraw their tokens 
        //#Arguments 
        //* `auth` Aidrop registration proof
        //#Return 
        // This function return a  bucket containing the quantity of tokens to be distributed 
        pub fn withdraw(&mut self, auth: Proof) -> Bucket {
           
            //checking if airdrop is filnalize
            assert!(self.amount_per_recipient > Decimal::zero() , "impossible withdraw : the airdrop is in progress"); 
            // checking participant badge
            assert_eq!(auth.resource_address(), self.participant_badge_address, "Invalid Badge Provided");
            // checking badge amount
            assert_eq!(auth.amount(), dec!("1"), "Invalid Badge Provided");
            let nft_id = auth.non_fungible::<AirdropWithTweeterOracleData>().id(); 
            // checking if current user completed all tasks
            assert!(self.recipients.contains(&nft_id), "you cannot receive the airdrop");
            let mut nft_data = auth.non_fungible::<AirdropWithTweeterOracleData>().data();
            // checking if withdrawal is already done
            assert!(!nft_data.is_collected, "withdraw already done");
            nft_data.is_collected = true;
            let amount = self.amount_per_recipient;
            // update nft data 
            self.minter_badge_vault.authorize({|| {
                auth.non_fungible().update_data(nft_data);
                }
            });
            info!("withdraw_token : {}", amount);
            // return tokens to caller
            return self.tokens.take(amount);
        }

        fn has_completed_all_tasks(&self, participant_tweeter_account : String) -> bool {

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