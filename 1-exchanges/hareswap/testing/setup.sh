#!/usr/bin/env sh
set -x
set -e

HARE=../hare/target/debug/hare

resim reset

# initial
resim publish ../target/wasm32-unknown-unknown/release/hareswap.wasm
PACKAGE=0124c5afc33cf45c06633d8fc0b0dfba2c82f14ec82ff7eb13483c

# 0.0 taker
# baseline taker account
resim new-account
ACCOUNT1=02e1bbfc1eb7b1fa431c9ae0b1f7ee66660a52adf2739f621ce424
ACCOUNT1_PUBKEY=006b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b
# baseline "T" tokens
resim new-token-fixed 1000
T=03d527faee6d0b91e7c1bab500c6a986e5777a25d704acc288d542

# 0.1 taker
# hareswap-specific: create a taker_auth to prevent frontrunning when submitted maker-signed orders
resim new-badge-fixed 1
TAKER_AUTH=0347dfe3a58e8a630305f2f3df82949cd70ce49e2cde097b259f8d

# 0.0 maker
# maker baseline new account
resim new-account
ACCOUNT2=022ab83d6a41454e5cf04a5442cf70acf5fb19af0c8938fadfe141
ACCOUNT2_PUBKEY=00ef2d127de37b942baad06145e54b0c619a1f22327b2ebbcfbec78f5564afe39d
resim set-default-account $ACCOUNT2 $ACCOUNT2_PUBKEY
# baseline "M" tokens
resim new-token-fixed 1000
M=0398652f4eb36dd2067191845deb68e54771074f35dc78fbf820a4

# 0.1 maker: account setup
# new badge for access to CustodialAccount
resim new-badge-fixed 2
# CallFunction { package_address: 010000000000000000000000000000000000000000000000000001, blueprint_name: "System", function: "new_resource", args: [Enum(0u8, {0u8}), HashMap<String, String>(), 0u64, 0u64, HashMap<Address, U64>(), Some(Enum(0u8, {Decimal("2")}))] }
MAKER_ACCOUNT_AUTH=031773788de8e4d2947d6592605302d4820ad060ceab06eb2d4711
# create the accont
resim call-function $PACKAGE "CustodialAccount" "new_easy" $MAKER_ACCOUNT_AUTH
MAKER_ACCOUNT=02d9e04ba122de13a58f80ea7a06a0e1aad665d23cbeb124c3c286
# put half the M in there ready to trade
resim transfer 500,$M $MAKER_ACCOUNT

### probably break the rest into seperate file later

# 0.2 Maker setup
MAKER_OFFLINE_KEY_PUB=maker.pub
MAKER_OFFLINE_KEY_PRI=maker.pri
pubkey_arg=$($HARE new-key-pair $MAKER_OFFLINE_KEY_PUB $MAKER_OFFLINE_KEY_PRI)
cat > maker_setup.rtm  <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account2_badge");
CALL_METHOD Address("$ACCOUNT2") "withdraw" Decimal("1") Address("$MAKER_ACCOUNT_AUTH") BucketRef("account2_badge");
TAKE_FROM_WORKTOP Decimal("1.0") Address("$MAKER_ACCOUNT_AUTH") Bucket("maker_account_auth");
CALL_FUNCTION Address("$PACKAGE") "Maker" "instantiate" $pubkey_arg None Address("$MAKER_ACCOUNT") Bucket("maker_account_auth");
EOF
rtmc --output maker_setup.rtmc maker_setup.rtm
resim run --trace maker_setup.rtm > maker_setup.trace 2>&1
MAKER_COMPONENT=$(tail -n1 maker_setup.trace | cut -d' ' -f3)
rm maker_setup.trace

# switch to taker
resim set-default-account $ACCOUNT1 $ACCOUNT1_PUBKEY

## TODO off-ledger stuff agree on order
#taker make RFQ
TAKER_AMOUNT=100
$HARE request-for-quote $TAKER_AMOUNT $T $M $TAKER_AUTH > partial_order.txt
# simulate send to maker
# maker decide on price and sign order
MAKER_AMOUNT=200
$HARE make-signed-order partial_order.txt $MAKER_AMOUNT $MAKER_COMPONENT $MAKER_OFFLINE_KEY_PRI > signed_order.txt
SIGNED_ORDER=$(cat signed_order.txt)

## 4-A taker: OPTION 1 - simple execution
FN=taker_submit_option1.rtm
cat > $FN   <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge");
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("auth_for_exec");
CALL_METHOD Address("$ACCOUNT1") "withdraw" Decimal("$TAKER_AMOUNT") Address("$T") BucketRef("account_badge");
TAKE_ALL_FROM_WORKTOP Address("$T") Bucket("T");
CALL_METHOD Address("$MAKER_COMPONENT") "execute_order" $SIGNED_ORDER Bucket("T") BucketRef("auth_for_exec");
ASSERT_WORKTOP_CONTAINS Decimal("$MAKER_AMOUNT") Address("$M");
CALL_METHOD_WITH_ALL_RESOURCES Address("$ACCOUNT1") "deposit_batch";
EOF
rtmc --output ${FN}c $FN
resim run --trace $FN


