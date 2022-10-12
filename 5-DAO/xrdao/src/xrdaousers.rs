/// ***This submission does not need to be judged due to being incomplete***
/// 
/// 


// use sbor::decode; // required for bool?
use scrypto::prelude::*;
use std::iter::FromIterator;

// User NFT is an NFT that represents users for this protocol. This NFT contains all the records of the user
// interacting with this protocol.
#[derive(NonFungibleData)]
pub struct User {
    #[scrypto(mutable)]
    pub handle: String,  // Name, handle, discord, telegram etc
    #[scrypto(mutable)]
    pub rank: Rank, // from Rank enum above
    #[scrypto(mutable)]
    pub xrdao_balance: Decimal, 
    #[scrypto(mutable)]
    pub rep_balance: Decimal, // balance of reputation
    #[scrypto(mutable)]
    pub delegated_vote_power: HashMap<NonFungibleId, Decimal>,
    #[scrypto(mutable)]
    pub total_vote_power: Decimal, // Total vote power = xrdao balance + delegated_vote_power
    #[scrypto(mutable)]
    pub vote_power_delegated: bool, // Is users total vote power delegated?
    #[scrypto(mutable)]
    pub vote_power_delegated_to: Option<NonFungibleId>, // SBT ID this user's total vote power is delegated to
    #[scrypto(mutable)]
    pub vote_record: HashMap<ResourceAddress, Decimal>,
}

// This enum describes the different ranks of repuatation for each user
#[derive(TypeId, Encode, Decode, Describe, Debug, PartialEq)]
pub enum Rank { 
    // Highest rep, linear relationship between rxdao and voting power, top 1% of users, 3x reward multiplier
    President, 
    // x = y^1.1 voting power, top 10% of users, 1.67x reward multiplier
    VicePresident, 
    // x = y^1.2 voting power, top 30% of users, 1.25x reward multiplier
    Senator, 
    // x = y^1.3 voting power, top 50% of users, 1x reward multiplier
    Governor, 
    // x = y^1.5 voting power, top 75% of users, .925x reward multiplier
    Citizen, 
    // Lowest rep, quadratic relationship, x = y^2 voting power, top 100% of users, 0x reward multiplier
    Pleb, 
}

