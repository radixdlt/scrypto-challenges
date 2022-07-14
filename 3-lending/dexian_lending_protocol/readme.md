
## work flow

![workflow](res/biz_flow.jpg)



```shell
scrypto build
resim reset
result=$(resim new-account)
export admin=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export admin_priv=$(echo $result|grep "Private key:" |awk -F "Private key: " '{print $2}')
result=$(resim new-account)
export p1=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export p1_priv=$(echo $result|grep "Private key:" |awk -F "Private key: " '{print $2}')
result=$(resim new-account)
export p2=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export p2_priv=$(echo $result|grep "Private key:" |awk -F "Private key: " '{print $2}')
result=$(resim new-account)
export p3=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export p3_priv=$(echo $result|grep "Private key:" |awk -F "Private key: " '{print $2}')

result=$(resim new-token-fixed --symbol=USDT 1000000)
export usdt=$(echo $result | grep "Resource:" | awk -F " " '{print $3}')
result=$(resim new-token-fixed --symbol=USDC 1000000)
export usdc=$(echo $result | grep "Resource:" | awk -F " " '{print $3}')

resim transfer 1000000 $usdt $p2
resim transfer 1000000 $usdc $p3


result=$(resim publish ".")
export pkg=$(echo $result | awk -F ": " '{print $2}')

result=$(resim call-function $pkg DefaultInterestModel "new")
export def_interest_model=$(echo $result | grep "Component: "| awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')

result=$(resim call-function $pkg StableInterestModel "new")
export stable_interest_model=$(echo $result | grep "Component: "| awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')

result=$(resim call-function $pkg PriceOracle "new" $usdt $usdc)
export oracle=$(echo $result | grep "Component: "| awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')

result=$(resim call-function $pkg LendingPool "instantiate_asset_pool" $oracle)
export component=$(echo $result | grep "Component: "| awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')
export admin_badge=$(echo $result | grep "Resource: " | awk -F "Resource: " '{if (NR==1) print $2}')

export xrd=030000000000000000000000000000000000000000000000000004
resim run -t ./transactions/new_pool_def.rtm
resim run -t ./transactions/new_usdt_stable.rtm
resim run -t ./transactions/new_usdc_stable.rtm


# export xd_xrd=033fac8766b9e37307051eecbde33f8ba980123aabf210b9ca1930

# xrd
resim set-default-account $p1 $p1_priv
resim call-method $component 'supply' 20000,$xrd

# usdt
resim set-default-account $p2 $p2_priv
resim call-method $component 'supply' 360,$usdt

# usdc
resim set-default-account $p3 $p3_priv
resim call-method $component 'supply' 360,$usdc


# p1(xrd) borrow 180 usdt
resim set-default-account $p1 $p1_priv
resim call-method $component 'borrow' 10000,$supply_xrd $usdt 180
resim call-method $component 'borrow' 10000,$supply_xrd $usdc 180

```


## Asset Risk Parameter
|  Symbol  |  Collateral  |  Loan To Value  |  Liquidation Threshold   |  Liquidation Bonus   | Insurance Ratio | Interest Model          |
| -------- | ------------ | --------------- | ------------------------ | -------------------- | --------------- | ----------------------- |
| XRD      | Yes          | 60%             | 70%                      |  7%                  |  25%            | Default Interest model  |
| USDT     | No           |                 |                          |                      |  10%            | Stable Interest model   | 
| USDC     | Yes          | 85%             | 87%                      |  2%                  |  10%            | Stable Interest model   |
