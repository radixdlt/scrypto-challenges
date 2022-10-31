#!/usr/bin/env bash

#set -x
set -e

source ./funding_and_assign.sh

logc "Neura holders staking with different amount in different validator"

resim set-default-account $VAL1_ACC $VAL1_PIV
resim run ./transaction_manifest/stake1

resim set-default-account $VAL2_ACC $VAL2_PIV
resim run ./transaction_manifest/stake2

resim set-default-account $VAL3_ACC $VAL3_PIV
resim run ./transaction_manifest/stake3

resim set-default-account $VAL4_ACC $VAL4_PIV
resim run ./transaction_manifest/stake4

resim set-default-account $VAL5_ACC $VAL5_PIV
resim run ./transaction_manifest/stake5

resim set-default-account $USER1_ACC $USER1_PIV
resim run ./transaction_manifest/stake6

resim set-default-account $USER2_ACC $USER2_PIV
resim run ./transaction_manifest/stake7

resim set-default-account $USER3_ACC $USER3_PIV
resim run ./transaction_manifest/stake8

resim set-default-account $USER4_ACC $USER4_PIV
resim run ./transaction_manifest/stake9

resim set-default-account $USER5_ACC $USER5_PIV
resim run ./transaction_manifest/stake10

logc "Some Neura holders addstake, unstake, stop unstake"

resim set-default-account $VAL4_ACC $VAL4_PIV
resim run ./transaction_manifest/add_stake

resim set-default-account $USER2_ACC $USER2_PIV
resim run ./transaction_manifest/unstake

resim set-default-account $USER4_ACC $USER4_PIV
resim run ./transaction_manifest/unstake2

logy "Set current epoch + 12"
epoch=$(($epoch + 12))
resim set-current-epoch $epoch
resim run ./transaction_manifest/stop_unstake

resim set-default-account $USER5_ACC $USER5_PIV
resim run ./transaction_manifest/unstake3

logy "Set current epoch + 23"
epoch=$(($epoch + 23))
resim set-current-epoch $epoch

logr "This user try to withdraw in unstaking period. This should show error!"
resim run ./transaction_manifest/withdraw || true

logy "Set current epoch + 500"
epoch=$(($epoch + 500))
resim set-current-epoch $epoch
logg "Now he can withdraw."
resim run ./transaction_manifest/withdraw

completed
