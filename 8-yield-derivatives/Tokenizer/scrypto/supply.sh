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
echo '>>> Supply tokens (amount 4000) of type xrd'
resim set-current-epoch 1
resim run rtm/supply_high.rtm
# 4000 xrd supplied in

echo '>>> Add Token 3'
export token=$demo2
resim run rtm/add_token.rtm

echo '>>> Supply 4000 Tokens of a different type'
export resource_address=$token
export amount='4000'
resim set-current-epoch 50
resim run rtm/supply_high.rtm


echo '>>> Tokenize 2000 tokens for 4000 epoch , type = '  $token
export amount='2000'
export length='4000'
resim set-current-epoch 5000
resim run rtm/tokenize_yield.rtm


resim set-current-epoch 100

echo '>>> Withdraw (amount 2000) of type ' $token
export amount='2000'
resim run rtm/takes_back.rtm

echo '>>> Have a look at the different tokens in the account '
resim show $account



