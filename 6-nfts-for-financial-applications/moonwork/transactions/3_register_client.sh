#!/bin/bash

resim set-default-account $address2 $priv_key2
export account=$address2
export client=slys
resim run ./transactions/register_as_client.rtm

resim set-default-account $address4 $priv_key4
export account=$address4
export client=dan
resim run ./transactions/register_as_client.rtm

resim set-default-account $address1 $priv_key1
