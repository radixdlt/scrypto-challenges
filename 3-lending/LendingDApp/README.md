# Lending App

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

LendingApp is a proof-of-concept protocol of an uncollateralized Lending Application built on the Radix ledger using v0.4.0 of Scrypto: the smart contract language of the Radix ledger.

## Table of Content

  * [Abstract](#abstract)
  * [Motivations](#motivations)
  * [Features](#features)
  * [Details of Design](#details-of-design)
    + [Constant Function Market Makers](#constant-function-market-makers)
    + [Liquidity Pools](#liquidity-pools)
    + [Blueprints Overview](#blueprints-overview)
      - [LiquidityPool blueprint](#liquiditypool-blueprint)
      - [RaDEX blueprint](#radex-blueprint)
  * [Examples](#examples)
    + [Getting Started](#getting-started)
    + [Example 1: Providing Liquidity](#example-1-providing-liquidity)
    + [Example 2: Simple Token Swap](#example-2-simple-token-swap)
    + [Example 3: Swapping Through Multiple Pools](#example-3-swapping-through-multiple-pools)
    + [Example 4: Selling and Providing Liquidity](#example-3-selling-and-providing-liquidity)
    + [Example 5: Removing Liquidity](#example-4-removing-liquidity)
    + [Quick Examples](#quick-examples)
  * [Future Work and Improvements](#future-work-and-improvements)
  * [Conclusion](#conclusion)
  * [License](#license)

## Abstract

Lending as a decentralized applications (dApps) is a functionality expected to rise in the near future in layer 1 blockchains, such applications are very demanding because they ask no collateral and they may handle up to millions or billions of dollars worth of tokens in a single day. 
Uncollateralized lendings aim to incentivizes rewards to lenders granting them a great 7% reward on each lend, it also aim to incentivizes borrower without asking for a collateral but instead asking them a 10% fee on each borrowing. At this time this is a proof-of-concept and no epoch are used while calculate this fee/reward. Any lender/borrower is allow to put in place a single operation at any time.

 When lenders start their loan they are given an amount of 'loan tokens' that is equivalent to the amount of xrd tokens given plus the reward (eg. a lend of 100xrd get back to the lender 107lnd), lenders are given the oppurtunity to get back the tokens at anytime, lenders are also assigned loyalty bonus each time they reach a predefined numbers of loans (eg. level1 after 20 loans, level2 after 50 loans).
 On the other side are the debtors that are allowed to ask for borrowing whitout presenting anything. Borrowers are given the amount of xrd tokens they request and have to pay back the same plus a fee (eg. a loan of 100xrd token has to be repaid with 110xrd tokens). Borrowers too are assigned loyalty bonus each time they reach a predefined numbers of repaid loans (eg. level1 after 20 fully repaid loans, level2 after 50 fully repaid loans).
 Levels are assigned to accounts using soulbound-tokens.

 The Lending Engine has some rules designed so that it can remain efficient, solvent and profitable for itself and for the parties involved:
 - No more loans are approved if the main xrd vaults is below 50% of its initial capacity (to prevent from debtors not repaying back their loans)
 - No more loans are accepted if the loan vaults is below 75% of its initial capacity (to prevent from creditors from consuming the main vault)
 - No lending is allowed if it is below 5% or above 20% of the main vault
 - No borrowing is allowed if it is below 1% or above 5% of the main vault

 The Lending Engine rules are fixed and in a subsequent rework of this proof of concept they should become dynamic with respect to the size of the vaults and the number of debtors/creditors.


## Setting up the environment
We will show how you can use the LendingApp blueprint. For this example, we will build a transaction that will first lend some money to the pool, then take the money back. 

## How to run
0. Export xrd resource: `export xrd=030000000000000000000000000000000000000000000000000004` -> save into **$xrd**
1. Reset your environment: `resim reset`
2. Create a new account: `resim new-account` -> save into **$account**
3. Build and deploy the blueprint on the local ledger: `resim publish .` -> save into **$package**
4. Call the `instantiate_pool` function to instantiate a component: `resim call-function $package LendingApp instantiate_pool 100,$xrd 100 10 7` -> save into **$component**, -> into **$lend_nft**, -> into **$borrow_nft**, -> into **$lnd**


## How to run for lenders example
6. Call the `register` method on the component: `resim call-method $component register` to get the `lending nft`
7. Call `resim show $account`
8. Call the `lend_money` method on the component: `resim call-method $component lend_money 10,$xrd 1,$lend_nft`
9. Call the `take_money_back` method on the component: `resim call-method $component take_money_back 10.7,$lnd 1,$lend_nft`
10. Verify that you received a token by running `resim show $account`

## How to run for borrower example
11. Call the `registerBorrower` method on the component: `resim call-method $component registerBorrower` to get the `borrower nft`
12. Call the `borrow_money` method on the component: `resim call-method $component borrow_money 10  1,$borrow_nft`
13. Call the `repay_money` method on the component: `resim call-method $component repay_money 11  1,$borrow_nft`

## How to run for lenders example
1. Run the transaction manifest: `resim run lend.rtm`
2. Run the transaction manifest: `resim run take.rtm`


## Motivations

This Lending App is a proof of concept written in Scrypto where tokens are natively supported as a core feature of the network and its aim is to better understand the asset oriented design pattern.


## Features

In this example, we will create an uncollateralized loan application. Everyone can lend or borrow. 
Level badges are assigned based on usage.

These are the main key features:

* Allows the creation of liquidity pools between two arbitrary tokens.
* Allows liquidity providers to add liquidity to liquidity pools in exchange for tracking tokens.
* Implements a tracking tokens system which tracks the percentage ownership that liquidity providers have over a given liquidity pool.
* Allows for the removal of liquidity using the liquidity pool tracking tokens.
* Allows users to swap their tokens for other tokens.
* Allows for user swaps that include slippage.
  

## Details of Design

<!-- In this section we look at some of the details of the design and mathematics involved that power RaDEX. We first begin by looking at the mathematics behind swaps in Constant Function Market Makers (CFMMs) such as RaDEX, then we move into the -->

### Constant Function Market Makers

To help explain the concept of liquidity pools and how swaps happen, let's begin by giving an example for a person who wants to swap their tokens. Let's say that a guy called Tim wants to swap 10 BTC for XRD and he wishes to use an AMM (example: RaDEX, Uniswap, Sushi Swap, etc...) to perform this swap of tokens. Tim goes to his favorite AMM and performs this swap and get's his XRD back in return. A question now begs itself: **Where did the XRD given to Tim come from?**

The XRD that Tim was given from the swap came from the BTC/XRD liquidity pool of the AMM. Liquidity pools are the core backbone of AMMs. They're typically implemented as smart contracts that hold the reserves of two tokens (BTC and XRD in this case) and allows users to trade these tokens for one another in what is called as bi-directional swaps. The swap that Tim did is actually quite a simple operation: Tim sent his BTC for the pool to deposit it and the pool sent back some XRD to Tim.

If Tim had used some form of a centralized exchange (CEX) to swap his BTC for XRD then the CEX could easily pull the latest exchange rates of these two tokens from the API of any major service if they wanted to (this is not what typically happens in reality, but this is beside the point). Blockchains and ledgers are typically isolated and contained and data from the outside world (from APIs and such) can't easily come into the blockchain. Another question now begs itself: **How much XRD will Tim be given for his swap? How can we even calculate that?**

### Liquidity Pools

Let's briefly touch back on the example that we gave previously of Tim who wanted to exchange 10 BTC for XRD. We mentioned that the swapping of tokens was made possible due to the BTC/XRD liquidity pool of the AMM that Tim was using. The question now is: **well, what exactly are liquidity pools?**

Liquidity pools are typically smart contracts that hold two tokens (BTC and XRD in this example) and allow for these two tokens to be traded bi-directionally for one another. The tokens stored in the liquidity pool (also known as liquidity) come from the liquidity providers who deposit liquidity into the liquidity pool. The question now is, **why do these liquidity providers deposit liquidity into liquidity pools? What incentive or benefit do they have for doing that?**

As has been already mentioned, liquidity is very important for DEXes and could even be thought of as the backbone of all AMM DEXes out there. Therefore, there needs to be an incentive for liquidity providers to deposit liquidity into liquidity pools. One of the things that we did not mention in the previous section is that all swaps in the current implementation of RaDEX have a 0.3% fee of the input token imposed on them. This 0.3% fee is put back into the liquidity pool when a swap happens. Since liquidity providers get ownership of a percentage of the liquidity pool, it means that the 0.3% fee imposed on swaps is divided across the liquidity providers on a swap takes place. Therefore, considering the 0.3% fee the constant market maker function may be redefined as the following:

### Blueprints Overview

![The two core blueprints used in the RaDEX protocol: the RaDEX blueprint and the LiquidityPool blueprint. ](./images/blueprints.svg)

The RaDEX protocol is made up of two core blueprints which are: the `RaDEX` blueprint and the `LiquidityPool` blueprint. Let's begin by discussing the scope of the `LiquidityPool` blueprint before moving into the `RaDEX` blueprint

#### LiquidityPool blueprint

The name of the `LiquidityPool` blueprint kind of gives it away, the `LiquidityPool` blueprint is a blueprint from which LiquidityPool components may be instantiated and created. The liquidity pools defined in this blueprint contain all of the methods needed to add liquidity, remove liquidity, swap tokens, and everything else in between from calculation of the input and outputa amounts and other functionality that the liquidity pool requires.

The key role or functionality of a liquidity pool as of the current implementation of RaDEX is as follows:

* Manages the two vaults that store the two token types that the liquidity pool is created on. These vaults are stored in a hashmap that maps the address to the vault to allow for easier accessing of the vaults.
* Creates and stores a tracking token admin badge which is a badge that has the authority to mint and burn the tracking tokens.
* Contains an `add_liquidity` methods which takes in two buckets of tokens and calculates the appropriate amount of liquidity that may be added based on the current ratio of tokens in the pool. The method then adds the appropriate amount of liquidity to the pool and returns the excess back to the caller.
* During the process of adding liquidity, tracking tokens are minted for the caller of the method and sent back at the end of the method.
* Contains a `remove_liquidity` method which verifies that a given tracking token does indeed belong to the liquidity pool and if it belongs then this method calculates the percentage ownership of the liquidity provider and then withdraws their liquidity and returns it to the caller.
* Contains methods that can be used to calculate the amount of output for a given input and the amount of input required for a given output based on the current state of the pool and the current reserves.
* Contains the swap methods that the users may call with their tokens to swap them for the other tokens. The liquidity pool components also include the needed methods to perform swaps with slippage if they wish to perform that via the smart contract and not a transaction manifest file.
* Has private methods which allow withdraws and deposits to be easily done from within the component itself. These methods may not be called outside of the component itself.
* Contains a number of helper methods for finding the addresses of tokens among other operations useful for the liquidity pool.

In addition to the above mentioned functionalities, the blueprint also contains a number of helper methods which allows for quick withdraws and deposits (these are private methods due to security concerns), methods to get the address of the other tokens, and other helper methods that the liquidity provider methods rely on.

#### RaDEX blueprint

If the `LiquidityPool` blueprint has all of the methods for the mathematics, swaps, withdrawals, deposits, and everything else in between, then what is the `RaDEX` blueprint used for then? The RaDEX blueprint has three core roles in the protocol:

* The RaDEX component acts more as a registry of all of the liquidity pools that belong to the protocol where it keeps a `HashMap` (signature is `HashMap<(Address, Address), LiquidityPool>`) of the address pair of the valid pairs and maps them to the correct liquidity pools. When a user requests the creation of a new liquidity pool, RaDEX checks to ensure that the liquidity pool does not already exist in the HashMap before it is created.
* Through the HashMap of all of the liquidity pools that belong to the protocol, the RaDEX component routes the method calls for adding liquidity, removing liquidity, and performing swaps to the correct liquidity pool so that the operation can be executed.
* The RaDEX component keeps track of the resource addresses of the tracking tokens and the respective address pair that they map to; therefore, when a liquidity provider wants to remove liquidity, the RaDEX component has the information it needs to tell whether the tracking tokens passed are legitimate or not and if they are, which liquidity pool they belong to.

As can be seen from the descriptions above, the LiquidityPool and RaDEX blueprints work hand-in-hand to ensure that RaDEX functions smoothly and predictably. In a typical setting, a `LiquidityPool` component would not be instantiated directly through the `LiquidityPool::new()` function; instead, a new liquidity pool would be created through the `RaDEX.add_liquidity()` or `RaDEX.new_liquidity_pool()` methods so that the liquidity pool can be registered in the RaDEX liquidity pool registry.

## Examples

All of the examples written for RaDEX use the transaction manifest files as well as the new transaction model to showcase Radix's atomically composable transactions and the power that they have.

### Getting Started

In order to ensure that the account and package addresses match on my local machine as well as on yours we need to first reset resim by doing the following:

```sh
$ resim reset
Data directory cleared.
```

The first thing that we need to do now is to create four different accounts to use for the testing of the dex. We can do that by using the following command which creates these four accounts and assigns their addresses and public keys to appropriate environment variables:

```sh
OP1=$(resim new-account)
export PRIV_KEY1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP2=$(resim new-account)
export PRIV_KEY2=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS2=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP3=$(resim new-account)
export PRIV_KEY3=$(echo "$OP3" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY3=$(echo "$OP3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS3=$(echo "$OP3" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP4=$(resim new-account)
export PRIV_KEY4=$(echo "$OP4" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY4=$(echo "$OP4" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS4=$(echo "$OP4" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
```

With the four accounts created, let's give some context as to what we will be doing next. The first thing that we wish to do is to create a number of test tokens that we can use to test out the functionality of the DEX. We would like Account 1 to be the creator of these test tokens and for it to then send some of these tokens to the other accounts so that they can test the DEX. Since Account 1 is the account that will be used for hte creation of the tokens, we need to set it as the default account:

```sh
$ resim set-default-account $ACC_ADDRESS1 $PUB_KEY1 $PRIV_KEY1
Default account updated!
```

The files [`token_creation.rtm`](./transactions/token_creation.rtm), and [`token_funding.rtm`](./transactions/token_funding.rtm) contain the instructions needed for account 1 to create 8 different tokens (we will have a total of 9 tokens after this transaction as we do not need to create XRD) and then fund the 3 other accounts created before depositing all of the remaining tokens back into account 1. To run the transaction file, run the following command:

```sh
resim run transactions/token_creation.rtm && resim run transactions/token_funding.rtm
```

When this transaction runs, all of the accounts that we had created would now have 100,000 of some of the tokens that we will be using for the testing of the DEX. We can now publish the RaDEX package and also instantiate a new RaDEX component by running the following commands:

```sh
PK_OP=$(resim publish ".")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
CP_OP=$(resim run "./transactions/component_creation.rtm")
export COMPONENT=$(echo "$CP_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
```

At this point, we are finally ready to begin testing the functionality of the DEX and to finally see the DEX working.

### Example 1: Providing Liquidity

In this example, we're taking a look at Lynn, the owner of Account 1. Lynn is very passionate about DeFI and about AMMs and she has found that one of the best investments that she can make in the DeFI space is by providing liquidity to AMMs in exchange for percentage ownership over the liquidity pool in addition to the percentage of the fees that Lynn will be taking when swaps are done through a given liquidity pool.

Lynn has just heard of RaDEX, a new and exciting DEX protocol on the Radix ledger and Lynn has decided to be one of the liquidity providers for RaDEX. Lynn wants to either create or provide liquidity of the following amounts in the following pools:

* XRD-USDT: 100,000 XRD and 14,000 USDT.
* QNT-USDT: 865.276 QNT and 100,000 USDT.
* ADA-USDT: 105263.1578 ADA and 100,000 USDT.
* BTC-USDT: 232.558 BTC and 10,000,000 USDT.
* LTC-BTC: 8720.6767 LTC and 23.1835 BTC.
* ADA-XRD: 19526.357 ADA and 67485 XRD.
* LTC-XRD: 88.67 LTC and 67485 XRD.
* LTC-BNB: 88.67 LTC and 24.2187 BNB.
* BNB-DOGE: 2481.57 BNB and 7692307.692 DOGE.

Luckily, Lynn can easily create all of the liquidity pools that she wishes to create using a single transaction manifest file that is atomically composed to provide liquidity to all of these liquidity pools. If one of the transaction instructions fails then the whole thing fails which is an added advantage.

The [`creating_initial_liquidity_pools.rtm`](./transactions/creating_initial_liquidity_pools.rtm) file is a transaction manifest file that contains the transaction instructions that are needed for Lynn to create the liquidity pools that she wishes to create. The following is a high level overview of the instructions included in the file:

1. We clone the account auth badge as many times as we need to perform account withdrawals. We need to do this because bucket references are dropped after method calls. Meaning that the auth badge needs to be cloned to be used for multiple withdrawals.
2. We perform the withdraw of the tokens into the transaction worktop with the cloned badges.
3. Creating buckets of the tokens that will be used to create the liquidity pools and calling the `add_liquidity` method on the RaDEX component to create the liquidity pool.

Now that the process that we will be following is somewhat clear, let's get into running this transaction using Lynn's account. First things first, let's make sure that Lynn's account (Account 1) is set the default account in resim:

```sh
$ resim set-default-account $ACC_ADDRESS1 $PUB_KEY1 $PRIV_KEY1
Default account updated!
```

Let's now run the [`creating_initial_liquidity_pools.rtm`](./transactions/creating_initial_liquidity_pools.rtm) file by running the following command:

```sh
resim run ./transactions/creating_initial_liquidity_pools.rtm
```

Now that this transaction has been executed, Lynn became the first ever liquidity provider in RaDEX! Let's check up on Lynn's account to see the tracking tokens that she has got from creating all of these liquidity pools. The following output is not the full output, it's only the lines which has the tracking tokens:

```sh
$ resim show $ACC_ADDRESS1
Resources:
├─ { amount: 100, resource_def: 032308b2a4f39c5927115792f51bc8f1e43cda373f41c144aff079, name: "USDT-BTC LP Tracking Token", symbol: "TT" }
├─ { amount: 100, resource_def: 03715ac1084d4d685e2223edf5611cc44931d1fcd90cfc7f7e3fbc, name: "LTC-XRD LP Tracking Token", symbol: "TT" }
├─ { amount: 100, resource_def: 0398d4d4f7df503ed07c62f9b4d274e8ad494f01e7aba96fa936bb, name: "LTC-BNB LP Tracking Token", symbol: "TT" }
├─ { amount: 100, resource_def: 0323a4ddb5c144c5f9634ef62fb59815aff5c351b6c5bd33f36710, name: "ADA-XRD LP Tracking Token", symbol: "TT" }
├─ { amount: 100, resource_def: 03de342609dde0abb8726ccd5b3df969b5c230010702d8c8521db9, name: "LTC-BTC LP Tracking Token", symbol: "TT" }
├─ { amount: 100, resource_def: 0366b092bcd71f4e79d6181a1ffa8130a10adb8074d1e9c3ad5429, name: "QNT-USDT LP Tracking Token", symbol: "TT" }
├─ { amount: 100, resource_def: 03ba796915323e06d4c9b608083d6716c9f10aec0275fa33ea8c3d, name: "BNB-DOGE LP Tracking Token", symbol: "TT" }
├─ { amount: 100, resource_def: 0318efcb0a67882180bc193561bc33810dfe716f3efbd03fcb82b8, name: "ADA-USDT LP Tracking Token", symbol: "TT" }
└─ { amount: 100, resource_def: 03e39197c5c3d205d2a0c6ea3b4c5ff262e0b1ffabf7f783755b4b, name: "USDT-XRD LP Tracking Token", symbol: "TT" }
```

As we can see from the resources that are currently in Lynn's account, the liquidity pool creation was successful and Lynn was given liquidity pool tracking tokens in exchange for the liquidity that she has provided to RaDEX. In the current implementation of the RaDEX protocol, the creator of a new liquidity pool is given a hard-coded value of 100 tracking tokens for creating the liquidity pool. Further reassessment will be done in the future as to whether this has any positive on negative implications on the protocol and this value could be changed in future implementations.

### Example 2: Simple Token Swap

Let's now move over from Lynn and talk about Josh, the owner of Account 2. Unfortunately, Josh has found himself in some financial troubles and he wishes to liquidate some of his Bitcoin for cash. Josh needs about \$500,000 USDT and is willing to exchange up to 20 of his Bitcoins for the USDT that he needs. However, Josh does not want to sell more BTC than he needs, he wants to sell the exact amount of BTC needed to get him the $500,000 USDT and thats all.

Josh has decided to use RaDEX to perform the swap of BTC for USDT as RaDEX has the methods and functions needed to swap his BTC for an exact value of USDT. Josh wishes to use Radix's new transaction model to write his own transaction manifest file so that he can independently verify that the exact amount of USDT that he needs was returned back from the swap before he accepts it. The [`swap_BTC_for_USDT.rtm`](./transactions/swap_BTC_for_USDT.rtm) file contains the instructions needed for Josh to perfrom the swap of his BTC for $500,000 USDT tokens. A high level overview of the instructions that are needed to perform this are as follows:

1. Withdraw the 20 Bitcoin from Josh's account and into the transaction worktop.
2. Create a bucket of the withdrawn Bitcoin.
3. Call the `swap_tokens_for_exact_tokens` method the RaDEX component specifying the amount of USDT to get back in return along with the token that Josh wishes to get from the swap (USDT in this case).
4. Assert that transaction worktop now contains the amount of USDT that Josh needs.
5. Deposit everything from the transaction worktop into Josh's account.

Let's now run the needed transaction manifest file for Josh to perform his swap of BTC for USDT. We first need to switch the default account in resim to Josh's account by doing the following:

```sh
$ resim set-default-account $ACC_ADDRESS2 $PUB_KEY2 $PRIV_KEY2
Default account updated!
```

Let's now run the transaction file and perform the Swap of BTC for USDT:

```sh
resim run ./transactions/swap_BTC_for_USDT.rtm
```

Let's take a look at the balances of the relevant tokens in Josh's account after the [`swap_BTC_for_USDT.rtm`](./transactions/swap_BTC_for_USDT.rtm) transaction ran:

```sh
$ resim show $ACC_ADDRESS2
Resources:
├─ { amount: 500000, resource_def: 03b5242185f98446b0c5bf47ce411477ae60fbd7f18b1f423d9b50, name: "Tether", symbol: "USDT" }
└─ { amount: 99987.72327508842316423, resource_def: 031773788de8e4d2947d6592605302d4820ad060ceab06eb2d4711, name: "Bitcoin", symbol: "BTC" }
```

We can see form the balances shown above that about 12.2767 BTC was swapped for $500,000 USDT tokens when the transaction ran. This is the first swap to take place on RaDEX and as we can see, the swapping process was very smooth and seamless.

### Example 3: Swapping Through Multiple Pools

This example truly showcases the power of atomically composable transactions and the power of Radix's new transaction model and how that it's truly a DeFI game changer. The example showcased here is an example that we can already see many people doing when Babylon is released to the mainnet.

Let's take a look at the owner of Account 3, our good friend named Tim. Tim has decided that he wishes to exchange some of his ADA tokens for DOGE tokens; so he decided to take a look at the liquidity pools that are currently available in RaDEX to see if there is an appropriate liquidity pool for this swap. Tim finds the following liquidity pools:

* XRD-USDT liquidity pool.
* QNT-USDT liquidity pool.
* ADA-USDT liquidity pool.
* BTC-USDT liquidity pool.
* LTC-BTC liquidity pool.
* ADA-XRD liquidity pool.
* LTC-XRD liquidity pool.
* LTC-BNB liquidity pool.
* BNB-DOGE liquidity pool.

Well, Tim was unable to find an ADA/DOGE liquidity pool for him to perform his swap. What could Tim possibly do now to perform his swap? Let's think through this step by step and see what could Tim do.

Since there is no liquidity pool for the token pair that Tim wishes to swap, Tim can't just directly swap these tokens for one another. What if, instead of Tim directly swapping ADA for DOGE (which is not possible due to there not being a liquidity pool for it), he swaps his ADA for XRD, then XRD for LTC, then LTC for BNB and then BNB for DOGE? So, Tim would follow the following path of swaps to get from an input of ADA to an output of DOGE:

<!-- ```text
ADA -> ADA/XRD -> XRD/LTC -> LTC/BNB -> BNB/DOGE -> DOGE
``` -->

![](./images/complex_swap_path.svg)

This approach will certainly get Tim from his point A to point B. However, performing swaps across multiple different liquidity pools means that for each swap Tim will have to pay a 0.3% fee and will have to go through the constant market maker function a number of times which could mean that the output of DOGE that Tim gets would be somewhat lower than what he thought it would be. Tim is now faced with two options in terms of how he should perform this swap, these two options are:

1. Tim could just perform regular method calls and perfrom these 4 swaps in 4 transactions.
2. Tim could use a transaction manifest file to perform his ADA -> DOGE swap in a single transaction.

Let's try to reason about the two options that Tim has and see which option could be the best for Tim's case.

As has been mentioned before, going through multiple liquidity pools to perform multiple swaps could result in the amount of output being lower (if not significantly lower) than what Tim thinks it would be. Let's say that Tim chooses option 1 and performs 4 transactions for the 4 swaps that he wishes to make. What kind of guarantee does Tim have that the amount of DOGE that he gets out would be satisfactory to him? There is none. In the first option, if Tim gets an amount of DOGE that he does not like then here is nothing that he could do at this point.

Let's now showcase the power of atomic composability with the second option which is: Tim creating a transaction manifest file for the transaction instructions used to perform the swap across the different liquidity pools. In this case, since all of the swaps will happen in a single transaction, Tim can just add an assertion instruction at the end of the transaction manifest file to ensure that the amount of DOGE that he gets out must exceed some number. If swapping across multiple liquidity pools did indeed have a very bad effect on the amount of output, then the transaction would just be canceled and Tim never loses any of his ADA. If the assertion is successful however, and a satisfactory amount of DOGE is found in transaction worktop after the swaps are performed, then the transaction goes through the DOGE is deposited into Tim's account.

I think that it goes without saying that in this case atomically composing a transaction to perform the swaps and check that the appropriate amount of DOGE is given back is the clear winner. It gives Tim the security of knowing that his tokens won't just be "burned" all along the long swap journey ending in him getting pennies out in return. This is one simple example as to where atomic composability helps the user in their DeFI needs. There are many other examples that could be given to showcase that even further.

Now that we have reasoned about which approach is better to take, we can give a better description so to what Tim is trying to do. Tim wants to swap 100.00 ADA for 300 DOGE or more through the following path of liquidity pools:

```text
ADA -> ADA/XRD -> XRD/LTC -> LTC/BNB -> BNB/DOGE -> DOGE
```

The [`swap_ADA_for_DOGE.rtm`](./transactions/swap_ADA_for_DOGE.rtm) file contains the transaction instructions that will be used to allow Tim to swap his ADA for DOGE. The diagram below shows a high level sequence diagram that is used to explain the process used to swap Tim's ADA for DOGE.

![A sequence diagram of the instructions and the process that will be carried out in order to allow Tim to exchange his ADA for DOGE when a direct liquidity pool doesn’t exist.](./images/complex_swap.svg)

As can be seen in the diagram above, the first step in the transaction is the withdrawal of the 100 ADA from Tim's account and into the transaction worktop. From there, we proceed to perform the swap of tokens through the appropriate liquidity pools. After performing the final swap which is swapping BNB for DOGE we assert that the transaction worktop contains at least 300 DOGE or more. If it does not, then the entire transaction fails and Tim's ADA is safe.

Let's now to get to work and try out this transaction. Let's begin by switching to Tim's account:

```sh
$ resim set-default-account $ACC_ADDRESS3 $PUB_KEY3 $PRIV_KEY3
Default account updated!
```

We're now ready to run the transaction detailed in the transaction manifest file, we can do that by:

```sh
resim run ./transactions/swap_ADA_for_DOGE.rtm
```

We can now take a look at the balances of tokens in Tim's account to see if the transaction went through successfully or not:

```sh
$ resim show $ACC_ADDRESS3
Resources:
├─ { amount: 99900, resource_def: 03de9068895b2f071d39e88c18bcb9f1968499e6948277ef445783, name: "Cardano", symbol: "ADA" }
└─ { amount: 374.142362410166858118, resource_def: 0365598cd30d9363369b5270553e51e1a5898412b8b1c8dedb9856, name: "Dogecoin", symbol: "DOGE" }
```

As we can see from the balances shown above, Tim's balance of ADA decreased by a 100 and for the 100 ADA that he swapped, he was given back 374.1423 DOGE tokens. The long journey of swaps that Tim went on did indeed work and it produced an amount of DOGE that was more than his 300 tokens minimum. Doing everything in a single atomic transaction gave Tim the security of knowing that even if at the end of the long swap journey the rate was bad, that he had the chance to just not accept the rate and retain his ADA.

Optimal path algorithms can be written to run off-ledger to try to find the most optional path that a user can take to perform some kind of swap even if a direct pair exists to attempt to maximize on the output that the user gets.

### Example 4: Selling and Providing Liquidity

Let's switch gears and look at Alfred: the owner of Account 4. He has just heard of yield framing and how that he could make some extra income by providing liquidity to a liquidity pool and earning a percentage of the pool fees that are imposed on swaps.

Alfred has decided that he wants to sell some of the Bitcoin that he owns for USDT and then he wants to provide liquidity to the XRD/USDT liquidity pool in RaDEX. If at some point during the transaction where he provides liquidity something fails for whatever reason, then Alfred no longer wants to go through with providing liquidity to the pool. The [`swap_BTC_for_USDT_and_add_liquidity.rtm`](./transactions/swap_BTC_for_USDT_and_add_liquidity.rtm) contains the instructions that Alfred can use to sell some of his Bitcoin for USDT to later use alongside some of his XRD to provide liquidity to the XRD/USDT pool. The following is a high level overview of the instructions in this transaction manifest file:

1. 500,000 XRD and 40 BTC will be withdrawn from Alfred's account and into the transaction worktop.
2. The 40 BTC will be swapped for USDT on RaDEX.
3. Liquidity will be added to the XRD/USDT liquidity pool using all of the tokens that are currently available in the transaction worktop.
4. The tracking tokens along with the excess amount of the tokens that were not used in adding liquidity to the pool will be deposited into Alfred's account.

Now that we understand what will be done, we can go ahead and perform this transaction. Let's begin by switching over to Alfred's account:

```sh
$ resim set-default-account $ACC_ADDRESS4 $PUB_KEY4 $PRIV_KEY4
Default account updated!
```

We can now run the transaction by running the following command:

```sh
resim run ./transactions/swap_BTC_for_USDT_and_add_liquidity.rtm
```

We can now inspect the balances of Alfred's account to see what has happened now that the swaps are completed.

```sh
$ resim show $ACC_ADDRESS4
Resources:
├─ { amount: 1260665.283004458708113051, resource_def: 03b5242185f98446b0c5bf47ce411477ae60fbd7f18b1f423d9b50, name: "Tether", symbol: "USDT" }
├─ { amount: 99960, resource_def: 031773788de8e4d2947d6592605302d4820ad060ceab06eb2d4711, name: "Bitcoin", symbol: "BTC" }
├─ { amount: 500, resource_def: 03e39197c5c3d205d2a0c6ea3b4c5ff262e0b1ffabf7f783755b4b, name: "USDT-XRD LP Tracking Token", symbol: "TT" }
├─ { amount: 500000, resource_def: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

As we can see from the balances shown above, 20 of Alfred's Bitcoin where sold for an undisclosed amount of USDT. Judging by the amount of XRD that was taken away from Alfred's account, it can be said that the 500,000 XRD provided was fully consumed and that the USDT tokens were in excess. Specifically, there was an excess of 1260665.2830 USDT which was not used when adding the liquidity. Alfred was 500 liquidity provider tracking tokens in exchange for the liquidity that he provided. The current total supply of `03e39197c5c3d205d2a0c6ea3b4c5ff262e0b1ffabf7f783755b4b` is 600 out of which Alfred has 500. This means that alfred owns `500 / 600 = 83.33%` of the USDT-XRD liquidity pool. As long as no other liquidity is added or removed from the pool, then if a swap comes through Alfred would be owed 83.33%`of the 0.3% fee imposed on swaps.

### Example 5: Removing Liquidity

With the last example we are going back to Lynn (Account 1). After providing liquidity for quite some time now, Lynn wants to withdraw her portion of the BTC-USDT liquidity from RaDEX. As always, Lynn wants to use the transaction manifest files to perform this despite it being a very simple operation to perform. The file [`remove_BTC_USDT_liquidity.rtm`](./transactions/remove_BTC_USDT_liquidity.rtm) contains the instructions that Lynn needs to remove liquidity from the BTC/USDT liquidity pool.

Let's begin by switching the default account in resim to be Lynn's account:

```sh
$ resim set-default-account $ACC_ADDRESS1 $PUB_KEY1 $PRIV_KEY1
Default account updated!
```

We can now run the transaction by running the following command:

```sh
resim run ./transactions/remove_BTC_USDT_liquidity.rtm
```

Let's now view the balances of Lynn's account:

```
$ resim show $ACC_ADDRESS1
Resources:
├─ { amount: 765030, resource_def: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
├─ { amount: 0, resource_def: 032308b2a4f39c5927115792f51bc8f1e43cda373f41c144aff079, name: "USDT-BTC LP Tracking Token", symbol: "TT" }
├─ { amount: 18667060.09322491157683577, resource_def: 031773788de8e4d2947d6592605302d4820ad060ceab06eb2d4711, name: "Bitcoin", symbol: "BTC" }
```

As we can see from the balances above, Lynn no longer has any USDT-BTC liquidity provider tracking tokens as she has removed all of the liquidity that she was owed from the BTC-USDT liquidity pool. Instead of the liquidity provider tokens, Lynn was given back her portion of BTC and USDT.

### Quick Examples

All of the commands discussed in this readme file are available in the `script.sh` for readers who wish to run all of the commands in part or in full from a script file. 

## Future Work and Improvements

There are many things that could be improved about the current implementation of RaDEX. Some of the key points which require improvement are:

* Researching methods to ensure that the precision of the calculations and math done by the liquidity pool components is as accurate and precise as it can be.
* Writing additional examples as well as tests for the DEX.
* Including a price oracle into the implementation of the DEX.
* Additional interface methods are needed in the RaDEX blueprint for RaDEX components.

## Conclusion

This work implements RaDEX, An AMM DEX on the Radix ledger built with v0.3.0 of Scrypto. RaDEX aims to be a complete reimagination of the Uniswap V2 protocol that is implemented on a modern ledger that allows for quick, secure, and seamless atomically composable transactions to take place. 

## License 

This work is licensed under Apache 2.0 and the license file is provided [here](./LICENSE).