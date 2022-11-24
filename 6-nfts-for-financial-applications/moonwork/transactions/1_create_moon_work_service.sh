#!/bin/bash

export radix=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag

# Create 4 accounts to test on
export op1=$(resim new-account)
export priv_key1=$(echo $op1 | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export address1=$(echo $op1 | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

export op2=$(resim new-account)
export priv_key2=$(echo $op2 | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export address2=$(echo $op2 | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

export op3=$(resim new-account)
export priv_key3=$(echo $op3 | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export address3=$(echo $op3 | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

export op4=$(resim new-account)
export priv_key4=$(echo $op4 | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export address4=$(echo $op4 | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

# Publish package address
export op1=$(resim publish .)
export package=$(echo $op1 | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

# Create Moon Work Service that has a fee of 5%
# Dispute requirements
# 1 participants allowed on each side (contractor/client) - 1 job completed by contractor - 1 job paid by client
export op2=$(resim run ./transactions/create_moonwork.rtm)
# Save the Moon Work component address
export moon_work_component=$(echo $op2 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")
# Save the service admin badge
export service_admin_badge=$(echo $op2 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
# Save the contractor badge resource address
export contractor_badge=$(echo $op2 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '3q;d')
# Save the client badge resource address
export client_badge=$(echo $op2 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '4q;d')
# Save the work badge resource address
export completed_work_resource=$(echo $op2 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '5q;d')
# Save the DisputedOutcome NFT resource
export dispute_outcome_resource=$(echo $op2 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '6q;d')
# Save the ContractorAccolades NFT resource
export contractor_accolade_resource=$(echo $op2 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '7q;d')
