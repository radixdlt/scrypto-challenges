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
#resim call-method ${component} supply $xrd:100
resim run rtm/supply_high.rtm

export resource_address=$demo1

echo '>>> Supply tokens (amount '$amount') of type demo1'
resim set-current-epoch 1
resim run rtm/supply_high.rtm

# 4000 xrd and 4000 demo1 supplied in
# 8000 tkn

export amount_reward='4'
echo '>>> Set Reward '$amount_reward' at epoch 100'
resim set-current-epoch 100
resim run rtm/set_reward.rtm

export amount_reward='8'
echo '>>> Set Reward '$amount_reward' at epoch 1000'
resim set-current-epoch 1000
resim run rtm/set_reward.rtm

export amount_reward='12'
echo '>>> Set Reward '$amount_reward' at epoch 2500'
resim set-current-epoch 2500
resim run rtm/set_reward.rtm

export amount='2000'
export length='4000'
echo '>>> Tokenize 1 @5000 (amount ' $amount ') for ' $length 'epoch'
resim set-current-epoch 5000
resim run rtm/tokenize_yield.rtm

resim show $account
export resource_address=$xrd
echo '>>> Tokenize 2 @5100 (amount ' $amount ') for ' $length 'epoch'
resim set-current-epoch 5100
resim run rtm/tokenize_yield.rtm

resim show $account

# 4000 xrd and 4000 demo1 supplied in
# 8000 tkn
# 2000 xrd and 2000 demo1 tokenized

# echo '>>> Redeem'

# resim set-current-epoch 200
# #resim call-method ${component} tokenize_yield $xrd:100
# resim run rtm/redeem.rtm

# resim show $account

export amount_reward='6'
echo '>>> Set Reward '$amount_reward' at epoch 6000'
resim set-current-epoch 6000
resim run rtm/set_reward.rtm

export amount_reward='3'
echo '>>> Set Reward '$amount_reward' at epoch 7000'
resim set-current-epoch 7000
resim run rtm/set_reward.rtm

export amount='50'
echo '>>> redeem_from_pt (After Maturity) (amount '$amount')'
resim set-current-epoch 9500
#resim call-method ${component} tokenize_yield $xrd:100
resim run rtm/redeem_from_pt.rtm

echo '>>> Claim Yield (After Maturity)'
resim set-current-epoch 10000
resim run rtm/claim_yield.rtm

echo '>>> Claim Yield (After Maturity2) '
resim set-current-epoch 11000
resim run rtm/claim_yield.rtm

# resim show $account

export amount='100'
echo '>>> redeem_from_pt, trying again (After Maturity 2) (amount '$amount') of type ' $resource_address
resim set-current-epoch 9500
resim run rtm/redeem_from_pt.rtm

export amount='3800'
echo '>>> redeem_from_pt, trying again (After Maturity 3) (amount '$amount') of type ' $resource_address
resim set-current-epoch 9500
resim run rtm/redeem_from_pt.rtm

resim show $account

# 4000 xrd and 4000 demo1 supplied in
# 8000 tkn -> 4000 xrd reedemed -> 4000 TKN in wallet
# 2000 xrd and 2000 demo1 tokenized


export amount='3000'
echo '>>> Takes Back (amount '$amount') of type '$resource_address
resim run rtm/takes_back.rtm

export resource_address=$xrd
export amount='1500'
echo '>>> Takes Back (amount '$amount') of type ' $resource_address
resim run rtm/takes_back.rtm

# 1500 xrd takes back -> 1500 TKN removed from wallet and 3000 xrd in the wallet

# 3000 demo1 takes back -> 3000 TKN removed from wallet and 3000 demo1 in the wallet (?)

# TKN Vault should contain about 3500 token (2500 from xrd supply, and 1000 from demo1 supply)

resim show $account

