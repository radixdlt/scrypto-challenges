#!/usr/bin/env bash

#set -x
set -e

#Use log
source ./log.sh

logc "New fresh start"
resim reset

export XRD=030000000000000000000000000000000000000000000000000004

logc "Creating Admin account."
output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export ADMIN_ACC=`echo $output | cut -d " " -f1`
export ADMIN_PIV=`echo $output | cut -d " " -f2`

logc "Creating 5 validators account."
output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export VAL1_ACC=`echo $output | cut -d " " -f1`
export VAL1_PIV=`echo $output | cut -d " " -f2`

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export VAL2_ACC=`echo $output | cut -d " " -f1`
export VAL2_PIV=`echo $output | cut -d " " -f2`

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export VAL3_ACC=`echo $output | cut -d " " -f1`
export VAL3_PIV=`echo $output | cut -d " " -f2`

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export VAL4_ACC=`echo $output | cut -d " " -f1`
export VAL4_PIV=`echo $output | cut -d " " -f2`

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export VAL5_ACC=`echo $output | cut -d " " -f1`
export VAL5_PIV=`echo $output | cut -d " " -f2`

logc "Creating 5 users account."
output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export USER1_ACC=`echo $output | cut -d " " -f1`
export USER1_PIV=`echo $output | cut -d " " -f2`

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export USER2_ACC=`echo $output | cut -d " " -f1`
export USER2_PIV=`echo $output | cut -d " " -f2`

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export USER3_ACC=`echo $output | cut -d " " -f1`
export USER3_PIV=`echo $output | cut -d " " -f2`

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export USER4_ACC=`echo $output | cut -d " " -f1`
export USER4_PIV=`echo $output | cut -d " " -f2`

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export USER5_ACC=`echo $output | cut -d " " -f1`
export USER5_PIV=`echo $output | cut -d " " -f2`

logc "Publish NeuRacle, this will take a bit at first"
export package=`resim publish ../. | awk '/Package:/ {print $NF}'`

logy "Since this prototype will highly depend on epoch progress, to elaborate the test, let's choose a random epoch from 0-500"
RANGE=500

epoch=$RANDOM
let "epoch %= $RANGE"

logc "Set random current epoch in range 500:" $epoch
resim set-current-epoch $epoch


