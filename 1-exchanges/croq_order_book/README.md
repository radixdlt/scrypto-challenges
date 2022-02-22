# Croq Order Book

Traditional order book

a list of ack offer, a list of bid offer. When they meet the exchange happen.

the tokens and cash gained, is deposed in a vault which belong to the user if the exchange is not immediate.

the flow to made an offer is the following:

- call `register` to have a user badge
- call `push_bid` or `push_ask` if not executed in full immediately, it give you an offer_token
- you can call `cancel` to cancel your offer and get back any money left inside (it may have been partially executed already)
- when your offer has been fullfil, you can call `withdraw` with your user badge. You'll get back any money owe to you

there is some read only methods:
- to help monitor the status of the auction, for displaying the book on your webapp
- to check if some money are owe to you

# Functions & Methods

```
pub fn instantiate(token: Address, cash: Address) -> Component
```

to create a new order book

- `token`: the token this order book will be about
- `cash`: the cash used in this order book (`RADIX_TOKEN` currently, but I dream of stable coin...)
- `return`: the instance of created

```
pub fn register(&self) -> Bucket
```

to get a user badge

- `return`: the user badge

```
pub fn push_bid(
            &mut self,
            user_badge: BucketRef,
            price: Decimal,
            mut token: Bucket,
        ) -> Vec<Bucket>
```

to add a bid offer to the order book. it may be executed immediately (partially or in full) Or it may be just registered in the book.

- `user_badge`: your user badge, created with `register`
- `price`: your floor price
- `token`: the token you want to sell
- `return`: if executed in full immediately you'll get a bucket of cash. if not executed immediately you'll get an offer badge. if executed partially, you'll get cash and badge.

```
pub fn push_ask(
            &mut self,
            user_badge: BucketRef,
            price: Decimal,
            mut token: Bucket,
        ) -> Vec<Bucket>
```

to add an ask offer to the order book. it may be executed immediately (partially or in full) Or it may be just registered in the book.

- `user_badge`: your user badge, created with `register`
- `price`: your ceiling price
- `token`: the cash you want to spend
- `return`: if executed in full immediately you'll get a bucket of tokens. if not executed immediately you'll get an offer badge. if executed partially, you'll get tokens and badge.

```
pub fn cancel(&mut self, offer_badge: Bucket) -> (Bucket, Bucket)
```

cancel an offer

- `offer_badge`: the badge of the offer you will cancel
- `return`: a tuple of bucket, cash and token

```
pub fn withdraw(&mut self, user_badge: BucketRef) -> Vec<Bucket>
```

withdraw the cash and tokens owe to you

- `user_badge`: your user badge
- `return`: a vector of bucket containing cash and tokens

```
pub fn user_vault_content(&self, user_badge: BucketRef) -> (Decimal, Decimal)
```

return the amount of cash and token owe to you

- `user_badge`: your user badge
- `return`: amount of cash and token owe to you

```
pub fn monitor(&self)
```

log some information about the auction

example:

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
