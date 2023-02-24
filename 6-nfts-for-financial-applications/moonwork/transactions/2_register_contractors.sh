#!/bin/bash

# Create a contractor named beemdpv
resim set-default-account $address1 $priv_key1
export contractor=beemdvp
export account=$address1
resim run ./transactions/register_as_contractor.rtm

# create another contractor named wylie
resim set-default-account $address3 $priv_key3
export contractor=wylie
export account=$address3
resim run ./transactions/register_as_contractor.rtm

resim set-default-account $address1 $priv_key1
