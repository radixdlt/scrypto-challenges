use sbor::*;
use scrypto::prelude::*;
use scrypto::rust::marker::PhantomData;
use std::cmp::Ordering;

static MAKER_FEE: usize = 5;
static TAKER_FEE: usize = 10;

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

#[derive(Debug, Default, TypeId, Encode, Decode, Describe)]
pub struct OrdersContainer {
    bids: BTreeSetOrder,
    asks: BTreeSetOrder,
}

#[derive(Debug, TypeId, Encode, Decode, Describe)]
pub struct DexParameters {
    pub quote_token: ResourceDef,
    pub base_token: ResourceDef,
    pub maker_fee: Decimal,
    pub taker_fee: Decimal,
}

#[derive(Debug, TypeId, Encode, Decode, Describe)]
pub struct Dex {
    // Define dex data
    pub fee_quote_vault: Vault,
    pub fee_base_vault: Vault,
    pub containers: OrdersContainer,
    pub user_orders: LazyMap<NonFungibleKey, UserOrders>,
    pub params: DexParameters,
    counter: u32,
}

impl Dex {
    pub fn new(quote_token: Address, base_token: Address) -> Self {
        Dex {
            fee_quote_vault: Vault::new(quote_token),
            fee_base_vault: Vault::new(base_token),
            user_orders: LazyMap::new(),
            counter: 0,
            containers: Default::default(),
            //rm I do it like that because I can't find a way to have const decimal without knowing the u128 value.
            params: DexParameters {
                quote_token: ResourceDef::from(quote_token),
                base_token: ResourceDef::from(base_token),
                maker_fee: From::<usize>::from(MAKER_FEE),
                taker_fee: From::<usize>::from(TAKER_FEE),
            },
        }
    }
    ///
    /// To manage a bid, user must provide enougth quote to buy the base at max limite price.
    /// Provided quote is put User quote vault. Quote vault amount but be greater that needed quote to match at limite price.
    /// Needed quote is locked in a vault for the whole duration of the buy order
    pub fn bid(
        &mut self,
        owner: NonFungibleKey,
        price: Decimal,
        amount: Decimal,
        mut quote: Bucket,
        order_type: OrderType,
    ) -> (Bucket, Bucket) {
        let (mut bid, mut bid_trader_orders) =
            StateOrder::<BidSide, TakerPos>::init_match(owner, price, amount, &mut quote, self);

        let new_id = bid.order.id;

        //manage order type
        match order_type {
            OrderType::Limit | OrderType::ImmediateOrCancel => {
                let (remain_bid, remain_ask) = loop {
                    match match_taker_order(
                        &bid,
                        &mut self.containers.asks,
                        StateOrder::<BidSide, TakerPos>::find_order,
                        StateOrder::<BidSide, TakerPos>::order_match,
                        &self.params,
                    ) {
                        None => break (Some(bid), None), //no match found
                        Some((matched, ask)) => {
                            let mut ask_trader_orders =
                                UserOrders::get_user_orders(&ask.order.owner, &self);
                            let remain_bid = bid.transfer_bid_match(
                                &mut bid_trader_orders,
                                &mut ask_trader_orders,
                                matched.remainder_taker_base,
                                &matched,
                                self,
                            );
                            let remain_ask = ask.transfer_ask_match(
                                &mut bid_trader_orders,
                                &mut ask_trader_orders,
                                matched.remainder_maker_base,
                                &matched,
                                self,
                            );
                            match (remain_bid, remain_ask) {
                                (None, None) => break (None, None),
                                (None, remain_ask) => {
                                    info!("bid remain_ask:{:?}", remain_ask);
                                    //all bid matched
                                    break (None, remain_ask);
                                }
                                (remain_bid, None) => {
                                    info!("bid remain_bid:{:?}", remain_bid);
                                    //continue to match, if ask exist
                                    bid = remain_bid.unwrap();
                                }
                                (Some(_), Some(_)) => {
                                    unreachable!("Double match should not arrive")
                                }
                            }
                        }
                    }
                };
                remain_ask.map(|ask| self.containers.asks.insert(ask.order));
                if let OrderType::Limit = order_type {
                    remain_bid.map(|bid| self.containers.bids.insert(bid.order));
                }
            }
            OrderType::PostOnly => {
                self.containers.bids.insert(bid.order);
            }
        };
        (create_order_id(new_id), quote)
    }

