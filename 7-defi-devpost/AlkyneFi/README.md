# AlkyneFi

- Detailed "How to run on local" guide at "scrypto-challenges/7-defi-devpost/AlkyneFi/scryptoDemo/README.md".

- To run the frontend, run this command on the root of Frontend `yarn && yarn run dev`

# The WHAT ?
The protocol is defined simply by 
*Invest through our Joint Contract and earn double the profit you were supposed to earn individually.*
We achieve this by using *Equally Collateralised Trade*.
let's see how 👇

## The HOW ?
Let's better understand this with a 5 simple step example, 
Lisa is a trader who is looking for a $100 investment.
1 ) She creates a joint account on our website and locks in $100.
2 ) The protocol backs her collateral by equivalent amount summing it to be $200.
3 ) Lisa governs the investment onto any of the given trade options.
4 ) Upon profitable investment Lisa withdraws the total amount.
5 ) Lisa receives  *2x* returns than what show could have if she were an individual investor.

Maths for Nerds :
Principal =  $200  -> $100 (Lisa) + $100 (Protocol) 
Return percentage = 40%
Return Amount = $40 (Lisa's investment) + $40 (Protocol's investment)
Total Protocol fees   = $1.6 (Protocol fees )

Total Return Amount Lisa Receives = *$178.4* 
($100 Principal + $80 profit - $1.6 [2% Protocol fees])
Total Return Amount Lisa would have received if there were no AlkyneFi = *$140* 
($100 Principal + $40 profit)

AlkyneFi helped Lisa Double her returns = *78.4 / 40 ~= 2x return*

# AlkyneFi

## Steps to test app

0. Deploy the radiswap app from <https://github.com/devmrfitz/scrypto-examples/tree/main/defi/radiswap>. Save pool address in $radiswapPool1. Put any 2 token addresses in $xrd and $secondToken. Put your account address in $account.
1. Publish AlkyneFi using `resim publish .`. Save the package address in $package and the owner badge obtained in $tradeXOwnerBadge.
2. `resim run rtm/instantiate.rtm`. Save the component address in $component.
3. `resim run rtm/fund_vault.rtm`
4. `resim run rtm/add_standard_radiswap_pool.rtm`
5. `resim run rtm/add_approved_pool.rtm`

### AlkyneFi is deployed now. Let's see how a user can create account, deposit funds and perform trades

6. `resim run rtm/trader/create_and_fund_wallet.rtm`
7. `resim run rtm/trader/fund_existing_wallet.rtm` (if needed)
8. `resim run rtm/trader/trade.rtm`
9. `resim run rtm/trader/withdraw_payment.rtm`

At any point `resim run rtm/trader/show_lending_balance.rtm` and `resim run rtm/trader/check_wallets.rtm` can be used to monitor the balance.

A regular cronjob of `resim run rtm/poll_all_traders_health.rtm` ensures noone's investment goes below threshold.

## Deployed addresses on testnet

```
 account=account_tdx_b_1pr253944sttq4axp958u8xxy6m52j9yr5nd8y7nju3csyxl7y5
 package=package_tdx_b_1q9xsqncvnxkd0vtqu7j8xvm8sprwdl9xzzzwl0kglvgszhfknm


 component=component_tdx_b_1q2h4tzz6gap02vfne5q7xa7h2g0ak62lulgxlv4kd0nsncakph
 tradeXOwnerBadge=resource_tdx_b_1qzh4tzz6gap02vfne5q7xa7h2g0ak62lulgxlv4kd0nsee0cyv

 xrd=resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp
 secondToken=resource_tdx_b_1qzdqwhlkmsxlw5qmpdz95rv7ve74e48v44ha4f53puzq5wv38h
 radiswapPool1=component_tdx_b_1qfkudyf9uwxs9fd7j37qla0k99zug7q5ryxkrlglc6usjjr8z2
 radiSwapPackage=package_tdx_b_1q8ga7qqs2kgyg9hqd6upr72zexhzy522zwrtc85lfs0scdjwdh
```
