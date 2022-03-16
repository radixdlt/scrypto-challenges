# TODO

* [X] handle all code TODOs
* quality
  * [x] remove all dead code
  * convert comments to trace/debug/info logging
  * [x] rename Account to SharedAccount
  * [x] remove dead imports/dependencies
  * reorganize file structure, of main package, and of cli package
  * run clippy
  * use scrypto_statictypes
* features
  * [x] add order deadline (from signer)
  * [x] add direction flag to support sell request (instead of just the buy request we have now) -- compare with Airswap/Swap documentation
  * [-] handle swap NFTs (should only need CLI changes?)
* testing
  * add scenerio where the sender composes a more interesting tx, where the get the money from somewhere else first (like a flash loan) or they combine 2 RFQs to do their own routing
  * add scenerio where non default maker callback is used (and they do something interesting, like maybe swap for the needed token using an AMM)
  * [x] delete/cleanup tests/lib.rs
* documentation
  * document EVERY function
  * document every file
  * [x] write up front README
  * [x] make sure to document how to use `cargo doc`


# HareSwap: P2P DEX Protocol on Radix

![](hareswap.png)

HareSwap is a (prototype) decentralized exchange platform that implements
part of the Swap protocol to create a peer-to-peer protocol for trading *any*
resource (fungible and non-fungible) on the Radix Ledger.

