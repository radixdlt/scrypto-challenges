#!/bin/bash

resim set-default-account $address1 $priv_key1

export account=$address1
export work_component=$development_it_component

export job_id=0a0100000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0200000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0300000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0400000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0500000000000000
resim run ./transactions/request_work.rtm

export work_component=$accounting_and_finance_component
export account=$address1

export job_id=0a0100000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0200000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0300000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0400000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0500000000000000
resim run ./transactions/request_work.rtm

export work_component=$digital_marketing_component
export account=$address1

export job_id=0a0100000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0200000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0300000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0400000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0500000000000000
resim run ./transactions/request_work.rtm

export work_component=$graphics_design_component
export account=$address1

export job_id=0a0100000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0200000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0300000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0400000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0500000000000000
resim run ./transactions/request_work.rtm

export work_component=$disputed_work_component
export account=$address1

export job_id=0a0100000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0200000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0300000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0400000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0500000000000000
resim run ./transactions/request_work.rtm

resim set-default-account $address3 $priv_key3
export account=$address3

export work_component=$development_it_component

export job_id=0a0600000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0700000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0800000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0900000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0a00000000000000
resim run ./transactions/request_work.rtm

export work_component=$accounting_and_finance_component

export job_id=0a0600000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0700000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0800000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0900000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0a00000000000000
resim run ./transactions/request_work.rtm

export work_component=$digital_marketing_component

export job_id=0a0600000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0700000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0800000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0900000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0a00000000000000
resim run ./transactions/request_work.rtm

export work_component=$graphics_design_component

export job_id=0a0600000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0700000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0800000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0900000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0a00000000000000
resim run ./transactions/request_work.rtm

export work_component=$disputed_work_component

export job_id=0a0600000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0700000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0800000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0900000000000000
resim run ./transactions/request_work.rtm

export job_id=0a0a00000000000000
resim run ./transactions/request_work.rtm

