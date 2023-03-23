#!/bin/bash

resim reset
clear
export XRD=$"resource_sim1qzkcyv5dwq3r6kawy6pxpvcythx8rh8ntum6ws62p95sqjjpwr"

# Create account #1
echo ""
echo -e "\e[7m Storing Accnt 1 credentials into cache"
account1_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT1=${account1_creds[0]}
export XRD_ACCNT1_pub=${account1_creds[1]}
export XRD_ACCNT1_priv=${account1_creds[2]}
resim show $XRD_ACCNT1

# # Storing Owner badge address
echo -e "\e[7m****** Echoing "owner_badge_address" ********"
owner_badge_address=$(resim show $XRD_ACCNT1 | grep '"Owner badge"' | awk -F": " '{print $3}' | awk -F ", " '{print $1}')
echo $owner_badge_address


# Create account #2
echo ""
echo -e "\e[7m Storing Accnt 2 credentials into cache"
account2_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT2=${account2_creds[0]}
export XRD_ACCNT2_pub=${account2_creds[1]}
export XRD_ACCNT2_priv=${account2_creds[2]}
# resim show $XRD_ACCNT2

# Create account #3
echo ""
echo -e "\e[7m Storing Accnt 3 credentials into cache"
account3_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT3=${account3_creds[0]}
export XRD_ACCNT3_pub=${account3_creds[1]}
export XRD_ACCNT3_priv=${account3_creds[2]}
# resim show $XRD_ACCNT3

# Create account #4
echo ""
echo -e "\e[7m Storing Accnt 4 credentials into cache"
account4_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT4=${account4_creds[0]}
export XRD_ACCNT4_pub=${account4_creds[1]}
export XRD_ACCNT4_priv=${account4_creds[2]}
# resim show $XRD_ACCNT4

# Create account #5
echo ""
echo -e "\e[7m Storing Accnt 5 credentials into cache"
account5_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT5=${account5_creds[0]}
export XRD_ACCNT5_pub=${account5_creds[1]}
export XRD_ACCNT5_priv=${account5_creds[2]}
# resim show $XRD_ACCNT5

# Create account #6
echo ""
echo -e "\e[7m Storing Accnt 6 credentials into cache"
account6_creds=($(resim new-account | awk -F": " '{print $2,$4,$6}'))
export XRD_ACCNT6=${account6_creds[0]}
export XRD_ACCNT6_pub=${account6_creds[1]}
export XRD_ACCNT6_priv=${account6_creds[2]}
# resim show $XRD_ACCNT6

# Set default account to xrd_accnt1
resim set-default-account $XRD_ACCNT1 $XRD_ACCNT1_priv $owner_badge_address:#1#

echo ""
package_address=$(resim publish . | awk -F": " '{print $2}')
echo -e "\e[7m****** ECHOING Package ADDRESS *******"
echo $package_address


# This component returns 3 component addresses because scrilla instantiates 2 others
echo ""
echo -e "\e[7m****** Initializing SCRILLA Component ********"

# resim call-function $package_address Scrilla instantiate_scrilla_module
component_addresses=($(resim call-function $package_address Scrilla instantiate_scrilla_module | awk -F"Component: " '{print $2}'))

export user_manangement_component_address=${component_addresses[0]}
export price_oracle_component_address=${component_addresses[1]}
export scrilla_component_address=${component_addresses[2]}
echo -e "\e[7m****** Echo user management component address ********"
echo $user_manangement_component_address
echo -e "\e[7m****** Echo price oracle component address ********"
echo $price_oracle_component_address
echo -e "\e[7m****** Echo scrilla component address ********"
echo $scrilla_component_address


# starting price for XRD
xrd_price1=$"0.05"

# accounts 1 and 2 deposit 990 xrd collateral each
account1_xrd_collateral_deposit=$"980"
account2_xrd_collateral_deposit=$"990"

# accounts 1 and 2 then borrow 41 USDS each against their 990 xrd collateral
account1_usds_borrow_amount="40"
account2_usds_borrow_amount="41"