blueprint! {
    // Everything here deals with the SBT data management.
    struct XrdaoUser {
        
        // Vault that holds the authorization badge
        sbt_badge_vault: Vault, 
        // Collects User SBT Address
        sbt_address: ResourceAddress, 
        // This is the user record registry. It is meant to allow people to query the users that belongs to this protocol.
        user_record: HashMap<NonFungibleId, User>, 
        // Keeps a record of wallet addresses to ensure that maps 1 SBT to 1 Wallet.
        account_record: Vec<ComponentAddress>, 
        // NFT ID, rep_balance from sbt
        rep_leaderboard: Hashmap<NonFungibleId, Decimal>,
        // Epoch of last decay
        last_decay_epoch: u64,
    }

    // Instantiates the XRDaoUser component. This is instantiated through the XRDao component. 
    impl XrdaoUser {
        
        pub fn new(access_badge_address: ResourceAddress) -> ComponentAddress{
            let access_rules: AccessRules = AccessRules::new()
                // .method("new_user", rule!(require(access_badge_address))) anyone should be able to create new user
                .method("new_user", rule!(require(access_badge_address)))
                .method("inc_xrdao_balance", rule!(require(access_badge_address)))
                .method("dec_xrdao_balance", rule!(require(access_badge_address)))
                .method("inc_rep_balance", rule!(require(access_badge_address)))
                .method("dec_rep_balance", rule!(require(access_badge_address)))
                .method("update_rank", rule!(require(access_badge_address)))
                .method("update_vote_record", rule!(require(access_badge_address)))
                .method("inc_delegated_vote_power", rule!(require(access_badge_address)))
                .method("dec_delegated_vote_power", rule!(require(access_badge_address)))
                .method("update_total_vote_power", rule!(require(access_badge_address)))
                .default(rule!(allow_all));

            // Badge that will be stored in the component's vault to provide authorization to update the User NFT.
            let sbt_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("user", "XRDao Protocol Badge")
                .initial_supply(1);

            // NFT description for user identification. 
            let sbt_address = ResourceBuilder::new_non_fungible()
                .metadata("user", "XRDao User")
                .mintable(rule!(require(sbt_badge.resource_address())), LOCKED)
                .burnable(rule!(require(sbt_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .updateable_non_fungible_data(rule!(require(sbt_badge.resource_address())), LOCKED)
                .no_initial_supply();
            
            let component = Self {
                sbt_badge_vault: Vault::with_bucket(sbt_badge),
                sbt_address: sbt_address,
                user_record: HashMap::new(), // collects  ************************************************************************************
                account_record: Vec::new(), // collected to make sure only 1 SBT per 1 account address
                rep_leaderboard: HashMap::new(), // Collects hashmap of (NonFungibleId of SBT, reputation balance)
                last_decay_epoch: 0, 
                
            }

            let x = component.instantiate()
            component.add_access_check(access_rules);
            (component.globalize(), sbt_badge)
        }
    
        // Creates a Soul Bount Token (SBT) for a new user to the XRDao Protocol
        // **Check 1:** Checks for 1 SBT per 1 account address.
        // Takes handle, and users account address
        // Returns bucket with user's SBT
        pub fn new_user(&mut self, account_address: ComponentAddress, handle: String) -> Bucket {

            // Checks whether the account address has already registered an SBT
            assert_ne!(self.account_record.contains(&account_address), true, "SBT already created for this account.");
            
            let new_user_sbt: Bucket = self.sbt_badge_vault.authorize(|| {  // Mint NFT to give to users as identification
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.sbt_address);
                resource_manager.mint_non_fungible(
                    &NonFungibleId::random(),
                    // The starting User data with handle from user input, with no balances, empty voting record, and lowest rank
                    User { 
                        handle: handle,
                        xrdao_balance: 0,
                        rep_balance: 0,
                        vote_record: HashMap::new(),
                        rank: Pleb;
                        delegated_vote_power 0,
                        total_vote_power: 0,
                    },
                    new_user_sbt
                )
            });

            // creates user reward vault
            let user_reward_vault: Vault = Vault::new(RADIX_TOKEN);
            // finds user SBT ID
            let user_id: NonFungibleId = new_user_sbt.non_fungible::<User>().id();
            // Stores SBT ID, Vault for user
            self.user_reward_vaults.insert(user_id, user_reward_vault);
            // Stores SBT data
            let sbt_data: User = new_user_sbt.non_fungible().data();
            // updates records held within component
            self.user_record.insert(user_id, sbt_data);
            self.account_record.push(account_address);

            new_user_sbt // Returns NFT to user
        }
    
        // Method call to authorize and increase user xrdao balance on SBT
        pub fn inc_xrdao_balance(&mut self, user_id: &NonFungibleId, amount: Decimal) {
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            *sbt_data.xrdao_balance += amount; // Increases the xrdao balance value stored on SBT
            self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.
        }

        // Method call to authorize and decrease user xrdao balance on SBT
        pub fn dec_xrdao_balance(&mut self, user_id: &NonFungibleId, amount: Decimal) {
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            *sbt_data.xrdao_balance -= amount; // Decreases the xrdao balance value stored on SBT
            self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.
        }     

        // Method call to authorize and increase user rep balance on SBT
        pub fn inc_rep_balance(&mut self, user_id: &NonFungibleId, amount: Decimal) { 
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            *sbt_data.rep_balance += amount; // Increases the rep balance value stored on SBT
            self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.

            // updates rep_leaderboard hashmap stored in xrdaouser component
            // If the rep leaderboard doesnt contain a key = user's NFT ID
            if self.rep_leaderboard.contains_key(&sbt_id) = false { 
                // Then insert into leaderboard hashmap this users NFT ID, amount of rep deposited
                self.rep_leaderboard.insert(&sbt_id, amount); 
            // otherwise, update the existing value for this key by adding amount of rep deposited to it
            } else {  
                self.rep_leaderboard.get_mut(&sbt_id).unwrap() += amount;
            }
            update_rank(user_id);
        }

        // Method call to authorize and decrease user rep balance on SBT
        pub fn dec_rep_balance(&mut self, user_id: &NonFungibleId, amount: Decimal) {
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            *sbt_data.rep_balance -= amount; // Decreases the rep balance value stored on SBT
            self.authorize_update(&user_id, user_sbt); // Authorizes the update to SBT.
            
            //remove key and value from hashmap for sbt_id if there is no rep balance
            self.rep_leaderboard.get_mut(&sbt_id).unwrap() -= amount; 
                if user_sbt_data.rep_balance == 0 {
                    self.rep_leaderboard.remove(&sbt_id);
                }
    
            update_rank(user_id);
        } 
        
        // Method to take hashmap of rep leaderboard, convert to vector, and sort by rep balance
        pub fn sort_rep_leaderboard(&mut self) -> (u64, Vec) {
            // convert leaderboard to vector
            let vec_rep_leaderboard = Vec::from_iter(self.rep_leaderboard); 
            // sort leaderboard by value
            let sorted_vec_rep_leaderboard = vec_rep_leaderboard.sort_by(|&(_, a), &(_, b)| b.cmp(&a)); 
            // position of last rank of NFTs
            let total_users = sorted_vec_rep_leaderboard.len(); 
            // returns the number of total users and the sorted vector of rep leaderboard
            (total_users, sorted_vec_rep_leaderboard) 
        }

        // Method call to authorize and update rank according to rep_balance
        pub fn update_rank(&mut self, user_id: &NonFungibleId) {
            
            // Finds user nft id & nft data from proof
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id); 
            // sort leaderboard
            let (total_users, sorted_vec_rep_leaderboard) = sort_rep_leaderboard();
            // find what position the users NFT Id is
            let position: u64 = sorted_vec_rep_leaderboard.iter().position(|&(x, _)| x == &sbt_id) 
            // Calculate what percentile user is in, +1 acounts for 0 position in vec
            let rank_percent: Decimal = (position + 1)/total_users; 

            // Assign ranks based on percentile
            if rank_percent <= dec!("1") {
                sbt_data.rank == Rank::President
            } else if rank_percent > dec!("1") && rank_percent <= dec!("10") {
                sbt_data.rank == Rank::VicePresident
            } else if rank_percent > dec!("10") && rank_percent <= dec!("30") {
                sbt_data.rank == Rank::Senator
            } else if rank_percent > dec!("30") && rank_percent <= dec!("50") {
                sbt_data.rank == Rank::Governor
            } else if rank_percent > dec!("50") && rank_percent <= dec!("90") {
                sbt_data.rank == Rank::Citizen
            } else if rank_percent > dec!("90") && rank_percent <= dec!("100") {
                sbt_data.rank == Rank::Pleb
            }
            self.authorize_update(&sbt_id, sbt_data);
        }
        
        // Method call to authorize and add to vote_record on user's SBT ********************************************************************
        pub fn update_vote_record(&mut self, proposal: ComponentAddress, user_id: Proof, vote: Vote, vote_weight: u64 ) {  
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);

            let vote_entry: HashMap<Vote, u64> = Hashmap::new();
            vote_entry.insert(vote, vote_weight); // Creates hashmap of individual vote
            sbt_data.vote_record.insert(protposal, vote_entry); // Adds hashmap of (proposal component address, vote_entry)
            self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.
        }

        // Automatically decays reputation over time.  
        // Each 500 epochs, 5% or 5 rep is decreased for each user (whichver is larger)
        pub fn reputation_decay(&mut self) {
            
            let current_epoch = Runtime::current_epoch();
            let next_decay_epoch = self.last_decay_epoch + 500;
            assert!(current_epoch >= next_decay_epoch, false, "Next decay will take place at epoch {} ", next_decay_epoch)
            
            // For every users rep balance in sbt data, decrease by actual decay rate
            for (sbt_id, rep_balance) in self.rep_leaderboard {
                // Take decay of all SBT rep balances
                    
                let decay_rate = dec!(".05"); // rep decays 5% every 500 epochs
                let minimum_decay = dec!("5"); // minimum rep to decay is 5 every 500 epochs
                let decay_rate_amount = user_sbt_data.rep_balance * decay_rate;
                if decay_rate_amount < minimum_decay {
                    actual_decay = minimum_decay
                } else {
                    actual_decay = decay_rate_amount
                }
            }

            // decreases rep by amount passed in for user SBT, updates rank after
            if rep_balance < actual_decay {
                dec_rep_balance(sbt_id, rep_balance); 
            } else {
                dec_rep_balance(sbt_id, actual_decay); 
            }
        }

        // Simple change to handle in SBT data
        pub fn change_handle(&mut self, user_id: NonFungibleId, new_handle: String) {
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            sbt_data.handle = new_handle;
            self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.
        }

        // I dont have a proof here because this changes another's SBT ****************************************************
        pub fn inc_delegated_vote_power(&mut self, user_id: NonFungibleId, amount: Decimal) { 
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.
        }

        // NEED TO ADD A DELEGATED TO STRUCTURE TO ACKNOWLEDGE A PERSONS SBT HAS GIVEN ALL VOTE POWER TO ANOTHER SBT***********************************************
        // I dont have a proof here because this changes another's SBT
        pub fn dec_delegated_vote_power(&mut self, user_id: NonFungibleId, amount: Decimal) { 
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            sbt_data.delegated_vote_power -= amount; // Decreases the delegated vote power value stored on SBT


            self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.
        }

        pub fn update_total_vote_power(&mut self, user_id: NonFungibleId) {
            let mut user_sbt = self.call_resource_mananger(&user_id); // Calls the resource manager.
            user_sbt.total_vote_power = user_sbt.xrdao_balance + delegated_vote_power; // How to add the values for every key in hashmap************************
            self.authorize_update(&user_id, user_sbt); // Authorizes the update to SBT.
        }

        // This method is used to allow users retrieval of their SBT data.
        pub fn get_sbt_info(&self, user_id: NonFungibleId) {
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            let handle = sbt_data.handle;
            let xrdao_balance = sbt_data.xrdao_balance;
            let rep_balance = sbt_data.rep_balance;
            let vote_record = sbt_data.vote_record;
            let rank = sbt_data.rank;
            let delegated_vote_power = sbt_data.delegated_vote_power;
            let total_vote_power = sbt_data.total_vote_power;

            info!("[User SBT]: Handle: {:?}", handle);
            info!("[User SBT]: Rank: {:?}", rank);
            info!("[User SBT]: XRDao Balance: {:?}", xrdao_balance);
            info!("[User SBT]: Reputation Balance: {:?}", rep_balance);
            info!("[User SBT]: Delegated vote power: {:?}", delegated_vote_power);
            info!("[User SBT]: Vote Record: {:?}", vote_record);
        }

        // Asserts that the Proof is for a XRDao user SBT
        // Returns user SBT ID, SBT data
        fn check_and_retrieve_user(&self, sbt: Proof) -> (NonFungibleId, User) {  
            assert_eq!(sbt.resource_address(), self.sbt_address, "Unsupported user SBT");
            let sbt_id = sbt.non_fungible::<User>().id();
            self.retrieve_user_from_id(sbt_id);
            (sbt_id, sbt_data)
        }

        /// Takes SBT ID and returns the ID and the data from the SBT
        fn retrieve_user_from_id(&self, sbt_id: NonFungibleId) -> (NonFungibleId, User) {
            let sbt_manager = borrow_resource_manager!(self.sbt_address);
            let sbt_data = sbt_manager.get_non_fungible_data(&sbt_id);
            (sbt_id, sbt_data)
        }

        // This is a helper function to borrow the resource manager
        // Takes `user_id` (&NonFungibleId) and returns User struct
        fn call_resource_mananger(&self, user_id: &NonFungibleId) -> User {
            let resource_manager = borrow_resource_manager!(self.sbt_address);
            let sbt: User = resource_manager.get_non_fungible_data(&user_id);
            sbt
        }

        // Authorizes data update for the User SBT.
        // Arguments: `user_sbt` (User), `user_id` (&NonFungibleId)
        // Returns: nothing
        fn authorize_update(&mut self, user_id: &NonFungibleId, user_sbt: User) {
            let resource_manager = borrow_resource_manager!(self.sbt_address);
            self.sbt_badge_vault.authorize(|| resource_manager.update_non_fungible_data(&user_id, user_sbt));
        }

    }
    
}

    
