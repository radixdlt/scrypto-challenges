time_update () {
    local time=$1
    export time=$time

    logbg "Update oracle unix time to $time"

    output=`resim run ./neuracle/round.rtm`
    assert_success "$output"

}

advance_epoch () {

    local advance=$1
    export epoch=$((epoch+advance))
    logbg "Advance epoch to $epoch"
    resim set-current-epoch $epoch

}

advance_time () {

    local advance=$1

    advance_epoch 1

    output=`resim set-default-account $ADMIN_ACC $ADMIN_PIV`

    export time=$((time+advance))

    logbg "Update oracle unix time to $time"

    output=`resim run ./neuracle/round.rtm`
    assert_success "$output"

}

check_resource_eq () {
    local before=$1
    local account=$2
    current=`resim show $account | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
    logy "New ALIGN amount: $current"
    assert_eq $before $current
    log_success
}

check_resource_neq () {
    local before=$1
    local account=$2
    current=`resim show $account | grep -oP '(?<=amount: ).*(?=Align DAO Share)' | cut -d "," -f1`
    logy "New ALIGN amount: $current"
    assert_not_eq $before $current
    log_success
}