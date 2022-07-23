use scrypto::prelude::*;

blueprint! {

   struct Token2Poche {
        token_supply: Vault,
        collected_xrd: Vault,
        admin_badge: ResourceDef,
        token_price: Decimal
   }

   impl Token2Poche {
       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
           // Your `new` implementation goes here.
       }

       pub fn buy(&mut self, funds: Bucket) -> Bucket {
           // Your `buy` implementation goes here.
       }

       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
           // Your `withdraw_funds` implementation goes here.
       }

       pub fn change_price(&mut self, price: Decimal) {
           // Your `change_price` implementation goes here.
       }
   }
}

