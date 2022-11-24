--------------------------------------------------------------------------------------------------
# Land Asset NFT buying and selling operations Test  
--------------------------------------------------------------------------------------------------

Demo test of Land Asset NFT buying and selling operations between different protocols SBT users between interconnected marketplaces.
Sellers and buyers are required to provide their own SBT proof with the purpose of correctly update their data concerning buyed orsold
Land Asset NFT properties.
Tokens exchange between dirrent protocols currencies is performed by a linked DEX through externl component call.
A Demo Tools Utility Component is used to retrieve property data info from involved user's SBT with the purpose of verify them. 
AssetSquare Marketplace can handle buying and selling operations of Asset NFT bundles. 
For who desire test it Demo Tools Utility Component can mint a couple of different Asset NFT resources as well as insert property ownership data within 
user SBT.
HouseHub Component is used to retrieve property ownership data within user SBT.

N.B."Instructions" data tabs within Transactions output as well as other empty fields have been intentionally obmitted.

-------------------------------------------------------------------------------------------
# Index  
-------------------------------------------------------------------------------------------
>	
> [Part_1](#part_1) . AssetSquare Land Asset NFT marketplaces Components, tokens resource addresses and active accounts list and a set of Aseet NFT to sell.
>
> [Part_2](#part_2) . Check Land Asset property within SBT data of Neverland NFT seller account, Neverland NFT buyer account, Mahoroba NFT buyer account.
>
> [Part_3](#part_3) . NORMAL SELLING MODE. Stock a Land Asset NFT in normal selling mode within Neverland AssetSquare. Display Land Asset NFT selling instance.
>
> [Part_4](#part_4) . Buy Neverland AssetSquare listed Land Asset NFT from Mahoroba AssetSquare with Mahoroba buyer account.
>
> [Part_5](#part_5) . Collect Land Asset NFT selling instance accrued payment and verify property hasbeen erased form Neverland seller user SBT data. 
>
> [Part_6](#part_6) . Stock a Land Asset NFT in normal selling mode within Neverland AssetSquare. Display Land Asset NFT selling instance.
>
> [Part_7](#part_7) . Place a buy proposal on Land Asset NFT selling instance on Neverland Assetsquare from Mahoroba AssetSquare.
>
> [Part_8](#part_8) . AUCTION SELLING MODE. Stock a Land Asset NFT in auction selling mode within Neverland AssetSquare. Display Land Asset NFT selling instance.
>
> [Part_9](#part_9) . Mahoroba buyer place a bid on a Land Asset NFT auction instance on Neverland AssetSquare Component.
>
> [Part_10](#part_10) . Check auction status and withdrawal auction payment. Check Land Asset NFTs property ownership within seller SBT.
>
> [Part_11](#part_11) . RAFFLE SELLING MODE. Stock a Land Asset NFT in auction selling mode within Neverland AssetSquare. Display Land Asset NFT selling instance.
>
> [Part_12](#part_12). Display listed Land Asset NFT. Buy some raffle tickets with a Neverland buyer account. Overtake raffle deadline.
>
> [Part_13](#part_13) . Check raffle status and withdrawal raffle jackpot. Check Land Asset NFTs property ownership within seller SBT.
>


#
### Part_1 
# AssetSquare Land Asset NFT marketplaces Components, tokens resource addresses and active accounts list and a set of Aseet NFT to sell
-------------------------------------------------------------------------------------------

>
> AssetSquare Land Asset NFT marketplaces Components
>

>```Neverland Auction Component
```
├─ Component: component_sim1q2d9c9wus40556ujg386n3ada7qmv72lctr2lx6y5m3sg733dr		Neverland AssetSquare component
```

>```Neverland Auction Component
```
├─ Component: component_sim1qfxgwghn69wz9sqdcm7vely8jy99xa9f00h9yjqyn54stf09zp		Mahoroba AssetSquare component
```

>
> Utility Components
>

>```DemoTools Component
```
└─ Component: component_sim1qtaxue485rswfzcx9dqcufv2xdl0u0wuuv28a590tqrqp02pvl		DemoTools Component address
```

>```HouseHub Component
```
└─ Component: component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26		Neverland HouseHub Component address
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
> neverland's land owner account and SBT resource address & ID
>

```
Account component address: account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t	
Public key: 03dae07d865f8902053911403291fa606d78f0081a40d65aa3ca0b7fd978ac5162
Private key: c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7

SBT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f 
SBT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
```

>
> neverland's land buyer account and SBT resource address & ID
>

```
Account component address: account_sim1q0whr39q2md5sdd7mlv5t6h9efrkvatfr62rgcyc24cs76sksn	
Public key: 0280c639797f8331769adac01c022d8063719640dd8bc94b5c32dc1a0c0b2de63e
Private key: ac1ddbf111dc1975c25b64d88de6204e84114762c4afbbe489cc2ccce6a580c4

SBT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
SBT id: 30071000000064b087e4aef4a0e5e1886a379ebdce90
```

>
> mahoroba's land buyer account and SBT resource address & ID
>

```
Account component address: account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman
Public key: 31a65b8e4070e543dba7daffa3703ea8dcbb2b472acc714cec390bfc6f843cfef
Private key: ed5bc42f40e114e0e1de7da0858b4929974ab6ea7f8cde61420355e09d336eb2

SBT resource address: resource_sim1qp4ssprn6cp053pwt5h6y2a7jxyjcz5jhcnqk2s460tqceylq5
SBT id: 30071000000056209feaa224368e6e9350ae01e2594f
```


>
> token resources 
>

```
Resource: resource_sim1qzlwc3akklnq0z6xmssar3998xnw9ezsfzyz38tafv4sa9ft2g		MahorobaLand environment currency $ONE
```

```
Resource: resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe		Neverland environment currency $TKN
```


>
> Minted Asset NFT to use within test
>

```
├─ [INFO ]  NFT ID: 300710000000e472963393192a86550334cbc2fe7a8e
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0103
```
>

```
├─ [INFO ]  NFT ID: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0203
```

>

```
├─ [INFO ]  NFT ID: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0301
```

>

```
├─ [INFO ]  NFT ID: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0302
```

>

```
├─ [INFO ]  NFT ID: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0303
```


#
### Part_2 
# Check Land Asset property within SBT data of Neverland NFT seller account, Neverland NFT buyer account, Mahoroba NFT buyer account
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> Check Land Asset NFTs property ownership within seller SBT

> cd house_hub_transaction_manifest

> update data on ```ask_property_sbt```

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.4333105 XRD burned, 0.021665525 XRD tipped to validators
Cost Units: 100000000 limit, 4333105 consumed, 0.0000001 XRD per cost unit
Logs: 38
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_1\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0101", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_2\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0102", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008018133901c22bcb8174846623c71db4
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_4\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0201", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_5\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0202", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_7\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0301", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_8\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0302", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e472963393192a86550334cbc2fe7a8e
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_3\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0103", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_6\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0203", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_9\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0303", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }

```


> Switch default account

```resim set-default-account account_sim1q0whr39q2md5sdd7mlv5t6h9efrkvatfr62rgcyc24cs76sksn ac1ddbf111dc1975c25b64d88de6204e84114762c4afbbe489cc2ccce6a580c4```


> Check Land Asset NFTs property ownership within Neverland buyer SBT and verify it's empty

> update data on ```ask_property_sbt```

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0974093 XRD burned, 0.004870465 XRD tipped to validators
Cost Units: 100000000 limit, 974093 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
└─ [INFO ]  Land Owner SBT NFT id: 30071000000064b087e4aef4a0e5e1886a379ebdce90

```


> Switch default account

```resim set-default-account account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman ed5bc42f40e114e0e1de7da0858b4929974ab6ea7f8cde61420355e09d336eb2```


> Check Land Asset NFTs property ownership within Mahoroba Land buyer SBT and verify it's empty

> update data on ```ask_property_sbt_ext```

>```ask_property_sbt_ext.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0986926 XRD burned, 0.00493463 XRD tipped to validators
Cost Units: 100000000 limit, 986926 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qp4ssprn6cp053pwt5h6y2a7jxyjcz5jhcnqk2s460tqceylq5
└─ [INFO ]  Land Owner SBT NFT id: 30071000000056209feaa224368e6e9350ae01e2594f

```


[Back Up](#index)
#
### Part_3 NORMAL SELLING MODE
# Stock a Land Asset NFT in normal selling mode within Neverland AssetSquare  
# Display Land Asset NFT selling instance 
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> Stock one Land Asset NFT in normal selling mode on Neverland AssetSquare Component

> cd asset_square_transaction_manifest

> update data on ```stock_nft```

>```stock_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.449262 XRD burned, 0.0224631 XRD tipped to validators
Cost Units: 100000000 limit, 4492620 consumed, 0.0000001 XRD per cost unit
Logs: 8
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Instance number: 1
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Added 1 NFT, resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy ResAddress Asset Ecosystem Neverland mint land Series 1 Number
├─ [INFO ]  UUID: 300710000000e472963393192a86550334cbc2fe7a8e Data:  asset type Neverland Property Certificate  Region North @20 $TKN
├─ [INFO ]  meta_NFT_id 3007100000009935e3542d01e0c95bd90535cd26257a
├─ [INFO ]  meta_NFT_res_addr resource_sim1qp20js4hp66ve8eyhjhpue4dzc8j8ekmsufp4hswfavsff7rqy
└─ [INFO ]  =====================================================================================
|
New Entities: 2
├─ Resource: resource_sim1qp20js4hp66ve8eyhjhpue4dzc8j8ekmsufp4hswfavsff7rqy	MetaNFT resource address
└─ Resource: resource_sim1qr0rf6t6tz2vl44ewgjxhkk70d0a6f7du5r5lwyyu3dszp7lv5	Seller Badge
```


> Display listed Land Asset NFT 

> update data on ```ask_instance```

>```ask_instance.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2238049 XRD burned, 0.011190245 XRD tipped to validators
Cost Units: 100000000 limit, 2238049 consumed, 0.0000001 XRD per cost unit
Logs: 14
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000e472963393192a86550334cbc2fe7a8e
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  NFT on Sell
            Instance number : 1
            Price: 20 $TKN
            Buy proposal: 0 $TKN
            Deadline: 0
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0103
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================

```


[Back Up](#index)
#
### Part_4 
# Buy Neverland AssetSquare listed Land Asset NFT from Mahoroba AssetSquare with Mahoroba buyer account
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman ed5bc42f40e114e0e1de7da0858b4929974ab6ea7f8cde61420355e09d336eb2```


> update data on ```buy_nft_ext```

>```buy_nft_ext.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 1.2552292 XRD burned, 0.06276146 XRD tipped to validators
Cost Units: 100000000 limit, 12552292 consumed, 0.0000001 XRD per cost unit
Logs: 10
├─ [INFO ]  Asset Dex external currency output amount: 83.8221953432113698
├─ [INFO ]  Requested amount: 20
├─ [INFO ]  Academy TKN contribution amount 0.33
├─ [INFO ]  NFT sell net gain 19
├─ [INFO ]  Rest 19
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT collected
├─ [INFO ]  resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  key: 300710000000e472963393192a86550334cbc2fe7a8e
└─ [INFO ]  =====================================================================================

```


> Check Land Asset NFTs property ownership within Mahoroba Land buyer SBT and verify one NFT has been buyed 

> cd house_hub_transaction_manifest

> update data on ```ask_property_sbt_ext```

>```ask_property_sbt_ext.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1358934 XRD burned, 0.00679467 XRD tipped to validators
Cost Units: 100000000 limit, 1358934 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qp4ssprn6cp053pwt5h6y2a7jxyjcz5jhcnqk2s460tqceylq5
├─ [INFO ]  Land Owner SBT NFT id: 30071000000056209feaa224368e6e9350ae01e2594f
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e472963393192a86550334cbc2fe7a8e
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_3\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0103", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }

```



[Back Up](#index)
#
### Part_5
# Collect Land Asset NFT selling instance accrued payment and verify property hasbeen erased form Neverland seller user SBT data
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> Collect Land Asset NFT selling instance accrued payment returning provided MetaNFT and providing personal SBT to update property ownership

> cd asset_square_transaction_manifest

> update data on ```collect_payment```

>```collect_payment.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.6406597 XRD burned, 0.032032985 XRD tipped to validators
Cost Units: 100000000 limit, 6406597 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Accrued tokens 19
└─ [INFO ]  NFT accrued selling amount 19
```

> Check Land Asset NFTs property ownership within seller SBT

> cd house_hub_transaction_manifest

> update data on ```ask_property_sbt```

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3944921 XRD burned, 0.019724605 XRD tipped to validators
Cost Units: 100000000 limit, 3944921 consumed, 0.0000001 XRD per cost unit
Logs: 34
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_1\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0101", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_2\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0102", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008018133901c22bcb8174846623c71db4
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_4\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0201", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_5\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0202", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_7\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0301", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_8\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0302", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_6\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0203", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_9\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0303", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
```


[Back Up](#index)
#
### Part_6 
# Stock a Land Asset NFT in normal selling mode within Neverland AssetSquare. Display Land Asset NFT selling instance  
-------------------------------------------------------------------------------------------

> Stock one Land Asset NFT in normal selling mode on Neverland AssetSquare Component

> cd asset_square_transaction_manifest

> update data on ```stock_nft```

>```stock_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.4174711 XRD burned, 0.020873555 XRD tipped to validators
Cost Units: 100000000 limit, 4174711 consumed, 0.0000001 XRD per cost unit
Logs: 8
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Instance number: 2
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Added 1 NFT, resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy ResAddress Asset Ecosystem Neverland mint land Series 1 Number
├─ [INFO ]  UUID: 300710000000e775db9b13a353bdeb5021e2d9a4e92f Data:  asset type Neverland Property Certificate  Region North @20 $TKN
├─ [INFO ]  meta_NFT_id 300710000000b59b8303e4edc21b5164a38bc5985611
├─ [INFO ]  meta_NFT_res_addr resource_sim1qp20js4hp66ve8eyhjhpue4dzc8j8ekmsufp4hswfavsff7rqy
└─ [INFO ]  =====================================================================================
|
New Entities: 1
└─ Resource: resource_sim1qqfv6pjnvd2tlxr2289l0kya6fgxt7wmrrlh9ynkvn8se00crw	Seller Badge 
```


> Display listed Land Asset NFT 

> update data on ```ask_instance```

>```ask_instance.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2302067 XRD burned, 0.011510335 XRD tipped to validators
Cost Units: 100000000 limit, 2302067 consumed, 0.0000001 XRD per cost unit
Logs: 14
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  NFT on Sell
            Instance number : 2
            Price: 20 $TKN
            Buy proposal: 0 $TKN
            Deadline: 0
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0203
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================

```



[Back Up](#index)
#
### Part_7 
# Place a buy proposal on Land Asset NFT selling instance on Neverland Assetsquare from Mahoroba AssetSquare.
# Check selling instance status. Seller accept the buy proposal collecting provided proposal payment. Check seller SBT property data.
# Check if buy proposal hasbeen accepted providing Buyer Badge. Collect Land Asset NFT.   
----------------------------------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman ed5bc42f40e114e0e1de7da0858b4929974ab6ea7f8cde61420355e09d336eb2```


> update data on ```buy_proposal_ext```

>```buy_proposal_ext.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.8837427 XRD burned, 0.044187135 XRD tipped to validators
Cost Units: 100000000 limit, 8837427 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Asset Dex external currency output amount: 39.959552055217621279
|
New Entities: 1
└─ Resource: resource_sim1qqqgmx0yufhqz2fv6328hz6ewke4c6uwc2k4xvzu9hvs99k2w8	Buyer Badge
```

> Switch default account

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```

> cd asset_square_transaction_manifest

> Check selling instance status on Seller's history providing Seller Badge 

> update data on ```ask_position.sh```

>```ask_position.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2389502 XRD burned, 0.01194751 XRD tipped to validators
Cost Units: 100000000 limit, 2389502 consumed, 0.0000001 XRD per cost unit
Logs: 14
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  NFT on Sell
            Instance number : 2
            Price: 20 $TKN
            Buy proposal: 19 $TKN
            Deadline: 16010
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0203
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================
```


> Accept buy proposal on Land Asset NFT sale collecting deposited proposal payment. 
> Return provided MetaNFT and provide personal SBT to update property ownership.

> update data on ```collect_buy_proposal```

>```collect_buy_proposal.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.7620672 XRD burned, 0.03810336 XRD tipped to validators
Cost Units: 100000000 limit, 7620672 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Academy TKN contribution amount 0.3135
└─ [INFO ]  NFT sell net gain 18.05
```

> Check Land Asset NFTs property ownership within seller SBT

> cd house_hub_transaction_manifest

> update data on ```ask_property_sbt```

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3587366 XRD burned, 0.01793683 XRD tipped to validators
Cost Units: 100000000 limit, 3587366 consumed, 0.0000001 XRD per cost unit
Logs: 30
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_1\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0101", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_2\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0102", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008018133901c22bcb8174846623c71db4
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_4\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0201", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_5\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0202", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_7\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0301", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_8\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0302", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_9\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0303", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
```

> Switch default account

```resim set-default-account account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman ed5bc42f40e114e0e1de7da0858b4929974ab6ea7f8cde61420355e09d336eb2```

> Check buy proposal status providing Buyer Badge 

> cd asset_square_transaction_manifest

> update data on ```ask_position.sh```

>```ask_position.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1786335 XRD burned, 0.008931675 XRD tipped to validators
Cost Units: 100000000 limit, 1786335 consumed, 0.0000001 XRD per cost unit
Logs: 14
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Buy proposal Accepted.
            Instance number : 2
            Payed amount: 19 $TKN
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0203
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================

```

> Buy proposal results accepted. Collect Land Asset NFT

> update data on ```reclaim_buy_proposal```

>```reclaim_buy_proposal.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3863087 XRD burned, 0.019315435 XRD tipped to validators
Cost Units: 100000000 limit, 3863087 consumed, 0.0000001 XRD per cost unit
Logs: 5
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT collected
├─ [INFO ]  resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  key: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
└─ [INFO ]  =====================================================================================

```

> Check Land Asset NFTs property ownership within Mahoroba Land buyer SBT and verify one NFT has been buyed 

> cd house_hub_transaction_manifest

> update data on ```ask_property_sbt_ext```

>```ask_property_sbt_ext.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1730711 XRD burned, 0.008653555 XRD tipped to validators
Cost Units: 100000000 limit, 1730711 consumed, 0.0000001 XRD per cost unit
Logs: 10
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qp4ssprn6cp053pwt5h6y2a7jxyjcz5jhcnqk2s460tqceylq5
├─ [INFO ]  Land Owner SBT NFT id: 30071000000056209feaa224368e6e9350ae01e2594f
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e472963393192a86550334cbc2fe7a8e
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_3\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0103", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_6\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0203", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }

```

[Back Up](#index)
#
### Part_8 AUCTION SELLING MODE 
# Stock a Land Asset NFT in auction selling mode within Neverland AssetSquare.   
# Display Land Asset NFT selling instance 
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> Stock one Land Asset NFT in normal selling mode on Neverland AssetSquare Component

> cd asset_square_transaction_manifest

> update data on ```stock_nft```

>```stock_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.4247324 XRD burned, 0.02123662 XRD tipped to validators
Cost Units: 100000000 limit, 4247324 consumed, 0.0000001 XRD per cost unit
Logs: 8
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Instance number: 3
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Added 1 NFT, resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy ResAddress Asset Ecosystem Neverland mint land Series 1 Number
├─ [INFO ]  UUID: 3007100000008678ffba9d10934c51efb93fa3292062 Data:  asset type Neverland Property Certificate  Region North @0 $TKN
├─ [INFO ]  meta_NFT_id 300710000000310dcc7a64af0193d9881ee2dd6fe86a
├─ [INFO ]  meta_NFT_res_addr resource_sim1qp20js4hp66ve8eyhjhpue4dzc8j8ekmsufp4hswfavsff7rqy
└─ [INFO ]  =====================================================================================

```


> Check auction instance status on Seller's history providing Seller Badge 

> update data on ```ask_position```

>```ask_position.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3941116 XRD burned, 0.01970558 XRD tipped to validators
Cost Units: 100000000 limit, 3941116 consumed, 0.0000001 XRD per cost unit
Logs: 28
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  NFT on Auction.
            Instance number : 3
            Reserve price: 20 $TKN
            Highest bid: 3 $TKN
            Deadline: 16010
            Bid bond: 4
            Last minute bid war deadline: 16015
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0301
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
├─ [INFO ]  =====================================================================================
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Buy proposal Accepted.
            Instance number : 2
            Payed amount: 19 $TKN
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0203
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================
```

[Back Up](#index)
#
### Part_9 
# Mahoroba buyer place a bid on a Land Asset NFT auction instance on Neverland AssetSquare Component 
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman ed5bc42f40e114e0e1de7da0858b4929974ab6ea7f8cde61420355e09d336eb2```

> Place a bid on a Land Asset NFT auction instance on Neverland AssetSquare Component

> update data on ```place_bid_ext```

>```place_bid_ext.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.8515736 XRD burned, 0.04257868 XRD tipped to validators
Cost Units: 100000000 limit, 8515736 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Asset Dex external currency output amount: 39.959511563771646879
|
New Entities: 1
└─ Resource: resource_sim1qqx3l49hycdsjzkcywcrw0nugyfqsdnxczk6x689hc8s738m7p	Bidder Badge
```


> Overtake auction deadline

> ```resim set-current-epoch 16020```



> Display Land Asset NFT instance auction result

> update data on ```ask_instance```

>```ask_instance.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3070422 XRD burned, 0.01535211 XRD tipped to validators
Cost Units: 100000000 limit, 3070422 consumed, 0.0000001 XRD per cost unit
Logs: 14
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Auction ended.
            Instance number : 3
            Reserve price: 20 $TKN
            Winning bid: 22 $TKN
            Payment deadline: 21010
            Bid bond: 4
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0301
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================
```


> Provide payment on Mahoroba AssetSquare to honour won auction held on Neverland AssetSquare in exchange for the Land Asset NFT.

> update data on ```pay_winner_bid```

>```pay_winner_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.8629904 XRD burned, 0.04314952 XRD tipped to validators
Cost Units: 100000000 limit, 8629904 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Asset Dex external currency output amount: 35.967168176281456175
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT collected
├─ [INFO ]  resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  key: 3007100000008678ffba9d10934c51efb93fa3292062
└─ [INFO ]  =====================================================================================
```

> Check Land Asset NFTs property ownership within Mahoroba Land buyer SBT and verify one NFT has been buyed 

> cd house_hub_transaction_manifest

>```ask_property_sbt_ext.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2101069 XRD burned, 0.010505345 XRD tipped to validators
Cost Units: 100000000 limit, 2101069 consumed, 0.0000001 XRD per cost unit
Logs: 14
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qp4ssprn6cp053pwt5h6y2a7jxyjcz5jhcnqk2s460tqceylq5
├─ [INFO ]  Land Owner SBT NFT id: 30071000000056209feaa224368e6e9350ae01e2594f
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e472963393192a86550334cbc2fe7a8e
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_3\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0103", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_6\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0203", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008678ffba9d10934c51efb93fa3292062
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_7\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0301", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }

```


[Back Up](#index)
#
### Part_10
# Check auction status and withdrawal auction payment. Check Land Asset NFTs property ownership within seller SBT. 
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```



> Check auction instance status on Seller's history providing Seller Badge  

> cd asset_square_transaction_manifest

> update data on ```ask_position```

>```ask_position.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3464954 XRD burned, 0.01732477 XRD tipped to validators
Cost Units: 100000000 limit, 3464954 consumed, 0.0000001 XRD per cost unit
Logs: 28
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Buy proposal Accepted.
            Instance number : 2
            Payed amount: 19 $TKN
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0203
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
├─ [INFO ]  =====================================================================================
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Auction honored & payment withdrawable.
            Instance number : 3
            Reserve price: 20 $TKN
            Accrued amount: 22 $TKN
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0301
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================

```

> Withdrawal Land Asset NFT auction instance payment. 
> Return provided MetaNFT and provide personal SBT to update property ownership.

> update data on ```collect_auction_payment```

>```collect_auction_payment.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.7437581 XRD burned, 0.037187905 XRD tipped to validators
Cost Units: 100000000 limit, 7437581 consumed, 0.0000001 XRD per cost unit
Logs: 3
├─ [INFO ]  accrued_amount 22
├─ [INFO ]  Academy TKN contribution amount 0.363
└─ [INFO ]  NFT sell net gain 20.9
```

> Check Land Asset NFTs property ownership within seller SBT

> cd house_hub_transaction_manifest

> update data on ```ask_property_sbt```

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3203483 XRD burned, 0.016017415 XRD tipped to validators
Cost Units: 100000000 limit, 3203483 consumed, 0.0000001 XRD per cost unit
Logs: 26
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_1\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0101", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_2\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0102", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008018133901c22bcb8174846623c71db4
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_4\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0201", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_5\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0202", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_8\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0302", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_9\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0303", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
```

[Back Up](#index)
#
### Part_11 RAFFLE SELLING MODE 
# Stock a Land Asset NFT in auction selling mode within Neverland AssetSquare.   
# Display Land Asset NFT selling instance
-------------------------------------------------------------------------------------------

> Stock one Land Asset NFT in normal selling mode on Neverland AssetSquare Component

> cd asset_square_transaction_manifest

> update data on ```stock_nft```

>```stock_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.4578123 XRD burned, 0.022890615 XRD tipped to validators
Cost Units: 100000000 limit, 4578123 consumed, 0.0000001 XRD per cost unit
Logs: 8
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Instance number: 4
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Added 1 NFT, resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy ResAddress Asset Ecosystem Neverland mint land Series 1 Number
├─ [INFO ]  UUID: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c Data:  asset type Neverland Property Certificate  Region North @0 $TKN
├─ [INFO ]  meta_NFT_id 300710000000dcca8241cb578a2f0c80eb1304afeafc
├─ [INFO ]  meta_NFT_res_addr resource_sim1qp20js4hp66ve8eyhjhpue4dzc8j8ekmsufp4hswfavsff7rqy
└─ [INFO ]  =====================================================================================
```


> Check auction instance status on Seller's history providing Seller Badge 


>```ask_position.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5525585 XRD burned, 0.027627925 XRD tipped to validators
Cost Units: 100000000 limit, 5525585 consumed, 0.0000001 XRD per cost unit
Logs: 42
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Auction honored & payment withdrawable.
            Instance number : 3
            Reserve price: 20 $TKN
            Accrued amount: 22 $TKN
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0301
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
├─ [INFO ]  =====================================================================================
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  NFT on Raffle.
            Instance number : 4
            Reserve price: 100 $TKN
            Jackpot: 0
            Ticket price: 1 $TKN
            Deadline: 20020
            Tickets sold: 0
            Last minute tickets fomo deadline: 20025
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0302
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
├─ [INFO ]  =====================================================================================
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Buy proposal Accepted.
            Instance number : 2
            Payed amount: 19 $TKN
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0203
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================

```


[Back Up](#index)
#
### Part_12 
# Display listed Land Asset NFT. Buy some raffle tickets with a Neverland buyer account. Overtake raffle deadline.
# Display raffle listed Land Asset NFT result and winner ticket. Reclaim winner ticket and collect NFT. Check SBT data.
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1q0whr39q2md5sdd7mlv5t6h9efrkvatfr62rgcyc24cs76sksn ac1ddbf111dc1975c25b64d88de6204e84114762c4afbbe489cc2ccce6a580c4```


> Display listed Land Asset NFT 

> update data on ```ask_instance```

>```ask_instance.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3021666 XRD burned, 0.01510833 XRD tipped to validators
Cost Units: 100000000 limit, 3021666 consumed, 0.0000001 XRD per cost unit
Logs: 14
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  NFT on Raffle.
            Instance number : 4
            Reserve price: 0 $TKN
            Jackpot: 0
            Ticket price: 1 $TKN
            Deadline: 20020
            Tickets sold: 0
            Last minute tickets fomo deadline: 20025
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0302
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================
```

> Buy some raffle tickets with a Neverland buyer account.

> update data on ```buy_ticket```

>```buy_ticket.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 2.6613427 XRD burned, 0.133067135 XRD tipped to validators
Cost Units: 100000000 limit, 26613427 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 4
├─ Resource: resource_sim1qrpgg84qxd0sqrquf52s7nskv3gre64k8kp859e82trq2aw27n	Raffle Buyer Badge 1
├─ Resource: resource_sim1qqz72xjexgj8snn03dew2nu6gffs42e5t8t0zsjycznsp3vaeu	Raffle Buyer Badge 2
├─ Resource: resource_sim1qzm4kptv078qv2dmlcgdet0875v35vp6jt3984erqfnsy7au9y	Raffle Buyer Badge 3
└─ Resource: resource_sim1qqme88kuvr7qllpw0vy6lcaeex2y7tg0gpytwgngv5ashyrkad	Raffle Buyer Badge 4
```

> Overtake auction deadline

> ```resim set-current-epoch 20021```


> Display listed Land Asset NFT raffle result and winne ticket ID 

>```ask_instance.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.844591 XRD burned, 0.04222955 XRD tipped to validators
Cost Units: 100000000 limit, 8445910 consumed, 0.0000001 XRD per cost unit
Logs: 20
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Instance number: 4
├─ [INFO ]  -------------------------------------------------------------------------------------
├─ [INFO ]  Winner ID: 88201871148152778410797637236293437554
├─ [INFO ]  Winner Badge: resource_sim1qzm4kptv078qv2dmlcgdet0875v35vp6jt3984erqfnsy7au9y
├─ [INFO ]  -------------------------------------------------------------------------------------
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Raffle ended.
            Instance number : 4
            Reserve price: 100 $TKN
            Jackpot: 100
            Ticket price: 1 $TKN
            Deadline: 20020
            Tickets sold: 100
            Winner ticket: 20025
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0302
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================
```


> Collect won Land Asset NFT providing raffle winner ticket.

> update data on ```reclaim_winner_ticket```

>```reclaim_winner_ticket.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 2.1382882 XRD burned, 0.10691441 XRD tipped to validators
Cost Units: 100000000 limit, 21382882 consumed, 0.0000001 XRD per cost unit
Logs: 5
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT collected
├─ [INFO ]  resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  key: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
└─ [INFO ]  =====================================================================================
```

> Check Land Asset NFTs property ownership within Mahoroba Land buyer SBT and verify one NFT has been included 

> cd house_hub_transaction_manifest

> update data on ```ask_property_sbt```

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1345466 XRD burned, 0.00672733 XRD tipped to validators
Cost Units: 100000000 limit, 1345466 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 30071000000064b087e4aef4a0e5e1886a379ebdce90
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_8\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0302", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
```


[Back Up](#index)
#
### Part_13 
# Check raffle status and withdrawal raffle jackpot. Check Land Asset NFTs property ownership within seller SBT. 
-------------------------------------------------------------------------------------------

> Switch default account

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> cd asset_square_transaction_manifest


> Check auction instance status on Seller's history providing Seller Badge 


>```ask_position.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5525585 XRD burned, 0.027627925 XRD tipped to validators
Cost Units: 100000000 limit, 5525585 consumed, 0.0000001 XRD per cost unit
Logs: 42
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5636577 XRD burned, 0.028182885 XRD tipped to validators
Cost Units: 100000000 limit, 5636577 consumed, 0.0000001 XRD per cost unit
Logs: 42
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Raffle ended.
            Instance number : 4
            Reserve price: 100 $TKN
            Jackpot: 100
            Ticket price: 1 $TKN
            Deadline: 20020
            Tickets sold: 100
            Winner ticket: 20025
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0302
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
├─ [INFO ]  =====================================================================================
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Buy proposal Accepted.
            Instance number : 2
            Payed amount: 19 $TKN
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0203
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
├─ [INFO ]  =====================================================================================
├─ [INFO ]  =====================================================================================
├─ [INFO ]  NFT: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  NFT key: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]  Auction honored & payment withdrawable.
            Instance number : 3
            Reserve price: 20 $TKN
            Accrued amount: 22 $TKN
├─ [INFO ] --------------------------------------------------------------------------------------
├─ [INFO ]   asset type Neverland Property Certificate
├─ [INFO ]   Region North
├─ [INFO ]   District NorthWest
├─ [INFO ]   Parcel 0301
├─ [INFO ]  value_1: 0
├─ [INFO ]  value_2: 0
├─ [INFO ]  value_3: 125
└─ [INFO ]  =====================================================================================

```

> Withdrawal Land Asset NFT raffle instance jackpot. 
> Return provided MetaNFT and provide personal SBT to update property ownership.

> update data on ```collect_raffle_jackpot```

>```collect_raffle_jackpot.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.7862906 XRD burned, 0.03931453 XRD tipped to validators
Cost Units: 100000000 limit, 7862906 consumed, 0.0000001 XRD per cost unit
Logs: 3
├─ [INFO ]  amount : 100
├─ [INFO ]  Academy TKN contribution amount 1.65
└─ [INFO ]  NFT sell net gain 95
```

> Check Land Asset NFTs property ownership within seller SBT

> cd house_hub_transaction_manifest

> update data on ```ask_property_sbt```

>```ask_property_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2827906 XRD burned, 0.01413953 XRD tipped to validators
Cost Units: 100000000 limit, 2827906 consumed, 0.0000001 XRD per cost unit
Logs: 22
├─ [INFO ]  Land Owner SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Land Owner SBT NFT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_1\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0101", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_2\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"0\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0102", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 3007100000008018133901c22bcb8174846623c71db4
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_4\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"0\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0201", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
├─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_5\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"66\" y=\"66\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0202", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
├─ [INFO ]  =========================================================================
├─ [INFO ]  Real estate property NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Real estate property NFT id: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
└─ [INFO ]  Real estate property NFT data: AssetNFT { uri: "https//ipfs.io/ipfs/land_asset_hash_9\" \n \"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"600\"><rect x=\"0\" y=\"0\" width=\"600\" height=\"600\" stroke=\"black\" stroke-width=\"5\" fill=\"plum\" /><rect x=\"0\" y=\"0\" width=\"600\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"lightgreen\" /><rect x=\"0\" y=\"0\" width=\"200\" height=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" /><rect x=\"133\" y=\"133\" width=\"66\" height=\"66\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" /></svg>", data_1: " asset type Neverland Property Certificate", data_2: " Region North", data_3: " District NorthWest", data_4: " Parcel 0303", value_1: 0, value_2: 0, value_3: 125, linked_assets: [] }
```



[Back Up](#index)
