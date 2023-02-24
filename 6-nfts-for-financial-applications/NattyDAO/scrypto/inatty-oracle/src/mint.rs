use scrypto::prelude::*;

// An NFT that will be minted each time there's a captured nature experience.
// It will then be listed for sale in a marketplace.
// 50% of the proceeds will be sent to the minter, and 50% to the DAO.
// These observation NFTs are purchased only with INATTY token. 
// That's what backs the token with relationships to nature.

// The relay needs to contain the private key to instantiate this component?
// Or the relay just needs to be able to call the mint function? (a mint badge)
// The relay will then be able to mint the NFTs and list them for sale.
// The relay gets triggered when new observations are made.

#[derive(NonFungibleData)]
pub struct ObservationData {
    pub id: String,
    pub date: String,
    pub location: String,
    pub user: String,
    pub image_url: String,
    pub species: String,
    pub description: String
}

blueprint! {

    struct INattyOracle {
        // Define what resources and data will be managed by INattyOracle component
        mint_badge: Vault,
        nft_resource_address: ResourceAddress
    }

    impl INattyOracle {
        
        // This is a function, and can be called directly on the blueprint once deployed
        // This returns a component address (whoever instantiates will receive an admin badge.)
        pub fn instantiate_oracle() -> (ComponentAddress, Bucket) {

            let mint_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Nature Experience Mint Badge")
                .initial_supply(1);

            let nft_resource_address = ResourceBuilder::new_non_fungible()
                .metadata("name", "INatty Nature Experience")
                .mintable(rule!(require(mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(mint_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(mint_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Access rule (for mint badge to call create_nft)
            let access_rule = AccessRules::new()
                .default(rule!(require(mint_badge.resource_address()))); // by default, every method requires mint badge

            // Instantiate a new INattyOracle component
            let mut mint_component = Self {
                mint_badge: Vault::new(mint_badge.resource_address()), // stays within component (as opposed to admin_badge which is used to access component)
                nft_resource_address
            }
            .instantiate();
            mint_component.add_access_check(access_rule);

            (mint_component.globalize(), mint_badge)
        }

        // Notes from Jake:
        // Need to have a Bucket as an output to create_nft
        // Could get a resource address of the bucket (which will stay the same)
        // Every NFT for a component will have a unique Non-fungible ID, but have 1 resourceaddress (collection id)
        // Once you instantiate component, you receive an admin badge.
        // Implement an access rule 

        // This will be called from the mint_manifest.rtm
        pub fn create_nft(&mut self, id: String, date: String, location: String, user: String, image_url: String, species: String) -> ResourceAddress {
            
            let d = format!("{} observed on {}", species, date);
            let idcopy = format!("{}", id);

            let nft_data = ObservationData {
                id: idcopy,
                date,
                location,
                user,
                image_url,
                species,
                description: d,
            };

            // convert id (which is a number) to u64
            let idcopy = format!("{}", id);
            let id_u64 = idcopy.parse::<u64>().unwrap();

            // Goes into mint badge vault, and authorizing (creating a proof of that badge),
            // Putting that into the local component's authorization zone. Which allows resource manager to do the updates.
            let nft = self.mint_badge.authorize(|| {
                let resource_manager = borrow_resource_manager!(self.nft_resource_address);
                resource_manager.mint_non_fungible(
                    // The NFT id
                    &NonFungibleId::from_u64(id_u64),
                    // The NFT data
                    nft_data,
                )
            });

            nft.resource_address()
            
        }

        // Method to send NFTs to the marketplace
        // They become for sale, and the proceeds are split between the minter and the DAO
        // pub fn list_on_marketplace(&mut self, nft: ResourceAddress) {
            
        // }

        // Update method, where the relay has access to account,
        // pulls a proof of the badge, and then they have update authority
        // this could be if the species changed due to curation
        // pub fn update_nft() {

        // }
    }
}