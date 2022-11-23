--------------------------------------------------------------------------------------------------
# Land Asset NFT Auction and Mint Test  
--------------------------------------------------------------------------------------------------

In this test we list some Land Asset NFT in related auction instances on Land Auction Component.
A registered Neverland user place some winning bids on some auction instances and, once provided required auction payments, 
retrieve his Land Asset NFT once minted by Asset Farm and related components system (Pitia oracle and NFT data component). 
A share of auctions selling profit goes to the Academy component Vault, as contribution to academy's courses of study.     


N.B."Instructions" data tabs within Transactions output as well as other empty fields have been intentionally obmitted.

-------------------------------------------------------------------------------------------
# Index  
-------------------------------------------------------------------------------------------	
>
> [Part_1](#part_1) . Neverland Auction Component and resource addresses and active list
>
> [Part_2](#part_2) . Launch a Neverland new land auction bundle, switch to auction bidder account and display open auctions list.
>
> [Part_3](#part_3) . Register bidder and place a bid on auction instance number 1
>
> [Part_4](#part_4) . Register bidder and place a bid on auction instance number 2
>
> [Part_5](#part_5) . Register bidder and place a bid on auction instance number 3 
>
> [Part_6](#part_6) . Register bidder and place a bid on auction instance number 4
>
> [Part_7](#part_7) . Register bidder and place a bid on auction instance number 5   
>
> [Part_8](#part_8) . Register bidder and place a bid on auction instance number 6 
>
> [Part_9](#part_9) . Register bidder and place a bid on auction instance number 7
>
> [Part_10](#part_10) . Register bidder and place a bid on auction instance number 8   
>
> [Part_11](#part_11) . Register bidder and place a bid on auction instance number 9
>
> [Part_12](#part_12) . Claim land asset of won auction instances providing bidder badges and related auction payments
>
> [Part_13](#part_13) . Claim payments of succesful auction instances
>


#
### Part_1 
# Neverland Auction Component and resource addresses and active accounts list and protocol currency
-------------------------------------------------------------------------------------------

>```Neverland Auction Component
```
└─ Component: component_sim1qtvk9300ckmwysd5z9tk04sk0ksquu5ap5qupqre3kmsadca6v		Neverland Land Auction Component address
├─ Resource: resource_sim1qq077g24fwpd77wnvfkll7avnwqgac4sxfz2my4du0rqkf26nn		Neverland Auction MinterBadge
└─ Resource: resource_sim1qq2lveq6jk38as3xj28pekfm6d3vkls4mvl34zce280sq3ktty		Neverland Auction OwnerBadge
```

>
> protocol's owner account
>
```
Account component address: account_sim1qwk73ye3gfmnxnw42jgpv3gey9jj8a50se753pvnccfquqkgk3	
Public key: 0383ffb219b35c04f26db0a1e8efb9efec16fdd931aef837512bd60aa172342fa4
Private key: 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28
No configuration found on system. will use the above account as default.
```

>
> neverland's land owner account and SBT resource address & ID
>
```
Account component address: account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t	
Public key: 03dae07d865f8902053911403291fa606d78f0081a40d65aa3ca0b7fd978ac5162
Private key: c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7

User SBT address added: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
User SBT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
```

>
> Neverland environment currency $TKN
>
```
Resource: resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe		
```


#
### Part_2
# Launch a Neverland new land auction bundle, switch to auction bidder account and display open auctions list
-------------------------------------------------------------------------------------------

> Switch default account to protocol owner

```resim set-default-account account_sim1qwk73ye3gfmnxnw42jgpv3gey9jj8a50se753pvnccfquqkgk3 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28```


> cd land_auction_transaction_manifest

> update data on ```new_land_auction```

>```new_land_auction.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.246819 XRD burned, 0.01234095 XRD tipped to validators
Cost Units: 100000000 limit, 2468190 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0

```

> Switch default account to auction bidder account

```resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7```


> update data on ```auction_list```

>```auction_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.6455743 XRD burned, 0.032278715 XRD tipped to validators
Cost Units: 100000000 limit, 6455743 consumed, 0.0000001 XRD per cost unit
Logs: 18
├─ [INFO ] ======================================================================
├─ [INFO ]  Auction instance number: 1
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0101
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_1
├─ [INFO ] ======================================================================
├─ [INFO ]  Auction instance number: 2
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0102
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_2
├─ [INFO ] ======================================================================
├─ [INFO ]  Auction instance number: 3
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0103
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_3
├─ [INFO ] ======================================================================
├─ [INFO ]  Auction instance number: 4
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0201
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_4
├─ [INFO ] ======================================================================
├─ [INFO ]  Auction instance number: 5
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0202
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_5
├─ [INFO ] ======================================================================
├─ [INFO ]  Auction instance number: 6
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0202
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_6
├─ [INFO ] ======================================================================
├─ [INFO ]  Auction instance number: 7
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0301
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_7
├─ [INFO ] ======================================================================
├─ [INFO ]  Auction instance number: 8
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0302
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_8
├─ [INFO ] ======================================================================
└─ [INFO ]  Auction instance number: 9
                        Auction deadline: 12007
                        Auction bid bond: 5
                        Auction payment deadline: 13507
                        Land Asset Parcel: parcel 0303
                        Land Asset NFT amount: 1
                        Land Asset surface: 125
                        Land Asset linked URL: https//ipfs.io/ipfs/land_asset_hash_9
|
New Entities: 0
```


[Back Up](#index)
#
### Part_3 
# Register bidder and place a bid on auction instance number 1   
------------------------------------------------------------------------------------------- 

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1501897 XRD burned, 0.007509485 XRD tipped to validators
Cost Units: 100000000 limit, 1501897 consumed, 0.0000001 XRD per cost unit
Logs: 0

New Entities: 1
└─ Resource: resource_sim1qqd3cz2l03az8zuwmrx8xeulzvk7aevelhesrw58s3nqv4pm89	Bidder Badge auction instance number 1
```

> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0994947 XRD burned, 0.004974735 XRD tipped to validators
Cost Units: 100000000 limit, 994947 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0

```

[Back Up](#index)
#
### Part_4 
# Register bidder and place a bid on auction instance number 2
-------------------------------------------------------------------------------------------

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1528673 XRD burned, 0.007643365 XRD tipped to validators
Cost Units: 100000000 limit, 1528673 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qppx878csep45s4mlx20hdp7hxcdtwj7e33glejqttrsnnev50	Bidder Badge auction instance number 2
```



> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0994978 XRD burned, 0.00497489 XRD tipped to validators
Cost Units: 100000000 limit, 994978 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```

[Back Up](#index)
#
### Part_5 
# Register bidder and place a bid on auction instance number 3
-------------------------------------------------------------------------------------------

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1528673 XRD burned, 0.007643365 XRD tipped to validators
Cost Units: 100000000 limit, 1528673 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qp75hjxh8f4wvr2xf8q5sna653xp4p0znctk7sp6d8tqelelgy	Bidder Badge auction instance number 3
```



> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0994978 XRD burned, 0.00497489 XRD tipped to validators
Cost Units: 100000000 limit, 994978 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```



[Back Up](#index)
#
### Part_6
# Register bidder and place a bid on auction instance number 4 
-------------------------------------------------------------------------------------------

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1549841 XRD burned, 0.007749205 XRD tipped to validators
Cost Units: 100000000 limit, 1549841 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qr95w6wsmervytqua7dtwy3f3dtdpyr69xk0ywr05yjq2phpqk	Bidder Badge auction instance number 4

```


> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.099504 XRD burned, 0.0049752 XRD tipped to validators
Cost Units: 100000000 limit, 995040 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```

[Back Up](#index)
#
### Part_7 
# Register bidder and place a bid on auction instance number 5
-------------------------------------------------------------------------------------------

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1574494 XRD burned, 0.00787247 XRD tipped to validators
Cost Units: 100000000 limit, 1574494 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qr3jpksgehypfphv38kl5049eja3yjvhrwnyhzp6sgcqjhl3j5	Bidder Badge auction instance number 5

```


> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0995071 XRD burned, 0.004975355 XRD tipped to validators
Cost Units: 100000000 limit, 995071 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```

[Back Up](#index)
#
### Part_8 
# Register bidder and place a bid on auction instance number 6
-------------------------------------------------------------------------------------------

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1574494 XRD burned, 0.00787247 XRD tipped to validators
Cost Units: 100000000 limit, 1574494 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qzemvhjrktjqy28v4hw2snprkn3qy22ucfhjeerx22vs93js5l	Bidder Badge auction instance number 6

```


> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0995071 XRD burned, 0.004975355 XRD tipped to validators
Cost Units: 100000000 limit, 995071 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```



[Back Up](#index)
#
### Part_9 
# Register bidder and place a bid on auction instance number 7  
-------------------------------------------------------------------------------------------

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1588646 XRD burned, 0.00794323 XRD tipped to validators
Cost Units: 100000000 limit, 1588646 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qrlyqd69g90vsqhxg6gqtknyee9wc26w0tnh42kkutwq8j6u5n	Bidder Badge auction instance number 7

```


> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0995133 XRD burned, 0.004975665 XRD tipped to validators
Cost Units: 100000000 limit, 995133 consumed, 0.0000001 XRD per cost unit
|
New Entities: 0
```

[Back Up](#index)
#
### Part_10 
# Register bidder and place a bid on auction instance number 8  
-------------------------------------------------------------------------------------------

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.160351 XRD burned, 0.00801755 XRD tipped to validators
Cost Units: 100000000 limit, 1603510 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qpyeyclqhk8zh8nhmnn0mhpvrv2qhqm3yelprxuqk4rqpmah4x	Bidder Badge auction instance number 8
```


> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0995164 XRD burned, 0.00497582 XRD tipped to validators
Cost Units: 100000000 limit, 995164 consumed, 0.0000001 XRD per cost unit
|
New Entities: 0
```

[Back Up](#index)
#
### Part_11 
# Register bidder and place a bid on auction instance number 9  
-------------------------------------------------------------------------------------------

> update data on ```register```

>```register.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.160351 XRD burned, 0.00801755 XRD tipped to validators
Cost Units: 100000000 limit, 1603510 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qz0q42y2vz7dqm3qh0yfjxttep077w7lry4nkurvsgxq7pcj7a	Bidder Badge auction instance number 9
```


> update data on ```place_bid```

>```place_bid.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0995164 XRD burned, 0.00497582 XRD tipped to validators
Cost Units: 100000000 limit, 995164 consumed, 0.0000001 XRD per cost unit
|
New Entities: 0
```


[Back Up](#index)
#
### Part_12 
# Overtake auction instances deadline and claim land asset of won auction instances
# providing bidder badges and related auction payments
-------------------------------------------------------------------------------------------

account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t				bidder account

└─ Component: component_sim1qtvk9300ckmwysd5z9tk04sk0ksquu5ap5qupqre3kmsadca6v		Neverland Land Auction Component address


> resim set-current-epoch 12008


> update data on ```claim_land_asset```

>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5447056 XRD burned, 0.02723528 XRD tipped to validators	
Cost Units: 100000000 limit, 5447056 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9999
├─ [INFO ]  NFT user address 9999 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 0
├─ [INFO ] asset_seed: 0
├─ [INFO ]  NFT ID: 3007100000006cf7f02f8557f2bfe4561861d3c30678
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0101
└─ [INFO ]  TKN locked amount 0.33
|
New Entities: 0
```


> update data on ```claim_land_asset```

>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5557948 XRD burned, 0.02778974 XRD tipped to validators
Cost Units: 100000000 limit, 5557948 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9998
├─ [INFO ]  NFT user address 9998 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 1
├─ [INFO ] asset_seed: 1
├─ [INFO ]  NFT ID: 300710000000a6d48acbf5097b821ea00a2d0cc84f71
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0102
└─ [INFO ]  TKN locked amount 0.33
|
New Entities: 0
```

> update data on ```claim_land_asset```

>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.6478573 XRD burned, 0.032392865 XRD tipped to validators
Cost Units: 100000000 limit, 6478573 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9997
├─ [INFO ]  NFT user address 9997 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 2
├─ [INFO ] asset_seed: 2
├─ [INFO ]  NFT ID: 300710000000e472963393192a86550334cbc2fe7a8e
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0103
└─ [INFO ]  TKN locked amount 0.33
|
New Entities: 0
```


> update data on ```claim_land_asset```

>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5661855 XRD burned, 0.028309275 XRD tipped to validators
Cost Units: 100000000 limit, 5661855 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9996
├─ [INFO ]  NFT user address 9996 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 3
├─ [INFO ] asset_seed: 3
├─ [INFO ]  NFT ID: 3007100000008018133901c22bcb8174846623c71db4
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0201
└─ [INFO ]  TKN locked amount 0.33
|
New Entities: 0
```


> update data on ```claim_land_asset```

>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5782401 XRD burned, 0.028912005 XRD tipped to validators
Cost Units: 100000000 limit, 5782401 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9995
├─ [INFO ]  NFT user address 9995 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 4
├─ [INFO ] asset_seed: 4
├─ [INFO ]  NFT ID: 300710000000e46922f6d9fc8d51aa5396a766ea0f27
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0202
└─ [INFO ]  TKN locked amount 0.33
|
New Entities: 0
```

> update data on ```claim_land_asset```

>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.6631972 XRD burned, 0.03315986 XRD tipped to validators
Cost Units: 100000000 limit, 6631972 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9994
├─ [INFO ]  NFT user address 9994 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 5
├─ [INFO ] asset_seed: 5
├─ [INFO ]  NFT ID: 300710000000e775db9b13a353bdeb5021e2d9a4e92f
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0203
└─ [INFO ]  TKN locked amount 0.33
|
New Entities: 0
```


> update data on ```claim_land_asset```

>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5901299 XRD burned, 0.029506495 XRD tipped to validators
Cost Units: 100000000 limit, 5901299 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9993
├─ [INFO ]  NFT user address 9993 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 6
├─ [INFO ] asset_seed: 6
├─ [INFO ]  NFT ID: 3007100000008678ffba9d10934c51efb93fa3292062
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0301
└─ [INFO ]  TKN locked amount 0.33
|
New Entities: 0
```


> update data on ```claim_land_asset```

>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.6000344 XRD burned, 0.03000172 XRD tipped to validators
Cost Units: 100000000 limit, 6000344 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9992
├─ [INFO ]  NFT user address 9992 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 7
├─ [INFO ] asset_seed: 7
├─ [INFO ]  NFT ID: 300710000000765a57e4f1bdf6649ee1298a21fc9e5c
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0302
|
New Entities: 0
```


>```claim_land_asset.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.6722251 XRD burned, 0.033611255 XRD tipped to validators
Cost Units: 100000000 limit, 6722251 consumed, 0.0000001 XRD per cost unit
Logs: 9
├─ [INFO ] mint_code_id: 9991
├─ [INFO ]  NFT user address 9991 map created
├─ [INFO ] [get_code]:All NFT production codes has been extracted !
├─ [INFO ] [get_code]:data 8
├─ [INFO ] asset_seed: 8
├─ [INFO ]  NFT ID: 30071000000050d6ddd23f0dca371fcd1d8e488bf861
├─ [INFO ]  NFT resource address: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy
├─ [INFO ]  Mint:  asset type Neverland Property Certificate,  Region North,  District NorthWest,  Parcel 0303
└─ [INFO ]  TKN locked amount 0.33
|
New Entities: 0
```


[Back Up](#index)
#
### Part_13
#  Claim payment on succesful auction instances  
-------------------------------------------------------------------------------------------

> Switch default account to protocol owner

```resim set-default-account account_sim1qwk73ye3gfmnxnw42jgpv3gey9jj8a50se753pvnccfquqkgk3 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28```


> update data on ```claim_payment```


>```claim_payment.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5058207 XRD burned, 0.025291035 XRD tipped to validators
Cost Units: 100000000 limit, 5058207 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0

```


[Back Up](#index)





