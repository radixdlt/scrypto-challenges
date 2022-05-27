# DeXianOracle

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Basic Request Model

![Basic Request Model](res/basic_req_model.png)

#### roles
###### Consumer
The `Consumer` is the Radix Engine component that consumes/uses data from the oracle. It make a request to send a certain data consumption request to an explicit oracle component, which also carries some parameter data that can be passed back.

###### OracleComponent
The `oracle` component, which is the hub of the entire architecture, is responsible for logged consumer requests, managing authorizations and revocations to data providers, as well as accepting data pushed by data providers and initiating callbacks to connected consumers while delivering the data they need, such as: prices, contest results, etc.

It implements permission management through the badge design pattern, and the address with the specified badge can only push data to the `oracle`.

###### DataProvider
The data provider, which is the functional unit that feeds data from the off-chain data to the on-chain `oracle`, needs to present a specific badge before pushing the data inside the `oracle` component.

#### process
1. Request with callback

Spend some `XRD`, call Oracle's `request_price` method, and wait for the callback.

2. Oracle request
3. Feed
4. Fulfil oracle request

The data provider calls `feed_price` to push the price to the `oracle` and trigger a callback.

5. callback

Callback as requested by the caller of 'request_price`


## Decentralized Model

![Decentralized Request Model](res/decentrailized_model.png)

It directly calls `get_price` to get the corresponding price and the epoch (timestamp) when the price was generated.

```

RESULT=$(resim publish ".")
export PACKAGE=$(echo "$RESULT" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

export pkg=015586c1be716163cfbd2128ecebae7026ad2dee38c0b91b1b1fb9

resim call-function $pkg DeXianOracle new 20

export comp=0269b1040c49308764fef17ff5b53e5f3a72d5aca766e7233b1b23
export badge=033a8a6d4e1e20c0da6a8db3c7754c2e83d32d90b781b6581553a0


