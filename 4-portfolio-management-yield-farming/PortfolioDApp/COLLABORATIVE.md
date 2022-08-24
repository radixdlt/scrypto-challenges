# Portfolio dApp

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
- Swap application [Not implemented]

# Design

Blueprint can create new components with only a single vault and a map containing all the info about the operation opened/in place/closed.

Component has some very simple method for depositing/taking from the main vault:
- deposit(bucket) -> tokens are put in the main vault -> account receives an nft (transferable) who states the amount of tokens deposited
- take(proof, amount) -> a bucket gets created with tokens from the main vault and sent back if the account has presented a valid proof 

And some others for executing orders/operations:
- buy(proof, amount, resource_address) -> a buy order is issued using the 'trading blueprint' for the amount specified and the resource address
- sell(proof, amount, resource_address) -> a sell order is issued using the 'trading blueprint' for the amount specified and the resource address
- register() -> a register is asked to the LendingdApp component
- lend(proof, bucket) -> a bucket has lent to the LendingdApp component 
- take_back(proof, amount) -> a bucket is created and sent back to the account

And also some for closing orders/operations:
- close_operation(proof) -> account that has opened the operation can close it anytime
- list_open_operation() -> list of open operation and its account creator 
- close_someone_else_operation(operation_id) -> close an operation opened by someone else, available only if the operation is losing

The following methods should update the soulbound token of the account that has created the operation:
- sell      -> should update in the sbt the number of positive operation if the result has been positive, otherwise no
- take_back -> should update in the sbt the number of positive operation if the result has been positive, otherwise no

The following methods should update the main map containing the info about all the operations:
- buy       -> should insert in the map the new operation with operation_id, amount, date_opened
- sell      -> should update in the map the closed operation with date_closed
- lend      -> should insert in the map the new operation with operation_id, amount, date_opened
- take_back -> should update in the map the closed operation with date_closed
- close_operation -> should find all the opening and close everything
- close_someone_else_operation -> 

The data about the operation contains the following:
- operation_id: id created random
- amount: size of the operation
- date_opened: epoch when it has been opened
- date_closed: epoch when it has been closed
- current_standing: actual result (profitable/losing position)
- number_of_request_for_autoclosing: number or request needed for the operation to be closed even if creator does not agree
- [current_requestor_for_closing]: account requesting its closing
 

# Portfolio dApp 

Let's proceed with a demo of the blueprints, start publishing the package

```
resim publish .
export package=
```

Then create the first account, name it Bob (the Admin)

```
resim new-account
A new account has been created!
Account component address: 021025cfda90adea21506170be47c67ec169e41dbbdd063d54d409
Public key: 0426599006343468593571484e418be9c40db4ffaf60ff9d98e6ccb13aec950ce2b70cd20e293909a6f1c380ab2cbad6c527c552c6aa7b7a664722df08fda26b8f
Private key: d86e8112f3c4c4442126f8e9f44f16867da487f29052bf91b810457db34209a4
export account=
export priv1=
```

Then create the second account, name it John

```
resim new-account
A new account has been created!
Account component address: 02d0da3fc806e20c508841efdcd412a53e50d1b80fb35ff1263214
Public key: 042f3ce83809c2c67057ff9aba2a95127e729b5439993051cc168a2939f655c904e976cf6db5cc51106dcd83b4b24d75c5e4e6f07c948b2cbca6eaef82bdc81832
Private key: f0a0278e4372459cca6159cd5e71cfee638302a7b9ca9b05c34181ac0a65ac5d
export account2=
export priv2=
```

Then create the third account, name it Max

```
resim new-account
A new account has been created!
Account component address: 02bf3aa95784d95a63dd6f8e3f0d06de6127e114cc275a13ae47b5
Public key: 04c1b4e1a0f1290b46b1836c4c4a9e6c7c963eb9b71e91bc0c3b32a99f79081634aa9719b7f8e5019bb918ace34f29d2ed66449eaf1c43deb9993642add0417b5a
Private key: 205df2fd636e9a2b6e81c3987fa3dcdd09d64c5c710dd61aaa50a97d222a3f74
export account3=02bf3aa95784d95a63dd6f8e3f0d06de6127e114cc275a13ae47b5
export priv3=205df2fd636e9a2b6e81c3987fa3dcdd09d64c5c710dd61aaa50a97d222a3f74
```

Then create the tokens to be used in the trading dapp

```
resim new-token-fixed --name bitcoin --symbol btc 10000
resim new-token-fixed --name ethereum --symbol eth 1000
resim new-token-fixed --name leonets --symbol leo 100
export xrd=030000000000000000000000000000000000000000000000000004
```

Then create the TradingApp component, it needs the resource address of 4 tokens and later on it needs to have its vaults funded

```
resim call-function $package TradingApp create_market $xrd $btc $eth $leo
export trading=
```

