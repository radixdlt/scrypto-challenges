use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        useful_tokens_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal    
    }

    impl TokenSale {
        pub fn new(price_per_token: Decimal) -> ComponentAddress {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Useful Token")
                .metadata("symbol", "USEFUL")
                .initial_supply(1_000);

            Self {
                useful_tokens_vault: Vault::with_bucket(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN), 
                price_per_token: price_per_token
            }
            .instantiate()
            .globalize()
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.useful_tokens_vault.take(purchase_amount)
        }
        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
             self.xrd_tokens_vault.take(amount);

        }
        
        pub fn change_price(&mut self, price: Decimal) { 
            self.price_per_token = price
        }
    }
} 