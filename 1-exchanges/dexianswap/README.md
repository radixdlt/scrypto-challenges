# DeXianSwap

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

DeXianSwap is a protocol of an Proactive Market Making (PMM) Decentralized Exchange (DEX), It is developed on the Radix Engine V2 with its own Scrypto(0.3.0) Language. DeXianSwap features highly capital-efficient liquidity pools that support single-token provision, reduce impermanent loss, and minimize slippage for traders.

## Abstract
DeXianSwap is not just a copycat of Uniswap re-implements on Radix Ledger, We developed a Proactive Market Making (PMM) algorithim base on Radix asset-orient programming method. DeXianSwap is different from the non-constant function market maker model, which separates the transaction-to-asset relationship. Parameters such as asset ratio and curve slope can be flexibly set. At the same time, an oracle machine can be introduced to guide prices or price discovery by the market on the Radix ledger. This oracle also gathers more funds near the market price and provides sufficient liquidity.

There are 3 key features in current version：
* Proactive market making with external price guidance.
* Low barrier-to-entry automated market making for new issued assets with little sell-side liquidity.
* capable of supporting stablecoin trading scenarios.

## BluePrint

The DeXianSwap protocol is made up of two core blueprints which are: the SimplePool blueprint and the PMMPool blueprint.


## Example

#### Getting Started

In order to ensure that the account and package addresses match on my local machine as well as on yours we need to first reset resim by doing the following:

``` shell
$ resim reset
Data directory cleared.


OP1=$(resim show-configs | grep "Default Account" | awk -F ": " '{print $2}')

OP2=$(resim new-account)
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS2=$(echo "$OP2" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")


PK_OP=$(resim publish ".")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")



```




## Conclusion
This work implements DeXian, An PMM DEX on the Radix ledger built with v0.3.0 of Scrypto. DeXian dedicated to be a a Scaleble, secure and atomically composable PPM protocol which is developed on Radix Ledger.


## License
This work is licensed under Apache 2.0 and the license file is provided [here](LICENSE).
