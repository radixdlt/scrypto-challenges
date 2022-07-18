use scrypto::prelude::*;
use std::cmp::Ordering;

blueprint! {
    #[derive(Debug, sbor::Decode, sbor::Encode, sbor::Describe, sbor::TypeId)]
    pub struct Order {
        pub price: Decimal,
        bond_store: Vault,
        payment: Vault,
        bond_address: ResourceAddress,
    }

    impl Order {
        
        pub fn new(price: Decimal, bonds: Bucket, withdraw_nft: ResourceAddress) -> ComponentAddress {

            let rules: AccessRules = AccessRules::new()
                .method("withdraw", rule!(require(withdraw_nft)));

            let component = Self {
                price: price,
                bond_store: Vault::with_bucket(bonds),
                payment: Vault::new(RADIX_TOKEN),
                bond_address: bonds.resource_address(),
            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            return component;
        }

        pub fn withdraw(&mut self, proof: Proof) -> Bucket{
            return self.payment.take_all();
        }

        pub fn is_this_bond(&self, bond: ResourceAddress) -> bool {
            return bond==self.bond_address;
        }

        pub fn get_price(&self) -> Decimal {
            return self.price;
        }


    }


}