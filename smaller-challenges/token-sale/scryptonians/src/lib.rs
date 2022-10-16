use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        // Define what resources and data will be managed by TokenSale components
        nian_tokens_vault: Vault,
        xrd_tokens_vault: Vault, 
        price_per_token: Decimal        
    }

    impl TokenSale {
        // Implement the functions and methods which will manage those resources and data
        
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(price_per_token:Decimal) -> (ComponentAddress, Bucket) {
            // Create a new token called "TokenSaleToken," with a fixed supply of 1000, and put that supply into a bucket
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Scryptonian")
                .metadata("symbol", "NIAN")
                .metadata("team-member-1-ticket-number","4133951729")
                .metadata("team-member-2-ticket-number","4081681819")
                .metadata("team-member-3-ticket-number","4018605109")
                .metadata("team-member-4-ticket-number","4114599189")
                .initial_supply(100000);
                

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Seller name")
                .metadata("symbol", "SELLER")
                .initial_supply(1);
                
            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

 
            let component_address: ComponentAddress = Self { 
                nian_tokens_vault: Vault::with_bucket(my_bucket),
                xrd_tokens_vault: Vault:: new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            
            
            .instantiate()
            .globalize();
            
            (component_address, seller_badge)
        }

        pub fn buy(&mut self, funds:Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() /self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.nian_tokens_vault.take(purchase_amount)
        }
        
        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            // Your `withdraw_funds` implementation goes here.
            self.xrd_tokens_vault.take(amount)
        }
  
        pub fn change_price(&mut self, new_price: Decimal) {
            // Your `change_price` implementation goes here.
            self.price_per_token = new_price
        }
 
    }
}