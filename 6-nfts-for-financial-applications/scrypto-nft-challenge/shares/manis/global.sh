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
echo -e "\e[7m****** Initializing Component with 3 Initial Owners"
component_address=$(resim call-function $package_address Shares new_shares_component 3 | awk -F"Component: " '{print $2}')

echo ""
echo -e "\e[7m****** ECHOING Component ADDRESS *******"
resim show $component_address 

echo ""
echo -e "\e[7m Storing { Resource Address: admin_badge } into cache"
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
echo -e "\e[7m ***** Calling Method: { new_depositor } **********"
resim call-method $component_address new_depositor --proofs 1,$owner_badge_resource_address

echo ""
echo "\e[7m *****showing new depositor badge in account******"
resim show $XRD_ACCNT1

echo ""
echo -e "\e[7m ***** Calling Method: { check_or_create_vault } with xrd token address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag **********"
resim call-method $component_address check_or_create_vault resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag --proofs 1,$owner_badge_resource_address

echo ""
echo "\e[7m *****showing new ResourceAddress in Component Self ******"
resim show $component_address

echo ""
echo -e "\e[7m ***** Calling Method: { deposit_to_treasury } with xrd token address: 999, resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag **********"
resim call-method $component_address deposit_to_treasury 999,resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag --proofs 1,$owner_badge_resource_address

echo ""
echo -e "\e[7m ***** Calling Method: { show_single_treasury_balance } with xrd token address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag **********"
resim call-method $component_address show_single_treasury_balance resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag

echo ""
echo -e "\e[7m ***** Calling Method: { show_all_treasury_balance } **********"
resim call-method $component_address show_all_treasury_balances

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

# resim run update_owner_username.rtm
sed -i "s/{account_address}/$(echo $XRD_ACCNT1)/g" ./manis/update_owner_username.rtm
sed -i "s/{owner_badge_resource_address}/$(echo $owner_badge_resource_address)/g" ./manis/update_owner_username.rtm
sed -i "s/{component_address}/$(echo $component_address)/g" ./manis/update_owner_username.rtm

echo ""
echo -e "\e[7m ***** Calling Method: { update_owner_username } with username: AustinBadass **********"
resim run ./manis/update_owner_username.rtm

echo ""
echo -e "\e[7m ***** ECHOING UPDATED NFTS **********"
echo -e "\033[0m $(resim show $XRD_ACCNT1 | grep "AustinBadass")"

# resim run update_owner_contact.rtm
sed -i "s/{account_address}/$(echo $XRD_ACCNT1)/g" ./manis/update_owner_contact.rtm
sed -i "s/{owner_badge_resource_address}/$(echo $owner_badge_resource_address)/g" ./manis/update_owner_contact.rtm
sed -i "s/{component_address}/$(echo $component_address)/g" ./manis/update_owner_contact.rtm

echo ""
echo -e "\e[7m ***** Calling Method: { update_owner_contact } with phone number  5559991234 **********"
resim run ./manis/update_owner_contact.rtm

echo -e ""
echo -e "\e[7m ***** ECHOING UPDATED NFTS **********"
echo -e "\033[0m $(resim show $XRD_ACCNT1 | grep "AustinBadass")"

# resim call-method distrubte_treasury_funds
# resim run claim_treasury_funds.rtm
echo ""
echo -e "\e[7m ***** Calling Method: { distribute_treasury_funds }  with 100 XRD to the 3 vaults associated with 3 nft badges **********"
resim call-method $component_address distribute_treasury_funds resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag 100 --proofs 3,$owner_badge_resource_address
resim show $component_address

sed -i "s/{account_address}/$(echo $XRD_ACCNT1)/g" ./manis/claim_treasury_funds.rtm
sed -i "s/{owner_badge_resource_address}/$(echo $owner_badge_resource_address)/g" ./manis/claim_treasury_funds.rtm
sed -i "s/{component_address}/$(echo $component_address)/g" ./manis/claim_treasury_funds.rtm

echo ""
echo -e "\e[7m ***** Calling Method: { claim_treasury_funds } **********"
resim run ./manis/claim_treasury_funds.rtm

# resim run split_ownership.rtm
sed -i "s/{account_address}/$(echo $XRD_ACCNT1)/g" ./manis/split_ownership.rtm
sed -i "s/{owner_badge_resource_address}/$(echo $owner_badge_resource_address)/g" ./manis/split_ownership.rtm
sed -i "s/{component_address}/$(echo $component_address)/g" ./manis/split_ownership.rtm

echo ""
echo -e "\e[7m ***** Calling Method: { split_ownership } **********"
resim run ./manis/split_ownership.rtm

# resim run merge_ownership.rtm
sed -i "s/{account_address}/$(echo $XRD_ACCNT1)/g" ./manis/merge_ownership.rtm
sed -i "s/{owner_badge_resource_address}/$(echo $owner_badge_resource_address)/g" ./manis/merge_ownership.rtm
sed -i "s/{component_address}/$(echo $component_address)/g" ./manis/merge_ownership.rtm

echo ""
echo -e "\e[7m ***** Calling Method: { merge_ownership } **********"
resim run ./manis/merge_ownership.rtm

# final show ledger, account, component
echo -e "\e[7m ***** FINAL LEDGER **********"
resim show-ledger
echo -e "\e[7m ***** FINAL COMPONENT **********"
resim show $component_address
echo -e "\e[7m ***** FINAL DEFAULT ACCOUNT **********"
resim show $XRD_ACCNT1

# refresh rtm for new run
sed -i "s/$(echo $XRD_ACCNT1)/{account_address}/g" ./manis/update_owner_username.rtm
sed -i "s/$(echo $owner_badge_resource_address)/{owner_badge_resource_address}/g" ./manis/update_owner_username.rtm
sed -i "s/$(echo $component_address)/{component_address}/g" ./manis/update_owner_username.rtm

sed -i "s/$(echo $XRD_ACCNT1)/{account_address}/g" ./manis/update_owner_contact.rtm
sed -i "s/$(echo $owner_badge_resource_address)/{owner_badge_resource_address}/g" ./manis/update_owner_contact.rtm
sed -i "s/$(echo $component_address)/{component_address}/g" ./manis/update_owner_contact.rtm

sed -i "s/$(echo $XRD_ACCNT1)/{account_address}/g" ./manis/claim_treasury_funds.rtm
sed -i "s/$(echo $owner_badge_resource_address)/{owner_badge_resource_address}/g" ./manis/claim_treasury_funds.rtm
sed -i "s/$(echo $component_address)/{component_address}/g" ./manis/claim_treasury_funds.rtm

sed -i "s/$(echo $XRD_ACCNT1)/{account_address}/g" ./manis/split_ownership.rtm
sed -i "s/$(echo $owner_badge_resource_address)/{owner_badge_resource_address}/g" ./manis/split_ownership.rtm
sed -i "s/$(echo $component_address)/{component_address}/g" ./manis/split_ownership.rtm

sed -i "s/$(echo $XRD_ACCNT1)/{account_address}/g" ./manis/merge_ownership.rtm
sed -i "s/$(echo $owner_badge_resource_address)/{owner_badge_resource_address}/g" ./manis/merge_ownership.rtm
sed -i "s/$(echo $component_address)/{component_address}/g" ./manis/merge_ownership.rtm