# XRD price increases to .10
xrd_price2=$"0.10"

# THIS NEEDS TO HAPPEN STILL
#accounts 1 and 2 deposit all borrowed USDS into shield pool
account1_usds_shield_deposit_amount="20"
account2_usds_shield_deposit_amount="40"

# account 3 deposits 990 XRD collateral
account3_xrd_collateral_deposit=$"977"
account4_xrd_collateral_deposit=$"984"

# account 3 borrows 81 USDS against 990 collateral
account3_usds_borrow_amount="81"
account4_usds_borrow_amount="81"

# XRD price increases to .07
xrd_price3=$"0.08"

# account 1 now wants to redeem 20 usds that wasnt deposited in shield
account1_usds_redemption_amount="20"



# # ACCOUNT 1
echo -e "\e[7m****** creating a new user for account1 ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/new_user.rtm
echo ""
resim run ./manifests/new_user.rtm
# resim call-method $scrilla_component_address new_user --manifest new_user.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/new_user.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

# # Storing Scrilla user NFT address
echo -e "\e[7m****** Echoing "scrilla_user_nft_address" ********"
# Saving Resource Addresses from Scrilla User NFT
scrilla_user_nft_address=$(resim show $XRD_ACCNT1 | grep '"Scrilla User"' | awk -F": " '{print $3}' | awk -F ", " '{print $1}')
echo $scrilla_user_nft_address
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_xrd_to_collateral for account1" ********"
echo -e "\e[7m****** Account 1 adds 980 XRD to collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account1_xrd_collateral_deposit'")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
resim run ./manifests/add_xrd_to_collateral.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("'$account1_xrd_collateral_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "borrow_usds for account1" ********"
echo -e "\e[7m****** Account 1 borrows 40 USDS against this 980 XRD collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("{amount_to_borrow}")/Decimal("'$account1_usds_borrow_amount'")/g' ./manifests/borrow_usds.rtm
echo ""
resim run ./manifests/borrow_usds.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("'$account1_usds_borrow_amount'")/Decimal("{amount_to_borrow}")/g' ./manifests/borrow_usds.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

# # Storing usds address
usds_address=$(resim show $XRD_ACCNT1 | grep '"USD-Scrilla"' | awk -F": " '{print $3}' | awk -F ", " '{print $1}')
echo -e "\e[7m****** Echoing usds address *******"
echo $usds_address

echo -e "\e[7m****** Running RTM "add_usds_to_shield" for account1 ********"
echo -e "\e[7m****** Account 1 adds 20 USDS to sheild pool ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account1_usds_shield_deposit_amount'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{usds_address}")/ResourceAddress("'$usds_address'")/g' ./manifests/add_usds_to_shield.rtm
echo ""
resim run ./manifests/add_usds_to_shield.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("'$account1_usds_shield_deposit_amount'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$usds_address'")/ResourceAddress("{usds_address}")/g' ./manifests/add_usds_to_shield.rtm
echo ""
echo ""
echo ""
echo ""
echo ""


resim show $XRD_ACCNT1

# # ACCOUNT 2
# Set default account to xrd_accnt2
resim set-default-account $XRD_ACCNT2 $XRD_ACCNT2_priv $owner_badge_address:#1#

echo -e "\e[7m****** Running RTM "new_user" for account2 ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/new_user.rtm
echo ""
resim run ./manifests/new_user.rtm
# resim call-method $scrilla_component_address new_user --manifest new_user.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/new_user.rtm
echo ""
echo ""
echo ""
echo ""
echo ""


