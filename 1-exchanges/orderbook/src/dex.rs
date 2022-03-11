use sbor::*;
use scrypto::prelude::*;
use scrypto::rust::marker::PhantomData;
use std::cmp::Ordering;

static MAKER_FEE: usize = 5;
static TAKER_FEE: usize = 10;

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
    pub user_orders: LazyMap<NonFungibleKey, UserOrders>,
    pub bids: BTreeSet<Order>,
    pub asks: BTreeSet<Order>,
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
            bids: BTreeSet::new(),
            asks: BTreeSet::new(),
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
    ) -> (u32, Vec<Bucket>) {
        let mut user_orders = self
            .user_orders
            .get(&owner)
            .ok_or_else(|| panic!("Badge provided not declared call create_openorders to get one"))
            .unwrap();

        //put quote in user Vault.
        user_orders.quote_vault.put(quote.take(quote.amount()));
        //lock enought quote for the order
        let available_amount = user_orders.quote_vault.amount();
        let locked_amount = price * amount;
        //verify there 're enougth quote to lock.
        assert!(
            available_amount >= locked_amount,
            "Not enougth quote provided."
        );
        user_orders
            .locked_quote_vault
            .put(user_orders.quote_vault.take(locked_amount));

        let order = Order {
            id: self.get_next_id(),
            owner: owner.clone(),
            price,
            amount,
            locked_amount,
        };

        info!("bid:{:?}", self.bids.len());
        info!("asks:{:?}", self.asks.len());
        let new_id = order.id;

        //manage order type
        match order_type {
            OrderType::Limit | OrderType::ImmediateOrCancel => {
                let mut bid =
                    StateOrder::<BidSide, TakerPos>::new_with_order(order, self.params.taker_fee);
                let (remain_bid, remain_ask) = loop {
                    match bid.match_bid_order(&mut self.asks, &self.params) {
                        None => break (Some(bid), None), //no match found
                        Some((matched, ask)) => {
                            let mut ask_trader_orders = self
                                .user_orders
                                .get(&ask.order.owner)
                                .unwrap_or_else(|| panic!("Provided badge not declared"));
                            match matched.transfert_match(
                                bid,
                                &mut user_orders,
                                ask,
                                &mut ask_trader_orders,
                                self,
                            ) {
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
                                _ => unreachable!("Double match should not arrive"),
                            }
                        }
                    }
                };
                remain_ask.map(|ask| self.asks.insert(ask.order));
                if let OrderType::Limit = order_type {
                    remain_bid.map(|bid| self.bids.insert(bid.order));
                }
                (new_id, vec![quote])
            }
            OrderType::PostOnly => {
                self.bids.insert(order);
                info!("insert bid:{:?}", self.bids.len());
                (new_id, vec![quote])
            }
        }
    }

    pub fn ask(
        &mut self,
        owner: NonFungibleKey,
        price: Decimal,
        amount: Decimal,
        mut base: Bucket,
        order_type: OrderType,
    ) -> (u32, Vec<Bucket>) {
        let mut user_orders = self
            .user_orders
            .get(&owner)
            .ok_or_else(|| panic!("Badge provided not declared call create_openorders to get one"))
            .unwrap();
        //put quote in user Vault.
        user_orders.base_vault.put(base.take(base.amount()));
        //lock enought quote for the order
        user_orders
            .locked_base_vault
            .put(user_orders.base_vault.take(amount));

        let order = Order {
            id: self.get_next_id(),
            owner: owner.clone(),
            price,
            amount,
            locked_amount: Decimal::zero(),
        };

        let new_id = order.id;

        //manage order type
        match order_type {
            OrderType::Limit | OrderType::ImmediateOrCancel => {
                let mut ask =
                    StateOrder::<AskSide, TakerPos>::new_with_order(order, self.params.taker_fee);
                let (remain_bid, remain_ask) = loop {
                    match ask.match_ask_order(&mut self.bids, &self.params) {
                        None => break (None, Some(ask)), //no match found
                        Some((matched, bid)) => {
                            let mut bid_trader_orders = self
                                .user_orders
                                .get(&bid.order.owner)
                                .unwrap_or_else(|| panic!("Provided badge not declared"));
                            match matched.transfert_match(
                                bid,
                                &mut bid_trader_orders,
                                ask,
                                &mut user_orders,
                                self,
                            ) {
                                (None, remain_ask) => {
                                    info!("ask remain_ask:{:?}", remain_ask);
                                    //apply fee
                                    //continue to match, if bid exist
                                    ask = remain_ask.unwrap();
                                }
                                (remain_bid, None) => {
                                    info!("ask remain_bid:{:?}", remain_bid);
                                    //all ask matched
                                    break (remain_bid, None);
                                }
                                _ => unreachable!("Double match should not arrive"),
                            }
                        }
                    }
                };
                remain_bid.map(|bid| self.bids.insert(bid.order));
                if let OrderType::Limit = order_type {
                    remain_ask.map(|ask| self.asks.insert(ask.order));
                }

                (new_id, vec![base])
            }
            OrderType::PostOnly => {
                self.asks.insert(order);
                (new_id, vec![base])
            }
        }
    }
    pub fn get_next_id(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        id
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

/*fn find_by_side<T: std::cmp::Ord>(set: &mut BTreeSet<T>, side: Side) -> Option<T> {
    let found = match side {
        Side::Bid => set.iter().max(), //max
        Side::Ask => set.iter().min(), //min
    };
    found.and_then(|order| set.take(order))
}*/

///Match indicate that a corresponding order has been found
// manage the transfert between the order.
// generate the remaining order to be processed.
#[derive(Debug)]
struct Match {
    transfert_user_base: Decimal,
    transfert_user_quote: Decimal,
    remainder_ask_base: Decimal,
    remainder_bid_base: Decimal,
}

impl Match {
    fn transfert_match<POSITION1, POSITION2>(
        &self,
        bid: StateOrder<BidSide, POSITION1>,
        bid_trader: &mut UserOrders,
        ask: StateOrder<AskSide, POSITION2>,
        ask_trader: &mut UserOrders,
        dex: &mut Dex,
    ) -> (
        Option<StateOrder<BidSide, POSITION1>>,
        Option<StateOrder<AskSide, POSITION2>>,
    ) {
        (
            bid.transfer_bid_match(bid_trader, ask_trader, self, dex),
            ask.transfer_ask_match(bid_trader, ask_trader, self, dex),
        )
    }
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
    amount_transfered: Decimal,
    state: PhantomData<(SIDE, POSITION)>,
}

impl<SIDE, POSITION> StateOrder<SIDE, POSITION> {
    fn new_with_order(order: Order, order_fee: Decimal) -> StateOrder<SIDE, POSITION> {
        StateOrder {
            order,
            order_fee,
            amount_transfered: Decimal::zero(),
            state: PhantomData,
        }
    }
}

impl StateOrder<BidSide, TakerPos> {
    fn match_bid_order(
        &self,
        set: &mut BTreeSet<Order>,
        dex: &DexParameters,
    ) -> Option<(Match, StateOrder<AskSide, MakerPos>)> {
        //BTreeSet min only in Nigthly.
        let found = { set.iter().cloned().min() };
        let matched = found
            .as_ref()
            .and_then(|order| set.take(order))
            .and_then(|ask| {
                if ask.price <= self.order.price {
                    let base_to_transfert = std::cmp::min(ask.amount, self.order.amount);
                    Some((
                        Match {
                            transfert_user_quote: base_to_transfert * ask.price,
                            transfert_user_base: base_to_transfert,
                            remainder_ask_base: ask.amount - base_to_transfert,
                            remainder_bid_base: self.order.amount - base_to_transfert,
                        },
                        StateOrder::<AskSide, MakerPos>::new_with_order(ask, dex.maker_fee),
                    ))
                } else {
                    None
                }
            });
        info!("bid matched:{:?}", matched);
        matched
    }
}

impl<POSITION> StateOrder<BidSide, POSITION> {
    fn transfer_bid_match(
        mut self,
        bid_trader: &mut UserOrders,
        ask_trader: &mut UserOrders,
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
            "transfert_match bid matched.transfert_user_base:{} bid bid_fee_base_amout:{}",
            matched.transfert_user_base, bid_fee_base_amount
        );
        bid_trader.base_vault.put(
            ask_trader
                .locked_base_vault
                .take(matched.transfert_user_base - bid_fee_base_amount),
        );

        //update bid and ask order with remainding
        self.amount_transfered += matched.transfert_user_quote;
        self.order.amount = matched.remainder_bid_base;

        if self.order.amount == Decimal::zero() {
            //remove unnecessary locked quote.
            let diff = self.order.locked_amount - self.amount_transfered;
            bid_trader
                .quote_vault
                .put(bid_trader.locked_quote_vault.take(diff));
        }

        (self.order.amount == Decimal::zero()).then(|| self)
    }
}

