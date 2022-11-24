#!/bin/bash

resim reset

echo ""
echo -e "\e[7m****** Storing Accnt 1 credentials into cache ******"
account1_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT1=${account1_creds[0]}
export XRD_ACCNT1_pub=${account1_creds[1]}
export XRD_ACCNT1_priv=${account1_creds[2]}
resim show $XRD_ACCNT1

echo ""
echo -e "\e[7m****** Storing Accnt 2 credentials into cache ******"
account2_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT2=${account2_creds[0]}
export XRD_ACCNT2_pub=${account2_creds[1]}
export XRD_ACCNT2_priv=${account2_creds[2]}
resim show $XRD_ACCNT2

echo ""
echo -e "\e[7m****** Storing Accnt 3 credentials into cache ******"
account3_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT3=${account3_creds[0]}
export XRD_ACCNT3_pub=${account3_creds[1]}
export XRD_ACCNT3_priv=${account3_creds[2]}
resim show $XRD_ACCNT3


package_address=$(resim publish . | awk -F": " '{print $2}')
echo ""
echo "\e[7m****** ECHOING Package ADDRESS *******"
echo $package_address

echo ""
echo -e "\e[7m****** Initializing Component with 3 Initial Owners"
component_address=$(resim call-function $package_address Shares new_shares_component 3 | awk -F"Component: " '{print $2}')

echo ""
echo -e "\e[7m****** ECHOING Component ADDRESS *******"
resim show $component_address 

echo ""
echo -e "\e[7m Storing { Resource Address: owner_badge } into cache"
resim show $XRD_ACCNT1
owner_badge_resource_address=$(resim show $XRD_ACCNT1 | grep 'owner_badge' | awk -F": " '{print $3}' | sed 's/, name//')
echo $owner_badge_resource_address

echo ""
echo -e "\e[7m****** Calling Method: { push_owner_record } *******"
resim call-method $component_address push_owner_record 3,$owner_badge_resource_address

echo ""
echo -e "\e[7m ***** ECHOING CREATED NFTs **********"
resim show $XRD_ACCNT1

echo ""
echo -e "\e[7m ***** Calling Method: { check_or_create_vault } with xrd token address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag **********"
resim call-method $component_address check_or_create_vault resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag --proofs 1,$owner_badge_resource_address

echo ""
echo "\e[7m *****showing new ResourceAddress in Component Self ******"
resim show $component_address

# begin sed on .rtm files

echo $component_address
echo $component_address
echo $component_address
echo $component_address
echo $component_address
echo $component_address


# transfer an owner nft from account 1 to account 2
resim transfer 1 $owner_badge_resource_address $XRD_ACCNT2
# transfer an owner nft from account 1 to account 3
resim transfer 1 $owner_badge_resource_address $XRD_ACCNT3


# resim call-method pool_escrow_funds

sed -i "s/{account_address_1}/$(echo $XRD_ACCNT1)/g" ./manis/Scenario2/pool_escrow_funds.rtm
sed -i "s/{account_address_2}/$(echo $XRD_ACCNT2)/g" ./manis/Scenario2/pool_escrow_funds.rtm
sed -i "s/{account_address_3}/$(echo $XRD_ACCNT3)/g" ./manis/Scenario2/pool_escrow_funds.rtm
sed -i "s/{owner_badge_resource_address}/$(echo $owner_badge_resource_address)/g" ./manis/Scenario2/pool_escrow_funds.rtm
sed -i "s/{component_address}/$(echo $component_address)/g" ./manis/Scenario2/pool_escrow_funds.rtm

echo ""
echo -e "\e[7m ***** Calling Method: { pool_treasury_funds } **********"
resim run ./manis/Scenario2/pool_escrow_funds.rtm


# final show ledger, account, component
echo -e "\e[7m ***** FINAL LEDGER **********"
resim show-ledger
echo -e "\e[7m ***** FINAL COMPONENT **********"
resim show $component_address
echo -e "\e[7m ***** FINAL ACCOUNT 1 **********"
resim show $XRD_ACCNT1
echo -e "\e[7m ***** FINAL ACCOUNT 2 **********"
resim show $XRD_ACCNT2
echo -e "\e[7m ***** FINAL ACCOUNT 3 **********"
resim show $XRD_ACCNT3

# refresh rtm for new run
sed -i "s/$(echo $XRD_ACCNT1)/{account_address_1}/g" ./manis/Scenario2/pool_escrow_funds.rtm
sed -i "s/$(echo $XRD_ACCNT2)/{account_address_2}/g" ./manis/Scenario2/pool_escrow_funds.rtm
sed -i "s/$(echo $XRD_ACCNT3)/{account_address_3}/g" ./manis/Scenario2/pool_escrow_funds.rtm
sed -i "s/$(echo $owner_badge_resource_address)/{owner_badge_resource_address}/g" ./manis/Scenario2/pool_escrow_funds.rtm
sed -i "s/$(echo $component_address)/{component_address}/g" ./manis/Scenario2/pool_escrow_funds.rtm


