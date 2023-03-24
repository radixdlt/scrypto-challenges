use scrypto::prelude::*;

blueprint! {
    struct Suleiman {
        token_vault: Vault,
        collected_xrd: Vault,
        price_per_token: Decimal
    }

    impl Suleiman {
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "SuleimanToken")
                .metadata("symbol", "ST")
                .metadata("team-member-1-ticket-number", "#4009957079")
                .metadata("team-member-2-ticket-number", "#4060576449")
                .metadata("team-member-3-ticket-number", "#4060577689")
                .divisibility(DIVISIBILITY_MAXIMUM)
                .initial_supply(100000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Seller admin badge")
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                token_vault: Vault::with_bucket(my_bucket),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.collected_xrd.put(funds);
            self.token_vault.take(purchase_amount)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.token_vault.take(amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price
        }
    }
}
