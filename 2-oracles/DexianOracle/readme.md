# DeXianOracle

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
export user_account_private=$(echo $result|grep "Account component address: "|awk -F "Private key: " '{print $2}')

result=$(resim publish ".")
export pkg=$(echo $result | awk -F ": " '{print $2}')

result=$(resim call-function $pkg DeXianOracle new 20)
export comp=$(echo $result | awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')
export badge=$(echo $result | awk -F "Resource: " '{print $2}' | awk -F " " '{print $1}')

resim run transactions/user_account_feed.rtm

result=$(resim new-account)
export user_account2=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export user_account2_private=$(echo $result|grep "Account component address: "|awk -F "Private key: " '{print $2}')

resim  set-default-account $user_account2 $user_account2_private

resim call-method $comp 'get_price' 'XRD/USD'

resim  set-default-account $user_account $user_account_private

resim run transactions/user_account2_request.rtm 


```


## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.