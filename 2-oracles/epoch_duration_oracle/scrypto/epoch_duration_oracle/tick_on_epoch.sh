#!/bin/bash

scrypto build && \
resim reset

export owner_account=$(resim new-account)

echo "${owner_account}"

export owner=$(echo "${owner_account}" | grep "Account component address:" | cut -d: -f2 | xargs)
export owner_private_key=$(echo "${owner_account}" | grep "Private key:" | cut -d: -f2 | xargs)

export package=$(resim publish target/wasm32-unknown-unknown/release/epoch_duration_oracle.wasm | grep "New Package" | cut -d: -f2 | xargs)

export timestamp=$(date +"%s%3N")
export epoch=$(curl -s GET -k 'https://pte01.radixdlt.com/epoch' | jq --raw-output .epoch)
export epoch_right_after=$((epoch + 1))

cat <<EOT > manifests/create_epoch_duration_oracle.manifest
CALL_FUNCTION PackageAddress("${package}") "EpochDurationOracle" "new_with_bootstrap" ${epoch}u64 ${timestamp}u64;
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("${owner}") "deposit_batch";
EOT

export oracle=$(resim run manifests/create_epoch_duration_oracle.manifest | grep "Component:" | cut -d: -f2 | xargs)

export owner_badge=$(resim show ${owner} | grep "Owner of" | cut -d: -f3 | cut -d, -f1 | xargs)

cat <<EOT > manifests/since_epoch_0.manifest
CALL_METHOD ComponentAddress("${oracle}") "millis_since_epoch" 0u64;
EOT

cat <<EOT > manifests/since_epoch_${epoch}.manifest
CALL_METHOD ComponentAddress("${oracle}") "millis_since_epoch" ${epoch}u64;
EOT

cat <<EOT > manifests/since_epoch_${epoch_right_after}.manifest
CALL_METHOD ComponentAddress("${oracle}") "millis_since_epoch" ${epoch_right_after}u64;
EOT

last_epoch=${epoch}
timestamp_at_last_epoch=${timestamp}

while :
do
    timestamp=$(date +"%s%3N")
    time_elapsed_in_epoch="$((timestamp - timestamp_at_last_epoch))"
    epoch=$(curl -s GET -k 'https://pte01.radixdlt.com/epoch' | jq --raw-output .epoch)

    echo "Current epoch ${epoch}, last epoch ${last_epoch}..."

    if [[ $((epoch - last_epoch)) > 0 ]]
    then
        resim set-current-epoch ${epoch}
        timestamp_at_last_epoch=${timestamp}
        last_epoch=${epoch}

        echo "CALL_METHOD ComponentAddress(\""${owner}"\") \"create_proof_by_amount\" Decimal(\"1\") ResourceAddress(\""${owner_badge}"\");" > manifests/tick.manifest
        echo "CALL_METHOD ComponentAddress(\""${oracle}"\") \"tick\" ${time_elapsed_in_epoch}u64;" >> manifests/tick.manifest

        resim run manifests/tick.manifest

        time_elapsed_in_epoch=0
    fi
done