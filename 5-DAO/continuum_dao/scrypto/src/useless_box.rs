use scrypto::prelude::*;
use sha2::{Sha256, Digest};

#[derive(NonFungibleData)]
struct UserNFT {
    user_number: u64,
    #[scrypto(mutable)]
    volume_counter: Decimal,
    #[scrypto(mutable)]
    usage_counter: u64,
}

blueprint! {
    struct UselessBox {
        user_counter  : u64,                // counts number of users that have interacted with the component 
        usage_counter: u64,                 // tracks the frequency transacted into the
        volume_counter: Decimal,            // tracks the volume transacted into the useless toy box 
        user_nft_address: ResourceAddress,  // resource address for user NFT 
        param_auth: Vault,                  // admin badge for manipulating component parameters 
        mint_auth: Vault,                   // admin badge for minting component tokens 
        ext_auth: Vec<Vault>,               // external component admin badges
        dummy_parameter: Decimal,           // dummy parameter than can be manipulated 
    }

    impl UselessBox {
        // =====================================================================
        // FUNCTIONS
        // =====================================================================
        pub fn instantiate() -> (ComponentAddress, Bucket) {
            // Admin badge definitions
            let mint_auth = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "UselessBox Mint Badge")
                .initial_supply(1);
            
            let mut param_auth = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "UselessBox Parameter Badge")
                .mintable(rule!(require(mint_auth.resource_address())), LOCKED)
                .initial_supply(2);

            // Resouce definitions 
            let user_nft_address = ResourceBuilder::new_non_fungible()
                .metadata("name", "UselessBox User NFT")
                .mintable(rule!(require(mint_auth.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(mint_auth.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), MUTABLE(rule!(require(mint_auth.resource_address()))))
                .no_initial_supply();
            
            // Component instantiation with AccessRules
            let access_rules = AccessRules::new()
                .method("set_dummy_parameter", rule!(require(param_auth.resource_address())))
                .default(rule!(allow_all));

            let mut component = Self {
                user_counter: u64::zero(),
                volume_counter: Decimal::zero(),
                usage_counter: u64::one(),
                user_nft_address: user_nft_address,
                param_auth: Vault::with_bucket(param_auth.take(1)),
                mint_auth: Vault::with_bucket(mint_auth),
                ext_auth: Vec::<Vault>::new(),
                dummy_parameter: Decimal::zero(),
            }
            .instantiate();
            component.add_access_check(access_rules);
            
            (component.globalize(), param_auth)
        }

        // =====================================================================
        // METHODS
        // =====================================================================
        /// Registers a new user and mint a UserNFT
        pub fn register_new_user(&mut self) -> Bucket {
            // update user count 
            self.user_counter += 1; 

            // generate userId from hashing the user_counter 
            let mut hasher = Sha256::new();
            hasher.update(self.user_counter.to_string());
            let user_counter_hash = hasher.finalize();
            let user_id = NonFungibleId::from_bytes(user_counter_hash.to_vec());

            // mint and return the new NFT 
            self.mint_auth.authorize(|| {
                borrow_resource_manager!(self.user_nft_address)
                    .mint_non_fungible(
                        &user_id, 
                        UserNFT {
                            user_number: self.user_counter,
                            volume_counter: Decimal::zero(),
                            usage_counter: u64::one()
                        }
                    )
                }
            )
        }

        /// Updates an existing NFT from their corresponding proof
        fn update_nft(&self, xrd: &Bucket, proof: ValidatedProof) -> () {
            // update metadata on existing NFT   
            let non_fungible: NonFungible<UserNFT> = proof.non_fungible();
            let mut nft_data = non_fungible.data();
            nft_data.volume_counter += xrd.amount(); 
            nft_data.usage_counter += 1;

            // update NFT metadata globally 
            self.mint_auth.authorize(|| {
                borrow_resource_manager!(self.user_nft_address)
                    .update_non_fungible_data(&non_fungible.id(), nft_data)
            });
        }

        /// The user deposits a bucket of XRD, and gets the bucket in return 
        /// (net-zero transfer). The amount of xrd deposited is cumulatively 
        /// recorded and the interaction frequency is incremented by 1. Same 
        /// information specific to the user is tracked via a UserNFT.
        pub fn deposit(&mut self, xrd: Bucket, proof: Proof) -> Bucket {
            // Component side analytics update
            self.volume_counter += xrd.amount();
            self.usage_counter += 1;

            // User side analytics update (via UserNFT's metadata)
            // new users are minted a UserNFT
            let mode = ProofValidationMode::ValidateContainsAmount(
                self.user_nft_address, Decimal::one());
            
            match proof.validate_proof(mode) {
                Ok(validated_proof) => { self.update_nft(&xrd, validated_proof); },
                Err(_) => { panic!("Multiple UserNFTs detected. Please merge duplicate user NFTs"); }
            };
            
            xrd
        }
        
        /// Update dummy_parameter (AccessRules restricted - param)
        pub fn set_dummy_parameter(&mut self, value: Decimal, proof: Proof) -> () {
            let mode = ProofValidationMode::ValidateResourceAddress(
                self.param_auth.resource_address());
            
            match proof.validate_proof(mode) {
                Ok(_) => { self.dummy_parameter = value; },
                Err(_) => { panic!("Insufficent access to modify parameter"); }
            };

            
        }

        pub fn get_dummy_parameter(&mut self) -> Decimal {
            self.dummy_parameter
        }
        
        /// Returns the total amount of XRD that has been deposited into the 
        /// useless box (amount returned is not counted)
        pub fn get_volume_counter(&self) -> Decimal {
            self.volume_counter
        }

        /// Returns the number of deposit instances that have occured
        pub fn get_usage_counter(&self) -> u64 {
            self.usage_counter
        }

        /// Returns the number of distinct users/wallets that have attempted a 
        /// deposit operation  
        pub fn get_user_counter(&self) -> u64 {
            self.user_counter
        }
    }
}