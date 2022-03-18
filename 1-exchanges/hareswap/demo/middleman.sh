#!/usr/bin/env sh
#set -x
set -e

# Uncomment the line below to trace all transactions
#TRACE=--trace

# setup the common baseline environment for this test
source ./baseline.sh

log "Tokenized Swap 'middleman' Example"
log "In this example, a 3rd party middleman negotiates a trade"
log "A buyer wants a unique NFT but nobody is selling"
log "Agrees with a middleman to pay 1000 XRD for the item if they can get it"
log "middleman makes a deal with the seller for 900 XRD"
log "seller executes the middleman's order and the middleman's callback executes the first order"
log "giving the NFT to the buyer and getting the payment"
log
log "(since everything is public the seller could read the data embedded in the order and"
log " realize they could deal with the buyer directly and save 100 XRD.  But maybe the buyer is semi-anonymous"
log " or the seller wont want to tank a good deal buy introducing a delay, etc etc.)"

# in this demo, the Seller is ACCOUNT2, the Middleman is ACCOUNT1 and the Buyer is ACCOUNT3

# Buyer's account
source ./maker3_setup.sh

# for the demo, we need an NFT, give one to the seller
log "Airdrop a one-of-a-kind NFT to the Seller (Account2)"
NFT_KEY=01
NFT_KEYS="TreeSet<NonFungibleKey>(NonFungibleKey(\"01\"))"
NFT_RESOURCE=$(../hare/target/debug/hare test nft-setup $ACCOUNT2 demo-nft-family $NFT_KEY ./helper)

log "Middleman, acting as a taker says they will sell the NFT to the buyer (when they get it)"
xlog $HARE request-for-quote sell-base middleman_sell_partial_order.txt $NFT_KEY $NFT_RESOURCE $XRD $TAKER_AUTH

log "Buyer agrees to pay 1000 XRD"

log "Buyer needs to move 1000 XRD into SharedAccount first"
resim set-default-account $ACCOUNT3 $ACCOUNT3_PUBKEY
resim transfer -s $ACCOUNT3_PUBKEY 1000,$XRD $MAKER3_ACCOUNT
log "Buyer signs the order"
BUYER_AMOUNT=1000.0
VOUCHER3_KEY=F1
DEADLINE_EPOCH=42
xlog $HARE make-signed-order middleman_sell_partial_order.txt $BUYER_AMOUNT $MAKER3_COMPONENT $VOUCHER3_ADDRESS $VOUCHER3_KEY $MAKER3_OFFLINE_KEY_PRI $DEADLINE_EPOCH > middleman_sell_signed_order.txt

log "Middleman tokenizes the order storing the token in their Middleman component"
resim set-default-account $ACCOUNT1 $ACCOUNT1_PUBKEY

