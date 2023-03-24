source ./init.sh

log_header "Begin testing resource distribution proposal"
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
kevin_commit_amount=$((initial_supply * (100 - liquidity_allocation) / 100 - alice_commit_amount - bond_delegate_amount - bob_commit_amount - lyn_delegate_amount)) # Remainder resource allocation of other.

alice_retirement_length=$((WEEK * 101))
bob_retirement_length=$((maximum_retirement))
kevin_retirement_length=$((minimum_retirement))

source ./default_users.sh

# The test distribution proposal input 
export fund_demand=0
export time_delay=$WEEK

logb "Accelerate time to get more power for DAO's member"
export accelerate=$((WEEK*52))
logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logc "Test distribution proposal on the primary reserve resource (This should distribute 0 resource since the primary reserve resource cannot be distributed)"
export resource=$stable_coin

export sbt=$member_sbt

logb "Test malicious pattern: Alice create the test proposal on the primary reserve resource"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/proposal/distribution_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test ok pattern: Alice vote support the proposal"
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

logb "Test ok pattern: Alice check the proposal status"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice execute the proposal"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
error "$output"
info "$output"
assert_success "$output"

logb "Test failure pattern: Alice try to take her distribution"
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
assert_failure "$output"

logb "Test ok pattern: Alice check distribution resource"
output=`resim run ../rtm/proposal/read_only/check_distribution_resource.rtm`
info  "$output"
assert_success "$output"

# logc "Test distribution proposal on NFTs (This should be failed)" // This test is postponed since resim cannot create NFT

# logb "Test failure pattern: Bob create and deposit the NFTs to the DAO's treasury"
# output=`resim set-default-account $BOB $BOB_PIV`
# export caller=$BOB

# export resource_amount=20
# export resource=`resim new-badge-fixed --name "Distribution NFT" "$resource_amount" | awk '/Resource: / {print $NF}'` 
# logb "Created new nft: $resource"

# export amount=$resource_amount
# output=`resim run ../rtm/deposit.rtm`
# info "$output"
# assert_success "$output"

# output=`resim run ../rtm/dao/proposal/distribution_proposal.rtm`
# assert_failure "$output"

logc "Test distribution proposal on normal fungible resource"

export sbt=$member_sbt
logb "Test ok pattern: Alice create and deposit the fungible resource into treasury"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE

export resource_amount=10000000
export resource=`resim new-token-fixed --name "Distribution Token" "$resource_amount" | awk '/Resource: / {print $NF}'`
logb "Created new distribution resource: $resource"

export amount=$resource_amount
output=`resim run ../rtm/dao/general/deposit.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice create the test distribution proposal on the resource"

output=`resim run ../rtm/dao/proposal/distribution_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test ok pattern: Alice vote support the proposal"
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

logb "Test ok pattern: Alice execute the proposal"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/proposal/execute_proposal.rtm`
error "$output"
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check distribution resource"
output=`resim run ../rtm/proposal/read_only/check_distribution_resource.rtm`
info  "$output"
assert_success "$output"

logc "Everyone check their vote status and withdraw their distribution (except for against voter on the proposal)"

export sbt=$member_sbt

logb "Test ok pattern: Alice check the proposal status"
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check her vote status"
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob check his vote status"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin check his vote status"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt
logb "Test ok pattern: Lyn check her vote status"
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bond check his vote status"
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

export sbt=$member_sbt

logb "Test ok pattern: Alice try to take her distribution"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
info "$output"
assert_success "$output"

logb "Test failure pattern: Bob try to take his distribution but he voted against the proposal"
logy "The transaction have to committed success to remove Bob from beneficiary list"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
error "$output"
assert_success "$output"

logb "Test ok pattern: Kevin try to take his distribution"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt

logb "Test ok pattern: Bond try to take his distribution"
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Lyn try to take her distribution but she voted against the proposal with community $community2_name"
logy "The transaction have to committed success to remove Lyn from benefactor list"
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
error "$output"
assert_success "$output"

logb "Test ok pattern: Alice check distribution resource"
output=`resim run ../rtm/proposal/read_only/check_distribution_resource.rtm`
info  "$output"
assert_success "$output"

completed


