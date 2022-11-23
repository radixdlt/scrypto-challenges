use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        mps_tokens_vault: Vault, 
        xrd_tokens_vault: Vault,
        price_per_token: Decimal
    }

    impl TokenSale {
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
           let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "MyPiSlice")
                .metadata("symbol", "MPS")
                .metadata("team-member-ticket-number", "394517801")
                .initial_supply(100000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Seller Badge")
                .metadata("symbol", "Seller")
                .initial_supply(1);
            
            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let mut component_address = Self {
                mps_tokens_vault: Vault::with_bucket(my_bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }.instantiate();

            component_address.add_access_check(access_rules);
            let component_address = component_address.globalize();

            (component_address, seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.mps_tokens_vault.take(purchase_amount)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_tokens_vault.take(amount)
        }

        pub fn change_price(&mut self, price: Decimal){
            self.price_per_token = price
        }
    }
}
