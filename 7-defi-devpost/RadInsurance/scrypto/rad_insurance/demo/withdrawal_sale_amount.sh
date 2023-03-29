source ./buy_on_marketplace.sh


#The insurer can retrieve the amount from the sale.

resim set-default-account $RAD_INSURANCE_INSURER_ADDRESS $RAD_INSURANCE_INSURER_PVKEY $RAD_INSURANCE_INSURER_NONFUNGIBLEGLOBALID

resim show $RAD_INSURANCE_INSURER_ADDRESS

echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_LISTING_BADGE\");" >> tx.rtm
echo "POP_FROM_AUTH_ZONE Proof(\"insurer_listing_proof\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"create_proof\" ResourceAddress(\"$RAD_INSURANCE_INSURER_LISTING_BADGE\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\") \"withdrawal_sale_amount\"  $RAD_INSURANCE_POLICY_ID_U128   Proof(\"insurer_listing_proof\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_INSURER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm

resim show $RAD_INSURANCE_INSURER_ADDRESS

rm tx.rtm