log "first, instantiate the Middleman component"
FN=middleman_instantiate.rtm
cat > $FN   <<EOF
CALL_FUNCTION Address("$PACKAGE") "Middleman" "instantiate" Address("$ACCOUNT1");
CALL_METHOD_WITH_ALL_RESOURCES Address("$ACCOUNT1") "deposit_batch";
EOF
OP=$(resim run --trace $FN)
# get the new auth resource and component address
MM_CB_AUTH=$(echo "$OP" | sed -nr "s/.*ResourceDef: ([[:alnum:]_]+)/\1/p")
MM_COMPONENT=$(echo "$OP" | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")

log "now tokenize and store in the middleman component"
xlog $HARE tokenize-order middleman_sell_signed_order.txt tokenize_auth order1 > middleman_tokenize_order_instruction.txt
TOKENIZED_ORDER=$(cat middleman_tokenize_order_instruction.txt)
FN=middleman_tokenize_order.rtm
cat > $FN   <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge_t_auth");
CALL_METHOD Address("$ACCOUNT1") "withdraw" Decimal("1") Address("$TAKER_AUTH") BucketRef("account_badge_t_auth");
TAKE_ALL_FROM_WORKTOP Address("$TAKER_AUTH") Bucket("auth_for_exec_bucket");
CREATE_BUCKET_REF Bucket("auth_for_exec_bucket") BucketRef("tokenize_auth");
$TOKENIZED_ORDER
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge_2");
CALL_METHOD Address("$ACCOUNT1") "withdraw" Decimal("1") Address("$MM_CB_AUTH") BucketRef("account_badge_2");
CALL_METHOD_WITH_ALL_RESOURCES Address("$MM_COMPONENT") "add_orders";
EOF
log "validate and run the transaction"
rtmc --output ${FN}c ${FN}
#_resim run --trace ${FN} > ${FN}.trace 2>&1
resim run $TRACE ${FN}

log "Middleman now setup the Maker component using the MM_CB_AUTH so the callback will work"
MM_OFFLINE_KEY_PUB=mm.pub
MM_OFFLINE_KEY_PRI=mm.pri
pubkeymm_arg=$($HARE new-key-pair $MM_OFFLINE_KEY_PUB $MM_OFFLINE_KEY_PRI)
FN=mm_maker_setup.rtm
cat > $FN  <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge");
CALL_METHOD Address("$ACCOUNT1") "withdraw" Decimal("1.0") Address("$MM_CB_AUTH") BucketRef("account_badge");
TAKE_FROM_WORKTOP Decimal("1.0") Address("$MM_CB_AUTH") Bucket("MM_CB_AUTH");
CALL_FUNCTION Address("$PACKAGE") "Maker" "instantiate_custom" $pubkeymm_arg Bucket("MM_CB_AUTH");
EOF
log "validate and run the transaction"
rtmc --output ${FN}c ${FN}
_resim run --trace ${FN} > ${FN}.trace 2>&1
# annoying to get the return values when using resim instead of Rust APIs against the ledger.  These are brittle.
MAKER_COMPONENT=$(tail -n1 ${FN}.trace | cut -d' ' -f3)
VOUCHER_ADDRESS=$(grep "INFO.*tokenized order resource address:" ${FN}.trace | cut -d':' -f2)
rm ${FN}.trace

log "Seller decides they will sell to the middleman, they create a request for quote"

# switch to Seller
resim set-default-account $ACCOUNT2 $ACCOUNT2_PUBKEY
# create anti-frontrunning auth
OP=$(resim new-badge-fixed 1 --name buyer-rfq-auth)
BUYER_AUTH=$(echo "$OP" | sed -nr "s/.*ResourceDef: ([[:alnum:]_]+)/\1/p")
# create the RFQ
xlog $HARE request-for-quote sell-base partial_order.txt $NFT_KEY $NFT_RESOURCE $XRD $BUYER_AUTH

log "Maker accepts the order and quotes 1000 XRD will be paid to buy the NFT being sold."
log "The unique identifier for this order is 'B1' which is good until (and including) epoch 42"
MAKER_AMOUNT=900.0
VOUCHER_KEY=B1
DEADLINE_EPOCH=42
CALLBACK="CALL_METHOD Address(\"$MM_COMPONENT\") \"middleman_callback\";"
xlog $HARE make-signed-order partial_order.txt $MAKER_AMOUNT $MAKER_COMPONENT $VOUCHER_ADDRESS $VOUCHER_KEY $MM_OFFLINE_KEY_PRI $DEADLINE_EPOCH "$CALLBACK" > signed_order.txt
cat signed_order.txt

######
# A bunch of error/safety checking would go here...(see simple_swap.sh for more)
#
#####

log "Finally the Seller executes the middleman's order which will callback to execute the buyer's order"

SIGNED_ORDER=$(cat signed_order.txt)
FN=seller_submit_mm.rtm
cat > $FN   <<EOF
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge_t_auth");
CALL_METHOD Address("$ACCOUNT2") "withdraw" Decimal("1") Address("$BUYER_AUTH") BucketRef("account_badge_t_auth");
TAKE_ALL_FROM_WORKTOP Address("$BUYER_AUTH") Bucket("auth_for_exec_bucket");
CLONE_BUCKET_REF BucketRef(1u32) BucketRef("account_badge_t");
CALL_METHOD Address("$ACCOUNT2") "withdraw_non_fungibles" $NFT_KEYS Address("$NFT_RESOURCE") BucketRef("account_badge_t");
TAKE_ALL_FROM_WORKTOP Address("$NFT_RESOURCE") Bucket("NFT");
CREATE_BUCKET_REF Bucket("auth_for_exec_bucket") BucketRef("auth_for_exec");
$SIGNED_ORDER Bucket("NFT") BucketRef("auth_for_exec");
ASSERT_WORKTOP_CONTAINS Decimal("$MAKER_AMOUNT") Address("$XRD");
CALL_METHOD_WITH_ALL_RESOURCES Address("$ACCOUNT2") "deposit_batch";
EOF

log "check the manifest syntax by compiling it"
xlog rtmc --output ${FN}c $FN && rm ${FN}c

log "submit the transaction to execute the trade"
resim run $TRACE $FN

success
# in this demo, the Seller is ACCOUNT2, the Middleman is ACCOUNT1 and the Buyer is ACCOUNT3
log "look at the accounts:"
log "Buyer has changed: no more demo-nft-family:01, +100 M"
log "Buyer's (Shared) Account has changed: XRD: 1000-1000=0, added demo-nft-family:01"
log "Seller's (Shared) Account has changed: XRD: 0+900=900, no more demo-nft-family:01"
log "Middleman System Account has changed: XRD: 1000000+1000-900=1000100, no nft"
log "---"
log "The Buyers's Shared Account"
resim show $MAKER3_ACCOUNT
log "The Seller's Shared Account"
resim show $ACCOUNT2
log "The Middleman's System Account"
resim show $ACCOUNT1