The TradingApp component has been created so now Bob needs to fund its vaults

```
resim call-method $component fund_market 1000,$xrd 1000,$btc 1000,$eth 100,$leo
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
export lending='component address'
export lend_nft='second resource address'
export lnd='fourth resource address'
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
resim call-function $package Portfolio new $xrd $btc $lending $trading $lend_nft
```

The call-function outputs the address of the component/resources created
```
New Entities: 4
└─ Component: 02e66d3340f5e9c272ff2592e8e8e8b05376af3e35fd2e3f3ce30d
├─ Resource: 03786df5a8851edaf8bf0ef318104b9c41f895c946fb080ca7c9dd
├─ Resource: 03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9
└─ Resource: 03cacd11c325cd75f7693ed8d99187f65ec303bdc1a0622cca283f
```

Let's export the variables we'll need later

```
$ export portfolio=02e66d3340f5e9c272ff2592e8e8e8b05376af3e35fd2e3f3ce30d
$ export ADMIN_BADGE=03786df5a8851edaf8bf0ef318104b9c41f895c946fb080ca7c9dd
$ export user_account_history_nft=03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9
$ export user_account_funding_nft=03cacd11c325cd75f7693ed8d99187f65ec303bdc1a0622cca283f
```

The following operation we need to execute is the register, we have two different types of registering, one for the user account to operate on the PortfolioApp and the others for the PortfolioApp itself with the LendingApp component

```
resim call-method $portfolio register $account
```

After the user account has been registered itself with the PortfolioApp component we can see the NFT that has been added to its resource's list 
```
├─ { amount: 1, resource address: 03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9, name: "User Account Trading History" }
│  └─ NonFungible { id: 0bfa93aa9159a62422fd0868d0ae4a16e32eff89f39d206bb5eb8267f265c424, immutable_data: Struct(), mutable_data: Struct(ComponentAddress("021025cfda90adea21506170be47c67ec169e41dbbdd063d54d409"), 0u32, 0u32, false) }
```

Let's register with the component

```
resim call-method $portfolio register_for_lending (resim run transactions/register_for_lending.rtm)
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
resim call-method $trading buy 500,$xrd (resim call-method $trading buy 500,$xrd  --manifest transactions/buy_with_trading.rtm)
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
resim call-method $trading sell 12.5,$btc  (resim call-method $trading sell 12.5,$btc   --manifest transactions/sell_with_trading.rtm)

├─ [INFO ] Current epoch 2 vs last epoch 2
├─ [INFO ] Current price of 030000000000000000000000000000000000000000000000000004/0396c203d001f1fa99fdf081dc2f30e7f3b921eb1b5c9cc9487630 is 36 
└─ [INFO ] N. xrd to receive: 450
```

So Bob got 10% less from its trade operation.

Now let's instead what could happen if Bob uses the PortfolioDapp

In this example Bob, as all the other users, has to fund directly inside the PortfolioApp component before starting to operate

```
resim call-method $portfolio fund_portfolio 10000,$xrd 1,$user_account_history_nft (resim call-method $portfolio fund_portfolio 10000,$xrd 1,$user_account_history_nft --manifest transactions/fund_portfolio_by_Bob.rtm)
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

resim call-method $portfolio register $account2 (resim call-method $portfolio register $account2 --manifest transactions/register_with_portfolio_by_John.rtm)
```

Also John gets its NFT

```
├─ { amount: 1, resource address: 03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9, name: "User Account Trading History" }
│  └─ NonFungible { id: 63d6d06bb0ba110877aff6def72a699ae852f4f7f14c546ef11f8c69638f47d7, immutable_data: Struct(), mutable_data: Struct(ComponentAddress("02d0da3fc806e20c508841efdcd412a53e50d1b80fb35ff1263214"), 0u32, 0u32, false) }

export user_account_history2=03113e60dbfe0fa744ca9fbecc2441ec230aca977f68bcc102bcb9
```


And then he can fund the Portfolio

```
resim call-method $portfolio fund_portfolio 10000,$xrd 1,$user_account_history2
```

The same has been done with Max's account

```
resim call-method $portfolio register $account3 (resim call-method $portfolio register $account3 --manifest transactions/register_with_portfolio_by_Max.rtm)

resim call-method $portfolio fund_portfolio 10000,$xrd 1,$user_account_history3
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

The portfolio now contains again only xrd tokens and Bob and Max are obviously allowed to withdraw.

```
├─ { amount: 20666.666666666666669915, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

No trading operation are open now, so for example Max decides to lend some of the current liquidity the get some reward

resim call-method $portfolio lend 100,$xrd 

# Test unitari

Eseguire 'scrypto test' 

# TODO

Su ogni account vengono registrati erroneamente 2 User Account Trading History NFT

test di tutti i token

//aggiorno il package
resim publish . --package-address $package

