resim reset 
export xrd="resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"

echo "Generating 5 accounts ================================"
OP1=$(resim new-account)
export PRIV_KEY=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export account=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

OP2=$(resim new-account)
export PRIV_KEY2=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export account2=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")


OP3=$(resim new-account)
export PRIV_KEY3=$(echo "$OP3" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY3=$(echo "$OP3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export account3=$(echo "$OP3" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")


OP4=$(resim new-account)
export PRIV_KEY4=$(echo "$OP4" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY4=$(echo "$OP4" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export account4=$(echo "$OP4" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")


OP5=$(resim new-account)
export PRIV_KEY5=$(echo "$OP5" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY5=$(echo "$OP5" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export account5=$(echo "$OP5" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

echo 
echo "Saved Environment Variables:"
echo "account  = $account"
echo "account2 = $account2"
echo "account3 = $account3"
echo "account4 = $account4"
echo "account5 = $account5"
echo 

echo "Publishing package ================================="
PKG=$(resim publish .) 
export package=$(echo "$PKG" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

echo 
echo "Saved Environment Variables:"
echo "package = $package"
echo 

echo "Instantiating Instapass Component =================="
OP6=$(resim call-function $package MockInstapass instantiate)
export instapass=$(echo "$OP6" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export veri_mint=$(echo "$OP6" | sed -nr "s/├─ Resource: ([[:alnum:]_]+)/\1/p")
export veri=$(echo "$OP6" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")

echo 
echo "Saved Environment Variables:"
echo "instapass = $instapass"
echo "veri_mint = $veri_mint"
echo "veri      = $veri" 
echo 

echo "Instantiating UselessBox Component =================="
OP7=$(resim call-function $package UselessBox instantiate)

resources_string=$(echo "$OP7" | sed -nr "s/├─ Resource: ([[:alnum:]_]+)/\1/p")
# resources_string="hey ho bitch"
IFS=$'\n'
arr=($resources_string)
IFS=' '

export useless_box=$(echo "$OP7" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export useless_box_mint_auth="${arr[0]}"
export useless_box_param_auth="${arr[1]}"
export useless_box_nft=$(echo "$OP7" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")

echo 
echo "Saved Environment Variables:"
echo "useless_box            = $useless_box"
echo "useless_box_mint_auth  = $useless_box_mint_auth"
echo "useless_box_param_auth = $useless_box_param_auth"
echo "useless_box_nft        = $useless_box_nft"
echo 

echo "Instantiating DAO Component =================="
OP8=$(resim call-function $package DAOComponent instantiate $useless_box 1,$useless_box_param_auth)
export dao=$(echo "$OP8" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export dao_admin=$(echo "$OP8" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")

echo 
echo "Saved Environment Variables:"
echo "dao       = $dao"
echo "dao_admin = $dao_admin"
echo 

echo "Adding function access to DAO Component =================="
resim call-method $dao add_external_function_control $useless_box "set_dummy_parameter" 1,$useless_box_param_auth
echo 

echo "change UselessBox's dummy_parameter through DAOComponent =================="
resim call-method $dao propose_parameter_change $useless_box "set_dummy_parameter" 100.0
echo 