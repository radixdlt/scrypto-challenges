source ./init.sh

log_header "Begin testing user design pattern with 3 type of users: member, representative, delegator"
logg "There will be 3 test users: Alice, Bob and Kevin"

# The input params in this test doesn't matter if it worked and in range
alice_commit_amount="310215" # must be in range of ALIGN initial_supply * (100 - liquidity_allocation) / 100
bob_commit_amount="134659" # must be in range of ALIGN initial_supply * (100 - liquidity_allocation) / 100
kevin_delegate_amount="365412" # must be in range of ALIGN initial_supply * (100 - liquidity_allocation) / 100
alice_retirement_length=$minimum_retirement # must be in range from minimum_retirement to maximum_retirement
bob_retirement_length=$maximum_retirement # must be in range from minimum_retirement to maximum_retirement

logc "ALICE - DAO's member become a Representative"
export sbt=$member_sbt
output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export ALICE=`echo $output | cut -d " " -f1`
export ALICE_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export commit_amount=$alice_commit_amount

export align_withdraw_amount=$alice_commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

logb "Test ok pattern: Alice want to become DAO member with retirement length: $((alice_retirement_length / WEEK)) weeks"
export retirement_length=$alice_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

logb "Test ok pattern: Alice want to become a representative and name her community: DEMOCRAT"
export tax_percent="0.5"
export name="DEMOCRAT"
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_success "$output"
DEMOCRAT=`echo "$output" | awk '/Component: / {print $NF}'`

logc "BOB - DAO's member"

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export BOB=`echo $output | cut -d " " -f1`
export BOB_PIV=`echo $output | cut -d " " -f2`
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export commit_amount=$bob_commit_amount
export align_withdraw_amount=$bob_commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

logb "Test ok pattern: Bob want to become DAO member with with retirement length: $((bob_retirement_length / WEEK)) weeks"
export retirement_length=$bob_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

logb "Test ok pattern: Bob want to become a representative and name his community: REPUBLICAN"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export tax_percent="0.4"
export name="REPUBLICAN"
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_success "$output"
REPUBLICAN=`echo "$output" | awk '/Component: / {print $NF}'`

logc "KEVIN - Delegator"

export sbt=$delegator_sbt
output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export KEVIN=`echo $output | cut -d " " -f1`
export KEVIN_PIV=`echo $output | cut -d " " -f2`
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export delegate_amount=$kevin_delegate_amount
export align_withdraw_amount=$delegate_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin want to become a delegator with his resource"
output=`resim run ../rtm/dao/general/become_delegator.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin request to join DEMOCRAT community"
export community_address=$DEMOCRAT
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Alice accept Kevin's request"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export id=1
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin join DEMOCRAT community"
export community_address=$DEMOCRAT
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin check his current community"
output=`resim run ../rtm/dao/member_delegator/check_community.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin add resource to his delegator account"
output=`resim set-default-account $KEVIN $KEVIN_PIV`

export add_amount="36585"
export align_withdraw_amount=$add_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"
output=`resim run ../rtm/dao/delegator_only/add_delegate.rtm`
assert_success "$output"

log_header "Begin testing withdraw design pattern with current users: Alice, Bob and Kevin"

logc "KEVIN withdrawal"

logb "Test ok pattern: Kevin try to withdraw after delegate"
export amount=$add_amount
output=`resim run ../rtm/dao/member_delegator/withdraw_by_amount.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin try to withdraw all"
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
assert_success "$output"

logb "Check Kevin account if it's the same as initial delegated amount plus add amount: $((kevin_delegate_amount + add_amount))"
check_resource_eq $((kevin_delegate_amount + add_amount)) $KEVIN

logb "Test ok pattern: Bob try to retire"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin quit the DEMOCRAT community"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export community_address=$DEMOCRAT
output=`resim run ../rtm/community/user_pattern/quit_community.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice try to retire"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_success "$output"

logc "ALICE withdrawal"

logc "$((minimum_retirement / WEEK)) weeks later..."
advance_time $((minimum_retirement))
output=`resim set-default-account $ALICE $ALICE_PIV`

before_align=`resim show $ALICE | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Alice try to withdraw all resources, current ALIGN amount in account: $before_align"
logy "This transaction should return all of her committed account"
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Alice account again if it's the same as initial committed amount: $alice_commit_amount"
check_resource_eq $alice_commit_amount $ALICE

logc "BOB withdrawal"
advance_time $((maximum_retirement - minimum_retirement))
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB

before_align=`resim show $BOB | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Bob try to withdraw all resources, current ALIGN amount in account: $before_align"
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Bob resource again if it's the same as initial committed amount: $bob_commit_amount"
check_resource_eq $bob_commit_amount $BOB

completed