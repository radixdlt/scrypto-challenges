use scrypto::prelude::*;


#[derive(NonFungibleData)]
pub struct SharedAsset {
	asset_name: String,
	asset_description: String,
	investment_goal: Decimal
	//accepted_token_address: ResourceAddress
	// mut owners? 
}


blueprint! {
    struct Fond {
        admin_badge: Vault,
        current_campaigns_vault: Vault,
        inventory_vault: Vault,
    }


    impl Fond {
        /// Creates a new Fond component.
        ///
        /// This function creates a new Fond component that lists items it intends to buy. To achieve this it collects investments
        /// from users that get an ownership badge (NFT) with their share of the item (proportional to their investment).
        /// (LATER) The Fond may accept different tokens for each item it lists, but all investments for the same item must be of the same token.
        /// Later, the Fond may decide to sell an item that was previously bought, thus distributing the appropriate funds to the users
        /// that invested in the item, according to their share.
        ///
        /// Initialises vaults, admin_badge
        /// 
        /// Checks:
        /// * **Check 1:** TODO:
        ///
        /// ### Arguments:
        /// * Are there any arguments here?
        ///
        /// #### Returns:
        /// * `ComponentAddress` - The address of the `Fond` component just created.
        pub fn instantiate_fond() -> ComponentAddress {
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .metadata("name", "Fond admin auth")
            .initial_supply(1);

            let current_campaigns_address = ResourceBuilder::new_non_fungible()
            .metadata("name", "Current Campaigns of company")
            .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
            .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
            .no_initial_supply();

            let current_campaigns_bucket = Bucket::new(current_campaigns_address);            


            let inventory_address = ResourceBuilder::new_non_fungible()
                .metadata("name", "Inventory of company")
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let inventory_bucket = Bucket::new(inventory_address);

            Self {
                admin_badge: Vault::with_bucket(admin_badge),
                current_campaigns_vault:  Vault::with_bucket(current_campaigns_bucket),
                inventory_vault: Vault::with_bucket(inventory_bucket)
            }
            .instantiate()
            .globalize()
        }


        /// The platform lists an item that it intends to buy, i.e., it mints an NFT that represents said asset
		/// with the appropriate metadata. Then stores it in a vault containing all listed assets 
		/// (assets that were not yet bought) and initializes the funds vault for that asset.
		/// 
        /// Checks:
        /// * **Check 1:** TODO:
		///
		/// ### Arguments:
		/// * `asset_name` (String) - Name of the asset to be bought
		/// * `asset_description` (String) - Description of the asset to be bought
		/// * `investment_goal` (Decimal) - Price of the asset to be bought
		/// * `accepted_token_address` (ResourceAddress) - Address of the token that is accepted as payment
		///
		/// ### Returns:
		/// * `Bucket` - A bucket containing all the listed items.
		/// 
        pub fn create_campaign(
            &mut self, 
            asset_name: String, 
			asset_description: String,
			investment_goal: Decimal
        ) -> Bucket {

            // create new_asset bucket
            // let new_asset = (
            //     NonFungibleId::random(),
            //     SharedAsset {
            //         asset_name: asset_name, 
            //         asset_description: asset_description,
            //         investment_goal: investment_goal
            //     }
            // );
            // let new_asset_bucket = ResourceBuilder::new_non_fungible()
            //     .initial_supply(new_asset);

            // self.current_campaigns_vault.put(new_asset_bucket.take(1))

                // alternative
                
                
            ComponentAuthZone::push(self.admin_badge.create_proof());

            let campaign_resource_manager = borrow_resource_manager!(self.current_campaigns_vault.resource_address());

            let asset = SharedAsset {
                asset_name: asset_name, 
                asset_description: asset_description,
                investment_goal: investment_goal
            };
            self.current_campaigns_vault.put(
                campaign_resource_manager
                    .mint_non_fungible(&NonFungibleId::random(), asset),
            );

            ComponentAuthZone::pop().drop();
   

            self.current_campaigns_vault.take_all()
        }
    }

}
