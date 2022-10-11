#!/usr/bin/env bash

set -o errexit

cd scrypto/test/

source ./utilities/log.sh
logc "This script will run all 8 comprehensive tests"

./test_user_and_withdraw_pattern.sh
./test_commitment_voting_mechanism.sh
./test_liquid_democracy.sh
./test_prm_quorum_voting_mechanism.sh
./test_internal_treasury_function.sh
./test_distribution_proposal.sh
./test_tokenomic.sh
./test_real_use_case.sh

logc "You have completed all the tests"
completed