#! /bin/bash

resim reset

# CREATE FIRST ACCOUNT
output1=$(resim new-account)
export ac1="$( echo ${output1} | cut -b 60-121)"
export pv1="$( echo ${output1} | cut -b 215-279)"


# CREATE SECONDARY ACCOUNT
output2=$(resim new-account)
export ac2="$( echo ${output2} | cut -b 60-121)"
export pv2="$( echo ${output2} | cut -b 215-279)"

# CREATE SECONDARY ACCOUNT
outputVendorAcc=$(resim new-account)
export itemVendorAccount="$( echo ${outputVendorAcc} | cut -b 60-121)"
export pv3="$( echo ${outputVendorAcc} | cut -b 215-279)"


# PUBLISH PACKAGE
output3=$(resim publish .)
export pkg="$( echo ${output3}| grep -E '\bpackage_' | cut -b 23-125)"

# INSTANTIATE COMPONENT
output4=$(resim call-function ${pkg} Fond instantiate_fond)
export comp="$( echo ${output4}| grep -E '\bcomponent_' | cut -b 779-842)"

# CREATE FIRST CAMPAIGN
resim call-method $comp create_campaign "MonaLisa" "You're an overrated piece of shit" 100.0

resim call-method $comp create_campaign "VanGogh" "I can't hear you" 50.0


# SWITCH ACCOUNTS
resim set-default-account $ac2 $pv2

# SET RADIX BUCKET VARIABLE
rdx=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag
# output5=$(resim show $ac2)
# export res2="$( echo ${output5}| cut -b 787-850)"

# INVEST IN CAMPAIGN 0
resim call-method $comp invest_in_campaigns 5,$rdx 5.0 "0" $ac2

# INVEST IN CAMPAIGN 0
resim call-method $comp invest_in_campaigns 7,$rdx 7.0 "0" $ac2

#INVEST IN CAMPAIGN 1
resim call-method $comp invest_in_campaigns 50,$rdx 50.0 "1" $ac2

# SWITCH ACCOUNTS
resim set-default-account $ac1 $pv1

## BUY ITEM
# resim call-method $comp buy_item "1" $itemVendorAccount


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
echo "item vendor account"
echo $itemVendorAccount


# CHECK RESOURCES OF ACCOUNT 2
resim show $ac2


