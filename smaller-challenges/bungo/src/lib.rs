use scrypto::prelude::*;

blueprint! {
    struct Bungo {
        bng_vault: Vault,
        xrd_vault : Vault,
        bng_to_xrd_rate: Decimal
    }

    impl Bungo {

        pub fn new (price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Bungo token")
                .metadata("about", "Bung bung")
                .metadata("symbol", "BNG")
                .metadata("team-member-1-ticket-number", "4104507209")
                .metadata("team-member-2-ticket-number", "4019578689")
                .divisibility(DIVISIBILITY_MAXIMUM)
                .initial_supply(100000);
            
            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("Name", "master access token")
                .metadata("symbol", "SELLER")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                    bng_vault: Vault::with_bucket(bucket),
                    xrd_vault: Vault::new(RADIX_TOKEN),
                    bng_to_xrd_rate: price_per_token
            }            
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, seller_badge)
        }

        pub fn buy (&mut self, funds: Bucket) -> Bucket {
            let xrd_to_giveout: Decimal = funds.amount() / self.bng_to_xrd_rate;
            self.xrd_vault.put(funds);
            self.bng_vault.take(xrd_to_giveout)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_vault.take(amount)
        }

        pub fn change_price(&mut self, price:Decimal){
            self.bng_to_xrd_rate = price
        }
    }
}