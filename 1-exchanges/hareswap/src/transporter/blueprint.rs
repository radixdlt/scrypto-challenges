//! The Transporter blueprint redeems SealedVouchers for NonFungibles
//! effectively moving assets onto the ledger (bringing them into existance)
//!
//! Transporter utilizes the Voucher implemenation for most of the hard work.
//!
//! # LIMITATIONS
//!
//! This prototype only supports NonFungibles and a single one per Voucher.  It
//! wouldn't be much harder to update this (and the Voucher type) to support an
//! entire arbitrary Bucket
use scrypto::prelude::*;

use super::voucher::*;

/// Used to verify the presented mint/burn authority badges work propertly as a safeguard
#[derive(NonFungibleData)]
struct AuthTestData {}

blueprint! {

    /// Contract storage, see `instantiate_with` for details
    struct Transporter {
        resource_def: ResourceDef,
        mint_authority: Vault,
        burn_authority: Vault,
        count: u128,
        public_key: EcdsaPublicKey,
        redeem_auth: ResourceDef, // avoid duplicates/frontrunning
    }

    impl Transporter {

        /// Create a Transporter with sane defaults when the resource being Transported does not already exist.  Wraps `instantiate_with`
        pub fn instantiate(public_key: EcdsaPublicKey, redeem_auth: ResourceDef) -> Component {
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

            Transporter::instantiate_with(public_key, resource_def, mint_authority, burn_authority, redeem_auth)
        }

        /// Create a Transporter which can only be called by those with redeem_auth and can mint/burn resource_def using the specified authorities for SealedVouchers verified with public_key.
        ///
        /// burn_authority may be empty, then mint_authority will be used for both
        ///
        /// # LIMITATIONS
        ///
        /// assumes the resource_def does not have an existing NonFungible with key == 0u128 since that is used in the mint/burn auth test.
        pub fn instantiate_with(public_key: EcdsaPublicKey, mut resource_def: ResourceDef, mint_authority: Bucket, burn_authority: Bucket, redeem_auth: ResourceDef) -> Component {
            assert_eq!(resource_def.resource_type(), ResourceType::NonFungible, "Transporter::instantiate_with: only supports transportation of NonFungibles (for now)");

            // mint and burn to check auth works, seperate burn auth is optional
            //
            // NOTE: This proves that the NonFungibleData content need not have the same schema/structure for all NonFungibleKey's of a given resource.  This is kinda neat and not something I'd considered.
            let minted = mint_authority.authorize(|auth| {
                let test_nfd = AuthTestData {};
                let test_key: NonFungibleKey = 0u128.into();
                let minted = resource_def.mint_non_fungible(&test_key, test_nfd, auth);
                minted
            });

            if burn_authority.is_empty() {
                mint_authority.authorize(|auth| {
                    resource_def.burn_with_auth(minted, auth);
                });
            } else {
                burn_authority.authorize(|auth| {
                    resource_def.burn_with_auth(minted, auth);
                });
            }

            Self {
                resource_def,
                mint_authority: Vault::with_bucket(mint_authority),
                burn_authority: Vault::with_bucket(burn_authority),
                count: 0,
                public_key,
                redeem_auth,
            }
            .instantiate()
        }

        /// return the ResourceDef this Transporter can mint/burn
        pub fn resource_def(&self) -> ResourceDef {
            self.resource_def.clone()
        }

        /// redeem SealedVoucher for Bucket without key (expecting Voucher has a baked in key)
        #[auth(redeem_auth)]
        pub fn redeem_without_key(&mut self, sealed_voucher: SealedVoucher) -> Bucket {
            self.voucher_redeem(sealed_voucher.unseal(&self.public_key), None)
        }

        /// redeem SealedVoucher for Bucket with key
        #[auth(redeem_auth)]
        pub fn redeem_with_key(&mut self, sealed_voucher: SealedVoucher, key: NonFungibleKey) -> Bucket {
            self.voucher_redeem(sealed_voucher.unseal(&self.public_key), Some(key))
        }

        /// redeem SealedVoucher for Bucket with optional key (ie. the flexible API)
        #[auth(redeem_auth)]
        pub fn redeem(&mut self, sealed_voucher: SealedVoucher, optional_key: Option<NonFungibleKey>) -> Bucket {
            self.voucher_redeem(sealed_voucher.unseal(&self.public_key), optional_key)
        }

        /// redeem SealedVoucher for Bucket without key, expecting Transporter to manage the NonFungibleKey using a counter
        ///
        /// Useful example API not utilized in HareSwap
        #[auth(redeem_auth)]
        pub fn redeem_next(&mut self, sealed_voucher: SealedVoucher) -> Bucket {
            self.count += 1;
            self.voucher_redeem(sealed_voucher.unseal(&self.public_key), Some(self.count.into()))
        }

        /// Dispose of the bucket and makes an unsigned opaque Voucher to be signed in it's place.
        ///
        /// LIMITATION: only handles a bucket with single NonFungible
        ///
        /// This would be used to move assets off-ledger.  Security tradeoffs
        /// exist for sure.  HareSwap does not make use of this functionality.
        /// See `voucher_make` for the full implementation.  This wrapper exists
        /// to keep Voucher out of the function signature
        pub fn make(&mut self, bucket: Bucket) -> Vec<u8> {
            scrypto_encode(&self.voucher_make(bucket))
        }

        /* non-public functionality */

        /// The main functionality to redeem the Voucher using mint_authority
        ///
        /// NOTE: This helps factor out common code but then becomes security
        /// sensitive, hence it is not public on purpose.  Because Voucher can't
        /// be in public signatures it helps us not make tha mistake too.
        fn voucher_redeem(&mut self, v: Voucher, optional_key: Option<NonFungibleKey>) -> Bucket {
            self.mint_authority.authorize(
                |auth| v.redeem(&self.resource_def, optional_key, auth)
            )
        }

        /// not public implementation for `make`.  Only non-public because of the Voucher return type
        ///
        /// LIMITATION: only handles a bucket with single NonFungible
        fn voucher_make(&mut self, bucket: Bucket) -> Voucher {
            assert_eq!(bucket.amount(), Decimal::one(), "Transporter::voucher_make: only supports transportation of one nonFungibles per Voucher (for now)");
            assert_eq!(bucket.resource_def().resource_type(), ResourceType::NonFungible, "Transporter::voucher_make: only supports transportation of NonFungibles (for now)");

            let mut resource_def = bucket.resource_def();
            assert_eq!(self.resource_def, resource_def, "Transporter::voucher_make: resource mismatch");

            let nfds = bucket.get_non_fungibles::<PassThruNFD>();
            for entry in &nfds {
                let nfd: PassThruNFD = entry.data();
                let key: NonFungibleKey = entry.key();
                let key = Some(key);

                let authority = if self.burn_authority.is_empty() { &mut self.mint_authority } else { &mut self.burn_authority };

                authority.authorize(|auth|
                    resource_def.burn_with_auth(bucket, auth)
                );

                return Voucher::from_nfd(resource_def, key, nfd);  // purposeful "return".  More complex implemenation would handle multiple nfds
            };
            panic!("unreachable"); // asserted 1 in bucket
        }
    }
}
