use crate::_comon::*;
use scrypto::prelude::*;

#[blueprint]
mod faucet_blueprint {
    struct Faucet {
        resource_creation_authority_bage: Vault,
        collected_xrd: Vault,
        prices: HashMap<ResourceAddress, Decimal>,
        resource_addresse_list: HashMap<ResourceAddress, bool>,
    }

    impl Faucet {
        pub fn new() -> Bucket {
            let faucet_admin_bage = ResourceBuilder::new_uuid_non_fungible()
                .metadata("internal_tag", "faucet_admin_bage")
                .metadata("name", "Faucet Admin badge")
                .mint_initial_supply([(AuthBadgeData {})]);

            let resource_creation_authority_bage = Vault::with_bucket(
                ResourceBuilder::new_uuid_non_fungible()
                    .metadata("internal_tag", "resource_creation_authority_bage")
                    .metadata("name", "Token creator authority badge")
                    .mint_initial_supply([(AuthBadgeData {})]),
            );

            let mut faucet = Self {
                prices: HashMap::new(),
                resource_addresse_list: HashMap::new(),
                resource_creation_authority_bage,
                collected_xrd: Vault::new(RADIX_TOKEN),
            }
            .instantiate();

            faucet.update_price(RADIX_TOKEN, dec!(250));

            let admin_rule = rule!(require(faucet_admin_bage.resource_address()));

            let access_rules = AccessRules::new()
                .method("create_resource", admin_rule.clone(), LOCKED)
                .method("update_price", admin_rule.clone(), LOCKED)
                .default(AccessRule::AllowAll, AccessRule::DenyAll);

            faucet.add_access_check(access_rules);
            faucet.globalize();

            faucet_admin_bage
        }

        /// FAUCET : CRETE AND SUPPLY RESOURCES FOR TEST

        pub fn create_resource(
            &mut self,
            symbol: String,
            name: String,
            icon: String,
            initial_price: Decimal,
        ) {
            let resource_address = ResourceBuilder::new_fungible()
                .metadata("symbol", symbol)
                .metadata("name", name)
                .metadata("icon", icon)
                .mintable(
                    rule!(require(
                        self.resource_creation_authority_bage.resource_address()
                    )),
                    LOCKED,
                )
                .burnable(
                    rule!(require(
                        self.resource_creation_authority_bage.resource_address()
                    )),
                    LOCKED,
                )
                .create_with_no_initial_supply();

            self.update_price(resource_address, initial_price);

            self.resource_addresse_list.insert(resource_address, true);
        }

        pub fn get_resource(&mut self, resource: ResourceAddress, xrd: Bucket) -> Bucket {
            assert!(
                xrd.resource_address() == RADIX_TOKEN,
                "Provied XRD to get faucet tokens"
            );

            let from_price = match self.get_price(RADIX_TOKEN) {
                Some(int) => int,
                None => panic!("Price not found"),
            };
            let to_price = match self.get_price(resource) {
                Some(int) => int,
                None => panic!("Price not found"),
            };

            let to_amount = xrd.amount() * from_price / to_price;

            self.collected_xrd.put(xrd);

            self.resource_creation_authority_bage.authorize(|| {
                let lp_resource_manager = borrow_resource_manager!(resource);
                lp_resource_manager.mint(to_amount)
            })
        }

        /// TEST ORACLE

        /// Returns the current price of a resource pair BASE/QUOTE.
        pub fn get_price(&self, quote: ResourceAddress) -> Option<Decimal> {
            match self.prices.get(&quote) {
                Some(price) => Some(*price),
                None => None,
            }
        }
        /// Updates the price of a resource pair BASE/QUOTE and its inverse.
        pub fn update_price(&mut self, resource: ResourceAddress, price: Decimal) {
            self.prices.insert(resource, price);
        }

        /// TEST EXCHANGE

        pub fn swap(&mut self, from_bucket: Bucket, to_resource: ResourceAddress) -> Bucket {
            let from_resource = from_bucket.resource_address();

            let from_price = match self.get_price(from_resource) {
                Some(int) => int,
                None => panic!("Price not found"),
            };
            let to_price = match self.get_price(to_resource) {
                Some(int) => int,
                None => panic!("Price not found"),
            };

            let to_amount = from_bucket.amount() * from_price / to_price;

            self.resource_creation_authority_bage.authorize(|| {
                let lp_resource_manager = borrow_resource_manager!(to_resource);
                from_bucket.burn();
                lp_resource_manager.mint(to_amount)
            })
        }

        /// INTEREST FACTORY (3 types of interest available)

        pub fn get_loan_interest_rate(
            &self,
            interest_type: u8,
            _pool_asset_ressource_address: ResourceAddress,
            available_liquidity_amount: Decimal,
            total_loan_amount: Decimal,
        ) -> Decimal {
            // Fix interest rate at 10%
            if interest_type == 0u8 {
                return dec!("0.1");
            };

            let pool_amount = available_liquidity_amount + total_loan_amount;

            let pool_utilization = if pool_amount == Decimal::ZERO {
                return Decimal::ZERO;
            } else {
                total_loan_amount / pool_amount
            };

            // Linear interest rate from 5 to 20%
            if interest_type == 1u8 {
                let min_interest_rate = dec!("0.05");
                let max_interest_rate = dec!("0.20");

                let interest_rate =
                    min_interest_rate + pool_utilization * (max_interest_rate - min_interest_rate);

                return interest_rate;
            }

            // Stage intereste rate from 7% to 15%
            if interest_type == 2u8 {
                let min_interest_rate = dec!("0.07");
                let max_interest_rate = dec!("0.15");

                return if pool_utilization < dec!("0.5") {
                    min_interest_rate
                } else {
                    max_interest_rate
                };
            }

            panic!("Interest rate type not supported: {}", interest_type)
        }
    }
}
