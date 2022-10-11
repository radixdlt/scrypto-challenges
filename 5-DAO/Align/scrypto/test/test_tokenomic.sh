source ./init.sh

log_header "Begin testing tokenomic model"
logbg "You can try changing the input params for testing on file test_tokenomic.sh"

# You can try changing the input params for testing
community1_name="DEMOCRAT"
tax1_percent="0.4"
community2_name="REPUBLICAN"
tax2_percent="0.8"
alice_commit_amount=$((representative_requirement)) # She is a representative so must have enough vote power for that
bond_delegate_amount=$((representative_requirement)) # Should be the same as other to check how vote power affect dividend
bob_commit_amount=$((representative_requirement)) # He is a representative so must have enough vote power for that
lyn_delegate_amount=$((representative_requirement)) # Should be the same as other to check how vote power affect dividend
kevin_commit_amount=$((initial_supply * (100 - liquidity_allocation) / 100 - alice_commit_amount - bond_delegate_amount - bob_commit_amount - lyn_delegate_amount)) # Remainder resource allocation of other, also have enough power to alter the vote outcome

alice_retirement_length=$((WEEK * 101))
bob_retirement_length=$((maximum_retirement))
kevin_retirement_length=$((minimum_retirement))

# The test proposal input to test the tokenomic model
export fund_demand=0
export time_delay=$WEEK

source ./default_users.sh

logb "Accelerate time to get more power for DAO's member"
export accelerate=$((WEEK*52))
logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logc "Test dividend for accepted proposal"

logb "Test ok pattern: Bob create the test proposal"
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

logb "Test ok pattern: Kevin vote support the proposal"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test ok pattern: Bob check the proposal status"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Alice, Bob, Kevin, Lyn and Bond check their current accounts"

export sbt=$member_sbt
logb "== Bob vote for =="
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Alice vote against =="
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Kevin vote for =="
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt
logb "== Lyn vote for =="
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Bond vote against =="
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob execute the proposal"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/execute_proposal.rtm`
info "$output"
assert_success "$output"

logb "Alice, Bob, Kevin, Lyn and Bond check their current accounts again"
export sbt=$member_sbt

logb "== Bob vote for =="
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Alice vote against =="
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Kevin vote for =="
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt
logb "== Lyn vote for =="
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Bond vote against =="
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logc "Test slash on rejected proposal"

logb "Test ok pattern: Alice create the test proposal"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/proposal/get_fund_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test ok pattern: Alice vote support the proposal"
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob vote support the proposal"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin vote reject the proposal"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export vote=false
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test ok pattern: Alice check the proposal status"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Alice, Bob, Kevin, Lyn and Bond check their current accounts"

export sbt=$member_sbt

logb "== Alice vote for =="
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Bob vote for =="
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Kevin vote against =="
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt
logb "== Lyn vote for =="
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Bond vote for =="
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice execute the proposal"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/proposal/execute_proposal.rtm`
error "$output"
assert_success "$output"

logb "Alice, Bob, Kevin, Lyn and Bond check their current accounts again"
export sbt=$member_sbt

logb "== Alice vote for =="
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Bob vote for =="
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Kevin vote against =="
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt
logb "== Lyn vote for =="
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Bond vote for =="
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logc "Test claim dividend & accumulate dividend"
logb "Bob, Kevin accumulate a part of their dividend; all claim their dividend then check their accounts again"

export sbt=$member_sbt

logb "== Bob =="
logb "Test failed pattern: He cannot accumulate more vote power since he's a representative"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export amount=1
output=`resim run ../rtm/dao/member_only/accumulate_dividend.rtm`
assert_failure "$output"

logb "He claim his dividend instead"
output=`resim run ../rtm/dao/member_delegator/claim_dividend.rtm`
info "$output"
assert_success "$output"

output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "== Kevin =="
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_only/accumulate_dividend.rtm`
info "$output"
assert_success "$output"

output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_delegator/claim_dividend.rtm`
info "$output"
assert_success "$output"

output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt
logb "== Lyn =="
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/dao/member_delegator/claim_dividend.rtm`
info "$output"
assert_success "$output"

output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logb "Check the DAO's remain dividend"
output=`resim run ../rtm/dao/read_only/check_dividend.rtm`
info "$output"
assert_success "$output"

completed