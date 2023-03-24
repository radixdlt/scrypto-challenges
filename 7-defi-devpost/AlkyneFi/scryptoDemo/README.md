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
export radiSwapPackage=package_tdx_b_1q8ga7qqs2kgyg9hqd6upr72zexhzy522zwrtc85lfs0scdjwdh
```
