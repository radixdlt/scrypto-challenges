use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        vault: Vault,
        xrd_vault: Vault,
        price_per_token: Decimal,
    }
  
    impl TokenSale {
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let tokens: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "medici")
                .metadata("team-member-1-ticket-number", "4059117939")
                .metadata("team-member-2-ticket-number", "4060198209")
                .metadata("team-member-3-ticket-number", "4073822699");
                .metadata("team-member-4-ticket-number", "4057731819")
                .divisibility(DIVISIBILITY_MAXIMUM)
                .initial_supply(100_000);

            assert!(price_per_token > Decimal::from(0), "price_per_token must be positive");

            let seller_badge = ResourceBuilder::new_fungible()
                .initial_supply(1);

            let access_rules = AccessRules::new()
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let component = Self {
                vault: Vault::with_bucket(tokens),
                xrd_vault: Vault::new(RADIX_TOKEN),
                price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component, seller_badge)
        }
  
        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let tokens_bought = funds.amount() / self.price_per_token; 
            // The above does integer math, so there might be some change
            let _change = funds.amount() - tokens_bought * self.price_per_token;
            // In general, you want to return the change as another bucket, but that change is not provided
            // for in the spec, SO WE TAKE THEM ALL, MWAH HA HA
            info!("You would have received {} in change", _change);
            self.xrd_vault.put(funds);
            let tokens = self.vault.take(tokens_bought);
            tokens
        }
  
        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            // Validation that amount < balanec is taken care of by Radix
            self.xrd_vault.take(amount)
        }
  
        pub fn change_price(&mut self, price: Decimal) {
            assert!(price.is_positive(), "Price must be greater than zero");
            self.price_per_token = price
        }
    }
 }