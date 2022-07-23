use scrypto::prelude::*;

blueprint! {
   struct TokenSale {
       ftt_vault: Vault,
       xrd_vault: Vault,
       price_per_token: Decimal,
   }

   impl TokenSale {
       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Fantastic Three")
                .metadata("symbol", "FTTT")
                .metadata("team-member-1-ticket-number", "4037681609")
                .metadata("team-member-2-ticket-number", "4027467209")
                .metadata("team-member-3-ticket-number", "4027515889")
                .initial_supply(100000);

            //TODO: Default rules for withdraw method, please change this to your own rules.
            let access_rules = AccessRules::new()
            .method("withdraw_funds", rule!(require(admin_badge.resource_address())))
            .method("change_price", rule!(require(admin_badge.resource_address())))
            .default(rule!(allow_all));

            (
                Self {ftt_vault: Vault::new(RADIX_TOKEN), xrd_vault: Vault::new(RADIX_TOKEN), price_per_token: price_per_token}
                    .instantiate()
                    .add_access_check(access_rules)
                    .globalize(),
                admin_badge
            )
       }

       pub fn buy(&mut self, mut funds: Bucket) -> Bucket {
            let ammount_of_tokens = funds.amount() / self.price_per_token;
            let fttt_share = funds.take(ammount_of_tokens);
            self.xrd_vault.put(fttt_share);
            let bought_tokens = self.xrd_vault.take(ammount_of_tokens);
            funds.put(bought_tokens);
            return funds;
       }

       pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price;
       }

       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
           return self.xrd_vault.take(amount);
       }
   }
}
