use sbor::*;
use scrypto::prelude::*;
use scrypto::rust::marker::PhantomData;

pub struct Market {
    //manage open orders user access
    orders_badge_minter: Vault,
    orders_badge_def: ResourceDef,

    // Define market data
    quote_token: ResourceDef,
    base_token: ResourceDef,
    open_orders: LazyMap<NonFungibleKey, UserOrders>,
    bids: BTreeSet<Order>,
    asks: BTreeSet<Order>,
    name: String,
    counter: u32,
}

impl Market {
    fn get_next_id(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        id
    }
}

pub fn bid(
    market: &mut Market,
    owner: NonFungibleKey,
    price: Decimal,
    amount: Decimal,
    mut quote: Bucket,
    order_type: OrderType,
) -> u32 {
    let mut open_order = market
        .open_orders
        .get(&owner)
        //panic because I can't fond how to return an error managed by the VM.
        .ok_or_else(|| panic!("Badge provided not declared call create_openorders to get one"))
        .unwrap();

    let order = Order {
        id: market.get_next_id(),
        owner,
        price,
        amount,
        quote_vault: Vault::new(market.quote_token.clone()),
        base_vault: Vault::new(market.base_token.clone()),
    };
    match order_type {
        OrderType::Limit => {
            find_by_side(&mut market.asks, Side::Ask)
                .map(|ask_order| {
                    if ask_order.price <= price {
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
                    }
                })
                .unwrap_or_else(|| {
                    //manage the case there is no order to match
                    //create a new order and store it.
                    //transfert all quote to vault: base amout=nt = quote * limit price
                    Decimal(0)
                });
        }
        OrderType::ImmediateOrCancel => (),
        OrderType::PostOnly => (),
    }

    0
}

fn find_by_side<T: std::cmp::Ord>(set: &mut BTreeSet<T>, side: Side) -> Option<T> {
    let found = match side {
        Side::Bid => set.iter().max(), //max
        Side::Ask => set.iter().min(), //min
    };
    found.and_then(|order| set.take(order))
}

#[derive(Debug, TypeId, Encode, Decode, Describe, NonFungibleData)]
pub struct UserOrders {
    pub quote_vault: Vault,
    pub base_vault: Vault,
    pub orders: Vec<Order>,
}

impl UserOrders {
    pub fn new(quote: ResourceDef, base: ResourceDef) -> Self {
        Self {
            orders: vec![],
            quote_vault: Vault::new(quote),
            base_vault: Vault::new(base),
        }
    }
}

#[derive(Debug, TypeId, Encode, Decode, Describe)]
pub struct Order {
    pub id: u32,
    pub owner: NonFungibleKey,
    pub price: Decimal,
    pub amount: Decimal, //amount in base to trade.
    pub quote_vault: Vault,
    pub base_vault: Vault,
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

//Type state logic
const BID_FEE: Decimal = Decimal(10);
const ASK_FEE: Decimal = Decimal(10);

struct BidSide;
struct AskSide;

//state list
struct New;
struct ToMatch;
struct Matched;

struct StateOrder<SIDE, STATE> {
    order: Order,
    fee: Decimal,
    state: PhantomData<(SIDE, STATE)>,
}

impl StateOrder<BidSide, New> {
    fn new(mut order: Order, mut quote: Bucket) -> StateOrder<BidSide, ToMatch> {
        let amount = quote.amount();
        let lock_amount = order.price * order.amount;
        //verify there 're enougth quote to lock.
        assert!(amount >= lock_amount, "Not enougth quote provided.");
        //transfert quote to order account for the match
        order.quote_vault.put(quote.take(amount));
        StateOrder {
            order,
            fee: Decimal(0),
            state: PhantomData,
        }
    }
}
impl StateOrder<BidSide, ToMatch> {
    fn new(stateorder: StateOrder<BidSide, ToMatch>) -> StateOrder<BidSide, Matched> {
        StateOrder {
            order: stateorder.order,
            fee: stateorder.fee,
            state: PhantomData,
        }
    }

    fn match_order(
        mut self,
        mut ask: StateOrder<AskSide, ToMatch>,
    ) -> (StateOrder<BidSide, Matched>, StateOrder<AskSide, Matched>) {
        let (matched_quote_amount, matched_base_amount) = ask.match_amount(&mut self);

        ask.order
            .quote_vault
            .put(self.order.quote_vault.take(matched_quote_amount));
        self.order
            .base_vault
            .put(ask.order.base_vault.take(matched_base_amount));
        (
            StateOrder::<BidSide, ToMatch>::new(self),
            StateOrder::<AskSide, ToMatch>::new(ask),
        )
    }
}

impl StateOrder<AskSide, New> {
    fn new(order: Order) -> StateOrder<AskSide, ToMatch> {
        StateOrder {
            order,
            fee: Decimal(0),
            state: PhantomData,
        }
    }
}

impl StateOrder<AskSide, ToMatch> {
    fn new(stateorder: StateOrder<AskSide, ToMatch>) -> StateOrder<AskSide, Matched> {
        StateOrder {
            order: stateorder.order,
            fee: stateorder.fee,
            state: PhantomData,
        }
    }
    fn match_amount(&mut self, bid: &mut StateOrder<BidSide, ToMatch>) -> (Decimal, Decimal) {
        let base_amount = std::cmp::min(self.order.amount, bid.order.amount);
        bid.order.amount -= base_amount;
        self.order.amount -= base_amount;
        let quote_amount = base_amount * self.order.price;
        (quote_amount, base_amount)
    }
}

fn match_orders(bid: StateOrder<BidSide, ToMatch>, ask: StateOrder<AskSide, ToMatch>) -> () {
    let (bid, ask) = bid.match_order(ask);
}
