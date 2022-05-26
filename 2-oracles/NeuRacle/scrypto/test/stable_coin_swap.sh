#!/bin/bash

#set -x
set -e

source ./log.sh


logc "Advance epoch by 1."
epoch=$(($epoch + 1))
resim set-current-epoch $epoch

logc "Run a data voting round to get newest data."
source ./data_refresh_round.sh || true

logc "Begin swap"

resim set-default-account $ADMIN_ACC $ADMIN_PIV || true

export NUM=15000 #You can edit this
export RS=$NEURA
resim run ./transaction_manifest/auto_swap || true


export NUM=120 #You can edit this
export RS=$USDN
resim run ./transaction_manifest/auto_swap || true


completed