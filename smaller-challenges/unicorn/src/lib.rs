use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        // Define what resources and data will be managed by Hello components
        useful_token_vault: Vault, 
        xrd_tokens_vault: Vault, 
        price_per_token: Decimal 


    }

    impl TokenSale {
        // Implement the functions and methods which will manage those resources and data
        
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "unicorn")
                .metadata("team-member-1-ticket-number", "4045333929")
                .metadata("team-member-2-ticket-number", "4056537309")
                .metadata("team-member-3-ticket-number", "4105999159")
                .metadata("team-member-4-ticket-number", "4106248989")
                .divisibility(DIVISIBILITY_MAXIMUM)
                .initial_supply(100000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .restrict_withdraw(rule!(deny_all), LOCKED)
            .metadata("name","Seller")
            .metadata("symbol", "SELLER")
            .initial_supply(1);
            
            let access_rules: AccessRules = AccessRules::new()
            .method("widthdraw_funds", rule!(require(seller_badge.resource_address())))
            .method("change_price", rule!(require(seller_badge.resource_address())))
            .default(rule!(allow_all));

            let component_address: ComponentAddress = Self{
                useful_token_vault: Vault::with_bucket(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, seller_badge)

 
        }

    

        pub fn buy(&mut self, funds: Bucket)->Bucket{
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.useful_token_vault.take(purchase_amount)
        }

        pub fn change_price(&mut self, price: Decimal){
            self.price_per_token = price
        }

        pub fn withdraw_funds(&mut self, amount: Decimal )->Bucket{
            self.xrd_tokens_vault.take(amount)
        }


    }
}