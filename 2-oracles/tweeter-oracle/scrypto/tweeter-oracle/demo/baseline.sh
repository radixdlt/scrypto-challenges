#!/usr/bin/env sh
set -x
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


##############################


resim reset

XRD=030000000000000000000000000000000000000000000000000004
# initial
# resim publish ../target/wasm32-unknown-unknown/release/tweeter_oracle.wasm
PACKAGE=`resim publish ../target/wasm32-unknown-unknown/release/tweeter_oracle.wasm | tee /dev/tty | awk '/Package:/ {print $NF}'`
echo $PACKAGE

# baseline Oracle Admin account
echo "Oracle Admin account"
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:/ {print $NF}'`
TWEETER_ORACLE_ADMIN_ADDRESS=`echo $out | cut -d " " -f1`
TWEETER_ORACLE_ADMIN_PUBKEY=`echo $out | cut -d " " -f2`
TWEETER_ORACLE_ADMIN_PVKEY=`echo $out | cut -d " " -f3`

echo "AIRDROP_ADMIN account (a System Account)"
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:/ {print $NF}'`
AIRDROP_ADMIN_ADDRESS=`echo $out | cut -d " " -f1`
AIRDROP_ADMIN_PUBKEY=`echo $out | cut -d " " -f2`
AIRDROP_ADMIN_PVKEY=`echo $out | cut -d " " -f3`

out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:/ {print $NF}'`
AIRDROP_REGISTER_ADDRESS_CYOVER=`echo $out | cut -d " " -f1`
AIRDROP_REGISTER_UBKEY_CYOVER=`echo $out | cut -d " " -f2`
AIRDROP_REGISTER_PVKEY_CYOVER=`echo $out | cut -d " " -f3`


out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:/ {print $NF}'`
AIRDROP_REGISTER_ADDRESS_CYROLSI=`echo $out | cut -d " " -f1`
AIRDROP_REGISTER_UBKEY_CYROLSI=`echo $out | cut -d " " -f2`
AIRDROP_REGISTER_PVKEY_CYROLSI=`echo $out | cut -d " " -f3`

