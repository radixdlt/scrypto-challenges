use crate::utils::*;
use scrypto::prelude::*;

blueprint! {

    // This oracle stores data from the twitter API. Instantiating this component makes it possible to administer this data and make it available to those who need it within the data ledger
    // For example is the radixdlt user account followed by the cyover user account? Or has a tweet been liked by cyover user account ? 
    // This data can be useful for automating airdrops. An example of component automating the airdrop was created to test this Oracle (AirdropWithTweeterOracle)
    struct TweeterOracle {
        // Defines the administrator badge which gives the right to administer the data by calling the methods provided for this purpose 
        admin_badge: ResourceAddress,
        //This field is used to store the followers of an account. for example: 
        //[{"radixdlt",["cyover","toto","titi"]},{"cyover",["toto"]}]  
        tweeter_account_followers: HashMap<String, HashSet<String>>,
        //This field is used to store the likers of a tweet. for example: 
        //[{"tweet-1",["cyover","toto","titi"]},{"tweet2",["toto"]}]
        tweets_likers: HashMap<String, HashSet<String>>,
        //This field is used to store the likers of a tweet. for example: 
        //[{"tweet-1",["cyover","toto","titi"]},{"tweet2",["toto"]}]
        tweets_retweeters: HashMap<String, HashSet<String>>,
    }

    impl TweeterOracle {
        // Implement the functions and methods which will manage those resources and data

        // This function isntanciate a TweeterOracle Component 
        pub fn instantiate_tweeter_oracle() -> (ComponentAddress, Bucket) {
            // created the admin badge 
            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);

            //Definition of the methods which will be accessible only to the administrator of the component 
            let access_check = AccessRules::new()
                .method(
                    "insert_account_followers",
                    rule!(require(admin_badge.resource_address())),
                )
                .method(
                    "insert_tweets_likers",
                    rule!(require(admin_badge.resource_address())),
                )
                .method(
                    "insert_tweets_retweeters",
                    rule!(require(admin_badge.resource_address())),
                )
                .method(
                    "remove_account_followers",
                    rule!(require(admin_badge.resource_address())),
                )
                .method(
                    "remove_tweets_likers",
                    rule!(require(admin_badge.resource_address())),
                )
                .method(
                    "remove_tweets_retweeters",
                    rule!(require(admin_badge.resource_address())),
                )
                .default(rule!(allow_all));

            // Instantiate TweeterOracle component and return it with the admin badge to caller
            let component = Self {
                admin_badge: admin_badge.resource_address(),
                tweeter_account_followers: HashMap::new(),
                tweets_likers: HashMap::new(),
                tweets_retweeters: HashMap::new(),
            }
            .instantiate()
            .add_access_check(access_check)
            .globalize();

            return (component, admin_badge);
        }

        // this method Allow to insert new followers of a user account 
        // # Arguments :  
        // * `twitter_account_user_name` String - A tweeter user account for which we want to store the followers  
        // * `new_followers` String -  A tweeter user account followers
        pub fn insert_account_followers(
            &mut self,
            tweeter_account_user_name: String,
            new_followers: HashSet<String>,
        ) {

            // checking the arguments
            assert!(!tweeter_account_user_name.is_empty(), "tweeter account user name can not be empty");
            assert!(new_followers.len() > 0 ,"followers hashset can not be empty");

            // store followers
            insert_items(
                tweeter_account_user_name,
                &mut self.tweeter_account_followers,
                new_followers,
            );
        }

        // this method Allow to remove followers of a user account 
        // # Arguments :  
        // * `twitter_account_user_name` String - A tweeter user account for which we want to delete the followers  
        // * `delete_followers` String -  A tweeter user account followers to delete
        pub fn remove_account_followers(
            &mut self,
            twitter_account_user_name: String,
            delete_followers: HashSet<String>,
        ) {

            // checking the arguments
            assert!(!twitter_account_user_name.is_empty(), "tweeter account user name can not be empty");
            assert!(delete_followers.len() > 0 ,"delete_followers hashset can not be empty");
            
            // remove folowers 
            remove_items(
                twitter_account_user_name,
                &mut self.tweeter_account_followers,
                delete_followers,
            );
        }

        // this method Allow  to check if an tweeter account is follow by another tweeter account
        // # Arguments :  
        // * `twitter_account_user_name` String - A tweeter user account to follow
        // * `follower_user_name` String -  A follower tweeter account 
        pub fn is_account_follower(
            &mut self,
            twitter_account_user_name: String,
            follower_user_name: String,
        ) -> bool {


            // checking the arguments
            assert!(!twitter_account_user_name.is_empty(), "tweeter account user name can not be empty");
            assert!(!follower_user_name.is_empty(), "follower user name can not be empty");
            

            return is_item_exist(
                twitter_account_user_name,
                &mut self.tweeter_account_followers,
                follower_user_name,
            );
        }

        // this method Allow to insert a tweet likers 
        // # Arguments :  
        // * `tweet_id` String - A tweeter user account for which we want to store the likers  
        // * `new_likers` String -  A tweeter user account likers
        pub fn insert_tweets_likers(&mut self, tweet_id: String, new_likers: HashSet<String>) {

            // checking the arguments
            assert!(!tweet_id.is_empty(), "tweet_id can not be empty");
            assert!(new_likers.len() > 0 ,"new_likers hashset can not be empty");

            // store likers
            insert_items(tweet_id, &mut self.tweets_likers, new_likers);
        }

        // this method Allow to insert a tweet likers 
        // # Arguments :  
        // * `tweet_id` String - A tweeter user account for which we want to remove the likers  
        // * `remove_likers` String -  A tweeter user account likers
        pub fn remove_tweets_likers(&mut self, tweet_id: String, remove_likers: HashSet<String>) {
            
            // Checking atguments 
            assert!(!tweet_id.is_empty(), "tweet_id can not be empty");
            assert!(remove_likers.len() > 0 ,"remove_likers hashset can not be empty");

            // remove likers
            remove_items(tweet_id, &mut self.tweets_likers, remove_likers);
        }

        // this method Allow to check if an tweet is like by an tweeter account
        // # Arguments :  
        // * `tweet_id` String - A tweet we want to like
        // * `liker_user_name` String -  A liker tweeter account 
        pub fn is_tweet_liker(&mut self, tweet_id: String, liker_user_name: String) -> bool {
            
            //Checking parameter 
            assert!(!tweet_id.is_empty(), "tweet_id can not be empty");
            assert!(!liker_user_name.is_empty(), "liker_user_name can not be empty");


            return is_item_exist(tweet_id, &mut self.tweets_likers, liker_user_name);
        }

        // this method Allow to insert a tweet retweeters 
        // # Arguments :  
        // * `tweet_id` String - A tweet we want to store the retweeters  
        // * `new_retweeters` String -  A tweeter user account retweeters
        pub fn insert_tweets_retweeters(
            &mut self,
            tweet_id: String,
            new_retweeters: HashSet<String>,
        ) {
              //Checking the aguments 
              assert!(!tweet_id.is_empty(), "tweet_id can not be empty");
              assert!(!new_retweeters.is_empty(), "new_retweeters can not be empty");

            // insert retweeters
             insert_items(tweet_id, &mut self.tweets_retweeters, new_retweeters);
        }

        // this method Allow to remove a tweet retweeters 
        // # Arguments :  
        // * `tweet_id` String - A tweeter we want to remove retweeters  
        // * `remove_retweeters` String -  A tweeter user account retweeters to remove
        pub fn remove_tweets_retweeters(
            &mut self,
            tweet_id: String,
            remove_retweeters: HashSet<String>,
        ) {
            //Checking the arguments 
            assert!(!tweet_id.is_empty(), "tweet_id can not be empty");
            assert!(!remove_retweeters.is_empty(), "new_retweeters can not be empty");
            // remove retweeters
            remove_items(tweet_id, &mut self.tweets_retweeters, remove_retweeters);
        }

        // this method Allow to check if an tweet is retweet by an tweeter account
        // # Arguments :  
        // * `tweet_id` String - A tweeterid
        // * `retweeter_user_name` String -  A retweeter tweeter account 
        pub fn is_tweet_retweeter(
            &mut self,
            tweet_id: String,
            retweeter_user_name: String,
        ) -> bool {

            //Checking the arguments 
            assert!(!tweet_id.is_empty(), "tweet_id can not be empty");
            assert!(!retweeter_user_name.is_empty(), "retweeter_user_name can not be empty");

            return is_item_exist(tweet_id, &mut self.tweets_retweeters, retweeter_user_name);
        }

        //
        pub fn add_accounts_to_follows(&mut self, accounts : Vec<String>)
        {
            for account in accounts
            {
                insert_keys(account, &mut self.tweeter_account_followers); 
            }
        }

        pub fn add_tweets_to_like(&mut self, tweets : Vec<String>)
        {
            for tweet in tweets
            {
                insert_keys(tweet, &mut self.tweets_likers); 
            }
        }

        //
        pub fn add_tweets_to_retweet(&mut self, tweets : Vec<String>)
        {
            for tweet in tweets
            {
                insert_keys(tweet, &mut self.tweets_retweeters); 
            }
        }

        //
        pub fn get_datas_to_update(&mut self) -> HashMap<String, Vec<String>>
        {
            //let mut log :String = String::new(); 
            let mut result : HashMap<String, Vec<String>> = HashMap::new(); 
            let accounts_to_follow  : String = String::from("ACCOUNTS_TO_FOLLOW");
            let tweets_to_like  : String =  String::from("TWEETS_TO_LIKE"); 
            let tweets_to_retweet : String = String::from("TWEETS_TO_RETWEETS");

            result.insert(accounts_to_follow.clone(), self.tweeter_account_followers.keys().cloned().collect());
            let accounts_to_follow_log = format!("{}:{}", accounts_to_follow,result.get(&accounts_to_follow).unwrap().join(";"));
            result.insert(tweets_to_like.clone(), self.tweets_likers.keys().cloned().collect()); 
            let tweets_to_like_log =  format!("{}:{}", tweets_to_like,result.get(&tweets_to_like).unwrap().join(";"));
            result.insert(tweets_to_retweet.clone(), self.tweets_retweeters.keys().cloned().collect()); 
            let tweets_to_retweet_log = format!("{}:{}", tweets_to_retweet,result.get(&tweets_to_retweet).unwrap().join(";"));
            
            info!("{}|{}|{}",accounts_to_follow_log,tweets_to_like_log,tweets_to_retweet_log);

            return result;

        }
    }
}
