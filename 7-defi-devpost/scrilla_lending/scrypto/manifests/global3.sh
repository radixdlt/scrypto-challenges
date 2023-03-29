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

# Create account #7
echo ""
echo -e "\e[7m Storing Accnt 7 credentials into cache"
account7_creds=($(resim new-account | awk -F": " '{print $2,$4,$7}'))
export XRD_ACCNT7=${account7_creds[0]}
export XRD_ACCNT7_pub=${account7_creds[1]}
export XRD_ACCNT7_priv=${account7_creds[2]}
# resim show $XRD_ACCNT7

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

# Getting the component addresses for the other components instantiated from within Scrilla
export user_manangement_component_address=${component_addresses[0]}
export price_oracle_component_address=${component_addresses[1]}
export scrilla_component_address=${component_addresses[2]}
echo -e "\e[7m****** Echo user management component address ********"
echo $user_manangement_component_address
echo -e "\e[7m****** Echo price oracle component address ********"
echo $price_oracle_component_address
echo -e "\e[7m****** Echo scrilla component address ********"
echo $scrilla_component_address


###### Setting variables for this scenario ######

# used later for targeting specific NFTs for liquidation
user_id1=$"#1#"
user_id2=$"#2#"
user_id3=$"#3#"
user_id4=$"#4#"
user_id5=$"#5#"
user_id6=$"#6#"

# starting price for XRD .05
xrd_price1=$"0.05"

# accounts 1 and 2 deposit 991 & 990 xrd collateral each
account1_xrd_collateral_deposit=$"991"
account2_xrd_collateral_deposit=$"990"

# accounts 1 and 2 then borrow 43 USDS each against their 990 xrd collateral
account1_usds_borrow_amount="43"
account2_usds_borrow_amount="44"

#accounts 1 and 2 deposit all borrowed USDS into shield pool
account1_usds_shield_deposit_amount="20"
account2_usds_shield_deposit_amount="40"

# XRD price increases to .08
xrd_price2=$"0.08"

# account 3 deposits 989 XRD collateral
account3_xrd_collateral_deposit=$"989"

# account 3 borrows 65 USDS against 989 collateral
account3_usds_borrow_amount="65"

#account 3 deposit all borrowed USDS into shield pool
account3_usds_shield_deposit_amount="65"

# XRD price increases to .10
xrd_price3=$"0.10"

# account 4 borrows 60 USDS against 990 collateral
account4_xrd_collateral_deposit=$"992"

# account 4 borrows 60 USDS against 990 collateral
account4_usds_borrow_amount="85"

#account 4 deposit all borrowed USDS into shield pool
account4_usds_shield_deposit_amount="85"

# XRD price increases to .12
xrd_price4=$"0.12"

# account 5 deposits 990 XRD collateral
account5_xrd_collateral_deposit=$"989"

# account 5 borrows 60 USDS against 990 collateral
account5_usds_borrow_amount="105"

#account 5 deposit all borrowed USDS into shield pool
account5_usds_shield_deposit_amount="105"

# XRD price increases to .14
xrd_price5=$"0.14"

# account 6 deposits 990 XRD collateral
account6_xrd_collateral_deposit=$"995"

# account 6 borrows 60 USDS against 990 collateral
account6_usds_borrow_amount="125"

#account 6 deposit all borrowed USDS into shield pool
account6_usds_shield_deposit_amount="125"

account1_scrilla_deposit=$"18"
account2_scrilla_deposit=$"9"
account6_xrd_collateral_withdraw=$"100"
account7_xrd_collateral_deposit=$"995"

echo ""
echo ""
echo ""
echo ""
echo ""
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
echo -e "\e[7m****** Account1 is adding 991 XRD to collateral  ********"

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
echo -e "\e[7m****** XRD price starts off at '$'0.05. Account1 is borrowing 43 USDS against the 991 XRD collateral  ********"

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
echo -e "\e[7m****** Account1 adds 20 of its 42 USDS to the shield pool ********"

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

