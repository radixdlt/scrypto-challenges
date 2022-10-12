logbg "There will be 5 default test users: Alice as the first Representative, Bob as the second Representative, Kevin as the DAO's member, Lyn as the first Delegator, Bond as the second Delegator"
logbg "Lyn follow Bob's community while Bond follow Alice's community"

logb "== Alice =="

export sbt=$member_sbt

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export ALICE=`echo $output | cut -d " " -f1`
export ALICE_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export commit_amount=$alice_commit_amount

export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

export retirement_length=$alice_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

export tax_percent=$tax1_percent
export name=$community1_name
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_success "$output"
export COMMUNITY1=`echo "$output" | awk '/Component: / {print $NF}'`

logb "== Bob =="

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export BOB=`echo $output | cut -d " " -f1`
export BOB_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export commit_amount=$bob_commit_amount

export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

export retirement_length=$bob_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

export name=$community2_name
export tax_percent=$tax2_percent
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_success "$output"
export COMMUNITY2=`echo "$output" | awk '/Component: / {print $NF}'`

logb "== Kevin =="

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export KEVIN=`echo $output | cut -d " " -f1`
export KEVIN_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export commit_amount=$kevin_commit_amount

export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

export retirement_length=$kevin_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

logb "== Lyn =="

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export LYN=`echo $output | cut -d " " -f1`
export LYN_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
export delegate_amount=$lyn_delegate_amount

export align_withdraw_amount=$delegate_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

output=`resim run ../rtm/dao/general/become_delegator.rtm`
assert_success "$output"

export sbt=$delegator_sbt

export community_address=$COMMUNITY2
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export id=1
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

logb "== Bond =="

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export BOND=`echo $output | cut -d " " -f1`
export BOND_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
export delegate_amount=$bond_delegate_amount

export align_withdraw_amount=$delegate_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

output=`resim run ../rtm/dao/general/become_delegator.rtm`
assert_success "$output"

export community_address=$COMMUNITY1
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export id=1
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"