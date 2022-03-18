use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct NftCollection{
    #[scrypto(mutable)]

    //nft generation, number of times it has been traded  
    generation: u8,
}

blueprint! {
    struct Hello {
        //XRD from initial sale of NFT are kept in this Vault
        collected_xrd:Vault,

        //NFT admin badge will be kept in a Vault
        nft_admin_badge:Vault,

        //Minted NFTs will be kept in this Vault until sold
        nft_collection_vault:Vault,

        //This vector keeps track of all NFT owners who sell their NFT
        nft_owners: Vec<Address>,

        //Sale price of minted NFTs
        nft_price: Decimal,

        //Royalties come from 10% of the swap price and are stored in this vault
        nft_royalty_vault: Vault,

        //Resource definition of a nft share
        nft_shares_def: ResourceDef,

    }

    impl Hello {
        
        pub fn new() -> Component {

            //Creating admin badge to mint and burn nft shares and increment mutable nft metadata
            let nft_admin_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Nft Admin Badge")
                .initial_supply_fungible(1);
            
            //Creating nft shares that are mintable and burnable
            let nft_shares_def: ResourceDef = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "NFT Shares")
                .metadata("symbol", "nfts")
                .flags(BURNABLE | MINTABLE)
                .badge(nft_admin_badge.resource_address(), MAY_BURN | MAY_MINT)
                .no_initial_supply();

            // Creates a fixed set of NFTs 
            let nft_collection :Bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "NFT Collection")
                .flags(INDIVIDUAL_METADATA_MUTABLE)
                .badge(nft_admin_badge.resource_address(),
                    MAY_CHANGE_INDIVIDUAL_METADATA)
                .initial_supply_non_fungible([
                    (
                        NonFungibleKey::from(1u128),
                        NftCollection {
                            generation: 0,
                        },
                    ),
                    (
                        NonFungibleKey::from(2u128),
                        NftCollection {
                            generation: 0,
                        },
                    ),
                    (
                        NonFungibleKey::from(3u128),
                        NftCollection {
                            generation: 0,
                        },
                    ),
                ]);

                  
            let component:Component = Self {
                collected_xrd: Vault::new(RADIX_TOKEN),
                nft_admin_badge:Vault::with_bucket(nft_admin_badge),
                nft_collection_vault:Vault::with_bucket(nft_collection),
                nft_owners:Vec::new(),
                nft_price: 1000.into(),
                nft_royalty_vault:Vault::new(RADIX_TOKEN),
                nft_shares_def: nft_shares_def,
            } 
            
            .instantiate();
            
            return component;   
        }

        pub fn buy_nft(&mut self, key:NonFungibleKey, mut payment:Bucket) -> (Bucket, Bucket) {
            //take the cost of nft out of payment Bucket
            self.collected_xrd.put(payment.take(self.nft_price));

            //takes the nft by id out of the vault and puts it in a Bucket
            let nft_bucket = self.nft_collection_vault.take_non_fungible(&key);

            //Return 2 Buckets with nft and any remaining change
            (nft_bucket, payment)
        }

        pub fn increment_nft (&mut self, key:NonFungibleKey) {

            //accessing NFTCollection struct mutable data
            let mut non_fungible_data: NftCollection = self.nft_collection_vault.get_non_fungible_data(&key);
            
            //increments the generation data by 1
            non_fungible_data.generation += 1;

            //updates the nft data by authorizing the nft admin badge located its vault in the component 
            self.nft_admin_badge.authorize(|auth|self.nft_collection_vault.update_non_fungible_data(&key, non_fungible_data, auth));
            
        }

        pub fn mint_nft_shares(&mut self, sellers_address: Address, mut buyers_funds:Bucket) ->Bucket {
            
            //adds sellers address to vector
            self.nft_owners.push(sellers_address);

            //finds current length of vector
            let total_number_accounts: usize = self.nft_owners.len();

            //finds the Decimal amount in the Bucket argument takes 10%
            let royalty_amount: Decimal = buyers_funds.amount() / 10;

            //takes the 10% royalty and puts it in the nft royalty vault
            self.nft_royalty_vault.put(buyers_funds.take(royalty_amount));

            //Take the 10% tax and divides it over total # of accounts in vector
            let distribution_amount: Decimal = royalty_amount / total_number_accounts;

            //iterates through each account in the vector
            for i in 0..total_number_accounts {

                //minting nft shares 1:1 ratio nft shares : XRD
                let shares_bucket: Bucket = self.nft_admin_badge.authorize(|auth| {self.nft_shares_def.mint(distribution_amount, auth)});
                
                //gets address from vector 
                let address:Address = self.nft_owners[i];

                //sends share distribution amount to each address
                Component::from(address).call::<()>("deposit", vec![scrypto_encode(&shares_bucket)]);
            }
            
            //returns the remaining funds after taking the 10% royalty amount
            return buyers_funds;
        }

        pub fn withdraw_xrd(&mut self, nft_shares:Bucket ) -> Bucket {

            //Finds the Decimal amount of nft shares in Bucket argument
            let number_of_shares: Decimal = nft_shares.amount();

            //Burns the shares nft shares in the bucket
            self.nft_admin_badge.authorize(|auth| {self.nft_shares_def.burn_with_auth(nft_shares, auth)});
            
            //Takes the 1:1 equivalant nft shares : XRD from vault 
            self.nft_royalty_vault.take(number_of_shares)
        } 
    }
}