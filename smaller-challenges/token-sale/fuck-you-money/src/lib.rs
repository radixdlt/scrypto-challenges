use scrypto::prelude::*;
 
blueprint! {
   struct TokenSale {
       price_per_token: Decimal,
       xrd_vault: Vault,
       token_vault: Vault
   }
 
   impl TokenSale {
       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Fuck You Money")
                .metadata("symbol", "FYM")
                .metadata("team-member-1-ticket-number", "4097518559")
                .metadata("team-member-2-ticket-number", "4135147959")
                .metadata("team-member-3-ticket-number", "4094193999")
                .metadata("team-member-4-ticket-number", "4135719059")
                .initial_supply(100_000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "admin page")
                .divisibility(DIVISIBILITY_NONE)
                .metadata("symbol", "SELLER")
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                token_vault: Vault::with_bucket(bucket),
                xrd_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, seller_badge)
       }
 
       pub fn buy(&mut self, funds: Bucket) -> Bucket {
           let amount: Decimal = funds.amount() / self.price_per_token;
           self.xrd_vault.put(funds);
           self.token_vault.take(amount)
       }
 
       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
           self.xrd_vault.take(amount)
       }
 
       pub fn change_price(&mut self, price: Decimal) {
           self.price_per_token = price
       }
   }
}
