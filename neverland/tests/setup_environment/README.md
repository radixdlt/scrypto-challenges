--------------------------------------------------------------------------------------------------
# Set up test enviroment to perform Test.  
--------------------------------------------------------------------------------------------------

Neverland environment's components setup is quite mandatory to perform other tests, as all components are instantiated and all various caller badges,
SBT updater badges and data sets are stored within related components during this setup session.

However if someone wish to test various methods through different Neverland enviroment Components without perform a complete tests session,
DemoTools Component allows tester to mint Asset NFs as well as update SBT data, once stored a SBT Updater Badge within his vault.


N.B."Instructions" data tabs within Transactions output as well as other empty fields have been intentionally obmitted.

-------------------------------------------------------------------------------------------
# Index  
-------------------------------------------------------------------------------------------	
> [Part_1](#part_1) . Create eight new accounts
>
> [Part_2](#part_2) . Create some token resources
>
> [Part_3](#part_3) . Setup a Dex Component and stock tokens
>
> [Part_4](#part_4) . Setup two LandData Components to mint two distinct SBT NFT resources referring to two distinct lands: Neverland and Mahoroba
>
> [Part_5](#part_5) . Setup one Neverland Academy Component
>
> [Part_6](#part_6) . Setup a AssetFarm Component to mint Neverland Asset NFT resource
>
> [Part_7](#part_7) . Setup Pitia oracle Component, mint 1 Caller Badges related to Neverland AssetFarm, insert NFT IDs and related production codes
>
> [Part_8](#part_8) . Setup NeverlandMintNftData Component, mint 1 badge, insert svg data
>
> [Part_9](#part_9) . Setup NeverlandMergeNftData Component, mint 1 badge, insert svg data
>
> [Part_10](#part_10) . Init components addresses and transfer badges to AssetFarm component
>
> [Part_11](#part_11). Setup a Neverland HouseHub Component & insert Asset NFT merge data
>
> [Part_12](#part_12) . Setup a MarketplaceVault component to deposit AssetSquare component accrued gains
>
> [Part_13](#part_13) . Setup a NeverlandAuction Component to auctioning Neverland land parcels
>
> [Part_14](#part_14) . Setup two AssetSquare components, a Neverland one and a Mahoroba one
>
> [Part_15](#part_15) . Transfer SBT Updater Badge from LandData Component to others Neverland environment Components
>
> [Part_16](#part_16) . Setup a DemoTools Component 
>


#
### Part_1 
# Create a new account
-------------------------------------------------------------------------------------------

>
> protocols owner account
>
>resim new-account
```A new account has been created!
Account component address: account_sim1qwk73ye3gfmnxnw42jgpv3gey9jj8a50se753pvnccfquqkgk3	
Public key: 0383ffb219b35c04f26db0a1e8efb9efec16fdd931aef837512bd60aa172342fa4
Private key: 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28
No configuration found on system. will use the above account as default.
```

>
> neverland's land owner account 
>
>resim new-account
```A new account has been created!
Account component address: account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t	
Public key: 03dae07d865f8902053911403291fa606d78f0081a40d65aa3ca0b7fd978ac5162
Private key: c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7
```

>
> neverland's teacher account 
>
>resim new-account
```A new account has been created!
Account component address: account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h	
Public key: 037a622c7d7f3730c1b1ec5b340f8ea78d040aba3af74e5df7bcf45284a1f01126
Private key: 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2
```

>
> neverland's architecture student account 
>
>resim new-account
```A new account has been created!
Account component address: account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg	
Public key: 03e4d1962763c01d07d9432a22bd41c7c6685487d7aa0b0efba9ba8163435fb4d9
Private key: 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c
```

>
> neverland's general construction student account 
>
>resim new-account
```A new account has been created!
Account component address: account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra	
Public key: 035d9746555d52eab4484c63b6b4df25b2186d44969bbacfb923f5de99e00df733
Private key: 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2
```

>
> neverland's land buyer account 
>
>resim new-account
```A new account has been created!
Account component address: account_sim1q0whr39q2md5sdd7mlv5t6h9efrkvatfr62rgcyc24cs76sksn	
Public key: 0280c639797f8331769adac01c022d8063719640dd8bc94b5c32dc1a0c0b2de63e
Private key: ac1ddbf111dc1975c25b64d88de6204e84114762c4afbbe489cc2ccce6a580c4
```

>
> mahoroba's land buyer account
>
>resim new-account
```A new account has been created!
Account component address: account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman
Public key: 31a65b8e4070e543dba7daffa3703ea8dcbb2b472acc714cec390bfc6f843cfef
Private key: ed5bc42f40e114e0e1de7da0858b4929974ab6ea7f8cde61420355e09d336eb2
```
resim show account_sim1q0whr39q2md5sdd7mlv5t6h9efrkvatfr62rgcyc24cs76sksn

>
> reserve account
>
>resim new-account
```A new account has been created!
Account component address: account_sim1q0pq0guvwum2dwgaav2zvfkeztz2tyef24275lxwq9fqlu9jqr
Public key: 02c33452fdb7cddb6c8e625d9ba2acbe6492a1b92a25487f740b4407cf96275b73
Private key: 06bdcab9736f4c0da3637b4cca35eb33a44fb1846a47d2af2e3382fcc8944cea
```

[Back Up](#index)
#
### Part_2 
# Create some token resources and transfer some amounts to created accounts 
-------------------------------------------------------------------------------------------

>resim new-token-fixed --name "ONE" 100000 --symbol "ONE" 
```
|
└─ Resource: resource_sim1qzlwc3akklnq0z6xmssar3998xnw9ezsfzyz38tafv4sa9ft2g		MahorobaLand environment currency
```

>resim new-token-fixed --name "TKN" 100000 --symbol "TKN" 
```
|
└─ Resource: resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe		Neverland environment currency
```

>resim new-token-fixed --name "TWO" 100000 --symbol "TWO" 
```
|
└─ Resource: resource_sim1qrpkrske663mm6s2cykm3qhagl2zre3zw4axxezkcyqq38nk82
```

>transfer some "TKN" token resource to Neverland accounts

>resim transfer 2000 resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t

>resim transfer 2000 resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h

>resim transfer 2000 resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg

>resim transfer 2000 resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra

>resim transfer 2000 resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe account_sim1q0whr39q2md5sdd7mlv5t6h9efrkvatfr62rgcyc24cs76sksn


>transfer some "ONE" token resource to mahoroba's land buyer account

>resim transfer 1000 resource_sim1qzlwc3akklnq0z6xmssar3998xnw9ezsfzyz38tafv4sa9ft2g account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman


[Back Up](#index)
#
### Part_3 
# Setup a Dex Component and stock tokens
-------------------------------------------------------------------------------------------

>
>resim publish .
```
Success! New Package: package_sim1q92r6ntt457cprta4m4rp3f86mg25l8ve2h3yqqjv0qq9gu4wf
```

>cd transaction_manifest

>cd instantiate_components_transaction_manifest

>update data on ```dex_instantiate```

>```dex_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.056364 XRD burned, 0.0028182 XRD tipped to validators
Cost Units: 100000000 limit, 563640 consumed, 0.0000001 XRD per cost unit
|
New Entities: 3
└─ Component: component_sim1qtwfalw8frs3pgywpsheurr5phqwa28wq4tfmc9k3kassnfzgl		DEX component address
├─ Resource: resource_sim1qrrapxxhxn3ckq0q3f6rd2cuq3mh539nzav4k4n3hc9qpfnwdv
└─ Resource: resource_sim1qq2uvwqpa6u945juml77d4vzamu4fn7t4qfsmacwu4esltzzp0
```

> cd dex_transaction_manifest

>update data on ```stock_token```

>```stock_token.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.9198587 XRD burned, 0.045992935 XRD tipped to validators
Cost Units: 100000000 limit, 9198587 consumed, 0.0000001 XRD per cost unit
Logs: 3
├─ [INFO ]  Added 20000 ONE token, ONE symbol @2XRD price
├─ [INFO ]  Added 40000 TKN token, TKN symbol @1XRD price
└─ [INFO ]  Added 30000 TWO token, TWO symbol @1.5XRD price
|
New Entities: 6
├─ Resource: resource_sim1qq26srueuhplma922nk64gk533z6uapkp22auqaq525srfssut
├─ Resource: resource_sim1qpwkk8sk3jj06nj22pypcu7t3l47mveac9mt2hsxvkdq6r2xru
├─ Resource: resource_sim1qpscfv9nglzsekp7czr7yvepnadnuylq7avnpszjdz6qew4lag
├─ Resource: resource_sim1qrxpahgx3flsggfk29veme5adg4vx85cspsyf3ykwk7qx8gjnj
├─ Resource: resource_sim1qqr8dw53r9hl02qw80cw0wacqw4z894yhwt3s0grnrgsec9gcw
└─ Resource: resource_sim1qz6xrzjluez0gw4fr24dtdmcy8mu75uj9dqa2p595x0sd63pge
```

[Back Up](#index)
#
### Part_4 
### Setup two LandData Components to mint two distinct SBT NFT resources referring to
### two distinct lands: Neverland and Mahoroba.
###
### Register five account as Neverland users by storing a SBT on their account interacting 
### with Neverland LandData Component method ```register_user```
###
### Register five account as Neverland users by storing a SBT on their account interacting 
### with Mahoroba LandData Component method ```register_user```
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```land_data_instantiate```

>```land_data_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1317706 XRD burned, 0.00658853 XRD tipped to validators
Cost Units: 100000000 limit, 1317706 consumed, 0.0000001 XRD per cost unit
|
New Entities: 10
├─ Component: component_sim1qg42wzycfq0czsaymfeu39n3efznsfjjv4kku9ef4n5q36wunf		Neverland LandData Component
├─ Resource: resource_sim1qrcqfe8clkmy9ns9y0whvqqjnw9j805qa5rxxtytm4yqhup84z		LandData Minter Badge
├─ Resource: resource_sim1qqanttcwf9yy7f72mm0lfn4u6e9efvqs5tn87gsu44wqh4vhhz		LandData Updater Badge
├─ Resource: resource_sim1qruscaxadxczl3npymdqvfh5nyxn5rktsr8hdujryfls6vnltk		LandData Owner Badge
├─ Resource: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f		Neverland LandData User SBT resource address
|
├─ Component: component_sim1qtvl6k294arp90pmzd4c9mjcys8my4z9rc73w2kg4djqnhygsy		Mahoroba LandData Component
├─ Resource: resource_sim1qzgt6feqfavj3wwuls437ax8g0jz470klxurt6f2u6asd5wnnu		LandData Minter Badge
├─ Resource: resource_sim1qpn7szf9qa8lk369lqpq9qtzy8xdupfufgztskxcfm4q6yn4vk		LandData Updater Badge
├─ Resource: resource_sim1qpqp4uk7qckhphg8xs4xzvrq288ekys58x5yxj9dh6tqmpmr35		LandData Owner Badge
└─ Resource: resource_sim1qp4ssprn6cp053pwt5h6y2a7jxyjcz5jhcnqk2s460tqceylq5		Mahoroba LandData User SBT resource address
```


>cd land_data_transaction_manifest

> Register five account within Neverland LandData Component

> Switch default account

resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7

>update data on ```register_users```

>```register_users.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0499221 XRD burned, 0.002496105 XRD tipped to validators
Cost Units: 100000000 limit, 499221 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  User SBT address added: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
└─ [INFO ]  User SBT id: 3007100000000d65b77d2e99af195c2c4ecb8a49a050
|
New Entities: 0
```

> Switch default account

resim set-default-account account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2

>update data on ```register_users```

>```register_users.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.053464 XRD burned, 0.0026732 XRD tipped to validators
Cost Units: 100000000 limit, 534640 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  User SBT address added: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
└─ [INFO ]  User SBT id: 300710000000b7b0771cabf0e0cee8b06b94a47490e1
|
New Entities: 0
```

> Switch default account

resim set-default-account account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c

>update data on ```register_users```

>```register_users.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0571121 XRD burned, 0.002855605 XRD tipped to validators
Cost Units: 100000000 limit, 571121 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  User SBT address added: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
└─ [INFO ]  User SBT id: 300710000000e492d1a29f9c41e86f8ba19215d0c46c
|
New Entities: 0
```

> Switch default account 	

resim set-default-account account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2

>update data on ```register_users```

>```register_users.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0619531 XRD burned, 0.003097655 XRD tipped to validators
Cost Units: 100000000 limit, 619531 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  User SBT address added: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
└─ [INFO ]  User SBT id: 300710000000985f0bf0a4fbceed30b544a167858b01
|
New Entities: 0
```


> Switch default account		

resim set-default-account account_sim1q0whr39q2md5sdd7mlv5t6h9efrkvatfr62rgcyc24cs76sksn ac1ddbf111dc1975c25b64d88de6204e84114762c4afbbe489cc2ccce6a580c4

>update data on ```register_users```

>```register_users.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0661708 XRD burned, 0.00330854 XRD tipped to validators
Cost Units: 100000000 limit, 661708 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  User SBT address added: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
└─ [INFO ]  User SBT id: 30071000000064b087e4aef4a0e5e1886a379ebdce90
|
New Entities: 0
```

> Register one account within Mahoroba LandData Component

> Switch default account

resim set-default-account account_sim1qw9kuggm30xx27d8hwxyf8qkym6nqhc8z3mhktam9z6qtpcman ed5bc42f40e114e0e1de7da0858b4929974ab6ea7f8cde61420355e09d336eb2

>update data on ```register_users```

>```register_users.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0499237 XRD burned, 0.002496185 XRD tipped to validators
Cost Units: 100000000 limit, 499237 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  User SBT address added: resource_sim1qp4ssprn6cp053pwt5h6y2a7jxyjcz5jhcnqk2s460tqceylq5
└─ [INFO ]  User SBT id: 30071000000056209feaa224368e6e9350ae01e2594f
|
New Entities: 0
```

> Switch default account

resim set-default-account account_sim1qdgzwrxzcmyw4sxwakljem07vtzlurr0zmllhcf7twgsjnru6t c3687e176b450b88f2381bf9c6f5eea46d4b9c252c59a00379452475c81f89d7


[Back Up](#index)
#
### Part_5 
# Setup one Neverland Academy Component 
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```academy_instantiate```

>```academy_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1017515 XRD burned, 0.005087575 XRD tipped to validators
Cost Units: 100000000 limit, 1017515 consumed, 0.0000001 XRD per cost unit
|
New Entities: 7
└─ Component: component_sim1qtfvyec6hzfuvfrzr08yyd3ts4j5tavzeuzl7u0sj9qsdpls0y		Neverland Academy Component
├─ Resource: resource_sim1qrpkt9wlm08n4z00ffe65ku4a62j485g0nlxnna99jesdd46c2		Academy MinterBadge
├─ Resource: resource_sim1qr7s9ayr765ce2llvd55ucpre7ggudxz6ayk5cludfus28zuyl		Academy OwnerBadge
├─ Resource: resource_sim1qqcstxhymrd6h4crsm5g878uqszxh3552gn8lw8u6mxqng98jf		Academy TeacherBadge MinterBadge
├─ Resource: resource_sim1qrzvr23pw2w4n6s7fj8hxwmvw6mpss3pr9tr68pla5vqryc65l		Academy Academy Test Template
├─ Resource: resource_sim1qqh9sk68463hn6l2x46f68paefxk28ss2lqeag4mxnmqdrgefu		Academy Academy Test Certificate
└─ Resource: resource_sim1qpyc88dfmeuvly4vu5ssdqnwzrpyzqrcp0juettpvttqp25370		Academy Academy Degree NFT
```

[Back Up](#index)
#
### Part_6 
# Setup a AssetFarm Component to mint Neverland Asset NFT resource
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```farm_instantiate```		

>```farm_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2395677 XRD burned, 0.011978385 XRD tipped to validators
Cost Units: 100000000 limit, 2395677 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 5
└─ Component: component_sim1qgzcvggvrj582z8fm8zn7ffgwn7m7jtzggvtgnw0whrqwttr95		AssetFarm Component
├─ Resource: resource_sim1qz5k4wsplg3ea80jm3kz8eqtja90vck77cpz0yrygduskun06u		AssetFarm OwnerBadge
├─ Resource: resource_sim1qr88m0e67s6plupgjpm3wfpey2pw9ssjy5lqy7c6gf2qdyqp55		Asset NFT MinterBadge
├─ Resource: resource_sim1qqn9x0pcses7ylwfydl62j87legsje9cn8305cm95jpqtxkrgy		Neverland mint land resource address
└─ Resource: resource_sim1qpujwawehl3yg329gs07snpp9544c22u4nrnsetqu54shhagad		Neverland merge land resource address
```

resim show resource_sim1qpujwawehl3yg329gs07snpp9544c22u4nrnsetqu54shhagad

[Back Up](#index)
#
### Part_7 
# Setup Pitia oracle Component, mint 1 Caller Badges related to Neverland AssetFarm, insert NFT IDs and related production codes
----------------------------------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```pitia_instantiate```		

>```pitia_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0768476 XRD burned, 0.00384238 XRD tipped to validators
Cost Units: 100000000 limit, 768476 consumed, 0.0000001 XRD per cost unit
|
New Entities: 3
└─ Component: component_sim1qtgp4q6rc7ztqmtrklwf0f44gtgw7nrgaaez7wqnyeaqc9xprd		Pitia component address
├─ Resource: resource_sim1qz9vz2q8ej0j42p8xlq76mxwglrs6wa2p8pzfua5fhzs8rtgl5		MinterBade
└─ Resource: resource_sim1qzutmgwwu9k4urh74x3jl9d277q3eaw5lpxtex626tjq2ke739		OwnerBadge
```

#
One Test purpose is, given a certain NFT ID and URL as input, mint related data(traits + svg data) within output NFT. 
So we're gong to call "get_code" method on Pitia oracle component, that return a numeric production code needed by NeverlandMintData
component and NeverlandMergeData component to retrieve final data to mint within NFT(traits + svg data).
To call "get_code" method we need to mint a specific badge.
#

>
>cd pitia_trans_manifest
>cd insert_data

>update data on ```insert_nft_buyer_data```		

>```insert_nft_buyer_data.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.5979744 XRD burned, 0.02989872 XRD tipped to validators
Cost Units: 100000000 limit, 5979744 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0

```

>cd nft_badge_mint  
>cd nft_mint_badge_getcode

>update data on ```nft_badge_mint_getcode```		

>```nft_badge_mint_getcode.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0814905 XRD burned, 0.004074525 XRD tipped to validators
Cost Units: 100000000 limit, 814905 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Component address stored: component_sim1qgzcvggvrj582z8fm8zn7ffgwn7m7jtzggvtgnw0whrqwttr95  AssetFarm component address
|
New Entities: 1
└─ Resource: resource_sim1qrmlc7m7psmhkfy33v560xhq4u0cpd6extvclln8krusxgaygv				Pitia Caller Badge  
```
 

[Back Up](#index)
#
### Part_8 
# Setup NeverlandMintNftData Component, mint 1 badge, insert svg data 
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```mint_nft_data_instantiate```		

>mint_nft_data_instantiate.sh
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0530251 XRD burned, 0.002651255 XRD tipped to validators
Cost Units: 100000000 limit, 530251 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 3
└─ Component: component_sim1qt8ke2fuk4ju4ze5jw5mak08q7wwax54hsvz32y09nasqkxv0s				NeverlandMintNftData component address
├─ Resource: resource_sim1qre09zcew8qrprywpfrxvy6x0kf0nakjw3lffq8pux9s8nuh4l
└─ Resource: resource_sim1qptscts97muzxea32dz773upglplty64r3l23axdrrqsqdv56s				NeverlandMintNftData OwnerBadge
```

resim show resource_sim1qptscts97muzxea32dz773upglplty64r3l23axdrrqsqdv56s

>cd mint_trans_manifest

>update data on ```nft_badge_mint```		

>```nft_badge_mint.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0605155 XRD burned, 0.003025775 XRD tipped to validators
Cost Units: 100000000 limit, 605155 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Component address: component_sim1qgzcvggvrj582z8fm8zn7ffgwn7m7jtzggvtgnw0whrqwttr95		AssetFarm component address
|
New Entities: 1
└─ Resource: resource_sim1qqmkhs4dpj6zns3uzwlpnn6jletfpvecxk5l4h900vyszu484k				NeverlandMintNftData Caller Badge 
```

>update data on ```insert_svg_data```		

>```insert_svg_data.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0996232 XRD burned, 0.00498116 XRD tipped to validators
Cost Units: 100000000 limit, 996232 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```

[Back Up](#index)
#
### Part_9 
# Setup NeverlandMergeNftData Component, mint 1 badge, insert svg data 
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```merge_nft_data_instantiate```		

>merge_nft_data_instantiate.sh
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0528411 XRD burned, 0.002642055 XRD tipped to validators
Cost Units: 100000000 limit, 528411 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 3
└─ Component: component_sim1q235xl3ths9vpxvsup4m6aq9g0w23gcwppgg4ghrt4fsdasmu9				NeverlandMergeNftData component address
├─ Resource: resource_sim1qpaxznlt8ysdppsxkje967h8jqcq3kexgaunt3stphnsa7k2tx
└─ Resource: resource_sim1qrar9zq7jm0gutgnxpkp7t6hqktupkl6g38tsk75djmq9r9dhe				NeverlandMergeNftData OwnerBadge 
```

>cd merge_trans_manifest

>update data on ```nft_badge_mint```		

>```nft_badge_mint.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0605283 XRD burned, 0.003026415 XRD tipped to validators
Cost Units: 100000000 limit, 605283 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Component address: component_sim1qgzcvggvrj582z8fm8zn7ffgwn7m7jtzggvtgnw0whrqwttr95		AssetFarm component address 
|
New Entities: 1
└─ Resource: resource_sim1qrdkflg389utghvfqv09apu8fmtxxk9qpzcegxup86jsy26f74				NeverlandMergeNftData Caller Badge
```

>update data on ```insert_svg_data```		

>```insert_svg_data.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1191044 XRD burned, 0.00595522 XRD tipped to validators
Cost Units: 100000000 limit, 1191044 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```


[Back Up](#index)
#
### Part_10 
# Init components addresses, set academy values on AssetFarm component, transfer badges to AssetFarm component
-------------------------------------------------------------------------------------------

>cd farm_trans_manifest
>cd admin_tools
>cd init_comp_addr

>
>```init_comp_addr.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0626789 XRD burned, 0.003133945 XRD tipped to validators
Cost Units: 100000000 limit, 626789 consumed, 0.0000001 XRD per cost unit
Logs: 4
├─ [INFO ]  Series component address: component_sim1qgzcvggvrj582z8fm8zn7ffgwn7m7jtzggvtgnw0whrqwttr95
├─ [INFO ]  Asset dex address: component_sim1qtwfalw8frs3pgywpsheurr5phqwa28wq4tfmc9k3kassnfzgl
└─ [INFO ]  TKN currency: resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe
|
New Entities: 0
```

>cd farm_trans_manifest
>cd admin_tools
>cd set_academy_values

>
>```set_academy_values.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1219991 XRD burned, 0.006099955 XRD tipped to validators
Cost Units: 100000000 limit, 1219991 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Academy Vault Component Address set to component_sim1qtfvyec6hzfuvfrzr08yyd3ts4j5tavzeuzl7u0sj9qsdpls0y
└─ [INFO ]  Academy share set to 33%
|
New Entities: 0
```


>cd transfer_badge

>```transfer_badge.sh```				
```
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1307932 XRD burned, 0.00653966 XRD tipped to validators
Cost Units: 100000000 limit, 1307932 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```


[Back Up](#index)
#
### Part_11 
### Setup a Neverland HouseHub Component & insert Asset NFT merge data 
### Mint a badge on AssetFarm to perform Land NFT upgrade when a new building project is realized within HouseHub Component
### Mint a badge on AssetFarm to perform Land NFTs merge when related method is called by HouseHub Component
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```house_hub_instantiate```		
>(follow info on ```transaction_manifest_info\house_hub_instantiate_info```)

>```house_hub_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1251031 XRD burned, 0.006255155 XRD tipped to validators
Cost Units: 100000000 limit, 1251031 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 7
└─ Component: component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26		Neverland HouseHub Component address
├─ Resource: resource_sim1qz48rjq6sqtvdvdtf6fkmqusw6q9ug7lm8qfpnt6p9hq78aakn		Neverland HouseHub MinterBadge
├─ Resource: resource_sim1qp7vkpmuxkdezyr6hklmhm0l84rslug9rm7wegmdm5zsypnh8c		Neverland HouseHub OwnerBadge
├─ Resource: resource_sim1qr9vxx976j9w225swt0wwhz35j8kgpws4vjywga3725q6t9crx		Neverland House Hub ArchBadge
├─ Resource: resource_sim1qzvnatv5jwre930mgkjy5g78f3dyf9aj6udx8kaw09wq3d9r3x		Neverland House Hub House Project
├─ Resource: resource_sim1qq45gmeqeemh3dmj76av46ma9fvrs3mrkm3dgz5hngrs4l74u6		Neverland House Hub Building Contract
└─ Resource: resource_sim1qqh9txs7vzlrcqcp9suesskwl4u5tewce2uhpan0uw7qwmwtva		Neverland House Hub Building Property
```

>cd house_hub_transaction_manifest

>update data on ```insert_merge_data```		

>```insert_merge_data.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0570704 XRD burned, 0.00285352 XRD tipped to validators
Cost Units: 100000000 limit, 570704 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```

> Mint a badge on AssetFarm Component to perform Land NFT upgrade when a new building project is realized within HouseHub Component

>update data on ```upgrade_nft_badge```		

>```upgrade_nft_badge.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1111735 XRD burned, 0.005558675 XRD tipped to validators
Cost Units: 100000000 limit, 1111735 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Component address: component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26
|
New Entities: 1
└─ Resource: resource_sim1qq4wjmyh7jpu30hcxkpa0v2dfzez9t49palrr364pq9q2jkwm5
```


> Mint a badge on AssetFarm Component to perform Land NFTs merge when related method is called by HouseHub Component 

> update data on ```merge_nft_badge```		

>```merge_nft_badge.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1126048 XRD burned, 0.00563024 XRD tipped to validators
Cost Units: 100000000 limit, 1126048 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Component address: component_sim1qtzauzgk9exy44faj7ep3yeufqwyd97cvlxz6rt38smsrlke26
|
New Entities: 1
└─ Resource: resource_sim1qr8krprm4f2aqhw2c4tpp7x8l3tl5vznd2j6ayfwvx5szvqtsc
```


> Stock AssetFarm badges into HouseHub component vaults	

> house_hub_transaction_manifest

> update data on ```stock_nft_farm_badge```		

>```stock_nft_farm_badge.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.085435 XRD burned, 0.00427175 XRD tipped to validators
Cost Units: 100000000 limit, 854350 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```


[Back Up](#index)
#
### Part_12 
## Setup a MarketplaceVault component to deposit AssetSquare component accrued gains
## (without incurring in unallowed reentrancy issues) 
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```marketplace_vault_instantiate```	
>(follow info on ```transaction_manifest_info\marketplace_vault_instantiate_info```)	

>```marketplace_vault_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0570704 XRD burned, 0.00285352 XRD tipped to validators
Cost Units: 100000000 limit, 570704 consumed, 0.0000001 XRD per cost unit
|
New Entities: 2
└─ Component: component_sim1qf006wy2qclusvr3z0p469rdrv2qlp7f78t379eg9q4qr8gffg		MarketplaceVault component address
└─ Resource: resource_sim1qprsk8r3c0yv80c5cuma23a7xz67vf92jwcxa76vp7gq7hu55j		MarketplaceVault OwnerBadge
```

[Back Up](#index)
#
### Part_13 
# Setup a NeverlandAuction Component to auctioning Neverland land parcels 
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```land_auction_instantiate```	

>```land_auction_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0652706 XRD burned, 0.00326353 XRD tipped to validators
Cost Units: 100000000 limit, 652706 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 3
└─ Component: component_sim1qtvk9300ckmwysd5z9tk04sk0ksquu5ap5qupqre3kmsadca6v		Neverland Land Auction Component address
├─ Resource: resource_sim1qq077g24fwpd77wnvfkll7avnwqgac4sxfz2my4du0rqkf26nn		Neverland Auction MinterBadge
└─ Resource: resource_sim1qq2lveq6jk38as3xj28pekfm6d3vkls4mvl34zce280sq3ktty		Neverland Auction OwnerBadge
```


> Mint a Caller badge on AssetFarm Component and transfer badge to NeverlandAuction Component to allow the latter to mint auctioneed Land Asset 
> NFTs on AssetFarm Component through external component call.


>cd farm_trans_manifest
>cd admin_tools
>cd mint_nft_badge

>update data on ```mint_nft_badge```

>```mint_nft_badge.sh```				
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0771294 XRD burned, 0.00385647 XRD tipped to validators
Cost Units: 100000000 limit, 771294 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Component address: component_sim1qtvk9300ckmwysd5z9tk04sk0ksquu5ap5qupqre3kmsadca6v		NeverlandAuction Component address
|
New Entities: 1
└─ Resource: resource_sim1qp6yrp2q5tgj8dhdrtaawy7ylwsk7x4vc75pac8f4erqtahe7y				NeverlandAuction Component Caller Badge

```

> cd land_auction_transaction_manifest

>update data on ```stock_nft_farm_badge```

>```stock_nft_farm_badge.sh```				
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0491056 XRD burned, 0.00245528 XRD tipped to validators
Cost Units: 100000000 limit, 491056 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```


[Back Up](#index)
#
### Part_14
## Setup two AssetSquare components, a Neverland one and a Mahoroba one, link them through 
## Caller Badge storing allowing them to invoke each other's methods.
-------------------------------------------------------------------------------------------

>cd instantiate_components_transaction_manifest

>update data on ```asset_square_instantiate```		

>```asset_square_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3276916 XRD burned, 0.01638458 XRD tipped to validators
Cost Units: 100000000 limit, 3276916 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 10
├─ Component: component_sim1q2d9c9wus40556ujg386n3ada7qmv72lctr2lx6y5m3sg733dr		Neverland AssetSquare component
├─ Component: component_sim1qtp8pvfc3kg3v2n8tdlvzz2c2s0e00tvkl6emca9s2ws248jmj
├─ Resource: resource_sim1qp20duj2hy577wahr493ucvtn6uxyv2v7gvqxh40xg2sjhjlc5		MinterBadge
├─ Resource: resource_sim1qrmrgh07sh2acdjx0umwptj8fcjxnftv45h3n0284a7q5q0q3m		OwnerBadge
├─ Resource: resource_sim1qpw8n3mjp77e5epe965u0k04lgzdyc57f3e6ky82r6wsszdajh		AssetBadge
|
|
├─ Component: component_sim1qfxgwghn69wz9sqdcm7vely8jy99xa9f00h9yjqyn54stf09zp		Mahoroba AssetSquare component
├─ Component: component_sim1qgl3j0lkd2vje8d5pz4uu7ch0yepct33pfewzjt76etq6pwywg
├─ Resource: resource_sim1qz997t75my44ed6fjcn36gy7jqq4hcmjyzeugylwe3nskvz20j		MinterBadge
├─ Resource: resource_sim1qry7evj0wjkmy9j74dj2yk7l33yvcutravj9cvxdk89svzr2m3		OwnerBadge
└─ Resource: resource_sim1qqmtd06xhc992zh7gxhzsere50pgkygm04wks9lqax9snmg5cu		AssetBadge
```


>cd asset_square_transaction_manifest
>cd admin_tools


>update data on ```set_deadlines```		

>```set_deadlines.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0694067 XRD burned, 0.003470335 XRD tipped to validators
Cost Units: 100000000 limit, 694067 consumed, 0.0000001 XRD per cost unit
Logs: 10
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Auction deadline set to 5000
├─ [INFO ]  Auction last bid deadline set to 5
├─ [INFO ]  Buy proposal deadline set to 5000
├─ [INFO ]  =====================================================================================
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Auction deadline set to 5000
├─ [INFO ]  Auction last bid deadline set to 5
├─ [INFO ]  Buy proposal deadline set to 5000
└─ [INFO ]  =====================================================================================
|
New Entities: 0
```

>update data on ```stock_badge_neverland```		


>```stock_badge_neverland.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.077063 XRD burned, 0.00385315 XRD tipped to validators
Cost Units: 100000000 limit, 770630 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0

```

>update data on ```reset_asset_square_values```
>(follow info on ```transaction_manifest_info\reset_asset_square_values_info```)		

>```reset_asset_square_values.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2735168 XRD burned, 0.01367584 XRD tipped to validators
Cost Units: 100000000 limit, 2735168 consumed, 0.0000001 XRD per cost unit
Logs: 7
├─ [INFO ]  =====================================================================================
├─ [INFO ]  AssetSquare fee set to 3%
├─ [INFO ]  Asset royalty fee set to 1%
├─ [INFO ]  TKN currency resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe
├─ [INFO ]  AssetSquare component address set to component_sim1q2d9c9wus40556ujg386n3ada7qmv72lctr2lx6y5m3sg733dr
├─ [INFO ]  TKN Vault component address set to component_sim1qf006wy2qclusvr3z0p469rdrv2qlp7f78t379eg9q4qr8gffg
└─ [INFO ]  =====================================================================================
|
New Entities: 0
```

>update data on ```set_asset_square_values```
>(follow info on ```transaction_manifest_info\set_asset_square_values_info```)		

>```set_asset_square_values.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2514905 XRD burned, 0.012574525 XRD tipped to validators
Cost Units: 100000000 limit, 2514905 consumed, 0.0000001 XRD per cost unit
Logs: 7
├─ [INFO ]  =====================================================================================
├─ [INFO ]  AssetSquare fee set to 5%
├─ [INFO ]  Asset royalty fee set to 1%
├─ [INFO ]  TKN currency resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe
├─ [INFO ]  AssetSquare component address set to component_sim1q2d9c9wus40556ujg386n3ada7qmv72lctr2lx6y5m3sg733dr
├─ [INFO ]  TKN Vault component address set to component_sim1qf006wy2qclusvr3z0p469rdrv2qlp7f78t379eg9q4qr8gffg
└─ [INFO ]  =====================================================================================
|
New Entities: 0
```

>update data on ```set_comp_addr```		

>```set_comp_addr.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0717574 XRD burned, 0.00358787 XRD tipped to validators
Cost Units: 100000000 limit, 717574 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0

```

>update data on ```set_academy_values```		
	

>```set_academy_values.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1168543 XRD burned, 0.005842715 XRD tipped to validators
Cost Units: 100000000 limit, 1168543 consumed, 0.0000001 XRD per cost unit
Logs: 4
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Academy Vault Component Address set to component_sim1qf006wy2qclusvr3z0p469rdrv2qlp7f78t379eg9q4qr8gffg
├─ [INFO ]  Academy share fee set to 30%
└─ [INFO ]  =====================================================================================
|
New Entities: 0

```

>update data on ```add_ext_marketplace```		
>(follow info on ```transaction_manifest_info\add_ext_marketplace_info```)


>```add_ext_marketplace.sh```
```
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1223829 XRD burned, 0.006119145 XRD tipped to validators
Cost Units: 100000000 limit, 1223829 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 1
└─ Resource: resource_sim1qzpfhc2ccq5dqqymeu5eews0965exhf0dyw5u4t2rczqlxwh0f		Mahoroba AssetSquare Caller Badge (to call methods on Neverland)
```

>update data on ```stock_badge_mahoroba```		
>(follow info on ```transaction_manifest_info\stock_badge_mahoroba_info```)

>```stock_badge_mahoroba.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0611566 XRD burned, 0.00305783 XRD tipped to validators
Cost Units: 100000000 limit, 611566 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```


>update data on ```ask_setting```		

>```ask_setting.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.6050767 XRD burned, 0.030253835 XRD tipped to validators
Cost Units: 100000000 limit, 6050767 consumed, 0.0000001 XRD per cost unit
Logs: 30
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Marketplace fee set to 3%
├─ [INFO ]  AssetSquare fee set to 5%
├─ [INFO ]  Asset royalty fee set to 1%
├─ [INFO ]  TKN token resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe
├─ [INFO ]  AssetSquare address set to component_sim1q2d9c9wus40556ujg386n3ada7qmv72lctr2lx6y5m3sg733dr
├─ [INFO ]  Asset Badge address set to resource_sim1qpw8n3mjp77e5epe965u0k04lgzdyc57f3e6ky82r6wsszdajh
├─ [INFO ]  Asset Oracle address set to component_sim1qtwfalw8frs3pgywpsheurr5phqwa28wq4tfmc9k3kassnfzgl
├─ [INFO ]  Asset Vault address set to component_sim1qf006wy2qclusvr3z0p469rdrv2qlp7f78t379eg9q4qr8gffg
├─ [INFO ]  Academy Vault address set to component_sim1qf006wy2qclusvr3z0p469rdrv2qlp7f78t379eg9q4qr8gffg
├─ [INFO ]  Academy Share set to 30%
├─ [INFO ]  Auction deadline set to 5000
├─ [INFO ]  Auction last bid deadline set to 5
├─ [INFO ]  Buy proposal deadline set to 5000
├─ [INFO ]  =====================================================================================
├─ [INFO ]  =====================================================================================
├─ [INFO ]  Marketplace fee set to 3%
├─ [INFO ]  AssetSquare fee set to 3%
├─ [INFO ]  Asset royalty fee set to 1%
├─ [INFO ]  TKN token resource_sim1qqrynk6yx98r6ddfrz2l0n2hz2cved95upn5v3x4ygnswqk2qe
├─ [INFO ]  AssetSquare address set to component_sim1q2d9c9wus40556ujg386n3ada7qmv72lctr2lx6y5m3sg733dr
├─ [INFO ]  Asset Badge address set to resource_sim1qqmtd06xhc992zh7gxhzsere50pgkygm04wks9lqax9snmg5cu
├─ [INFO ]  Asset Oracle address set to component_sim1qtwfalw8frs3pgywpsheurr5phqwa28wq4tfmc9k3kassnfzgl
├─ [INFO ]  Asset Vault address set to component_sim1qf006wy2qclusvr3z0p469rdrv2qlp7f78t379eg9q4qr8gffg
├─ [INFO ]  Academy Vault address set to component_sim1qtwfalw8frs3pgywpsheurr5phqwa28wq4tfmc9k3kassnfzgl
├─ [INFO ]  Academy Share set to 0%
├─ [INFO ]  Auction deadline set to 5000
├─ [INFO ]  Auction last bid deadline set to 5
├─ [INFO ]  Buy proposal deadline set to 5000
└─ [INFO ]  =====================================================================================
|
New Entities: 0
```

Kindly note MarketplaceVault component address on Mahoroba AssetSquare is still preinitialized one, in fact
I didn't set up a specific vault component as for this test isn't required. If you want to enable buying Asset NFTs
on Mahoroba you need to set it up and update its MarketplaceVault component address value. 


[Back Up](#index)
#
### Part_15
### Transfer SBT Updater Badge from LandData Component to others Neverland environment Components allowing them to update data 
### of user's SBT. 
###
### List of Neverland Components able to update SBT data:
### 
### Academy, AssetSquare, House Hub, LandAuction, DemoTols, 
### although the latter is only a test component to mint some Asset NFT and manipulate some data to 
### perform some isolated tests without having to set up the entire environment.  
### 
### A SBT updater Badge is also sent to Mahoroba AssetSquare Component from Mahoroba AssetSquare LandData Component
### to allow SBT data update on Mahoroba users who buy Asset NFTs on Neverland AssetSquare.
-------------------------------------------------------------------------------------------

>cd land_data_transaction_manifest


>update data on ```transfer_updater_badge_neverland```		


>```transfer_updater_badge_neverland.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2619328 XRD burned, 0.01309664 XRD tipped to validators
Cost Units: 100000000 limit, 2619328 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```


>update data on ```transfer_updater_badge_mahoroba```		


>```transfer_updater_badge_mahoroba.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0839363 XRD burned, 0.004196815 XRD tipped to validators
Cost Units: 100000000 limit, 839363 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```


[Back Up](#index)
#
### Part_16
## Setup a DemoTools Component, not mandatory as unused within current test session, however useful if someone   
## wish to test various methods through different Neverland enviroment Components without perform a complete setup,
## as it allow tester to mint Asset NFs as well as update SBT data, once stored a SBT Updater Badge within his vault.
--------------------------------------------------------------------------------------------------------------------

demo_tools_instantiate.sh


>cd instantiate_components_transaction_manifest

>update data on ```demo_tools_instantiate```		


>```demo_tools_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0649658 XRD burned, 0.00324829 XRD tipped to validators
Cost Units: 100000000 limit, 649658 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 4
└─ Component: component_sim1qtaxue485rswfzcx9dqcufv2xdl0u0wuuv28a590tqrqp02pvl
├─ Resource: resource_sim1qpudqjee57qylkcp9hy9hst9htqwgdlcfmtzr7rude0qthdxfa
├─ Resource: resource_sim1qrug6t6jau5x9a95t6s5cu8yr7d73ugzqt0yqsddpg2qgzq9hv
└─ Resource: resource_sim1qpzqf42cslqqlx205hhm08ht5p4ag3ncvmp0e9yjlwnsrnkcu5
```

>cd land_data_transaction_manifest

>update data on ```transfer_updater_badge```		

>```transfer_updater_badge.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0678185 XRD burned, 0.003390925 XRD tipped to validators
Cost Units: 100000000 limit, 678185 consumed, 0.0000001 XRD per cost unit
Logs: 0
|
New Entities: 0
```


[Back Up](#index)

