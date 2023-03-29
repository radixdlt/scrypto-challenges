//! The `DemoTokenFaucet` Blueprint can be used to create and mint tokens that may be used to demo
//! functionalities of the flashyfi dApp. When instantiated, the `DemoTokenFaucet` mints 10 of the
//! most popular tokens on OCI-Swap (at the time of this writing). User can then call the
//! component's `free` method to receive some of these tokens.

use scrypto::prelude::*;

#[blueprint]
mod demo_token_faucet {

    use super::*;

    struct DemoTokenFaucet {
        minter: Vault,
    }

    impl DemoTokenFaucet {
        pub fn instantiate_global() -> (ComponentAddress, Vec<ResourceAddress>) {
            let minter = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE).mint_initial_supply(dec!("1"));

            let tokens = [
                TokenDefinition::new("Ociswap", "OCI"),
                TokenDefinition::new("DefiPlaza", "DFP2"),
                TokenDefinition::new("Floop", "FLOOP"),
                TokenDefinition::new("CaviarNine", "CAVIAR"),
                TokenDefinition::new("Astrolescent", "ASTRL"),
                TokenDefinition::new("XIDAR", "IDA"),
                TokenDefinition::new("Foton", "FOTON"),
                TokenDefinition::new("DogeCube", "DGC"),
                TokenDefinition::new("Genesis Nerds", "GNRD"),
                TokenDefinition::new("Bobby", "BOBBY"),
            ];

            let tokens_addresses: Vec<ResourceAddress> = tokens
                .into_iter()
                .map(|token| {
                    ResourceBuilder::new_fungible()
                        .divisibility(DIVISIBILITY_MAXIMUM)
                        .metadata("name", token.name)
                        .metadata("symbol", token.symbol)
                        .mintable(rule!(require(minter.resource_address())), LOCKED)
                        .create_with_no_initial_supply()
                })
                .collect();

            let component = Self { minter: Vault::with_bucket(minter) };

            (component.instantiate().globalize(), tokens_addresses)
        }

        pub fn free(&self, token_address: ResourceAddress, amount: Decimal) -> Bucket {
            assert!(amount > Decimal::zero() && amount <= dec!("10000"));

            let rm = borrow_resource_manager!(token_address);
            self.minter.authorize(|| rm.mint(amount))
        }
    }
}

#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe)]
pub struct TokenDefinition {
    name: String,
    symbol: String,
}

impl TokenDefinition {
    fn new<S: Into<String>>(name: S, symbol: S) -> Self {
        TokenDefinition { name: name.into(), symbol: symbol.into() }
    }
}