resim show $XRD_ACCNT1
echo ""
echo ""
echo ""
echo ""
echo ""
echo -e "\e[7m****** Changing account2 to default account ********"
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
echo -e "\e[7m****** Account2 adds 990 XRD to collateral  ********"

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
echo -e "\e[7m****** Account2 borrows 44 USDS against its 991 XRD collateral ********"

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
echo -e "\e[7m****** Account2 now adds 40 of its 44 USDS to the Shield pool.  Account 1 has 20 USDS in shield
giving it 1/3 ownership and account2 2/3 ownership of Shield pool ********"

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

echo -e "\e[7m****** Running RTM "set_price" moving xrd price to .08 ********"

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

echo -e "\e[7m****** Setting Account3 as default account ********"

# # ACCOUNT 3
# Set default account to xrd_accnt3
resim set-default-account $XRD_ACCNT3 $XRD_ACCNT3_priv $owner_badge_address:#1#
echo ""
echo ""
echo ""
echo ""
echo ""

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
echo -e "\e[7m****** Account3 borrows 65 USDS against the 989 XRD collateral ********"

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

echo -e "\e[7m****** Running RTM "add_usds_to_shield" for account3 ********"
echo -e "\e[7m****** Account3 deposits all 65 USDS into the shield pool ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT3'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account3_usds_shield_deposit_amount'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{usds_address}")/ResourceAddress("'$usds_address'")/g' ./manifests/add_usds_to_shield.rtm
echo ""
resim run ./manifests/add_usds_to_shield.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT3'")/ComponentAddress("{account}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("'$account3_usds_shield_deposit_amount'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$usds_address'")/ResourceAddress("{usds_address}")/g' ./manifests/add_usds_to_shield.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "set_price" moving xrd price to .10 ********"

sed -i 's/Decimal("{set_price}")/Decimal("'$xrd_price3'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/set_price.rtm
#sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/set_price.rtm
echo ""
resim run ./manifests/set_price.rtm
# resim call-method $scrilla_component_address set_price "0.10" --manifest set_price.rtm
echo ""
sed -i 's/Decimal("'$xrd_price3'")/Decimal("{set_price}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/set_price.rtm
#sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/set_price.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Setting Account4 as default account ********"
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
echo -e "\e[7m****** Account4 borrows 85 USDS against the 992 XRD collateral ********"

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

echo -e "\e[7m****** Running RTM "add_usds_to_shield" for account4 ********"
echo -e "\e[7m****** Account4 deposits all 85 USDS into the shield pool ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT4'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account4_usds_shield_deposit_amount'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{usds_address}")/ResourceAddress("'$usds_address'")/g' ./manifests/add_usds_to_shield.rtm
echo ""
resim run ./manifests/add_usds_to_shield.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT4'")/ComponentAddress("{account}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("'$account4_usds_shield_deposit_amount'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$usds_address'")/ResourceAddress("{usds_address}")/g' ./manifests/add_usds_to_shield.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "set_price" moving xrd price to .12 ********"

sed -i 's/Decimal("{set_price}")/Decimal("'$xrd_price4'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT4'")/g' ./manifests/set_price.rtm
echo ""
resim run ./manifests/set_price.rtm
# resim call-method $scrilla_component_address set_price "0.10" --manifest set_price.rtm
echo ""
sed -i 's/Decimal("'$xrd_price4'")/Decimal("{set_price}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT4'")/ComponentAddress("{account}")/g' ./manifests/set_price.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Setting Account5 as default account ********"
# # ACCOUNT 5
# Set default account to xrd_accnt5
resim set-default-account $XRD_ACCNT5 $XRD_ACCNT5_priv $owner_badge_address:#1#

