//! [NStableCoin] is the blueprint for creating native stable coin project of NeuRacle ecosystem.
//! User can use this blueprint for swapping into algorithmed stablecoin.
//! 
//! Other individuals, teams can also utilize this blueprint to create "stable value" for their own token by getting a Neuracle user badge and feed that on the input arguments.
//!
//! Eg: Radix DLT can buy a Neuracle user badge and use this blueprint to make their Radix token both a smartcontract platform medium and a stable coin medium.
//! Though, this blueprint haven't got a re-funding method to continue using oracle yet. 
//!
//! Moreover, this approach (same as [Luna](https://www.terra.money/)) has proven not viable. The stability of the coin will come at the cost of the medium token's inflation.
//! Eg: When people swap to the stablecoin at the top price of xrd, they can swap again to xrd at the lower price, so they made money at the cost of xrd inflation.
//! 
//! Therefore, this blueprint is only for showing how NeuRacle data validation service can be of benefit to other DeFi projects.

use scrypto::prelude::*;
use crate::neuracle::NeuRacle;

blueprint! {
    struct NStableCoin {
        fee: Decimal,
        medium: ResourceAddress,
        symbol: String,
        pegged_to: String,
        stablecoin: ResourceAddress,
        controller_badge: Vault,
        data_badge: Vault,
        neuracle: ComponentAddress
    }

    impl NStableCoin {
        
        pub fn new(medium_token: ResourceAddress, pegged_to: String, neuracle: ComponentAddress, controller_badge: Bucket, data_badge: Bucket, fee: Decimal) -> ComponentAddress {

            let symbol: String = borrow_resource_manager!(medium_token).metadata().get("symbol").unwrap().into();

            let name: String = pegged_to.clone() + "NStable Coin";

            info!("Using {} as {}NStableCoin medium token", symbol, pegged_to.clone());

            info!("Using NeuRacle's service to get price data");

            let stablecoin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", name.clone())
                .metadata("pegged_into", pegged_to.clone())
                .metadata("symbol", pegged_to.clone() + "N")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

                info!(
                    "{}N: {}", pegged_to.clone(), stablecoin
                );

            let component = Self {
                fee: fee / dec!("100"),
                medium: medium_token,
                symbol: symbol,
                pegged_to: pegged_to,
                stablecoin: stablecoin,
                controller_badge: Vault::with_bucket(controller_badge),
                data_badge: Vault::with_bucket(data_badge),
                neuracle: neuracle
            }
            .instantiate()
            .globalize();

            info!(
                "{} address: {}", name, component
            );

            return component
        }

        pub fn auto_swap(&mut self, token_bucket: Bucket) -> Bucket {

            let neuracle: NeuRacle = self.neuracle.into();

            let (data_badge, price) = neuracle.get_data(self.data_badge.take(dec!("1")));

            self.data_badge.put(data_badge);

            let price = Decimal::from(price);

            info!("Current {} price is {} {}, begin auto swap", self.symbol, price, self.pegged_to.clone());
            
            let token = token_bucket.resource_address();
            assert!(
                (token == self.stablecoin) || (token == self.medium) ,
                "Vault not contain this resource, cannot swap."
            );

            if token == self.medium {

                let initial_amount: Decimal = token_bucket.amount();

                let amount: Decimal = initial_amount * price * (dec!("1") - self.fee);

                let stable_coin_bucket = self.controller_badge.authorize(|| {
                    borrow_resource_manager!(self.stablecoin).mint(amount)
                });

                self.controller_badge.authorize(|| borrow_resource_manager!(self.medium).burn(token_bucket));

                info!("You have swappd {} {} for {} {}.", initial_amount, self.symbol, amount, self.pegged_to.clone() + "N");

                return stable_coin_bucket
            }

            else {

                let initial_amount: Decimal = token_bucket.amount();

                let amount: Decimal = (initial_amount / price) * (dec!("1") - self.fee);

                let medium_token_bucket = self.controller_badge.authorize(|| {
                    borrow_resource_manager!(self.stablecoin).mint(amount)
                });

                self.controller_badge.authorize(|| {
                    token_bucket.burn()
                });

                info!("You have swappd {} {} for {} {}.", initial_amount, self.pegged_to.clone() + "N", amount, self.symbol);

                return medium_token_bucket
            }
        }
    }
}
