#!/usr/bin/env sh
set -x
set -e

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

