use scrypto::prelude::*;
use sbor::{Decode, Describe, Encode, TypeId};

use super::authentication::*;
use super::decoder::*;
use super::voucher::*;

#[derive(TypeId, Describe, Encode, Decode)]
pub struct SealedVoucher {
    serialized: Vec<u8>,
    signature: Vec<u8>
}
impl SealedVoucher {
    pub fn unseal(&self, public_key: &EcdsaPublicKey) -> Voucher {
        verify(public_key, &self.serialized, &self.signature); // panics on failure
        private_decode_with_type(&self.serialized).unwrap()
    }
}

// QUESTION:  can i mint type-mismatched NonFungibleData all for the same ResourceDef?  Probably?

blueprint! {
    struct Transporter {
        resource_def: ResourceDef,
        mint_authority: Vault,
        burn_authority: Vault,
        count: u128,
        public_key: EcdsaPublicKey
    }

    impl Transporter {
        pub fn instantiate(public_key: EcdsaPublicKey) -> Component {
            let mint_authority = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .initial_supply_fungible(1);
            let burn_authority = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .initial_supply_fungible(1);

            let resource_def = ResourceBuilder::new_non_fungible()
                .flags(MINTABLE | BURNABLE)
                .badge(
                    mint_authority.resource_def(),
                    MAY_MINT
                )
                .badge(
                    burn_authority.resource_def(),
                    MAY_BURN
                )
                .no_initial_supply();

            Transporter::instantiate_with(public_key, resource_def, mint_authority, burn_authority)
        }

        // QUESTION: mut not needed across function boundaries?  Is that because of the macro?

        // burn_authority may be empty, then mint_authority will be used for both
        pub fn instantiate_with(public_key: EcdsaPublicKey, mut resource_def: ResourceDef, mint_authority: Bucket, burn_authority: Bucket) -> Component {
            assert_eq!(resource_def.resource_type(), ResourceType::NonFungible); // TODO for now only handle NF

            //mint and burn to check auth works, seperate burn auth is optional
            let key: NonFungibleKey = 0u128.into();
            mint_authority.authorize(|auth| {
                let default_nfd = PassThruNFD::default(); // TODO check this is not too degenerate, may need some data
                let minted = resource_def.mint_non_fungible(&key, default_nfd, auth.clone());
                if burn_authority.is_empty() {
                    resource_def.burn_with_auth(minted, auth)
                } else {
                    burn_authority.authorize(|auth|
                        resource_def.burn_with_auth(minted, auth)
                    );
                }
            });

            Self {
                resource_def,
                mint_authority: Vault::with_bucket(mint_authority),
                burn_authority: Vault::with_bucket(burn_authority),
                count: 0,
                public_key,
            }
            .instantiate()
        }

        // not public (pub would require Voucher to impl Decode which we don't want)
        fn voucher_redeem(&mut self, v: Voucher, optional_key: Option<NonFungibleKey>) -> Bucket {
            self.mint_authority.authorize(
                |auth| v.redeem(&self.resource_def, optional_key, auth)
            )
        }

        // public
        pub fn redeem_without_key(&mut self, sealed_voucher: SealedVoucher) -> Bucket {
            self.voucher_redeem(sealed_voucher.unseal(&self.public_key), None)
        }

        // public
        pub fn redeem_with_key(&mut self, sealed_voucher: SealedVoucher, key: NonFungibleKey) -> Bucket {
            self.voucher_redeem(sealed_voucher.unseal(&self.public_key), Some(key))
        }

        // public
        pub fn redeem(&mut self, sealed_voucher: SealedVoucher, optional_key: Option<NonFungibleKey>) -> Bucket {
            self.voucher_redeem(sealed_voucher.unseal(&self.public_key), optional_key)
        }

        // public
        pub fn redeem_next(&mut self, sealed_voucher: SealedVoucher) -> Bucket {
            self.count += 1;
            self.voucher_redeem(sealed_voucher.unseal(&self.public_key), Some(self.count.into()))
        }

        // not public (pub would require Voucher to impl Decode which we don't want)
        fn voucher_make(&mut self, bucket: Bucket) -> Voucher {
            assert_eq!(bucket.amount(), Decimal::one()); // TODO for now only handle single NFD
            assert_eq!(bucket.resource_def().resource_type(), ResourceType::NonFungible); // TODO for now only handle NF

            let mut resource_def = bucket.resource_def();
            assert_eq!(self.resource_def, resource_def);

            let nfds = bucket.get_non_fungibles::<PassThruNFD>();
            for entry in &nfds {
                let nfd: PassThruNFD = entry.data();
                let key: NonFungibleKey = entry.key();
                let key = Some(key);
           
                let authority = if self.burn_authority.is_empty() { &mut self.mint_authority } else { &mut self.burn_authority };

                authority.authorize(|auth|
                    resource_def.burn_with_auth(bucket, auth)
                );

                return Voucher {
                    resource_def,
                    key,
                    nfd,
                }
            };
            panic!("unreachable"); // asserted 1 in bucket
        }

        // make an unsigned voucher
        pub fn make(&mut self, bucket: Bucket) -> Vec<u8> {
            scrypto_encode(&self.voucher_make(bucket))
        }

    }
}
