#!/bin/bash

#set -x
set -e
source ./log.sh

logc "Admin instantiate new algorithm stable coin project, pegged in USD"
logy "Because NAR token is still haven't launched yet, let's use XRD instead"
logy "This prototype also haven't implement a fee mechanism yet"

resim set-default-account $ADMIN_ACC $ADMIN_PIV
output=`resim run ./transaction_manifest/stable_coin | awk '/USDN: |USDNStable Coin address: / {print $NF}'`
export USDN=`echo $output | cut -d " " -f1`
export SC_COMP=`echo $output | cut -d " " -f2`

completed