echo -e "\e[7m****** Running RTM "add_xrd_to_collateral for account2" ********"
echo -e "\e[7m****** Account 2 adds 990 XRD to collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account2_xrd_collateral_deposit'")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
resim run ./manifests/add_xrd_to_collateral.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("'$account2_xrd_collateral_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
echo ""
echo ""
echo ""
echo ""


echo -e "\e[7m****** Running RTM "borrow_usds for account2" ********"
echo -e "\e[7m****** Account 1 borrows 41 USDS against these 990 XRD ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("{amount_to_borrow}")/Decimal("'$account2_usds_borrow_amount'")/g' ./manifests/borrow_usds.rtm
echo ""
resim run ./manifests/borrow_usds.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("'$account2_usds_borrow_amount'")/Decimal("{amount_to_borrow}")/g' ./manifests/borrow_usds.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_usds_to_shield" for account2 ********"
echo -e "\e[7m****** Account 1 adds 41 USDS to sheild pool ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account2_usds_shield_deposit_amount'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{usds_address}")/ResourceAddress("'$usds_address'")/g' ./manifests/add_usds_to_shield.rtm
echo ""
resim run ./manifests/add_usds_to_shield.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("'$account2_usds_shield_deposit_amount'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$usds_address'")/ResourceAddress("{usds_address}")/g' ./manifests/add_usds_to_shield.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "set_price" moving xrd price to .10 ********"

sed -i 's/Decimal("{set_price}")/Decimal("'$xrd_price2'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/set_price.rtm
#sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/set_price.rtm
echo ""
resim run ./manifests/set_price.rtm
# resim call-method $scrilla_component_address set_price "0.10" --manifest set_price.rtm
echo ""
sed -i 's/Decimal("'$xrd_price2'")/Decimal("{set_price}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/set_price.rtm
#sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/set_price.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

# # ACCOUNT 3
# Set default account to xrd_accnt3
resim set-default-account $XRD_ACCNT3 $XRD_ACCNT3_priv $owner_badge_address:#1#

echo -e "\e[7m****** Running RTM "new_user" for account3 ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT3'")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/new_user.rtm
echo ""
resim run ./manifests/new_user.rtm
# resim call-method $scrilla_component_address new_user --manifest new_user.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT3'")/ComponentAddress("{account}")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/new_user.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_xrd_to_collateral for account3" ********"
echo -e "\e[7m****** Account 3 adds 977 XRD to collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT3'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account3_xrd_collateral_deposit'")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
resim run ./manifests/add_xrd_to_collateral.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT3'")/ComponentAddress("{account}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("'$account3_xrd_collateral_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "borrow_usds for account3" ********"
echo -e "\e[7m****** Account 3 borrows 81 USDS against this 977 XRD collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT3'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("{amount_to_borrow}")/Decimal("'$account3_usds_borrow_amount'")/g' ./manifests/borrow_usds.rtm
echo ""
resim run ./manifests/borrow_usds.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT3'")/ComponentAddress("{account}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("'$account3_usds_borrow_amount'")/Decimal("{amount_to_borrow}")/g' ./manifests/borrow_usds.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

# # ACCOUNT 4
# Set default account to xrd_accnt4
resim set-default-account $XRD_ACCNT4 $XRD_ACCNT4_priv $owner_badge_address:#1#

echo -e "\e[7m****** Running RTM "new_user" for account4 ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT4'")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/new_user.rtm
echo ""
resim run ./manifests/new_user.rtm
# resim call-method $scrilla_component_address new_user --manifest new_user.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT4'")/ComponentAddress("{account}")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/new_user.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_xrd_to_collateral for account4" ********"
echo -e "\e[7m****** Account 4 adds 984 XRD to collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT4'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account4_xrd_collateral_deposit'")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
resim run ./manifests/add_xrd_to_collateral.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT4'")/ComponentAddress("{account}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("'$account4_xrd_collateral_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "borrow_usds for account4" ********"
echo -e "\e[7m****** Account 4 borrows 81 USDS against this 984 XRD collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT4'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("{amount_to_borrow}")/Decimal("'$account4_usds_borrow_amount'")/g' ./manifests/borrow_usds.rtm
echo ""
resim run ./manifests/borrow_usds.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT4'")/ComponentAddress("{account}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("'$account4_usds_borrow_amount'")/Decimal("{amount_to_borrow}")/g' ./manifests/borrow_usds.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "set_price" moving xrd price to .09 ********"

