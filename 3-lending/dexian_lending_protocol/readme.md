```shell
resim reset
result=$(resim new-account)
export admin=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export admin_priv=$(echo $result|grep "Private key:" |awk -F "Private key: " '{print $2}')
result=$(resim new-account)
export p1=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export p1_priv=$(echo $result|grep "Private key:" |awk -F "Private key: " '{print $2}')

result=$(resim publish ".")
export pkg=$(echo $result | awk -F ": " '{print $2}')

result=$(resim call-function $pkg DefaultInterestModel "new")
export def_interest_model=$(echo $result | grep "Component: "| awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')

result=$(resim call-function $pkg StableInterestModel "new")
export stable_interest_model=$(echo $result | grep "Component: "| awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')

result=$(resim call-function $pkg LendingPool "instantiate_asset_pool" 0.25)
export component=$(echo $result | grep "Component: "| awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')
export admin_badge=$(echo $result | grep "Resource: " | awk -F "Resource: " '{if (NR==1) print $2}')

export xrd=030000000000000000000000000000000000000000000000000004
resim run -t ./transactions/new_pool_def.rtm

# export xd_xrd=033fac8766b9e37307051eecbde33f8ba980123aabf210b9ca1930

resim set-default-account $p1 $p1_priv
resim call-method $component 'supply' 200,$xrd
```


## Asset Risk Parameter
|  Symbol  |  Collateral  |  Loan To Value  |  Liquidation Threshold   |  Liquidation Bonus   | Insurance Ratio |
| -------- | ------------ | --------------- | ------------------------ | -------------------- | --------------- |
| XRD      | Yes          | 60%             | 70%                      |  7%                  |  25%            |
| USDT     | No           |                 |                          |                      |  10%            |
| USDC     | Yes          | 85%             | 87%                      |  2%                  |  10%            |