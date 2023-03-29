#!/usr/bin/env sh
set -x
set -e

resim reset

XRD=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety

# baseline Admin account
echo "Admin account"
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:|NonFungibleGlobalId:/ {print $NF}'`
echo $out
RAD_INSURANCE_ADMIN_ADDRESS=`echo $out | cut -d " " -f1`
RAD_INSURANCE_ADMIN_PUBKEY=`echo $out | cut -d " " -f2`
RAD_INSURANCE_ADMIN_PVKEY=`echo $out | cut -d " " -f3`
RAD_INSURANCE_ADMIN_NONFUNGIBLEGLOBALID=`resim new-simple-badge --name 'OwnerBadge' | awk '/NonFungibleGlobalId:/ {print $NF}'`

resim set-default-account $RAD_INSURANCE_ADMIN_ADDRESS $RAD_INSURANCE_ADMIN_PVKEY $RAD_INSURANCE_ADMIN_NONFUNGIBLEGLOBALID

PACKAGE=`resim publish ../target/wasm32-unknown-unknown/release/rad_insurance.wasm | tee /dev/tty | awk '/Package:/ {print $NF}'`
echo $PACKAGE

echo "Insurer Account"
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:|NonFungibleGlobalId:/ {print $NF}'`
RAD_INSURANCE_INSURER_ADDRESS=`echo $out | cut -d " " -f1`
RAD_INSURANCE_INSURER_PUBKEY=`echo $out | cut -d " " -f2`
RAD_INSURANCE_INSURER_PVKEY=`echo $out | cut -d " " -f3`
RAD_INSURANCE_INSURER_NONFUNGIBLEGLOBALID=`resim new-simple-badge --name 'OwnerBadge' | awk '/NonFungibleGlobalId:/ {print $NF}'`

echo "Insured Account"
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:|NonFungibleGlobalId:/ {print $NF}'`
RAD_INSURANCE_INSURED_ADDRESS=`echo $out | cut -d " " -f1`
RAD_INSURANCE_INSURED_PUBKEY=`echo $out | cut -d " " -f2`
RAD_INSURANCE_INSURED_PVKEY=`echo $out | cut -d " " -f3`
RAD_INSURANCE_INSURED_NONFUNGIBLEGLOBALID=`resim new-simple-badge --name 'OwnerBadge' | awk '/NonFungibleGlobalId:/ {print $NF}'`

echo "Insured Account"
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:|NonFungibleGlobalId:/ {print $NF}'`
RAD_INSURANCE_BUYER_ADDRESS=`echo $out | cut -d " " -f1`
RAD_INSURANCE_BUYER_PUBKEY=`echo $out | cut -d " " -f2`
RAD_INSURANCE_BUYER_PVKEY=`echo $out | cut -d " " -f3`
RAD_INSURANCE_BUYER_NONFUNGIBLEGLOBALID=`resim new-simple-badge --name 'OwnerBadge' | awk '/NonFungibleGlobalId:/ {print $NF}'`


#creating rad_insurance component
out=`resim call-function $PACKAGE RadInsurance instanciate_rad_insurance 2 $XRD | tee /dev/tty | awk '/Component:|Resource:/ {print $NF}'`
RAD_INSURANCE_COMPONENT_ADDRESS=`echo $out | cut -d " " -f1`
RAD_INSURANCE_ADMIN_BADGE=`echo $out | cut -d " " -f2`
RAD_INSURANCE_INSURER_BADGE=`echo $out | cut -d " " -f4`
RAD_INSURANCE_INSURED_BADGE=`echo $out | cut -d " " -f5`
RAD_INSURANCE_INSURED_CLAIM_BADGE=`echo $out | cut -d " " -f6`
RAD_INSURANCE_INSURER_LISTING_BADGE=`echo $out | cut -d " " -f7`

resim show $RAD_INSURANCE_ADMIN_ADDRESS



