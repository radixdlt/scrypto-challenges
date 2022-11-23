#!/usr/bin/env bash

#set -x
set -e

source ./log.sh



logc "A person start a round"

resim run ./transaction_manifest/start_round 

source ./update_data.sh

logc "A person begin conclude the round"

resim run ./transaction_manifest/end_round



completed