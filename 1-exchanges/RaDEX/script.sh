#!/usr/bin/env bash

set -x
set -e

# Getting the directory of this bash script
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
echo "Operating out of: $SCRIPT_DIR"

# Resetting the local script simulator 
resim reset

# Creating a new account. This account will be mainly used for the creation of the tokens
OP1=$(resim new-account)
export PRIV_KEY1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

# Creating a number of additional accounts for the users that will be using the DEX.
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

resim set-default-account $ACC_ADDRESS1 $PUB_KEY1 $PRIV_KEY1

# Publishing the package to the local simulator
PK_OP=$(resim publish "$SCRIPT_DIR")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

export REPLACEMENT_LOOKUP=" s/<<<account1_address>>>/$ACC_ADDRESS1/g; \
    s/<<<account2_address>>>/$ACC_ADDRESS2/g; \
    s/<<<account3_address>>>/$ACC_ADDRESS3/g; \
    s/<<<account4_address>>>/$ACC_ADDRESS4/g; \
    s/<<<package_address>>>/$PACKAGE/g; \
"

sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/token_creation.rtm > $SCRIPT_DIR/transactions/token_creation.rtm
TOKENS_OP=$(resim run "$SCRIPT_DIR/transactions/token_creation.rtm")
export BTC=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')
export LTC=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '2!d')
export XRP=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '3!d')
export DOGE=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '4!d')
export XMR=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '5!d')
export USDT=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '6!d')
export BNB=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '7!d')
export ADA=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '8!d')
export QNT=$(echo "$TOKENS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '9!d')

REPLACEMENT_LOOKUP+="s/<<<bitcoin_resource_address>>>/$BTC/g; \
    s/<<<litecoin_resource_address>>>/$LTC/g; \
    s/<<<xrp_resource_address>>>/$XRP/g; \
    s/<<<doge_resource_address>>>/$DOGE/g; \
    s/<<<monero_resource_address>>>/$XMR/g; \
    s/<<<tether_resource_address>>>/$USDT/g; \
    s/<<<bnb_resource_address>>>/$BNB/g; \
    s/<<<cardano_resource_address>>>/$ADA/g; \
    s/<<<quant_resource_address>>>/$QNT/g; \
"

sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/token_funding.rtm > $SCRIPT_DIR/transactions/token_funding.rtm
resim run "$SCRIPT_DIR/transactions/token_funding.rtm"

sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/component_creation.rtm > $SCRIPT_DIR/transactions/component_creation.rtm
CP_OP=$(resim run "$SCRIPT_DIR/transactions/component_creation.rtm")
export COMPONENT=$(echo "$CP_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
REPLACEMENT_LOOKUP+="s/<<<component_address>>>/$COMPONENT/g;"

sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/creating_initial_liquidity_pools.rtm > $SCRIPT_DIR/transactions/creating_initial_liquidity_pools.rtm
POOLS_OP=$(resim run "$SCRIPT_DIR/transactions/creating_initial_liquidity_pools.rtm")
export BTC_USDT=$(echo "$POOLS_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '4!d')
REPLACEMENT_LOOKUP+="s/<<<btc_usdt_resource_address>>>/$BTC_USDT/g;"

resim set-default-account $ACC_ADDRESS2 $PUB_KEY2 $PRIV_KEY2
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/swap_BTC_for_USDT.rtm > $SCRIPT_DIR/transactions/swap_BTC_for_USDT.rtm
resim run "$SCRIPT_DIR/transactions/swap_BTC_for_USDT.rtm"

resim set-default-account $ACC_ADDRESS3 $PUB_KEY3 $PRIV_KEY3
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/swap_ADA_for_DOGE.rtm > $SCRIPT_DIR/transactions/swap_ADA_for_DOGE.rtm
resim run "$SCRIPT_DIR/transactions/swap_ADA_for_DOGE.rtm"

resim set-default-account $ACC_ADDRESS4 $PUB_KEY4 $PRIV_KEY4
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/swap_BTC_for_USDT_and_add_liquidity.rtm > $SCRIPT_DIR/transactions/swap_BTC_for_USDT_and_add_liquidity.rtm
resim run "$SCRIPT_DIR/transactions/swap_BTC_for_USDT_and_add_liquidity.rtm"

resim set-default-account $ACC_ADDRESS1 $PUB_KEY1 $PRIV_KEY1
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/remove_BTC_USDT_liquidity.rtm > $SCRIPT_DIR/transactions/remove_BTC_USDT_liquidity.rtm
resim run "$SCRIPT_DIR/transactions/remove_BTC_USDT_liquidity.rtm"