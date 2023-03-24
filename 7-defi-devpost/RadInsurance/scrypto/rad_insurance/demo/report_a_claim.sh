source ./subscribe_to_insurance_policy.sh

resim set-default-account $RAD_INSURANCE_INSURED_ADDRESS $RAD_INSURANCE_INSURED_PVKEY $RAD_INSURANCE_INSURED_NONFUNGIBLEGLOBALID
# The Insured make a claim and get a claim badge.
resim set-current-time 2023-05-07T23:01:50Z
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURED_BADGE\");" >> tx.rtm
echo "POP_FROM_AUTH_ZONE Proof(\"insured_proof\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURED_BADGE\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\")  \"report_a_claim\" Proof(\"insured_proof\")  $RAD_INSURANCE_POLICY_ID_U128 \"Hack of RadixSwap which took place on 02/07/2023\" Decimal(\"200\") 1688329936i64;" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

RESULT=$(resim run "tx.rtm")

RAD_INSURANCE_INSURED_CLAIM_BADGE_ID=$(echo "$RESULT" | sed -nr "s/.*claim_badge_id: ([[:graph:]_]+)/\1/p") 

resim show $RAD_INSURANCE_INSURED_ADDRESS

# resim show $RAD_INSURANCE_COMPONENT_ADDRESS

rm tx.rtm