source ./report_a_claim.sh

#admin agrees to pay following the claim
resim set-default-account $RAD_INSURANCE_ADMIN_ADDRESS $RAD_INSURANCE_ADMIN_PVKEY $RAD_INSURANCE_ADMIN_NONFUNGIBLEGLOBALID
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_ADMIN_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_ADMIN_ADDRESS\") \"create_proof\"  ResourceAddress(\"$RAD_INSURANCE_ADMIN_BADGE\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\") \"make_claim_as_accepted\"  $RAD_INSURANCE_POLICY_ID_U128 NonFungibleLocalId(\"$RAD_INSURANCE_INSURED_CLAIM_BADGE_ID\") ;" >> tx.rtm

resim run tx.rtm 

rm tx.rtm

