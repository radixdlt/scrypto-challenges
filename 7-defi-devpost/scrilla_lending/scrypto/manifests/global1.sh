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


user_id1=$"#1#"
user_id2=$"#2#"
user_id3=$"#3#"
user_id4=$"#4#"
user_id5=$"#5#"
user_id6=$"#6#"

# starting price for XRD
xrd_price1=$"0.05"

# accounts 1 and 2 deposit 990 xrd collateral each
account1_xrd_collateral_deposit=$"991"
account2_xrd_collateral_deposit=$"990"

# accounts 1 and 2 then borrow 41 USDS each against their 990 xrd collateral
account1_usds_borrow_amount=$"42"
account2_usds_borrow_amount=$"41"

# XRD price increases to .10
xrd_price2=$"0.10"

# THIS NEEDS TO HAPPEN STILL
#accounts 1 and 2 deposit all borrowed USDS into shield pool
account1_usds_shield_deposit_amount="20"
account2_usds_shield_deposit_amount="40"

# account 3 deposits 990 XRD collateral
account3_xrd_collateral_deposit=$"989"
account4_xrd_collateral_deposit=$"992"

# account 3 borrows 50 USDS against 990 collateral
account3_usds_borrow_amount=$"50"
# account 4 borrows 81 USDS against 990 collateral
account4_usds_borrow_amount=$"50"

# XRD price increases to .055
xrd_price3=$"0.055"

# This should put the two recent loans under 110% collateralization ratio
# We go ahead and liquidate account3



account2_scrilla_deposit=$"60"



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

echo -e "\e[7m****** Running RTM "add_xrd_to_collateral for account1" ********"
echo -e "\e[7m****** The starting price for XRD is 0.05.  Account1 adds 991 XRD to collateral.  There is a .5% fee that applies to XRD collateral deposts so 4.955 XRD was taken and stored inside the component fee vault ********"

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
echo -e "\e[7m****** Account1 borrows 42 USDS against 991 XRD to collateral.  As seen in the below data, this loan gives user 1 a collateralization rate of 117.4%.  This loan is safe from liquidation for the time being unless XRD drops below .046 as seen from the liquidation book entry that is paired with the NFT ID for account1, #1# ********"

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
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_usds_to_shield" for account1 ********"
echo -e "\e[7m****** Account1 adds 20 of the USDS borrowed to the Shield pool ********"

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
echo -e "\e[7m****** Account2 adds 990 XRD to collateral ********"

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
echo -e "\e[7m****** Account2 borrows 41 USDS against these 990 XRD collateral ********"

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
echo -e "\e[7m****** Account2 adds 40 of the USDS borrowed to the Shield pool, this gives account 1 1/3 ownership of the shield pool and account 2 2/3 ownership ********"

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
echo -e "\e[7m****** Account3 adds 989 XRD to collateral ********"


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
echo -e "\e[7m****** Account3 borrows 50 USDS against these 989 XRD collateral. As you can see from the data below, this loan gives user 3 a collateraliation rate of 196.81% since the xrd price previously moved up from .05 at the beginning to .10 per token.  You can also see that a new entry in the liquidation book has appeared for account3 which shows that this loan can be liquidated if XRD falls below .0559 per token ********"

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
echo -e "\e[7m****** Account4 adds 992 XRD to collateral ********"

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
echo -e "\e[7m****** Account4 borrows 50 USDS against these 992 XRD collateral ********"

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
echo -e "\e[7m****** Here we are moving the price of XRD back down in order to put some loans below their liquidation prices ********"

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

echo -e "\e[7m****** Running RTM "call_liquidation" on account3 (NFT ID #3#) ********"

sed -i 's/NonFungibleLocalId("{user_id}")/NonFungibleLocalId("'$user_id3'")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT4'")/g' ./manifests/call_liquidation.rtm
echo ""
resim run ./manifests/call_liquidation.rtm
echo ""
sed -i 's/NonFungibleLocalId("'$user_id3'")/NonFungibleLocalId("{user_id}")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT4'")/ComponentAddress("{account}")/g' ./manifests/call_liquidation.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** ACCOUNT 1 BEFORE CLAIMING SHIELD REWARDS ********"
resim show $XRD_ACCNT1

