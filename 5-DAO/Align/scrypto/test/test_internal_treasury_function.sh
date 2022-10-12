source ./init.sh

log_header "Begin testing Internal Treasury"
logbg "You can try changing the input params for testing on file test_internal_treasury.sh"
logbg "This test need high centralized user pattern to test malicious acts"

# You can try changing the input params for testing
community1_name="DEMOCRAT"
tax1_percent="0.4"
community2_name="REPUBLICAN"
tax2_percent="0.8"
alice_commit_amount=$((representative_requirement)) # She is a representative so must have enough vote power for that
bond_delegate_amount=$((representative_requirement)) # Should be equal to Alice to test how it different when rage withdraw
bob_commit_amount=$((2200000)) # Create a pretty centralized test user pattern to test the Treasury's security policies.
lyn_delegate_amount=$((100)) 
kevin_commit_amount=$((initial_supply * (100 - liquidity_allocation) / 100 - alice_commit_amount - bond_delegate_amount - bob_commit_amount - lyn_delegate_amount)) # Remainder resource allocation of other.

alice_retirement_length=$((minimum_retirement))
bob_retirement_length=$((maximum_retirement))
kevin_retirement_length=$((WEEK * 101))

# The test proposal input to get all the primary reserve resource from the DAO
export fund_demand=$stablecoin_amount
export time_delay=$proposal_minimum_delay

source ./default_users.sh

logc "Test Internal Treasury withdraw threshold"
export sbt=$member_sbt

logb "Test ok pattern: Bob make the proposal to withdraw all the primary reserve resource from the DAO"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/dao/proposal/get_fund_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test ok pattern: Bob vote support the proposal"
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice vote against the proposal"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export vote=false
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin vote against the proposal"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export vote=false
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt

logb "Test ok pattern: Lyn rage quit Bob's community"
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/dao/member_delegator/rage_quit.rtm`
info "$output"
assert_success "$output"

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test ok pattern: Bob check the current proposal status"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test malicious pattern: Bob execute the centralized proposal but cannot withdraw the whole treasury according to the threshold"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
error "$output"
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob withdraw the provided fund on the proposal"
output=`resim run ../rtm/proposal/proposer_only/withdraw_fund.rtm`
info "$output"
assert_success "$output"

logc "Test Rage Withdraw"

export sbt=$member_sbt

logb "Test failure pattern: Alice try to rage withdraw from the DAO"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/member_delegator/rage_withdraw.rtm`
assert_failure "$output"

logb "Test failure pattern: Alice try to retire"
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_failure "$output"

export sbt=$delegator_sbt
logb "Test ok pattern: Alice advise Bond to quit the community and do rage withdraw"
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
export community_address=$COMMUNITY1
output=`resim run ../rtm/community/user_pattern/quit_community.rtm`
assert_success "$output"

output=`resim run ../rtm/dao/member_delegator/rage_withdraw.rtm`
info "$output"
assert_success "$output"

export sbt=$member_sbt
logb "Test ok pattern: Alice retire and do rage withdraw"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_success "$output"

output=`resim run ../rtm/dao/member_delegator/rage_withdraw.rtm`
info "$output"
assert_success "$output"

logb "$((rage_withdraw_time_limit/ WEEK)) weeks later..."
advance_time $rage_withdraw_time_limit

export sbt=$member_sbt
logb "Test failure pattern: Kevin forgot for $((rage_withdraw_time_limit/WEEK)) weeks and now he try retire and rage withdraw after the time limit"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_success "$output"
output=`resim run ../rtm/dao/member_delegator/rage_withdraw.rtm`
assert_failure "$output"
logy "Kevin is such a forgetful person and now he have to go through the retirement normally since he can no longer do rage withdraw"

logc "Test Internal Treasury withdraw period"

logb "Test ok pattern: Bob try make the proposal again"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/dao/proposal/get_fund_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test ok pattern: Bob vote support the proposal"
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "$((time_delay/ WEEK)) weeks later..."
advance_time $time_delay

logb "Test malicious pattern: Bob execute the centralized proposal but cannot withdraw anything since he have to wait for $(( (withdraw_period - rage_withdraw_time_limit - time_delay) / WEEK)) more weeks to reset the threshold"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/execute_proposal.rtm`
info "$output"
assert_success "$output"

logc "Test Internal decentralized share market"

export sbt=$delegator_sbt
logb "Test ok pattern: Lyn forgot to withdraw before, now she try to withdraw all and swap for the primary reserve resource (stablecoins)"
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/dao/member_delegator/withdraw_all_and_swap.rtm`
info "$output"
assert_success "$output"

logb "Now there's no people on the DAO except for Bob and he can do no more malicious acts"
logb "Bob realized still commit a lot of resource on the DAO, so he try swap half the received resource and re-deposit the other half"

export sbt=$member_sbt

logb "Test ok pattern: Bob try to swap primary reserve resource (stablecoins) to the DAO share"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export amount=$((stablecoin_amount * withdraw_threshold / 100 / 2))
output=`resim run ../rtm/dao/general/swap.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob try to deposit primary reserve resource (stablecoins) to the DAO's treasury"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export amount=$((stablecoin_amount * withdraw_threshold / 100 / 2))
export resource=$stable_coin
output=`resim run ../rtm/dao/general/deposit.rtm`
info "$output"
assert_success "$output"

completed