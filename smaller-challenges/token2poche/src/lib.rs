use scrypto::prelude::*;

blueprint! {

   struct Token2Poche {
        token_supply: Vault,
        collected_xrd: Vault,
        admin_badge: ResourceAddress,
        token_price: Decimal
   }

   impl Token2Poche {
       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            // Create the admin badge bucket
            let init_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name","admin badge")
                .burnable(rule!(allow_all), LOCKED)
                .initial_supply(1);

            // Create the token bucket
            let init_bucket: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Token2Pochent")
                .metadata("persoent -->", "ent")
                .metadata("symbol", "ENT")
                .metadata("team-member-1-ticket-number", "40893648296566556829001")
                .metadata("team-member-2-ticket-number", "40893648296566556829001")
                .initial_supply(100000);

            // Create the access rules
            let rulze: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(init_badge.resource_address())))
                .method("change_price", rule!(require(init_badge.resource_address())))
                .default(rule!(allow_all));

            // Instantiate the component
            let comp = Self {
                token_supply: Vault::with_bucket(init_bucket),
                collected_xrd: Vault::new(RADIX_TOKEN),
                admin_badge: init_badge.resource_address(),
                token_price: price_per_token
            }
            .instantiate()
            .add_access_check(rulze)
            .globalize();

            (comp, init_badge)
       }

       pub fn buy(&mut self, funds: Bucket) -> Bucket {
		let amount : Decimal = funds.amount() / self.token_price;
		self.collected_xrd.put(funds);
		return self.token_supply.take(amount)
       }

       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
		return self.collected_xrd.take(amount)
       }

       pub fn change_price(&mut self, price: Decimal) {
        if price > scrypto::math::Decimal(0)
            {
                self.token_price = price;
            }
       }
   }
}

