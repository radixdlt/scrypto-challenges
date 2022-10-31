use scrypto::prelude::*;
use crate::bond::BondToken;
use crate::order::Order;

#[derive(NonFungibleData)]
pub struct SellTicket {
    order_number: u64,
    price: Decimal,
    bond_id: u64,
}

// No AMMs, future work.
blueprint! {
    struct BondMarket {
        bonds: LazyMap<u64, ComponentAddress>, // Issuer details w Vault of Bonds, Vault of XRD to repay
        market: HashMap<u64, HashMap<u64, (Decimal, ComponentAddress)>>, // Bond id mapped to Order details
        dead_vaults: Vec<Vault>, // For getting rid of dead vaults
        order_count: u64,   // For getting count of orders
        issuer_count: u64,  // For getting number of bonds issued in market
        account_count: u64, // For keeping track of credit rating NFT IDs
        internal_admin_badge: Vault,
    }

    impl BondMarket {

        // Instantiate BondMarket component
        pub fn instantiate_bond_market() -> ComponentAddress {
        
            let internal_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Bond Market Internal Admin Badge")
                .metadata(
                    "description",
                    "A badge used by bond market to mint and burn other badges",
                )
                .initial_supply(dec!("1"));

            // TODO: Add access rules 

            // Bond Market Instantiation
            let bond_market = Self {
                bonds: LazyMap::new(),
                market: HashMap::new(),
                dead_vaults: Vec::new(),
                order_count: 0,
                issuer_count: 0,
                account_count: 0,
                internal_admin_badge: Vault::with_bucket(internal_admin_badge),
            }
            .instantiate()
            .globalize();

            return bond_market
        }

        // Allow user to issue a bo&nd to sell on the market. 
        // Returns a bucket that contains the issuer badge NFT, 
        pub fn issue_bond(&mut self, face_value: Decimal, coupon_epoch: u64, 
            maturity_epoch: u64, coupon_rate: Decimal, issue_price: Decimal, supply:u32) -> Bucket {
            
            // BACKLOG: Add coupon epoch check

            let (new_bond_component, issuer_badge) = BondToken::instantiate_bond(
                self.issuer_count, face_value, coupon_epoch, maturity_epoch,
                coupon_rate, issue_price, supply
            );

            self.bonds.insert(self.issuer_count, new_bond_component);                
            self.issuer_count += 1;
            
            return issuer_badge;
        }

        pub fn sell_bond(&mut self, bonds: Bucket, price: Decimal) -> Bucket {

            assert!(bonds.amount() > Decimal::zero(), "Did not provide any bonds");

            let order_count:u64 = self.order_count;

            // Get the bond details: Particularly care about bond_address, and bond_id
            let bond_address = bonds.resource_address();
            let bond_resource_manager: &ResourceManager = borrow_resource_manager!(bond_address);
            let bond_metadata:HashMap<String, String> = bond_resource_manager.metadata();
            let bond_id_str: String = bond_metadata.get("bond_id").unwrap().to_string();
            let bond_id: u64 = bond_id_str.parse::<u64>().unwrap();
            
            // Create seller badge NFT data encoding
            let mut seller_badge_data = Vec::new();
            seller_badge_data.push((NonFungibleId::random(),
                SellTicket { order_number: order_count, 
                             price: price, 
                             bond_id: bond_id }));

            // Create seller badge NFT based on seller_badge_data
            let seller_badge: Bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "Sell order badge to withdraw money when sold")
                .burnable(rule!(require(self.internal_admin_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(require(self.internal_admin_badge.resource_address())), LOCKED)
                .initial_supply(seller_badge_data);

            // Add bond to market listing            
            self.market.insert(bond_id, 
                HashMap::from([(order_count, (price,Order::new(price, bonds, seller_badge.resource_address(), order_count)))]));   
            self.order_count += 1;
            return seller_badge;
        }

        pub fn withdraw_sell(&mut self, sell_proof: Proof) -> Bucket {

            // Get sell_id from proof of posession of seller badge NFT
            let sell_ticket: SellTicket = sell_proof.non_fungible::<>().data();

            
            // Assert user's proof contains one of the sold market stuff
            assert!(self.market.contains_key(&sell_ticket.bond_id), "No bond found with that proof");
            
            let bond_listings: &HashMap<u64, (Decimal, ComponentAddress)> = &*self.market.get(&sell_ticket.bond_id).unwrap();

            let (_, corresponding_order_address) = bond_listings.get(&sell_ticket.order_number).unwrap();
            let corresponding_order: Order = (*corresponding_order_address).into();

            let unclaimed_funds: Bucket = corresponding_order.withdraw(sell_proof);
            return unclaimed_funds;
        }

        fn get_bond_id(&self, bond_address: ResourceAddress) -> u64 {
            // Retrieve bond_id based on the bond resourceadress
            let bond_resource_manager: &ResourceManager = borrow_resource_manager!(bond_address);
            let bond_id: u64 = bond_resource_manager.metadata().get("bond_id").unwrap().parse::<u64>().unwrap();

            // Assert specified bond contains one of the sold market bonds
            assert!(self.market.contains_key(&bond_id), "No bond found with that address");

            return bond_id;
        }

        fn get_bond_id_and_listings(&self, bond_address: ResourceAddress) -> (u64, BTreeMap<Decimal, ComponentAddress>) {
            // Get the bond_id from the resource address
            let bond_id = self.get_bond_id(bond_address);

            // Get the listing of that specific bond
            let bond_listings: &HashMap<u64, (Decimal, ComponentAddress)> = &*self.market.get(&bond_id).unwrap();

            // Sort the bond order listings by price with BTreeMap
            let listings_sorted: BTreeMap<Decimal, ComponentAddress> = 
                                bond_listings.iter().map(|(_,v)| (v.0,v.1)).collect();
            
            return (bond_id,listings_sorted);
        } 

        pub fn buy_bond(&mut self, bond_address: ResourceAddress, payment: Bucket) -> Bucket {
           
            // Get bond_id and sorted listings in the market for that bond
            let (_bond_id,listings_sorted) = self.get_bond_id_and_listings(bond_address);

            // All the bonds purchased
            let mut purchased_bonds: Bucket = Bucket::new(bond_address);

            let mut remaining_payment: Bucket = payment;
            // Buy up the bonds until the payment is exhausted
            for (_price, order_addr) in listings_sorted {
                let order: Order = order_addr.into();
                let (bonds, change) = order.buy_bond(remaining_payment); 
                remaining_payment = change;
                purchased_bonds.put(bonds);
            }

            return purchased_bonds;
        }


        // pub fn list_orders(&self, bond_address: ResourceAddress) {

        //     // Get bond_id and sorted listings in the market for that bond
        //     let (bond_id,listings_sorted) = self.get_bond_id_and_listings(bond_address);

        //     info!(" ========================================== ");
        //     info!("");
        //     info!("");
        //     info!(" ========================================== ");
        //     info!(" |''''''''''''''''''''''''''''''''''''''''|");


        // }

    }
}