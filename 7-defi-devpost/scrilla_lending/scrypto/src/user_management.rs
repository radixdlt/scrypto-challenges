use scrypto::prelude::*;
use crate::structs::*;
// use crate::price_oracle::*;

#[blueprint]
mod user_management_module {
    // Everything here deals with the SBT data management.
    struct UserManagement {
        
        user_auth_badge: ResourceAddress,
        user_auth_badge_vault: Vault, // Vault that holds the authorization badge
        user_nft_address: ResourceAddress, // stores user NFT resource address
        // user_record: Vec<NonFungibleLocalId>,
        counter: u64,
    }

    // Instantiates the XRDaoUser component. This is instantiated through the XRDao component. 
    impl UserManagement {
        
        pub fn new_usermanagement_module(auth_badge_address: ResourceAddress) -> ComponentAddress {
            let access_rules: AccessRules = AccessRules::new()
                .method("increase_collateral_balance", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .method("decrease_collateral_balance", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .method("increase_borrowed_usds_balance", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .method("decrease_borrowed_usds_balance", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .method("authorize_update", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .method("increase_usds_shield_deposit", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .method("decrease_usds_shield_deposit", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .method("increase_scrilla_stake", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .method("decrease_scrilla_stake", rule!(require(auth_badge_address)), AccessRule::DenyAll)
                .default(rule!(allow_all), AccessRule::DenyAll);

            // Badge that will be stored in the component's vault to provide authorization to update the User NFT.
            let user_auth_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Scrilla Protocol Badge")
                .mint_initial_supply(1);

            // NFT description for user identification. 
            let user_nft = ResourceBuilder::new_integer_non_fungible()
                
                .mintable(rule!(require(user_auth_badge.resource_address())), LOCKED)
                .burnable(rule!(require(user_auth_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(user_auth_badge.resource_address())), LOCKED)
                .metadata("name", "Scrilla User")
                .create_with_no_initial_supply();
            
            let mut user_management_component = Self {

                user_auth_badge: user_auth_badge.resource_address(),
                user_auth_badge_vault: Vault::with_bucket(user_auth_badge),
                user_nft_address: user_nft,
                counter: 1,
            }

            .instantiate();
            user_management_component.add_access_check(access_rules);
            user_management_component.globalize()   
        }
    
        // Creates a new user NFT for caller
        pub fn new_user(&mut self) -> Bucket {
            let new_user_nft: Bucket = self.user_auth_badge_vault.authorize(|| {
                let resource_manager: &mut ResourceManager = borrow_resource_manager!(self.user_nft_address);
                resource_manager.mint_non_fungible(
                    &NonFungibleLocalId::Integer(self.counter.into()),
                    User { 
                        xrd_collateral: dec!(0),
                        usds_borrowed:  dec!(0),
                        loan_collateralization_rate: None,
                        shield_deposits: Vec::new(),
                        scrilla_deposits: Vec::new(),
                        current_usds_in_shield: dec!(0),
                        current_scrl_staked: dec!(0),
                    },
                )
            });

            // add 1 to counter for NFT IDs
            self.counter += 1;

            // Returns NFT to user
            new_user_nft
        }

        // Method call to authorize and increase user collateral balance on NFT User data
        pub fn increase_collateral_balance(&mut self, user_id: NonFungibleLocalId, amount: Decimal, mut data: User) {

            // update the total collateral amount on user's NFT
            data.xrd_collateral += amount;

            // authorize the update for these changes to user's NFT
            self.authorize_update(user_id, data);
        }

        // Authorizes data update for the User NFT.
        // Arguments: `user_data` (User), `user_id` (&NonFungibleLocalId)
        // Returns: nothing
        pub fn authorize_update(&mut self, user_id: NonFungibleLocalId, user_data: User) {

            let resource_manager = borrow_resource_manager!(self.user_nft_address);

            self.user_auth_badge_vault.authorize(|| resource_manager.update_non_fungible_data(&user_id, user_data));
        }

        /// takes nonfungibleid and returns nonfungibleid, nft data
        pub fn get_data_from_id(&self, nft_id: NonFungibleLocalId) -> User {
            
            let resource_manager: &mut ResourceManager = borrow_resource_manager!(self.user_nft_address);
            
            let nft_data: User = resource_manager.get_non_fungible_data(&nft_id);

            nft_data
        }

        // Method call to authorize and decrease user xrd collateral balance on NFT
        pub fn decrease_collateral_balance(&mut self, user_id: NonFungibleLocalId, amount: Decimal, mut data: User) {
            
            // Update user NFT data
            data.xrd_collateral -= amount;
            
            // authorize the update for these changes to user's NFT
            self.authorize_update(user_id, data);
        }     

        // Method call to authorize and increase user usds borrowed balance on NFT
        pub fn increase_borrowed_usds_balance(&mut self, user_id: NonFungibleLocalId, amount: Decimal, mut data: User) { 
            
            data.usds_borrowed += amount;
            
            // authorize the update for these changes to user's NFT
            self.authorize_update(user_id, data);
        }

        // Method call to authorize and decrease user usds borrowed balance on NFT
        pub fn decrease_borrowed_usds_balance(&mut self, user_id: NonFungibleLocalId, amount: Decimal, mut data: User) {
            
            // update count of usds borrowed on user's NFT
            data.usds_borrowed -= amount;
            
            self.authorize_update(user_id, data);
        } 
        
        // Method call to authorize and increase user shield deposit balance on NFT
        pub fn increase_usds_shield_deposit(&mut self, user_id: NonFungibleLocalId, amount: Decimal, mut data: User) { 

            // update count of usds borrowed on user's NFT
            data.current_usds_in_shield += amount;

            self.authorize_update(user_id, data); // Authorizes the update to SBT.
        }

        /// Method called from call_liquidation method in Scrilla blueprint to assist in
        /// updating user NFT after a liquidation.
        pub fn liquidate_user_nft(&mut self, user_id: NonFungibleLocalId, mut data: User) { 
            
            // Update user NFT that got liquidated
            data.xrd_collateral = dec!(0);
            data.usds_borrowed = dec!(0);
            data.loan_collateralization_rate = None;
            self.authorize_update(user_id, data);
        }
        
         // Method call to authorize and decrease user shield deposit balance on NFT
        pub fn decrease_usds_shield_deposit(&mut self, user_id: NonFungibleLocalId, mut data: User) { 
            
            // update count of usds borrowed on user's NFT
            data.current_usds_in_shield = dec!(0);
            
            // I plan to implement a total transaction history here in future instead of just clearing ********************
            data.shield_deposits.clear();

            self.authorize_update(user_id, data); // Authorizes the update to SBT.
        }

        /// delivers user NFT address when called in Scrilla component for verification
        pub fn return_user_nft_address(&mut self) -> ResourceAddress { 
            
            let address = self.user_nft_address;
            
            address
        }

        // Method call to authorize the decrease of scrilla stake on users NFT
        pub fn decrease_scrilla_stake(&mut self, user_id: NonFungibleLocalId, mut data: User) {
    
            // update count of usds borrowed on user's NFT
            data.current_scrl_staked = dec!(0);
            
            // I plan to implement a total transaction history here in future instead of just clearing ********************
            data.scrilla_deposits.clear();

            self.authorize_update(user_id, data); // Authorizes the update to SBT.
        } 
        
        // Method call to authorize the increase of scrilla stake on users NFT
        pub fn increase_scrilla_stake(&mut self, user_id: NonFungibleLocalId, amount: Decimal, mut data: User) { 

            // update count of usds borrowed on user's NFT
            data.current_scrl_staked += amount;

            self.authorize_update(user_id, data); // Authorizes the update to SBT.
        }
    }
}