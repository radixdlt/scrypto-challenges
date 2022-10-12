source ./init.sh

log_header "Begin testing real use case"
logbg "You can try changing the input params for testing on file test_real_use_cases.sh"

# You can try changing the input params for testing
community1_name="DEMOCRAT"
tax1_percent="0.4"
community2_name="REPUBLICAN"
tax2_percent="0.8"
alice_commit_amount=$((representative_requirement))
bond_delegate_amount=$((665587))
bob_commit_amount=$((representative_requirement)) 
lyn_delegate_amount=$((965421)) 
kevin_commit_amount=$((initial_supply * (100 - liquidity_allocation) / 100 - alice_commit_amount - bond_delegate_amount - bob_commit_amount - lyn_delegate_amount)) # Remainder resource allocation of other

alice_retirement_length=$((WEEK * 101))
bob_retirement_length=$((maximum_retirement))
kevin_retirement_length=$((minimum_retirement))

source ./default_users.sh

logb "Kevin want to build a Car Assembly Workshop for the DAO and will do a fund raising to get more leverage on the project"
logb "Test ok pattern: Kevin instantiate the Proposal Component and call the 'new_fund_raising_protocol' method to instantiate new Fund Raising Component"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export fee=0.5
export name="Car Assembly Workshop FundRaising"
output=`resim run ../rtm/test/use_case_test_init.rtm`
info "$output"
assert_success "$output"
export test_proposal_comp=`echo "$output" | awk '/test proposal component: / {print $NF}'` 
export bond_token=`echo "$output" | awk '/TestFundraising Bond: / {print $NF}'` 
export fund_raising_comp=`echo "$output" | awk '/TestFundraising Component: / {print $NF}'` 

logc "Test Funding public goods"
logb "Test ok pattern: Kevin make a proposal for DAO to invest on the fundraising and distribute the bond token for all supporters"
logy "He also demand 1000 stable coins to pay for his work on this protocol and the idea."
export component_address=$test_proposal_comp
export method="invest_through_dao"
export args="1001000000a120000000000000a1edccce1bc2d300000000000000000000000000000000000000000000" # Encoded hex of Decimal("1000000"), you can try run the test function at the end of lib.rs to get the input hex
export resource=$bond_token
export fund_demand=1000
export time_delay=$WEEK
output=`resim run ../rtm/test/test_proposal.rtm`
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test ok pattern: Kevin vote support the proposal"
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

logb "Test ok pattern: Bob vote support the proposal"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test ok pattern: Kevin check the proposal status"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin execute the proposal"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin take the allocated fund for the proposal"
output=`resim run ../rtm/proposal/proposer_only/withdraw_fund.rtm`
info "$output"
assert_success "$output"

export sbt=$member_sbt
logb "Test ok pattern: Kevin take his distribution"
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
info "$output"
assert_success "$output"

logb "Test malicious pattern: Alice try to take her distribution but she voted against the proposal"
logy "The transaction have to committed success to remove Lyn from benefactor list"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
error "$output"
assert_success "$output"

export sbt=$delegator_sbt
logb "Test ok pattern: Alice advise her follower - Bond to quit her community and do rage withdraw"
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
info "$output"
assert_success "$output"

output=`resim run ../rtm/dao/member_delegator/rage_withdraw.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob take his distribution"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
info "$output"
assert_success "$output"

export sbt=$delegator_sbt
logb "Test ok pattern: Lyn take her distribution"
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/proposal/user_pattern/take_distribution.rtm`
info "$output"
assert_success "$output"

export sbt=$member_sbt
logb "Test ok pattern: Now Kevin make a proposal for the DAO to assign him 500000 token fund for renting place, building infrastructure and buying equipments."
logy "He also show every other members the legal contracts that he signed to earn more trust."
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export component_address=$test_proposal_comp
export method="assign_fund"
export args="1001000000a120000000000080d07666e70de16900000000000000000000000000000000000000000000" # Encoded hex of Decimal("500000"), you can try run the test function at the end of lib.rs to get the input hex
export fund_demand=0
export time_delay=$WEEK
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test ok pattern: Kevin vote support the proposal"
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

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test ok pattern: Kevin check the proposal status"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin execute the proposal"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
info "$output"
error "$output"
assert_success "$output"

export after_time=$((WEEK * 52))
logb "$((after_time / WEEK)) weeks later..."
advance_time $after_time

output=`resim set-default-account $ADMIN_ACC $ADMIN_PIV`
export project_lead=$KEVIN
export amount=200000
output=`resim run ../rtm/test/give_profit.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin profited 200000 from the business and use the profit method on the fund raising component to distribute them again for shareholders"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export amount=700000
output=`resim run ../rtm/test/profit.rtm`
info "$output"
error "$output"
assert_success "$output"

logb "Test ok pattern: Lyn use her bonds to reclaim both initial fund and her share of profit"
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/test/claim_profit.rtm`
info "$output"
assert_success "$output"

logc "Test Protocol's maintenance and upgrades"
logb "Test ok pattern: Since there's more fund going in the fund raising platform but Kevin would not need those yet, he decided to make a proposal for the DAO to raise the investment fee"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export component_address=$fund_raising_comp
export method="change_fee"
export args="1001000000a120000000000064a7b3b6e00d000000000000000000000000000000000000000000000000" # Encoded hex of Decimal("1"), you can try run the test function at the end of lib.rs to get the input hex
export fund_demand=0
export time_delay=$WEEK
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test ok pattern: Kevin vote support the proposal"
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

logb "$((time_delay / WEEK)) weeks later..."
advance_time $time_delay

logb "Test ok pattern: Kevin check the proposal status"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin execute the proposal"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
info "$output"
error "$output"
assert_success "$output"

completed