    pub fn ask(
        &mut self,
        owner: NonFungibleKey,
        price: Decimal,
        amount: Decimal,
        mut base: Bucket,
        order_type: OrderType,
    ) -> (Bucket, Bucket) {
        let (mut ask, mut ask_trader_orders) =
            StateOrder::<AskSide, TakerPos>::init_match(owner, price, amount, &mut base, self);
        let new_id = ask.order.id;

        //manage order type
        match order_type {
            OrderType::Limit | OrderType::ImmediateOrCancel => {
                let (remain_bid, remain_ask) = loop {
                    match match_taker_order(
                        &ask,
                        &mut self.containers.bids,
                        StateOrder::<AskSide, TakerPos>::find_order,
                        StateOrder::<AskSide, TakerPos>::order_match,
                        &self.params,
                    ) {
                        None => break (None, Some(ask)), //no match found
                        Some((matched, bid)) => {
                            let mut bid_trader_orders =
                                UserOrders::get_user_orders(&bid.order.owner, &self);
                            let remain_bid = bid.transfer_bid_match(
                                &mut bid_trader_orders,
                                &mut ask_trader_orders,
                                matched.remainder_maker_base,
                                &matched,
                                self,
                            );
                            let remain_ask = ask.transfer_ask_match(
                                &mut bid_trader_orders,
                                &mut ask_trader_orders,
                                matched.remainder_taker_base,
                                &matched,
                                self,
                            );
                            match (remain_bid, remain_ask) {
                                (None, None) => break (None, None),
                                (None, remain_ask) => {
                                    info!("ask remain_ask:{:?}", remain_ask);
                                    ask = remain_ask.unwrap();
                                }
                                (remain_bid, None) => {
                                    info!("ask remain_bid:{:?}", remain_bid);
                                    break (remain_bid, None);
                                }
                                (Some(_), Some(_)) => {
                                    unreachable!("Double match should not arrive")
                                }
                            }
                        }
                    }
                };
                remain_bid.map(|bid| self.containers.bids.insert(bid.order));
                if let OrderType::Limit = order_type {
                    remain_ask.map(|ask| self.containers.asks.insert(ask.order));
                }
            }
            OrderType::PostOnly => {
                self.containers.asks.insert(ask.order);
            }
        };
        (create_order_id(new_id), base)
    }
    pub fn get_next_id(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        id
    }

    pub fn cancel_order(&mut self, order_id_badge: Bucket, owner: NonFungibleKey) -> Bucket {
        let data = order_id_badge.resource_def().metadata();
        let id: u32 = data.get("id").unwrap().parse().unwrap();
        self.containers
            .asks
            .take_order_with_id(id)
            .map(|order| StateOrder::<AskSide, TakerPos>::cancel_order(&order, &owner, &self))
            .or_else(|| {
                self.containers.bids.take_order_with_id(id).map(|order| {
                    StateOrder::<BidSide, TakerPos>::cancel_order(&order, &owner, &self);
                });
                None
            });

        order_id_badge
    }
}

#[derive(Debug, TypeId, Encode, Decode, Describe, NonFungibleData)]
pub struct UserOrders {
    pub quote_vault: Vault,
    pub locked_quote_vault: Vault,
    pub base_vault: Vault,
    pub locked_base_vault: Vault,
    pub orders: Vec<Order>,
}

