#!/usr/bin/env sh
set -x

resim reset

# initial
resim publish ../target/wasm32-unknown-unknown/release/hareswap.wasm
PACKAGE=0124c5afc33cf45c06633d8fc0b0dfba2c82f14ec82ff7eb13483c

# 0.0 taker
resim new-account
ACCOUNT1=02e1bbfc1eb7b1fa431c9ae0b1f7ee66660a52adf2739f621ce424
ACCOUNT1_PUBKEY=006b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b

resim new-token-fixed 1000
T=03d527faee6d0b91e7c1bab500c6a986e5777a25d704acc288d542

# 0.1 taker
resim new-badge-fixed 1
TAKER_AUTH=0347dfe3a58e8a630305f2f3df82949cd70ce49e2cde097b259f8d

# 0.0 maker
resim new-account
ACCOUNT2=022ab83d6a41454e5cf04a5442cf70acf5fb19af0c8938fadfe141
ACCOUNT2_PUBKEY=00ef2d127de37b942baad06145e54b0c619a1f22327b2ebbcfbec78f5564afe39d
resim set-default-account $ACCOUNT2 $ACCOUNT2_PUBKEY

resim new-token-fixed 1000
M=0398652f4eb36dd2067191845deb68e54771074f35dc78fbf820a4

# 0.1 maker: account setup
resim new-badge-fixed 2
# CallFunction { package_address: 010000000000000000000000000000000000000000000000000001, blueprint_name: "System", function: "new_resource", args: [Enum(0u8, {0u8}), HashMap<String, String>(), 0u64, 0u64, HashMap<Address, U64>(), Some(Enum(0u8, {Decimal("2")}))] }
MAKER_ACCOUNT_AUTH=031773788de8e4d2947d6592605302d4820ad060ceab06eb2d4711
resim call-function $PACKAGE "CustodialAccount" "new_easy" $MAKER_ACCOUNT_AUTH
MAKER_ACCOUNT=02d9e04ba122de13a58f80ea7a06a0e1aad665d23cbeb124c3c286
resim transfer 500,$M $MAKER_ACCOUNT

### probably break the rest into seperate file later

# 0.2 Maker setup

# TODO: get a public key off-ledger
pubkey=0x1234
resim call-function $PACKAGE "Maker" "instantiate" $pubkey EMPTY_BUCKET $MAKER_ACCOUNT bucket_of_MAKER_ACCOUNT_AUTH
MAKER_COMPONENT=...


#resim call-function $HARESWAP_PACKAGE "Maker" "instantiate" public_key bucket component auth_bucket
