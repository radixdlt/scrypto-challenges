use scrypto::prelude::*;
 
blueprint! {
   struct TokenSale {
        token_vault: Vault,
        xrd_vault: Vault,
        price_per_token: Decimal
   }
 
   impl TokenSale {

       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
           let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Seller Badge")
                .initial_supply(1);
           
           let token: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Everyone's Got One")
                .metadata("symbol", "EGO")
                .metadata("team-member-1-ticket-number", "4060779029")
                .metadata("team-member-2-ticket-number", "4057634689")
                .metadata("team-member-3-ticket-number", "4065937579")
                .metadata("team-member-4-ticket-number", "4035242529")
                .initial_supply(100_000);

           let auth = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));
                    
            let component = Self {
                token_vault: Vault::with_bucket(token),
                xrd_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(auth)
            .globalize();

            (component, seller_badge)
       }
 
       pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let qty = funds.amount() / self.price_per_token;
            self.xrd_vault.put(funds);
            self.token_vault.take(qty)
       }
 
       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
           self.xrd_vault.take(amount)
       }
 
       pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price;
       }

   }

}
