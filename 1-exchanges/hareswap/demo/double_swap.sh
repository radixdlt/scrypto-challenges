#!/usr/bin/env sh
#set -x
set -e

# Uncomment the line below to trace all transactions
#TRACE=--trace

# setup the common baseline environment for this test
source ./baseline.sh

log "Double Swap Example"

# This test has 2 makers, named (somewhat confusingly) maker3 and maker4

source ./maker3_setup.sh
source ./maker4_setup.sh

# in this example
# Taker wants to sell 200 T and buy C
# Taker requests quote to sell T to maker3 for B
# Taker requests quote to sell B to maker4 for C
# Taker executes first order followed by second order in single transaction

# start by giving maker3 (ie. B) some B tokens
resim set-default-account $ACCOUNT3 $ACCOUNT3_PUBKEY
log "Maker 'B' tokens B1000 seems like a good amount"
OP=$(resim new-token-fixed 1000 --symbol B)
B=$(echo "$OP" | sed -nr "s/.*ResourceDef: ([[:alnum:]_]+)/\1/p")
# and then move them to their shared account
resim transfer -s $ACCOUNT3_PUBKEY 1000,$B $MAKER3_ACCOUNT

# start by giving maker4 (ie. C) some C tokens
resim set-default-account $ACCOUNT4 $ACCOUNT4_PUBKEY
log "Maker 'C' tokens C1000 seems like a good amount"
OP=$(resim new-token-fixed 1000 --symbol C)
C=$(echo "$OP" | sed -nr "s/.*ResourceDef: ([[:alnum:]_]+)/\1/p")
# and then move them to their shared account
resim transfer -s $ACCOUNT4_PUBKEY 1000,$C $MAKER4_ACCOUNT

log "Taker to use HareSwap"

# switch to taker
resim set-default-account $ACCOUNT1 $ACCOUNT1_PUBKEY

log "Taker does 1st RFQ sell T for B"

T_AMOUNT=200.0
log "Taker requests: Whoever has TAKER_AUTH would like to sell $T_AMOUNT of T in exchange for B"
xlog $HARE request-for-quote sell-base partial_order_sell_t200_b.txt $T_AMOUNT $T $B $TAKER_AUTH

log "simulate sending partial order to Maker"
log "Taker >>> partial_order_sell_t200_b.txt >>> Maker"

log "Maker accepts the order and decides to quote 100.0 B will be paid to buy the 200.0 T."
log "The unique identifier for this order is 'C1' which is good until (and including) epoch 42"
B_AMOUNT=100.0
VOUCHER3_KEY=B1
DEADLINE3_EPOCH=42
xlog $HARE make-signed-order partial_order_sell_t200_b.txt $B_AMOUNT $MAKER3_COMPONENT $VOUCHER3_ADDRESS $VOUCHER3_KEY $MAKER3_OFFLINE_KEY_PRI $DEADLINE3_EPOCH > signed_order_b1.txt

log "simulate sending signed order back to Taker"
log "Maker >>> signed_order_b1.txt >>> Taker"

log "Taker does 2nd RFQ sell B for C"

log "Taker requests: Whoever has TAKER_AUTH would like to sell $B_AMOUNT of B in exchange for C"
xlog $HARE request-for-quote sell-base partial_order_sell_b100_c.txt $B_AMOUNT $B $C $TAKER_AUTH

log "simulate sending partial order to Maker"
log "Taker >>> partial_order_sell_b100_c.txt >>> Maker"

log "Maker accepts the order and decides to quote 50.0 C will be paid to buy the 100.0 B."
log "The unique identifier for this order is 'C1' which is good until (and including) epoch 42"
C_AMOUNT=50.0
VOUCHER4_KEY=C1
DEADLINE4_EPOCH=42
xlog $HARE make-signed-order partial_order_sell_b100_c.txt $C_AMOUNT $MAKER4_COMPONENT $VOUCHER4_ADDRESS $VOUCHER4_KEY $MAKER4_OFFLINE_KEY_PRI $DEADLINE4_EPOCH > signed_order_c1.txt

log "simulate sending signed order back to Taker"
log "Maker >>> signed_order_c1.txt >>> Taker"

######
# A bunch of error/safety checking would go here to ensure the signed orders matches the original orders etc
# see simple_swap.sh demo for more discussion
#####

log "Taker things selling 200 T for 50 C is a good deal so..."
log "Taker combines both orders into a single transaction and executes it"

SIGNED_ORDER_B=$(cat signed_order_b1.txt)
SIGNED_ORDER_C=$(cat signed_order_c1.txt)
FN=taker_submit_double_swap.rtm
cat > $FN   <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge_t_auth");
CALL_METHOD Address("$ACCOUNT1") "withdraw" Decimal("1") Address("$TAKER_AUTH") BucketRef("account_badge_t_auth");
TAKE_ALL_FROM_WORKTOP Address("$TAKER_AUTH") Bucket("auth_for_exec_bucket");
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge_t");
CALL_METHOD Address("$ACCOUNT1") "withdraw" Decimal("$T_AMOUNT") Address("$T") BucketRef("account_badge_t");
TAKE_ALL_FROM_WORKTOP Address("$T") Bucket("T");
CREATE_BUCKET_REF Bucket("auth_for_exec_bucket") BucketRef("auth_for_exec_b");
CREATE_BUCKET_REF Bucket("auth_for_exec_bucket") BucketRef("auth_for_exec_c");
$SIGNED_ORDER_B Bucket("T") BucketRef("auth_for_exec_b");
ASSERT_WORKTOP_CONTAINS Decimal("$B_AMOUNT") Address("$B");
TAKE_ALL_FROM_WORKTOP Address("$B") Bucket("B");
$SIGNED_ORDER_C Bucket("B") BucketRef("auth_for_exec_c");
ASSERT_WORKTOP_CONTAINS Decimal("$C_AMOUNT") Address("$C");
CALL_METHOD_WITH_ALL_RESOURCES Address("$ACCOUNT1") "deposit_batch";
EOF

log "check the manifest syntax by compiling it"
xlog rtmc --output ${FN}c $FN && rm ${FN}c

log "submit the transaction to execute the trade"
resim run $TRACE $FN

success
log "look at the accounts:"
log "Taker has changed: T: 1000-200=800, C: +50"
log "Maker3 (Shared Account) has changed: B: 1000-100=900, T: +200"
log "Maker4 (Shared Account) has changed: C: 1000-50=950, B: +100"
log "---"
log "The Taker's System Account"
resim show $ACCOUNT1
log "The Maker3's Shared Account"
resim show $MAKER3_ACCOUNT
log "The Maker4's Shared Account"
resim show $MAKER4_ACCOUNT
