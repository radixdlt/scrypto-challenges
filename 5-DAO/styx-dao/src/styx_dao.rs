//! Main blueprint with which members of the DAO will interact with

use scrypto::prelude::*;
use crate::ballot_box::BallotBox;
use crate::proposal::{Vote, Change};
use crate::voter_card::VoterCard;

blueprint! {
    struct Styx {

        /// Vault containing all Styx tokens owned by the DAO
        styx_vault: Vault,

        /// Total of Styx tokens emitted by the DAO
        emitted_tokens: Decimal,

        /// Vault containing the admin_badge of the DAO
        internal_authority : Vault,

        /// Vault containing Styx tokens locked by the users of the DAO
        locker_vault : Vault,

        /// Address of the Styx token
        styx_address: ResourceAddress,

        /// Address of the VoterCard NFT
        voter_card_address: ResourceAddress,

        /// Id of the next voting card that will be minted
        new_voter_card_id: u64,

        /// Ballot Box dealing with votes in the DAO
        ballot_box: BallotBox,

        /// Assets owned and managed by the DAO
        assets_under_management: HashMap<ResourceAddress, Vault>,

        /// Assets that can be claimed by specific members of the DAO
        claimable_assets: HashMap<u64, HashMap<ResourceAddress, Decimal>>
    }

    impl Styx {
        

        /// Instantiates a Styx DAO and returns the address of the blueprint and a Bucket containing an admin badge.
        /// The admin badge can be used to emit new Styx tokens or withdraw some tokens from the styx_vault.
        ///
        /// # Arguments
        /// * `initial_supply` - Initial Supply of Styx tokens to put in the styx_vault
        pub fn instantiate(initial_supply: Decimal) -> (ComponentAddress, Bucket) {


            // If the DAO is not instantiated with an admin badge, a default one is created
            // and returned
            let default_admin_badge = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .metadata("name", "External Admin Badge")
            .burnable(rule!(allow_all), LOCKED)
            .initial_supply(dec!(1));
 
            Self::instantiate_custom(default_admin_badge, initial_supply)
        }


        /// Instantiates a Styx DAO and returns the address of the blueprint and a Bucket containing the given admin badge.
        /// will be able to emit new Styx tokens or withdraw some tokens.
        ///
        /// # Arguments
        /// * `initial_supply` - Initial supply of Styx tokens to put in the styx_vault
        /// * `admin_badge` - Admin badge to give permission to mint and withdraw to
        pub fn instantiate_custom(admin_badge : Bucket, initial_supply: Decimal) -> (ComponentAddress, Bucket) {

            // Creates the admin badge owned by the DAO contract
            let internal_admin: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Internal Admin Badge")
                .burnable(rule!(allow_all), LOCKED)
                .initial_supply(dec!(1));

            // Access rule that requires the internal badge
            let internal_access: AccessRule = rule!(require(internal_admin.resource_address()));

            let blueprint_rules: AccessRules = AccessRules::new()
                .method("withdraw", rule!(require(internal_admin.resource_address()) || require(admin_badge.resource_address())))
                .method("emit", rule!(require(internal_admin.resource_address()) || require(admin_badge.resource_address())))
                .default(rule!(allow_all));


            // Creation of the Styx token
            let styx_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Styx")
                .metadata("symbol", "STX")
                .updateable_metadata(
                    internal_access.clone(),
                    MUTABLE(internal_access.clone())
                )
                .mintable(
                    rule!( require(internal_admin.resource_address())),
                    MUTABLE(internal_access.clone())
                )
                .initial_supply(initial_supply);

            let styx_address: ResourceAddress = styx_bucket.resource_address();

            // Creation of the address of voter cards NFT
            let voter_card_address = ResourceBuilder::new_non_fungible()
                .metadata("name","VoterCard")
                .mintable(internal_access.clone(), LOCKED)
                .burnable(internal_access.clone(), LOCKED)
                .restrict_withdraw(internal_access.clone(), MUTABLE(internal_access.clone()))
                .updateable_non_fungible_data(internal_access.clone(), LOCKED)
                .no_initial_supply();


            let styx_dao = Self {
                styx_vault: Vault::with_bucket(styx_bucket),
                internal_authority: Vault::with_bucket(internal_admin),
                voter_card_address : voter_card_address,
                locker_vault : Vault::new(styx_address),
                styx_address,
                ballot_box: BallotBox::new(),
                new_voter_card_id: 0,
                emitted_tokens: initial_supply,
                assets_under_management: HashMap::new(),
                claimable_assets: HashMap::new()
            };


            info!(" external_admin : {},
                    internal_admin : {},
                    styx : {},
                    voter_card : {}",
                    admin_badge.resource_address(),
                    styx_dao.internal_authority.resource_address(),
                    styx_dao.styx_address,
                    styx_dao.voter_card_address
                );
            
            let mut dao = styx_dao.instantiate();
            dao.add_access_check(blueprint_rules);

            return (dao.globalize(),admin_badge)
        }

        /// Mints a new voter card with initial locked tokens given as deposit and returns it
        ///
        /// # Arguments
        /// * `deposit` - Bucket containing some Styx tokens to deposit
        pub fn mint_voter_card_with_bucket(&mut self, deposit : Bucket) -> Bucket {
            assert_eq!(deposit.resource_address(), self.styx_address);

            info!("You are going to lock : {}", deposit.amount());
            let mut voter_card = VoterCard::new(self.new_voter_card_id);
            if !deposit.amount().is_zero()
            {
                voter_card.add_tokens(deposit.amount(), Runtime::current_epoch());
            }

            let voter_card_bucket = self.internal_authority.authorize(|| {
                borrow_resource_manager!(self.voter_card_address).mint_non_fungible(
                    &NonFungibleId::from_u64(self.new_voter_card_id),
                    voter_card
                )
            });
            self.locker_vault.put(deposit);
            self.new_voter_card_id+=1;

            voter_card_bucket
        }

        /// Withdraws money from the styx_vault and returns it
        ///
        /// # Arguments
        /// * `amount` - amount of Styx tokens to emit
        ///
        /// # Access Rule
        /// Can only be called by this blueprint or the owner of the DAO
        pub fn withdraw(&mut self, amount: Decimal) -> Bucket
        {
            assert!(amount <= self.styx_vault.amount());
            self.styx_vault.take(amount)
        }

        /// Emits a certain amount of Styx tokens and deposit them in the styx_vault
        ///
        /// # Arguments
        /// * `amount` - amount of Styx tokens to emit
        ///
        /// # Access Rule
        /// Can only be called by this blueprint or the owner of the DAO
        pub fn emit(&mut self, amount: Decimal)
        {
            let bucket = self.internal_authority.authorize(|| {
                borrow_resource_manager!(self.styx_address).mint(amount)
            });
            self.emitted_tokens = self.emitted_tokens + amount;
            self.styx_vault.put(bucket);
        }

        /// Locks the deposited amount of Styx tokens and updates the VoterCard associated with the proof
        ///
        /// # Arguments
        /// * `voter_card_proof` - Proof of the user's VoterCard
        /// * `deposit` - Bucket containing Styx tokens to lock
        pub fn lock(&mut self, voter_card_proof : Proof, deposit : Bucket)
        {
            assert_eq!(deposit.resource_address(), self.styx_address);
            let validated_proof = self.check_proof(voter_card_proof);

            let amount = deposit.amount();
            let mut voter_card : VoterCard = self.get_voter_card_data_from_proof(&validated_proof);
            voter_card.add_tokens(amount, Runtime::current_epoch());
            self.change_data(&validated_proof, voter_card);

            self.styx_vault.put(deposit);

        }
        /// Unlocks the given amount of Styx tokens and updates the VoterCard associated with the proof
        ///
        /// # Arguments
        /// * `voter_card_proof` - Proof of the user's VoterCard
        /// * `deposit` - amount of tokens to unlock
        pub fn unlock(&mut self, proof : Proof, amount: Decimal) -> Bucket
        {

            let validated_proof = self.check_proof(proof);
            let mut voter_card : VoterCard = self.get_voter_card_data_from_proof(&validated_proof);
            assert!(voter_card.total_number_of_token >= amount);

            voter_card.retrieve_tokens(amount);

            self.change_data(&validated_proof, voter_card);
            self.locker_vault.take(amount)
        }

        /// Unlocks all the Styx tokens of a user and updates the VoterCard associated with the proof
        ///
        /// # Arguments
        /// * `voter_card_proof` - Proof of the user's VoterCard
        pub fn unlock_all(&mut self, proof : Proof) -> Bucket
        {
            let validated_proof = self.check_proof(proof);

            let mut voter_card : VoterCard = self.get_voter_card_data_from_proof(&validated_proof);

            let total_number_of_token = voter_card.retrieve_all_tokens();

            self.change_data(&validated_proof, voter_card);
            self.locker_vault.take(total_number_of_token)
        }

        /// Make a new Proposal to the Styx DAO. The Proposal then enters the Suggestion phase
        ///
        /// # Arguments
        /// * `description` - description of the Proposal
        /// * `suggested_changes` - list of changes to be made to the DAO
        /// * `voter_card_proof` - proof of the user's VoterCard
        pub fn make_proposal(&mut self, description: String, suggested_changes: Vec<Change>, voter_card_proof: Proof)
        {
            // Check that it is a user of the DAO
            self.check_proof(voter_card_proof);
            self.ballot_box.make_proposal(description, suggested_changes, Runtime::current_epoch(), self.emitted_tokens);
        }

        /// Support a given Proposal that is in Suggestion phase
        ///
        /// # Arguments
        /// * `proposal_id` - id of the Proposal to support
        /// * `voter_card_proof` - proof of the user's VoterCard
        pub fn support_proposal(&mut self, proposal_id: usize, voter_card_proof: Proof)
        {
            let validated_id = self.check_proof(voter_card_proof);
            let mut voter_card = self.get_voter_card_data_from_proof(&validated_id);

            self.ballot_box.support_proposal(proposal_id, &mut voter_card, Runtime::current_epoch());
            self.change_data(&validated_id, voter_card);
        }

        /// Tries to make a Proposal advance to its next phase and executes the changes if the Proposal
        /// is accepted
        ///
        /// # Arguments
        /// * `proposal_id` - id of the Proposal
        pub fn advance_with_proposal(&mut self, proposal_id: usize)
        {
            match self.ballot_box.advance_with_proposal(proposal_id, Runtime::current_epoch())
            {
                // the BallotBox passes changes that are made to this blueprints
                None => {}
                Some(changes) =>
                {
                    for change in changes
                    {
                        match change
                        {
                            Change::AllowSpending(address, amount, to) =>
                                {
                                    self.allow_spending(address, amount, to);
                                }

                            Change::AllowMinting(amount) =>
                                {
                                    self.emit(amount);
                                }
                            _ => { panic!("critical error in code. This should not happen.") }
                        }
                    }
                }
            }
        }

        /// Delegates locked tokens to a given user for a Proposal that is in Voting phase
        ///
        /// # Arguments
        /// * `proposal_id` - id of the Proposal
        /// * `delegate_to` - user's VoterCard id to whom to delegate
        /// * `voter_card_proof` - proof of the user's VoterCard
        pub fn delegate_for_proposal(&mut self, proposal_id: usize, delegate_to: u64, voter_card_proof: Proof)
        {
            let validated_id = self.check_proof(voter_card_proof);
            let mut voter_card = self.get_voter_card_data_from_proof(&validated_id);

            self.ballot_box.delegate_for_proposal(proposal_id, delegate_to, &mut voter_card, Runtime::current_epoch());
            self.change_data(&validated_id, voter_card);
        }

        /// Votes with locked and delegated tokens for a Proposal that is in Voting Phase
        ///
        /// # Arguments
        /// * `proposal_id` - id of the Proposal
        /// * `voter_card_proof` - proof of the user's VoterCard
        /// * `vote` - vote to cast
        pub fn vote_for_proposal(&mut self, proposal_id: usize, voter_card_proof: Proof, vote: Vote)
        {
            let validated_id = self.check_proof(voter_card_proof);
            let mut voter_card = self.get_voter_card_data_from_proof(&validated_id);

            self.ballot_box.vote_for_proposal(proposal_id, &mut voter_card, vote, Runtime::current_epoch());
            self.change_data(&validated_id, voter_card);
        }

        /// Gifts an asset to the DAO and puts it in the assets_under_management
        ///
        /// # Arguments
        /// * `asset` - Bucket containing the asset to gift to the DAO
        pub fn gift_asset(&mut self, asset: Bucket)
        {
            let asset_address = asset.resource_address();
            if asset_address == self.styx_address
            {
                self.styx_vault.put(asset)
            }
            else
            {
                match self.assets_under_management.get_mut(&asset_address)
                {
                    None =>
                        {
                            let mut  vault = Vault::new(asset_address);
                            vault.put(asset);
                            self.assets_under_management.insert(asset_address, vault);
                        }
                    Some(vault) =>
                        {
                            vault.put(asset);
                        }
                }
            }
        }


        /// Returns the amount owned by the DAO in a given asset
        ///
        /// # Arguments
        /// * `asset_address` - address of the asset to check
        pub fn amount_owned(&self, asset_address: ResourceAddress) -> Decimal
        {
            if asset_address == self.styx_address
            {
                self.styx_vault.amount()
            }
            else
            {
                match self.assets_under_management.get(&asset_address)
                {
                    None => Decimal::zero(),
                    Some(vault) => vault.amount()
                }
            }
        }

        /// Returns the amount of DAO tokens locked
        pub fn amount_locked(&self) -> Decimal
        {
            self.locker_vault.amount()
        }


        /// Claims the assets due to a user and returns them as a list of buckets
        ///
        /// # Arguments
        /// * `voter_card_proof` - proof of the user's VoterCard
        pub fn claim_assets(&mut self, voter_card_proof: Proof) -> Vec<Bucket>
        {
            let validated_proof = self.check_proof(voter_card_proof);
            let voter_card = self.get_voter_card_data_from_proof(&validated_proof);

            let mut buckets: Vec<Bucket> = vec![];

            match self.claimable_assets.get_mut(&voter_card.voter_id)
            {
                None => {}
                Some(hashmap) =>
                    {

                        let mut resource_to_remove = vec![];

                        for (resource, amount) in hashmap.iter_mut()
                        {
                            let mut opt_bucket = None;
                            let mut opt_resource_to_remove = None;

                            let vault_to_take_from : Option<&mut Vault>;

                            if *resource == self.styx_address
                            {
                                vault_to_take_from = Some(&mut self.styx_vault);
                            }
                            else {
                                vault_to_take_from = self.assets_under_management.get_mut(resource);
                            }

                            match vault_to_take_from
                            {
                                None => {}
                                Some(vault) =>
                                    {
                                        let mut new_bucket = Bucket::new(*resource);
                                        let owned = vault.amount();
                                        let amount_to_take = owned.max(*amount);

                                        new_bucket.put(vault.take(amount_to_take));

                                        if amount_to_take == owned
                                        {
                                            // If the resource is the DAO token then this line does not do anything
                                            self.assets_under_management.remove(&resource);
                                        }

                                        *amount = *amount - amount_to_take;
                                        opt_bucket = Some(new_bucket);
                                        if amount.is_zero()
                                        {
                                            opt_resource_to_remove = Some(*resource);
                                        }
                                    }
                            }

                            match opt_bucket
                            {
                                None => {},
                                Some(bucket) => { buckets.push(bucket); }
                            }

                            match opt_resource_to_remove
                            {
                                None => {},
                                Some(resource_rem) => {resource_to_remove.push(resource_rem);}
                            }
                        }

                        for resource in resource_to_remove.into_iter()
                        {
                            hashmap.remove(&resource);
                        }

                    }
            }

            buckets
        }


        /// Internal function that adds a certain amount of asset owned to be claimable by a user
        ///
        /// # Arguments
        /// * `address` - address of the asset
        /// * `amount` - asset amount to be claimable
        /// * `to` - user that can claim the asset
        fn allow_spending(&mut self, address: ResourceAddress, amount: Decimal, to: u64)
        {

            match self.claimable_assets.get_mut(&to)
            {
                None =>
                    {
                        let mut new_hashmap: HashMap<ResourceAddress, Decimal> = HashMap::new();
                        new_hashmap.insert(address, amount);
                        self.claimable_assets.insert(to, new_hashmap);
                    },
                Some(hashmap) =>
                    {
                        match hashmap.get_mut(&address)
                        {
                            None => { hashmap.insert(address, amount); }
                            Some(tokens) =>
                                {
                                    *tokens = *tokens + amount;
                                }
                        }
                    }
            }

        }

        /// Internal function that changes the data of a VoterCard
        ///
        /// # Arguments
        /// * `valid_proof` - valid_proof of a user's VoterCard
        /// * `new_voter_card` - new data of the VoterCard
        fn change_data(&self, valid_proof: &ValidatedProof, new_voter_card: VoterCard)
        {
            let resource_manager : &mut ResourceManager = borrow_resource_manager!(self.voter_card_address);
            let id = valid_proof.non_fungible::<VoterCard>().id();
            self.internal_authority
                .authorize(|| resource_manager.update_non_fungible_data(&id, new_voter_card));
        }


        /// Internal function that checks that a given Proof corresponds to a unique VoterCard Proof
        /// and returns a ValidatedProof if so
        ///
        /// # Arguments
        /// * `voter_card_proof` - proof of a user's VoterCard
        fn check_proof(&self, voter_card_proof: Proof) -> ValidatedProof
        {

            let valid_proof: ValidatedProof =  voter_card_proof.validate_proof
            (
                    ProofValidationMode::ValidateContainsAmount
                        (
                            self.voter_card_address,
                            dec!(1)
                        )
            ).expect("Invalid proof provided");

            valid_proof
        }

        /// Internal function that extracts the VoterCard data associated to a ValidatedProof
        ///
        /// # Arguments
        /// * `validated_proof` - ValidatedProof associated to a user's VoterCard
        fn get_voter_card_data_from_proof(&self, validated_proof: &ValidatedProof) -> VoterCard
        {
            let resource_manager: &ResourceManager =
                borrow_resource_manager!(self.voter_card_address);
            let id = validated_proof.non_fungible::<VoterCard>().id();
            resource_manager.get_non_fungible_data::<VoterCard>(&id)
        }
    }
}