echo -e "\e[7m****** Running RTM "new_user" for account5 ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT5'")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/new_user.rtm
echo ""
resim run ./manifests/new_user.rtm
# resim call-method $scrilla_component_address new_user --manifest new_user.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT5'")/ComponentAddress("{account}")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/new_user.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_xrd_to_collateral for account5" ********"
echo -e "\e[7m****** Account5 adds 989 XRD to collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT5'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account5_xrd_collateral_deposit'")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
resim run ./manifests/add_xrd_to_collateral.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT5'")/ComponentAddress("{account}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("'$account5_xrd_collateral_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "borrow_usds for account5" ********"
echo -e "\e[7m****** Account5 borrows 105 USDS against the 989 XRD collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT5'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("{amount_to_borrow}")/Decimal("'$account5_usds_borrow_amount'")/g' ./manifests/borrow_usds.rtm
echo ""
resim run ./manifests/borrow_usds.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT5'")/ComponentAddress("{account}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("'$account5_usds_borrow_amount'")/Decimal("{amount_to_borrow}")/g' ./manifests/borrow_usds.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_usds_to_shield" for account5 ********"
echo -e "\e[7m****** Account5 adds all 105 USDS to the sheild pool ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT5'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account5_usds_shield_deposit_amount'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{usds_address}")/ResourceAddress("'$usds_address'")/g' ./manifests/add_usds_to_shield.rtm
echo ""
resim run ./manifests/add_usds_to_shield.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT5'")/ComponentAddress("{account}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("'$account5_usds_shield_deposit_amount'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$usds_address'")/ResourceAddress("{usds_address}")/g' ./manifests/add_usds_to_shield.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "set_price" moving xrd price to .12 ********"

sed -i 's/Decimal("{set_price}")/Decimal("'$xrd_price5'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT5'")/g' ./manifests/set_price.rtm
echo ""
resim run ./manifests/set_price.rtm
# resim call-method $scrilla_component_address set_price "0.10" --manifest set_price.rtm
echo ""
sed -i 's/Decimal("'$xrd_price5'")/Decimal("{set_price}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT5'")/ComponentAddress("{account}")/g' ./manifests/set_price.rtm

echo -e "\e[7m****** Setting Account6 as default account ********"
# # ACCOUNT 6
# Set default account to xrd_accnt6
resim set-default-account $XRD_ACCNT6 $XRD_ACCNT6_priv $owner_badge_address:#1#
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "new_user" for account6 ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT6'")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/new_user.rtm
echo ""
resim run ./manifests/new_user.rtm
# resim call-method $scrilla_component_address new_user --manifest new_user.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT6'")/ComponentAddress("{account}")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/new_user.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_xrd_to_collateral for account6" ********"
echo -e "\e[7m****** Account6 adds 995 XRD to collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT6'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account6_xrd_collateral_deposit'")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
resim run ./manifests/add_xrd_to_collateral.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT6'")/ComponentAddress("{account}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("'$account6_xrd_collateral_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "borrow_usds for account6" ********"
echo -e "\e[7m****** Account6 borrows 125 USDS against the 995 XRD collateral ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT6'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("{amount_to_borrow}")/Decimal("'$account6_usds_borrow_amount'")/g' ./manifests/borrow_usds.rtm
echo ""
resim run ./manifests/borrow_usds.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT6'")/ComponentAddress("{account}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/borrow_usds.rtm
sed -i 's/Decimal("'$account6_usds_borrow_amount'")/Decimal("{amount_to_borrow}")/g' ./manifests/borrow_usds.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "add_usds_to_shield" for account6 ********"
echo -e "\e[7m****** Account6 deposits all 125 USDS into the shield pool ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT6'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account6_usds_shield_deposit_amount'")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("{usds_address}")/ResourceAddress("'$usds_address'")/g' ./manifests/add_usds_to_shield.rtm
echo ""
resim run ./manifests/add_usds_to_shield.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT6'")/ComponentAddress("{account}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/Decimal("'$account6_usds_shield_deposit_amount'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_usds_to_shield.rtm
sed -i 's/ResourceAddress("'$usds_address'")/ResourceAddress("{usds_address}")/g' ./manifests/add_usds_to_shield.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "set_price" moving xrd price to .12 ********"
echo -e "\e[7m****** Here we are lowering the XRD price to put account 6 just below liquidation price ********"

