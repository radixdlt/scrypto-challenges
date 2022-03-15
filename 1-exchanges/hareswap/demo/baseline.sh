#!/usr/bin/env sh
#set -x
set -e

# Setup a baseline environment with 2 users each holding a resource they would
# trade.
#
# To avoid parsing resim output I've manually set the variables based on their
# (deterministic) values.
#
# The "Maker" has a pre-swap setup phase where they make sure they have an
# account to fund trades from and instantate the Maker Component to handle
# exzecution of their orders by a sender.  This only needs to be done once, no
# matter how many trades they sign of any resource type.
#
# The "Taker" has a pre-swap setup phase where they nominate (or create) a
# unique resource they own to act as a badge to prevent frontrunning the
# transaction submission.  Note their transaction signing key could be used as
# a virtual badge for this purpose to avoid a pre-swap ledger interaction by
# the Taker.  But, I'm choosing to be explicit for readability/flexibility to
# show this need not be directly tied to a given public key.  Anyone with the
# badge could submit the signed order.

# pretty up the CLI output
source ./logging.sh

# path to the hare cli tool (make sure to run the examples from the "testing" directory)
HARE=../hare/target/debug/hare

##############################

log "Setting up baseline"

log "start with fresh ledger state"
resim reset

# initial
log "publish HareSwap"
resim publish ../target/wasm32-unknown-unknown/release/hareswap.wasm
PACKAGE=0124c5afc33cf45c06633d8fc0b0dfba2c82f14ec82ff7eb13483c

# 0.0 taker
log "Taker Baseline"

# baseline taker account
log "Taker account"
resim new-account
ACCOUNT1=02e1bbfc1eb7b1fa431c9ae0b1f7ee66660a52adf2739f621ce424
ACCOUNT1_PUBKEY=006b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b
# baseline "T" tokens
log "Taker 'T' tokens T1000 seems like a good amount"
resim new-token-fixed 1000 --symbol T
T=03d527faee6d0b91e7c1bab500c6a986e5777a25d704acc288d542

# 0.1 taker
# hareswap-specific: create a taker_auth to prevent frontrunning when submitted maker-signed orders
log "Taker: Create badge for Hareswap submission frontrunning avoidance"
resim new-badge-fixed 1 --name TAKER_AUTH
TAKER_AUTH=0347dfe3a58e8a630305f2f3df82949cd70ce49e2cde097b259f8d

log "Maker baseline"

# 0.0 maker
# maker baseline new account
log "Maker account (a System Account)"
resim new-account
ACCOUNT2=022ab83d6a41454e5cf04a5442cf70acf5fb19af0c8938fadfe141
ACCOUNT2_PUBKEY=00ef2d127de37b942baad06145e54b0c619a1f22327b2ebbcfbec78f5564afe39d
resim set-default-account $ACCOUNT2 $ACCOUNT2_PUBKEY
# baseline "M" tokens
log "Maker 'M' tokens M1000 seems like a good amount"
resim new-token-fixed 1000 --symbol M
M=0398652f4eb36dd2067191845deb68e54771074f35dc78fbf820a4

log "Maker HareSwap pre-swap one-time setup"

# 0.1 maker: account setup
# new badge for access to SharedAccount
log "Create badges for access to a SharedAccount"
resim new-badge-fixed 2 --name shared_account_auth
MAKER_ACCOUNT_AUTH=031773788de8e4d2947d6592605302d4820ad060ceab06eb2d4711
# create the accont
log "Create the SharedAccount"
resim call-function $PACKAGE "SharedAccount" "new_easy" $MAKER_ACCOUNT_AUTH
MAKER_ACCOUNT=02d9e04ba122de13a58f80ea7a06a0e1aad665d23cbeb124c3c286
# put half the M in there ready to trade
log "Move M500 into the SharedAccount for use with HareSwap"
resim transfer 500,$M $MAKER_ACCOUNT

log "simulate a reasonable non-zero epoch for testing"

# for consistency testing
resim set-current-epoch 20

