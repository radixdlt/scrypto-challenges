use scrypto::prelude::*;
 
blueprint! {
   struct TokenSale {
       // Your state variables go here.
       
       ultimate_vault: Vault,
       xrd_vault: Vault,
       price_per_token: Decimal
   }
 
   impl TokenSale {
       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            // Your `new` implementation goes here.
            let ultimate_tokens = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_MAXIMUM)
            .metadata("name", "Ulti Token")
            .metadata("team-member-1-ticket-number ", "40698137096534576059001")
            .metadata("symbol", "ULTI")
            .initial_supply(100_000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Seller Badge")
                .metadata("symbol", "SELLER")
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component = Self {
                ultimate_vault: Vault::with_bucket(ultimate_tokens),
                xrd_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component, seller_badge)
       }
 
       pub fn buy(&mut self, funds: Bucket) -> Bucket {
           // Your `buy` implementation goes here.
           let token_amount: Decimal = funds.amount() / self.price_per_token;
           self.xrd_vault.put(funds);
           self.ultimate_vault.take(token_amount)
       }
 
       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
           // Your `withdraw_funds` implementation goes here.
           self.xrd_vault.take(amount)
       }
 
       pub fn change_price(&mut self, price: Decimal) {
           // Your `change_price` implementation goes here.
           self.price_per_token = price;
       }
   }
}
