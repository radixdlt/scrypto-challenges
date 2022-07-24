use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        bnb_vault: Vault,
        xrd_vault: Vault,
        price_per_token: Decimal
    }

    impl TokenSale {        
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let bnb_bucket : Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("team-member-1-ticket-number", "#4047063629")
                .metadata("team-member-2-ticket-number", "#4140149019")
                .metadata("team-member-3-ticket-number", "#4130251199")
                .metadata("name", "blackandblue")
                .metadata("symbol", "BNB")
                .initial_supply(100000);

            let seller_badge : Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Seller Badge")
                .metadata("symbol", "SELLER")
                .initial_supply(1);

            let access_rules = AccessRules::new()
            .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
            .method("change_price", rule!(require(seller_badge.resource_address())))
            .default(rule!(allow_all));

            let token_sale_component : ComponentAddress = Self {
                bnb_vault: Vault::with_bucket(bnb_bucket),
                xrd_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (token_sale_component, seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            // Your `buy` implementation goes here.
            let amount : Decimal = funds.amount() / self.price_per_token;
            self.xrd_vault.put(funds);
            self.bnb_vault.take(amount)
        }
  
        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_vault.take(amount)
        }
  
        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price
        }
 
    }
}