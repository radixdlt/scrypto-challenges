source ./init.sh

log_header "Begin testing commitment voting mechanism"
logbg "There will be 3 test users: Alice, Bob and Kevin and 1 supreme user: 'other' to test the replacement process"
logbg "You can try changing the input params for testing on file test_commitment_voting_mechanism.sh"

# You can try changing the input params for testing
alice_commit_amount=100000 # Should be the same to compare the vote power
bob_commit_amount=100000 # Should be the same to compare the vote power
kevin_commit_amount=100000 # Should be the same to compare the vote power
other_commit_amount=$((initial_supply * (100 - liquidity_allocation) / 100 - alice_commit_amount - bob_commit_amount - kevin_commit_amount ))

alice_retirement_length=$minimum_retirement # Should be smallest of three
bob_retirement_length=$((WEEK * 101)) # Should be in the middle of three
kevin_retirement_length=$maximum_retirement # Should be largest of three
other_retirement_length=$maximum_retirement

logc "ALICE - Commitment Voting Tester - DAO's member no.1"

export sbt=$member_sbt

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export ALICE=`echo $output | cut -d " " -f1`
export ALICE_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export commit_amount="$alice_commit_amount"

logb "Funding Alice's account"
export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

logb "Test failed pattern: Alice want to become DAO member with retirement time less than minimum retirement length"
export retirement_length=$((minimum_retirement - 1))
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_failure "$output"

logb "Test failed pattern: Alice want to become DAO member with retirement time more than maximum retirement length"
export retirement_length=$((maximum_retirement + 1 ))
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_failure "$output"

logb "Test ok pattern: Alice want to commit $alice_commit_amount ALIGN and become DAO member with retirement length: $((alice_retirement_length / WEEK)) weeks"
export retirement_length=$alice_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

logb "Test ok pattern: Alice get her current vote power"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logc "BOB - Commitment Voting Tester - DAO's member no.2"

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export BOB=`echo $output | cut -d " " -f1`
export BOB_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
export commit_amount="$bob_commit_amount"

logb "Funding Bob's account"
export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

logb "Test ok pattern: Bob want to commit $bob_commit_amount ALIGN and become DAO member with retirement length: $((bob_retirement_length / WEEK)) weeks"
export retirement_length=$bob_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

logb "Test ok pattern: Bob get his current vote power"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logc "KEVIN - Commitment Voting Tester - DAO's member no.3"

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export KEVIN=`echo $output | cut -d " " -f1`
export KEVIN_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
export commit_amount="$kevin_commit_amount"

logb "Funding Kevin's account"
export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin want to commit $kevin_commit_amount ALIGN and become DAO member with retirement length: $((kevin_retirement_length / WEEK)) weeks"
export retirement_length=$kevin_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

logb "Test ok pattern: Kevin get his current vote power"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logc "OTHER - Supreme user to test the DAO's Replacement Process"

output=`resim new-account | awk '/Account component address: |Private key: / {print $NF}'`
export OTHER=`echo $output | cut -d " " -f1`
export OTHER_PIV=`echo $output | cut -d " " -f2`

output=`resim set-default-account $OTHER $OTHER_PIV`
export caller=$OTHER
export commit_amount="$other_commit_amount"

logb "Funding Other's account"
export align_withdraw_amount=$commit_amount
output=`resim run ../rtm/init/withdraw_align.rtm`
assert_success "$output"

logb "Test ok pattern: Other want to commit $other_commit_amount ALIGN and become DAO member with retirement length: $((other_retirement_length / WEEK)) weeks"
export retirement_length=$other_retirement_length
output=`resim run ../rtm/dao/general/become_member.rtm`
assert_success "$output"

logb "Test ok pattern: Other get their current vote power"
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "== 1 year later... =="
advance_time $((WEEK * 52))

logc "Each person recheck his/her vote power"

logb "ALICE"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "BOB"
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "KEVIN"
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logb "OTHER"
output=`resim set-default-account $OTHER $OTHER_PIV`
export caller=$OTHER
output=`resim run ../rtm/dao/member_delegator/show_vote_power.rtm`
info "$output"
assert_success "$output"

logc "Test Retirement Process"
logb "== ALICE retirement, she try something funny go against smartcontract logic! =="

logb "Test failed pattern: Alice try to withdraw before retirement"
output=`resim set-default-account $ALICE $ALICE_PIV`
export caller=$ALICE
export amount="1"
output=`resim run ../rtm/dao/member_delegator/withdraw_by_amount.rtm`
assert_failure "$output"

logb "Test ok pattern: Alice try to retire"
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_success "$output"

