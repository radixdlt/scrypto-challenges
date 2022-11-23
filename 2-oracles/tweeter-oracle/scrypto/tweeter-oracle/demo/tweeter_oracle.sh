source ./create_components.sh

#Inserting datas to update
# Any users can call these methods
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"add_accounts_to_follows\" Vec<String>(\"radixdlt\");" > tx.rtm
resim run tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"add_tweets_to_like\" Vec<String>(\"tweet1\");" > tx.rtm
resim run tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"add_tweets_to_retweet\" Vec<String>(\"tweet1\");" > tx.rtm
resim run tx.rtm
# get datas to updates
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"get_datas_to_update\";" > tx.rtm
resim run tx.rtm

resim set-default-account $TWEETER_ORACLE_ADMIN_ADDRESS  $TWEETER_ORACLE_ADMIN_PVKEY
#inserting datas by TWEETER_ORACLE_ADMIN_ADDRESS
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_ADMIN_ADDRESS\") \"create_proof_by_amount\" Decimal(\"1\") ResourceAddress(\"$TWEETER_ORACLE_ADMIN_BADGE\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"insert_account_followers\" \"radixdlt\" HashSet<String>(\"cyover\",\"ade\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"insert_tweets_likers\" \"tweet1\" HashSet<String>(\"cyover\",\"cyrolsi\",\"vivi\");" >> tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"insert_tweets_retweeters\" \"tweet1\" HashSet<String>(\"cyover\",\"cyrolsi\");" >> tx.rtm
resim run tx.rtm


#checking datas after inserting
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"is_account_follower\" \"radixdlt\" \"cyover\";" > tx.rtm
resim run tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"is_tweet_liker\" \"tweet1\" \"cyover\";" > tx.rtm
resim run tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"is_tweet_liker\" \"tweet1\" \"titi\";" > tx.rtm
resim run tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"is_tweet_retweeter\" \"tweet1\" \"cyrolsi\";" > tx.rtm
resim run tx.rtm
#cyover user follow radixdlt, like and retweet tweet1

resim set-default-account $TWEETER_ORACLE_ADMIN_ADDRESS  $TWEETER_ORACLE_ADMIN_PVKEY
#removing datas by TWEETER_ORACLE_ADMIN_ADDRESS
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_ADMIN_ADDRESS\") \"create_proof_by_amount\" Decimal(\"1\") ResourceAddress(\"$TWEETER_ORACLE_ADMIN_BADGE\");" > tx.rtm
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"remove_account_followers\" \"radixdlt\" HashSet<String>(\"cyover\");" >> tx.rtm
resim run tx.rtm

#checking data after removing
echo "CALL_METHOD ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\") \"is_account_follower\" \"radixdlt\" \"cyover\";" > tx.rtm
resim run tx.rtm

rm tx.rtm