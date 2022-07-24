use scrypto::prelude::*;

blueprint! {
	struct TokenSale {
		// The vault where the BasedInLisbonTokens will be stored.
		based_in_lisbon_tokens_vault: Vault,

		// The vault where the xrd payments will be stored.
		xrd_tokens_vault: Vault,

		// The price of a single UsefulToken.
		price_per_token: Decimal
	}

	impl TokenSale {
		pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
			// Creating a token called 'BasedInLisbon'
			let my_bucket: Bucket = ResourceBuilder::new_fungible()
					.metadata("name", "BasedInLisbon")
					.metadata("team-member-1-ticket-number", "4138556559") // Xavier
					.metadata("team-member-2-ticket-number", "4086720419") // Marco
					.metadata("team-member-3-ticket-number", "4145776169") // Francisco
					.metadata("team-member-4-ticket-number", "4116602549") // Carlos
					.divisibility(DIVISIBILITY_MAXIMUM)
					.initial_supply(100000);
					// Creating a new seller badge which we will give the withdraw authority to
					let seller_badge: Bucket = ResourceBuilder::new_fungible()
							.metadata("name", "Seller Badge")
							.metadata("symbol", "SELLER")
							.initial_supply(1);

					// Setting the access rules to only allow the seller badge to withdraw the funds or change the price
					let access_rules: AccessRules = AccessRules::new()
							.method("withdraw_funds", rule!(require(seller_badge.resource_address())))
							.method("change_price", rule!(require(seller_badge.resource_address())))
							.default(rule!(allow_all));

					let component_address: ComponentAddress = Self {
							based_in_lisbon_tokens_vault: Vault::with_bucket(my_bucket),
							xrd_tokens_vault: Vault::new(RADIX_TOKEN),
							price_per_token: price_per_token
					}
					.instantiate()
					.add_access_check(access_rules)
					.globalize();

					return (component_address, seller_badge)
		}

		pub fn buy(&mut self, funds: Bucket) -> Bucket {
			let purchase_amount: Decimal = funds.amount() / self.price_per_token;
			self.xrd_tokens_vault.put(funds);
			self.based_in_lisbon_tokens_vault.take(purchase_amount)
		}

		pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
			self.xrd_tokens_vault.take(amount)
		}

		pub fn change_price(&mut self, price: Decimal) {
			self.price_per_token = price;
		}
	}
}
