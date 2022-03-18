// Licensed under the Apache 2.0 open-source licence https://www.apache.org/licenses/LICENSE-2.0

use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct Order {
    is_buy: bool,           // is b for a
    price: Decimal,         // a per b
    #[scrypto(mutable)]
    amount: Decimal,        // a to buy/sell
    #[scrypto(mutable)]
    filled: Decimal,        // amount of order filled
    #[scrypto(mutable)]
    next: NonFungibleKey    // key of next order in buy/sell chain
}

blueprint! {
    struct ChainBook {
        name: String,
        admin_badge: Vault,         // mint/update/burn badge
        order_def: ResourceDef,
        a_pool: Vault,
        b_pool: Vault,
        sell_head: NonFungibleKey,  // key of first in sell chain
        buy_head: NonFungibleKey,   // key of first in buy chain
        count: u128,                // used to create new order key
    }

    impl ChainBook {
        pub fn instantiate_chain_book(name: String, a_token_address: Address, b_token_address: Address) -> Component {
            let a_def: ResourceDef = a_token_address.into();
            let b_def: ResourceDef = b_token_address.into();

            let admin_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .initial_supply_fungible(1);
            let order_def: ResourceDef = ResourceBuilder::new_non_fungible()
                .metadata("name", "Order")
                .metadata("symbol", "ODR")
                .flags(MINTABLE | BURNABLE | INDIVIDUAL_METADATA_MUTABLE)
                .badge(admin_badge.resource_address(), MAY_MINT | MAY_BURN | MAY_CHANGE_INDIVIDUAL_METADATA)
                .no_initial_supply();

            Self {
                name: name,
                admin_badge: Vault::with_bucket(admin_badge),
                order_def: order_def,
                a_pool: Vault::new(a_def),
                b_pool: Vault::new(b_def),
                sell_head: 0.into(),
                buy_head: 0.into(),
                count: 0,
            }
            .instantiate()
        }

        // fills orders with a better price than asking then creates a limit order with the remaining tokens
        // returns (tokens, ?order)
        pub fn create_order(&mut self, mut tokens: Bucket, price: Decimal) -> (Bucket, Bucket) {
            assert!(
                tokens.resource_def() == self.a_pool.resource_def() || tokens.resource_def() == self.b_pool.resource_def(),
                "Tokens not in this pair"
            );

            self.count += 1;
            let order_key: NonFungibleKey = self.count.into();
            let is_buy: bool = tokens.resource_def() == self.b_pool.resource_def();
            
            if is_buy { // b for a (buy)
                let mut filled: Bucket = Bucket::new(self.a_pool.resource_def());
                // fill colisions (market buy)
                {
                    let mut sell_pointer: NonFungibleKey = self.sell_head.clone();

                    // market buy till price too high
                    while tokens.amount() != 0.into() && sell_pointer != 0.into() && price >= self.order_def.get_non_fungible_data::<Order>(&sell_pointer).price {
                        let mut data: Order = self.order_def.get_non_fungible_data::<Order>(&sell_pointer);
                        
                        // min(can_buy, in_order)
                        let to_fill: Decimal = if tokens.amount() / data.price >= data.amount {
                            self.sell_head = data.next.clone();
                            data.amount
                        } else {
                            tokens.amount() / data.price
                        };

                        // update state
                        self.b_pool.put(tokens.take(to_fill * data.price));
                        data.amount -= to_fill;
                        data.filled += to_fill;
                        filled.put(self.a_pool.take(to_fill));
                        
                        self.admin_badge.authorize(|auth| {
                            self.order_def.update_non_fungible_data(&sell_pointer, data, auth);
                        });

                        // move to next order
                        sell_pointer = self.order_def.get_non_fungible_data::<Order>(&sell_pointer).next;
                    }
                }

                // make order (limit buy)
                let to_order: Decimal = tokens.amount() / price;
                self.b_pool.put(tokens);

                if to_order > 0.into() {
                    let next: NonFungibleKey = if self.buy_head == 0.into() {
                        // if no other orders, point head to new order
                        self.buy_head = order_key.clone();
                        0.into()
                    } else if price > self.order_def.get_non_fungible_data::<Order>(&self.buy_head).price {
                        // if best price put at front and point head to new order
                        let buy_pointer: NonFungibleKey = self.buy_head.clone();
                        self.buy_head = order_key.clone();
                        buy_pointer
                    } else {
                        let mut buy_pointer: NonFungibleKey = self.buy_head.clone();
                        let mut next : NonFungibleKey = self.order_def.get_non_fungible_data::<Order>(&buy_pointer).next;

                        // find orders to place new order between
                        while next != 0.into() && price <= self.order_def.get_non_fungible_data::<Order>(&next).price {
                            buy_pointer = next;
                            next = self.order_def.get_non_fungible_data::<Order>(&buy_pointer).next;
                        }

                        // update prev order to point to new order
                        let mut data: Order = self.order_def.get_non_fungible_data::<Order>(&buy_pointer);
                        data.next = order_key.clone();

                        self.admin_badge.authorize(|auth| {
                            self.order_def.update_non_fungible_data(&buy_pointer, data, auth);
                        });

                        next
                    };
                    // return (filled, order)
                    (
                        filled,
                        self.admin_badge.authorize(|auth| {
                            self.order_def.mint_non_fungible(&order_key,
                                Order {
                                    is_buy: is_buy,
                                    price: price,
                                    amount: to_order,
                                    filled: 0.into(),
                                    next: next,
                                },
                                auth)
                        })
                    )
                } else {
                    // return (filled, empty bucket)
                    (filled, Bucket::new(self.order_def.clone()))
                }
            } else { // a for b (sell)
                let mut filled: Bucket = Bucket::new(self.b_pool.resource_def());
                // fill colisions (market sell)
                {
                    let mut buy_pointer: NonFungibleKey = self.buy_head.clone();

                    // market sell till price too low
                    while tokens.amount() != 0.into() && buy_pointer != 0.into() && price <= self.order_def.get_non_fungible_data::<Order>(&buy_pointer).price {
                        let mut data: Order = self.order_def.get_non_fungible_data::<Order>(&buy_pointer);
                        
                        // min(can_sell, in_order)
                        let to_fill: Decimal = if tokens.amount() >= data.amount {
                            self.buy_head = data.next.clone();
                            data.amount
                        } else {
                            tokens.amount()
                        };

                        // update state
                        self.a_pool.put(tokens.take(to_fill));
                        data.amount -= to_fill;
                        data.filled += to_fill;
                        filled.put(self.b_pool.take(to_fill * data.price));

                        self.admin_badge.authorize(|auth| {
                            self.order_def.update_non_fungible_data(&buy_pointer, data, auth);
                        });
                        
                        // move to next order
                        buy_pointer = self.order_def.get_non_fungible_data::<Order>(&buy_pointer).next;
                    }
                }

                // make order (limit sell)
                let to_order: Decimal = tokens.amount();
                self.a_pool.put(tokens);

                if to_order > 0.into() {
                    let next: NonFungibleKey = if self.sell_head == 0.into() {
                        // if no other orders, point head to new order
                        self.sell_head = order_key.clone();
                        0.into()
                    } else if price < self.order_def.get_non_fungible_data::<Order>(&self.sell_head).price {
                        // if best price put at front and point head to new order
                        let sell_pointer: NonFungibleKey = self.sell_head.clone();
                        self.sell_head = order_key.clone();
                        sell_pointer
                    } else {
                        let mut sell_pointer: NonFungibleKey = self.sell_head.clone();
                        let mut next : NonFungibleKey = self.order_def.get_non_fungible_data::<Order>(&sell_pointer).next;

                        // find orders to place new order between
                        while next != 0.into() && price >= self.order_def.get_non_fungible_data::<Order>(&next).price {
                            sell_pointer = next;
                            next = self.order_def.get_non_fungible_data::<Order>(&sell_pointer).next;
                        }

                        // update prev order to point to new order
                        let mut data: Order = self.order_def.get_non_fungible_data::<Order>(&sell_pointer);
                        let next: NonFungibleKey = data.next.clone();
                        data.next = order_key.clone();
                        
                        self.admin_badge.authorize(|auth| {
                            self.order_def.update_non_fungible_data(&sell_pointer, data, auth);
                        });

                        next
                    };
                    // return (filled, order)
                    (   
                        filled,
                        self.admin_badge.authorize(|auth| {
                            self.order_def.mint_non_fungible(&order_key,
                                Order {
                                    is_buy: is_buy,
                                    price: price,
                                    amount: to_order,
                                    filled: 0.into(),
                                    next: next,
                                },
                                auth)
                        })
                    )
                } else {
                    // return (filled, empty bucket)
                    (filled, Bucket::new(self.order_def.clone()))
                }
            }
        }

        // gives tokens for filled part of order, burns order if completed else returns updated order
        // returns (tokens, ?order)
        pub fn claim_tokens(&mut self, mut order: Bucket) -> (Bucket, Bucket) {
            assert!(
                order.resource_def() == self.order_def,
                "Order not for this pair"
            );

            let order_key: NonFungibleKey = order.get_non_fungible_key();
            let mut data: Order = self.order_def.get_non_fungible_data::<Order>(&order_key);
            
            if data.filled > 0.into() {
                // get tokens owed
                let tokens: Bucket = if data.is_buy {
                    self.a_pool.take(data.filled)
                } else {
                    self.b_pool.take(data.filled * data.price)
                };

                // burn order if completed else update
                if data.amount == 0.into() {
                    self.admin_badge.authorize(|auth| {
                        order.take_non_fungible(&order_key).burn_with_auth(auth);
                    });
                } else {
                    data.filled = 0.into();
                    self.admin_badge.authorize(|auth| {
                        self.order_def.update_non_fungible_data(&order_key, data, auth);
                    });
                }

                (tokens, order)
            } else {
                // if nothing to claim return (empty bucket, order)
                (
                    if data.is_buy {
                        Bucket::new(self.a_pool.resource_def())
                    } else {
                        Bucket::new(self.b_pool.resource_def())
                    },
                    order
                )
            }
        }

        // claims filled part of order then refunds remaining part of order
        // return (remaining_tokens, filled_tokens)
        pub fn cancel_order(&mut self, order: Bucket) -> (Bucket, Bucket) {
            assert!(
                order.resource_def() == self.order_def,
                "Order not for this pair"
            );
            
            let order_key: NonFungibleKey = order.get_non_fungible_key();
            let data: Order = self.order_def.get_non_fungible_data::<Order>(&order_key);
            
            // claim filled part of order if any
            let (filled_tokens, claimed_order): (Bucket, Bucket) =  self.claim_tokens(order);
            
            // remove order from order chain and get remaining part of order if any
            let remaining_tokens: Bucket = if data.amount > 0.into() {
                let mut curr: NonFungibleKey = if data.is_buy {
                    self.buy_head.clone()
                } else {
                    self.sell_head.clone()
                };
                let mut next: NonFungibleKey = self.order_def.get_non_fungible_data::<Order>(&curr).next;

                // find order that points to order to cancel
                while next != order_key {
                    curr = next;
                    next = self.order_def.get_non_fungible_data::<Order>(&curr).next;
                }

                // update order that points to order to cancel
                let mut curr_data: Order = self.order_def.get_non_fungible_data::<Order>(&curr);
                curr_data.next = data.next;

                self.admin_badge.authorize(|auth| {
                    self.order_def.update_non_fungible_data(&curr, curr_data, auth);
                });
                
                // return remaining tokens
                if data.is_buy {
                    self.b_pool.take(data.amount * data.price)
                } else {
                    self.a_pool.take(data.amount)
                }
            } else {
                // return empty bucket
                if data.is_buy {
                    Bucket::new(self.b_pool.resource_def())
                } else {
                    Bucket::new(self.a_pool.resource_def())
                }
            };

            // burn order
            self.admin_badge.authorize(|auth| {
                claimed_order.burn_with_auth(auth);
            });

            (remaining_tokens, filled_tokens)
        }
    }
}
