use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        // Define what resources and data will be managed by Hello components
        useful_toekns_vault: Vault,
        xrd_toekns_vault: Vault,
        price_per_token: Decimal

    }

    impl TokenSale {
        // Implement the functions and methods which will manage those resources and data
        
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket)  {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Useful Token")
                .metadata("teamname", "Rajnish Kumar")
                .metadata("team-member-1", "4117002169")
                .metadata("team-member-2", "0")
                .metadata("team-member-3", "0")
                .metadata("team-member-4", "0")
                .metadata("symbol", "USEFUL")
                .initial_supply(100000);


            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Seller Badge")
                .metadata("symbol", "SELLER")
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let component_address: ComponentAddress =  Self {
                                        useful_toekns_vault: Vault::with_bucket(bucket),
                                        xrd_toekns_vault: Vault::new(RADIX_TOKEN),
                                        price_per_token: price_per_token
                                        }
                                        .instantiate()
                                        .add_access_check(access_rules)
                                        .globalize();
            

            (component_address, seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_toekns_vault.put(funds);
            self.useful_toekns_vault.take(purchase_amount)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_toekns_vault.take(amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price
        }
    }
}