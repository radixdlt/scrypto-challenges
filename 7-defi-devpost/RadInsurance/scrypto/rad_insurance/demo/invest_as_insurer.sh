source ./create_policies.sh

#user invest an xrd amount and get INSURER BADGE
resim set-default-account $RAD_INSURANCE_INSURER_ADDRESS $RAD_INSURANCE_INSURER_PVKEY $RAD_INSURANCE_INSURER_NONFUNGIBLEGLOBALID
resim set-current-time 2023-03-10T23:01:50Z
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
RAD_INSURANCE_POLICY_ID_U128=$RAD_INSURANCE_POLICY_ID'u128'
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\")  \"get_policy_info\" $RAD_INSURANCE_POLICY_ID_U128 ;" >> tx.rtm
resim run tx.rtm

echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"withdraw_by_amount\" Decimal(\"502\") ResourceAddress(\"$XRD\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"$XRD\") Bucket(\"service_fee\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"500\") ResourceAddress(\"$XRD\") Bucket(\"invest_amount\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\")  \"invest_as_insurer\" $RAD_INSURANCE_POLICY_ID_U128 Bucket(\"invest_amount\") Bucket(\"service_fee\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm


resim run tx.rtm

resim show $RAD_INSURANCE_INSURER_ADDRESS 


rm tx.rtm




