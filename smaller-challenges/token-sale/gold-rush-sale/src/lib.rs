// UTIL COMMANDS:
// resim reset
// resim new-account
// resim publish .
// resim call-function <PACKAGE_ADDRESS> TokenSale new 100

use scrypto::prelude::*;

blueprint! {
    // XRD -> GRSH
    // GRSH -> XRD

    struct TokenSale {
        stored_gold_rush: Vault,
        stored_xrd: Vault,
        price_per_token: Decimal
    }

    impl TokenSale {
        
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            pub const DIVISIBILIY: u8 = 18;

            let bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILIY)
                .metadata("name", "The GOLDRUSH of the 21st CENTURY")
                .metadata("team-member-1-ticket-number", "4025384239")   
                .metadata("team-member-2-ticket-number", "4015981649")
                .metadata("team-member-3-ticket-number", "4054925729")
                .metadata("team-member-4-ticket-number", "4075211669")
                .metadata("team-member-5-ticket-number", "4024184869")
                .metadata("symbol", "GRSH")
                .mintable(rule!(allow_all), LOCKED)
                .initial_supply(100000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Seller Badge")
                .metadata("symbol", "SELLER")
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                stored_gold_rush: Vault::with_bucket(bucket),
                stored_xrd: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.stored_xrd.put(funds);
            self.stored_gold_rush.take(purchase_amount)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.stored_xrd.take(amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price
        }
    }
}