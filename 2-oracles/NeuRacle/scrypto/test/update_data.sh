#!/usr/bin/env bash

#set -x
set -e

source ./log.sh

logc "The Gateway get api list from NeuRacle component, fetch and feed data to the system"

logy "This will take a bit"

resim set-default-account $VAL1_ACC $VAL1_PIV
export VALUP_ADDRESS=$VAL1_ADDRESS
export VALUP_ACC=$VAL1_ACC

export RESULTS=`./neuracle_gateway.exe`

resim run ./transaction_manifest/update_data

resim set-default-account $VAL2_ACC $VAL2_PIV
export VALUP_ADDRESS=$VAL2_ADDRESS
export VALUP_ACC=$VAL2_ACC
resim run ./transaction_manifest/update_data

logy "Let one validator non-active and one validator to intervene in the Gateway process and behave malicious"
resim set-default-account $VAL4_ACC $VAL4_PIV
export VALUP_ADDRESS=$VAL4_ADDRESS
export VALUP_ACC=$VAL4_ACC
resim run ./transaction_manifest/update_data_malicious

resim set-default-account $VAL5_ACC $VAL5_PIV
export VALUP_ADDRESS=$VAL5_ADDRESS
export VALUP_ACC=$VAL5_ACC
resim run ./transaction_manifest/update_data

completed