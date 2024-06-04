clear 
set -e

export xrd=resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3

echo "Resetting environment"
resim reset
export owner_account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo "Owner Account = " $owner_account
echo "XRD = " $xrd

echo "Creating tokens"
export demo1=$(resim new-token-fixed --name "DEMO1" 100000 --symbol "DEMO1" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
export demo2=$(resim new-token-fixed --name "DEMO2" 100000 --symbol "DEMO2" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")

echo "Publishing dapp"
export dapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $dapp_package

echo "Instantiate dapp"
output=`resim call-function $dapp_package Tokenizer instantiate 5 TKN timebased $xrd $demo1 | awk '/Component: |Resource: / {print $NF}'`
export component=`echo $output | cut -d " " -f1`
export owner_badge=`echo $output | cut -d " " -f2`
export admin_badge=`echo $output | cut -d " " -f3`
export tokenizer_token=`echo $output | cut -d " " -f4`
export userdata_nft_manager=`echo $output | cut -d " " -f5`
export pt=`echo $output | cut -d " " -f6`
export yt=`echo $output | cut -d " " -f7`

echo "Export component test"
export component_test=component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh

echo "Instantiate output"
echo 'output = '$output

echo 'component = '$component
echo 'owner_badge = '$owner_badge
echo 'admin_badge = '$admin_badge
echo 'tokenizer_token = ' $tokenizer_token
echo 'userdata_nft_manager = ' $userdata_nft_manager
echo 'pt = ' $pt
echo 'yt = ' $yt

echo ' '
echo 'account = ' $owner_account
echo 'xrd = ' $xrd
echo 'test faucet for lock fee = ' $component_test
echo ' '

resim show $owner_account

echo ' > owner'
resim show $owner_badge
echo ' > admin'
resim show $admin_badge
echo ' > userdata_nft_manager'
resim show $userdata_nft_manager
echo ' > zero tokenizer_token'
resim show $tokenizer_token
echo ' > pt'
resim show $pt
echo ' > yt'
resim show $yt

echo '>>> Extend Lending Pool High'
export amount='5000'
resim run rtm/extend_lending_pool.rtm

export fund='1000'
echo '>>> Fund Main Vault'
resim run rtm/fund.rtm

export account=$owner_account
echo '>>> Register'
resim run rtm/register.rtm

export resource_address=resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3

export amount='4000'
echo '>>> Supply tokens (amount '$amount') of type xrd'
resim set-current-epoch 1
resim run rtm/supply_high.rtm

# 4000 xrd supplied in

export amount_reward='4'
echo '>>> Set Reward '$amount_reward' at epoch 100'
resim set-current-epoch 100
resim run rtm/set_reward.rtm

export amount_reward='8'
echo '>>> Set Reward '$amount_reward' at epoch 1000'
resim set-current-epoch 1000
resim run rtm/set_reward.rtm

export amount_reward='12'
echo '>>> Set Reward '$amount_reward' at epoch 1000'
resim set-current-epoch 2500
resim run rtm/set_reward.rtm
resim run rtm/set_extra.rtm

export amount='2000'
export length='4000'
echo '>>> Tokenize 1 @5000 (amount '$amount') for '$length' epoch at extra_reward ' $amount_reward
resim set-current-epoch 5000
resim run rtm/tokenize_yield.rtm

resim show $account

# 2000 xrd tokenized and the interest rate drops from 12% to 6% and then 3%

export amount_reward='6'
echo '>>> Set Reward '$amount_reward' at epoch 6000'
resim set-current-epoch 6000
resim run rtm/set_extra.rtm

export amount_reward='3'
echo '>>> Set Reward '$amount_reward' at epoch 7000'
resim set-current-epoch 7000
resim run rtm/set_extra.rtm

export amount='2000'
echo '>>> Redeem amount ' $amount
resim run rtm/redeem.rtm

# 2000 xrd swapped
echo 'Result after swapping after the interest rate drops'
resim show $account

# tokenize again 2000 xrd 

export amount='2000'
export length='4000'
echo '>>> Tokenize 2 @5000 (amount '$amount') for '$length' epoch  at extra_reward ' $amount_reward
resim set-current-epoch 5000
resim run rtm/tokenize_yield.rtm

export amount_reward='15'
echo '>>> Set Reward '$amount_reward' at epoch 7000'
resim set-current-epoch 7000
resim run rtm/set_extra.rtm

# 2000 xrd tokenized and the interest rate rises from 3% to 15%

export amount='2000'
echo '>>> Redeem amount ' $amount
resim run rtm/redeem.rtm

# 2000 xrd swapped
echo 'Result after swapping after the interest rate rise'
resim show $account