before_align=`resim show $ALICE | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Alice try to withdraw again, current ALIGN amount in account: $before_align"
logy "This transaction should return no resources since Alice haven't gone through the minimum retirement time yet"
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Alice account again if it's the same"
check_resource_eq $before_align $ALICE

logb "== BOB retirement =="
output=`resim set-default-account $BOB $BOB_PIV`
export caller=$BOB

logb "Test ok pattern: Bob try to retire"
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_success "$output"

logb "== KEVIN retirement =="
output=`resim set-default-account $KEVIN $KEVIN_PIV`
export caller=$KEVIN

logb "Test ok pattern: Kevin try to retire"
output=`resim run ../rtm/dao/member_only/retire.rtm`
assert_success "$output"

logb "== $((minimum_retirement / WEEK)) weeks later... =="
advance_time $((minimum_retirement))

before_align=`resim show $ALICE | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Alice try to withdraw resource again, now should be able to withdraw all, current ALIGN amount in account: $before_align"
logy "This transaction should return all of her committed account"
export caller=$ALICE
output=`resim set-default-account $ALICE $ALICE_PIV`
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Alice account again if it's the same as initial committed amount: $alice_commit_amount"
check_resource_eq $alice_commit_amount $ALICE

before_align=`resim show $BOB | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Bob try to withdraw resource, current ALIGN amount in account: $before_align"
logy "This transaction should return a period worth of his committed resource"
export caller=$BOB
output=`resim set-default-account $BOB $BOB_PIV`
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Bob account again if it's not the same"
check_resource_neq $before_align $BOB

before_align=`resim show $KEVIN | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Kevin try to withdraw resource, current ALIGN amount in account: $before_align"
logy "This transaction should return a period worth of his committed resource"
export caller=$KEVIN
output=`resim set-default-account $KEVIN $KEVIN_PIV`
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Kevin account again if it's not the same"
check_resource_neq $before_align $KEVIN

logb "== $(( (bob_retirement_length - minimum_retirement) / WEEK)) weeks later... =="
advance_time $((bob_retirement_length - minimum_retirement))

before_align=`resim show $BOB | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Bob try to withdraw resource again, now should be able to withdraw all, current ALIGN amount in account: $before_align"
export caller=$BOB
output=`resim set-default-account $BOB $BOB_PIV`
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Bob resource again if it's the same as initial committed amount: $bob_commit_amount"
check_resource_eq $bob_commit_amount $BOB

before_align=`resim show $KEVIN | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Kevin try to withdraw resource, current ALIGN amount in account: $before_align"
logy "This transaction should return a part of his committed resource"
export caller=$KEVIN
output=`resim set-default-account $KEVIN $KEVIN_PIV`
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Kevin account again if it's not the same"
check_resource_neq $before_align $KEVIN

logc "Test Replacement process"

export id=`resim show $KEVIN | grep -oP '(?<=id: NonFungibleId).*(?=, immutable_data)' | sed 's/"//g' | sed 's/(//' | sed 's/)//'`
logb "Test malicious pattern: Kevin want to try call the replacement method for his account"
logy "The transaction cannot go through because the method is protected by Access Rule"
output=`resim run ../rtm/dao/malicious/malicious_accept_replacement.rtm`
assert_failure "$output"

logb "Test malicious pattern: Now Kevin try call the replacement method with the proof created from the 'dao_proof' method to unlock his account"
logy "he transaction cannot go through because the method is protected by Access Rule"
output=`resim run ../rtm/dao/malicious/malicious_dao_proof.rtm`
assert_failure "$output"

logb "Test ok pattern: Kevin find a suitable replacement and make a proposal to end his current retirement process on 1 week"
export time_delay=$WEEK
output=`resim run ../rtm/dao/member_only/replacement_proposal.rtm`
assert_success "$output"
export proposal=`echo "$output" | awk '/Component: / {print $NF}'`

logb "Test malicious pattern: Kevin want to vote support his proposal while already on retirement"
export vote=true
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
assert_failure "$output"

logb "Test ok pattern: Other vote accept on Kevin's proposal"
export caller=$OTHER
output=`resim set-default-account $OTHER $OTHER_PIV`
output=`resim run ../rtm/proposal/user_pattern/vote.rtm`
assert_success "$output"

logc "1 week later..."
advance_time $WEEK
export caller=$KEVIN
output=`resim set-default-account $KEVIN $KEVIN_PIV`

logb "Test ok pattern: Kevin execute the accepted proposal"
output=`resim run ../rtm/proposal/execute_proposal.rtm`
info "$output"
assert_success "$output"

before_align=`resim show $KEVIN | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
logb "Test ok pattern: Kevin try to withdraw resource again, now should be able to withdraw all, current ALIGN amount in account: $before_align"
output=`resim run ../rtm/dao/member_delegator/withdraw_all.rtm`
info "$output"
assert_success "$output"
logb "Check Kevin resource again if it's the same as initial committed amount: $kevin_commit_amount"
check_resource_eq $kevin_commit_amount $KEVIN

completed