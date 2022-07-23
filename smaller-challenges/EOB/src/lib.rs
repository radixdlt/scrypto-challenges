use scrypto::prelude::*;

blueprint! {

    struct TokenSale{

        useful_token_vault: Vault,
        xrd_token_vault: Vault,
        price_per_token: Decimal
    }

    impl TokenSale{
        pub fn new(price_per_token: Decimal)  -> (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
             .metadata("name", "EOB Token")
             .metadata("symbol", "EOB")
             .metadata("team-member-1-ticket-number", "#4004245329")
             .metadata("team-member-2-ticket-number", "#4133171009")
             .metadata("team-member-3-ticket-number", "#4089299329")
             .initial_supply(100000);
            
            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Sellet badge")
                .metadata("symbol", "SELLER")
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                useful_token_vault: Vault::with_bucket(bucket),
                xrd_token_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
    
            .instantiate()
            .add_access_check(access_rules)
            .globalize();
            (component_address, seller_badge)
        }
         
        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_token_vault.put(funds);
            self.useful_token_vault.take(purchase_amount)
            }
        
        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket{
            self.xrd_token_vault.take(amount)

        }
        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price;
        }   
    }
}