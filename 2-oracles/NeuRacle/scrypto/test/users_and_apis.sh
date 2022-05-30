#!/usr/bin/env bash

#set -x
set -e

source ./stake_and_unstake.sh

logc "NAR holders (or dapps's component) ask a data query from an api and become user"

resim set-default-account $USER1_ACC $USER1_PIV
resim run ./transaction_manifest/become_user

resim set-default-account $USER2_ACC $USER2_PIV
resim run ./transaction_manifest/become_user2

resim set-default-account $USER3_ACC $USER3_PIV
resim run ./transaction_manifest/become_user3

resim set-default-account $USER4_ACC $USER4_PIV
resim run ./transaction_manifest/become_user4

resim set-default-account $USER5_ACC $USER5_PIV
resim run ./transaction_manifest/become_user5

logc "NAR holders (or dapps's component) refund their data accounts"

resim set-default-account $USER2_ACC $USER2_PIV
resim run ./transaction_manifest/refund

resim set-default-account $USER4_ACC $USER4_PIV
resim run ./transaction_manifest/refund2

logc "Show current apis"
resim run ./transaction_manifest/show_apis

logy "In real case, the data source feed to NeuRacle Gateway should return only 1 specific data that user need"
logy "For testing purpose, the Gateway prototype will just assume that users need 1 specific data from chunks of data that those apis return, and feedback the data users need"
logy "In this case the Gateway prototype will assume that:"
logp "user1 want 'the last traded price of xrd/usd on Bitfinex'"
logp "user2 want 'realtime in Ho Chi Minh/VietNam timezone'"
logp "user3 want 'the last aggregrated price of xrd/usd on CoinGecko'"
logp "user4 want 'today date in London/Europe timezone'"
logp "user5 want 'the last aggregrated price of btc/usd on CoinGecko'"

completed