sed -i 's/Decimal("{set_price}")/Decimal("'$xrd_price4'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT6'")/g' ./manifests/set_price.rtm
echo ""
resim run ./manifests/set_price.rtm
# resim call-method $scrilla_component_address set_price "0.10" --manifest set_price.rtm
echo ""
sed -i 's/Decimal("'$xrd_price4'")/Decimal("{set_price}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT6'")/ComponentAddress("{account}")/g' ./manifests/set_price.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

# user tries to withdraw XRD before getting liquidated but fails because the user's collateralization rate is already
# below what is needed to maintain the loan

echo -e "\e[7m****** Running RTM "remove_xrd_from_collateral" ********"
echo -e "\e[7m****** Now from account 6 we are trying to remove just a small portion of the collateral.  As you can see, this fails because
the collateralization rate after removing this amount XRD would result in a rate below liquidation levels. ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT6'")/g' ./manifests/remove_xrd_from_collateral.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/remove_xrd_from_collateral.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/remove_xrd_from_collateral.rtm
sed -i 's/Decimal("{amount_to_remove}")/Decimal("'$account6_xrd_collateral_deposit'")/g' ./manifests/remove_xrd_from_collateral.rtm
echo ""
resim run ./manifests/remove_xrd_from_collateral.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT6'")/ComponentAddress("{account}")/g' ./manifests/remove_xrd_from_collateral.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/remove_xrd_from_collateral.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/remove_xrd_from_collateral.rtm
sed -i 's/Decimal("'$account6_xrd_collateral_deposit'")/Decimal("{amount_to_remove}")/g' ./manifests/remove_xrd_from_collateral.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Setting default account back to account1 ********"
# # ACCOUNT 1
# Set default account to xrd_accnt1
resim set-default-account $XRD_ACCNT1 $XRD_ACCNT1_priv $owner_badge_address:#1#

echo -e "\e[7m****** Running RTM "call_liquidation" on account6 (NFT ID #6#) ********"
echo -e "\e[7m****** Account 1 then liquidates account 6 in order to gain the Scrilla token bounty on the liquidation ********"

sed -i 's/NonFungibleLocalId("{user_id}")/NonFungibleLocalId("'$user_id6'")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/call_liquidation.rtm
echo ""
resim run ./manifests/call_liquidation.rtm
echo ""
sed -i 's/NonFungibleLocalId("'$user_id6'")/NonFungibleLocalId("{user_id}")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/call_liquidation.rtm

echo -e "\e[7m****** Showing Account 6 after being liquidated ********"
resim call-method $scrilla_component_address show_info "#6#"

# # Storing scrilla address
scrilla_address=$(resim show $XRD_ACCNT1 | grep '"Scrilla Token"' | awk -F": " '{print $3}' | awk -F ", " '{print $1}')
echo -e "\e[7m****** Echoing scrilla address *******"
echo $scrilla_address
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Running RTM "stake scrilla"  for account1 ********"
echo -e "\e[7m****** Account 1 then decides to stake 18 of these scrilla tokens ********"

sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account1_scrilla_deposit'")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ResourceAddress("{scrilla_address}")/ResourceAddress("'$scrilla_address'")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/stake_scrilla.rtm
echo ""
resim run ./manifests/stake_scrilla.rtm
echo ""
sed -i 's/Decimal("'$account1_scrilla_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ResourceAddress("'$scrilla_address'")/ResourceAddress("{scrilla_address}")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/stake_scrilla.rtm
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Setting default account back to account2 ********"
# # ACCOUNT 2
# Set default account to xrd_accnt2
resim set-default-account $XRD_ACCNT2 $XRD_ACCNT2_priv $owner_badge_address:#1#

# claim shield rewards for account 2 to make sure account 2 has Scrilla tokens

echo -e "\e[7m****** Running RTM "withdraw_shield_deposit_and_rewards"  for account2 ********"
echo -e "\e[7m****** Account 2 decides to withdraw all remaining shield deposit and rewards.  Account 2 started by depositing
 40 USDS into shield.  The balance to withdraw is now down to 29.617 but has earned 98.06 XRD in shield rewards ********"

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

