source ./rewards_withdrawal.sh

#The insurer, in need of liquidity, decides to sell their investment and lists it on the marketplace
resim set-default-account $RAD_INSURANCE_INSURER_ADDRESS $RAD_INSURANCE_INSURER_PVKEY $RAD_INSURANCE_INSURER_NONFUNGIBLEGLOBALID
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
# echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_BADGE\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"withdraw_by_amount\" Decimal(\"2\") ResourceAddress(\"$XRD\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"withdraw_by_amount\" Decimal(\"1\") ResourceAddress(\"$RAD_INSURANCE_INSURER_BADGE\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"$XRD\") Bucket(\"service_fee\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"1\") ResourceAddress(\"$RAD_INSURANCE_INSURER_BADGE\") Bucket(\"insurer_bucket_to_list\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\") \"list_on_marketplace\"  $RAD_INSURANCE_POLICY_ID_U128  Bucket(\"insurer_bucket_to_list\") Bucket(\"service_fee\") Decimal(\"400\") ;" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

RESULT=$(resim run "tx.rtm")

RAD_INSURANCE_INSURER_LISTING_BADGE_ID=$(echo "$RESULT" | sed -nr "s/.*listing_badge_id: ([[:graph:]_]+)/\1/p") 

resim show $RAD_INSURANCE_INSURER_ADDRESS 

rm tx.rtm