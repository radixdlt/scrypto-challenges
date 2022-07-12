```shell
resim reset
result=$(resim new-account)
export admin=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export admin_priv=$(echo $result|grep "Account component address: "|awk -F "Private key: " '{print $2}')
result=$(resim new-account)
export p1=$(echo $result|grep "Account component address: "|awk -F ": " '{print $2}'|awk -F " " '{print $1}')
export p1_priv=$(echo $result|grep "Account component address: "|awk -F "Private key: " '{print $2}')

result=$(resim publish ".")
export pkg=$(echo $result | awk -F ": " '{print $2}')

result=$(resim call-function $pkg DefaultInterestModel "new")
export def_interest_model=$(echo $result | awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')

result=$(resim call-function $pkg StableInterestModel "new")
export stable_interest_model=$(echo $result | awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')

result=$(resim call-function $pkg LendingPool "instantiate_asset_pool" 0.25)
export component=$(echo $result | awk -F "Component: " '{print $2}' | awk -F " " '{print $1}')
export admin_badge=$(echo $result | awk -F "Resource: " '{print $2}' | awk -F " " '{print $1}')

export xrd=030000000000000000000000000000000000000000000000000004
resim run -t ./transactions/new_pool_def.rtm

resim set-default-account $p1 $p1_priv
resim call-method $component 'supply' 200,$xrd
```