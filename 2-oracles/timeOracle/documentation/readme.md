# Time Oracle

Allows user to get an UNIX timestring on ledger.

### How to test

# Scrypto

Publish package to the ledger
Instantiate component

# Frontend

(A frontend can be found here: https://github.com/MiroLiebschner/babylon-pte/pulls For some reasons some methods are buggy :/)
Minimal viable frontend:
button "Update Time"
a) API request: "http://worldtimeapi.org/api/timezone/Europe" (includes UNIX string)
b) callback "update_time" with the UNIX String as argument

### Process user

a) callback "pay_for_update_time" to increase the amount of paid_for_requests
b) press button described in frontend to trigger API request & update_time (paid_for_requests > 0)
c) callback "get_time" to get the newly refreshed time
