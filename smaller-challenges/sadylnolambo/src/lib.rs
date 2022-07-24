use scrypto::prelude::*;
 
blueprint! {
   struct TokenSale {
       awesome_resource: Vault,
       xrd_tokens_vault: Vault,
       price_per_token: Decimal
   }
 
   impl TokenSale {
       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "sadlynolambo")
            .metadata("team-member-1-ticket-number", "#4065105369")
            .metadata("team-member-2-ticket-number", "#4115513309")
            .metadata("team-member-3-ticket-number", "#4146963599")
            .divisibility(18)
            .initial_supply(100000);

            let seller_badge :Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "sellerbadge")
                .metadata("symbol", "SELLER")
                .divisibility(DIVISIBILITY_NONE)    
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                awesome_resource: Vault::with_bucket(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();
            (component_address, seller_badge)

        }

       
       pub fn buy(&mut self, funds: Bucket) -> Bucket {
           let amount: Decimal = funds.amount() / self.price_per_token;
           self.xrd_tokens_vault.put(funds);
           self.awesome_resource.take(amount)
       }
 
       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
           self.xrd_tokens_vault.take(amount)
       }
 
       pub fn change_price(&mut self, price: Decimal) {
           self.price_per_token = price;
       }
   }
}

