

#set -x
set -e

export xrd=030000000000000000000000000000000000000000000000000004

echo "Resetting environment"
resim reset

OP1=$(resim new-account)
export PRIV_KEY1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP2=$(resim new-account)
export PRIV_KEY2=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS2=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP3=$(resim new-account)
export PRIV_KEY3=$(echo "$OP3" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY3=$(echo "$OP3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS3=$(echo "$OP3" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP4=$(resim new-account)
export PRIV_KEY4=$(echo "$OP4" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY4=$(echo "$OP4" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS4=$(echo "$OP4" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

export account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo "Account = " $account
echo "XRD = " $xrd

echo "Publishing dapp"
export lendingapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $lendingapp_package

output=`resim call-function $lendingapp_package LendingApp instantiate_pool 1000,$xrd 1000 10 7 | awk '/Component: |Resource: / {print $NF}'`
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


resim set-default-account $ACC_ADDRESS1 $PRIV_KEY1
resim call-method $component register
resim call-method $component register_borrower

echo '====== ACCOUNT 1 ======'

for i in web{0..5};
do 
    resim call-method $component lend_money 100,$xrd 1,$lend_nft;

    resim call-method $component take_money_back 107,$lnd 1,$lend_nft;

    resim call-method $component borrow_money 100  1,$borrow_nft;

    resim call-method $component repay_money 110,$xrd  1,$borrow_nft;
done;

resim show $account
resim show $component


echo '====== ACCOUNT 2 ======'
resim set-default-account $ACC_ADDRESS2 $PRIV_KEY2
resim call-method $component register
resim call-method $component register_borrower
for i in web{0..5};
do 
    resim call-method $component lend_money 100,$xrd 1,$lend_nft;
    resim call-method $component take_money_back 107,$lnd 1,$lend_nft;
    resim call-method $component borrow_money 100  1,$borrow_nft;
    resim call-method $component repay_money 110,$xrd  1,$borrow_nft;
done;


resim show $account
resim show $component


echo '====== ACCOUNT 3 ======'
resim set-default-account $ACC_ADDRESS3 $PRIV_KEY3
resim call-method $component register
resim call-method $component register_borrower
for i in web{0..5};
do 
    resim call-method $component lend_money 100,$xrd 1,$lend_nft;
    resim call-method $component take_money_back 107,$lnd 1,$lend_nft;
    resim call-method $component borrow_money 100  1,$borrow_nft;
    resim call-method $component repay_money 110,$xrd  1,$borrow_nft;
done;

resim show $account
resim show $component


echo '====== ACCOUNT 4 ======'
resim set-default-account $ACC_ADDRESS4 $PRIV_KEY4
resim call-method $component register
resim call-method $component register_borrower
for i in web{0..5};
do 
    resim call-method $component lend_money 100,$xrd 1,$lend_nft;
    resim call-method $component take_money_back 107,$lnd 1,$lend_nft;
    resim call-method $component borrow_money 100  1,$borrow_nft;
    resim call-method $component repay_money 110,$xrd  1,$borrow_nft;
done;


resim show $account
resim show $component
