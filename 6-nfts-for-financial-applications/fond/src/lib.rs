use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct Campaign {
    asset_name: String,
    asset_description: String,
    investment_goal: Decimal,
    campaign_vault_address: ResourceAddress,
    active: bool, //accepted_token_address: ResourceAddress
                  // mut owners?
}

#[derive(Debug, NonFungibleData)]
pub struct Contribution {
    /// The amount of tokens which this party needs to pay to the other party.
    amount: Decimal,
    campaign_address: ResourceAddress
}

blueprint! {
    struct Fond {
        admin_badge: Vault,
        current_campaigns_address: ResourceAddress,
        // current_campaigns_vault: Vault,
        inventory_vault: Vault,
        campaign_vaults:  HashMap<ResourceAddress, Vault>,
        campaigns: Vec<NonFungibleId>
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

            // let current_campaigns_bucket = Bucket::new(current_campaigns_address);


            let inventory_address = ResourceBuilder::new_non_fungible()
                .metadata("name", "Inventory of company")
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                // any one can update the data so that investment can be done
                .updateable_non_fungible_data(rule!(allow_all), LOCKED)
                .no_initial_supply();

            let inventory_bucket = Bucket::new(inventory_address);

            // let current_campaigns_vault = Vault::with_bucket(current_campaigns_bucket);
            let campaigns = Vec::new();
            // campaigns.push(current_campaigns_vault);

            Self {
                campaigns: campaigns,
                admin_badge: Vault::with_bucket(admin_badge),
                current_campaigns_address:current_campaigns_address,
                // current_campaigns_vault:  Vault::with_bucket(current_campaigns_bucket),
                inventory_vault: Vault::with_bucket(inventory_bucket),
                campaign_vaults: HashMap::new()
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
            investment_goal: Decimal,
        ) ->Bucket {


            // let current_campaigns_address = ResourceBuilder::new_non_fungible()
            // .metadata("name", "Current Campaigns of company")
            // .mintable(rule!(require(self.admin_badge.resource_address())), LOCKED)
            // .updateable_non_fungible_data(rule!(require(self.admin_badge.resource_address())), LOCKED)
            // .no_initial_supply();

            // let current_campaigns_bucket = Bucket::new(current_campaigns_address);
            // let current_campaigns_vault = Vault::with_bucket(current_campaigns_bucket);




            ComponentAuthZone::push(self.admin_badge.create_proof());
            
            let campaign_resource_manager = borrow_resource_manager!(self.current_campaigns_address);
            // do we need to put admin on this vault??
            let campaign_vault = Vault::with_bucket(Bucket::new(RADIX_TOKEN));
            let asset = Campaign {
                asset_name: asset_name,
                asset_description: asset_description,
                investment_goal: investment_goal,
                campaign_vault_address: campaign_vault.resource_address(),
                active: true
            };
            let id = &NonFungibleId::random();
            let nft = campaign_resource_manager.mint_non_fungible(id, asset);
            self.campaigns.push( 
                nft.non_fungible_id()
            );
            // self.current_campaigns_vault.put(
            //     campaign_resource_manager
            //         .mint_non_fungible(id, asset),
            // );
            ComponentAuthZone::pop().drop();

            self.campaign_vaults.insert(campaign_vault.resource_address(),campaign_vault);
            // let campaigns = self.campaigns;
            return nft
        }

        //
        // get campaign by nftbyID
        // take users payment from their bucket and put into campaign vault
        // return user an NFT which has respective amount of their investment
        //
        pub fn invest_in_campaigns(
            &mut self,
            mut customer_account: Bucket,
            amount: Decimal,
            campaign_id:usize,
        )-> (Bucket,Bucket){

            let campaign_nft_data: Campaign = borrow_resource_manager!(self.current_campaigns_address).get_non_fungible_data(&self.campaigns[campaign_id]);
            
            let campaign_vault_address = campaign_nft_data.campaign_vault_address;

            let campaign_vault = self.campaign_vaults.get_mut(&campaign_vault_address).unwrap();

            campaign_vault.put(customer_account.take(amount));

            let amount_data: Contribution = Contribution {
                amount: amount,
                campaign_address: campaign_vault_address
            };
            
            let shareNFT = ResourceBuilder::new_non_fungible()
                .metadata("name", "amount-paid")
                .initial_supply([
                    (
                        NonFungibleId::from_u32(1),
                        amount_data,

                    )]);
            return (shareNFT,customer_account)
        }



    }

}