Yes, inspired by [AirSwap](https://about.airswap.io). While this is only a small part of AirSwap, it is a uniquely important example showing how **composability and transaction manifests on Radix take things to the next level!**

This implementation is an adaptation of a subset of the Swap protocol and Request-for-Quote
interactions described here:
- Swap Protocol: <https://www.airswap.io/whitepaper.htm>
- Request-for-Quote: <https://about.airswap.io/technology/request-for-quote>

License: Apache + MIT

## Quick Start

1. Build:  `./build.sh`
2. Try: `cd demos && ./simple_swap.sh`
3. Learn: `cargo doc --no-deps --document-private-items --open`

-- Tested on Linux, should work on macOS too.  Windows users start with "Learn" and move to another system to build and execute the demos.

## The Code

This implementation is only a small part of a full P2P system and intentionally cuts some corners  compared to a production implementation.  What it does not cut corners on is documentation.  The code is fully commented with a complete "simple" example.  The blueprint implementation can also do more advanced things not all shown in the demos (due to time constraints).

## Why a P2P DEX?

AirSwap states it well:

  > At its core, AirSwap enables two parties to perform an "atomic swap"
  > transaction, through which both sides succeed or the entire transaction
  > reverts. These transactions are "trustless" in a way that neither party needs
  > to trust one another to complete the swap.  - https://aboout.airswap.io

This is interesting in comparison to Order Book or even AMM (and PMM) DEXs because of the tradeoffs.  It's even **better on Radix** with transaction manifests and the composability they provide.

### Problems with Order Books

* Don't scale (because execution is dependent on order book size)
* Trades are public *before* they settle leading to frontrunning risk and unfairness
* Liquidity is potentially locked up on the platform

Read more here: https://www.airswap.io/whitepaper.htm

### Compared to AMMs

  * no slippage
  * unlimited trade size

AirSwap says:

> How is AirSwap different than Uniswap? Uniswap is a "peer-to-contract"
> automated market maker (AMM) that runs fully on-chain. AirSwap is a peer-to-peer
> network combining off-chain negotiation and on-chain settlement by atomic swap.- https://about.airswap.io/frequently-asked-questions

## How it works

I suggest executing or reading through the comments in `demos/simple_swap.sh` to get an understanding
of how a HareSwap swap is negotiated and settled.

This implementation only handles the "Request for Quote" interaction.  The Swap protocol and AirSwap specifically have other ways to interact, but they are left for future work.  For example, it is assumed that any "discovery" protocol is already completed and the peers have already "found each other".

The `hare` command line interface (CLI) tool is included to facilitate off-ledger operations.  Get more details with `hare --help` after building (it will end up in `hare/target/debug/hare`)

### Setup

There is one-time work the Maker/Signer must do both on and off-ledger.  It can of course be done multiple times if desired.

  * Maker/Signer one-time setup:
      1. Generate key pair for signing orders off-ledger. See: `hare new-key-pair --help`
      2. Maker/Signers instantiate a HareSwap "Maker" component to settle orders out of their `SharedAccount`
        * This only needs to be done once (and can even be on-demand when an order worth giving a quote on is received)
        * Optionally create arbitrarily complex on-ledger "Callback" logic, or just rely on the `Maker::handler_order_default_callback` to interact with the account
        * Note: A more complex implementation with different tradeoffs might have a single HareSwap component registry and common implementations for all users.

### Per-Swap

Now, per-swap, once a Taker/Sender decides to initiate a buy/sell order the following happens:

  1. Taker/Sender initiates a "Request For Quote" to one or more possible Makers/Signers.
      * See `hare request-for-quote --help`
  2. Maker/Signer(s) decides on the price and responds with a SignedOrder (and makes sure the on-ledger state is ready to settle the order)
      * See `hare make-sign-order --help`
  3. Taker/Sender decides to accept the order and submits it in a transaction (along with their Buckets).  This can have a few steps, depending on the scenerio....  Let's build the transaction

### Taker/Sender Transaction Options

  * Build a transaction that executes the order, maybe after some up-front work:

      1. Do anything in a transaction manifest to get the selling bucket ready for the swap
          * maybe a flash loan, or aggregating from multiple accounts, or simply withdrawing from an account.
      2. Take the result SignedOrder instruction received and include it in the transaction and add the selling bucket, and authentication back to prevent frontrunning.
      3. Check the worktop has the expected buying bucket amount
      4. Do anything in a transaction manifest with the result
          * maybe pay back the flash loan, append another swap, or simply deposit to an account

  * Build a transaction that tokenizes the order (This is where things get interesting)

      0. First, convert the SignedOrder instruction into a TokenizeOrder instruction
          * See: `hare tokenize-order --help`
      1. Start a transaction manifest with bucket names matching the `tokenize-order` setup including the anti-frontrunning auth and result bucket which will hold a NonFungible representing this SignedOrder.  It is redeemable later to execute the order (up until expiration)
      2. Include the instruction output from `tokenize-order`
      3. Do anything else with the new order NonFungible, like trade it on a secondary market, use it as collateral or a guarentee to a third party, etc.
      4. At some point later (in the same transaction or not) anyone can now `CALL_METHOD "execute_order_token" Bucket("order_token") Bucket("to_sell")` and the swap is guarenteed to result in the agreed upon amount on the worktop, or failure.  (To do something in a seperate transaction more guarentees are needed beyond the scope of this example implementation)


## Why is this better on Radix?

In AirSwap, on an EVM-based implementation the Taker/Sender submitting the
transaction still needs to trust the AirSwap contracts, and has to give ERC-20
approvals all over the place.  This is not trustless.  And you could compose
transactions but not in an externally verifiable way, and not without first
creating more on-chain smart contracts for the Taker/Sender to also trust.

But in HareSwap, on Radix, the transaction manifest makes this design even more
interesting because the Taker/Sender does not need any trust in on-ledger
blueprints and they are guaranteed exactly the trade they negotiated (offline,
with no typical blockchain latency or fees) and can still make an arbitrarily
complex transaction composed with many steps.  They can do all of this with no
up-front work.  (Ok, maybe they should create a badge for authentication, but
even that can be avoided using a virtual badge and the new auth worktop that is
coming soon and front running is avoided too.)

Taking it further, the order can be tokenized into a non-fungible and moved
through the DeFi ecosystem extending these same trustless guarantees to anyone
interacting with the token.  It does become a bit more complicated if the order
token will be executed in a separate transaction, then some additional
guarantees would be needed.  But, at least within the same transaction, this
prototype code shows there is now a way to change this order into an
asset-oriented guarantee that is a first class citizen on the ledger.  This is
truly amazing and the future of DeFi.