impl UserOrders {
    pub fn new(quote: ResourceDef, base: ResourceDef) -> Self {
        Self {
            orders: vec![],
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

///Match indicate that a corresponding order has been found
// manage the transfert between the order.
// generate the remaining order to be processed.
#[derive(Debug)]
struct Match {
    transfert_user_base: Decimal,
    transfert_user_quote: Decimal,
    remainder_maker_base: Decimal,
    remainder_taker_base: Decimal,
}

#[derive(Debug)]
struct BidSide;
#[derive(Debug)]
struct AskSide;

#[derive(Debug)]
struct MakerPos;
#[derive(Debug)]
struct TakerPos;

#[derive(Debug)]
struct StateOrder<SIDE, POSITION> {
    order: Order,
    order_fee: Decimal,
    state: PhantomData<(SIDE, POSITION)>,
}

impl<SIDE, POSITION> StateOrder<SIDE, POSITION> {
    fn new_with_order(order: Order, order_fee: Decimal) -> StateOrder<SIDE, POSITION> {
        StateOrder {
            order,
            order_fee,
            state: PhantomData,
        }
    }
}

impl StateOrder<BidSide, TakerPos> {
    fn init_match(
        owner: NonFungibleKey,
        price: Decimal,
        amount: Decimal,
        quote: &mut Bucket,
        dex: &mut Dex,
    ) -> (StateOrder<BidSide, TakerPos>, UserOrders) {
        let mut bid_trader_orders = UserOrders::get_user_orders(&owner, &dex);

        //put quote in user Vault.
        bid_trader_orders
            .quote_vault
            .put(quote.take(quote.amount()));
        //lock enought quote for the order
        let available_amount = bid_trader_orders.quote_vault.amount();
        let locked_amount = price * amount;
        //verify there 're enougth quote to lock.
        assert!(
            available_amount >= locked_amount,
            "Not enougth quote provided."
        );
        bid_trader_orders
            .locked_quote_vault
            .put(bid_trader_orders.quote_vault.take(locked_amount));

        let order = Order {
            id: dex.get_next_id(),
            owner: owner.clone(),
            price,
            amount,
            locked_amount,
        };
        (
            StateOrder::<BidSide, TakerPos>::new_with_order(order, dex.params.taker_fee),
            bid_trader_orders,
        )
    }

    fn cancel_order(order: &Order, owner: &NonFungibleKey, dex: &Dex) {
        let mut user_orders = UserOrders::get_user_orders(&owner, dex);
        //Remove locked quote and base for bid order.
        user_orders
            .quote_vault
            .put(user_orders.locked_quote_vault.take(order.locked_amount));
        info!(
            "cancel_order user_orders.quote_vault:{}, order.locked_amount:{}",
            user_orders.quote_vault.amount(),
            order.locked_amount
        )
    }

    fn order_match(bid_price: Decimal, found_order: &Order) -> bool {
        bid_price >= found_order.price
    }

    fn find_order(set: &BTreeSet<PriceOrder>) -> Option<PriceOrder> {
        set.iter().cloned().min()
    }
}

impl StateOrder<AskSide, TakerPos> {
    fn init_match(
        owner: NonFungibleKey,
        price: Decimal,
        amount: Decimal,
        base: &mut Bucket,
        dex: &mut Dex,
    ) -> (StateOrder<AskSide, TakerPos>, UserOrders) {
        let mut ask_trader_orders = UserOrders::get_user_orders(&owner, &dex);
        //put quote in user Vault.
        ask_trader_orders.base_vault.put(base.take(base.amount()));
        //lock enought quote for the order
        ask_trader_orders
            .locked_base_vault
            .put(ask_trader_orders.base_vault.take(amount));

        let order = Order {
            id: dex.get_next_id(),
            owner: owner.clone(),
            price,
            amount,
            locked_amount: Decimal::zero(),
        };
        (
            StateOrder::<AskSide, TakerPos>::new_with_order(order, dex.params.taker_fee),
            ask_trader_orders,
        )
    }

    fn cancel_order(order: &Order, _owner: &NonFungibleKey, _dex: &Dex) {
        let mut user_orders = UserOrders::get_user_orders(&_owner, _dex);
        //Remove locked quote and base for bid order.
        user_orders
            .base_vault
            .put(user_orders.locked_base_vault.take(order.amount));
    }

    fn order_match(ask_price: Decimal, found_order: &Order) -> bool {
        ask_price <= found_order.price
    }

    fn find_order(set: &BTreeSet<PriceOrder>) -> Option<PriceOrder> {
        set.iter().cloned().max()
    }
}

impl<POSITION> StateOrder<BidSide, POSITION> {
    fn transfer_bid_match(
        mut self,
        bid_trader: &mut UserOrders,
        ask_trader: &mut UserOrders,
        remain_base: Decimal,
        matched: &Match,
        dex: &mut Dex,
    ) -> Option<StateOrder<BidSide, POSITION>> {
        //Calculate fee and transfert matched order in trader's Vaults.
        //for bid side take fee from base because bid receive base.
        let bid_fee_base_amount =
            matched.transfert_user_base * self.order_fee / Into::<Decimal>::into(100);
        dex.fee_base_vault
            .put(ask_trader.locked_base_vault.take(bid_fee_base_amount));

        info!(
            "transfert_match bid matched.transfert_user_base:{} bid bid_fee_base_amout:{} self.order.locked_amount:{}",
            matched.transfert_user_base, bid_fee_base_amount,self.order.locked_amount
        );
        bid_trader.base_vault.put(
            ask_trader
                .locked_base_vault
                .take(matched.transfert_user_base - bid_fee_base_amount),
        );

        //update bid and ask order with remainding
        self.order.amount = remain_base;

        if self.order.amount == Decimal::zero() {
            //remove unnecessary locked quote.
            let diff = self.order.locked_amount - matched.transfert_user_quote;
            info!(
                "transfer_bid_match user_orders.quote_vault:{}, self.order.locked_amount:{} diff:{}",
                bid_trader.locked_quote_vault.amount(),
                self.order.locked_amount,diff
            );
            bid_trader
                .quote_vault
                .put(bid_trader.locked_quote_vault.take(diff));
            self.order.locked_amount = Decimal::zero();
        } else {
            info!(
                "transfer_bid_match matched.transfert_user_quote:{}, self.order.locked_amount:{}",
                matched.transfert_user_quote, self.order.locked_amount
            );
            self.order.locked_amount -= matched.transfert_user_quote;
        }

        (self.order.amount > Decimal::zero()).then(|| self)
    }
}

impl<POSITION> StateOrder<AskSide, POSITION> {
    fn transfer_ask_match(
        mut self,
        bid_trader: &mut UserOrders,
        ask_trader: &mut UserOrders,
        remain_base: Decimal,
        matched: &Match,
        dex: &mut Dex,
    ) -> Option<StateOrder<AskSide, POSITION>> {
        //Calculate fee and transfert matched order in trader's Vaults.
        //for ask side take fee from quote because ask receive quote.
        let ask_fee_base_amount =
            matched.transfert_user_quote * self.order_fee / Into::<Decimal>::into(100);
        dex.fee_quote_vault
            .put(bid_trader.locked_quote_vault.take(ask_fee_base_amount));
        info!(
            "transfert_match ask matched.transfert_user_quote:{} bid bid_fee_base_amout:{}",
            matched.transfert_user_quote, ask_fee_base_amount
        );
        ask_trader.quote_vault.put(
            bid_trader
                .locked_quote_vault
                .take(matched.transfert_user_quote - ask_fee_base_amount),
        );
        info!(
            "transfert_match ask ask_trader.quote_vault:{} matched.remainder_ask_base:{} self.order.amount:{}",
            ask_trader.quote_vault.amount(),
            matched.remainder_taker_base,
            self.order.amount
        );
        //update bid and ask order with remainding
        self.order.amount = remain_base;

        (self.order.amount > Decimal::zero()).then(|| self)
    }
}

fn match_taker_order<SIDE, SIDE2: std::fmt::Debug, F, M>(
    taker: &StateOrder<SIDE, TakerPos>,
    set: &mut BTreeSetOrder,
    find_order: F,
    order_match: M,
    dex: &DexParameters,
) -> Option<(Match, StateOrder<SIDE2, MakerPos>)>
where
    F: Fn(&BTreeSet<PriceOrder>) -> Option<PriceOrder>,
    M: Fn(Decimal, &Order) -> bool,
{
    //BTreeSet min only in Nigthly.
    let matched = set
        .find_match_and_take_order(find_order)
        .and_then(|found_order| {
            if order_match(taker.order.price, &found_order) {
                let match_price = std::cmp::min(taker.order.price, found_order.price);
                let base_to_transfert = std::cmp::min(found_order.amount, taker.order.amount);
                info!(
                    "match_taker_order found_order.amount:{} base_to_transfert:{} taker.order.amount:{}",
                    found_order.amount, base_to_transfert, taker.order.amount
                );

                Some((
                    Match {
                        transfert_user_quote: base_to_transfert * match_price,
                        transfert_user_base: base_to_transfert,
                        remainder_maker_base: found_order.amount - base_to_transfert,
                        remainder_taker_base: taker.order.amount - base_to_transfert,
                    },
                    StateOrder::<SIDE2, MakerPos>::new_with_order(found_order, dex.maker_fee),
                ))
            } else {
                None
            }
        });
    info!("taker matched:{:?}", matched);
    matched
}

fn create_order_id(id: u32) -> Bucket {
    ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
        .metadata("id", id.to_string())
        .flags(BURNABLE | FREELY_BURNABLE)
        .initial_supply_fungible(1)
}