sed -i 's/Decimal("{set_price}")/Decimal("'$xrd_price3'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT4'")/g' ./manifests/set_price.rtm
echo ""
resim run ./manifests/set_price.rtm
# resim call-method $scrilla_component_address set_price "0.10" --manifest set_price.rtm
echo ""
sed -i 's/Decimal("'$xrd_price3'")/Decimal("{set_price}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT4'")/ComponentAddress("{account}")/g' ./manifests/set_price.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** ACCOUNT 1 BEFORE REDEEMING 20 USDS WORTH 250 XRD ********"
resim show $XRD_ACCNT1

echo -e "\e[7m****** Showing all stats BEFORE REDEEMING 20 USDS WORTH 250 XRD ********"

# manually updating all loans
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#1#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#2#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#3#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#4#"

# resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#1#"
echo -e "\e[7m****** Account 1 ********"
resim call-method $scrilla_component_address show_info "#1#"
echo -e "\e[7m****** Account 2 ********"
resim call-method $scrilla_component_address show_info "#2#"
echo -e "\e[7m****** Account 3 ********"
resim call-method $scrilla_component_address show_info "#3#"
echo -e "\e[7m****** Account 4 ********"
resim call-method $scrilla_component_address show_info "#4#"
echo ""
echo ""
echo ""
echo ""
echo ""

# # ACCOUNT 1
# Set default account to xrd_accnt1
echo -e "\e[7m****** Setting default account to account1 ********"
resim set-default-account $XRD_ACCNT1 $XRD_ACCNT1_priv $owner_badge_address:#1#
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "redeem_usds" for account1 for 20 USDS ********"
echo -e "\e[7m****** This method is crucial to maintaining the USDS peg to 1 USD.  It allows anyone (even those not participating or taking loans from the platform) who has USDS to exchange it for the exact market rate of XRD at any time.  This method redeems it against the loans that are closest to liquidation (in this case it is redeemed against loans from account 3 and account 4).  You can compare data given before and after this method call to see that hte XRD collateral for accounts 3 and 4 have been lowered and the cooresponding proportional amounts of USDS have been repaid for them to allow the user calling the redeem_usds method to get their XRD in exchange for USDS.  ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/redeem_usds.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/redeem_usds.rtm
sed -i 's/Decimal("{amount_to_redeem}")/Decimal("'$account1_usds_redemption_amount'")/g' ./manifests/redeem_usds.rtm
sed -i 's/ResourceAddress("{usds_address}")/ResourceAddress("'$usds_address'")/g' ./manifests/redeem_usds.rtm
echo ""
#resim call-method $scrilla_component_address redeem_usds 20,$usds_address --manifest redeem_usd2.rtm
resim run ./manifests/redeem_usds.rtm 
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/redeem_usds.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/redeem_usds.rtm
sed -i 's/Decimal("'$account1_usds_redemption_amount'")/Decimal("{amount_to_redeem}")/g' ./manifests/redeem_usds.rtm
sed -i 's/ResourceAddress("'$usds_address'")/ResourceAddress("{usds_address}")/g' ./manifests/redeem_usds.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** ACCOUNT 1 AFTER REDEEMING 20 USDS WORTH 250 XRD ********"
resim show $XRD_ACCNT1

# manually updating all loans
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#1#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#2#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#3#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#4#"

echo -e "\e[7m****** Showing all stats AFTER redeeming 20 USDS ********"
# resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#1#"
echo -e "\e[7m****** Account 1 ********"
resim call-method $scrilla_component_address show_info "#1#"
echo -e "\e[7m****** Account 2 ********"
resim call-method $scrilla_component_address show_info "#2#"
echo -e "\e[7m****** Account 3 ********"
resim call-method $scrilla_component_address show_info "#3#"
echo -e "\e[7m****** Account 4 ********"
resim call-method $scrilla_component_address show_info "#4#"
