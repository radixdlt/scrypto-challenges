use scrypto::prelude::*;
use crate::bond::BondToken;
use crate::order::Order;

#[derive(NonFungibleData)]
pub struct SellTicket {
    order_number: u64,
    price: Decimal,
    bond_issuer_nft_address: ResourceAddress,
}

// No AMMs
blueprint! {
    struct BondMarket {
        bonds: LazyMap<u64, ComponentAddress>, // Issuer details w Vault of Bonds, Vault of XRD to repay
        market: HashMap<u64, ComponentAddress>, // Bond addr mapped to Order details
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

        // Allow user to register themselves on the market and recieve 
        // unique user ID NFT
        // pub fn register(&mut self) -> Bucket {

            // let beneficiary_id: NonFungibleId =
            //     NonFungibleId::from_u64((self.funds.len() + self.dead_vaults.len()) as u64 + 1u64);
            // let beneficiary_badge: Bucket = self.internal_admin_badge.authorize(|| {
            //     borrow_resource_manager!(self.beneficiary_vesting_badge).mint_non_fungible(
            //         &beneficiary_id,
            //         BeneficiaryVestingSchedule::new(
            //             relative_cliff_epoch,
            //             relative_ending_epoch,
            //             funds.amount(),
            //             percentage_available_on_cliff,
            //         ),
            //     )
            // });
        // }

        // Allow user to issue a bond to sell on the market. 
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

            let bond_address = bonds.resource_address();
            let bond_resource_manager: &ResourceManager = borrow_resource_manager!(bond_address);
            let bond_id = bond_resource_manager.metadata().get("bond_id").parse::<u64>();


            let seller_badge_data = Vec::new();
            seller_badge_data.push((NonFungibleId::random(),
                SellTicket { order_number: order_count, price: price, bond_issuer_nft_address: bond_address }));

            let seller_badge: Bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "Sell order badge to withdraw money when sold")
                .burnable(rule!(require(self.internal_admin_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(require(self.internal_admin_badge.resource_address())), LOCKED)
                .initial_supply(seller_badge_data);

            self.order_count += 1;
            self.market.insert(bond_id, 
                Order::new(price, bonds, seller_badge.resource_address()));

            return seller_badge;
        }

        pub fn withdraw_sell(&mut self, sell_proof: Proof) -> Bucket {

            // Proof of posession of seller badge NFT
            let address = sell_proof.resource_address();

            // Assert user's proof contains one of the sold market stuff
            assert!(self.market.contains_key(&address), "No bond found with that proof");

            let corresponding_order_address: ComponentAddress = *self.market.get(&address).unwrap();
            let corresponding_order: Order = corresponding_order_address.into();

            let unclaimed_funds: Bucket = corresponding_order.withdraw(sell_proof);
            return unclaimed_funds;
        }

        pub fn buy_bond(&mut self, bond_address: ResourceAddress, payment: Bucket) -> Bucket {
            let bond_resource_manager: &ResourceManager = borrow_resource_manager!(bond_address);
            let bond_id = bond_resource_manager.metadata().get("bond_id").parse::<u64>();

            // Assert specified bond contains one of the sold market bonds
            assert!(self.market.contains_key(&bond_id), "No bond found with that proof");

            let corresponding_order_address: ComponentAddress = *self.market.get(&bond_id).unwrap();
            let corresponding_order: Order = corresponding_order_address.into();
            assert!(corresponding_order.bond_id);

            return self.market.get_mut(&bond_id).unwrap().2.take_all();
        }

        

        pub fn list_orders(&self, bond_address: ResourceAddress) {

            let mut orders: Vec<ComponentAddress> = (&self.market).values().filter(|o| self.test_bond(&o, bond_address)).collect();
            orders.sort_by_key(|o| o.get_price());
        }



        // pub fn list_orders(&self, buy: bool) {
        //     let title = if buy { "BUY" } else { "SELL" };

        //     info!(" \\====================================================/");
        //     info!("");
        //     info!(" /'''''''''''''''''' {:>4} ORDERS '''''''''''''''''\\", title);
        //     info!(" +======================================================+");
        //     info!(" | #    | Token | {:>7} | Filled | {:>8} | Payment |", kind, store);
        //     info!(" +======================================================+");

        //     let orders = 
        //     let orders = (&self.orders).into_iter().filter(|o| o.is_buy_order() == buy).collect::<Vec<&Order>>();

        // }
    }
}