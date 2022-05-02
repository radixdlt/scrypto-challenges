#!/bin/bash

./build.sh && \
resim reset

export owner_account=$(resim new-account)
export owner=$(echo $owner_account | grep "Account component address:" | cut -d: -f2 | xargs)
export owner_private_key=$(echo $owner_account | grep "Private key:" | cut -d: -f2 | xargs)

export package=$(resim publish auction/target/wasm32-unknown-unknown/release/auction.wasm | grep "New Package" | cut -d: -f2 | xargs)
export xrd=$(resim show $owner | grep XRD | cut -d: -f3 | cut -d, -f1 | xargs)

cat <<EOT > manifests/create_auction.manifest
CALL_METHOD ComponentAddress("$owner") "withdraw_by_amount" Decimal("1") ResourceAddress("$xrd");
TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("1") ResourceAddress("$xrd") Bucket("auction_bucket");

CALL_FUNCTION PackageAddress("$package") "Auction" "new" Decimal("1") Decimal("10") Bucket("auction_bucket");
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$owner") "deposit_batch";
EOT

export auction=$(resim run manifests/create_auction.manifest | grep "Component:" | cut -d: -f2 | xargs)

export owner_badge=$(resim show $owner | grep "Owner of" | cut -d: -f3 | cut -d, -f1 | xargs)

export participant_3_account=$(resim new-account)
export participant_3=$(echo $participant_3_account | grep "Account component address:" | cut -d: -f2 | xargs)
export participant_3_private_key=$(echo $participant_3_account | grep "Private key:" | cut -d: -f2 | xargs)

cat <<EOT > manifests/register_3.manifest
CALL_METHOD ComponentAddress("$auction") "register";
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$participant_3") "deposit_batch";
EOT

resim set-default-account $participant_3 $participant_3_private_key
resim run manifests/register_3.manifest

export participant_3_badges_resource=$(resim show $participant_3 | grep "Participant of" | cut -d: -f3 | cut -d, -f1 | xargs)
export participant_3_badge_id=$(resim show $participant_3 | grep NON_FUNGIBLE | cut -d: -f2 | cut -d, -f1 | xargs)



export participant_6_account=$(resim new-account)
export participant_6=$(echo $participant_6_account | grep "Account component address:" | cut -d: -f2 | xargs)
export participant_6_private_key=$(echo $participant_6_account | grep "Private key:" | cut -d: -f2 | xargs)

cat <<EOT > manifests/register_6.manifest
CALL_METHOD ComponentAddress("$auction") "register";
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$participant_6") "deposit_batch";
EOT

resim set-default-account $participant_6 $participant_6_private_key
resim run manifests/register_6.manifest

export participant_6_badges_resource=$(resim show $participant_6 | grep "Participant of" | cut -d: -f3 | cut -d, -f1 | xargs)
export participant_6_badge_id=$(resim show $participant_6 | grep NON_FUNGIBLE | cut -d: -f2 | cut -d, -f1 | xargs)

cat <<EOT > manifests/withdraw.manifest
CALL_METHOD ComponentAddress("$owner") "create_proof_by_amount" Decimal("1") ResourceAddress("$owner_badge");
CALL_METHOD ComponentAddress("$auction") "withdraw";
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$owner") "deposit_batch";
EOT

cat <<EOT > manifests/bid_3.manifest
CALL_METHOD ComponentAddress("$participant_3") "create_proof_by_ids" TreeSet<NonFungibleId>(NonFungibleId("$participant_3_badge_id")) ResourceAddress("$participant_3_badges_resource");
CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS TreeSet<NonFungibleId>(NonFungibleId("$participant_3_badge_id")) ResourceAddress("$participant_3_badges_resource") Proof("proof_of_participation");

CALL_METHOD ComponentAddress("$participant_3") "withdraw_by_amount" Decimal("3") ResourceAddress("$xrd");
TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("3") ResourceAddress("$xrd") Bucket("bidding_bucket");

CALL_METHOD ComponentAddress("$auction") "bid" Bucket("bidding_bucket") Proof("proof_of_participation");
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$participant_3") "deposit_batch";
EOT

cat <<EOT > manifests/withdraw_3.manifest
CALL_METHOD ComponentAddress("$participant_3") "create_proof_by_ids" TreeSet<NonFungibleId>(NonFungibleId("$participant_3_badge_id")) ResourceAddress("$participant_3_badges_resource");
CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS TreeSet<NonFungibleId>(NonFungibleId("$participant_3_badge_id")) ResourceAddress("$participant_3_badges_resource") Proof("proof_of_participation");