echo -e "\e[7m****** Running RTM "stake scrilla"  for account2 ********"
echo -e "\e[7m****** Account2 decides to stake 9 of its earned scrilla tokens ********"

sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account2_scrilla_deposit'")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ResourceAddress("{scrilla_address}")/ResourceAddress("'$scrilla_address'")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/stake_scrilla.rtm
echo ""
resim run ./manifests/stake_scrilla.rtm
echo ""
sed -i 's/Decimal("'$account2_scrilla_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ResourceAddress("'$scrilla_address'")/ResourceAddress("{scrilla_address}")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/stake_scrilla.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/stake_scrilla.rtm
echo ""
echo ""
echo ""
echo ""
echo ""
# Now we will move price back to .10 so we can liquidate account 5
echo -e "\e[7m****** Running RTM "set_price" moving xrd price to .10 ********"

sed -i 's/Decimal("{set_price}")/Decimal("'$xrd_price3'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/set_price.rtm
echo ""
resim run ./manifests/set_price.rtm
# resim call-method $scrilla_component_address set_price "0.10" --manifest set_price.rtm
echo ""
sed -i 's/Decimal("'$xrd_price3'")/Decimal("{set_price}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/set_price.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/set_price.rtm
echo ""
echo ""
echo ""
echo ""
echo ""
# Now account 2 will call liquidation on account 5
echo -e "\e[7m****** Running RTM "call_liquidation" on account5 (NFT ID #5#) ********"
echo -e "\e[7m****** Account 5 XRD collateral is now dispersed to all those deposited into the shield pool (including account5!) ********"

sed -i 's/NonFungibleLocalId("{user_id}")/NonFungibleLocalId("'$user_id5'")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/call_liquidation.rtm
echo ""
resim run ./manifests/call_liquidation.rtm
echo ""
sed -i 's/NonFungibleLocalId("'$user_id5'")/NonFungibleLocalId("{user_id}")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/call_liquidation.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/call_liquidation.rtm

echo -e "\e[7m****** Showing Account 5 after being liquidated ********"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#5#"
resim call-method $scrilla_component_address show_info "#5#"

# Do something here to trigger platform fees like deposit XRD on account 7
# Set default account to xrd_accnt7
resim set-default-account $XRD_ACCNT7 $XRD_ACCNT7_priv $owner_badge_address:#1#

echo -e "\e[7m****** creating a new user for account7 ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT7'")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/new_user.rtm
echo ""
resim run ./manifests/new_user.rtm
# resim call-method $scrilla_component_address new_user --manifest new_user.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT7'")/ComponentAddress("{account}")/g' ./manifests/new_user.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/new_user.rtm
echo ""
echo ""
echo ""
echo ""
echo ""
echo -e "\e[7m****** Running RTM "add_xrd_to_collateral for account7" ********"
echo -e "\e[7m****** Account 7 adds 995 XRD to collateral.  This will trigger the Scrilla component to collect more fees now that 
accounts 1 and 2 have staked Scrilla and can earn a portion of the platform fees ********"

sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT7'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("{amount_to_deposit}")/Decimal("'$account7_xrd_collateral_deposit'")/g' ./manifests/add_xrd_to_collateral.rtm
echo ""
resim run ./manifests/add_xrd_to_collateral.rtm
echo ""
sed -i 's/ComponentAddress("'$XRD_ACCNT7'")/ComponentAddress("{account}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/add_xrd_to_collateral.rtm
sed -i 's/Decimal("'$account7_xrd_collateral_deposit'")/Decimal("{amount_to_deposit}")/g' ./manifests/add_xrd_to_collateral.rtm

# Set default account to xrd_accnt2
echo -e "\e[7m****** Setting Account 2 as default account ********"
resim set-default-account $XRD_ACCNT2 $XRD_ACCNT2_priv $owner_badge_address:#1#

echo -e "\e[7m****** Showing Account 2 before withdrawing Scrilla stake and rewards ********"
resim call-method $scrilla_component_address show_info "#2#"
resim show $XRD_ACCNT2
echo ""
echo ""
echo ""
echo ""
echo ""
echo -e "\e[7m****** Running RTM "unstake scrilla and withdraw rewards" on account2 ********"

