![](./images/logo-cubi4_scritta1.jpg)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Portfolio dApp is a proof-of-concept protocol of a collaborative portfolio management solution built on the Radix ledger using v0.4.1 of Scrypto: the smart contract language of the Radix ledger.

## Table of Content

  * [Abstract](#abstract)
  * [Design](#design)
  * [Portfolio dApp demo](#portfolio-dApp-demo)
  * [Portfolio dApp demo (Transaction Manifest only)](#portfolio-dApp-demo(Transaction-Manifest-only))
  * [Integration Test](#integration-Test)
  * [Unit Test](#unit-Test)    
  * [TODO & Useful commands](#TODO-&-Useful-commands)     
  
# Abstract 

The Portfolio dApp is a decentralized application where users can collaborate to a portfolio management solution where no users profiling exist and it is completely permissionless, any user can deposit asset and any user can put asset at work towards the simulated available deFi applications.

It is a kind of a social trading application but it differs from it because we think that financial knowledge is spread and also the final retail user has a great understanding and could operate in such a broad market for opportunities.

It is different from a usual social trading application where retail traders have the opportunity to follow experienced traders copying their portfolio.
It is different because I think that also the experts get things wrong so there is no guarenteed result and also I think that is a no-brainer for a retail trader to simply copy trades decided by others.

This is a social collaborative portfolio management solution where anyone can put its tokens and execute operation on behalf of the whole community, at the time of withdraw the user account gets back its tokens with a reward or penalty based on the performance of the portfolio. There is not a personal user account on the platform, the positive or negative portfolio result is redistributed among all participants,

Portfolio dApp has only some simple rules:
* users can deposit asset
* users can reedem asset at any moment
* users can put asset at work until 10X of its asset value
* users can close any current operation only if it's current value is lower than initial value
* users can close their operations anytime

The rules aim: 
   - to incentivates users that have not sufficient capital to put it at work but have instead knowledge 
   - to help users to close their losing position that is usually a difficult decision to put in action
   - empower the user that his operations are valid for everyone

Operations history will be registered on each user so anyone can evaluate each other's.

Simulated deFi applications used in this portfolio management solution are the following:
- Lending application (we'll use LendingdApp developed for the previous challenge)
- Trading application [Price changes is simulated and changes randomly when epoch advances]

# Design

Blueprint can create new components with only a single vault and a map containing all the info about the operation opened/in place/closed.

Each user account has to register itself getting back wo nft, one containing the amount of tokens funded in the portfolio and the other one for containing its operations summarized history (let's call it user_history_nft and user_funding_nft)

Component has some very simple method for depositing/taking from the main vault:
- fund_portfolio(bucket, user_funding_nft) -> tokens are put in the main vault -> user_funding_nft (not transferable) get the data updated with the  amount of tokens deposited
- withdraw_portfolio(proof, amount) -> a bucket gets created with tokens from the main vault and sent back if the account has presented a valid proof.
The amount of tokens is increased or decreased respecting the ratio of increment/decrease of the main vault. The user account gets rewarded for the result that all the partecipants to the portfolio had reached. 

And some others for executing orders/operations toward the [TradingApp component](TRADING.md):
- buy(amount, user_account_address, resource_address, user_funding_nft) -> a buy order is issued using the 'TradingApp' for the amount specified and the resource address is the token that will be bought, while the user_account_address and the user_funding_nft will be used to calculate the max leverage allowed (10x of the personal fundings in the portfolio)
- sell(proof, amount, resource_address) -> a sell order is issued using the 'TradingApp' for the amount specified and the resource address

And some others for executing orders/operations toward the [LendingApp component](https://github.com/radixdlt/scrypto-challenges/tree/main/3-lending/LendingDApp#readme):
- register_for_lending() -> a register to lend the LendingdApp component (anyone can also lend some tokens when the main vault has liquidity)
- register_for_borrowing() -> a register to borrow to the LendingdApp component (anyone can also borrow some tokens to add liquidity to the main vault)
- lend(decimal) -> To ask for a lending you only need to specify the amount (lending_nft is inside the portfolio component) 
- take_back(decimal) -> To get back the lending you only need to specify the amount (borrowing_nft is inside the portfolio component)

And also some for closing orders/operations:
- close_position(positionId) -> any account can close a position if it is a losing one, it only need its positionId
- position() -> calculates and outputs the list of open position (positionId can be used for closing)
- portfolio_value() -> Calculate the value of the other vaults (getting the token current price from the tradingapp component)
- portfolio_total_value() -> Calculate the total value of the portfolio (main vault + other vaults + lnd vault)

The following methods are for the portfolio admin manager and are protected by access rules:
- sell                  -> The admin can close any opened position 
- close_all_positions   -> The admin can close all the opened position with a single request
- reset_positions       -> The admin can reset the list of positions 


# Portfolio dApp demo

Let's proceed with a demo of the blueprints, start publishing the package

```
resim publish .
export package=017045972dc31c4425bde71adf087ddedbf7b10adf56ec71a6ce1b
```

Then create the first account, name it Bob (the Admin)

```
resim new-account
A new account has been created!
Account component address: 021025cfda90adea21506170be47c67ec169e41dbbdd063d54d409
Public key: 0426599006343468593571484e418be9c40db4ffaf60ff9d98e6ccb13aec950ce2b70cd20e293909a6f1c380ab2cbad6c527c552c6aa7b7a664722df08fda26b8f
Private key: d86e8112f3c4c4442126f8e9f44f16867da487f29052bf91b810457db34209a4
export account=02e0905317d684478c275540e2ed7170f217e0c557805f7fd2a0d3
export priv1=35be322d094f9d154a8aba4733b8497f180353bd7ae7b0a15f90b586b549f28b
```

Then create the second account, name it John

```
resim new-account
A new account has been created!
Account component address: 02d0da3fc806e20c508841efdcd412a53e50d1b80fb35ff1263214
Public key: 042f3ce83809c2c67057ff9aba2a95127e729b5439993051cc168a2939f655c904e976cf6db5cc51106dcd83b4b24d75c5e4e6f07c948b2cbca6eaef82bdc81832
Private key: f0a0278e4372459cca6159cd5e71cfee638302a7b9ca9b05c34181ac0a65ac5d
export priv2=f13ee6ed54ea2aae9fc49a9faeb5da6e8ddef0e12ed5d30d35a624ae813e0485
export account2=02b61acea4378e307342b2b684fc35acf0238a4accb9f91e8a4364
```

Then create the third account, name it Max

```
resim new-account
A new account has been created!
Account component address: 02bf3aa95784d95a63dd6f8e3f0d06de6127e114cc275a13ae47b5
Public key: 04c1b4e1a0f1290b46b1836c4c4a9e6c7c963eb9b71e91bc0c3b32a99f79081634aa9719b7f8e5019bb918ace34f29d2ed66449eaf1c43deb9993642add0417b5a
Private key: 205df2fd636e9a2b6e81c3987fa3dcdd09d64c5c710dd61aaa50a97d222a3f74
export account3=0200098f161a7691fa7ae380e41aed27ab5c4f969e8e563ce4275a
export priv3=aae89fc0f03e2959ae4d701a80cc3915918c950b159f6abb6c92c1433b1a8534
```

Set to the first account and then create the tokens to be used in the trading dapp

```
resim set-default-account $account $priv1
resim new-token-fixed --name bitcoin --symbol btc 10000
resim new-token-fixed --name ethereum --symbol eth 1000
resim new-token-fixed --name leonets --symbol leo 100
export xrd=030000000000000000000000000000000000000000000000000004
export leo=03b5c2df770cac9a330c3a535a7da82054d1ef1dba6f29302e2dee
export eth=030d4d068757932f986bb98fb65166cd7e3c20b43e71cc775e687b
export btc=039179c95de06571b3cec262e71854f9296f025f30d1688a3cae56
```

Then create the TradingApp component, it needs the resource address of 4 tokens and later on it needs to have its vaults funded

```
resim call-function $package TradingApp create_market $xrd $btc $eth $leo
resim run transactions/create_market.rtm 
export trading=0210e82c05fd5ea2c5413f8571a1f43df537a47b2d26613f36095e
```

The TradingApp component has been created so now Bob needs to fund its vaults

```
resim call-method $trading fund_market 1000,$xrd 1000,$btc 1000,$eth 100,$leo
resim run transactions/fund_market.rtm 
```

This is the Bob's account after he has funded the TradingApp component
```
Resources:
├─ { amount: 9000, resource address: 0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630, name: "bitcoin", symbol: "btc" }
├─ { amount: 0, resource address: 032bad3fbe45916ae1bad5492668f28f76960a8527e263993b78d9, name: "leonets", symbol: "leo" }
├─ { amount: 0, resource address: 03342be5a1549bfa7f3a58b1c0e0669a96f99eec47a2bcb2ac12ef, name: "ethereum", symbol: "eth" }
└─ { amount: 999000, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

Now the trading market has been funded and vault are ready for accepting buy/sell trades

This are the vaults in the TradingApp component
```
Resources:
├─ { amount: 1000, resource address: 03342be5a1549bfa7f3a58b1c0e0669a96f99eec47a2bcb2ac12ef, name: "ethereum", symbol: "eth" }
├─ { amount: 100, resource address: 032bad3fbe45916ae1bad5492668f28f76960a8527e263993b78d9, name: "leonets", symbol: "leo" }
├─ { amount: 1000, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
└─ { amount: 1000, resource address: 0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630, name: "bitcoin", symbol: "btc" }
```

Then we can create another component to be used in the Portfolio App

```
resim call-function $package LendingApp instantiate_pool 1000,$xrd 1000 10 7
resim run transactions/create_lending.rtm 
export lending=02c887fe5316b2b9fcecfed965b308a67e207a5df67bbf17282f98
export admin_badge=03d987113ce50a6077a4b4b5b9ef29e6798c20c79a1b1370d56893
export lend_nft=0345a475f23e171428540acd6dfc2628229480614ddbb069cde5b0
export borrow_nft=0393dfaf83eff4942e65ac1b587ed989bfe6ea2adc432e6b99f972
export lnd=03629e07a727c9b17ed3b5984701ec846872bd09e6ba7d6aa3de85
```

The LendingApp component output its address plus some resource address we will not use here, except for the lending_nft and the lnd token

```
New Entities: 5
└─ Component: 02a10c7dd7c36a011a7f832ea715486fc81306f90e5e752d9a1f72
├─ Resource: 03559019d9ab6c597fb6e4cdfeeec2ea6b3e9e329ce9ca1cc5adfc
├─ Resource: 0391dadf83168f81053b8ea363479a8d773f1472f46fae205d93d0
├─ Resource: 03c94831cea71e10ff2c77e94bc871157454de281e1c0eca6254ab
└─ Resource: 0376d186eaf84de50bf41e986d09e6fa91a4e379b6967b7c4ca21f
```

This component is the same from the previous challenge and has been reused here from the main component

This is the output from the component creation.

```
Logs: 5
├─ [INFO ] Starting amount is: 1000
├─ [INFO ] Fee for borrower is: 10
├─ [INFO ] Reward for lenders is: 7
├─ [INFO ] Loan pool size is: 1000
└─ [INFO ] Main pool size is: 1000
```

Now we have created the two components we need for the main component, the PortfolioApp

```
resim call-function $package Portfolio new $xrd $btc $eth $leo $lending $trading $lend_nft $borrow_nft $lnd
resim run transactions/create_portfolio.rtm 
```

The call-function outputs the address of the component/resources created
```
New Entities: 4
└─ Component: 025d180a419b7d526eba63c0b971b875885c0618fc219843685549
├─ Resource: 03cb755aaf8f17311a8fb00fafe507f7f27a87eff48e40aa1f3d9e
├─ Resource: 03a3c7111213486713c0d5ad6ac43fc7cdbe5f6353df8247b8d94b
└─ Resource: 032a450d815ecda8c1bfccd52e608a61ce8fec23a21892e2d1314b
```

Let's export the variables we'll need later

```
$ export portfolio=025d180a419b7d526eba63c0b971b875885c0618fc219843685549
$ export ADMIN_BADGE=03cb755aaf8f17311a8fb00fafe507f7f27a87eff48e40aa1f3d9e
$ export user_account_history_nft=03a3c7111213486713c0d5ad6ac43fc7cdbe5f6353df8247b8d94b
$ export user_account_funding_nft=032a450d815ecda8c1bfccd52e608a61ce8fec23a21892e2d1314b
```

The following operation we need to execute is the register, we have two different types of registering, one for the user account to operate on the PortfolioApp and the others for the PortfolioApp itself with the LendingApp component

```
resim call-method $portfolio register $account
resim run transactions/register_with_portfolio.rtm
```

After the user account has been registered itself with the PortfolioApp component we can see the NFT that has been added to its resource's list 
```
├─ { amount: 1, resource address: 03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9, name: "User Account Trading History" }
│  └─ NonFungible { id: 0bfa93aa9159a62422fd0868d0ae4a16e32eff89f39d206bb5eb8267f265c424, immutable_data: Struct(), mutable_data: Struct(ComponentAddress("021025cfda90adea21506170be47c67ec169e41dbbdd063d54d409"), 0u32, 0u32, false) }
```

Let's register with the component

```
resim call-method $portfolio register_for_lending 
resim run transactions/register_for_lending.rtm
```

Here we get the output log from the LendingApp component 

```
Logs: 5
├─ [INFO ] Registering for lending 
├─ [INFO ] Vault for Lending NFT, accept resource address : 0391dadf83168f81053b8ea363479a8d773f1472f46fae205d93d0 
├─ [INFO ] Min/Max Ratio for lenders is: 5  20
├─ [INFO ] Extra L1 reward is : 0.4 and L2 reward is : 0.8
└─ [INFO ] Lending NFT resource address : 0391dadf83168f81053b8ea363479a8d773f1472f46fae205d93d0 
```

Let's register with the component

```
resim call-method $portfolio register_for_borrowing (resim run transactions/register_for_borrowing.rtm)
```

Here we get the output log from the LendingApp component

```
Logs: 3
├─ [INFO ] Registering for borrowing 
├─ [INFO ] Min/Max Ratio for borrowers is: 3  12
└─ [INFO ] Bonus L1 fee is : 0.4 and L2 is : 0.8
```

After the registering we can find the lending/borrowing NFT in the component account instead of in the user account, this is because in this demo
it'll be the PortfolioApp component that we'll cooperate with the LendingApp component. The user account has not direct access to the lendings/borrowings (Otherwise if he obviouvsly wants to do he has to register itself).

If we look at the resources in the PortfolioApp component we can see the new NFTs

```
└─ { amount: 1, resource address: 0391dadf83168f81053b8ea363479a8d773f1472f46fae205d93d0, name: "Lending NFTs" }
   └─ NonFungible { id: 8afb5440391776aa8ca6dc33d95feb05cf5d595c3f9972b8f4e39c10d503c66d, immutable_data: Struct(), mutable_data: Struct(0i32, false, false, false) }
```

Bob could operate directly with the TradingApp...

```
resim call-method $trading buy_generic 500,$xrd $btc 
resim run transactions/buy_with_trading_btc.rtm
```

The current pair value of this operation for this demo is fixed and is xrd/btc = 40 so Bob gets 12.5 btc for its 500 xrd

```
├─ [INFO ] N. to buy: 12.5
```

Let's advance some epoch so to let the price changes...

```
epoch=$(($epoch + 1))
resim set-current-epoch $epoch
```

Then let's look at the price...

```
resim call-method $trading current_price $xrd $btc (resim call-method $trading current_price $xrd $btc --manifest transactions/current_price.rtm)

├─ [INFO ] Current epoch 1 vs last epoch 0
├─ [INFO ] The random movement is: 5 and direction is 1 
└─ [INFO ] New price is : 45 
```

Let's advance again some epoch so to let the price changes...
```
epoch=$(($epoch + 1))
resim set-current-epoch $epoch
```

Then let's look at the price again...


```
resim call-method $trading current_price $xrd $btc (resim call-method $trading current_price $xrd $btc --manifest transactions/current_price.rtm)

├─ [INFO ] Current epoch 2 vs last epoch 1
├─ [INFO ] The random movement is: 9 and direction is 0 
└─ [INFO ] New price is : 36 
```

Now Bob decides to sell 


```
resim call-method $trading sell_generic 12.5,$btc  
resim run transactions/sell_with_trading.rtm

├─ [INFO ] Current epoch 2 vs last epoch 2
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630 is 36 
└─ [INFO ] N. xrd to receive: 450
```

So Bob got 10% less from its trade operation.

Now let's instead what could happen if Bob uses the PortfolioDapp

In this example Bob, as all the other users, has to fund directly inside the PortfolioApp component before starting to operate

```
resim call-method $portfolio fund_portfolio 10000,$xrd 1,$user_account_funding_nft 
resim run transactions/fund_portfolio_by_Bob.rtm
```

The user account of Bob show the NFT of its 10000 xrd funded, he obsiously will need this to get back its xrd tokens

```
├─ { amount: 1, resource address: 03cacd11c325cd75f7693ed8d99187f65ec303bdc1a0622cca283f, name: "User Account Funding Data NFTs" }
│  └─ NonFungible { id: 2da0d5453fe732fbf26d621497e2cee62d8f12459995e240604c9fe5acf65f11, immutable_data: Struct(), mutable_data: Struct(Decimal("10000"), true, Decimal("10000"), Decimal("100"), 2u64) }
```

The same should be done by John and Max.

At the end of their funding we can look at the portfolio component.

```
$resim set-default-account $account2 $priv2

Default account updated!

resim call-method $portfolio register $account2 
resim run transactions/register_with_portfolio_by_John.rtm
```

Also John gets its NFT

```
├─ { amount: 1, resource address: 03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9, name: "User Account Trading History" }
│  └─ NonFungible { id: 63d6d06bb0ba110877aff6def72a699ae852f4f7f14c546ef11f8c69638f47d7, immutable_data: Struct(), mutable_data: Struct(ComponentAddress("02d0da3fc806e20c508841efdcd412a53e50d1b80fb35ff1263214"), 0u32, 0u32, false) }

export user_account_funding_nft2=032a450d815ecda8c1bfccd52e608a61ce8fec23a21892e2d1314b
```


And then he can fund the Portfolio

```
resim call-method $portfolio fund_portfolio 10000,$xrd $user_account_funding_nft2
resim run transactions/fund_portfolio_by_John.rtm 
```

The same has been done with Max's account

```
$resim set-default-account $account3 $priv3

resim call-method $portfolio register $account3 
resim run transactions/register_with_portfolio_by_Max.rtm

export user_account_funding_nft3=032a450d815ecda8c1bfccd52e608a61ce8fec23a21892e2d1314b

resim call-method $portfolio fund_portfolio 10000,$xrd $user_account_funding_nft3
resim run transactions/fund_portfolio_by_Max.rtm
```

At this point the Portfolio has been funded all the user account registered

```
├─ { amount: 30000, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

So let's start trading on behalf of the PortfolioApp !!

At the beginning it does not exist no open position as we can check with the followings

```
resim run transactions/show_positions.rtm 
```

So let's execute some operation, Max and and John are buying 


```
resim run transactions/buy_by_Max.rtm

├─ [INFO ] N. to buy: 27.777777777777777777
└─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630 is 36 
```

Let's advance some epoch to let the price change so also Bob can buy

```
├─ [INFO ] N. to buy: 33.333333333333333333
└─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630 is 30 
```

If we look at the position now we can see all of these 3 operation:

```
├─ [INFO ] Position size 3
├─ [INFO ] Position Id 267625258182426733516147742632270442300
├─ [INFO ] Xrd used for trade 1000
├─ [INFO ] Starting price 36
├─ [INFO ] Current price 30

├─ [INFO ] Position Id 40219526619530914621781254589091376690
├─ [INFO ] Xrd used for trade 1000
├─ [INFO ] Starting price 36
├─ [INFO ] Current price 30

├─ [INFO ] Position Id 230509327473859403985102491547209909823
├─ [INFO ] Xrd used for trade 1000
├─ [INFO ] Starting price 30
├─ [INFO ] Current price 30
```

Let's advance some epoch again a let's look at the position

```
├─ [INFO ] Position size 3
├─ [INFO ] Position Id 267625258182426733516147742632270442300

├─ [INFO ] Xrd used for trade 1000
├─ [INFO ] Starting price 36
├─ [INFO ] Current price 45

├─ [INFO ] Position Id 40219526619530914621781254589091376690
├─ [INFO ] Xrd used for trade 1000
├─ [INFO ] Starting price 36
├─ [INFO ] Current price 45

├─ [INFO ] Position Id 230509327473859403985102491547209909823
├─ [INFO ] Xrd used for trade 1000
├─ [INFO ] Starting price 30
├─ [INFO ] Current price 45
```

At this point John decides to withdraw and gets 10333 xrd in face of its funding of 10000, this result comes from the collaboration trades opened by all the user accounts!!
 
``` 
├─ [INFO ] Position size inside portfolio 3
├─ [INFO ]  Portfolio amount at time of funding 30000 and actual 30999.999999999999999915 
├─ [INFO ]  Portfolio increase/decrease ratio  3.3333333333333333 
└─ [INFO ]  you got 10333.33333333333333 from 10000 in 3 epoch
```

The portfolio now contains the following

```
Resources:
├─ { amount: 88.888888888888888887, resource address: 0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630, name: "bitcoin", symbol: "btc" }
└─ { amount: 16666.66666666666667, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

Let's look now at how we can close the operation

Max for example looks at the position and decides to close its position but also all the other ones because he thinks the price will decrease

```
resim call-method $portfolio close_position 230509327473859403985102491547209909823

├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630 is 45 
└─ [INFO ] N. xrd to receive: 1499.999999999999999985

resim call-method $portfolio close_position 40219526619530914621781254589091376690

├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630 is 45 
└─ [INFO ] N. xrd to receive: 1249.999999999999999965

resim call-method $portfolio close_position 267625258182426733516147742632270442300

├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630 is 45 
└─ [INFO ] N. xrd to receive: 1249.999999999999999965
```


No trading operation are open now, so for example Max decides to lend some of the current liquidity the get some reward

```
resim run transactions/lend.rtm
resim run transactions/takeback.rtm
```

The portfolio now contains again only xrd tokens and Bob and Max are obviously allowed to withdraw.

```
├─ { amount: 20666.666666666666669915, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

Bob withdraws and unfortunately he gets less than that he funded in but this is a situation can happen.

```
resim run transactions/withdraw_by_Bob.rtm

├─ [INFO ]  Amount of funded tokens in the portfolio 20000 
├─ [INFO ]  Amount of yours funded tokens in the portfolio 10000 

├─ [INFO ]  Portfolio amount at time of funding 20000 and actual 19950 
├─ [INFO ]  Portfolio increase/decrease ratio  -0.25 
├─ [INFO ]  you got 9975 from 10000 in 3 epoch 
└─ [INFO ]  Updated Amount of funded tokens  0 
```

At the final time the portfolio contains some xrd in the main vault, some lnd from the LendingApp and some tokens bought with the TradingApp

```
resim run transactions/portfolio_total_value.rtm
─ [INFO ] Position size inside portfolio 2
├─ [INFO ] Added value from token1 vault 550
├─ [INFO ] Added value from token2 vault 500
├─ [INFO ] Added value from token3 vault 0
├─ [INFO ] Value in main vault 8925
├─ [INFO ] Value in lnd vault 107
└─ [INFO ] Grandtotal 10082
```

# Portfolio dApp demo (Transaction Manifest only)

Let's proceed again with a demo of the blueprints, here we recap the final test done using transaction manifest only.
Let's start publishing the package, all the component/resource need to be exported as shell variable, then the transaction manifest are already using the exact variable names, so please be sure to use the following names!

```
resim publish .
export package=017045972dc31c4425bde71adf087ddedbf7b10adf56ec71a6ce1b

resim new-account

export account=
export priv1=


resim new-account

export priv2=
export account2=

resim new-account

export account3=
export priv3=


resim set-default-account $account $priv1

resim new-token-fixed --name bitcoin --symbol btc 10000

resim new-token-fixed --name ethereum --symbol eth 1000

resim new-token-fixed --name leonets --symbol leo 1000

export xrd=030000000000000000000000000000000000000000000000000004
export leo=
export eth=
export btc=

resim call-function $package TradingApp create_market $xrd $btc $eth $leo
resim run transactions/create_market.rtm 

export trading=


resim call-method $trading fund_market 1000,$xrd 1000,$btc 1000,$eth 100,$leo
resim run transactions/fund_market.rtm 


resim call-function $package LendingApp instantiate_pool 1000,$xrd 1000 10 7
resim run transactions/create_lending.rtm 


export lending=
export admin_badge=
export lend_nft=
export borrow_nft=
export lnd=


resim call-function $package Portfolio new $xrd $btc $eth $leo $lending $trading $lend_nft $borrow_nft $lnd
resim run transactions/create_portfolio.rtm 


export portfolio=
export admin_badge=
export user_account_history_nft_address=
export user_account_funding_nft_address=

resim call-method $portfolio register $account
resim run transactions/register_with_portfolio.rtm
```

After the user account has been registered itself with the PortfolioApp component we can see the NFT that has been added to its resource's list 

```
├─ { amount: 1, resource address: 03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9, name: "User Account Trading History" }
│  └─ NonFungible { id: 0bfa93aa9159a62422fd0868d0ae4a16e32eff89f39d206bb5eb8267f265c424, immutable_data: Struct(), mutable_data: Struct(ComponentAddress("021025cfda90adea21506170be47c67ec169e41dbbdd063d54d409"), 0u32, 0u32, false) }

export user_account_history_nft=03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9
```
Then we can continue registering with the LendingApp

```
resim run transactions/register_for_lending.rtm

resim run transactions/register_for_borrowing.rtm
```

Now all the components are ready and Bob could operate directly with the TradingApp...

```
resim run transactions/buy_with_trading.rtm
```

Let's advance some epoch so to let the price changes...

```
epoch=$(($epoch + 1))
resim set-current-epoch $epoch
```

Then let's look at the price...

```
└─ [INFO ] New price is : 39
```

Now Bob decides to sell 


```
resim run transactions/sell_with_trading_btc.rtm

├─ [INFO ] Current epoch 2 vs last epoch 2
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630 is 36 
└─ [INFO ] N. xrd to receive: 487.5
```

So Bob got less from its trade operation.

Now let's instead what could happen if Bob uses the PortfolioDapp

In this example Bob, as all the other users, has to fund directly inside the PortfolioApp component before starting to operate

```
resim run transactions/fund_portfolio_by_Bob.rtm
```

The user account of Bob show the NFT of its 10000 xrd funded, he obsiously will need this to get back its xrd tokens

```
├─ { amount: 1, resource address: 03cacd11c325cd75f7693ed8d99187f65ec303bdc1a0622cca283f, name: "User Account Funding Data NFTs" }
│  └─ NonFungible { id: 2da0d5453fe732fbf26d621497e2cee62d8f12459995e240604c9fe5acf65f11, immutable_data: Struct(), mutable_data: Struct(Decimal("10000"), true, Decimal("10000"), Decimal("100"), 2u64) }
```

The same should be done by John and Max.

At the end of their funding we can look at the portfolio component.

```
resim set-default-account $account2 $priv2

Default account updated!

resim run transactions/register_with_portfolio_by_John.rtm
```

Also John gets its NFT

```
resim show $account2

export user_account_funding_nft2=
```


And then he can fund the Portfolio

```
resim run transactions/fund_portfolio_by_John.rtm 
```

The same has been done with Max's account

```
$resim set-default-account $account3 $priv3

resim run transactions/register_with_portfolio_by_Max.rtm

export user_account_funding_nft3=

resim run transactions/fund_portfolio_by_Max.rtm
```

At this point the Portfolio has been funded all the user account registered

```
├─ { amount: 30000, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

So let's start trading on behalf of the PortfolioApp !!

At the beginning it does not exist no open position as we can check with the followings

```
resim run transactions/show_positions.rtm 
```

So let's execute some operation, Max ,  John and Bob are buying 


```
resim run transactions/buy_by_Max.rtm

resim run transactions/buy_by_John.rtm

resim run transactions/buy_by_Bob.rtm
```

At this point the portfolio is filled with some different tokens!!
 
``` 
Resources:
├─ { amount: 12.820512820512820512, resource address: 03fd755fa368cb0b27571d485ab4f5aef45b395e53116a68b378b4, name: "bitcoin", symbol: "btc" }
├─ { amount: 1, resource address: 03db0e91c949aaa724d3c3c881de727c856d342a409a9e3908ef34, name: "Lending NFTs" }
│  └─ NonFungible { id: 826cf39e6dbc27111f5d3ff63ec112c144e0e57f193ff051d8a6b8797727b71b, immutable_data: Struct(), mutable_data: Struct(0i32, false, false, false) }
├─ { amount: 28500, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
├─ { amount: 50, resource address: 0345e6a8b141c210e7421387da83a62ca0fcb388b45f61448f3484, name: "ethereum", symbol: "eth" }
├─ { amount: 0, resource address: 030d9e30c06cd711af6f76bd415ebcda1c19dda165a48291c8b0d7, name: "Loan token", symbol: "LND" }
├─ { amount: 1, resource address: 03b1f068cfcec8c34f7f8fe0a8830b9190f5ef2cd433c08c3a1bd1, name: "Admin Badge" }
├─ { amount: 100, resource address: 03de10b1672c9a8ab30bd71f719dd3b6c94a771ed68941b72e5187, name: "leonets", symbol: "leo" }
└─ { amount: 1, resource address: 03ef002af8c51cbb344dd813838e67f2c723b60c81f73a117ee672, name: "Borrowing NFTs" }
   └─ NonFungible { id: 0e53576b148454554d40654e6ba1c0ba6f3fc76352609783dde517c218748036, immutable_data: Struct(), mutable_data: Struct(0i32, Decimal("0"), false, false, false) }
```


Let's look now at how we can close the operation

Max for example looks at the position and decides to close its position but also all the other ones because he thinks the price will decrease

```
resim call-method $portfolio close_position 230509327473859403985102491547209909823

├─ [INFO ] === SELL OPERATION START === 
├─ [INFO ] Current epoch 20 vs last epoch 20
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/03de10b1672c9a8ab30bd71f719dd3b6c94a771ed68941b72e5187 is 5 
├─ [INFO ] N. xrd to receive: 500
└─ [INFO ] === SELL OPERATION END === 
```


No trading operation are open now, so for example Max decides to lend some of the current liquidity the get some reward

```
resim run transactions/lend.rtm
resim run transactions/takeback.rtm
```

The portfolio now contains again only xrd tokens and Bob and Max are obviously allowed to withdraw.

```
└─ { amount: 30007, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

Max withdraws 

```
resim run transactions/withdraw_by_Max.rtm

├─ [INFO ] === WITHDRAW PORTFOLIO OPERATION START === 
├─ [INFO ]  Amount of funded tokens in the portfolio 30000 
├─ [INFO ]  Amount of yours funded tokens in the portfolio 10000 

├─ [INFO ]  Portfolio amount at time of funding 30000 and actual 30007 
├─ [INFO ]  Portfolio increase/decrease ratio  0.0233333333333333 
├─ [INFO ]  you got 10002.33333333333333 from 10000 in 0 epoch 
```

Bob and John also can withdraw their amount or continue, getting a reward based on the total portfolio result, whose result has been achieved by all cumulative efforts ! 

# Check this

├─ [INFO ] === WITHDRAW PORTFOLIO OPERATION START === 
├─ [INFO ]  Amount of funded tokens in the portfolio 30000 
├─ [INFO ]  Amount of yours funded tokens in the portfolio 10000 
├─ [INFO ] Position size inside portfolio 0
├─ [INFO ] Current epoch 20 vs last epoch 20
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/03fd755fa368cb0b27571d485ab4f5aef45b395e53116a68b378b4 is 39 
├─ [INFO ] Current epoch 20 vs last epoch 20
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0345e6a8b141c210e7421387da83a62ca0fcb388b45f61448f3484 is 10 
├─ [INFO ] Current epoch 20 vs last epoch 20
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/03de10b1672c9a8ab30bd71f719dd3b6c94a771ed68941b72e5187 is 5 
├─ [INFO ] 0 tokens are valued xrd 0
├─ [INFO ] 0 tokens are valued xrd 0
├─ [INFO ] 0 tokens are valued xrd 0
├─ [INFO ]  Portfolio amount at time of funding 30000 and actual 30007 
├─ [INFO ]  Portfolio increase/decrease ratio  0.0233333333333333 
├─ [INFO ]  you got 10002.33333333333333 from 10000 in 0 epoch 
└─ [INFO ]  Updated Amount of funded tokens  0 
New Entities: 0
lbattagli@DLT016:~/Software/Rust/radixdlt/scrypto-challenges/4-portfolio-management-yield-farming/PortfolioDApp$ 
lbattagli@DLT016:~/Software/Rust/radixdlt/scrypto-challenges/4-portfolio-management-yield-farming/PortfolioDApp$ 
lbattagli@DLT016:~/Software/Rust/radixdlt/scrypto-challenges/4-portfolio-management-yield-farming/PortfolioDApp$ 
lbattagli@DLT016:~/Software/Rust/radixdlt/scrypto-challenges/4-portfolio-management-yield-farming/PortfolioDApp$ resim set-default-account $account $priv2
Default account updated!
lbattagli@DLT016:~/Software/Rust/radixdlt/scrypto-challenges/4-portfolio-management-yield-farming/PortfolioDApp$ resim run transactions/withdraw_by_John.rtm 
Transaction Status: InvokeError
Execution Time: 78 ms
Instructions:
├─ CallMethod { component_address: 02f5ad66df5f5dc26a67939b1d03ae3b5708f42a3466fcfa8cb130, method: "create_proof_by_amount", args: [Decimal("1"), ResourceAddress("03f601495a1e3904c6d2a8db6dff4e1e9944160396fce7ab494758")] }
├─ PopFromAuthZone
├─ CallMethod { component_address: 022127235adc9ea993b0e99f40eb7ba6aef54a7f783cd6b775f02f, method: "withdraw_portfolio", args: [Proof(512u32)] }
└─ CallMethodWithAllResources { component_address: 02f5ad66df5f5dc26a67939b1d03ae3b5708f42a3466fcfa8cb130, method: "deposit_batch" }
Instruction Outputs:
├─ Proof(1024u32)
└─ Proof(512u32)
Logs: 15
├─ [INFO ] === WITHDRAW PORTFOLIO OPERATION START === 
├─ [INFO ]  Amount of funded tokens in the portfolio 0 
├─ [INFO ]  Amount of yours funded tokens in the portfolio 10000 
├─ [INFO ] Position size inside portfolio 0
├─ [INFO ] Current epoch 20 vs last epoch 20
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/03fd755fa368cb0b27571d485ab4f5aef45b395e53116a68b378b4 is 39 
├─ [INFO ] Current epoch 20 vs last epoch 20
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0345e6a8b141c210e7421387da83a62ca0fcb388b45f61448f3484 is 10 
├─ [INFO ] Current epoch 20 vs last epoch 20
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/03de10b1672c9a8ab30bd71f719dd3b6c94a771ed68941b72e5187 is 5 
├─ [INFO ] 0 tokens are valued xrd 0
├─ [INFO ] 0 tokens are valued xrd 0
├─ [INFO ] 0 tokens are valued xrd 0
├─ [INFO ]  Portfolio amount at time of funding 20000 and actual 20004.66666666666667 
└─ [ERROR] Panicked at 'attempt to divide by zero', /home/lbattagli/.cargo/registry/src/github.com-1ecc6299db9ec823/num-bigint-0.4.3/src/biguint/division.rs:168:9
New Entities: 0
Error: TransactionExecutionError(InvokeError)
lbattagli@DLT016:~/Software/Rust/radixdlt/scrypto-challenges/4-portfolio-management-yield-farming/PortfolioDApp$ resim set-default-account $account $priv1
Default account updated!
lbattagli@DLT016:~/Software/Rust/radixdlt/scrypto-challenges/4-portfolio-management-yield-farming/PortfolioDApp$ resim run transactions/withdraw_by_Bob.rtm 
Error: CompileError(GeneratorError(InvalidResourceAddress("0236df90ef193cf06840c15984927728408704ab793d63f104d5e0")))
lbattagli@DLT016:~/Software/Rust/radixdlt/scrypto-challenges/4-portfolio-management-yield-farming/PortfolioDApp$ resim run transactions/withdraw_by_Bob.rtm 
Transaction Status: InvokeError
Execution Time: 93 ms
Instructions:
├─ CallMethod { component_address: 0236df90ef193cf06840c15984927728408704ab793d63f104d5e0, method: "create_proof_by_amount", args: [Decimal("1"), ResourceAddress("03f601495a1e3904c6d2a8db6dff4e1e9944160396fce7ab494758")] }
├─ PopFromAuthZone
├─ CallMethod { component_address: 022127235adc9ea993b0e99f40eb7ba6aef54a7f783cd6b775f02f, method: "withdraw_portfolio", args: [Proof(512u32)] }
└─ CallMethodWithAllResources { component_address: 0236df90ef193cf06840c15984927728408704ab793d63f104d5e0, method: "deposit_batch" }
Instruction Outputs:
├─ Proof(1024u32)
└─ Proof(512u32)


# Integration Test

The portfolio_dapp.sh is a bash script that contains all the functions and methods tested, from the token creation to the component creation, from the fund to the withdraw methods, from the buy/sell methods to the lend/take back methods and it uses some user account to simulate some different events that could happen with these blueprints.


# Unit Test

Execute 'scrypto test' 

# TODO & Useful commands

//to update the package without resetting resim 
resim publish . --package-address $package

find *.rtm -exec sed -i 's/02e0905317d684478c275540e2ed7170f217e0c557805f7fd2a0d3/${account}/g' {} \;

echo $account
02e0905317d684478c275540e2ed7170f217e0c557805f7fd2a0d3

echo $account2
02b61acea4378e307342b2b684fc35acf0238a4accb9f91e8a4364

echo $account3
0200098f161a7691fa7ae380e41aed27ab5c4f969e8e563ce4275a

echo $priv1
35be322d094f9d154a8aba4733b8497f180353bd7ae7b0a15f90b586b549f28b

echo $priv2
f13ee6ed54ea2aae9fc49a9faeb5da6e8ddef0e12ed5d30d35a624ae813e0485

echo $priv3
aae89fc0f03e2959ae4d701a80cc3915918c950b159f6abb6c92c1433b1a8534

echo $portfolio
025d180a419b7d526eba63c0b971b875885c0618fc219843685549

echo $trading
0210e82c05fd5ea2c5413f8571a1f43df537a47b2d26613f36095e

echo $lending
02c887fe5316b2b9fcecfed965b308a67e207a5df67bbf17282f98

echo $lend_nft
0345a475f23e171428540acd6dfc2628229480614ddbb069cde5b0

echo $borrow_nft
0393dfaf83eff4942e65ac1b587ed989bfe6ea2adc432e6b99f972

echo $lnd
03629e07a727c9b17ed3b5984701ec846872bd09e6ba7d6aa3de85

echo $user_account_history_nft
03a3c7111213486713c0d5ad6ac43fc7cdbe5f6353df8247b8d94b

echo $user_account_funding_nft
032a450d815ecda8c1bfccd52e608a61ce8fec23a21892e2d1314b

echo $user_account_funding_nft2
032a450d815ecda8c1bfccd52e608a61ce8fec23a21892e2d1314b--errore

echo $user_account_funding_nft3
032a450d815ecda8c1bfccd52e608a61ce8fec23a21892e2d1314b--errore

echo $admin_badge
03d987113ce50a6077a4b4b5b9ef29e6798c20c79a1b1370d56893

echo $btc
039179c95de06571b3cec262e71854f9296f025f30d1688a3cae56

echo $eth
030d4d068757932f986bb98fb65166cd7e3c20b43e71cc775e687b

echo $leo
03b5c2df770cac9a330c3a535a7da82054d1ef1dba6f29302e2dee