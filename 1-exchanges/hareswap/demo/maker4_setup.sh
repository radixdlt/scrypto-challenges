# meant to be sourced from one of the swap demos

OP=$(resim new-account)
ACCOUNT4=$(echo "$OP" | sed -nr "s/.*Account address: ([[:alnum:]_]+)/\1/p")
ACCOUNT4_PUBKEY=$(echo "$OP" | sed -nr "s/.*Public key: ([[:alnum:]_]+)/\1/p")

resim set-default-account $ACCOUNT4 $ACCOUNT4_PUBKEY

log "Maker4 HareSwap pre-swap one-time setup"
# actually this could be done as many times as the Maker wants to derisk

log "Setup a SharedAccount"
####
# I'd expect at Babylon something like this SharedAccount will already exist
# for every user, so this wouldn't actually be part of a HareSwap setup,  But
# we still need it here.
###

log "Create badges for access to a SharedAccount"
OP=$(resim new-badge-fixed 2 --name shared_account_auth)
MAKER4_ACCOUNT_AUTH=$(echo "$OP" | sed -nr "s/.*ResourceDef: ([[:alnum:]_]+)/\1/p")

log "Create the SharedAccount"
OP=$(resim call-function $PACKAGE "SharedAccount" "new_easy" $MAKER4_ACCOUNT_AUTH)
MAKER4_ACCOUNT=$(echo "$OP" | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")

log "Get ready to handle quote requests on HareSwap"

log "Maker generate a key pair to sign orders off-ledger and build a transaction manifest to create a Component to support HareSwap: maker_setup_simple.rtm"
MAKER4_OFFLINE_KEY_PUB=maker4.pub
MAKER4_OFFLINE_KEY_PRI=maker4.pri
pubkey4_arg=$($HARE new-key-pair $MAKER4_OFFLINE_KEY_PUB $MAKER4_OFFLINE_KEY_PRI)
FN=maker4_setup_double_swap.rtm
cat > $FN  <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge");
CALL_METHOD Address("$ACCOUNT4") "withdraw" Decimal("1") Address("$MAKER4_ACCOUNT_AUTH") BucketRef("account_badge");
TAKE_FROM_WORKTOP Decimal("1.0") Address("$MAKER4_ACCOUNT_AUTH") Bucket("maker_account_auth");
CALL_FUNCTION Address("$PACKAGE") "Maker" "instantiate" $pubkey4_arg None Address("$MAKER4_ACCOUNT") Bucket("maker_account_auth");
EOF
log "validate and run the transaction"
rtmc --output ${FN}c ${FN}
_resim run --trace ${FN} > ${FN}.trace 2>&1
# annoying to get the return values when using resim instead of Rust APIs against the ledger.  These are brittle.
MAKER4_COMPONENT=$(tail -n1 ${FN}.trace | cut -d' ' -f3)
VOUCHER4_ADDRESS=$(grep "INFO.*tokenized order resource address:" ${FN}.trace | cut -d':' -f2)
rm ${FN}.trace
