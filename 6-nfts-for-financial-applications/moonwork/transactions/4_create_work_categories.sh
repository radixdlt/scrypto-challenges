#!/bin/bash

resim set-default-account $address1 $priv_key1
export account=$address1

export category="Development & IT"
export op1=$(resim run ./transactions/create_work_category.rtm)
export development_it_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export development_it_dispute_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export development_it_resource=$(echo $op1 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')

export category="Accounting & Finance"
export op1=$(resim run ./transactions/create_work_category.rtm)
export accounting_and_finance_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export accounting_and_finance_dispute_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export accounting_and_finance_resource=$(echo $op1 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')

export category="Digital Marketing"
export op1=$(resim run ./transactions/create_work_category.rtm)
export digital_marketing_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export digital_marketing_dispute_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export digital_marketing_resource=$(echo $op1 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')

export category="Graphics Design"
export op1=$(resim run ./transactions/create_work_category.rtm)
export graphics_design_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export graphics_design_dispute_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export graphics_design_resource=$(echo $op1 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')

export category="Disputed Category"
export op1=$(resim run ./transactions/create_work_category.rtm)
export disputed_work_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export disputed_work_dispute_component=$(echo $op1 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export disputed_work_resource=$(echo $op1 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
