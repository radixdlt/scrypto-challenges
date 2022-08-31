#!/usr/bin/env bash

# set -x
set -e

# Getting the directory of this bash script
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
echo "Operating out of: $SCRIPT_DIR"

resim reset

OP1=$(resim new-account)
export PRIV_KEY1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo acc_address1 $ACC_ADDRESS1


OP2=$(resim new-account)
export PRIV_KEY2=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS2=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo acc_address2 $ACC_ADDRESS2

OP2=$(resim new-account)
export PRIV_KEY2=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS2=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo acc_address3 $ACC_ADDRESS3

resim set-default-account $ACC_ADDRESS1 $PRIV_KEY1



PK_OP=$(resim publish ".")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo package_address $PACKAGE

# export REPLACEMENT_LOOKUP=" s/<<<account1_address>>>/$ACC_ADDRESS1/g; \
#    s/<<<package_address>>>/$PACKAGE/g; \
#"
#sed "$REPLACEMENT_LOOKUP" test_transactions/create_component.rtm > test_transactions/create_component.rtm

# Update the package address in the transaction manifest.
# Create the test environment
resim run test_transactions/create_test_environment.rtm

# This sould be done automatically. Unfortunately I didn't have the time until challenge deadline.
export ORACLE=0249d9942ff120650564b715d5af3a1f7717660d457bfe3c59d7a0
export RADEX=0284818bee9dd6f9aa092c1460e1f4abe1bdfcbae91f6ba342c135

export BASE_CURRENCY=035f4ded5d92cbbfa36931a2c54c3569c26083732a4aefdcb10661
export BTC=032c6a0f01710384a21e579e367dfe1ff2884b6f2700707e685ee6
export ETH=03e1a8c56ef3e564685dc4227c4f4edcce0afa2ce8c63378be7a3b
export BNB=039dc9ef1a1156c409f98bcfa90a9aa40ca8796a65a364828255ec
export ADA=030f1c2c1136df4f41dc5959a31109b89d2f2bb88c14d1931ba3cd

# Switch to account 2: the pool manager. Due to shortness of time till deadline everything will be done from one account (all resources are there for now)
# Of course that doesn't make any sense in reality.
# resim set-default-account $ACC_ADDRESS2 $PRIV_KEY2

# Create an investment_pool
resim run test_transactions/create_investment_pool.rtm

export POOL=02deee92bbb0daa74e7846103b138e5c7ea6375d55ebe7d5c60306
export ADMIN_BADGE=03fedc655667f36e2e5db15a8d388126042075d9a66fc40383fdc3

# Fund the investment_pool with eth and btc. Still in the role of the pool manager.
resim run test_transactions/fund_pool.rtm


# Switch to account 3: the pool manager. Due to shortness of time till deadline everything will be done from one account (all resources are there for now)
# Of course that doesn't make any sense in reality.
# resim set-default-account $ACC_ADDRESS3 $PRIV_KEY3
