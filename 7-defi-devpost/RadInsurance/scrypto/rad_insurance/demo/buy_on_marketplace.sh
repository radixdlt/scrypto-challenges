source ./list_on_marketplace.sh
#A person buys the insurer's investment and, in return, receives the insurer badge
resim set-default-account $RAD_INSURANCE_BUYER_ADDRESS $RAD_INSURANCE_BUYER_PVKEY $RAD_INSURANCE_BUYER_NONFUNGIBLEGLOBALID

echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_BUYER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_BUYER_ADDRESS\") \"withdraw_by_amount\" Decimal(\"500\") ResourceAddress(\"$XRD\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"2\") ResourceAddress(\"$XRD\") Bucket(\"service_fee\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"448\") ResourceAddress(\"$XRD\") Bucket(\"payment_amount\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_COMPONENT_ADDRESS\")  \"buy_on_marketplace\" $RAD_INSURANCE_POLICY_ID_U128 Bucket(\"payment_amount\") Bucket(\"service_fee\") NonFungibleLocalId(\"$RAD_INSURANCE_INSURER_LISTING_BADGE_ID\") ;" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$RAD_INSURANCE_BUYER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm

resim show $RAD_INSURANCE_BUYER_ADDRESS

rm tx.rtm