use scrypto::prelude::*;

blueprint! {
    struct TokenSales {
        useful_tokens_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal,
        seller_badge: ResourceAddress,
    }

    impl TokenSales {
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "Scrypto Challenge Token")
            .metadata("symbol", "SCT")
            .metadata("team-member-1-ticket-number", "4054057019")
            .metadata("team-member-2-ticket-number", "4073361989")
            .metadata("team-member-3-ticket-number", "4066471649")
            .metadata("team-member-4-ticket-number", "4102398759")
            .divisibility(DIVISIBILITY_MAXIMUM)
            .initial_supply(100_000);

            let seller_badge = ResourceBuilder::new_fungible()
            .metadata("name", "Seller Badge")
            .divisibility(DIVISIBILITY_NONE)
            .initial_supply(1);

            let access_rules = AccessRules::new()
            .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
            .method("change_price", rule!(require(seller_badge.resource_address())))
            .default(rule!(allow_all));

            let component = Self {
                useful_tokens_vault: Vault::with_bucket(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token,
                seller_badge: seller_badge.resource_address(),
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component, seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.useful_tokens_vault.take(purchase_amount)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_tokens_vault.take(amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price;
        }
 
    }
}