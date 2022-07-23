use scrypto::prelude::*;

blueprint! {
    struct TokenSale{useful_tokens_vault: Vault,
        useful_tokens_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal
    
    
    }

    impl TokenSale{
        pub fn new(price_per_token: Decimal) -> ComponentAddress{
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Useful Token")
                .metadata("symbol", "USEFUL")
                .initial_supply(1_000);

            let seller_badge: Bucket = ResourceBuilder:: new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Seller Badge")
                .metadata("symbol", "SELLER")
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                useful_tokens_vault: Vault:: with_buckets(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize()

            (component_address, seller_badge)
                
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.useful_tokens_vault.take(purchase_amount)

        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket{
            self.xrd_tokens_vault.take(amount)

        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price
        }
    }

}