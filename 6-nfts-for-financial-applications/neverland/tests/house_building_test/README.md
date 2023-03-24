--------------------------------------------------------------------------------------------------
# House property building Asset NFT and Land Asset NFT Merge Test  
--------------------------------------------------------------------------------------------------

> House property building Asset NFT Test workflow:
> 	Architect submit a House Project.
> 	Land owner buy the project and make a build call.
> 	General Contractor answers the call and build the house.
> 	Once house is built Land owner approve the building contract.
> 	General Contractor claims the building contract payment.
> 	Land owner check his SBT and Asset NFTs data.


> Land Asset NFT Merge Test workflow:
> 	Land owner provide two contiguous Land Asset NFTs to merge.
> 	Land owner check his SBT data.
 

N.B."Instructions" data tabs within Transactions output as well as other empty fields have been intentionally obmitted.

-------------------------------------------------------------------------------------------
# Index  
-------------------------------------------------------------------------------------------	
> [Part_1](#part_1) . HouseHub Component and active accounts list and resource addresses
>
> [Part_2](#part_2) . Architect signup within protocol and submit a house project.
>
> [Part_3](#part_3) . Neverland Land Asset NFT owner consult house project list within protocol and buy a house project  to realize it within his property.
>
> [Part_4](#part_4) . Architect evaluate his credit amount within protocol once his submitted house projec has been sold and claim then.
>
> [Part_5](#part_5) . Neverland Land Asset NFT owner submit a build call within protocol to find a contractor wishing to build up a house in his land property
>
> [Part_6](#part_6) . General Contractor submit a build call. General Contractor deliver the buiding.
>
> [Part_7](#part_7) . Neverland Land Asset NFT owner verify building contract has been executed, approve it and performs a datacheck of his SBT and AssetNFT
>
> [Part_8](#part_8) . General Contractor inspects building contract to verify it's approved and claims contract payment
>
> [Part_9](#part_9) . Merge two contiguous Land AssetNFT properties into a single Land AssetNFT property.
>


#
### Part_1 
# HouseHub Component and active accounts list and resource addresses
-------------------------------------------------------------------------------------------

>```HouseHub Component
```
└─ Component: component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26		Neverland HouseHub Component address
├─ Resource: resource_sim1qz48rjq6sqtvdvdtf6fkmqusw6q9ug7lm8qfpnt6p9hq78aakn		Neverland HouseHub MinterBadge
├─ Resource: resource_sim1qp7vkpmuxkdezyr6hklmhm0l84rslug9rm7wegmdm5zsypnh8c		Neverland HouseHub OwnerBadge
├─ Resource: resource_sim1qr9vxx976j9w225swt0wwhz35j8kgpws4vjywga3725q6t9crx		Neverland House Hub ArchBadge
├─ Resource: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x		Neverland House Hub House Project
├─ Resource: resource_sim1qq45gmeqeemh3dmj76av46ma9fvrs3mrkm3dgz5hngrs4l74u6		Neverland House Hub Building Contract
└─ Resource: resource_sim1qqh9txs7vzlrcqcp9suesskwl4u5tewce2uhpan0uw7qwmwtva		Neverland House Hub Building Property
```

>
> protocol's owner account
>
```
Account component address: account_sim1qwk73ye3gfmnxnw42jgpv3gey9jj8a50se753pvnccfquqkgk3	
Public key: 0383ffb219b35c04f26db0a1e8efb9efec16fdd931aef837512bd60aa172342fa4
Private key: 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28
```

>
> neverland's general contractor account 
>
```
Account component address: account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra	
Public key: 035d9746555d52eab4484c63b6b4df25b2186d44969bbacfb923f5de99e00df733
Private key: 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2

SBT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
SBT id: 300710000000985f0bf0a4fbceed30b544a167858b01

NFT Degree resource address: resource_sim1qpyc88dfmeuvly4vu5ssdqnwzrpyzqrcp0juettpvttqp25370
NFT Degree id: 300710000000d379d1ad6cedb9c2e17e8a4263ba8bee
```

>
> neverland's architect account 
>
```
Account component address: account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg	
Public key: 03e4d1962763c01d07d9432a22bd41c7c6685487d7aa0b0efba9ba8163435fb4d9
Private key: 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c

SBT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
SBT id: 300710000000e492d1a29f9c41e86f8ba19215d0c46c

NFT Degree resource address: resource_sim1qpyc88dfmeuvly4vu5ssdqnwzrpyzqrcp0juettpvttqp25370
NFT Degree id: 300710000000b64e9392d358c2388481975e9ef7124d
```

>
> neverland's land owner account 
>
```
Account component address: account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t	
Public key: 03dae07d865f8902053911403291fa606d78f0081a40d65aa3ca0b7fd978ac5162
Private key: c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7

SBT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
SBT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050

Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
Parcel 0101

Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
Parcel 0102

Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
Parcel 0303

Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
Real estate property NFT id: 3007100000008018133901c22bcb8174846623c71db4
Parcel 0201

Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
Real estate property NFT id: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
Parcel 0202

```

```
> resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe		Neverland environment currency
```


> Switch default account protocol owner

```resim set-default-account account_sim1qwk73ye3gfmnxnw42jgpv3gey9jj8a50se753pvnccfquqkgk3 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28```

> Switch default account land owner

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```

> Switch default account general contractor

```resim set-default-account account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28```

> Switch default account architect

```resim set-default-account account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c```



#
### Part_2 
## Architect signup showing his own SBT as proof whom carry on his Study title degrees data info. Protocol mint an architect badge to allow
## him to submit house buildings projects within protocol whom later can be purchased by other Neverland land owners wishing to build them up
## within their land properties.
-------------------------------------------------------------------------------------------

> Architect signup showing his own SBT as proof whom carry on his Study title degrees data info.

> Switch default account architect

```resim set-default-account account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c```


>cd house_hub_transaction_manifest


> update data on ```mint_arch_badge```

>```mint_arch_badge.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1330393 XRD burned, 0.006651965 XRD tipped to validators
Cost Units: 100000000 limit, 1330393 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Architect Badge address: resource_sim1qr9vxx976j9w225swt0wwhz35j8kgpws4vjywga3725q6t9crx
└─ [INFO ]  Architect Badge id: 30071000000011e85561c470301a489bc37b24f67d09
```

> Architect submit a house building project within protocol.

> update data on ```submit_house_project```

>```submit_house_project.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1129731 XRD burned, 0.005648655 XRD tipped to validators
Cost Units: 100000000 limit, 1129731 consumed, 0.0000001 XRD per cost unit
Logs: 0

```



[Back Up](#index)
#
### Part_3 
## Neverland Land Asset NFT owner consult house project list within protocol and buy a house project  
## to realize it within his property, protocol mint a House Project Asset NFT. 
-------------------------------------------------------------------------------------------

> Neverland Land Asset NFT owner consult house project list within protocol

> Switch default account land owner

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> update data on ```house_project_list```

>```house_project_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1793557 XRD burned, 0.008967785 XRD tipped to validators
Cost Units: 100000000 limit, 1793557 consumed, 0.0000001 XRD per cost unit
Logs: 16
├─ [INFO ]  =========================================================================
├─ [INFO ]  Architect Badge resource address: resource_sim1qr9vxx976j9w225swt0wwhz35j8kgpws4vjywga3725q6t9crx
├─ [INFO ]  Architect Badge id: 30071000000011e85561c470301a489bc37b24f67d09
├─ [INFO ]  =====================================================================
├─ [INFO ]  House Project NFT resource address: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x
├─ [INFO ]  House Project NFT id: 30071000000068230c0e0db22a129a64fb6bf1c52616
├─ [INFO ]  House Project URI: "ipfs.io/ipfs/house_project_hash\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"400\">"
├─ [INFO ]  Building description: House Building Project stand alone house
├─ [INFO ]  Levels description: double level
├─ [INFO ]  Rooms description: room 3, bathroom 2, living room 1, kitchen 1, entrance hall 1, sun deck 1, car box 1
├─ [INFO ]  Installations description: thermal insulation, solar panels, heat pump
├─ [INFO ]  Number of realized buildings: 0
├─ [INFO ]  Building square meters surface: 25
├─ [INFO ]  Building energetic class: 1
├─ [INFO ]  House Project price: 100
└─ [INFO ]  Name: TKN Symbol: TKN
```

> Neverland Land Asset NFT owner buy a house project within protocol and the latter mint and return a House Project Asset NFT.

> update data on ```buy_house_project```

>```buy_house_project.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2002874 XRD burned, 0.01001437 XRD tipped to validators
Cost Units: 100000000 limit, 2002874 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  House Project NFT address: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x
└─ [INFO ]  House Project NFT id: 30071000000068230c0e0db22a129a64fb6bf1c52616
```


[Back Up](#index)
#
### Part_4 
## Architect evaluate his credit amount within protocol once his submitted house projec has been sold and claim then
## providing his Arch Badge as proof.
-------------------------------------------------------------------------------------------

> Architect evaluate his credit amount within protocol once his submitted house projec has been sold.


> Switch default account architect

```resim set-default-account account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c```


> update data on ```ask_accrued_amount```

>```ask_accrued_amount.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1142651 XRD burned, 0.005713255 XRD tipped to validators
Cost Units: 100000000 limit, 1142651 consumed, 0.0000001 XRD per cost unit
Instruction Outputs:
├─ Decimal("100")
```


> Architect collect credit amount.

> update data on ```collect_project_payment```

>```collect_project_payment.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1235706 XRD burned, 0.00617853 XRD tipped to validators
Cost Units: 100000000 limit, 1235706 consumed, 0.0000001 XRD per cost unit
```



[Back Up](#index)
#
### Part_5
## Neverland Land Asset NFT owner submit a build call within protocol to find a contractor wishing to build up a house in his land property
## following architect's house project.
## Land owner needs to specify contract amount, duration, contract's URL pointer, deposit an amount in protocol's currency, deposit 
## House Project NFT as well as Land Property AssetNFT and authenticate himself through SBT proof to testify land property ownership's correspondence.
## General contractor consults build calls list within protocol.
-------------------------------------------------------------------------------------------

> Switch default account land owner

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```

> update data on ```build_call```

>```build_call.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3403093 XRD burned, 0.017015465 XRD tipped to validators
Cost Units: 100000000 limit, 3403093 consumed, 0.0000001 XRD per cost unit

```


> General contractor consults build calls list within protocol

> Switch default account general contractor

```resim set-default-account account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2```


> update data on ```build_call_list```

>```build_call_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2757605 XRD burned, 0.013788025 XRD tipped to validators
Cost Units: 100000000 limit, 2757605 consumed, 0.0000001 XRD per cost unit
Logs: 19
├─ [INFO ]  =========================================================================
├─ [INFO ]  House Hub address: component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26
├─ [INFO ]  Building Contract resource address: resource_sim1qq45gmeqeemh3dmj76av46ma9fvrs3mrkm3dgz5hngrs4l74u6
├─ [INFO ]  Building Contract id: 3007100000007ec86f1d551d90da1e85c44d8af69cfb
├─ [INFO ]  =========================================================================
├─ [INFO ]  Building Contract URL: https://ipfs.io/ipfs/build_call_hash
├─ [INFO ]  Land owner SBT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  Contractor SBT address: None
├─ [INFO ]  Contractor SBT id: None
├─ [INFO ]  Land property NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Land property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  House project NFT: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x
├─ [INFO ]  House project NFT id : 30071000000068230c0e0db22a129a64fb6bf1c52616
├─ [INFO ]  Building surface: 25
├─ [INFO ]  Contract amount: 100
├─ [INFO ]  Deadline: None
├─ [INFO ]  Executed: false
├─ [INFO ]  Building Contract value: 100 $TKN TKN
└─ [INFO ]  Penalty: None

```



[Back Up](#index)
#
### Part_6 
## General Contractor submit a build call.
## He needs to specify which build call he's subscribing, provide a bond in protocol 
## currency, provide his study Degree data within SBT proof.
## General Contractor deliver the building.
## General Contractor check the building contract to verify data has been correctly updated.
-------------------------------------------------------------------------------------------

> General Contractor submit a build call

> update data on ```subscribe_build_call```

>```subscribe_build_call.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2152013 XRD burned, 0.010760065 XRD tipped to validators
Cost Units: 100000000 limit, 2152013 consumed, 0.0000001 XRD per cost unit
Logs: 0
```


> General Contractor check the building contract to verify data has been correctly updated.

>```inspect_building_contract.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2454743 XRD burned, 0.012273715 XRD tipped to validators
Cost Units: 100000000 limit, 2454743 consumed, 0.0000001 XRD per cost unit
Logs: 17
├─ [INFO ] url: https://ipfs.io/ipfs/build_call_hash
├─ [INFO ] house hub address : component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26
├─ [INFO ] land owner sbt_address : resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ] land owner sbt id : 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ] contractor sbt address : resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ] contractor sbt id : 300710000000985f0bf0a4fbceed30b544a167858b01
├─ [INFO ] land property nft: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ] land property nft_id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ] house project nft: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x
├─ [INFO ] house project nft id: 30071000000068230c0e0db22a129a64fb6bf1c52616
├─ [INFO ] property building nft: resource_sim1qqh9txs7vzlrcqcp9suesskwl4u5tewce2uhpan0uw7qwmwtva
├─ [INFO ] property building nft_id: 3007100000004b7e367b76f1ab21f045d96995b03490
├─ [INFO ] building surface: 25
├─ [INFO ] contract amount: 100
├─ [INFO ] deadline: 30021
├─ [INFO ] executed: false
└─ [INFO ] approved: false

```



> General Contractor deliver the building

> update data on ```contractor_delivery```

>```contractor_delivery.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.215554 XRD burned, 0.0107777 XRD tipped to validators
Cost Units: 100000000 limit, 2155540 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Building Property NFT address: resource_sim1qqh9txs7vzlrcqcp9suesskwl4u5tewce2uhpan0uw7qwmwtva
└─ [INFO ]  Building Property NFT id: 3007100000004b7e367b76f1ab21f045d96995b03490

```


> General Contractor check the building contract to verify data has been correctly updated.

>```inspect_building_contract.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2454727 XRD burned, 0.012273635 XRD tipped to validators
Cost Units: 100000000 limit, 2454727 consumed, 0.0000001 XRD per cost unit
Logs: 17
├─ [INFO ] url: https://ipfs.io/ipfs/build_call_hash
├─ [INFO ] house hub address : component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26
├─ [INFO ] land owner sbt_address : resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ] land owner sbt id : 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ] contractor sbt address : resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ] contractor sbt id : 300710000000985f0bf0a4fbceed30b544a167858b01
├─ [INFO ] land property nft: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ] land property nft_id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ] house project nft: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x
├─ [INFO ] house project nft id: 30071000000068230c0e0db22a129a64fb6bf1c52616
├─ [INFO ] property building nft: resource_sim1qqh9txs7vzlrcqcp9suesskwl4u5tewce2uhpan0uw7qwmwtva
├─ [INFO ] property building nft_id: 3007100000004b7e367b76f1ab21f045d96995b03490
├─ [INFO ] building surface: 25
├─ [INFO ] contract amount: 100
├─ [INFO ] deadline: 30021
├─ [INFO ] executed: true
└─ [INFO ] approved: false

```


[Back Up](#index)
#
### Part_7 
## Neverland Land Asset NFT owner check the buid call list to verify building contract has been executed,
## approve building contract and performs a datacheck within his SBT and AssetNFTs to verify they've been succesfully updated.
----------------------------------------------------------------------------------------------------------------------

> Verify building contract has been executed


> Switch default account land owner

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> update data on ```build_call_list```

>```build_call_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2879746 XRD burned, 0.01439873 XRD tipped to validators
Cost Units: 100000000 limit, 2879746 consumed, 0.0000001 XRD per cost unit
Logs: 19
├─ [INFO ]  =========================================================================
├─ [INFO ]  House Hub address: component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26
├─ [INFO ]  Building Contract resource address: resource_sim1qq45gmeqeemh3dmj76av46ma9fvrs3mrkm3dgz5hngrs4l74u6
├─ [INFO ]  Building Contract id: 3007100000007ec86f1d551d90da1e85c44d8af69cfb
├─ [INFO ]  =========================================================================
├─ [INFO ]  Building Contract URL: https://ipfs.io/ipfs/build_call_hash
├─ [INFO ]  Land owner SBT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  Contractor SBT address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Contractor SBT id: 300710000000985f0bf0a4fbceed30b544a167858b01
├─ [INFO ]  Land property NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Land property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  House project NFT: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x
├─ [INFO ]  House project NFT id : 30071000000068230c0e0db22a129a64fb6bf1c52616
├─ [INFO ]  Building surface: 25
├─ [INFO ]  Contract amount: 100
├─ [INFO ]  Deadline: 30021
├─ [INFO ]  Executed: true
├─ [INFO ]  Building Contract value: 100 $TKN TKN
└─ [INFO ]  Penalty: None

```

> Approve building contract 

> update data on ```approve_contract```

>```approve_contract.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5751312 XRD burned, 0.02875656 XRD tipped to validators
Cost Units: 100000000 limit, 5751312 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Academy TKN contribution amount 0.33

```


> Check land owner SBT  to verify Land Property AssetNFT data has been succesfully updated within ```real_estate_properties``` field 
> with Building Propery AssetNFT identification

> update data on ```ask_property_sbt```	

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3128532 XRD burned, 0.01564266 XRD tipped to validators
Cost Units: 100000000 limit, 3128532 consumed, 0.0000001 XRD per cost unit
Logs: 18
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_1\" \n \
		"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"0\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0101", value_1: 0, value_2: 0, value_3: 125, 
		linked_assets: [(resource_sim1qqh9txs7vzlrcqcp9suesskwl4u5tewce2uhpan0uw7qwmwtva, 3007100000004b7e367b76f1ab21f045d96995b03490)] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_2\" \n \
		"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"66\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0102", value_1: 0, value_2: 0, value_3: 125, 
		linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_9\" \n \
		"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"133\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0303", value_1: 0, value_2: 0, value_3: 125, 
		linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000f313a5e55bdfefd076547972e6cd5de8
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https://ipfs.io/ipfs/merge_properties_hash\" \n \
		"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"0\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />
		<rect x=\"66\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 02010202", value_1: 0, value_2: 0, value_3: 250, 
		linked_assets: [] }
```


> Check Land Property AssetNFT to verify Building Propery AssetNFT identification data has been succesfully updated within ```linked_assets``` fields

> update data on ```check_asset_nft```	(Land Property AssetNFT)

>```check_asset_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1520383 XRD burned, 0.007601915 XRD tipped to validators
Cost Units: 100000000 limit, 1520383 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ]  data_1:  asset type Neverland Property Certificate
├─ [INFO ]  data_2:  Region North
├─ [INFO ]  data_3:  District NorthWest
├─ [INFO ]  data_4:  Parcel 0101
├─ [INFO ]  value_1: 1
├─ [INFO ]  value_2: 25
├─ [INFO ]  value_3: 125
├─ [INFO ]  linked_assets resource address:
                    resource_sim1qqh9txs7vzlrcqcp9suesskwl4u5tewce2uhpan0uw7qwmwtva
└─ [INFO ]  linked_assets ID:
                    3007100000004b7e367b76f1ab21f045d96995b03490

```


> Check Building Property AssetNFT to verify House Project AssetNFT identification data has been succesfully updated within ```linked_assets``` fields

> update data on ```check_asset_nft```	(Building Property AssetNFT)

>```check_asset_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.152503 XRD burned, 0.00762515 XRD tipped to validators
Cost Units: 100000000 limit, 1525030 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ]  data_1: Real Estate: Bulding Property Certificate
├─ [INFO ]  data_2: double level
├─ [INFO ]  data_3: room 3, bathroom 2, living room 1, kitchen 1, entrance hall 1, sun deck 1, car box 1
├─ [INFO ]  data_4: thermal insulation, solar panels, heat pump
├─ [INFO ]  value_1: 1
├─ [INFO ]  value_2: 25
├─ [INFO ]  value_3: 1
├─ [INFO ]  linked_assets resource address:
                    resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x
└─ [INFO ]  linked_assets ID:
                    30071000000068230c0e0db22a129a64fb6bf1c52616

```


[Back Up](#index)
#
### Part_8 
# General Contractor inspects building contract to verify it's approved and claims contract payment 
-------------------------------------------------------------------------------------------

> General Contractor inspects building contract to verify approved flag is setted as true


> Switch default account general contractor

```resim set-default-account account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2```


> update data on ```inspect_building_contract```	

>```inspect_building_contract.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2454711 XRD burned, 0.012273555 XRD tipped to validators
Cost Units: 100000000 limit, 2454711 consumed, 0.0000001 XRD per cost unit
Logs: 17
├─ [INFO ] url: https://ipfs.io/ipfs/build_call_hash
├─ [INFO ] house hub address : component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26
├─ [INFO ] land owner sbt_address : resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ] land owner sbt id : 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ] contractor sbt address : resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ] contractor sbt id : 300710000000985f0bf0a4fbceed30b544a167858b01
├─ [INFO ] land property nft: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ] land property nft_id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ] house project nft: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x
├─ [INFO ] house project nft id: 30071000000068230c0e0db22a129a64fb6bf1c52616
├─ [INFO ] property building nft: resource_sim1qqh9txs7vzlrcqcp9suesskwl4u5tewce2uhpan0uw7qwmwtva
├─ [INFO ] property building nft_id: 3007100000004b7e367b76f1ab21f045d96995b03490
├─ [INFO ] building surface: 25
├─ [INFO ] contract amount: 100
├─ [INFO ] deadline: 30021
├─ [INFO ] executed: true
└─ [INFO ] approved: true


```


> General Contractor claims contract payment 

> update data on ```collect_contract_payment```	

>```collect_contract_payment.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2113095 XRD burned, 0.010565475 XRD tipped to validators
Cost Units: 100000000 limit, 2113095 consumed, 0.0000001 XRD per cost unit
Logs: 0

```




[Back Up](#index)
#
### Part_9 
# Merge two contiguous Land AssetNFT properties into a single Land AssetNFT property. 
-------------------------------------------------------------------------------------------

> Check land owner SBT to verify two Land Property AssetNFT data are present within ```real_estate_properties``` field.
> Check ```data_4: parcel``` field to verify properties are contiguous.  


> Switch default account land owner

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> update data on ```ask_property_sbt```	

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3394194 XRD burned, 0.01697097 XRD tipped to validators
Cost Units: 100000000 limit, 3394194 consumed, 0.0000001 XRD per cost unit
Logs: 22
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https//ipfs.io/ipfs/land_asset_hash_1\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"0\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0101", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https//ipfs.io/ipfs/land_asset_hash_2\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"66\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />
		</svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0102", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008018133901c22bcb8174846623c71db4
├─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https//ipfs.io/ipfs/land_asset_hash_4\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"0\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />
		</svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0201", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
├─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https//ipfs.io/ipfs/land_asset_hash_5\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"66\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />
		</svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0202", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
└─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https//ipfs.io/ipfs/land_asset_hash_9\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"133\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />
		</svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0303", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }

```


> Select two contiguous land properties from land owner account

```
Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
Real estate property NFT id: 3007100000008018133901c22bcb8174846623c71db4
Parcel 0201

Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
Real estate property NFT id: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
Parcel 0202
```


> Merge two Land AssetNFT properties into a single Land AssetNFT property, as far as they are contiguous and minted by Neverland AssetFarm Component or
> a cloned authorized one to succesfully burn originals AssetNFTs once merged Land AssetNFT is mint.   

> update data on ```merge_properties```	

>```merge_properties.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.8123369 XRD burned, 0.040616845 XRD tipped to validators
Cost Units: 100000000 limit, 8123369 consumed, 0.0000001 XRD per cost unit
Logs: 11
├─ [INFO ] [merge_properties]: parcel_one:  Parcel 0201
├─ [INFO ] [merge_properties]: parcel_two:  Parcel 0202
├─ [INFO ] [merge_properties]: Mint code ID 02010202
├─ [INFO ]  comp_addr_one: component_sim1qtwfalw8frs3pgywpsheurr5phqwa28wq4tfmc9k3kassnfzgl
├─ [INFO ]  comp_addr_two: component_sim1qtwfalw8frs3pgywpsheurr5phqwa28wq4tfmc9k3kassnfzgl
├─ [INFO ]  NFT user address 9981 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 38
├─ [INFO ] asset_seed: 38
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 02010202
└─ [INFO ]  Academy TKN contribution amount 0.33
```


> Check land owner SBT to verify Land Property AssetNFT data has been succesfully updated within ```real_estate_properties``` field:
> new merged Land Property AssetNFT data has been saved and the data related to two original Land Property AssetNFT erased.

> update data on ```ask_property_sbt```	

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.305605 XRD burned, 0.01528025 XRD tipped to validators
Cost Units: 100000000 limit, 3056050 consumed, 0.0000001 XRD per cost unit
Logs: 18
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https//ipfs.io/ipfs/land_asset_hash_1\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"0\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0101", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https//ipfs.io/ipfs/land_asset_hash_2\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"66\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0102", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
├─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https//ipfs.io/ipfs/land_asset_hash_9\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"133\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 0303", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000f313a5e55bdfefd076547972e6cd5de8
└─ [INFO ]  Real estate property NFT data: AssetNFT { 
		uri: "https://ipfs.io/ipfs/merge_properties_hash\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\">
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" />
		<rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" />
		<rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />
		<rect x=\"0\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />
		<rect x=\"66\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", 
		data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", 
		data_4: " Parcel 02010202", value_1: 0, value_2: 0, value_3: 250, linked_assets: [] }

```

> Check new merged Land AssetNFT within land owner accound 

> update data on ```check_asset_nft```	

>```check_asset_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1361665 XRD burned, 0.006808325 XRD tipped to validators
Cost Units: 100000000 limit, 1361665 consumed, 0.0000001 XRD per cost unit
Logs: 7
├─ [INFO ]  data_1:  asset type Neverland Property Certificate
├─ [INFO ]  data_2:  Region North
├─ [INFO ]  data_3:  District NorthWest
├─ [INFO ]  data_4:  Parcel 02010202
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
└─ [INFO ]  value_3: 250

```

[Back Up](#index)
