use scrypto::prelude::*;

blueprint! {
    #[derive(Debug, sbor::Decode, sbor::Encode, sbor::Describe, sbor::TypeId)]
    pub struct  Order {
        pub price: Decimal, // Price per bond
        bond_store: Vault, // Contains n buckets
        bond_count: u32,
        tvl: Decimal,
        payment: Vault,
        bond_address: ResourceAddress,
        withdraw_nft: ResourceAddress,
        order_id: u64,
    }

    impl Order {
        
        pub fn new(price: Decimal, bonds: Bucket, withdraw_nft: ResourceAddress, order_id: u64) -> ComponentAddress {

            let rules: AccessRules = AccessRules::new()
                .method("withdraw", rule!(require(withdraw_nft)));
            
            let bond_amount_dec:Decimal = bonds.amount();
            let bond_amount:u32 = bond_amount_dec.to_string().parse::<u32>().unwrap();
            let bond_address = bonds.resource_address();

            let component = Self {
                price: price,
                bond_store: Vault::with_bucket(bonds),
                bond_count: bond_amount,
                tvl: price * Decimal::from(bond_amount),
                payment: Vault::new(RADIX_TOKEN),
                bond_address: bond_address,
                withdraw_nft: withdraw_nft,
                order_id: order_id,
            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            return component;
        }

        pub fn withdraw(&mut self, proof: Proof) -> Bucket {
            if proof.resource_address() == self.withdraw_nft {
                return self.payment.take_all();
            } else {
                return Bucket::new(RADIX_TOKEN);
            }
        }

        // Return the (bond, change) 
        pub fn buy_bond(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            // If insufficient funds to buy this bond, return empty bucket and change
            if payment.amount() < self.price {
                return (Bucket::new(self.bond_address), payment);
            }


            let purchase_amount;
            let purchase_number;
            if payment.amount() > self.tvl {
                purchase_amount = self.tvl;
                purchase_number = Decimal::from(self.bond_count);
            } else{
                purchase_number = (payment.amount() / self.price).floor();
                purchase_amount = purchase_number * self.price;
            }

            let payment_taken: Bucket = payment.take(purchase_amount);
            self.payment.put(payment_taken);
            return (self.bond_store.take(purchase_number), payment);
        }

        pub fn is_this_bond(&self, bond: ResourceAddress) -> bool {
            return bond==self.bond_address;
        }

        pub fn get_price(&self) -> Decimal {
            return self.price;
        }


    }


}