CALL_METHOD ComponentAddress("$auction") "withdraw_outbidded" Proof("proof_of_participation");
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$participant_3") "deposit_batch";
EOT

cat <<EOT > manifests/unregister_3.manifest
CALL_METHOD ComponentAddress("$participant_3") "withdraw_by_ids" TreeSet<NonFungibleId>(NonFungibleId("$participant_3_badge_id")) ResourceAddress("$participant_3_badges_resource");
TAKE_FROM_WORKTOP_BY_IDS TreeSet<NonFungibleId>(NonFungibleId("$participant_3_badge_id")) ResourceAddress("$participant_3_badges_resource") Bucket("participant_badge_bucket");

CALL_METHOD ComponentAddress("$auction") "unregister" Bucket("participant_badge_bucket");
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$participant_3") "deposit_batch";
EOT

cat <<EOT > manifests/bid_6.manifest
CALL_METHOD ComponentAddress("$participant_6") "create_proof_by_ids" TreeSet<NonFungibleId>(NonFungibleId("$participant_6_badge_id")) ResourceAddress("$participant_6_badges_resource");
CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS TreeSet<NonFungibleId>(NonFungibleId("$participant_6_badge_id")) ResourceAddress("$participant_6_badges_resource") Proof("proof_of_participation");

CALL_METHOD ComponentAddress("$participant_6") "withdraw_by_amount" Decimal("6") ResourceAddress("$xrd");
TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("6") ResourceAddress("$xrd") Bucket("bidding_bucket");

CALL_METHOD ComponentAddress("$auction") "bid" Bucket("bidding_bucket") Proof("proof_of_participation");
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$participant_6") "deposit_batch";
EOT

cat <<EOT > manifests/unregister_6.manifest
CALL_METHOD ComponentAddress("$participant_6") "withdraw_by_ids" TreeSet<NonFungibleId>(NonFungibleId("$participant_6_badge_id")) ResourceAddress("$participant_6_badges_resource");
TAKE_FROM_WORKTOP_BY_IDS TreeSet<NonFungibleId>(NonFungibleId("$participant_6_badge_id")) ResourceAddress("$participant_6_badges_resource") Bucket("participant_badge_bucket");

CALL_METHOD ComponentAddress("$auction") "unregister" Bucket("participant_badge_bucket");
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$participant_6") "deposit_batch";
EOT

resim set-default-account $participant_3 $participant_3_private_key
resim run manifests/bid_3.manifest
# Should fail
echo "==================="
echo "Begin failure case"
echo "==================="
resim run manifests/bid_6.manifest
resim run manifests/unregister_3.manifest
resim run manifests/unregister_6.manifest
echo "==================="
echo "End of failure case"
echo "==================="

resim set-default-account $participant_6 $participant_6_private_key
resim run manifests/bid_6.manifest
# Should fail
echo "==================="
echo "Begin failure case"
echo "==================="
resim run manifests/bid_3.manifest
resim run manifests/unregister_6.manifest
resim run manifests/unregister_3.manifest
echo "==================="
echo "End of failure case"
echo "==================="
resim run manifests/bid_6.manifest
resim run manifests/unregister_6.manifest
echo "==================="
echo "Begin failure case"
echo "==================="
resim run manifests/bid_3.manifest
resim run manifests/unregister_6.manifest
resim run manifests/unregister_3.manifest
echo "==================="
echo "End of failure case"
echo "==================="

resim set-default-account $participant_3 $participant_3_private_key
resim run manifests/withdraw_3.manifest
resim run manifests/unregister_3.manifest
echo "==================="
echo "Begin failure case"
echo "==================="
resim run manifests/bid_3.manifest
resim run manifests/unregister_6.manifest
resim run manifests/unregister_3.manifest
echo "==================="
echo "End of failure case"
echo "==================="

resim set-default-account $owner $owner_private_key

resim run manifests/withdraw.manifest

echo "Auction outbidded user has $(resim show $participant_3 | grep $xrd | grep Radix | cut -d, -f1 | cut -d: -f2 | xargs) XRD"
echo "Auction winner has $(resim show $participant_6 | grep $xrd | grep Radix | cut -d, -f1 | cut -d: -f2 | xargs) XRD"
echo "Owner has $(resim show $owner | grep $xrd | grep Radix | cut -d, -f1 | cut -d: -f2 | xargs) XRD"