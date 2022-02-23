# Croq Traditional Order Book

A Traditional Order Book including lists of Ask Offers and Bid Offers.

When the highest Bid is below the lowest Ask, then an exchange is made and the Order is fulfilled.

When an open offer is partially filled or fulfilled/closed, the tokens and cash gained are deposited in a vault which belongs to the user.

**Offer Flow**

The flow to make an offer is the following:

1. Call `register` to register a user badge
2. Call `push_bid` or `push_ask` if not executed in full immediately, this will give you an `offer_token`
3. You can call `cancel` to cancel your offer and get back any money left inside (it may have been partially executed already)
4. When your offer has been fullfilled, you can call `withdraw` with your user badge. You'll receive any money owed to you

There are some read only methods:

1. To help monitor the status of the auction, for displaying the book on your webapp
2. To check if some money are owe to you

## Functions & Methods

### Create a new Order Book

```
pub fn instantiate(token: Address, cash: Address) -> Component
```

- `token`: The token this Order Book will be about
- `cash`: The cash used in this Order Book (`RADIX_TOKEN`currently, but I dream of stable coin...)
- `return`: The instance of created Order Book


### Register a User Badge

```
pub fn register(&self) -> Bucket
```

- `return`: The User Badge


### Create Bid Offer

To add a Bid Offer to the Order Book.

It may be executed immediately (partially or in full), or, it may be just registered in the book.

```
pub fn push_bid(
            &mut self,
            user_badge: BucketRef,
            price: Decimal,
            mut token: Bucket,
        ) -> Vec<Bucket>
```

- `user_badge`: Your User Badge, created with `register`
- `price`: Your floor price
- `token`: The token you want to sell
- `return`: If executed in full, you'll immediately get a bucket of cash. If not executed immediately, you'll get an offer badge. If executed partially, you'll get cash and badge.



### Create Ask Offer
Add a Ask Offer to the order book.

It may be executed immediately (partially or in full), or, it may be just registered in the book.

```
pub fn push_ask(
            &mut self,
            user_badge: BucketRef,
            price: Decimal,
            mut token: Bucket,
        ) -> Vec<Bucket>
```

- `user_badge`: Your User Badge, created with `register`
- `price`: Your ceiling price
- `token`: The cash you want to spend
- `return`: If fulfilled immediately, you'll get a bucket of tokens. If not fulfilled immediately you'll get an offer badge. If partially fulfilled, you'll get tokens and badge.



### Cancel an Offer

```
pub fn cancel(&mut self, offer_badge: Bucket) -> (Bucket, Bucket)
```

- `offer_badge`: The badge of the offer you will cancel
- `return`: A tuple of bucket, cash and token


### Withdraw

Withdraw the cash and tokens owed to you.

```
pub fn withdraw(&mut self, user_badge: BucketRef) -> Vec<Bucket>
```

- `user_badge`: Your User Badge
- `return`: A vector of bucket containing cash and tokens


### Get Balances Owed
Return the amount of cash and tokens owed to you.

```
pub fn user_vault_content(&self, user_badge: BucketRef) -> (Decimal, Decimal)
```

- `user_badge`: Your user badge
- `return`: Amount of cash and tokens owed to you


### Monitor
Log some information about the auction.

```
pub fn monitor(&self)
```
**Example:**

```
Logs: 11
├─ [INFO ] token addr: 03de6e411593dcb3817187562c26c972cb024524f7b798f1c2980c
├─ [INFO ] cash addr: 030000000000000000000000000000000000000000000000000004
├─ [INFO ] **bid**
├─ [INFO ] floor price, quantity of tokens
├─ [INFO ] 6, 200
├─ [INFO ] 5, 20
├─ [INFO ] 5, 20
├─ [INFO ] **ask**
├─ [INFO ] ceilling price, amount of cash
├─ [INFO ] 3, 100
└─ [INFO ] 4, 25
```
