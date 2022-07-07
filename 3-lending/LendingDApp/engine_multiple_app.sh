set -e

echo "Resetting environment"
resim reset
export xrd=030000000000000000000000000000000000000000000000000004

OP1=$(resim new-account)
export priv_key=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

echo "XRD = " $xrd
resim set-default-account $account $priv_key

export unknown=$(resim new-token-fixed 2000 --description "The unknown token" --name "Unknown" --symbol "UKN" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
echo "Create new token with supply of 2000 = " $unknown

resim transfer 1 $unknown $account

echo "Publishing dapp"
export lendingapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $lendingapp_package

output=`resim call-function $lendingapp_package LendingEngine instantiate_pool| awk '/Component: |Resource: / {print $NF}'`
export componentEngine=`echo $output | cut -d " " -f1`
export ADMIN_BADGE=`echo $output | cut -d " " -f2`
export lend_nft=`echo $output | cut -d " " -f3`
export borrow_nft=`echo $output | cut -d " " -f4`
export lnd=`echo $output | cut -d " " -f5`

echo 'COMPONENT-ENGINE = '$componentEngine
echo 'ADMIN_BADGE = '$ADMIN_BADGE
echo 'LEND_NFT = '$lend_nft
echo 'BORROW_NFT = '$borrow_nft
echo 'LND = ' $lnd

echo ' ======================= ACCOUNT '
resim show $account

echo ' ======================= ENGINE'
resim show $componentEngine

echo ' ======================= Creating new Loan Pool with token ' $xrd
app_loan=`resim call-method $componentEngine new_loan_pool 1000,$xrd 1000 10 7 | awk '/Component: |Resource: / {print $NF}'`
echo 'TODO here you need to parse again component and resources'
export app=`echo $app_loan | cut -d " " -f1`
export ADMIN_BADGE=`echo $app_loan | cut -d " " -f2`
export lend_nft_app=`echo $app_loan | cut -d " " -f3`
export borrow_nft_app=`echo $app_loan | cut -d " " -f4`
export lnd_app=`echo $app_loan | cut -d " " -f5`

echo ' ======================= Creating new Loan Pool with token ' $unknown
app_loan2=`resim call-method $componentEngine new_loan_pool 1000,$unknown 1000 10 7 | awk '/Component: |Resource: / {print $NF}'`
echo 'TODO here you need to parse again component and resources'
export app_unknown=`echo $app_loan2 | cut -d " " -f1`
export ADMIN_BADGE_unknown=`echo $app_loan2 | cut -d " " -f2`
export lend_nft_app_unknown=`echo $app_loan2 | cut -d " " -f3`
export borrow_nft_app_unknown=`echo $app_loan2 | cut -d " " -f4`
export lnd_app_unknown=`echo $app_loan2 | cut -d " " -f5`

echo ' ======================= App ' 
resim show $app
echo ' ======================= Lend NFT ' 
resim show $lend_nft_app
echo ' ======================= Lend Token ' 
resim show $lnd_app
echo ' ======================= App_unknown ' 
resim show $app_unknown
echo ' ======================= Lend NFT_unknown ' 
resim show $lend_nft_app_unknown
echo ' ======================= Lend Token_unknown ' 
resim show $lnd_app_unknown

echo ' ======================= Registering for lending '

resim call-method $app register $xrd
resim call-method $app_unknown register $unknown

echo ' ======================= Account should show two LendNFT = ' 
resim show $account

echo ' ======================= Lending from App' 
    resim call-method $app lend_money 100,$xrd 1,$lend_nft_app;

echo ' ======================= Lending from App Unknown' 
    resim call-method $app_unknown lend_money 100,$unknown 1,$lend_nft_app_unknown;

echo ' ======================= ACCOUNT '
resim show $account

echo ' ======================= ENGINE'
resim show $componentEngine

echo ' ======================= APP with XRD '
resim show $app

echo ' ======================= APP with Unknown '
resim show $app_unknown

echo '====== Show Pools ========='
resim call-method $componentEngine show_pools

echo '====== New Pool forbidden since it already exist  ========='
resim call-method $componentEngine new_loan_pool 100,$unknown 1000 8 7