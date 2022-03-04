# Scrypto DEX Challenge
## _By petitcroco for the 'xrd ducks mafia'_

[![N|Scrypto](dTxpPi9lDf.svg)](https://www.radixdlt.com/post/scrypto-v0-3-released)

<img src="https://cdn.discordapp.com/attachments/900022149153161286/938773880120705084/loader3.png" width="200">
Our participation in the challenge is part of a decentralized Order book. Thanks to this set of blueprints you can exchange on peers defined by an administrator.


# Features
- Place sales orders
- Buy a certain resource by buying it back from a seller.
- See placed orders.
- Found the most interesting order (best price).
- Accept a secure exchange.
- A badge system for DEX administrators and members.

# Sources
| src/*.rs | Blueprint name | Description |
| ------ | ------ | ------ |
| lib |  | Handle imports |
| duckm_test_tokens | TestTokens | Test Tokens blueprint |
| order_book | OrderBook | OrderBook blueprint |


# Components functions
## Blueprint: `TestTokens`
| Function | Argument | Returns
| ------ | ------ | ------ |
| init | nameOfTheTestToken: String | Component: Component

## Blueprint: `OrderBook`
| Function | Argument | Returns
| ------ | ------ | ------ |
| init | | (AdminBadge: Bucket, Component: Component)


# Components methods
## Blueprint: `TestTokens`
| Methods | Authorisation required | Arguments | Description
| ------ | ------ | ------ | ------ |
| count | None | None | Writes in console the number of DUCKM test tokens in the component | 
| get_for_free | None | `nbr`: Decimal | Return as many duckm test tokens as requested for free
| get_with_radix | None | `nbr`: Decimal, payment: Bucket  | Return as many duckm test tokens as requested for 1 xrd per token. taken from the given bucket

## Blueprint: `OrderBook`
| Methods | Authorisation required | Arguments | Description
| ------ | ------ | ------ | ------ |
| get_admin_badge_address | None | None | Returns the resource address of the admin badge |
| get_member_badge_address | None | None | Returns the resource address of the member badge
| become_member | None | `name`: String  | Returns your user badge allowing you to interact with the order book
| look_orderbook | None | `input`: Address, `output`: Address | Returns the order book for the pair: (`input`,`output`)  |

When we talk about peer, it is simply two exchangeable Resource Addresses. For example when using the pair (`input`, `output`). *If someone places an order with this one, it means that they want to sell the `input` resource and get in exchange the `output` ressource.*
| Methods | Authorisation required | Arguments | Description
| ------ | ------ | ------ | ------ |
| init_pair_orderbook | Admin badge | `input`: Address, `ouput`: Address | Initialize the pair (`input`, `output`) |
| reset_pair_orderbook | Admin badge | `input`: Address, `ouput`: Address | Destroy all orders present for the pair (`input`, `output`)
| withdraw | Admin badge | `tokenAddress`: Address, `amount`: Decimal | Withdraw `amount` token from the `tokenAddress` Component Vault
| add_order_orderbook | Member badge | `input`: Address, `output`: Address, `amount`: Decimal, `price`: Decimal, `payment`: Bucket, `member_address`: Address | Place an order like this: `member_address` sell `amount` tokens of `input` for `price`*`amount` tokens of `output`.  |
| accept_an_order | Member badge | `input`: Address, `output`: Address, `amount`: Decimal, `price`: Decimal, `payment`: Bucket | Accept an order from this pair : (`input`,`output`) if anyone wants to sell `amount` tokens of `input` for `amount`*`price` tokens of `output`. The `payment` bucket need to be filled with enough tokens `output`, you need at least: `amount`\*`price` tokens of `output`  |
|update_register_orderbook | Member badge | `input`: Address, `ouput`: Address, `amoun`t: Decimal, `price`: Decimal, `member_address`: Address | Allows you to add a given order in the book of this pair |
|update_unsubscribe_orderbook | Member badge | `input`: Address, `ouput`: Address, `amount`: Decimal, `price`: Decimal, `member_address`: Address | Allows you to remove a given order in the book of this pair |

# How are orders stored ?

![alt text](stored_orders.png)

# Scrypto v0.3
Install the Scrypto Toolchain [docs.radixdlt.com](https://docs.radixdlt.com/main/scrypto/getting-started/install-scrypto.html) v0.3 to run. Or updating Scrypto to the latest version [docs.radixdlt.com](https://docs.radixdlt.com/main/scrypto/getting-started/updating-scrypto.html).
 
# Usage of the transaction manifest

Transaction manifest is the Radix-way of building transactions. Transaction manifests are human-readable and are translated into binary transactions by a compiler.

To show composability and allow easier testing we use this improvement

```sh
---Soon---
```

# License
No Licence yet.

** Quack, Quack 🦆 **
