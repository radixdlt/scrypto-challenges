use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct Campaign {
    asset_name: String,
    asset_description: String,
    investment_goal: Decimal,
    campaign_vault_address: ResourceAddress,
    active: bool,
    fullfilled: bool,
    ids_of_contributors: HashSet<String>,
}

#[derive(Debug, NonFungibleData)]
pub struct Contribution {
    /// The amount of tokens which this party needs to pay to the other party.
    amount: Decimal,
    campaign_id: usize,
}

#[derive(Debug, NonFungibleData)]
pub struct Todo {
    fix: Decimal,
}

#[derive(Debug, NonFungibleData)]
pub struct Inventory {
    campaign_id: usize,
    name: String,
    description: String,
    price: Decimal,
}

blueprint! {
    struct Fond {
        admin_badge: Vault,
        current_campaigns_address: ResourceAddress,
        inventory_vault: Vault,
        campaign_vaults:  HashMap<usize, Vault>,
        campaigns: Vec<NonFungibleId>,
        inventory_address:ResourceAddress,
        item_vaults: HashMap<usize, Vault>,
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
            // should change this to allow anyone to update mutable data only.
            .updateable_non_fungible_data(rule!(allow_all), LOCKED)
            .no_initial_supply();

            // let current_campaigns_bucket = Bucket::new(current_campaigns_address);

            let inventory_address = ResourceBuilder::new_non_fungible()
                .metadata("name", "Inventory of company")
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)

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
                inventory_address:inventory_address,
                campaign_vaults: HashMap::new(),
                item_vaults: HashMap::new(),
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

            ComponentAuthZone::push(self.admin_badge.create_proof());
            let campaign_resource_manager = borrow_resource_manager!(self.current_campaigns_address);
            let campaign_vault = Vault::with_bucket(Bucket::new(RADIX_TOKEN));
            let asset = Campaign {
                asset_name: asset_name,
                asset_description: asset_description,
                investment_goal: investment_goal,
                campaign_vault_address: campaign_vault.resource_address(),
                fullfilled:false,
                active: true,
                ids_of_contributors: HashSet::new()
            };
            let id = &NonFungibleId::random();
            let nft = campaign_resource_manager.mint_non_fungible(id, asset);
            self.campaigns.push(
                nft.non_fungible_id()
            );
            ComponentAuthZone::pop().drop();

            self.campaign_vaults.insert(self.campaigns.len()-1,campaign_vault);
            return nft
        }

        pub fn invest_in_campaigns(
            &mut self,
            mut customer_account: Bucket,
            amount: Decimal,
            campaign_id:usize,
            customer_address: String
        )-> (Bucket,Bucket) {
            let resource_manager = borrow_resource_manager!(self.current_campaigns_address);
            let campaign_nft_id = &self.campaigns[campaign_id];
            let campaign_nft_data: Campaign = resource_manager.get_non_fungible_data(campaign_nft_id);

            // fix this return types issue
            if campaign_nft_data.fullfilled {
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
            }

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
        }


        // when a campaign is fulfilled, this function will hit the market and buy the item
        // for now, it just means the admin mints an nft with that value embedded into it.
        // it should also take the money out of the respective campaign vault.
        // pub fn buy_item(& mut self, campaign_id:usize, seller_address:ComponentAddress){
        //     let resource_manager = borrow_resource_manager!(self.current_campaigns_address);
        //     let campaign_nft_id = &self.campaigns[campaign_id];
        //     let campaign_nft_data: Campaign = resource_manager.get_non_fungible_data(campaign_nft_id);

            
            
        //     ComponentAuthZone::push(self.admin_badge.create_proof());

        //     let inventory_resource_manager = borrow_resource_manager!(self.inventory_address);       
        //     let item = Inventory {
        //         campaign_id: campaign_id,
        //         name: campaign_nft_data.asset_name,
        //         description: campaign_nft_data.asset_description,
        //         price: campaign_nft_data.investment_goal
        //     };

        //     let id = &NonFungibleId::random();
        //     let nft = inventory_resource_manager.mint_non_fungible(id, item);
        //     let inventory_vault = Vault::with_bucket(nft);

        //     ComponentAuthZone::pop().drop();

        //     self.item_vaults.insert(campaign_id,inventory_vault);

            // =======
            // handle mocking payment
            // =======
            // let campaign_vault = self.campaign_vaults.get_mut(&campaign_id).unwrap();
            // let payment_bucket = campaign_vault.take_all();
            // let comp = borrow_component!(seller_address);

            // comp.
        // }


        // This should
        // fn sell_item(item_to_buy:String, amount:Decimal, vault_address:ResourceAddress){

        // }

    }

}
