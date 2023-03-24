source ./baseline.sh

#creating policy by admin
# resim set-default-account $RAD_INSURANCE_ADMIN_ADDRESS $RAD_INSURANCE_ADMIN_PVKEY $RAD_INSURANCE_ADMIN_NONFUNGIBLEGLOBALID
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_ADMIN_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_ADMIN_ADDRESS\") \"withdraw_by_amount\" Decimal(\"500\") ResourceAddress(\"$XRD\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_ADMIN_ADDRESS\") \"create_proof\"  ResourceAddress(\"$RAD_INSURANCE_ADMIN_BADGE\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP ResourceAddress(\"$XRD\") Bucket(\"initial_liquidity\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\") \"create_policy\" \"insurance policy for radiwswap hack\" \"insurance policy for radiwswap hack\" Decimal(\"10\") Bucket(\"initial_liquidity\");" >> tx.rtm
echo "CLEAR_AUTH_ZONE;" >> tx.rtm
RESULT=$(resim run "tx.rtm")

RAD_INSURANCE_POLICY_ID=$(echo "$RESULT" | sed -nr "s/.*policy_id: ([[:alnum:]_]+)/\1/p") 
# resim set-default-account $RAD_INSURANCE_INSURER_ADDRESS $RAD_INSURANCE_INSURER_PVKEY $RAD_INSURANCE_INSURER_NONFUNGIBLEGLOBALID
# echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"lock_fee\" Decimal(\"1\");" > tx.rtm
# echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\") \"get_policies\" ;" >> tx.rtm
# # echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_POLICY_COMPONENT\") \"get_policy_info\" ;" >> tx.rtm
# resim run tx.rtm

resim show $RAD_INSURANCE_ADMIN_ADDRESS


# rm tx.rtm