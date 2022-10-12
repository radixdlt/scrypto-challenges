#!/usr/bin/env bash

# set -x
set -e

#Use log
source ./utilities/scrypto_unit.sh
source ./utilities/utilities.sh

export WEEK=604800
# Total stablecoin liquidity config
export init_stablecoin_amount=30000000

# Protocol configs
export stablecoin_amount=20000000
export initial_supply=10000000
export liquidity_allocation=70

export swap_fee=0.5
export withdraw_threshold=10
export withdraw_period=$((WEEK * 4))
export rage_withdraw_decline_multiply=100 # People with minimum retirement length will got 5% decrease on rage withdraw and 3.2% additional decrease/additional month retirement length according to current params.
export rage_withdraw_time_limit=$WEEK

export dividend=150
export slash_rate=1

export initital_commitment_rate=0.05
export minimum_retirement=$((WEEK * 26))
export maximum_retirement=$((WEEK * 286))
# the calculation to come up with this number is actually a bit complicated, 
# it's a reverse from the vote power calculation from the view of the people with maximum retirement can get max vote power in 1 year with all other input
# (exp(ln(300 / 100)/52) - 1 - (0.05 / 100)) / (286-26) / 4 * 100 (%)
export commitment_grow_rate=0.03207987365
export maximum_vote_rate=300
export period_length=$((WEEK * 4))

export initial_credibility=4 # This is for faster testing the rage quit & credibility score system
export representative_requirement=100000

export proposal_requirement=100000
export proposal_quorum=1700000
export proposal_minimum_delay=$WEEK

log_header "New fresh start"
resim reset

export XRD=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag


logy "Creating Admin account."
output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export ADMIN_ACC=`echo $output | cut -d " " -f1`
export ADMIN_PIV=`echo $output | cut -d " " -f2`

export package=`resim publish ./neuracle/neuracle.wasm | awk '/Package:/ {print $NF}'`

RANGE=500
export epoch=$RANDOM
let "epoch %= $RANGE"

logy "Set random epoch: $epoch"
resim set-current-epoch $epoch

output=`resim run ./neuracle/init.rtm | awk '/Admin badge address: |NeuRacle token address: |Validator badge address: |operator badge address: |New NeuRacle Component: |New NeuraProject Component: / {print $NF}'`
export admin_badge=`echo $output | cut -d " " -f1` 
export NAR=`echo $output | cut -d " " -f2` 
export validator_badge=`echo $output | cut -d " " -f3`
export operator_badge=`echo $output | cut -d " " -f4`
export neuracle_component=`echo $output | cut -d " " -f5` 
export project_component=`echo $output | cut -d " " -f6` 

output=`resim run ./neuracle/init2.rtm | awk '/Staker Badge Address: |Validator Address: |User badge: |int Component: / {print $NF}'`
export staker_badge=`echo $output | cut -d " " -f1` 
export validator=`echo $output | cut -d " " -f2` 
export data_badge=`echo $output | cut -d " " -f3`
export oraclecomp=`echo $output | cut -d " " -f4`

output=`resim run ./neuracle/init3.rtm`

logy "Publish Align package."

export package=`resim run ../rtm/init/publish.rtm --blobs ../rtm/init/9b0048e68866e3990e297e2526149af9abe50e379ed750970a1098095873c696.blob ../rtm/init/c294bbe49c2cc0deaaafe6f7004cba937bedf0b6e9f9f3b513b6a6bdbef05fc6.blob | awk '/Package:/ {print $NF}'`
export stable_coin=`resim new-token-fixed "$init_stablecoin_amount" | awk '/Resource: / {print $NF}'`

logy "Created new pseudo stablecoin resource: $stable_coin"

output=`resim run ../rtm/init/init.rtm | awk '/Align DAO Badge Address: |ALIGN Address: |DAO Member SBT address: |Delegator SBT address: |Proposal badge address: |New Align DAO Component: |New AlignProject Component: |Local Oracle Component address: / {print $NF}'`

export dao_badge=`echo $output | cut -d " " -f1` 
export ALIGN=`echo $output | cut -d " " -f2` 
export member_sbt=`echo $output | cut -d " " -f3` 
export delegator_sbt=`echo $output | cut -d " " -f4`
export proposal_badge=`echo $output | cut -d " " -f5`
export local_oracle=`echo $output | cut -d " " -f6`
export dao_comp=`echo $output | cut -d " " -f7`
export project_comp=`echo $output | cut -d " " -f8`

logc "Created new Align DAO with the input and policies as follow"
logp "### Project input
- initial_supply: $initial_supply,
- liquidity_allocation: $liquidity_allocation%,

### Treasury input
- dao_share: Bucket with $((initial_supply * liquidity_allocation / 100)) ALIGN,
- stable_coin: Bucket with $stablecoin_amount stable coin,
- swap_fee: $swap_fee%,
- withdraw_threshold: $withdraw_threshold%,
- withdraw_period: $((withdraw_period / WEEK)) weeks, 
- rage_withdraw_decline_multiply: $rage_withdraw_decline_multiply,
- rage_withdraw_time_limit: $((rage_withdraw_time_limit / WEEK)) weeks,

### Economic policy
- dividend: $dividend DAO share / proposal,
- slash_rate: $slash_rate%,

### Commitment policy
- initital_commitment_rate: $initital_commitment_rate%,
- minimum_retirement: $((minimum_retirement / WEEK)) weeks,
- maximum_retirement: $((maximum_retirement / WEEK)) weeks,
- commitment_grow_rate: $commitment_grow_rate%,
- maximum_vote_rate: $maximum_vote_rate%,
- period_length: $((period_length / WEEK)) weeks,

### Community policy
- initial_credibility: $initial_credibility,
- representative_requirement: $representative_requirement,

### Oracle
- oracle: from NeuRacle package,

### Proposal policy
- proposal_requirement: $proposal_requirement,
- proposal_quorum: $proposal_quorum,
- proposal_minimum_delay: $((proposal_minimum_delay / WEEK)) weeks,

You can change these initialize input for testing on file init.sh"

output=`resim run ../rtm/init/fund_oracle.rtm`
assert_success "$output"

logy "New Align DAO component: $dao_comp"

export time=1663774412
time_update $time