use scrypto::prelude::*;

blueprint! {
    struct Hello {
        // Define what resources and data will be managed by Hello components
        washing_machine_vault: Vault
        price_per_token: Decimal
        xrd_vault: Vault
        
    }

    impl Hello {
        
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let clean_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "WashingMachineCoin")
                .metadata("team-member-1-ticket-number ", "#4078804119")
                .metadata("team-member-2-ticket-number ", "#4080431879")
                .metadata("team-member-3-ticket-number ", "#4099422709")
                .divisibility(DIVISIBILITY_MAXIMUM)
                .initial_supply(100000);

            let dirty_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "XRDCoin")
                .divisibility(DIVISIBILITY_MAXIMUM)
                .initial_supply(10000000000000)

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Seller badge")
                .metadata("symbol", "SELLLER")
                .divisibility(0)
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
            .method("withdraw", rule!(require(seller_badge.resource_address())))
            .method("change_price", rule!(require(seller_badge.resource_address())))
            .default(rule!(allow_all));
            
            let componentAddress : ComponentAddress = self {
                self.price_per_token : Decimal::price_per_token,
                self.washing_machine_vault: Vault::with_bucket(clean_bucket),
                self.xrd_vault: Vault::new(RADIX_TOKEN)
                
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize()

            (componentAddress, seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let bought_tokens: Decimal = funds.amount / price_per_token
            washing_machine_vault.put(bought_tokens)
            rdx_vault.take(funds)
            
        }
        
        pub fn withdraw(&mut self, amount: Decimal) {
            self.xrd_vault.take(amount)

        }

        pub fn change_price(&mut self, price:Decimal){
            self.price_per_token = price
        }
    }
}