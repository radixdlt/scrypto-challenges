

#set -x
set -e

echo "Resetting environment"
resim reset
export xrd=030000000000000000000000000000000000000000000000000004

#export account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p" )
#echo "Account = " $account
OP1=$(resim new-account)
export priv_key=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

echo "XRD = " $xrd
resim set-default-account $account $priv_key

export unknown=$(resim new-token-fixed 10 --description "The unknown token" --name "Unknown" --symbol "UKN" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
echo "unknown = " $unknown

resim transfer 1 $unknown $account

echo "Publishing dapp"
export lendingapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $lendingapp_package

output=`resim call-function $lendingapp_package LendingEngine instantiate_pool 1000,$xrd 1000 10 7 | awk '/Component: |Resource: / {print $NF}'`
export component=`echo $output | cut -d " " -f1`
export ADMIN_BADGE=`echo $output | cut -d " " -f2`
export lend_nft=`echo $output | cut -d " " -f3`
export borrow_nft=`echo $output | cut -d " " -f4`
export lnd=`echo $output | cut -d " " -f5`

echo 'COMPONENT = '$component
echo 'ADMIN_BADGE = '$ADMIN_BADGE
echo 'LEND_NFT = '$lend_nft
echo 'BORROW_NFT = '$borrow_nft
echo 'LND = ' $lnd

resim show $account

resim call-method $component register 0,$unknown
