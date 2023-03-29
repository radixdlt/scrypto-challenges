source ./invest_as_insurer.sh

#echo $RAD_INSURANCE_INSURER_BADGE
#user invest an xrd amount and get INSURER BADGE
resim set-current-time 2023-09-10T23:01:50Z

echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm 
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_BADGE\");" >> tx.rtm
echo "POP_FROM_AUTH_ZONE Proof(\"insurer_proof\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_BADGE\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\")  \"get_rewards\" $RAD_INSURANCE_POLICY_ID_U128 Proof(\"insurer_proof\");" >> tx.rtm
resim run tx.rtm
rm tx.rtm