#!/bin/bash

resim set-default-account $address1 $priv_key1
export account=$address1
resim run ./transactions/promotion/promote_contractor.rtm
