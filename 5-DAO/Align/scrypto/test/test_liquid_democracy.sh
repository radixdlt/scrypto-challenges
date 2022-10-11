source ./init.sh

log_header "Begin testing Liquid Democracy governance technique"
logbg "There will be 5 test users: Alice as the first Representative, Bob as the second Representative, Kevin as the DAO's member, Lyn as the first Delegator, Bond as the second Delegator"
logbg "You can try changing the input params for testing on file test_liquid_democracy_mechanism.sh"

# You can try changing the input params for testing
community1_name="DEMOCRAT"
tax1_percent="0.4"
community2_name="REPUBLICAN"
tax2_percent="0.8"
alice_commit_amount=$((representative_requirement - 1)) # Should be smaller than the representative_requirement to test that input params
bob_commit_amount=$((representative_requirement - 1)) # Should be the same to compare the vote power
kevin_commit_amount=$((representative_requirement - 1)) # Should be the same to compare the vote power
lyn_delegate_amount=$((representative_requirement - 1)) # Should be the same to compare the vote power
bond_delegate_amount=$((representative_requirement - 1)) # Should be the same to compare the vote power

alice_retirement_length=$((WEEK * 101)) # Should be the same to compare the vote power
bob_retirement_length=$((WEEK * 101)) # Should be the same to compare the vote power
kevin_retirement_length=$((WEEK * 101)) # Should be the same to compare the vote power

logc "Each person create their individual accounts"

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

logb "Test ok pattern: Alice want to commit $alice_commit_amount ALIGN and become DAO member with retirement length: $((alice_retirement_length / WEEK)) weeks"
export retirement_length=$alice_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

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

logb "Test ok pattern: Bob want to commit $commit_amount ALIGN and become DAO member with retirement length: $((bob_retirement_length / WEEK)) weeks"
export retirement_length=$bob_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

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

logb "Test ok pattern: Kevin want to commit $commit_amount ALIGN and become DAO member with retirement length: $((kevin_retirement_length / WEEK)) weeks"
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

logb "Test ok pattern: Lyn want to delegate $delegate_amount ALIGN and become a Delegator"
output=`resim run ../rtm/dao/general/become_delegator.rtm`
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

logb "Test ok pattern: Bond want to delegate $delegate_amount ALIGN and become a Delegator"
output=`resim run ../rtm/dao/general/become_delegator.rtm`
assert_success "$output"

logc "Alice and Bob become representatives"
logb "== Alice - Representative no.1 =="

output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE

logb "Test failed pattern: Alice want to become a representative when doesn't have enough voting power"
export tax_percent=$tax1_percent
export name=$community1_name
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_failure "$output"

logb "Test ok pattern: Alice get her current vote power"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
assert_success "$output"
info "$output"

logb "1 week later..."
advance_time $((WEEK * 1))

logb "Test ok pattern: Alice get her current vote power again"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
assert_success "$output"
info "$output"

logb "Test ok pattern: Alice want to become a representative again and name her community: $community1_name"
export tax_percent=$tax1_percent
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_success "$output"
COMMUNITY1=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test malicious pattern: Alice want to become a representative again while already running a community"
export name="REPUBLICAN"
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
assert_failure "$output"

logb "Test ok pattern: Alice check her current vote power when there's no people follow her community"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "== Bob - Representative no.2 - Try to join Alice community but later just host his own community =="

output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB

logb "Test ok pattern: Bob request to join the $community1_name community run by Alice"
export community_address=$COMMUNITY1
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Alice reject Bob's request"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export id=1
export result=false
output=`resim run ../rtm/community/representative/review_request.rtm`
info "$output"
assert_success "$output"

logb "Test failed pattern: Bob try to join after rejected"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_failure "$output"

logb "Test ok pattern: Bob contact Alice and request to join the $community1_name community again"
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Alice accept Bob's request"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export id=2
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob ignored Alice acceptance on joining the $community1_name community and want to run a community on his own, named: $community2_name"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export name=$community2_name
export tax_percent=$tax2_percent
output=`resim run ../rtm/dao/member_only/become_representative.rtm`
info "$output"
assert_success "$output"
COMMUNITY2=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test malicious pattern: Bob try to join DEMOCRAT community after already representing a community"
logy "This transaction have to success to remove Bob's request from the DEMOCRAT community"
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
error "$output"
assert_success "$output"

