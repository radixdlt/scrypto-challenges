source ./list_on_marketplace.sh
#The insurer decides to delist their sale.
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"withdraw_by_amount\" Decimal(\"2\") ResourceAddress(\"$XRD\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_LISTING_BADGE\");" >> tx.rtm
echo "POP_FROM_AUTH_ZONE Proof(\"insurer_listing_proof\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"$XRD\") Bucket(\"service_fee\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_LISTING_BADGE\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\") \"delist_on_marketplace\"  $RAD_INSURANCE_POLICY_ID_U128   Bucket(\"service_fee\") Proof(\"insurer_listing_proof\") ;" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm

resim show $RAD_INSURANCE_INSURER_ADDRESS 

rm tx.rtm