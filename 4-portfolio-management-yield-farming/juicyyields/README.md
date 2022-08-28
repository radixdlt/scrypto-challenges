![](./images/Juicy_Yields.png)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# Juicy Yields

## Table of Content

  * [About](#about)
    + [Lottery](#lottery)
    + [Staking](#staking)
    + [Betting](#betting)
    + [Lending](#lending)
    + [Arbitrage](#arbitrage)
    + [Juice Token](#juice-token)
  * [Testing](#testing)
    + [Prepare the environment](#prepare-the-environment)   
    + [Create Juice Token](#create-juice-token) 
    + [Create User](#create-user) 
    + [Set User values ](#set-user-values)     
    + [Try all options](#try-all-options)
      + [Buy a lottery ticket](#buy-a-lottery-ticket)
      + [Stake your XRD](#stake-your-xrd) 
      + [Try out betting](#try-out-betting) 
      + [Lend your money ](#lend-your-money) 
      + [Do some arbitrage](#do-some-arbitrage) 
      + [Make any investment](#make-any-investment)  
    + [Collect fee](#collect-fee) 
    + [Send out rewards](#send-out-rewards)  
    + [Run lottery](#run-lottery)  
  * [Open tasks](#open-tasks)
  * [License](#license)


## About
Juicy Yields is a concept for yield management based on a few core principles: staking, lending, betting, arbitrage and lottery.
The user does not have to understand what staking or lending is, but just choose some preferences and the dApp decides the best strategy for him. 
Advanced users can for sure still select one option by themselves to decide how to invest their XRD.

However, the rules, the selection and the functions as such are very basic or even dummies and would need to be developed further.

### Lottery
The user can buy a lottery ticket which is used for the platform internal lottery. He pais 100 XRD per ticket from which we take 5 XRD fee. The complete lottery pot goes to the winner.

### Staking
For staking we check the user preferences and select the best validator for him. The validator data is faked. 2.5 % fee are taken for that staking service via Juicy Yields.

### Betting
Betting is a complete dummy, just to show this might be a good option to offer on the platform.

### Lending
Lending is a complete dummy, just to show this might be a good option to offer on the platform.

### Arbitrage
Arbitrage can be done swapping XRD against a wished token on different dexes. The data is faked. The platform takes 5% fee on the profit.

### Juice Token
Besides the pure rewards through the selected investment strategy, the concept also introduces and uses the JUICE Token, which is used as an extra incentive for the users of Juicy Yields. `$JUICE` can be earned by pure registration to the platform and with each investment that is done via Juicy Yields. 

50% of the fees that the platform takes for the provided services will be distributed back to the JUICE Token holders proportionally.


## Testing
### Prepare the environment
1. Make sure to have any data cleared via
```sh
resim reset
```
2. Then create a new account via
```sh
resim new-account
```
and save the given account address to a variable using 
```sh
export account=<account component address>
```
3. Build and deploy the package on the local ledger via
```sh
resim publish . 
```
and save the address of the created package in 
```sh
export package=<package address>
```
4. Save the XRD resource address 
```sh
export xrd=030000000000000000000000000000000000000000000000000004
```

### Create Juice token
1. Create the initial supply of the Juice token (just do this once!)
```sh
resim run rtm/01_instantiate_juice.rtm
```
2. Save the component addreess
```sh
export juice_comp=<component address>
```
3. Save the admin token of the juice token
```sh
export admin_badge=<first resource address>
```
4. Save the resource address of the juice token
```sh
export juice_token=<second resource address>
```

### Create user
1. Create a new user in the JYields
```sh
resim run rtm/02_create_user.rtm
```
2. Save the component address
```sh
export yields_comp=<component address>
```
3. Save user ID
```sh
export user_id=<resource address>
```

### Set user values 
Create user preferences and set initial deposit
```sh
resim run rtm/03_set_user_values.rtm
```

### Try all options
Juicy Yields provides a few different investment functions:

#### Buy a lottery ticket 
```sh
resim run rtm/04a_buy_lottery_ticket.rtm
```

#### Stake your XRD
```sh
resim run rtm/04b_perform_staking.rtm
```

#### Try out betting 
```sh
resim run rtm/04c_perform_betting.rtm
```

#### Lend your money 
```sh
resim run rtm/04d_perform_lending.rtm
```

#### Do some arbitrage
```sh
resim run rtm/04e_perform_arbitrage.rtm
```

#### Make any investment
Make a proposal and create an investment
```sh
resim run rtm/04f_make_proposal.rtm
```

### Collect fee
Collect the fee and return juice token for that
```sh
resim run rtm/05_get_fee.rtm
```

### Send out rewards
Create user preferences and set initial deposit
```sh
resim run rtm/06_create_reward.rtm
```
NOTE! This is checking for rewards payout every 500 epochs only. So you might want to set the current epoch to another higher value via
```sh
resim set-current-epoch <EPOCH_NUMBER>
```
Then just run it again.

### Run lottery
In case you bought a lottery ticket, run the lottery and check if you won
```sh
resim run rtm/07_run_lottery.rtm
```

## Open Tasks
* *User management*: users need to be all added to one user list. Ensure users are not doubled. Ensure users can change their settings and delete their account etc.
* *Access rules*: are currently just implemented for the main juice vault. This would need to be added to the user vaults in the JYields component as well.
* *JYields blueprint*: some of the methods choosing the investment should actually be moved to another blueprint to be more modular and provide the possibility to exchange that part with another blueprint (with e.g. other fees or other strategies)
* *Cross component calls*: I simply didn't know that cross component calls are possible until two days before deadline and therefore tried to be creative and do every interaction via the worktop in the transaction manifests. So a lot of things need to be changed, e.g. payment of fees from JYields to Juice could be done directly and checking the amount of Juice token from the user would not need to send around the bucket.
* *Frontend*: for sure it needs a nice and easy user frontend (both web and app).
* *Integration*: there is no integration to a dex or other platforms to actually select and perform a real investment.
* *Currencies*: at the moment only XRD is accepted

## License 
This work is licensed under Apache 2.0 and the license file is provided [here](./LICENSE).


