source ./create_components.sh

#
resim set-default-account $AIRDROP_REGISTER_ADDRESS_CYOVER  $AIRDROP_REGISTER_PVKEY_CYOVER
#tweeter account cyover subscribe to the airdrop
echo 'account cyover subscribe to the airdrop'
echo "CALL_METHOD ComponentAddress(\"$AIRDROP_WITH_TWEETER_ORACLE_COMPONENT\") \"register\" \"cyover\" ;" > tx.rtm
echo "CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress(\"$AIRDROP_REGISTER_ADDRESS_CYOVER\") \"deposit_batch\";" >> tx.rtm
resim run tx.rtm

# tweeter account cyrolsi subscribe to the airdrop
echo 'tweeter account cyrolsi subscribe to the airdrop'
resim set-default-account $AIRDROP_REGISTER_ADDRESS_CYROLSI  $AIRDROP_REGISTER_PVKEY_CYROLSI
echo "CALL_METHOD ComponentAddress(\"$AIRDROP_WITH_TWEETER_ORACLE_COMPONENT\") \"register\" \"cyrolsi\" ;" > tx.rtm
echo "CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress(\"$AIRDROP_REGISTER_ADDRESS_CYROLSI\") \"deposit_batch\";" >> tx.rtm
resim run tx.rtm


#cyover has completed alls tasks need by the airdrop (like radixdlt, like and reweet tweet1)  in contrast to cyrolsi 
echo 'cyover has completed alls tasks need by the airdrop (like radixdlt, like and reweet tweet1)  in contrast to cyrolsi '
resim set-default-account $TWEETER_ORACLE_ADMIN_ADDRESS  $TWEETER_ORACLE_ADMIN_PVKEY
#inserting datas by TWEETER_ORACLE_ADMIN_ADDRESS
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_ADMIN_ADDRESS\") \"create_proof_by_amount\" Decimal(\"1\") ResourceAddress(\"$TWEETER_ORACLE_ADMIN_BADGE\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"insert_account_followers\" \"radixdlt\" HashSet<String>(\"cyover\",\"ade\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"insert_tweets_likers\" \"tweet1\" HashSet<String>(\"cyover\",\"cyrolsi\",\"vivi\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"insert_tweets_retweeters\" \"tweet1\" HashSet<String>(\"cyover\",\"cyrolsi\");" >> tx.rtm
resim run tx.rtm


#AIRDROP_ADMIN_ADDRESS  find and store airdrop recipients and finalize it
echo 'AIRDROP_ADMIN_ADDRESS  find and store airdrop recipients and finalize it'
resim set-default-account $AIRDROP_ADMIN_ADDRESS  $AIRDROP_ADMIN_PVKEY
echo "CALL_METHOD ComponentAddress(\"$AIRDROP_ADMIN_ADDRESS\")  \"create_proof_by_amount\" Decimal(\"1\") ResourceAddress(\"$AIRDROP_WITH_TWEETER_ORACLE_ADMIN_BADGE\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$AIRDROP_WITH_TWEETER_ORACLE_COMPONENT\") \"find_and_store_airdrop_recipients\" ;" >> tx.rtm
resim run tx.rtm

echo "CALL_METHOD ComponentAddress(\"$AIRDROP_ADMIN_ADDRESS\")  \"create_proof_by_amount\" Decimal(\"1\") ResourceAddress(\"$AIRDROP_WITH_TWEETER_ORACLE_ADMIN_BADGE\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$AIRDROP_ADMIN_ADDRESS\") \"withdraw\" ResourceAddress(\"$XRD\");" >> tx.rtm
echo "TAKE_FROM_WORKTOP_BY_AMOUNT Decimal(\"1000\") ResourceAddress(\"$XRD\") Bucket(\"xrd_bucket\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$AIRDROP_WITH_TWEETER_ORACLE_COMPONENT\") \"finalize_airdrop\" Bucket(\"xrd_bucket\") ;" >> tx.rtm
echo "CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress(\"$AIRDROP_ADMIN_ADDRESS\") \"deposit_batch\";" >> tx.rtm
resim run tx.rtm
rm tx.rtm


#withdraw

#cyover withdrawal is a success
echo 'cyover withdrawal is a success '
resim show $AIRDROP_REGISTER_ADDRESS_CYOVER 
resim set-default-account $AIRDROP_REGISTER_ADDRESS_CYOVER  $AIRDROP_REGISTER_PVKEY_CYOVER
resim call-method $AIRDROP_WITH_TWEETER_ORACLE_COMPONENT "withdraw" 1,$AIRDROP_WITH_TWEETER_ORACLE_PARTICIPANT_BADGE
resim show $AIRDROP_REGISTER_ADDRESS_CYOVER 


#cyrolsi withdrawal is a failure
echo 'cyrolsi withdrawal is a failure'
resim show $AIRDROP_REGISTER_ADDRESS_CYROLSI
resim set-default-account $AIRDROP_REGISTER_ADDRESS_CYROLSI  $AIRDROP_REGISTER_PVKEY_CYROLSI
resim call-method $AIRDROP_WITH_TWEETER_ORACLE_COMPONENT "withdraw" 1,$AIRDROP_WITH_TWEETER_ORACLE_PARTICIPANT_BADGE
resim show $AIRDROP_REGISTER_ADDRESS_CYROLSI