logb "Test ok pattern: Bob check his current vote power when there's no people follow his community"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logc "Test Power Taxing Mechanism - Kevin, Lyn and Bond join Alice and Bob's communities"

logb "== Kevin - DAO's Member - Request to join both communities but later join Alice's community =="

output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN

logb "Test ok pattern: Kevin request to join $community1_name community"
export community_address=$COMMUNITY1
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin request to join $community2_name community"
export community_address=$COMMUNITY2
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Alice accept Kevin's request"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export community_address=$COMMUNITY1
export id=3
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

logb "Test ok pattern: Bob accept Kevin's request"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export community_address=$COMMUNITY2
export id=1
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin check his current vote power before joined Alice's community"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin join $community1_name community"
export community_address=$COMMUNITY1
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

logb "Test malicious pattern: Kevin try to join $community2_name community after already joined $community1_name community"
logy "This transaction have to success to remove Kevin's request from the $community2_name community"
export community_address=$COMMUNITY2
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
error "$output"
assert_success "$output"

logb "Test ok pattern: Kevin check his current community"
output=`resim run ../rtm/dao/member_delegator/check_community.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin check his current vote power after joined Alice's community"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check her current vote power after Kevin has joined"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "== Lyn - Delegator - Join Bob's community =="

export sbt=$delegator_sbt
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN

logb "Test ok pattern: Lyn request to join $community2_name community"
export community_address=$COMMUNITY2
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Bob accept Lyn's request"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export id=2
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

logb "Test ok pattern: Lyn check her current vote power before joined Bob's community"
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Lyn join $community2_name community"
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Lyn check her current community"
output=`resim run ../rtm/dao/member_delegator/check_community.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Lyn check her current vote power after joined Bob's community"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob check his current vote power after Lyn has joined"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "== Bond - Delegator - Join Alice's community =="

export sbt=$delegator_sbt

output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND

logb "Test ok pattern: Bond request to join $community1_name community"
export community_address=$COMMUNITY1
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

logb "Test ok pattern: ALice accept Bond's request"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export id=4
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

logb "Test ok pattern: Bond check her current vote power before joined Alice's community"
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bond join $community1_name community"
output=`resim set-default-account $BOND $BOND_PIV`
export caller=$BOND
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Bond check his current community"
output=`resim run ../rtm/dao/member_delegator/check_community.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bond check his current vote power after joined Alice's community"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check her current vote power after Kevin, Bond has joined"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "== Alice amend her community tax policy =="
logb "Test ok pattern: Alice amend the community tax policy"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export new_tax=0.8
output=`resim run ../rtm/community/representative/amend_tax_policy.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check her current vote power after amended the tax policy"
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logc "Test Liquid Democracy in voting"

logb "Test ok pattern: Kevin make a random proposal"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export component_address=$dao_comp
export method="amend_treasury_policy"
export input=0.5
export fund_demand=200
export time_delay=$proposal_minimum_delay
export args="1001000000a1200000000000b2d3595bf006000000000000000000000000000000000000000000000000" # Encoded hex of Decimal("0.5"), you can try run the encode function at the end of lib.rs to get the input hex.
output=`resim run ../rtm/dao/proposal/one_method_proposal.rtm`
info "$output"
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test failed pattern: Kevin vote accept the proposal when already following a community"
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
assert_failure "$output"

logb "Test ok pattern: Alice vote accept the proposal"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check the proposal status"
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check her direct vote on the proposal"
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Kevin check his indirect vote on the proposal"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bond check his indirect vote on the proposal"
output=`resim set-default-account $BOND $BOND_PIV`
export sbt=$delegator_sbt
export caller=$BOND
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

logc "Test Rage Quitting & Credibility Score System"

