source ./invest_as_insurer.sh

#echo $RAD_INSURANCE_INSURER_BADGE
#A few months later the insurers decided to recover his interest
resim set-current-time 2023-09-10T23:01:50Z

echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm 
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_BADGE\");" >> tx.rtm
echo "POP_FROM_AUTH_ZONE Proof(\"insurer_proof\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_BADGE\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\")  \"rewards_withdrawal\" $RAD_INSURANCE_POLICY_ID_U128 Proof(\"insurer_proof\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm

resim show $RAD_INSURANCE_INSURER_ADDRESS 

rm tx.rtm