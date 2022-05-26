## NeuRacle prototype test

**End to end** test `. end_to_end.sh` (emit "/" to save and use current environment variables) should run through almost examples and failed-exploit-attempt cases of NeuRacle prototype.

### Initalize test environment:

`. init.sh` get a test environment with 1 admin, 5 validators, 5 users account, also set epoch to a random number in range 500.

`. funding_and_assign.sh` admin create new token, instantiate NeuRacle component, begin funding 10 members account with 1000 NAR each and give validators their badge.

`. stake_and_unstake.sh` go through all stake, unstake, withdraw examples.

`. users_and_apis.sh` set a testing environment of data demands, also test refund account examples.

### Utility

`. data_refresh_round.sh` set an example of a person calling, concluding a data feeding round. (Actually anyone can call or conclude the round, you can change [./transaction_manifest/end_round](./transaction_manifest/end_round) and [./transaction_manifest/start_round](./transaction_manifest/start_round) to try)

`. staked_amount_before_and_after.sh` show the staker account changes after a round. 

`. update_data.sh` set an example of 1 validator inactive, 1 validator have untruthful behavior on total 5 validators. This will also use a prototype of NeuRacle Gateway

`. stable_coin.sh` instantiate a native algorithmed stablecoin project that peg stablecoin USDN to USD on XRD/USD rate. (just use XRD as an example)

`. stable_coin_swap.sh` set an example of the stablecoin project use NeuRacle to swap between NAR and USDN on current XRD/USD coingecko aggregrated rate.

Since it also include the data feeding round source code, you can try `. stable_coin_swap.sh` repeatedly to see how the amount change based on realtime XRD/USD rate on coingecko.

### Other

`. user_get_data_and_etc.sh` set examples of users get their data using badges, a person try to call, conclude a round when it haven't meet requirement, user out of time limit and funding account again.
