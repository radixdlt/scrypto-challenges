#!/usr/bin/env bash

#set -x
set -e

source ./log.sh

logc "Users use badge to get data"

resim set-default-account $USER1_ACC $USER1_PIV
resim call-method $COMP get_data 1,$USER_BADGE

resim set-default-account $USER2_ACC $USER2_PIV
resim call-method $COMP get_data 1,$USER_BADGE

resim set-default-account $USER3_ACC $USER3_PIV
resim call-method $COMP get_data 1,$USER_BADGE

resim set-default-account $USER4_ACC $USER4_PIV
resim call-method $COMP get_data 1,$USER_BADGE

resim set-default-account $USER5_ACC $USER5_PIV
resim call-method $COMP get_data 1,$USER_BADGE

logr "A person try to call new round again within round-length time. This should show error!"

resim run ./transaction_manifest/start_round || true

logc "Advance epoch by 1000."
epoch=$(($epoch + 1000))
resim set-current-epoch $epoch

logg "That person call new round again."
resim run ./transaction_manifest/start_round

resim set-default-account $VAL1_ACC $VAL1_PIV
export VALUP_ADDRESS=$VAL1_ADDRESS
export VALUP_ACC=$VAL1_ACC
resim run ./transaction_manifest/update_data

logr "Only one validator is active, but that validator try to end the round. This should show error!"
resim run ./transaction_manifest/end_round || true

resim set-default-account $VAL2_ACC $VAL2_PIV
export VALUP_ADDRESS=$VAL2_ADDRESS
export VALUP_ACC=$VAL2_ACC
resim run ./transaction_manifest/update_data

resim set-default-account $VAL4_ACC $VAL4_PIV
export VALUP_ADDRESS=$VAL4_ADDRESS
export VALUP_ACC=$VAL4_ACC
resim run ./transaction_manifest/update_data

resim set-default-account $VAL5_ACC $VAL5_PIV
export VALUP_ADDRESS=$VAL5_ADDRESS
export VALUP_ACC=$VAL5_ACC
resim run ./transaction_manifest/update_data_malicious

logg "Now 4/5 validator voted, someone try to end the round."
resim run ./transaction_manifest/end_round

logr "User try to get data after out of time. This should show error!"
resim set-default-account $USER2_ACC $USER2_PIV
resim call-method $COMP get_data 1,$USER_BADGE || true

logg "User funding account again to get data."
resim call-method $COMP refund_account 1,$USER_BADGE 100,$NEURA 
resim call-method $COMP get_data 1,$USER_BADGE 

completed