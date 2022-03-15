# DeXianSwap

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

DeXianSwap is a protocol of an Proactive Market Making (PMM) Decentralized Exchange (DEX), It is developed on the Radix Engine V2 with its own Scrypto(0.3.0) Language. DeXian features highly capital-efficient liquidity pools that support single-token provision, reduce impermanent loss, and minimize slippage for traders.

## DeXianSwap Abstract
DeXianSwap is not just a copycat of Uniswap re-implements on Radix Ledger, We developed a Proactive Market Making (PMM) algorithim base on Radix asset-orient programming method. DeXian is different from the non-constant function market maker model, which separates the transaction-to-asset relationship. Parameters such as asset ratio and curve slope can be flexibly set. At the same time, an oracle machine can be introduced to guide prices or price discovery by the market on the Radix ledger. This oracle also gathers more funds near the market price and provides sufficient liquidity.
There are 3 key features in current version：
* Proactive market making with external price guidance
* Low barrier-to-entry automated market making for new issued assets with little sell-side liquidity
* capable of supporting stablecoin trading scenarios