echo -e "\e[7m****** Showing all stats before claiming shield rewards ********"
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

# CLAIM SHIELD DEPOSITS AND REWARDS FOR ACCOUNTS 1 AND 2

# # ACCOUNT 1
# Set default account to xrd_accnt1
resim set-default-account $XRD_ACCNT1 $XRD_ACCNT1_priv $owner_badge_address:#1#

echo -e "\e[7m****** Running RTM "withdraw_shield_deposit_and_rewards"  for account1 ********"
echo -e "\e[7m****** As you can see from the data presented before and after this method call, account 1 receives 1/3 of the total amount of collateral that has been liquidated from account 3.  Account 2 will receive 2/3 of all the collateral from this liquidation after they claim and remove stake ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
echo ""
resim run ./manifests/withdraw_shield_deposit_and_rewards.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** ACCOUNT 1 AFTER CLAIMING SHIELD REWARDS ********"
resim show $XRD_ACCNT1

# # ACCOUNT 2
# Set default account to xrd_accnt2
resim set-default-account $XRD_ACCNT2 $XRD_ACCNT2_priv $owner_badge_address:#1#
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "withdraw_shield_deposit_and_rewards"  for account2 ********"
echo -e "\e[7m****** As you can see from the data presented before and after this method call, Account 2 receives 2/3 of all the collateral from this liquidation because this account owned 2/3 of the shield pool at time of liquidation.  You can see that any remainder of USDS in the shield pool was also distributed back to each user proportionally to ownership of the pool********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
echo ""
resim run ./manifests/withdraw_shield_deposit_and_rewards.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/withdraw_shield_deposit_and_rewards.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

# # Storing scrilla address
scrilla_address=$(resim show $XRD_ACCNT1 | grep '"Scrilla Token"' | awk -F": " '{print $3}' | awk -F ", " '{print $1}')
echo -e "\e[7m****** Echoing scrilla address *******"
echo $scrilla_address

# # Manually updating NFT data to display all results
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#1#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#2#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#3#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#4#"

# # Displaying all results
echo -e "\e[7m****** Account 1 has lost all but .50 of shield deposits, but shares all of the liquidated collateral with account 2 in the reward pool ********"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#1#"
resim call-method $scrilla_component_address show_info "#1#"
echo -e "\e[7m****** Account 2 has lost all but .50 of shield deposits, but shares all of the liquidated collateral with account 1 in the reward pool ********"
resim call-method $scrilla_component_address show_info "#2#"
echo -e "\e[7m****** Account 3 has now been liquidated.  Collateral, borrow balance reset to 0 ********"
resim call-method $scrilla_component_address show_info "#3#"
echo -e "\e[7m****** Account 4 has not been liquidated yet, but could be. ********"
resim call-method $scrilla_component_address show_info "#4#"



# echo -e "\e[7m****** Running RTM "repay_usds_loan" for account1 ********"
# # Loans can be repaid with the following unused method in this script

# sed -i "s/{account}/$(echo $XRD_ACCNT1)/g" ./manifests/repay_usds.rtm
# sed -i "s/{scrilla_user_nft_address}/$(echo $scrilla_user_nft_address)/g" ./manifests/repay_usds.rtm
# sed -i "s/{scrilla_component_address}/$(echo $scrilla_component_address)/g" ./manifests/repay_usds.rtm
# sed -i "s/{amount_to_repay}/$(echo $account1_usds_repay_amount)/g" ./manifests/repay_usds.rtm
# sed -i "s/{usds_address}/$(echo $usds_address)/g" ./manifests/repay_usds.rtm
# echo ""
# resim run ./manifests/repay_usds.rtm
# echo ""
# sed -i "s/$(echo $XRD_ACCNT1)/{account}/g" ./manifests/repay_usds.rtm
# sed -i "s/$(echo $scrilla_user_nft_address)/{scrilla_user_nft_address}/g" ./manifests/repay_usds.rtm
# sed -i "s/$(echo $scrilla_component_address)/{scrilla_component_address}/g" ./manifests/repay_usds.rtm
# sed -i "s/$(echo $account1_usds_repay_amount)/{amount_to_repay}/g" ./manifests/repay_usds.rtm
# sed -i "s/$(echo $usds_address)/{usds_address}/g" ./manifests/repay_usds.rtm





