use scrypto::prelude::*;


#[derive(NonFungibleData)]
pub struct Asset {

}

#[derive(NonFungibleData)]
pub struct InvestorAssetOwnershipBadge {
    //...
    share: Decimal,
    shared_asset_badge_id: NonFungibleId
}

#[derive(NonFungibleData)]
pub struct SharedAsset {
    //SharedAssetBadge might make more sense


	asset_name: String,
	asset_description: String,
	investment_goal: Decimal,

    original_asset_id: NonFungibleId,

    bought: bool,
    sold: bool

	//accepted_token_address: ResourceAddress


}


blueprint! {
    struct Fond {
        admin_badge: Vault,
        //FIXME: name
        current_campaigns_vault: Vault,

        //actual assets
        inventory_vault: Vault,
        
        //Total funds collected per asset, both during investment phase as well as when the item's been sold. 
        // K = (NonFungibleId) The ID of the shared_asset_badge
        // V = (Vault) A vault containing all funds respective to the asset
        collected_assets_funds: HashMap<NonFungibleId, Vault>,

        dead_vaults: Vec<Vault>
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
                inventory_vault: Vault::with_bucket(inventory_bucket),
                collected_assets_funds: HashMap::new(),
                dead_vaults: Vec::new()
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
        /// ### Requires:
        /// * `admin_badge`
        /// 
		/// ### Arguments:
		/// * `asset_name` (String) - Name of the asset to be bought
		/// * `asset_description` (String) - Description of the asset to be bought
		/// * `investment_goal` (Decimal) - Price of the asset to be bought
		/// * (LATER)`accepted_token_address` (ResourceAddress) - Address of the token that is accepted as payment
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
                investment_goal: investment_goal,
                bought: false,
                sold: false,
                original_asset_id: NonFungibleId::random()

            };
            self.current_campaigns_vault.put(
                campaign_resource_manager
                    .mint_non_fungible(&NonFungibleId::random(), asset),
            );

            ComponentAuthZone::pop().drop();
   

            self.current_campaigns_vault.take_all()
        }

         
		/// When funding for an item is achieved, the platform automatically buys the item. 
		/// It transfers the asset from the listed assets vault to the bought assets.
        /// 
		/// TODO: So, everytime a user invests in an item it needs to call a function that checks that compare
		/// current funds against the investment goal for the item
        /// when a user invests -> call on_campaign_fulfilled() -> if true call buy_item()
		/// 
		/// Checks:
		/// * TODO:
        /// 
        /// ### Requires:
        /// * `admin_badge`
		/// 
		/// ### Arguments:
		/// * shared_asset_badge (Bucket) - The badge representing the asset that will be bought
		/// 
		/// ### Returns:
		/// * `Bucket` - The updated badge that represents the shared asset
		pub fn add_to_inventory(&mut self, shared_asset_badge: Bucket) -> Bucket {

            // 1. extract original asset NFT ID from the shared_asset_badge NFT

            // Get the ID of the listed asset NFT
            let shared_asset_badge_id: NonFungibleId = shared_asset_badge.non_fungible::<SharedAsset>().id();
            
            //Get the non fungible data of the listed asset NFT, so we can update it
            let mut shared_asset_badge_non_fungible_data: SharedAsset = shared_asset_badge.non_fungible().data();
            
            // Retrieve the ID of the NFT of the actual asset
            let original_asset_id = shared_asset_badge_non_fungible_data.original_asset_id.clone();
            

            // 2. simulate buying the asset: 
            // TODO: How do we deal with non-NFT items?

            //For simulation purposes we'll mint an NFT with the ID of the original asset and add it to our inventory.
            // We then burn the funds in order to simulate payment to an external source.
            let inventory_resource_manager = borrow_resource_manager!(self.inventory_vault.resource_address());

            //FIXME: needs to be admin burnable (for simulation purposes)
            // Asset {} was included because the function was complaining. 
            // I don't know exactly which data should the asset contain 
            self.admin_badge.authorize(|| self.inventory_vault.put(
                    inventory_resource_manager.mint_non_fungible(&original_asset_id, Asset {})
                )
            );

            //Collect the funds saved to pay for the asset
            let mut collected_asset_funds_vault: Vault = self.collected_assets_funds.remove(&shared_asset_badge_id).unwrap();
            let collected_asset_funds_bucket: Bucket = collected_asset_funds_vault.take_all();


            //simulate payment to external source: burn collected funds for that asset
            self.admin_badge.authorize(|| collected_asset_funds_bucket.burn());

            //on success:
            shared_asset_badge_non_fungible_data.bought = true;
            
            // Then commit our updated data to our shared_asset_badge NFT
            self.admin_badge.authorize(|| shared_asset_badge.non_fungible().update_data(shared_asset_badge_non_fungible_data));

            shared_asset_badge
		}


		/// The platform sells an asset from the inventory and collects the funds.
		/// 
		/// Checks:
		/// * TODO:
		/// 
		/// ### Arguments:
		/// * `shared_asset_badge` (Bucket) - The original asset to be sold
		/// 
		/// ### Returns:
		/// * `Bucket` - The updated badge that represents the shared asset
		pub fn sell_item(&mut self, shared_asset_badge: Bucket) -> Bucket{

            //1. get the ID of the actual asset from the shared_asset_badge

            //Get the non fungible data part of the shared asset badge NFT
            let mut shared_asset_badge_non_fungible_data: SharedAsset = shared_asset_badge.non_fungible().data();
            
            // The NFT ID of the actual asset
            let original_asset_id = shared_asset_badge_non_fungible_data.original_asset_id.clone();


            //2. With that ID retrieve actual asset from inventory.
            let original_asset: Bucket = self.inventory_vault.take_non_fungible(&original_asset_id);


            //3. Simulate selling it: 
            // Burn the asset (sell to external source)
            self.admin_badge.authorize(|| original_asset.burn());

            // Collect some funds greater than the original price (investment_goal)
            // For simulation purposes, the item always sells for 110% of the original price
            
            // FIXME: how do we mint fungibles
            // let accquired_funds: Bucket = 

            //TODO:
            //we then have a bucket, take the funds out of the bucket and store them in the appropriate vault
            // (collected_assets_funds vault)


            //on success:
            shared_asset_badge_non_fungible_data.sold = true;

            // Then commit our updated data to our shared_asset_badge NFT
            self.admin_badge.authorize(|| shared_asset_badge.non_fungible().update_data(shared_asset_badge_non_fungible_data));

            shared_asset_badge
		}

        /// The investor retrieves their cut of the sold asset according to their ownership share
		/// 
		/// Checks:
		/// * **Check 1:** Check if the ID of the investor's asset ownership badge is a key of the collected_assets_funds
        /// HashMap
        /// //TODO:
		/// 
		/// ### Arguments:
		/// * `investor_asset_ownership_badge (Bucket) - The NFT representing the investor's share of the asset.
		/// 
		/// ### Returns:
		/// * `Bucket` - The investor's cut of the funds
        pub fn retrieve_funds(&mut self, investor_asset_ownership_badge: Bucket) -> Bucket {
            //1. Get investor's share value
            let mut investor_ownership_badge_data: InvestorAssetOwnershipBadge 
                = investor_asset_ownership_badge.non_fungible().data();
            let share = investor_ownership_badge_data.share;
            let shared_asset_badge_id = investor_ownership_badge_data.shared_asset_badge_id;
            
            //2. Calculate and retrieve amount owed from vault with the shared asset badge ID
            //let mut funds_vault: ResourceAddress = self.collected_assets_funds.get(&shared_asset_badge_id);

            //let investor_owed_funds: Bucket = funds_vault.take(share )

            //3. Burn the investor's ownership badge
            
            //investor_owed_funds

        }

    }

}
