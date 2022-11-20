#! /bin/bash

resim reset

output1=$(resim new-account)
export ac1="$( echo ${output1} | cut -b 60-121)"
export pv1="$( echo ${output1} | cut -b 215-279)"

output2=$(resim new-account)
export ac2="$( echo ${output2} | cut -b 60-121)"
export pv2="$( echo ${output2} | cut -b 215-279)"
echo $ac2

output3=$(resim publish .)
export pkg="$( echo ${output3}| grep -E '\bpackage_' | cut -b 23-125)"

output4=$(resim call-function ${pkg} Fond instantiate_fond)
export comp="$( echo ${output4}| grep -E '\bcomponent_' | cut -b 779-842)"

resim call-method $comp create_campaign "MonaLisa" "You're an overrated piece of shit" 100.0

resim set-default-account $ac2 $pv2

res2=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag
# output5=$(resim show $ac2)
# export res2="$( echo ${output5}| cut -b 787-850)"

resim call-method $comp invest_in_campaigns 5,$res2 5.0 "0"

resim call-method $comp invest_in_campaigns 7,$res2 7.0 "0"

echo "VARS"
echo "account1"
echo $ac1
echo "private key1"
echo $pv1
echo "account2"
echo $ac2
echo "private key2"
echo $pv2
echo "package"
echo $pkg
echo "component"
echo $comp
echo "resource2"
echo $res2






# resim new-account
# export ac2=<account-address>
# export p2=<private-key>
# resim set-default-account $ac2 $p2

