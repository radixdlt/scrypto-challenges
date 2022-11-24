#!/bin/bash

# Creates a dispute between client and contractor
source ./transactions/dispute/1_create_new_dispute.sh
# Submits a document nft by client and contractor
source ./transactions/dispute/2_submit_document.sh
# Participant that is eligble joins and decides on the dispute in favour of contractor
source ./transactions/dispute/3_join_and_decide_dispute.sh
# Contractor completes the dispute and both client and contractor claim their dispute outcome NFT
source ./transactions/dispute/4_complete_dispute.sh
