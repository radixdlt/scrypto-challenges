#!/usr/bin/env sh
#set -x
set -e

# Reset the simulator
resim reset ; echo

# Create an account for the admin/operator of the CallbackScheduler component
result=$(resim new-account)
printf "$result\n\n"
export admin_private_key=$(echo "$result" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export admin_account=$(echo "$result" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

# Publish the package
result=$(resim publish .)
printf "$result\n\n"
export package=$(echo "$result" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

# Instantiate a CallbackScheduler component with a fee of 10 XRD per callback scheduling
result=$(resim call-function $package CallbackScheduler instantiate_callback_scheduler 10)
export scheduler_component=$(echo "$result" | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")
export scheduler_component_admin_badge=$(echo "$result" | sed -nr "s/.*admin_badge=([[:alnum:]_]+)/\1/p")
export callback_admin_handle_resource=$(echo "$result" | sed -nr "s/.*callback_admin_handle_resource=([[:alnum:]_]+)/\1/p")
export callback_handle_resource=$(echo "$result" | sed -nr "s/.*callback_handle_resource=([[:alnum:]_]+)/\1/p")
printf "$result\n\n"

# Create another account for a user of the CallbackScheduler component
result=$(resim new-account)
printf "$result\n\n"
export user_private_key=$(echo "$result" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export user_account=$(echo "$result" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

# Take on the role of the user
resim set-default-account $user_account $user_private_key ; echo

# Instantiate a TestComponent that demonstrates the usage of the CallbackScheduler component
# The only argument to the instantiate_test_component method is the address of the CallbackScheduler component
result=$(resim call-function $package TestComponent instantiate_test_component $scheduler_component)
printf "$result\n\n"
export test_component=$(echo "$result" | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")
export test_component_admin_badge=$(echo "$result" | sed -nr "s/.*admin_badge=([[:alnum:]_]+)/\1/p")

# Schedule 5 example callbacks
result=$(resim run transactions/test/user/schedule_example_callbacks.rtm)
printf "$result\n\n"
export callback_id_1=$(echo "$result" | sed -nr "s/.*CallbackAdminHandle: #([[:alnum:]_]+).*/\1/p" | sed '1!d')
export callback_id_2=$(echo "$result" | sed -nr "s/.*CallbackAdminHandle: #([[:alnum:]_]+).*/\1/p" | sed '2!d')
export callback_id_3=$(echo "$result" | sed -nr "s/.*CallbackAdminHandle: #([[:alnum:]_]+).*/\1/p" | sed '3!d')
export callback_id_4=$(echo "$result" | sed -nr "s/.*CallbackAdminHandle: #([[:alnum:]_]+).*/\1/p" | sed '4!d')
export callback_id_5=$(echo "$result" | sed -nr "s/.*CallbackAdminHandle: #([[:alnum:]_]+).*/\1/p" | sed '5!d')

# Immediately cancel callback #5
# At the moment, fees are not reimbursed.
resim run transactions/test/user/cancel_callback_#5.rtm ; echo

# Take on the role of the CallbackScheduler admin/operator
resim set-default-account $admin_account $admin_private_key ; echo

# Retrieve the admin handles of all newly scheduled callbacks
resim run transactions/test/admin/get_new_callback_admin_handles.rtm ; echo

# Take a look at the admin account and observe that it contains the CallbackAdminHandles for the first 4 callbacks.
# The 5th callback was canceled by the user before we could retrieve it from the scheduler component.
resim show $admin_account ; echo

# Try to execute the first callback
# Observe that it fails as the SchedulerComponent prevents us from executing the callback in the wrong epoch
resim run transactions/test/admin/execute_callback_#1.rtm || : ; echo

# Advance the epoch and try again
resim set-current-epoch 1
resim run transactions/test/admin/execute_callback_#1.rtm ; echo

# Also execute callback #2 and #3
resim run transactions/test/admin/execute_callback_#2.rtm ; echo
resim run transactions/test/admin/execute_callback_#3.rtm ; echo

# Switch to the user account and cancel the 4th callback
# The admin/operator will still be in posession of their CallbackAdminHandle
# but will no longer be able to execute the callback.
resim set-default-account $user_account $user_private_key ; echo
resim run transactions/test/user/cancel_callback_#4.rtm ; echo

# Switch back to the admin/operator account and try to execute callback #4
# This will fail as the callback has just been canceled by the user
resim set-default-account $admin_account $admin_private_key ; echo
resim run transactions/test/admin/execute_callback_#4.rtm || : ; echo

# Also cancel the callback from the admin/operator side
resim run transactions/test/admin/cancel_callback_#4.rtm ; echo

# Finally withdraw the earned fees and show the admin account.
# Observe that all callback handles are now gone and the XRD
# balance has increased by 50.
resim run transactions/test/admin/withdraw_fees.rtm ; echo
resim show $admin_account

printf "\n\n\n\t\033[0;32mTest successful\033[0m\n\n\n"
