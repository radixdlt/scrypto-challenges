#!/usr/bin/env sh
#set -x
set -e

# Uncomment the line below to trace all transactions
#TRACE=--trace

# setup the common baseline environment for this test
source ./baseline.sh

log "Simple Swap Example"

# 0.2 Maker setup
log "Maker generate a key pair to off-ldeger sign orders and build a transaction manifest to create a Component to support HareSwap: maker_setup.rtm"
resim set-default-account $ACCOUNT2 $ACCOUNT2_PUBKEY
MAKER_OFFLINE_KEY_PUB=maker.pub
MAKER_OFFLINE_KEY_PRI=maker.pri
pubkey_arg=$($HARE new-key-pair $MAKER_OFFLINE_KEY_PUB $MAKER_OFFLINE_KEY_PRI)
FN=maker_setup_simple_swap.rtm
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

log "Taker decides to get a Request-For-Quote using HareSwap"

# switch to taker
resim set-default-account $ACCOUNT1 $ACCOUNT1_PUBKEY

log "Taker requests: Whoever has TAKER_AUTH would like to buy 200.0 of M in exchange for T"
MAKER_AMOUNT=200.0
xlog $HARE request-for-quote buy-base partial_order.txt $MAKER_AMOUNT $M $T $TAKER_AUTH

log "simulate sending partial order to Maker"
log "Taker >>> partial_order.txt >>> Maker"

log "Maker accepts the order and decide to quote 100.0 T will be required to buy the 200.0 M asked."
log "The unique identifier for this order is 'A1' which is good until (and including) epoch 42"
TAKER_AMOUNT=100.0
VOUCHER_KEY=A1
DEADLINE_EPOCH=42
xlog $HARE make-signed-order partial_order.txt $TAKER_AMOUNT $MAKER_COMPONENT $VOUCHER_ADDRESS $VOUCHER_KEY $MAKER_OFFLINE_KEY_PRI $DEADLINE_EPOCH > signed_order.txt

log "simulate sending signed order back to Taker"
log "Maker >>> signed_order.txt >>> Taker"

log "Taker, unsurprisingly, decides to submit the order.  They construct a transaction manifest using the order"
log "The manifest 'simply' withdraws the correct amount from their account, call's the Maker Component function with signed order"
log "and CRITICALLY, verifies the return bucket has the right amount before depositing in their account"

SIGNED_ORDER=$(cat signed_order.txt)
FN=taker_submit_simple_swap.rtm
cat > $FN   <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge_t_auth");
CALL_METHOD Address("$ACCOUNT1") "withdraw" Decimal("1") Address("$TAKER_AUTH") BucketRef("account_badge_t_auth");
TAKE_ALL_FROM_WORKTOP Address("$TAKER_AUTH") Bucket("auth_for_exec_bucket");
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge_t");
CALL_METHOD Address("$ACCOUNT1") "withdraw" Decimal("$TAKER_AMOUNT") Address("$T") BucketRef("account_badge_t");
TAKE_ALL_FROM_WORKTOP Address("$T") Bucket("T");
CREATE_BUCKET_REF Bucket("auth_for_exec_bucket") BucketRef("auth_for_exec");
CALL_METHOD Address("$MAKER_COMPONENT") "execute_order" $SIGNED_ORDER Bucket("T") BucketRef("auth_for_exec");
ASSERT_WORKTOP_CONTAINS Decimal("$MAKER_AMOUNT") Address("$M");
CALL_METHOD_WITH_ALL_RESOURCES Address("$ACCOUNT1") "deposit_batch";
EOF

log "check the manifest syntax by compiling it"
xlog rtmc --output ${FN}c $FN && rm ${FN}c

log "submit the transaction to execute the trade"
resim run $TRACE $FN

success
log "look at the accounts:"

log "The Taker's System Account"
resim show $ACCOUNT1
log "The Maker's System Account"
resim show $ACCOUNT2
log "The Maker's Shared Account"
resim show $MAKER_ACCOUNT
