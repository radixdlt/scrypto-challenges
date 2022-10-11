
source ./utilities/assert.sh
source ./utilities/log.sh

assert_failure() {
    local transaction="$1"
    assert_contain "$transaction" "COMMITTED FAILURE" "The transaction have to commit failure"
    status=`echo "$transaction" | awk '/Transaction Status: /'`
    logbr $status
    error=`echo "$transaction" | awk '/Panicked at/'`
    if [ ! -z "$error" ] 
    then 
        logr "$error" 
    fi
    log_success
} 

assert_success() {
    local transaction="$1"
    assert_contain "$transaction" "COMMITTED SUCCESS" "The transaction have to commit success"
    status=`echo "$transaction" | awk '/Transaction Status: /'`
    logbg $status
    log_success
}

inspect() {
    local address="$1"
    resim show $address
    false
}

info() {
    local transaction="$1"
    infos=`echo "$transaction" | awk '/[INFO ] /'`
    if [ ! -z "$infos" ] 
    then 
        for info in "${infos[@]}" 
        do 
            logg "$info"
        done
    fi
}

error() {
    local transaction="$1"
    errors=`echo "$transaction" | awk '/ERROR/'`
    if [ ! -z "$errors" ] 
    then 
        for error in "${errors[@]}" 
        do 
            logr "$error"
        done
    fi
}