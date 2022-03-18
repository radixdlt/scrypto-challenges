# meant to be sourced from one of the swap demos

OP=$(resim new-account)
ACCOUNT3=$(echo "$OP" | sed -nr "s/.*Account address: ([[:alnum:]_]+)/\1/p")
ACCOUNT3_PUBKEY=$(echo "$OP" | sed -nr "s/.*Public key: ([[:alnum:]_]+)/\1/p")

resim set-default-account $ACCOUNT3 $ACCOUNT3_PUBKEY

log "Maker3 HareSwap pre-swap one-time setup"
# actually this could be done as many times as the Maker wants to derisk

log "Setup a SharedAccount"
####
# I'd expect at Babylon something like this SharedAccount will already exist
# for every user, so this wouldn't actually be part of a HareSwap setup,  But
# we still need it here.
###

log "Create badges for access to a SharedAccount"
OP=$(resim new-badge-fixed 2 --name shared_account_auth)
MAKER3_ACCOUNT_AUTH=$(echo "$OP" | sed -nr "s/.*ResourceDef: ([[:alnum:]_]+)/\1/p")

log "Create the SharedAccount"
OP=$(resim call-function $PACKAGE "SharedAccount" "new_easy" $MAKER3_ACCOUNT_AUTH)
MAKER3_ACCOUNT=$(echo "$OP" | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")

log "Get ready to handle quote requests on HareSwap"

log "Maker generate a key pair to sign orders off-ledger and build a transaction manifest to create a Component to support HareSwap: maker_setup_simple.rtm"
MAKER3_OFFLINE_KEY_PUB=maker3.pub
MAKER3_OFFLINE_KEY_PRI=maker3.pri
pubkey3_arg=$($HARE new-key-pair $MAKER3_OFFLINE_KEY_PUB $MAKER3_OFFLINE_KEY_PRI)
FN=maker3_setup_double_swap.rtm
cat > $FN  <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge");
CALL_METHOD Address("$ACCOUNT3") "withdraw" Decimal("1") Address("$MAKER3_ACCOUNT_AUTH") BucketRef("account_badge");
TAKE_FROM_WORKTOP Decimal("1.0") Address("$MAKER3_ACCOUNT_AUTH") Bucket("maker_account_auth");
CALL_FUNCTION Address("$PACKAGE") "Maker" "instantiate" $pubkey3_arg None Address("$MAKER3_ACCOUNT") Bucket("maker_account_auth");
EOF
log "validate and run the transaction"
rtmc --output ${FN}c ${FN}
_resim run --trace ${FN} > ${FN}.trace 2>&1
# annoying to get the return values when using resim instead of Rust APIs against the ledger.  These are brittle.
MAKER3_COMPONENT=$(tail -n1 ${FN}.trace | cut -d' ' -f3)
VOUCHER3_ADDRESS=$(grep "INFO.*tokenized order resource address:" ${FN}.trace | cut -d':' -f2)
rm ${FN}.trace
