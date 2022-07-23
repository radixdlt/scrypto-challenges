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
       }

       pub fn buy(&mut self, funds: Bucket) -> Bucket {
		let amount : Decimal = funds.amount() / self.token_price;
		self.collected_xrd.put(funds.take(funds.amount()));
		return self.collected_xrd.take(amount)
       }

       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
		return self.collected_xrd.take(amount)
       }

       pub fn change_price(&mut self, price: Decimal) {
		self.token_price = price;
       }
   }
}

