use scrypto::prelude::*;
 
blueprint! {
    struct TokenSale {
        treasury_xrd: Vault,
        treasury_tokens: Vault,
        price_per_token: Decimal
    }
 
   impl TokenSale {
       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let token_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Babylonians")
                .metadata("team-member-1-ticket-number", "3999689809")
                .metadata("team-member-2-ticket-number", "4072398489")
                .metadata("team-member-3-ticket-number", "4086028099")
                .metadata("symbol", "BT")
                .initial_supply(100000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "seller")
                .metadata("symbol", "SEL")
                .initial_supply(1);

            let rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component = Self {
                treasury_tokens: Vault::with_bucket(token_bucket),
                treasury_xrd: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(rules)
            .globalize();
            (component, seller_badge)
       }
 
    pub fn buy(&mut self, funds: Bucket) -> Bucket {
        let amount_tokens = funds.amount() / self.price_per_token;
        self.treasury_xrd.put(funds);
        self.treasury_tokens.take(amount_tokens)
    }
 
    pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
        self.treasury_xrd.take(amount)
    }
 
    pub fn change_price(&mut self, price: Decimal) {
        self.price_per_token = price;
    }
   }
}
