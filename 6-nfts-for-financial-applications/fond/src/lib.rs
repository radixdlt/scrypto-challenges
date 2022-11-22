use scrypto::prelude::*;
//use rand::Rng;



#[derive(NonFungibleData)]
pub struct Asset {

}

#[derive(NonFungibleData)]
pub struct InvestorAssetOwnershipBadge {
    
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

        dead_vaults: Vec<Vault>,

        mock_funds: Vault
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
        pub fn instantiate_fond(admin_funds_bucket: Bucket) -> ComponentAddress {
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
                dead_vaults: Vec::new(),
                mock_funds: Vault::with_bucket(admin_funds_bucket)
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

            let new_shared_asset_badge_id = NonFungibleId::random(); 

            self.current_campaigns_vault.put(
                campaign_resource_manager
                    .mint_non_fungible(&new_shared_asset_badge_id, asset),
            );

            self.collected_assets_funds.insert(new_shared_asset_badge_id, Vault::new(RADIX_TOKEN));

  

            ComponentAuthZone::pop().drop();
   

            //self.current_campaigns_vault.take_all()
        }





        pub fn invest_in_campaigns(
            &mut self,
            /*mut customer_account: Bucket,
            amount: Decimal,
            campaign_id:usize,
            customer_address: String,*/

            payment: Bucket,
            shared_asset_badge_id: NonFungibleId
        ) ->  Bucket /*(Bucket,Bucket)*/ {
            //let resource_manager = borrow_resource_manager!(self.current_campaigns_address);
            //let campaign_nft_id = &self.campaigns[campaign_id];
            //let campaign_nft_data: Campaign = resource_manager.get_non_fungible_data(campaign_nft_id);


            //let shared_asset_badge: Bucket =  self.current_campaigns_vault.take_non_fungible(&shared_asset_badge_id);


            self.collected_assets_funds.get_mut(&shared_asset_badge_id).unwrap().put(payment);

            // fix this return types issue
            /*if campaign_nft_data.fullfilled {
                let x: Todo = Todo {
                    fix: dec!(0),
                };
                let nft = ResourceBuilder::new_non_fungible()
                .metadata("TODO", "fix workaround :)")
                .initial_supply([
                    (
                        NonFungibleId::from_u32(1),
                        x,

                    )]);
                return (nft,customer_account)
            }*/

            /*
            let campaign_vault = self.campaign_vaults.get_mut(&campaign_id).unwrap();


            let amount_possible_to_invest = campaign_nft_data.investment_goal - campaign_vault.amount();


            let difference = amount_possible_to_invest - amount;

            let amount_customer_can_invest = if difference <  dec!(0) {amount + difference } else {amount};


            let balance = customer_account.take(amount_customer_can_invest);
            campaign_vault.put(balance);

            let mut new_ids_of_contributors = campaign_nft_data.ids_of_contributors.clone();

            new_ids_of_contributors.insert(customer_address);

            if campaign_vault.amount() + amount  >= campaign_nft_data.investment_goal {
                // set fulfilled, need to find spread operator
                resource_manager.update_non_fungible_data(campaign_nft_id, Campaign {
                    fullfilled: true,
                    asset_name: campaign_nft_data.asset_name,
                    asset_description: campaign_nft_data.asset_description,
                    investment_goal: campaign_nft_data.investment_goal,
                    campaign_vault_address: campaign_nft_data.campaign_vault_address,
                    active: campaign_nft_data.active,
                    ids_of_contributors: new_ids_of_contributors
                });
            }
            else {
                resource_manager.update_non_fungible_data(campaign_nft_id, Campaign {
                    fullfilled: campaign_nft_data.fullfilled,
                    asset_name: campaign_nft_data.asset_name,
                    asset_description: campaign_nft_data.asset_description,
                    investment_goal: campaign_nft_data.investment_goal,
                    campaign_vault_address: campaign_nft_data.campaign_vault_address,
                    active: campaign_nft_data.active,
                    ids_of_contributors: new_ids_of_contributors
                });
            }


            let amount_data: Contribution = Contribution {
                amount: amount,
                campaign_id: campaign_id
            };
            let shareNFT = ResourceBuilder::new_non_fungible()
                .metadata("name", "amount-paid")
                .initial_supply([
                    (
                        NonFungibleId::from_u32(1),
                        amount_data,

                    )]);
            return (shareNFT,customer_account)
            */
            let investor_ownership_badge_data = InvestorAssetOwnershipBadge {
                //FIXME: this is only for testing purps (should be payment / investment_goal)
                share: dec!(1),
                shared_asset_badge_id: shared_asset_badge_id
            };
            return ResourceBuilder::new_non_fungible()
                .metadata("name", "Share of ownership of asset")
                .initial_supply([
                    (
                        NonFungibleId::random(),
                        investor_ownership_badge_data
                    )
                ]);
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
		pub fn add_to_inventory(&mut self, shared_asset_badge_id: NonFungibleId) -> Bucket {

            //TODO: check Proof
            /*let shared_asset_badge: ValidatedProof = shared_asset_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.shareholder_badge_resource_address,
                    dec!("1"),
                ))
                .expect("[Withdraw by Amount]: Invalid badge resource address or amount");

            */

            // 1. extract original asset NFT ID from the shared_asset_badge NFT

            // Get the ID of the listed asset NFT
            //let shared_asset_badge_id: NonFungibleId = shared_asset_badge.non_fungible::<SharedAsset>().id();
            let shared_asset_badge: Bucket =  self.current_campaigns_vault.take_non_fungible(&shared_asset_badge_id);

            
            //Get the non fungible data of the listed asset NFT, so we can update it
            //let mut shared_asset_badge_non_fungible_data: SharedAsset = shared_asset_badge.non_fungible().data();
            
            // Retrieve the ID of the NFT of the actual asset
            //let original_asset_id = shared_asset_badge_non_fungible_data.original_asset_id.clone();
            

            // 2. simulate buying the asset: 
            // TODO: How do we deal with non-NFT items?

            //For simulation purposes we'll mint an NFT with the ID of the original asset and add it to our inventory.
            // We then burn the funds in order to simulate payment to an external source.
            //let inventory_resource_manager = borrow_resource_manager!(self.inventory_vault.resource_address());

            //FIXME: needs to be admin burnable (for simulation purposes)
            // Asset {} was included because the function was complaining. 
            // I don't know exactly which data should the asset contain 
            /*self.admin_badge.authorize(|| self.inventory_vault.put(
                    inventory_resource_manager.mint_non_fungible(&original_asset_id, Asset {})
                )
            );*/

            //Collect the funds saved to pay for the asset
            /*let collected_asset_funds_bucket: Bucket = 
                self.collected_assets_funds
                .get_mut(&shared_asset_badge_id)
                .unwrap()
                .take_all();
            */
            
            //let collected_asset_funds_bucket: Bucket = collected_asset_funds_vault.take_all();


            //simulate payment to external source: store funds in mock funds vault
            //self.mock_funds.put(collected_asset_funds_bucket);

            //on success:
            //shared_asset_badge_non_fungible_data.bought = true;
            
            // Then commit our updated data to our shared_asset_badge NFT
            //self.admin_badge.authorize(|| shared_asset_badge.non_fungible().update_data(shared_asset_badge_non_fungible_data));

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
            
            let shared_asset_badge_id: NonFungibleId = shared_asset_badge.non_fungible::<SharedAsset>().id();


            // The NFT ID of the actual asset
            let original_asset_id = shared_asset_badge_non_fungible_data.original_asset_id.clone();


            //2. With that ID retrieve actual asset from inventory.
            let original_asset: Bucket = self.inventory_vault.take_non_fungible(&original_asset_id);


            //3. Simulate selling it: 
            // Burn the asset (sell to external source)
            self.admin_badge.authorize(|| original_asset.burn());

            // Collect some funds greater than the original price (investment_goal)
            // For simulation purposes, the item always sells for 5-12% more of the original price (random)
            
            let original_price = shared_asset_badge_non_fungible_data.investment_goal.clone();

            //calculate 5-12% of the original price and retrieve funds from mock_funds vault
            //we then have a bucket, take the funds out of the bucket and store them in the appropriate vault
            // (collected_assets_funds vault)
            
            //FIXME: this is not working, so I'll just change it to 10% for now
            //let mut rng = rand::thread_rng();
            //let generated_percentage = rng.gen_range(5..12);
            let generated_percentage = 10;
            
            let simulated_return = original_price + (original_price * (generated_percentage / 100));

            let acquired_funds: Bucket = self.mock_funds.take(simulated_return);
            

            
            let mut asset_funds_vault: Vault = self.collected_assets_funds.remove(&shared_asset_badge_id).unwrap();
            asset_funds_vault.put(acquired_funds);
            self.collected_assets_funds.insert(shared_asset_badge_id, asset_funds_vault);


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
            let investor_ownership_badge_data: InvestorAssetOwnershipBadge 
                = investor_asset_ownership_badge.non_fungible().data();
            let share = investor_ownership_badge_data.share;
            let shared_asset_badge_id = investor_ownership_badge_data.shared_asset_badge_id.clone();
            
            //2. Get shared asset badge and retrieve non-fungible data
            let shared_asset_badge: Bucket = self.current_campaigns_vault.take_non_fungible(&shared_asset_badge_id);
            let shared_asset_badge_non_fungible_data: SharedAsset = shared_asset_badge.non_fungible().data();
            
            let original_price: Decimal = shared_asset_badge_non_fungible_data.investment_goal.clone();

            //3. Calculate and retrieve amount owed from vault with the shared asset badge ID

            let investor_owed_funds: Bucket = self.collected_assets_funds
                .get_mut(&shared_asset_badge_id)
                .unwrap()
                .take(share * original_price);            

            //4. Burn the investor's ownership badge
            self.admin_badge.authorize(|| investor_asset_ownership_badge.burn());
            
            investor_owed_funds
        }

    }

}
