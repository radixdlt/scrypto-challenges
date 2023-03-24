use scrypto::prelude::*;

#[derive(NonFungibleData, Debug, Encode, Decode, Describe)]
pub struct Owner {
    #[scrypto(mutable)]
    pub percent_ownership_of_treasury: Decimal, // used to store percent ownership of treasury
    #[scrypto(mutable)]
    pub username: Option<String>,  // used to store telegram/discord handle etc
    #[scrypto(mutable)]
    pub contact: Option<String>, // used to store email or any other contact info if desired
}

blueprint! {
    struct Shares {

        // badge kept in component to approve minting of new owners
        owner_mint_badge_vault: Vault,
        
        // badge kept in component to approve minting of new depositors
        depositor_mint_badge_vault: Vault,

        // nft representing ownership of component treasury
        owner_badge_add: ResourceAddress,  // can also deposit tokens with this badge. 

        // shared vault for all owners, ResourceAddress is for token
        component_treasury: HashMap<ResourceAddress, Vault>, 

        // badge allowing deposits into treasury vault - can be given to clients or contributors to deposit to treasury
        depositor_badge: ResourceAddress,

        // starting number of owners that own an equal amount of the treasury
        initial_equal_shareholders: u64,

        // Decimal version of initial_equal_shareholders
        shareholder_decimal: Decimal,

        // collects all NonFungibleIds of current owners
        owner_record: Vec<NonFungibleId>, // owners of nft ids that are taken away when something is merged or added to or split an nft or creating initial owners

        // collects amounts of resources owed to specific owners
        funds_owed: Vec<(NonFungibleId, Decimal, ResourceAddress)>, 
    }

    impl Shares {
        pub fn new_shares_component(initial_equal_shareholders: u64) -> (ComponentAddress, Bucket) {

            let owner_mint_badge: Bucket = ResourceBuilder::new_fungible() // stays in component
            .metadata("name", "owner_mint_badge")
            .restrict_withdraw(rule!(deny_all), LOCKED)
            .divisibility(DIVISIBILITY_NONE)
            .initial_supply(1);

            let depositor_mint_badge: Bucket = ResourceBuilder::new_fungible() // stays in component
            .metadata("name", "depositor_mint_badge")
            .restrict_withdraw(rule!(deny_all), LOCKED)
            .divisibility(DIVISIBILITY_NONE)
            .initial_supply(1); 

            let depositor_badge: ResourceAddress = ResourceBuilder::new_fungible() 
            .metadata("name", "depositor_badge")
            .mintable(rule!(require(depositor_mint_badge.resource_address())), LOCKED)
            .burnable(rule!(require(depositor_mint_badge.resource_address())), LOCKED)
            .restrict_withdraw(rule!(require(depositor_mint_badge.resource_address())), LOCKED)
            .divisibility(DIVISIBILITY_NONE)
            .no_initial_supply();

            // logic calculating ownership % of each owner based upon initial amount of owners
            let initial_share_per_owner: Decimal = dec!(1) / initial_equal_shareholders; 
            let mut owners: Vec<(NonFungibleId, Owner)> = Vec::new();
            for i in 0..initial_equal_shareholders {
                owners.push((NonFungibleId::from_u64(i), Owner { 
                    percent_ownership_of_treasury: initial_share_per_owner,
                    username: None,
                    contact: None,
                        }));        
                }          

            let owner_badge_bucket: Bucket = ResourceBuilder::new_non_fungible()
            .metadata("name", "owner_badge")
            .mintable(rule!(require(owner_mint_badge.resource_address())), LOCKED)
            .burnable(rule!(require(owner_mint_badge.resource_address())), LOCKED)
            .updateable_non_fungible_data(rule!(require(owner_mint_badge.resource_address())), LOCKED) // allows mutable metadata
            .initial_supply(owners);

            let owner_badge_add: ResourceAddress = owner_badge_bucket.resource_address(); // stored this in the sate for convenience in calling 
            
            let shareholder_decimal = Decimal::from(initial_equal_shareholders);

            let rules: AccessRules = AccessRules::new()
                .method("new_depositor", rule!(require(owner_badge_bucket.resource_address())))
                .method("check_or_create_vault", rule!(require(owner_badge_bucket.resource_address())))
                .method("deposit_to_treasury", rule!(require(depositor_badge) || require(owner_badge_bucket.resource_address())))
                .method("distribute_treasury_funds", rule!(require_amount(shareholder_decimal, owner_badge_bucket.resource_address())))
                .default(rule!(allow_all));

            let mut shares_component = Self {
                initial_equal_shareholders: initial_equal_shareholders,
                owner_badge_add,
                owner_mint_badge_vault: Vault::with_bucket(owner_mint_badge), // stays in component to authorize component to mint new owners
                depositor_mint_badge_vault: Vault::with_bucket(depositor_mint_badge), // stays in component to authorize component to mint new depositors
                component_treasury: HashMap::new(),
                depositor_badge: depositor_badge,
                shareholder_decimal,
                funds_owed: Vec::new(),
                owner_record: Vec::new(),
            }

            .instantiate();
            shares_component.add_access_check(rules);
            let component_add = shares_component.globalize();
            
            (component_add, owner_badge_bucket)
        }
        
        /// pushes owner nonfungibleid to owner_record vector
        pub fn push_owner_record(&mut self, owner_badge_bucket: Bucket) -> Bucket{
            
            for owner_id in owner_badge_bucket.non_fungible_ids() {
                
                    self.owner_record.push(owner_id);
            }
            info!("owner_record vector is {:?}", self.owner_record);

            owner_badge_bucket
        }

        /// updates name field on owner nft data
        pub fn update_owner_username(&mut self, username: String, owner_badge_auth: Proof) {
            
            let (id, mut owner_data) = self.call_id_and_data_from_proof(owner_badge_auth); // calls the resource manager for owner_badge nft id from proof
            
            owner_data.username = Some(username); // updates name stored on nft
            
            self.authorize_update(&id, owner_data); // authorizes the update
        }

        /// updates contact field on owner nft data
        pub fn update_owner_contact(&mut self, contact: String, owner_badge_auth: Proof) {
            
            let (id, mut owner_data) = self.call_id_and_data_from_proof(owner_badge_auth); // calls the resource manager for owner_badge nft id from proof
            
            owner_data.contact = Some(contact); // updates contact stored on nft
            
            self.authorize_update(&id, owner_data); // authorizes the update
        }
        
        /// returns 1 new depositor badge to caller that can be then given to person/contract that needs deposit authority
        /// this could be useful for a person wanting to pay a team for work performed instead of an individual or
        /// for a property's revenue stream to deposited to the shared wallet
        pub fn new_depositor(&self) -> Bucket { 
            
            let depositor_badge_resource_manager: &mut ResourceManager = borrow_resource_manager!(self.depositor_badge);
            
            let new_depositor_badge = self.depositor_mint_badge_vault.authorize(|| depositor_badge_resource_manager.mint(1));
            
            new_depositor_badge 
        }
        
        /// checks if a vault exists for the component_treasury, if not creates one
        pub fn check_or_create_vault(&mut self, token_resource_address: ResourceAddress) { 

            if self.component_treasury.contains_key(&token_resource_address) == false {
                
                let new_vault: Vault = Vault::new(token_resource_address);

                self.component_treasury.insert(token_resource_address, new_vault);
            }
        }
        
        /// this method deposits funds to the treasury
        pub fn deposit_to_treasury(&mut self, bucket: Bucket) {
            
            self.check_or_create_vault(bucket.resource_address());
            
            let resource = bucket.resource_address();
            
            let vault = self.component_treasury.get_mut(&resource.clone()).unwrap();
            
            vault.put(bucket);
        }

        /// this method is used to check the balance of a desired token vault
        pub fn show_single_treasury_balance(&self, token_resource_id: ResourceAddress) { 
            
            // check that key entered is actually mapped to a vault
            assert_eq!(self.component_treasury.contains_key(&token_resource_id), true, "No vault exists for this resource.");
            
            // this is the line that prints the balance of the desired token
            info!("The balance of token {:?} is {:?}", &token_resource_id, self.component_treasury.get(&token_resource_id).unwrap().amount()); 
        }
            
        /// this method is used to check the balance of all tokens
        pub fn show_all_treasury_balances(&self) {      
            
            for (resource_id, vault) in &self.component_treasury {
                
                info!("The balance of token {:?} is {:?}", &resource_id, vault.amount());
            }
        }

        /// allows users to take their owner nft and split off some of the ownership into a 2nd owner nft
        pub fn split_ownership(&mut self, split_percent: Decimal, owner_badge_auth: Proof) -> Bucket { 
            
            // add check here to make sure percent is less than the percent on nft metadata being split
            let (id, mut owner_data) = self.call_id_and_data_from_proof(owner_badge_auth);
            
            let original_percent = owner_data.percent_ownership_of_treasury;
            
            assert!(split_percent < original_percent, "you cant split more than you own");
            
            let owner = Owner { 
                percent_ownership_of_treasury: split_percent,
                username: None,
                contact: None,
            };
           
            // borrow resource manager, mint new owner nft with random id and new owner struct with percent split from other owner nft
            let owner_badge_resource_manager: &mut ResourceManager = borrow_resource_manager!(self.owner_badge_add);
            
            let new_owner_badge = self.owner_mint_badge_vault.authorize(|| owner_badge_resource_manager.mint_non_fungible(&NonFungibleId::random(), owner));
            
            owner_data.percent_ownership_of_treasury -= split_percent;

            // update data on original nft (subtract what was split)
            self.authorize_update(&id, owner_data);

            // add new id to owner_record
            self.owner_record.push(id);
        
            new_owner_badge // returns 1 new owner badge to caller
        }

        /// allows users to take two owner nfts from this copmonent and combine ownership metadata into 1 nft
        pub fn merge_ownership(&mut self, owner_badge_auth1: Proof, owner_badge_2: Bucket) {
            
            // get id and data from first proof
            let (id1, mut owner_data1) = self.call_id_and_data_from_proof(owner_badge_auth1);
            
            // get id and data from second proof
            let (id2, owner_data2) = self.call_id_and_data_from_id(owner_badge_2.non_fungible_id());

            // add the two percentages together
            owner_data1.percent_ownership_of_treasury += owner_data2.percent_ownership_of_treasury;

            // authorize update of nft
            self.authorize_update(&id1, owner_data1);

            // remove id to owner_record 
            self.owner_record.retain(|k| !(&id2 == k));
            
            // burn the second nft
            let owner_badge_resource_manager: &mut ResourceManager = borrow_resource_manager!(self.owner_badge_add);
            
            self.owner_mint_badge_vault.authorize(|| owner_badge_resource_manager.burn(owner_badge_2));
        }

        /// helper function to more easily update owner_badge metadata
        fn authorize_update(&mut self, owner_badge_auth: &NonFungibleId, owner_data: Owner) {
            
            let resource_manager: &mut ResourceManager = borrow_resource_manager!(self.owner_badge_add);
            
            self.owner_mint_badge_vault.authorize(|| resource_manager.update_non_fungible_data(&owner_badge_auth, owner_data));
        }
        /// ***UNDER DEVELOPMENT FOR SCENARIO 2***
        /// this method will take in equal amounts of funds from all users 
        /// it requires all owners to provide an equal bucket of a resource upon execution, it will pool all funds into the treasury vault  
        /// this method can be called at any time and requires owners to contribute a pro rata share of total amount being 
        /// escrowed based on ownership (an owner owning half of the component_treasury will have to supply half the escrow funds etc)
        // pub fn pool_escrow_funds(&mut self, escrow_amount: Decimal, token_resource_address: ResourceAddress, buckets: Vec<Bucket>, proofs: Vec<Proof>) { 

        //     let bucket_proof_vec = zip(buckets, proofs);

        //     for (bucket, proof) in bucket_proof_vec {
                
        //         // check the right token is being deposited
        //         assert_eq!(bucket.resource_address(), token_resource_address, "incorrect resource being deposited");
                
        //         // check that each person is depositing at least the required amount
        //         let (_, owner_data) = self.call_id_and_data_from_proof(proof);
        //         let owner_amount_required = owner_data.percent_ownership_of_treasury * escrow_amount  / dec!(1);
        //         assert!(
        //             bucket.amount() / (owner_data.percent_ownership_of_treasury)  >= owner_amount_required,
        //             "This bucket does not contain enough escrow tokens");
                
        //         self.component_treasury.get_mut(&token_resource_address).unwrap().put(bucket);
        //     }
        // }

        /// this method will allow users to withdraw their pro rata share of the escrowed funds by logging the metadata into the funds_owed vector
        pub fn distribute_treasury_funds(&mut self, token_resource_address: ResourceAddress, amount: Decimal) {

            // check that the amount is less than or equal to vault balance
            assert!(amount <= self.component_treasury.get(&token_resource_address).unwrap().amount(), "Not enough funds to withdraw");

            // find list of owner_badge ids, find all ownership %, split bucket to each person
            for owner in self.owner_record.clone() {
                
                let (id, owner_data) = self.call_id_and_data_from_id(owner);
                
                let owner_deposit: Decimal = owner_data.percent_ownership_of_treasury * amount;
                
                self.funds_owed.push((id, owner_deposit, token_resource_address));
            }
            
            info!("owner_record vector after clone is {:?}", self.owner_record);
            
            info!("funds owed vector is {:?}", self.funds_owed);
        }

        /// method for each user to claim funds owed to them from the component_treasury
        pub fn claim_treasury_funds(&mut self, token_resource_address: ResourceAddress, owner_badge_auth: Proof) -> Bucket {
            
            info!("funds owed vector is {:?}", self.funds_owed);
            
            let (owner, _) = self.call_id_and_data_from_proof(owner_badge_auth);
            
            // take id, find what is owed, pass bucket(s) to owner, remove paid funds entries from funds_owed vector       
            info!("current owner is {:?}", owner);
            
            let index_element = self.funds_owed
            .iter()       
            .position(|x| x.0 == owner) // &x -> accessing the element value by reference
            .unwrap();
            
            // takes resources out of vault with matching nft id
            self.component_treasury.get_mut(&token_resource_address).unwrap().take(self.funds_owed[index_element].1)
        }

        /// takes a proof, extracts the nonfungibleid, returns nonfungibleid, owner data
        fn call_id_and_data_from_proof(&self, owner_badge_auth: Proof) -> (NonFungibleId, Owner) {
            
            let validated_proof = owner_badge_auth.validate_proof(ProofValidationMode::ValidateResourceAddress(self.owner_badge_add))
                .expect("invalid proof");

            let nft_id = validated_proof.non_fungible_id();
            
            self.call_id_and_data_from_id(nft_id)
        }

        /// takes nonfungibleid and returns nonfungibleid, owner data
        fn call_id_and_data_from_id(&self, owner_badge_id: NonFungibleId) -> (NonFungibleId, Owner) {
            
            let resource_manager: &mut ResourceManager = borrow_resource_manager!(self.owner_badge_add);
            
            let owner_data: Owner = resource_manager.get_non_fungible_data(&owner_badge_id);

            (owner_badge_id, owner_data)
        } 
    }
}
