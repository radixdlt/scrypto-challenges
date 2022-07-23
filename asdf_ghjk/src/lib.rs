use scrypto::prelude::*;

blueprint! {
    struct Asdf{
        asdf_token_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal,
    }

    impl Asdf{

        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket){
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "asdf_ghjk")
                .metadata("symbol", "ASDF")
                .metadata("team-member-1-ticket-number", "4102209389")
                // .metadata("team-member-2-ticket-number", "13")
                // .metadata("team-member-3-ticket-number", "11")
                .initial_supply(100_000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Seller Badge")
                .metadata("symbol", "SELLER")
                .initial_supply(1);


            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress= Self {
                asdf_token_vault: Vault::with_bucket(my_bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token:  price_per_token
            }

            .instantiate()
            .add_access_check(access_rules)
            .globalize();
            (component_address, seller_badge)

        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            // how much can be bought
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            // deposit xrd into its vault
            self.xrd_tokens_vault.put(funds);
            // take the bought tokens and return it
            self.asdf_token_vault.take(purchase_amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price
        }


        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            // returns bucket with xrd
            self.xrd_tokens_vault.take(amount)
        }

      

    }
}
