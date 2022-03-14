# meant to be sourced from one of the swap demos

log "Maker generate a key pair to sign orders off-ledger and build a transaction manifest to create a Component to support HareSwap: maker_setup_simple.rtm"
resim set-default-account $ACCOUNT2 $ACCOUNT2_PUBKEY
MAKER_OFFLINE_KEY_PUB=maker.pub
MAKER_OFFLINE_KEY_PRI=maker.pri
pubkey_arg=$($HARE new-key-pair $MAKER_OFFLINE_KEY_PUB $MAKER_OFFLINE_KEY_PRI)
FN=maker_setup_simple.rtm
cat > $FN  <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account2_badge");
CALL_METHOD Address("$ACCOUNT2") "withdraw" Decimal("1") Address("$MAKER_ACCOUNT_AUTH") BucketRef("account2_badge");
TAKE_FROM_WORKTOP Decimal("1.0") Address("$MAKER_ACCOUNT_AUTH") Bucket("maker_account_auth");
CALL_FUNCTION Address("$PACKAGE") "Maker" "instantiate" $pubkey_arg None Address("$MAKER_ACCOUNT") Bucket("maker_account_auth");
EOF
log "validate and run the transaction"
rtmc --output ${FN}c ${FN}
_resim run --trace ${FN} > ${FN}.trace 2>&1
# annoying to get the return values when using resim instead of Rust APIs against the ledger.  These are brittle.
MAKER_COMPONENT=$(tail -n1 ${FN}.trace | cut -d' ' -f3)
VOUCHER_ADDRESS=$(grep "INFO.*tokenized order resource address:" ${FN}.trace | cut -d':' -f2)
rm ${FN}.trace
