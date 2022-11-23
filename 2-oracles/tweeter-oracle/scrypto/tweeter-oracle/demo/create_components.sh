source ./baseline.sh

# instanciating TweeterOracle Component by TWEETER_ORACLE_ADMIN_ADDRESS
resim set-default-account $TWEETER_ORACLE_ADMIN_ADDRESS  $TWEETER_ORACLE_ADMIN_PVKEY
out=`resim call-function $PACKAGE TweeterOracle instantiate_tweeter_oracle  | tee /dev/tty | awk '/Component:|Resource:/ {print $NF}'`
TWEETER_ORACLE_COMPONENT=`echo $out | cut -d " " -f1`
TWEETER_ORACLE_ADMIN_BADGE=`echo $out | cut -d " " -f2`

#instanciating AirdropWithTweeterOracle Compoenent by AIRDROP_ADMIN_ADDRESS
resim set-default-account $AIRDROP_ADMIN_ADDRESS  $AIRDROP_ADMIN_PVKEY
echo "CALL_FUNCTION PackageAddress(\"$PACKAGE\") \"AirdropWithTweeterOracle\" \"new\" ResourceAddress(\"030000000000000000000000000000000000000000000000000004\") Vec<String>(\"radixdlt\") Vec<String>(\"tweet1\") Vec<String>(\"tweet1\") ComponentAddress(\"$TWEETER_ORACLE_COMPONENT\");" > tx.rtm
echo "CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress(\"$AIRDROP_ADMIN_ADDRESS\") \"deposit_batch\";" >> tx.rtm
RESULT=$(resim run "tx.rtm")



export AIRDROP_WITH_TWEETER_ORACLE_COMPONENT=$(echo "$RESULT" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export AIRDROP_WITH_TWEETER_ORACLE_ADMIN_BADGE=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')
export AIRDROP_WITH_TWEETER_ORACLE_PARTICIPANT_BADGE=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '3!d')


