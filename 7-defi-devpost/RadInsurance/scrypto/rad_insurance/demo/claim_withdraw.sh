source ./make_claim_as_accepted.sh
#The insured withdraws the amount corresponding to their claim.
resim set-default-account $RAD_INSURANCE_INSURED_ADDRESS $RAD_INSURANCE_INSURED_PVKEY $RAD_INSURANCE_INSURED_NONFUNGIBLEGLOBALID
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURED_CLAIM_BADGE\");" >> tx.rtm
echo "POP_FROM_AUTH_ZONE Proof(\"claim_proof\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURED_CLAIM_BADGE\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\")  \"claim_withdraw\" $RAD_INSURANCE_POLICY_ID_U128  Proof(\"claim_proof\") ;" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm

resim show $RAD_INSURANCE_INSURED_ADDRESS 

rm tx.rtm