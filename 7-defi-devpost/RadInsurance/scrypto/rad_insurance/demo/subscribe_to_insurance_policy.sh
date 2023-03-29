source ./invest_as_insurer.sh 

# A person to purchase an insurance policy and get the Insured badge. 
resim set-default-account $RAD_INSURANCE_INSURED_ADDRESS $RAD_INSURANCE_INSURED_PVKEY $RAD_INSURANCE_INSURED_NONFUNGIBLEGLOBALID
resim set-current-time 2023-03-11T23:01:50Z
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"withdraw_by_amount\" Decimal(\"32\") ResourceAddress(\"$XRD\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"$XRD\") Bucket(\"service_fee\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"30\") ResourceAddress(\"$XRD\") Bucket(\"deposit_amount\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\")  \"subscribe_to_insurance_policy\" $RAD_INSURANCE_POLICY_ID_U128 Decimal(\"500\") Bucket(\"deposit_amount\") Bucket(\"service_fee\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURED_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
resim run tx.rtm

resim show $RAD_INSURANCE_INSURED_ADDRESS

rm tx.rtm