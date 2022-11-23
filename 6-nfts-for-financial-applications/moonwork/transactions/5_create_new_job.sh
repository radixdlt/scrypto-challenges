#!/bin/bash

resim set-default-account $address2 $priv_key2
export account=$address2

export work_component=$development_it_component
export total_compensation='1'
export work_title='Dex'
export work_description='Create a DEX in Scrypto'
export skills_required='"scrypto", "rustlang"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

export work_component=$accounting_and_finance_component
export total_compensation='1'
export work_title='Yearly personal tax'
export work_description='Need my taxes done for end of year'
export skills_required='"accounting", "personal tax"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

export work_component=$digital_marketing_component
export total_compensation='1'
export work_title='Socials Marketing'
export work_description='Grow twitter from 195k to 500k'
export skills_required='"twitter", "social engagement"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

export work_component=$graphics_design_component
export total_compensation='1'
export work_title='DEX Logo'
export work_description='Create logo for dex'
export skills_required='"logo design", "creativity"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

export work_component=$disputed_work_component
export total_compensation='1'
export work_title='Example title to dispute'
export work_description='Example description of disputed work'
export skills_required='"logo design", "creativity"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

resim set-default-account $address4 $priv_key4
export account=$address4

export work_component=$development_it_component
export total_compensation='1'
export work_title='Dex'
export work_description='Create a DEX in Scrypto'
export skills_required='"scrypto", "rustlang"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

export work_component=$accounting_and_finance_component
export total_compensation='1'
export work_title='Yearly personal tax'
export work_description='Need my taxes done for end of year'
export skills_required='"accounting", "personal tax"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

export work_component=$digital_marketing_component
export total_compensation='1'
export work_title='Socials Marketing'
export work_description='Grow twitter from 195k to 500k'
export skills_required='"twitter", "social engagement"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

export work_component=$graphics_design_component
export total_compensation='1'
export work_title='DEX Logo'
export work_description='Create logo for dex'
export skills_required='"logo design", "creativity"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

export work_component=$disputed_work_component
export total_compensation='1'
export work_title='Example title to dispute'
export work_description='Example description of disputed work'
export skills_required='"logo design", "creativity"'
for i in {1..5}; do resim run ./transactions/create_new_work.rtm; done

resim set-default-account $address1 $priv_key1