logb "Test ok pattern: Bond somehow unhappy with Alice's vote so he rage quitted her community"
output=`resim run ../rtm/dao/member_delegator/rage_quit.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bond check if his vote got retracted or not"
output=`resim run ../rtm/proposal/user_pattern/check_vote.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check the proposal status after Bond has rage quitted"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/proposal/read_only/vote_status.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice check her vote power after Bond has rage quitted"
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Some more people become delegator and join Bob's community"

export sbt=$delegator_sbt
output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export DELEGATOR1=`echo $output | cut -d " " -f1`
export DELEGATOR1_PIV=`echo $output | cut -d " " -f2`
output=`resim set-default-account $DELEGATOR1 $DELEGATOR1_PIV`
export caller=$DELEGATOR1
export delegate_amount=1
export align_withdraw_amount=$delegate_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"
output=`resim run ../rtm/dao/general/become_delegator.rtm`
assert_success "$output"
export community_address=$COMMUNITY2
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export DELEGATOR2=`echo $output | cut -d " " -f1`
export DELEGATOR2_PIV=`echo $output | cut -d " " -f2`
output=`resim set-default-account $DELEGATOR2 $DELEGATOR2_PIV`
export caller=$DELEGATOR2
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"
output=`resim run ../rtm/dao/general/become_delegator.rtm`
assert_success "$output"
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export DELEGATOR3=`echo $output | cut -d " " -f1`
export DELEGATOR3_PIV=`echo $output | cut -d " " -f2`
output=`resim set-default-account $DELEGATOR3 $DELEGATOR3_PIV`
export caller=$DELEGATOR3
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"
output=`resim run ../rtm/dao/general/become_delegator.rtm`
assert_success "$output"
output=`resim run ../rtm/community/user_pattern/request_join_community.rtm`
assert_success "$output"

output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export id=3
export result=true
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

export id=4
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

export id=5
output=`resim run ../rtm/community/representative/review_request.rtm`
assert_success "$output"

output=`resim set-default-account $DELEGATOR1 $DELEGATOR1_PIV`
export caller=$DELEGATOR1
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

output=`resim set-default-account $DELEGATOR2 $DELEGATOR2_PIV`
export caller=$DELEGATOR2
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

output=`resim set-default-account $DELEGATOR3 $DELEGATOR3_PIV`
export caller=$DELEGATOR3
output=`resim run ../rtm/community/user_pattern/join_community.rtm`
assert_success "$output"

logb "Test ok pattern: Bob check his current vote power"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Lyn rage quitted Bob's community"
output=`resim set-default-account $LYN $LYN_PIV`
export caller=$LYN
export sbt=$delegator_sbt
output=`resim run ../rtm/dao/member_delegator/rage_quit.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob check his current vote power again"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Second delegator rage quitted Bob's community"
output=`resim set-default-account $DELEGATOR1 $DELEGATOR1_PIV`
export caller=$DELEGATOR1
export sbt=$delegator_sbt
output=`resim run ../rtm/dao/member_delegator/rage_quit.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob check his current vote power again"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Third delegator rage quitted Bob's community"
output=`resim set-default-account $DELEGATOR2 $DELEGATOR2_PIV`
export caller=$DELEGATOR2
export sbt=$delegator_sbt
output=`resim run ../rtm/dao/member_delegator/rage_quit.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob check his current vote power again"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Fourth delegator rage quitted Bob's community"
output=`resim set-default-account $DELEGATOR3 $DELEGATOR3_PIV`
export caller=$DELEGATOR3
export sbt=$delegator_sbt
output=`resim run ../rtm/dao/member_delegator/rage_quit.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob check his current vote power again"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Bob check his account again"
output=`resim run ../rtm/dao/member_delegator/show_account.rtm`
info "$output"
assert_success "$output"

logc "Test Representative's Retirement"

logb "Test failed pattern: Alice try to retire when there's still Kevin following"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_failure "$output"

logb "Test ok pattern: Kevin quit the DEMOCRAT community"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export community_address=$COMMUNITY1
output=`resim run ../rtm/community/user_pattern/quit_community.rtm`
info "$output"
assert_success "$output"

logb "Test ok pattern: Alice try to retire again"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export sbt=$member_sbt
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_success "$output"

completed