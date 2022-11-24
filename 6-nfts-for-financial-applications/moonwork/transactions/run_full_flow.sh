#!/bin/bash

# Creates the MoonWork service
source ./transactions/1_create_moon_work_service.sh
# Registers 2 contractors
source ./transactions/2_register_contractors.sh
# Registers 2 clients
source ./transactions/3_register_client.sh
# Creates a bunch of work categories, including one category specifically for handling disputes flow
source ./transactions/4_create_work_categories.sh
# Client creating a bunch of new work
source ./transactions/5_create_new_job.sh
# Contractor requesting to get a lot of work
source ./transactions/6_request_work.sh
# Client accepting contractor for all work
source ./transactions/7_accept_contractor_for_work.sh
# Client & contractor agree to finishing work
source ./transactions/8_finish_work.sh
# Contractor claims all compensation from work thats been completed
source ./transactions/9_claim_compensation.sh
