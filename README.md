# Scrypto DEX Challenge
## _By petitcroco for the 'xrd ducks mafia'_

[![N|Scrypto](https://raw.githubusercontent.com/p3titcr0c0/scrypto-challenges/6cc5145efaa1ac473c15c32ec9e7788a3bc569c8/dTxpPi9lDf.svg)](https://www.radixdlt.com/post/scrypto-v0-3-released)

Our participation in the challenge is part of a decentralized Order book. Thanks to this set of blueprints you can exchange on peers defined by an administrator.


## Features
- Place sales orders
- Buy a certain resource by buying it back from a seller.
- See placed orders.
- Found the most interesting order (best price).
- Accept a secure exchange.
- A badge system for DEX administrators and members.

## Sources
| src/*.rs | Blueprint name | Description |
| ------ | ------ | ------ |
| lib |  | Handle imports |
| duckm_test_tokens | TestTokens | Test Tokens blueprint |
| order_book | OrderBook | OrderBook blueprint |


## Components functions
## Blueprint: `TestTokens`
| Function | Argument | Returns
| ------ | ------ | ------ |
| init | nameOfTheTestToken: String | Component: Component

## Blueprint: `OrderBook`
| Function | Argument | Returns
| ------ | ------ | ------ |
| init | | (AdminBadge: Bucket, Component: Component)


## Components methods
## Blueprint: `TestTokens`

| Methods | Authorisation required | Arguments | Description
| ------ | ------ | ------ | ------ |
| count | |  | Writes in console the number of DUCKM test tokens in the component | 
| get_for_free | | nbr: Decimal | Return as many duckm test tokens as requested for free
| get_with_radix | | nbr: Decimal, payment: Bucket  | Return as many duckm test tokens as requested for 1 xrd per token. taken from the given bucket


## Blueprint: `OrderBook`

## Scrypto v0.3
Install the Scrypto Toolchain [docs.radixdlt.com](https://docs.radixdlt.com/main/scrypto/getting-started/install-scrypto.html) v0.3 to run. Or updating Scrypto to the latest version [docs.radixdlt.com](https://docs.radixdlt.com/main/scrypto/getting-started/updating-scrypto.html).
 
 ## Usage of the transaction manifest

Transaction manifest is the Radix-way of building transactions. Transaction manifests are human-readable and are translated into binary transactions by a compiler.

To show composability and allow easier testing we use this improvement

```sh
---Soon---
```

## License
No Licence yet.

** Quack, Quack 🦆 **
