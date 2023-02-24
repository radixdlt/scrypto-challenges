#!/bin/bash

resim set-default-account $address1 $priv_key1
export account=$address1
export op1=$(resim run ./transactions/promotion/create_promotion_service.rtm)
# promotion component address
export promotion_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")
# promotion resource which is stored in the vault
export promotion_resource=$(echo $op1 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
