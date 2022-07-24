use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        secret_contractor_tokens_vault: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal
    }

    impl TokenSale {
        pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let secret_contractor_tokens: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "SecretContractorsToken")
                .metadata("symbol", "SCT")
                .metadata("team-member-1-ticket-number", "#4122423769")
                .metadata("team-member-2-ticket-number", "#4122096389")
                .metadata("team-member-3-ticket-number", "#4093820369")
                .metadata("team-member-4-ticket-number", "#4117118029")
                .initial_supply(100000);
                
            let secret_contractor_token_seller_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "SecretContractorsToken Seller Badge")
                .metadata("symbol", "SCTSELLER")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);

            let access_rules: AccessRules = AccessRules::new()
                .method("change_price", rule!(require(secret_contractor_token_seller_badge.resource_address())))
                .method("withdraw_funds", rule!(require(secret_contractor_token_seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                secret_contractor_tokens_vault: Vault::with_bucket(secret_contractor_tokens),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, secret_contractor_token_seller_badge)
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            info!("purchase: {} XRD -> {} SCT", funds.amount(), purchase_amount);
            self.xrd_tokens_vault.put(funds);
            self.secret_contractor_tokens_vault.take(purchase_amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            info!("changing price {}", price);
            self.price_per_token = price
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            info!("withdrawing funds {}", amount);
            self.xrd_tokens_vault.take(amount)
        }
    }
}