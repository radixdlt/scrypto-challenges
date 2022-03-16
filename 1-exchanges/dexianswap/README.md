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
export pub_key2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export acct2=$(echo "$OP2" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")


PK_OP=$(resim publish ".")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

export bucket_t1="10000,03ad8ad4fa972bca8ee488458f0c67a1dc95e91ced95e3e0e70634"
export bucket_xrd="5000,030000000000000000000000000000000000000000000000000004"
export simple_url="https://goxrd.com"
export fee=0.002
export initial_supply=10000

resim call-function $PACKAGE SimplePool new $bucket_t1 $bucket_xrd $initial_supply $simple_url $fee


export simple_pool="027217a5820825527b0a9691184894c03f9d348e187cd833818c3a"
resim set-default-account $acct2 $pub_key2

resim call-method $simple 'swap' "10,030000000000000000000000000000000000000000000000000004"

resim show $acct2
Component: 024cd90412a91302949e533738ad69c7eb559aca1fe2ba70986957
Blueprint: { package_address: 010000000000000000000000000000000000000000000000000003, blueprint_name: "Account" }
State: Struct({Struct((Array<U8>(0u8, 83u8, 95u8, 163u8, 13u8, 126u8, 37u8, 221u8, 138u8, 73u8, 241u8, 83u8, 103u8, 121u8, 115u8, 78u8, 200u8, 40u8, 97u8, 8u8, 209u8, 21u8, 218u8, 80u8, 69u8, 215u8, 127u8, 59u8, 65u8, 133u8, 216u8, 247u8, 144u8))), LazyMap("c2356069e9d1e79ca924378153cfbbfb4d4416b1f99d41a2940bfdb66c5319db01040000")})
Lazy Map: 024cd90412a91302949e533738ad69c7eb559aca1fe2ba70986957c2356069e9d1e79ca924378153cfbbfb4d4416b1f99d41a2940bfdb66c5319db01040000
├─ Address("03ad8ad4fa972bca8ee488458f0c67a1dc95e91ced95e3e0e70634") => Vault("b7a56873cd771f2c446d369b649430b65a756ba278ff97ec81bb6f55b2e7356903040000")
└─ Address("030000000000000000000000000000000000000000000000000004") => Vault("c2356069e9d1e79ca924378153cfbbfb4d4416b1f99d41a2940bfdb66c5319db02040000")
Resources:
├─ { amount: 19.920239202551706794, resource_def: 03ad8ad4fa972bca8ee488458f0c67a1dc95e91ced95e3e0e70634, name: "T1", symbol: "TEST1" }
└─ { amount: 999990, resource_def: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }

```




## Conclusion
This work implements DeXian, An PMM DEX on the Radix ledger built with v0.3.0 of Scrypto. DeXian dedicated to be a a Scaleble, secure and atomically composable PPM protocol which is developed on Radix Ledger.


## License
This work is licensed under Apache 2.0 and the license file is provided [here](LICENSE).
