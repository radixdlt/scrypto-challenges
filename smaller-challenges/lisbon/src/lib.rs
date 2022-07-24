use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        usefull_token_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal

    }
    impl TokenSale {
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "RdxLisbon")
            .metadata("team-member-1-ticket-number", "4075670059")
            .metadata("team-member-2-ticket-number", "4114764599")
            .metadata("symbol", "RDXLX")
            .divisibility(DIVISIBILITY_MAXIMUM)
            .initial_supply(100000);


            let seller_badge: Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "Seller Badge")
            .metadata("symbol", "SELLER")
            .divisibility(DIVISIBILITY_NONE)
            .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
            .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
            .method("change_price", rule!(require(seller_badge.resource_address())))
            .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                usefull_token_vault: Vault::with_bucket(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, seller_badge)
            
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.usefull_token_vault.take(purchase_amount)
        }


    pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket{
        self.xrd_tokens_vault.take(amount)
    }

    pub fn change_price(&mut self, price: Decimal) {
        self.price_per_token = price;
    }
    }
}