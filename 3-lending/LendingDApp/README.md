# Lending App

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

LendingApp is a proof-of-concept protocol of an uncollateralized Lending Application built on the Radix ledger using v0.4.0 of Scrypto: the smart contract language of the Radix ledger.


### Example2: Borrow tokens and repay 


### Example3: Multiple operation with different accounts 


### Example4: Lending App rules overview

## Table of Content

  * [Abstract](#abstract)
  * [Motivations](#motivations)
  * [Features](#features)
  * [Details of Design](#details-of-design)
    + [Lending Engine](#lending-engine)
    + [LendingApp blueprint](#lendingapp-blueprint)
    + [Blueprints Overview](#blueprints-overview)
  * [Examples](#examples)
    + [Getting Started](#getting-started)
    + [Example 1: Lending tokens and getting back](#example-1-lending-tokens-and-getting-back)
    + [Example 2: Borrow tokens and repay](#example-2-borrow-tokens-and-repay)
    + [Example 3: Multiple operation with different accounts](#example-3-multiple-operations-with-different-accounts)
    + [Example 4: Lending App rules overview](#example-3-lending-app-rules-overview)
    + [Quick Examples](#quick-examples)
  * [Future Work and Improvements](#future-work-and-improvements)

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

## Motivations

This Lending App is a proof of concept written in Scrypto where tokens are natively supported as a core feature of the network and its aim is to better understand the asset oriented design pattern.

## Features

In this example, we will create an uncollateralized loan application. Everyone can lend or borrow. 
Level badges are assigned based on usage.

These are the main key features:

* Accept lendings from lenders.
* Allow lenders to get back the loan along with a reward.
* Implements rules for preventing from debtors not repaying back their loans and from creditors from consuming the main vault.
* Approve borrowings to borrowers.
* Allow borrowers to repay loans.
* Forbid to anyone to have multiple loans at the same time (but an account could lend and then borrow some xrd tokens).

## Details of Design

To help explain the concept of the lending app, let's begin by giving an example for a person who wants to lend their tokens. 
Let's say that a guy called Leo wants to lend 100 XRD. Leo goes to the Lending App and lends this bucket of XRD getting back a bucket of LND token plus a soulbound token. Two question arises: **Where will go this XRD given from Tim ? Where does the LND token comes from and are to be used for ?**

The XRD that Leo gives to the Lending App goes in the main pool of XRD tokens where creditors will draw their uncollateralized loan. 
The LND that Leo receives comes from the Loan pool and are to used to claim back the XRD tokens, LND tokens are not transferable and are inclusive of the reward.

Let's continue with an example for a person who wants instead to borrow some tokens. 
Let's say that a guy called Lory wants to borrow 100 XRD. Lory goes to the Lending App and ask for a bucket of XRD, if the engine rules are met the loan gets approved and Lory gets back a bucket of XRD token plus a soulbound token. Two question arises: **Where this XRD tokes comes from ? How could Lory repay the full amount ?**

The XRD that Lory gets from the Lending App comes from the main pool of XRD tokens that grows thanks to the difference between rewards and fees. 
The soulbound token that Lory receives is not transferable and contains the amount to be repaid back, credit level can be awarded only when the full loan is repaid and could take to better fees.

### Lending Engine

Let's briefly touch back on the rules of the engine.
There needs to be an incentive for all the actors:
- lenders
- borrowers
- the app itself

The engine gets reward from the difference payed by borrowers to what it has to pay to lenders (eg. 10% - 7% result in a 3%).
The net result is put back into the main pool.

The incentive may help encourage the actors to stay honest. The lenders should find it profitable, the borrowers should find it convenient because of its uncollateralized nature, it should be more profitable for everyone to play by the rules than to undermine the system.

#### LendingApp blueprint

`LendingApp` is a blueprint from which components may be instantiated and created. The pools defined in this blueprint contain all of the methods needed to accept and approve loan, to pay back and repay and other functionality required.

The key role or functionality is as follows:

* Manages the two vaults that store the two token types, the XRD vault (main pool) and the loan vault (loan pool).
* Creates and stores an admin badge that has the authority to mint and burn the loan tokens.

* Contains an `lend_money` methods which takes in a bucket of XRD tokens and the LendingNFT, then check if the lend is acceptable (the bucket size has to be between 5% and 20% of the main vault size) and if the ratio lenders/borrowers is appropriate (loan pool vault size needs over 75%). The method then gives back a bucket of LND tokens with a reward, that bucket could be used later to claim back the original XRD tokens. Finally the LendingNFT gets updated adding to the counter, updating the level and setting that a lend is running to avoid concurrent operation from the same account.
* Contains an `take_money` method that takes the LND bucket and the LendingNFT. The LND tokens are divided in two parts, the fee gets burned and the original amount goes back in the loan pool, then the XRD tokens to be sent to the account are taken from the main pool and sent back to the lender. Finally the LendingNFT gets updated setting that a lend is not anymore running.
* Contains an `borrow_money` 
* Contains an `repay_money` 

In addition to the above mentioned functionalities, the blueprint also contains a number of helper methods.


## Examples

All of the examples written for LendingApp use the transaction manifest files as well as the new transaction model.

### Getting Started

In order to ensure that the account and package addresses match on my local machine as well as on yours we need to first reset resim by doing the following:

```sh
$ resim reset
Data directory cleared.
```

The first thing that we need to do now is to create four different accounts to use for the testing. We can do that by using the following command which creates these four accounts and assigns their addresses and public keys to appropriate environment variables:

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

With the four accounts created, let's give some context as to what we will be doing next. 
The first thing that we wish to do is to lend and take back some token to the dApp, then we wish to borrow and repay with the same account, later we wish to interact with the dApp with all the accounts at the same time but every time in a different role.

```sh
$ resim set-default-account $ACC_ADDRESS1 $PUB_KEY1 $PRIV_KEY1
Default account updated!
```
We have to publish and create the package that we will use for creating the component

```sh 
$ resim publish . 
    Finished release [optimized] target(s) in 0.58s
    Success! New Package: 012899b03a2aa1fdcf1df883c820934ba9b089f88a1850249636fa
$ export package=012899b03a2aa1fdcf1df883c820934ba9b089f88a1850249636fa
```
Then we can produce the component and the resources needed:

```sh 
$ resim call-function $package LendingApp instantiate_pool 1000,$xrd 1000 10 7
Logs: 5
├─ [INFO ] My start amount is: 1000
├─ [INFO ] My fee for borrower is: 10
├─ [INFO ] My reward for lenders is: 7
├─ [INFO ] Loan pool size is: 1000
└─ [INFO ] Main pool size is: 1000
New Entities: 5
└─ Component: 0296a6bed1c65f7d3e29f9f880f6ca17a8402ff180ced054050f2b
├─ Resource: 0364529170a265ba94bd4c6b932a04e6406bb4797e655cd371f60d
├─ Resource: 03101176511a0f44db8f5d33d10985c163aaf642ab779bee697dc4
├─ Resource: 030ce6b265aeab99a94aa57bf3b6086255dd4a3e0b2f262229848f
└─ Resource: 03c20299928dc86cb3f8571b8843e4e10e38931f977b5ac47e4610
$ export component=03101176511a0f44db8f5d33d10985c163aaf642ab779bee697dc4
$ export lend_nft=03101176511a0f44db8f5d33d10985c163aaf642ab779bee697dc4
$ export borrow_nft=030ce6b265aeab99a94aa57bf3b6086255dd4a3e0b2f262229848f
$ export lnd=03c20299928dc86cb3f8571b8843e4e10e38931f977b5ac47e4610
```
### Example 1: Lending tokens and getting back 

Now we can register the account and then finally start using the Dapp.
The 'register' method gives the account non-fungible token that contains:
    - the number_of_lendings the account has successfully completed
    - if the l1 level has been reached
    - if the l2 level has been reached
    - if an operation is in progress

```sh 
$ resim call-method $component register
Resources:
├─ { amount: 999000, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
└─ { amount: 1, resource address: 03101176511a0f44db8f5d33d10985c163aaf642ab779bee697dc4, name: "Lending NFTs" }
   └─ NonFungible { id: b5d7571f8ddfd21b3db40abf1ba34479dcaac58e1a9e1e72b48a2091aef90903, immutable_data: Struct(), mutable_data: Struct(0u16, false, false, false) }
```

Then we can execute the first lending operation from account, this operation moves 80XRD to the LendingApp and the receives LND tokens (the same amount plus the reward).
Herein you can check the vault of both the account and the component.

Transaction Manifest is here [`lend1.rtm`](/transactions/lend1.rtm) 

```sh 
$ resim run transactions/lend1.rtm
$ resim show $ACC_ADDRESS1
[CUT]
Resources:
├─ { amount: **85.6**, resource address: 03d3f4664be91fe88c6155f025090dfe7b25b9a372ddebd1c3c38e, name: "Loan token", symbol: "LND" }
├─ { amount: **998920**, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
└─ { amount: 1, resource address: 03b6ccc92abe889a4a9b8003f7b6a867d620ca7fc95c7d1902aea2, name: "Lending NFTs" }
   └─ NonFungible { id: 0b2ec0c0a413879b9148a76fc5a425dd8e7c1f8bc471b478a24020930509fefc, immutable_data: Struct(), mutable_data: Struct(1u16, false, false, **true**) }

$ ./comp.sh 
Component: 02078c8d2c66a18db7c6363afd2d8ab873756204818e5d68e993a4
Blueprint: { package_address: 0145fddf3be40fce31be8820da86bacfe2175054e006966ae9fd3c, blueprint_name: "LendingApp" }
[CUT]
Resources:
├─ { amount: 1, resource address: 0356acb2964de8fd035a1655bf23d0a50f27302190d1d03be43cd2, name: "Loan Token Auth" }
├─ { amount: **914.4**, resource address: 03d3f4664be91fe88c6155f025090dfe7b25b9a372ddebd1c3c38e, name: "Loan token", symbol: "LND" }
└─ { amount: **1080**, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
```

Later on the account holder ask to get its XRD tokens back from the Lending App.
As before, you can check the vault of both the account and the component.

Transaction Manifest is here [`take1.rtm`](/transactions/take1.rtm) 

```sh 
$ resim run transactions/take1.rtm
Resources:
├─ { amount: 0, resource address: 03d3f4664be91fe88c6155f025090dfe7b25b9a372ddebd1c3c38e, name: "Loan token", symbol: "LND" }
├─ { amount: **999005.6**, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
└─ { amount: 1, resource address: 03b6ccc92abe889a4a9b8003f7b6a867d620ca7fc95c7d1902aea2, name: "Lending NFTs" }
   └─ NonFungible { id: 0b2ec0c0a413879b9148a76fc5a425dd8e7c1f8bc471b478a24020930509fefc, immutable_data: Struct(), mutable_data: Struct(1u16, false, false, **false**) }

$ ./comp.sh 
Component: 02078c8d2c66a18db7c6363afd2d8ab873756204818e5d68e993a4
Blueprint: { package_address: 0145fddf3be40fce31be8820da86bacfe2175054e006966ae9fd3c, blueprint_name: "LendingApp" }
[CUT]
Resources:
├─ { amount: **994.4**, resource address: 03d3f4664be91fe88c6155f025090dfe7b25b9a372ddebd1c3c38e, name: "Loan token", symbol: "LND" }
├─ { amount: **994.4**, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
└─ { amount: 1, resource address: 0356acb2964de8fd035a1655bf23d0a50f27302190d1d03be43cd2, name: "Loan Token Auth" }
```

We can see that the reward has gone from the main vault of the Lending App to the one of the account holder.


### Example2: Borrow tokens and repay 


### Example3: Multiple operation with different accounts 


### Example4: Lending App rules overview

Here a simple schema at a given time showing the actors, their levels and the operations completed/in progress.

![](images/firstSchema.png.svg)



### Quick Examples

All of the commands and transactions discussed in this readme file can also be verified directly using the resim tool that you can use with the LendingApp blueprint. 
For this example, we will build a transaction that will first lend some tokens to the pool, then take the tokens back. We will also build a reverse of it, meaning that we will first borrow some tokens to the pool, then take the tokens back

## How to run
0. Export xrd resource: `export xrd=030000000000000000000000000000000000000000000000000004` -> save into **$xrd**
1. Reset your environment: `resim reset`
2. Create a new account: `resim new-account` -> save into **$account**
3. Build and deploy the blueprint on the local ledger: `resim publish .` -> save into **$package**
4. Call the `instantiate_pool` function to instantiate a component: `resim call-function $package LendingApp instantiate_pool 1000,$xrd 1000 10 7` -> save component[0] into **$component**, resources[1] -> into **$lend_nft**, resources[2]  -> into **$borrow_nft**, resources[3]  -> into **$lnd**

## How to run for lenders example
6. Call the `register` method on the component: `resim call-method $component register` to get the `lending nft`
7. Call `resim show $account` to look at the received nft
8. Call the `lend_money` method on the component: `resim call-method $component lend_money 100,$xrd 1,$lend_nft`
9. Call the `take_money_back` method on the component: `resim call-method $component take_money_back 107,$lnd 1,$lend_nft`
10. Verify that you received a reward for the lending `resim show $account`

## How to run for borrower example
11. Call the `registerBorrower` method on the component: `resim call-method $component registerBorrower` to get the `borrower nft`
12. Call `resim show $account` to look at the received nft
13. Call the `borrow_money` method on the component: `resim call-method $component borrow_money 100  1,$borrow_nft`
14. Call the `repay_money` method on the component: `resim call-method $component repay_money 110  1,$borrow_nft`
15. Verify that you paied a fee for the borrowing `resim show $account`

## How to run for lenders example
1. Run the transaction manifest: `resim run transactions/lend.rtm`
2. Run the transaction manifest: `resim run transactions/take.rtm`
## How to run for borrowers example
1. Run the transaction manifest: `resim run transactions/borrow.rtm`
2. Run the transaction manifest: `resim run transactions/repay.rtm`


## Future Work and Improvements

There is a lot that could be improved about the current implementation of LendingApp. Some of the key points which require improvement are:

* Researching methods to ensure that the precision of the calculations and math done by the engine to calculate fees andrewards is as accurate and precise as it can be.
* Writing additional examples as well as tests.
* Adding the concept of time passing to the calculation of fee and rewards.
* Additional analisys on the business model to make sure it will be successful.


## License 

This work is licensed under Apache 2.0 and the license file is provided [here](./LICENSE).







