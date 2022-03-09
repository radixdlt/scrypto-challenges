use sbor::*;
use scrypto::prelude::*;
use std::cmp::Ordering;

mod dex;

blueprint! {

    pub struct Market {
        //manage open orders user access
        orders_badge_minter: Vault,
        orders_badge_def: ResourceDef,

        // Define market data
        pub quote_token: ResourceDef,
        pub base_token: ResourceDef,
        pub open_orders: LazyMap<NonFungibleKey, OpenOrders>,
        pub bids: BTreeSet<Order>,
        pub asks: BTreeSet<Order>,
        name: String,
        counter: u32,
    }

    impl Market {
        // create a market with specified quote and base token as initiale quantity in the vault.
        pub fn instantiate_market(
            quote_token: Address,
            base_token: Address,
            name: String,
        ) -> (Component, Bucket) {
            // Create a badge for internal use which will hold mint/burn authority for the admin badge we will soon create
            let orders_badge_minter: Bucket =
                ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", &format!("market:{}", name))
                .initial_supply_fungible(1);

            // Define the admin badge
            let admin_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", &format!("Market {} access Badge", name))
                .initial_supply_fungible(1);


            // Create the ResourceDef for a mutable supply admin badge
            let orders_badge_def = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone())
                .flags(MINTABLE )
                .badge(orders_badge_minter.resource_def(), MAY_MINT)
                .no_initial_supply();

            (
                Self {
                    orders_badge_minter: Vault::with_bucket(orders_badge_minter),
                    orders_badge_def,
                    name,
                    quote_token: ResourceDef::from(quote_token),
                    base_token: ResourceDef::from(base_token),
                    open_orders: LazyMap::new(),
                    counter: 0,
                    bids: BTreeSet::new(),
                    asks: BTreeSet::new(),
                }
                .instantiate(),
                admin_badge
            )
        }

        pub fn create_openorders(&mut self) -> Bucket {
            let badge = self.orders_badge_minter.authorize(|auth| {
                self.orders_badge_def.mint_non_fungible(
                    &NonFungibleKey::from(Uuid::generate()),
                    BadgeData {
                        name: self.name.clone(),
                    },
                    auth,
                )
            });

            info!("badge:{:?}", badge.get_non_fungible_keys().get(0).unwrap());

            let orders = OpenOrders::new(self.quote_token.clone(), self.base_token.clone());
            self.open_orders.insert(
                badge.get_non_fungible_keys().get(0).unwrap().clone(),
                orders,
            );

            badge
        }

        //#[auth(orders_badge_def)]
        pub fn buy_order(&mut self, price: Decimal, amount: Decimal, mut quote: Bucket, auth: Bucket) -> (u32, Bucket) {
            info!("buy_order");
            let mut open_order = self.open_orders
                .get(&auth.get_non_fungible_keys().get(0).unwrap())
                //panic because I can't fond how to return an error managed by the VM.
                .ok_or_else(|| panic!("Badge provided not declared call create_openorders to get one")).unwrap();

            let order_type = OrderType::Limit;
            let data: BadgeData = auth
                .resource_def()
                .get_non_fungible_data(auth.get_non_fungible_keys().get(0).unwrap());
            assert!(data.name == self.name, "Not current market open order badge");

            //calculate amount to remove from quote
            let open_order_quote_amount = open_order.quote_vault.amount();
            let mut quote_amount = price*amount;
            let vault_remove_amount = std::cmp::min(open_order_quote_amount, quote_amount);
            let provided_amount = quote.amount();
            //verify there 're enougth quote to lock.
            assert!(provided_amount >= (quote_amount - vault_remove_amount), "Not enougth quote provided.");

            let id = self.counter;
            self.counter +=1;
            let order = Order {
                id,
                price,
                amount,
                provided_amount,
                order_type: OrderType::Limit,
            };


            info!("before non fungible");

            //remove from quote
            if !open_order.quote_vault.is_empty() {
                let remove = std::cmp::min(open_order_quote_amount, quote_amount);
                open_order.base_vault.put(open_order.quote_vault.take(remove));
                quote_amount -= remove;
            }

            if quote_amount.0 > 0 {
                open_order.base_vault.put(quote.take(quote_amount));
            }

            //try to match order
/*            match order_type {
                OrderType::Limit => {
                    find_by_side(&self.asks, Side::Ask)
                        .map(|ask_order| if ask_order.price <= price {
                            //get amunt that can be tranfered now
                            let base_transfert_amount = if amount >= ask_order.amount {
                                amount
                                //remove teh amount to the base order.
                            } else {
                                ask_order.amount
                                //a new order fot the diff and add it to openorders.
                            };
                            //transfert the amount is base. Use ask price to calculate the quote to get.
                            //return the rest from blocked quote to initial acconut.
                            base_transfert_amount
                        } else {
                            Decimal(0)
                        }).unwrap_or_else(||
                        {
                            //manage the case there is no order to match
                            //create a new order and store it.
                            //transfert all quote to vault: base amout=nt = quote * limit price
                            Decimal(0)
                        });
                },
                OrderType::ImmediateOrCancel =>(),
                OrderType::PostOnly =>(),
            }
*/

            //save order
            self.open_orders
                .get(&auth.get_non_fungible_keys().get(0).unwrap())
                .map(|mut open_orders| open_orders.orders.push(order));
            (id,quote)
        }
    }

}

fn find_by_side<T: std::cmp::Ord>(set: &BTreeSet<T>, side: Side) -> Option<&T> {
    match side {
        Side::Bid => set.iter().max(), //max
        Side::Ask => set.iter().min(), //min
    }
}

#[derive(Debug, Clone, NonFungibleData)]
pub struct BadgeData {
    name: String,
}

#[derive(Debug, TypeId, Encode, Decode, Describe, NonFungibleData)]
pub struct OpenOrders {
    pub quote_vault: Vault,
    pub base_vault: Vault,
    pub orders: Vec<Order>,
}

impl OpenOrders {
    pub fn new(quote: ResourceDef, base: ResourceDef) -> Self {
        Self {
            orders: vec![],
            quote_vault: Vault::new(quote),
            base_vault: Vault::new(base),
        }
    }
}

#[derive(Debug, Clone, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub struct Order {
    pub id: u32,
    pub price: Decimal,
    pub amount: Decimal, //amount in base to trade.
    pub provided_amount: Decimal,
    pub order_type: OrderType,
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    ImmediateOrCancel,
    PostOnly,
}
