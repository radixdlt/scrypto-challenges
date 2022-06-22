export xrd=030000000000000000000000000000000000000000000000000004

echo "Reseting environment"
resim reset
export account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

echo "Setting up gumball machine"
cd ../2_gumball_machine
export gumball_machine_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
PACKAGE_RESULT1=$(resim call-function $gumball_machine_package GumballMachine instantiate_machine 25)
export gumball_machine=$(echo $PACKAGE_RESULT1 | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export gumball=$(echo $PACKAGE_RESULT1 | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
resim call-method $gumball_machine buy_gumball 25,$xrd
resim call-method $gumball_machine buy_gumball 25,$xrd
resim call-method $gumball_machine buy_gumball 25,$xrd

echo "Setting up RadiSwap"
cd ../5_decentralized_exchange

echo "Creating BTC token"
NEW_TOKEN=$(resim new-token-fixed --name BitCoin --symbol BTC 21000000)
export btc=$(echo "$NEW_TOKEN" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")

export radiswap_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
PACKAGE_RESULT=$(resim call-function $radiswap_package Radiswap instantiate_pool 100,$btc 3,$gumball 100 0.01)
export radiswap=$(echo $PACKAGE_RESULT | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export lp_token=$(echo $PACKAGE_RESULT | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")