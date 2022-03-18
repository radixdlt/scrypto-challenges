#!/usr/bin/env bash

# Getting the directory of this bash script
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
echo "Operating out of: $SCRIPT_DIR"

# Resetting the local script simulator 
resim reset

# Creating a new account. This account will be mainly used for the creation of the tokens
OP1=$(resim new-account)
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS1=$(echo "$OP1" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")
echo "Created account: $ACC_ADDRESS1; Public Key: $PUB_KEY1"

# Creating a number of additional accounts for the users that will be using the DEX.
OP2=$(resim new-account)
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS2=$(echo "$OP2" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")
echo "Created account: $ACC_ADDRESS2; Public Key: $PUB_KEY2"

OP3=$(resim new-account)
export PUB_KEY3=$(echo "$OP3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS3=$(echo "$OP3" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")
echo "Created account: $ACC_ADDRESS3; Public Key: $PUB_KEY3"

OP4=$(resim new-account)
export PUB_KEY4=$(echo "$OP4" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS4=$(echo "$OP4" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")
echo "Created account: $ACC_ADDRESS4; Public Key: $PUB_KEY4"

# Switching the default account to account 1 and creating the sample tokens using this account
# and then funding the other accounts
resim set-default-account $ACC_ADDRESS1 $PUB_KEY1 > /dev/null
resim run "$SCRIPT_DIR/transactions/token_creation_and_funding.rtm" > /dev/null
echo "Created tokens using account: $ACC_ADDRESS1"

# Publishing the package to the local simulator
PK_OP=$(resim publish "$SCRIPT_DIR")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Created package: $PACKAGE"

# Creating the RaDEX component
CP_OP=$(resim run "$SCRIPT_DIR/transactions/component_creation.rtm")
export COMPONENT=$(echo "$CP_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
echo "Created component: $COMPONENT"

# Creating the liquidity pools
resim run "$SCRIPT_DIR/transactions/creating_initial_liquidity_pools.rtm" > /dev/null
echo "Created the liquidity pools."

# Switching to account 2 and performing the simple token swap from BTC to usdt
resim set-default-account $ACC_ADDRESS2 $PUB_KEY2 > /dev/null
resim run "$SCRIPT_DIR/transactions/swap_BTC_for_USDT.rtm" > /dev/null
echo "Simple token swap BTC -> USDT done"

# Switching to account 3 and performing a more complicated swap that goes through a number of liquidity pools 
resim set-default-account $ACC_ADDRESS3 $PUB_KEY3 > /dev/null
resim run "$SCRIPT_DIR/transactions/swap_ADA_for_DOGE.rtm" > /dev/null
echo "Complex token swap ADA -> DOGE done"

# Switching to account 4 and performing the swap of BTC -> USDT and then providing liquidity to the XRD/USDT pool.
resim set-default-account $ACC_ADDRESS4 $PUB_KEY4 > /dev/null
resim run "$SCRIPT_DIR/transactions/swap_BTC_for_USDT_and_add_liquidity.rtm" > /dev/null
echo "Complex token swap BTC -> USDT and providing liquidity done"

# Switching to account 1 to remove all of the BTC/USDT liquidity.
resim set-default-account $ACC_ADDRESS1 $PUB_KEY1 > /dev/null
resim run "$SCRIPT_DIR/transactions/remove_BTC_USDT_liquidity.rtm" > /dev/null
echo "Removing BTC/USDT liquidity done"