sed -i 's/ResourceAddress("{scrilla_address}")/ResourceAddress("'$scrilla_address'")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT2'")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
echo ""
resim run ./manifests/unstake_scrilla_and_claim_rewards.rtm
echo ""
sed -i 's/ResourceAddress("'$scrilla_address'")/ResourceAddress("{scrilla_address}")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT2'")/ComponentAddress("{account}")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm

echo -e "\e[7m****** Showing Account 2 after withdrawing Scrilla stake and rewards ********"
resim call-method $scrilla_component_address show_info "#2#"
resim show $XRD_ACCNT2

# Now claim Scrilla staking rewards from account 1
echo ""
echo ""
echo ""
echo ""
echo ""

echo -e "\e[7m****** Showing Account 1 before withdrawing Scrilla stake and rewards ********"
resim call-method $scrilla_component_address show_info "#1#"
resim show $XRD_ACCNT1

# Set default account to xrd_accnt1
resim set-default-account $XRD_ACCNT1 $XRD_ACCNT1_priv $owner_badge_address:#1#

echo -e "\e[7m******We are now going to withdraw the scrilla staking deposit that account 1 made previously
 to check that the running totals of collected fees are propertly distributed between accounts 1 and 2********"

echo -e "\e[7m****** Running RTM "unstake scrilla and withdraw rewards" on account1********"
echo -e "\e[7m****** As you can see from the logs within this method call or the account details shown after, account 1 
earned 1/3 of the total fees from the scrilla stake pool while account 2 earned 2/3.  This cooresponds to the ownership
of the pool between those two accounts showing that the scalable reward tracking for Scrilla staking is working correctly********"

sed -i 's/ResourceAddress("{scrilla_address}")/ResourceAddress("'$scrilla_address'")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ComponentAddress("{account}")/ComponentAddress("'$XRD_ACCNT1'")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ComponentAddress("{scrilla_component_address}")/ComponentAddress("'$scrilla_component_address'")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ResourceAddress("{scrilla_user_nft_address}")/ResourceAddress("'$scrilla_user_nft_address'")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
echo ""
resim run ./manifests/unstake_scrilla_and_claim_rewards.rtm
echo ""
sed -i 's/ResourceAddress("'$scrilla_address'")/ResourceAddress("{scrilla_address}")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ComponentAddress("'$XRD_ACCNT1'")/ComponentAddress("{account}")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ComponentAddress("'$scrilla_component_address'")/ComponentAddress("{scrilla_component_address}")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm
sed -i 's/ResourceAddress("'$scrilla_user_nft_address'")/ResourceAddress("{scrilla_user_nft_address}")/g' ./manifests/unstake_scrilla_and_claim_rewards.rtm


echo -e "\e[7m****** Showing Account 1 after unstaking Scrilla and claiming rewards ********"
resim call-method $scrilla_component_address show_info "#1#"
resim show $XRD_ACCNT1

# # manually updating all NFTs (this would be done automatically when a user interacts with the dapp)
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#1#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#2#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#3#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#4#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#5#"
resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#6#"

echo -e "\e[7m****** Showing all final user data ********"
# resim call-method $scrilla_component_address update_collateralization_and_liquidation_value "#1#"
echo -e "\e[7m****** Account 1 ********"
resim call-method $scrilla_component_address show_info "#1#"
echo -e "\e[7m****** Account 2 ********"
resim call-method $scrilla_component_address show_info "#2#"
echo -e "\e[7m****** Account 3 ********"
resim call-method $scrilla_component_address show_info "#3#"
echo -e "\e[7m****** Account 4 ********"
resim call-method $scrilla_component_address show_info "#4#"
echo -e "\e[7m****** Account 5 ********"
resim call-method $scrilla_component_address show_info "#5#"
echo -e "\e[7m****** Account 6 ********"
resim call-method $scrilla_component_address show_info "#6#"


















































