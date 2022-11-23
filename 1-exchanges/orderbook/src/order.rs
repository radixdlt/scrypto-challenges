//! Contains the order struct and storage collection.
//!
use crate::dex::Dex;
use sbor::*;
use scrypto::prelude::*;
use std::cmp::Ordering;

/// Order are store in a set ordered by its price.
/// Use PriceOrder to store order with the same price.
/// When matched the first order found is return.
#[derive(Debug, Default, TypeId, Encode, Decode, Describe)]
pub struct BTreeSetOrder {
    set: BTreeSet<PriceOrder>,
    order_index: HashMap<u32, Decimal>,
}

impl BTreeSetOrder {
    pub fn find_match_and_take_order<F>(&mut self, find_order: F) -> Option<Order>
    where
        F: Fn(&BTreeSet<PriceOrder>) -> Option<PriceOrder>,
    {
        let found = find_order(&self.set);
        found
            .as_ref()
            .and_then(|order| self.take_order_with_price(order))
    }

    fn take_order_with_price(&mut self, order: &PriceOrder) -> Option<Order> {
        //rm not get_mut in BtreeSet
        let found = self.set.take(order);
        found.map(|mut found_order| {
            let ret = found_order.take_next_order();
            if found_order.is_empty() {
                self.order_index.remove(&ret.id);
                ret
            } else {
                self.set.insert(found_order);
                ret
            }
        })
    }

    pub fn insert(&mut self, order: Order) {
        let empty = PriceOrder::new(order.price);
        let mut price_order = if self.set.contains(&empty) {
            self.set.take(&empty).unwrap()
        } else {
            empty
        };
        self.order_index.insert(order.id, order.price);
        price_order.order_list.push(order);
        self.set.insert(price_order);
    }

    pub fn take_order_with_id(&mut self, id: u32) -> Option<Order> {
        self.order_index.remove(&id).and_then(|price| {
            let order = PriceOrder::new(price);
            let found = self.set.take(&order);
            found
                .map(|mut found_order| {
                    found_order.take_order_with_id(id).map(|removed_order| {
                        if found_order.is_empty() {
                            self.order_index.remove(&removed_order.id);
                            removed_order
                        } else {
                            self.set.insert(found_order);
                            removed_order
                        }
                    })
                })
                .flatten()
        })
    }
}

#[derive(Debug, Clone, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub struct PriceOrder {
    pub price: Decimal,
    pub order_list: Vec<Order>,
}

impl Default for PriceOrder {
    fn default() -> Self {
        PriceOrder::new(Decimal::zero())
    }
}

impl PriceOrder {
    pub fn new(price: Decimal) -> Self {
        PriceOrder {
            price: price,
            order_list: vec![],
        }
    }
    pub fn take_next_order(&mut self) -> Order {
        self.order_list.remove(0)
    }

    pub fn is_empty(&self) -> bool {
        return self.order_list.is_empty();
    }

    pub fn take_order_with_id(&mut self, id: u32) -> Option<Order> {
        self.order_list
            .iter()
            .position(|o| o.id == id)
            .map(|pos| self.order_list.remove(pos))
    }
}

impl Ord for PriceOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for PriceOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, TypeId, Encode, Decode, Describe)]
pub struct OrdersContainer {
    pub bids: BTreeSetOrder,
    pub asks: BTreeSetOrder,
}

/// Contains the user Vaults when transferred asset are put
/// Store in a Lazy map indexed with the user badge UID.
#[derive(Debug, TypeId, Encode, Decode, Describe, NonFungibleData)]
pub struct UserOrders {
    pub quote_vault: Vault,
    pub locked_quote_vault: Vault,
    pub base_vault: Vault,
    pub locked_base_vault: Vault,
}

impl UserOrders {
    pub fn new(quote: ResourceDef, base: ResourceDef) -> Self {
        Self {
            quote_vault: Vault::new(quote.clone()),
            locked_quote_vault: Vault::new(quote),
            base_vault: Vault::new(base.clone()),
            locked_base_vault: Vault::new(base),
        }
    }

    pub fn get_user_orders(owner: &NonFungibleKey, dex: &Dex) -> UserOrders {
        dex.user_orders.get(owner).unwrap_or_else(|| {
            panic!("Badge provided not declared call create_openorders to get one")
        })
    }
}
#[derive(Debug, Clone, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub struct Order {
    pub id: u32,
    pub owner: NonFungibleKey,
    pub price: Decimal,
    pub amount: Decimal, //amount in base to trade.
    pub locked_amount: Decimal,
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

impl From<u8> for OrderType {
    fn from(order_type: u8) -> Self {
        match order_type {
            1 => OrderType::ImmediateOrCancel,
            2 => OrderType::PostOnly,
            _ => OrderType::Limit,
        }
    }
}
