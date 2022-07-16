# Getting the current script dir
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)


# Resetting resim
resim reset


# Create Accounts
echo -e "\n\nCreate test admin account"
ADMIN_ACCOUNT_DETAILS=$(resim new-account)
admin_account=$(echo "$ADMIN_ACCOUNT_DETAILS" | grep "Account component address" | cut -d " " -f4)
admin_private_key=$(echo "$ADMIN_ACCOUNT_DETAILS" | grep "Private key" | cut -d " " -f3)
export admin_account
export admin_private_key
echo "admin_account=$admin_account"
echo "admin_private_key=$admin_private_key"

echo -e "\n\nCreate test account 1"
ACCOUNT_1_DETAILS=$(resim new-account)
account_1=$(echo "$ACCOUNT_1_DETAILS" | grep "Account component address" | cut -d " " -f4)
account_1_private_key=$(echo "$ACCOUNT_1_DETAILS" | grep "Private key" | cut -d " " -f3)
export account_1
export account_1_private_key
echo "account_1=$account_1"
echo "account_1_private_key=$account_1_private_key"

echo -e "\n\nCreate test account 2"
ACCOUNT_2_DETAILS=$(resim new-account)
account_2=$(echo "$ACCOUNT_2_DETAILS" | grep "Account component address" | cut -d " " -f4)
account_2_private_key=$(echo "$ACCOUNT_2_DETAILS" | grep "Private key" | cut -d " " -f3)
export account_2
export account_2_private_key
echo "account_2=$account_2"
echo "account_2_private_key=$account_2_private_key"


# Store XRD as a variable
echo -e "\n\nStore XRD address as variable"
xrd=$(resim show "$admin_account" | grep XRD | cut -d " " -f7 | cut -d "," -f1)
export xrd
echo "xrd=$xrd"


# Build Application
echo -e "\n\nBuild app"
package=$(resim publish . | grep Package | cut -d " " -f4)
export package
echo "package=$package"


# Create instance of app using admin account
echo -e "\n\nCreate instance of app"
resim set-default-account "$admin_account" "$admin_private_key"
lending_pool=$(resim call-function "$package" LendingPlatform instantiate_lending_platform | grep Component | tail -1 | cut -d " " -f3)
lending_platform_admin_badge=$(resim show "$admin_account" | grep 'Lending Platform Admin Badge' | cut -d " " -f7 | cut -d "," -f1)
export lending_pool
export lending_platform_admin_badge
echo "lending_pool=$lending_pool"
echo "lending_platform_admin_badge=$lending_platform_admin_badge"


# Register Users
export REPLACEMENT_LOOKUP_ACCOUNT_CREATION=" \
    s/<<<lending_platform_component_address>>>/$lending_pool/g; \
    s/<<<account_1_address>>>/$account_1/g; \
    s/<<<account_2_address>>>/$account_2/g; \
"
sed "$REPLACEMENT_LOOKUP_OTHER" "$SCRIPT_DIR"/transactions_raw/account_creation_user_1.rtm >"$SCRIPT_DIR"/transactions/account_creation_user_1.rtm
sed "$REPLACEMENT_LOOKUP_OTHER" "$SCRIPT_DIR"/transactions_raw/account_creation_user_2.rtm >"$SCRIPT_DIR"/transactions/account_creation_user_2.rtm

echo -e "\n\nRegister User 1"
resim set-default-account "$account_1" "$account_1_private_key"
resim run "$SCRIPT_DIR"/transactions/account_creation_user_1.rtm
account_1_lending_platform_badge=$(resim show "$account_1" | grep 'Lending Platform User Badge' | cut -d " " -f7 | cut -d "," -f1)
export account_1_lending_platform_badge
echo "account_1_lending_platform_badge=$account_1_lending_platform_badge"

echo -e "\n\nRegister User 2"
resim set-default-account "$account_2" "$account_2_private_key"
resim run "$SCRIPT_DIR"/transactions/account_creation_user_2.rtm
account_2_lending_platform_badge=$(resim show "$account_2" | grep 'Lending Platform User Badge' | cut -d " " -f7 | cut -d "," -f1)
export account_2_lending_platform_badge
echo "account_2_lending_platform_badge=$account_2_lending_platform_badge"


# Run sample transactions
export REPLACEMENT_LOOKUP_OTHER=" \
    s/<<<xrd_token>>>/$xrd/g; \
    s/<<<lending_platform_component_address>>>/$lending_pool/g; \
    s/<<<admin_account_address>>>/$admin_account/g; \
    s/<<<lending_platform_admin_badge>>>/$lending_platform_admin_badge/g; \
    s/<<<account_1_address>>>/$account_1/g; \
    s/<<<account_1_lending_platform_badge>>>/$account_1_lending_platform_badge/g; \
    s/<<<account_2_address>>>/$account_2/g; \
    s/<<<account_2_lending_platform_badge>>>/$account_2_lending_platform_badge/g; \
"
sed "$REPLACEMENT_LOOKUP_OTHER" "$SCRIPT_DIR"/transactions_raw/add_xrd_to_lending_pool.rtm >"$SCRIPT_DIR"/transactions/add_xrd_to_lending_pool.rtm
sed "$REPLACEMENT_LOOKUP_OTHER" "$SCRIPT_DIR"/transactions_raw/transactions_user_1.rtm >"$SCRIPT_DIR"/transactions/transactions_user_1.rtm
sed "$REPLACEMENT_LOOKUP_OTHER" "$SCRIPT_DIR"/transactions_raw/transactions_user_2.rtm >"$SCRIPT_DIR"/transactions/transactions_user_2.rtm

echo -e "\n\nAdd XRD to lending pool"
resim set-default-account "$admin_account" "$admin_private_key"
resim run "$SCRIPT_DIR"/transactions/add_xrd_to_lending_pool.rtm

echo -e "\n\nTest User 1 Transactions"
resim set-default-account "$account_1" "$account_1_private_key"
resim run "$SCRIPT_DIR"/transactions/transactions_user_1.rtm

echo -e "\n\nTest User 2 Transactions"
resim set-default-account "$account_2" "$account_2_private_key"
resim run "$SCRIPT_DIR"/transactions/transactions_user_2.rtm
