#! /bin/bash

resim reset

# SET RADIX BUCKET VARIABLE
rdx=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag

# CREATE FIRST ACCOUNT
output1=$(resim new-account)
export ac1="$( echo ${output1} | cut -b 60-121)"
export pv1="$( echo ${output1} | cut -b 215-279)"


# CREATE SECONDARY ACCOUNT
output2=$(resim new-account)
export ac2="$( echo ${output2} | cut -b 60-121)"
export pv2="$( echo ${output2} | cut -b 215-279)"

# CREATE SECONDARY ACCOUNT
#outputVendorAcc=$(resim new-account)
#export itemVendorAccount="$( echo ${outputVendorAcc} | cut -b 60-121)"
#export pv3="$( echo ${outputVendorAcc} | cut -b 215-279)"


# PUBLISH PACKAGE
output3=$(resim publish .)
export pkg="$( echo ${output3}| grep -E '\bpackage_' | cut -b 23-125)"

# INSTANTIATE COMPONENT
# output4=$(resim call-function ${pkg} Fond instantiate_fond)
output4=$(resim call-function ${pkg} Fond instantiate_fond 800,${rdx})
export comp="$( echo ${output4}| grep -E '\bcomponent_' | cut -b 1332-1396)"

# CREATE FIRST CAMPAIGN
resim call-method $comp create_campaign "MonaLisa" "You're an overrated piece of shit" 100.0

resim show $ac1

resim call-method $comp create_campaign "VanGogh" "I can't hear you" 50.0


# SWITCH ACCOUNTS
resim set-default-account $ac2 $pv2


# output5=$(resim show $ac2)
# export res2="$( echo ${output5}| cut -b 787-850)"

# INVEST IN ASSET
#resim call-method $comp invest_in_campaigns <AMOUNT>,$rdx <ASSET_NFT_ID>
resim call-method $comp invest_in_campaigns 100,$rdx 0

# INVEST IN CAMPAIGN 0
#resim call-method $comp invest_in_campaigns 5,$rdx 5.0 "0" $ac2

# INVEST IN CAMPAIGN 0
#resim call-method $comp invest_in_campaigns 7,$rdx 7.0 "0" $ac2

#INVEST IN CAMPAIGN 1
resim call-method $comp invest_in_campaigns 50,$rdx 1

# SWITCH ACCOUNTS
resim set-default-account $ac1 $pv1

## BUY ITEM
resim call-method $comp add_to_inventory 0

## SELL ITEM
resim call-method $comp sell_item 0

# SWITCH ACCOUNTS
resim set-default-account $ac2 $pv2


# resim call-method retrieve_funds 1,<nft-resource-address> 0


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
#echo "item vendor account"
#echo $itemVendorAccount


# CHECK RESOURCES OF ACCOUNT 2
#resim show $ac2


