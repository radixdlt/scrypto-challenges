use scrypto::prelude::*;

#[derive(NonFungibleData)]
struct NftData {
    #[scrypto(mutable)]
    data: Decimal
}

blueprint! {
    struct Ddex {
       admin_def: ResourceDef,
       nft_def: ResourceDef,
       data: Decimal,
       ratio: Decimal,
       pool: Vault,
       auth: Vault
    }

    impl Ddex {
      
        pub fn new() -> (Component, Bucket) {
                let admin: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                    .metadata("name", "Admin NFDDex")
                    .initial_supply_fungible(1);

                let auth: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                    .metadata("name", "Auth mint/metadata NFDDex")
                    .initial_supply_fungible(1);

                let nft: ResourceDef = ResourceBuilder::new_non_fungible()
                    .metadata("name", "CO2")
                    .flags(MINTABLE | INDIVIDUAL_METADATA_MUTABLE)
                    .badge(auth.resource_def(), MAY_MINT|MAY_CHANGE_INDIVIDUAL_METADATA)
                    .no_initial_supply();


        let comp = Self {
                admin_def: admin.resource_def(),
                nft_def: nft,
                data: Decimal::zero(),
                ratio: Decimal::one(),
                pool: Vault::new(RADIX_TOKEN),
                auth: Vault::with_bucket(auth)
            }
            .instantiate();

        (comp, admin)
        }

        pub fn mint(&mut self) -> Bucket {
            self.auth.authorize(|auth| {
                self.nft_def.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), NftData{ data: Decimal::zero() }, auth)
            })
        }

        #[auth(admin_def)]
        pub fn data_transfer(&mut self, data: Decimal) {
            self.data += data;
        }

        #[auth(admin_def)]
        pub fn ratio(&mut self, ratio: Decimal) {
            self.ratio = ratio;
        }

        pub fn buy(&mut self, xrd: Bucket, nft: BucketRef) {
            // We check that the definition matches
            assert_eq!(nft.resource_def(), self.nft_def, "Nft type mismatch");
            // Take the data from the nft
            let mut data_nft: NftData = self.nft_def.get_non_fungible_data(&nft.get_non_fungible_key());
            // Calculate the ratio
            let amount_data: Decimal = xrd.amount() / self.ratio;
            // Update nft data
            data_nft.data += amount_data;
            self.auth.authorize(|auth|self.nft_def.update_non_fungible_data(&nft.get_non_fungible_key(), data_nft, auth));
            // Update Struct data
            self.data -= amount_data;
            self.pool.put(xrd);
        }

        pub fn sell(&mut self, data: Decimal, nft: BucketRef) -> Bucket {
            // We check that the definition matches
            assert_eq!(nft.resource_def(), self.nft_def, "Nft type mismatch");
            // Take the data from the nft
            let mut data_nft: NftData = self.nft_def.get_non_fungible_data(&nft.get_non_fungible_key());
            // Calculate the ratio
            let amount_data: Decimal = self.ratio * data;
              // Update nft data
            data_nft.data -= amount_data;
            self.auth.authorize(|auth|self.nft_def.update_non_fungible_data(&nft.get_non_fungible_key(), data_nft, auth));
            // Update Struct data
            self.data += amount_data;
            self.pool.take(amount_data)
        }
        
    }
}

