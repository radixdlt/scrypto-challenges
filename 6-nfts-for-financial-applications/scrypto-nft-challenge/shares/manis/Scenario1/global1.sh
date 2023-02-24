#!/bin/bash

resim reset

echo ""
echo -e "\e[7m Storing Accnt 1 credentials into cache"
account1_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT1=${account1_creds[0]}
export XRD_ACCNT1_pub=${account1_creds[1]}
export XRD_ACCNT1_priv=${account1_creds[2]}
resim show $XRD_ACCNT1

package_address=$(resim publish . | awk -F": " '{print $2}')
echo ""
echo "\e[7m****** ECHOING Package ADDRESS *******"
echo $package_address

echo ""
echo -e "\e[7m****** Initializing Component with 1 Initial Owners"
component_address=$(resim call-function $package_address Shares new_shares_component 1 | awk -F"Component: " '{print $2}')

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
resim call-method $component_address push_owner_record 1,$owner_badge_resource_address

echo ""
echo -e "\e[7m ***** ECHOING CREATED NFTs **********"
resim show $XRD_ACCNT1

echo ""
echo -e "\e[7m ***** Calling Method: { check_or_create_vault } with xrd token address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag **********"
resim call-method $component_address check_or_create_vault resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag --proofs 1,$owner_badge_resource_address

echo ""
echo "\e[7m *****showing new ResourceAddress in Component Self ******"
resim show $component_address

echo ""
echo -e "\e[7m ***** Calling Method: { show_single_treasury_balance } with xrd token address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag **********"
resim call-method $component_address show_single_treasury_balance resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag

echo ""
echo -e "\e[7m ***** Calling Method: { show_all_treasury_balance } **********"
resim call-method $component_address show_all_treasury_balances

# begin sed on .rtm files

echo $component_address
echo $component_address
echo $component_address
echo $component_address
echo $component_address
echo $component_address

# resim run split_ownership.rtm
sed -i "s/{account_address}/$(echo $XRD_ACCNT1)/g" ./manis/Scenario1/split_ownership.rtm
sed -i "s/{owner_badge_resource_address}/$(echo $owner_badge_resource_address)/g" ./manis/Scenario1/split_ownership.rtm
sed -i "s/{component_address}/$(echo $component_address)/g" ./manis/Scenario1/split_ownership.rtm

echo ""
echo -e "\e[7m ***** Calling Method: { split_ownership } **********"
resim run ./manis/Scenario1/split_ownership.rtm

echo -e "\e[7m ***** You can now see the ownerhip NFT has been split into two NFTs, one with 95% ownership, one with 5% ownership******"
resim show $XRD_ACCNT1

# refresh rtm for new run
sed -i "s/$(echo $XRD_ACCNT1)/{account_address}/g" ./manis/Scenario1/split_ownership.rtm
sed -i "s/$(echo $owner_badge_resource_address)/{owner_badge_resource_address}/g" ./manis/Scenario1/split_ownership.rtm
sed -i "s/$(echo $component_address)/{component_address}/g" ./manis/Scenario1/split_ownership.rtm


