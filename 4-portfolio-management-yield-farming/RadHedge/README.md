# RadHedge

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

The RadHedge crate is a prototype / proof-of-concept of a decentralized asset management platform meant to be operating on the Radix-Dlt. It is built with Scrypto v0.4.1 on the Radix Engine V2.

## Motivation

The motivation of this prototype is to build a showcase of how a decentralized asset management platform can be built on the Radix-Dlt. In the future (and after further development) this might become one building block for the DeFi ecosystem on Radix. The target was not to simply to reimplement a set of protocols such as TokenSets or DHedge but to rebuild from the ground up using the asset-oriented concepts that Scrypto and the Radix Engine provide.

As it is built upon an external blueprint for a price_oracle and the RaDEX (written by 0xOmar) this is also a very good example of how easy it is to interact with external component and built a layered DeFi system.

## Key Features related to the pool manager

### Creating a pool

### The pool manager badge

### Funding of the pool

### Performance fee

Performance fees can be freely set by the pool manager and changed afterwrads.

### Base currency

The pool manager gets to decide the base currency for the investment pool. All investments and withdrawals are done via the base currency.

## Key Features related to the investors

#### Non-custodial

The investment pools are non-custodial in the sense that the pool manager does not have any access to the assets in the pools. He is able to trade the assets, but he will never be able to withdraw them. All assets are kept in a decentralized component. No one has access to that.

### Pool tracking tokens

You get an exact representation of your share of ownership of the pool via pool tracking tokens. The only way to withdraw assets is to return the tracking tokens. Of course you can also trade your pool tracking tokens on a secondary market (not implemented here) to reduce transactions fees.

### Investing

    How to, Base Currency, Performance fees, etc

### Withdrawing

## Architecture

## Example of use

## Considerations and future improvements

As I had to research related to necessary functionalities of a decentralized asset management platform and also had to get a better understanding of Scrypto and the transaction manifests.
Thats why in the end this code (from first to last line!) was written in the 7 days before the end of the challenge (while also working at a full time job). For me it feels incredibly rewarding how much can be achieved in such a short time.

However there is quite a lot of functionality that I wanted to include but didn't manage to so in time. Also during development various new ideas came up. Here are some of the future improvements:

1. Build a one-stop location on-chain: Build another blueprint on top of the investment pool. This blueprint shall be the main interface to pool managers and investors and hold all instantiated investment_pools. This will be called the **"RadHedge"**.
2. Build a web-frontend as graphical interface to the RadHedge.
3. Implement private investment_pools with whitelisting
4. Implement the possibility to make the investment_pool work with various DEXs and Oracles.
5. Implement a score for the best performing investment pools.
6. Implement multiple base currencies (let the pool manager decide which ones).
7. Implement a multi-admin scheme to make it easier to have multiple pool-managers.
8. Implement a DAO structure for general governance of the protocol and to decided on changes of the protocol.

**Smaller improvements aka Low Hanging Fruits**

1. More testing! Testing via Scrypto unit testing and also more integration test via the simulator. Finally test it on the PTE!
2. Implement a feature to read out the stake of the pool_manager (his share of assets on the pool.)
3. Analyze and optimize for transaction fees and slippage.
4. Lock up newly invested funds for at least 24 hours.
5. Wait 42 days before a change of performance fees actually takes place.
6. Implement an on-chain way of how the fund-manager could communicate with his investors.

### Important Note

Unfortunately I ran out of time to actually finish this project. Therefore the documentation as well as the testing is not finished at all...
The time for the deadline ends in five minutes. I am super sad, that I couldn't finish the docs and testing. Will do that after the challenge ends though and
would be happy if I could update the repo (of course not to be evaluated for the challenge.)

I startet the transaction manifests for testing and a shell script.
Unfortunately my test don't run because I forgot to initialize prices for the oracle..
