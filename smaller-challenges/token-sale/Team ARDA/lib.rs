use scrypto::prelude::*;

blueprint! {
   struct TokenSale {
        // The vault where the UsefulTokens will be stored.
        arda_tokens_vault: Vault,
        // The vault where the xrd payments will be stored.
        xrd_tokens_vault: Vault,
        // The price of a single UsefulToken.
        price_per_token: Decimal

   }

   impl TokenSale {

       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
        // Creating a new token called "UsefulToken"
        let my_bucket: Bucket = ResourceBuilder::new_fungible()
        .metadata("name", "ArdaToken")
        .metadata("symbol", "ARDA")
        .initial_supply(100000);

        // Creating a new seller badge which we will give the withdraw authority to
        let seller_badge: Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "Seller Badge")
            .metadata("symbol", "SELLER")
            .divisibility(0)
            .initial_supply(1);

        // Setting the access rules to only allow the seller badge to withdraw the funds or change the price
        let access_rules: AccessRules = AccessRules::new()
            .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
            .method("change_price", rule!(require(seller_badge.resource_address())))
            .default(rule!(allow_all));

        let component_address: ComponentAddress = Self {
            arda_tokens_vault: Vault::with_bucket(my_bucket),
            xrd_tokens_vault: Vault::new(RADIX_TOKEN),
            price_per_token: price_per_token
        }
        .instantiate()
        .add_access_check(access_rules)
        .globalize();

        return (component_address, seller_badge)
    }

       pub fn buy(&mut self, funds: Bucket) -> Bucket {
           // Your `buy` implementation goes here.
           let purchase_amount: Decimal = funds.amount() / self.price_per_token;
           self.xrd_tokens_vault.put(funds);
           self.arda_tokens_vault.take(purchase_amount)
       }

       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
           // Your `withdraw_funds` implementation goes here.
           self.xrd_tokens_vault.take(amount)
       }

       pub fn change_price(&mut self, price: Decimal) {
           // Your `change_price` implementation goes here.
           self.price_per_token = price;

       }
   }
}