impl<POSITION> StateOrder<AskSide, POSITION> {
    fn match_ask_order(
        &self,
        set: &mut BTreeSet<Order>,
        dex: &DexParameters,
    ) -> Option<(Match, StateOrder<BidSide, MakerPos>)> {
        //BTreeSet min only in Nigthly.
        let found = { set.iter().cloned().max() };
        let matched = found
            .as_ref()
            .and_then(|order| set.take(order))
            .and_then(|bid| {
                if bid.price >= self.order.price {
                    let base_to_transfert = std::cmp::max(bid.amount, self.order.amount);
                    Some((
                        Match {
                            transfert_user_quote: base_to_transfert * bid.price,
                            transfert_user_base: base_to_transfert,
                            remainder_ask_base: self.order.amount - base_to_transfert,
                            remainder_bid_base: bid.amount - base_to_transfert,
                        },
                        StateOrder::<BidSide, MakerPos>::new_with_order(bid, dex.maker_fee),
                    ))
                } else {
                    None
                }
            });
        info!("ask matched:{:?}", matched);
        matched
    }
}

impl<POSITION> StateOrder<AskSide, POSITION> {
    fn transfer_ask_match(
        mut self,
        bid_trader: &mut UserOrders,
        ask_trader: &mut UserOrders,
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
            "transfert_match ask ask_trader.quote_vault:{}",
            ask_trader.quote_vault.amount()
        );
        //update bid and ask order with remainding
        self.amount_transfered += matched.transfert_user_base;
        self.order.amount = matched.remainder_ask_base;

        (self.order.amount == Decimal::zero()).then(|| self)
    }
}
