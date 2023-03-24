source ./init.sh

log_header "Begin testing Permissioned Relative Majority & Quorum Voting Mechanism"
logbg "You can try changing the input params for testing on file test_sm_quorum_voting_mechanism.sh"

# You can try changing the input params for testing
community1_name="DEMOCRAT"
tax1_percent="0.4"
community2_name="REPUBLICAN"
tax2_percent="0.8"
alice_commit_amount=$((representative_requirement)) # Alice and Bond vote power combined must smaller 1/2 total 5 users voting power combined to test the Permissioned Relative Majority Voting mechanism
bond_delegate_amount=$((representative_requirement)) # Alice and Bond vote power combined must smaller 1/2 total 5 users voting power combined to test the Permissioned Relative Majority Voting mechanism
bob_commit_amount=$((proposal_quorum - 2)) # Bob and Lyn vote power combined must smaller than proposal quorum to test the Quorum Voting mechanism
lyn_delegate_amount=$((1)) # Bob and Lyn vote power combined must smaller than proposal quorum to test the quorum voting mechanism
kevin_commit_amount=$((proposal_requirement - 1)) # Kevin vote power must smaller than proposal requirement to test its function

alice_retirement_length=$((WEEK * 101))
bob_retirement_length=$((WEEK * 101))
kevin_retirement_length=$((WEEK * 101))

# The test proposal input
export component_address=$dao_comp
export method="amend_treasury_policy"
export fund_demand=200
export malicious_time_delay=10
export time_delay=$proposal_minimum_delay
export args="1001000000a1200000000000b2d3595bf006000000000000000000000000000000000000000000000000" # Encoded hex of Decimal("0.5"), you can try run the test function at the end of lib.rs to get the input hex

source ./default_users.sh

export sbt=$member_sbt
logc "Test Proposal Requirement"
logb "Test failure pattern: Kevin make the test proposal when didn't meet the proposal requirement"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
assert_failure "$output"

logb "Test malicious pattern: Bob make the test proposal but try to set it on malicious time delay ($((malicious_time_delay)) seconds) to get through the proposal fast so no one notice"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export time_delay=10
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
assert_failure "$output"

logb "Test ok pattern: Bob try make the test proposal again with the minimum time delay"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export time_delay=$proposal_minimum_delay
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logc "Test Quorum Voting Mechanism"
logb "Test ok pattern: Bob vote support the proposal"
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test failure pattern: Kevin try to vote on the proposal while it's already ended"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
assert_failure "$output"

logb "Test ok pattern: Bob check the current proposal status"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test failure pattern: Bob execute the proposal which haven't meet the proposal quorum"
logy "This transaction have to commit success to remove the proposal from the DAO"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
error "$output"
assert_success "$output"

logb "Test ok pattern: Bob re-make the test proposal and voted on it again"
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin also vote the proposal on-time this time"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
assert_success "$output"

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test ok pattern: Bob check the current proposal status"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob execute the proposal which have meet the proposal quorum and have the supported voted power > against voted power"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob withdraw the provided fund on the proposal"
output=`resim run ../rtm/proposal/proposer_only/withdraw_fund.rtm`
info "$output"
assert_success "$output"

logc "Test Permissioned Relative Majority Voting Mechanism"

logb "Test ok pattern: Alice make the test proposal and vote support it"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob vote against the proposal"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export vote=false
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin vote support the proposal"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test ok pattern: Alice check the current proposal status"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test failure pattern: Alice execute the proposal which have the total support voted power < against voted power"
logy "This transaction have to commit success to remove the proposal from the DAO and punish those who voted FOR"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
error "$output"
assert_success "$output"

logc "Test Vote Reentrancy"

logy "Since DAO's Member cannot withdraw the DAO's share (which's the core unit in the Commitment Voting Mechanism) without going through a retirement process, vote reentrancy attack can only happen with Delegator"
logy "Let's test the reentrancy case with someone called Mal want to create 3 accounts for his attack (1 Delegator account to do the reentrancy, 2 accounts enough to become DAO's representative to vote in the Delegator account's stead)"

logb "== Mal1 - representative account 1 =="

export sbt=$member_sbt

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export MAL1=`echo $output | cut -d " " -f1`
export MAL1_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $MAL1 $MAL1_PIV`
export caller=$MAL1
export commit_amount=$representative_requirement

export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

export retirement_length=$minimum_retirement
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

export tax_percent=0
export name="My Heaven"
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_success "$output"
export HEAVEN=`echo "$output" | awk '/Component: / {print $NF}'`

logb "== Mal2 - representative account 2 =="

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export MAL2=`echo $output | cut -d " " -f1`
export MAL2_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $MAL2 $MAL2_PIV`
export caller=$MAL2
export commit_amount=$representative_requirement

export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

export retirement_length=$minimum_retirement
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

export tax_percent=0
export name="My Heaven 2"
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_success "$output"
export HEAVEN2=`echo "$output" | awk '/Component: / {print $NF}'`

logb "== Mal3 - Delegator Account first will follow the account 1's community =="

export sbt=$delegator_sbt

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export MAL3=`echo $output | cut -d " " -f1`
export MAL3_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $MAL3 $MAL3_PIV`
export caller=$MAL3
export delegate_amount=300000

export align_withdraw_amount=$delegate_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

output=`resim run ../rtm/dao/general/become_delegator.rtm`
assert_success "$output"

export community_address=$HEAVEN
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

output=`resim set-default-account $MAL1 $MAL1_PIV`
export caller=$MAL1
export id=1
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

output=`resim set-default-account $MAL3 $MAL3_PIV`
export caller=$MAL3
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Now the first representative account of Mal will make a proposal and vote on it"
output=`resim set-default-account $MAL1 $MAL1_PIV`
export caller=$MAL1
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test malicious pattern: He try to withdraw on the Delegator account to create another account for vote reentrancy attack but the smartcontract logic prevent him from withdraw when there's on-going vote"
output=`resim set-default-account $MAL3 $MAL3_PIV`
export caller=$MAL3
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
assert_failure "$output"

completed