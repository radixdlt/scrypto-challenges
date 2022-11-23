use scrypto::prelude::*;
blueprint! {
    struct TokenSale {
        momentum_tokens_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal
    }

    impl TokenSale {
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Momentum One's Token")
                .metadata("symbol", "MOMENTUM")
                .metadata("team-member-1-ticket-number", "4094634189")
                .metadata("team-member-2-ticket-number", "4146477819")
                .metadata("team-member-3-ticket-number", "4146477489")
                .metadata("team-member-4-ticket-number", "4141430489")
                .metadata("team-member-5-ticket-number", "4094682069")
                .initial_supply(100_000);

            let badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Nerd Badge")
                .metadata("symbol", "NERD")
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(badge.resource_address())))
                .method("change_price", rule!(require(badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                momentum_tokens_vault: Vault::with_bucket(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, badge)
        }
        
        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount : Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.momentum_tokens_vault.take(purchase_amount)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_tokens_vault.take(amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price;
        }
    }
}
