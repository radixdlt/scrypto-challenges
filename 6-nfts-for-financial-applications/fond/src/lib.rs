use scrypto::prelude::*;
//use rand::Rng;



#[derive(NonFungibleData)]
pub struct Asset {
    
}

#[derive(NonFungibleData)]
pub struct InvestorContribution {
    amount_contributed: Decimal, 
    share: Decimal,
    campaign_id: usize
}

#[derive(Encode, Decode, TypeId, Describe)] 
pub struct Campaign {

	asset_name: String,
	asset_description: String,
	investment_goal: Decimal,

    original_asset_id: NonFungibleId,

    bought: bool,
    fulfilled: bool,
    sold: bool,

	//accepted_token_address: ResourceAddress

    collected_funds: Vault

}


blueprint! {
    struct Fond {
        admin_badge: Vault,
        
        //current_campaigns_vault: Vault,

        inventory_vault: Vault,
        current_campaigns: HashMap<usize, Campaign>,
        
        //Total funds collected per asset, both during investment phase as well as when the item's been sold. 
        // K = (NonFungibleId) The ID of the shared_asset_badge
        // V = (Vault) A vault containing all funds respective to the asset
        //collected_assets_funds: HashMap<NonFungibleId, Vault>,
        
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

            let inventory_address = ResourceBuilder::new_non_fungible()
                .metadata("name", "Inventory of company")
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let inventory_bucket = Bucket::new(inventory_address);

            Self {
                admin_badge: Vault::with_bucket(admin_badge),
                //current_campaigns_vault:  Vault::with_bucket(current_campaigns_bucket),
                inventory_vault: Vault::with_bucket(inventory_bucket),
                //collected_assets_funds: HashMap::new(),
                current_campaigns: HashMap::new(),
                dead_vaults: Vec::new(),
                mock_funds: Vault::with_bucket(admin_funds_bucket)
            }
            .instantiate()
            .globalize()
        } 
        //TODO: update return to give admin badge

        
        pub fn create_campaign(
            &mut self,
            asset_name: String,
            asset_description: String,
            investment_goal: Decimal,
        )  {
             //let campaign_resource_manager = borrow_resource_manager!(self.current_campaigns_address);
            //let campaign_vault = Vault::with_bucket(Bucket::new(RADIX_TOKEN));
            
            //TODO: access rules: only allow admin to instantiate a Campaign

            let new_campaign = Campaign {
                asset_name: asset_name,
                asset_description: asset_description,
                investment_goal: investment_goal,

                original_asset_id: NonFungibleId::random(),

                bought:false,
                sold: false,
                fulfilled:false,

                collected_funds: Vault::new(RADIX_TOKEN)

            };

            self.current_campaigns.insert(self.current_campaigns.len(), new_campaign);
            
        }

   
 
        pub fn invest_in_campaigns(
            &mut self,
            mut investment: Bucket,
            campaign_id:usize,
        ) -> (Bucket, Bucket) {
            let mut campaign_data: &mut Campaign = self.current_campaigns.get_mut(&campaign_id).unwrap();

            // TODO: validation if fullfilled - return 

            let campaign_vault = &mut campaign_data.collected_funds;
            let amount_possible_to_invest = campaign_data.investment_goal - campaign_vault.amount();
            let investment_amount = investment.amount();

            let difference = amount_possible_to_invest - investment_amount;
            
            let amount_customer_can_invest = if difference <  dec!(0) {investment_amount + difference } else {investment_amount};

            let balance: Bucket = investment.take(amount_customer_can_invest);
            campaign_vault.put(balance);

            let investor_contribution_data: InvestorContribution = InvestorContribution {
                amount_contributed: amount_customer_can_invest,
                share: amount_customer_can_invest / campaign_data.investment_goal, 
                campaign_id: campaign_id
            };

            if campaign_vault.amount() + investment_amount  >= campaign_data.investment_goal {
                // set fulfilled, need to find spread operator
                campaign_data.fulfilled = true;
            }

            let shareNFT = ResourceBuilder::new_non_fungible()
                .metadata("name", "Share of ownership of asset")
                .initial_supply([
                    (
                        NonFungibleId::random(),
                        investor_contribution_data,

                    )]);
            return (shareNFT, investment)
        }







         
	/*
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

     */

     /*
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
*/
        
 /*
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
*/
    
    //}
    }
}
