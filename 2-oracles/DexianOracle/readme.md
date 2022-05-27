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



## Test (command line)

``` shell
resim reset
result=$(resim new-account)
export user_account=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')

result=$(resim publish ".")
export pkg=$(echo $result | awk -F ": " '{print $2}')

result=$(resim call-function $pkg DeXianOracle new 20)
export comp=$(echo $result | awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')
badge=$(echo $result | awk -F "Resource: " '{print $2}' | awk -F " " '{print $1}')

resim run transactions/user_account_feed.rtm

result=$(resim new-account)
export user_account2=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export user_account2_private=$(echo $result|grep "Account component address: "|awk -F "Private key: " '{print $2}')

resim  set-default-account $user_account2 $user_account2_private

resim call-method $comp 'get_price' 'XRD/USD'


# resim call-method $comp 'request_price' 'XRD/USD' "xxx" "yyy"

```

