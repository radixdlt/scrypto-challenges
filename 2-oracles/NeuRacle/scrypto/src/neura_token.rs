//! [NeuraToken] blueprint is the example token blueprint to create NeuRacle token.
//! This blueprint can also include vesting, locking, ico,... method to further decentralize Neura token.
//! 
//! Since all NeuRacle core blueprint don't require cross-blueprint call from this blueprint. We can also build a decentralized token first and launch NeuRacle project later.
//! 
//! However, to make things simple for NeuRacle showcase, this blueprint will only create a sample token, a mint-burn controller badge and an admin badge.

use scrypto::prelude::*;

blueprint! {
    struct NeuraToken {
    }

    impl NeuraToken {

            pub fn new_token(name: String, symbol: String, initial_supply: Decimal, divisibility: u8) -> (Bucket, Bucket, Bucket) {

                let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", name.clone() +  "Admin Badge")
                .initial_supply(dec!("1"));

                info!(
                    "Admin badge address: {}", admin_badge.resource_address()
                );

                let mint_controller_badge = ResourceBuilder::new_fungible()
                .metadata("name", name.clone() +  "Mint Controller Badge")
                .initial_supply(dec!("1"));

                info!(
                    "Mint controller badge address: {}", mint_controller_badge.resource_address()
                );

                let controller_badge = ResourceBuilder::new_fungible()
                .mintable(rule!(require(mint_controller_badge.resource_address())), LOCKED)
                .metadata("name", name.clone() + "Controller Badge")
                .no_initial_supply();

                info!(
                    "Controller badge address: {}", controller_badge
                );

                let token_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(divisibility)
                .updateable_metadata(rule!(require(admin_badge.resource_address())), MUTABLE(rule!(require(admin_badge.resource_address()))))
                .mintable(rule!(require(controller_badge)), LOCKED)
                .burnable(rule!(require(controller_badge)), LOCKED)
                .metadata("name", name.clone())
                .metadata("symbol", symbol)
                .initial_supply(initial_supply);

                info!("{}: {}", name, token_bucket.resource_address());

            return (token_bucket, admin_badge, mint_controller_badge)

            }
        }
    }