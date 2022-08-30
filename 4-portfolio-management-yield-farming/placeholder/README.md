![](./public/images/IMG_1112.jpg)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Table of Content

  * [Abstract](#abstract)
  * [Motivations](#motivations)
  * [Challenges](#challenges)
  * [Protocol Admin Features](#protocol-admin-features)
  * [Farmers Features ](#farmers-features)
  * [Index Fund Features](#index-fund-features)
  * [Debt Fund Features](#debt-fund-features)
  * [Borrower Features](#borrower-fatures)
  * [Investor Features](#investor-features)
  * [Design Details](#design-details)
    + [Blueprints Overview](#blueprints-overview)
    + [Dashboard Blueprints](#dashboard-blueprints)
    + [IndexFund Blueprints](#indexfund-blueprint)
    + [DebtFund Blueprint](#debtfund-blueprint)
    + [FundingLocker Blueprint](#fundinglocker-blueprint)
  * [Examples](#examples)
    + [Getting Started](#getting-started)
    + [Example 1: Creating pools & depositing supply](#example-1-creating-pools-&-depositing-supply)
    + [Example 2: Leverage 1x Long Strategy](#example-2-leverage-1x-long-strategy)
    + [Example 3: Leverage 2x Short Strategy](#example-3-leverage-2x-short-strategy)
    + [Example 4: Leverage 3x Long Strategy](#example-4-leverage-3x-long-strategy)
    + [Example 5: Flash Liquidation](#example-5-flash-liquidation)
    + [Example 6: Closing out a leveraged position](#example-6-closing-out-a-leveraged-position)
    + [Example 7: Loan Auctioning](#example-7-loan-auctioning)
  * [Future Work and Improvements](#future-work-and-improvements)
  * [Conclusion](#conclusion)
  * [License](#license)


## Abstract
Farmers Market Protocol is a permissioned DeFi protocol that allows insitutional Farmerss to bring capital markets on-chain. Farmers Market allows Farmers, or otherwise known as Farmerss or portfolio managers with a suite of tools to start a fund to manage other people's money on-chain. Farmers can start a fund focusing on equity investments by creating an index fund with a basket of assets or they can start a fund focusing on debt investments by creating a debt fund. The index fund provides Farmers with the ability to create a diversified portfolio, rebalance their protfolio using [RaDEX](https://github.com/radixdlt/scrypto-challenges/tree/main/1-exchanges/RaDEX), and leverage their portoflio using [DegenFi](https://github.com/PostNutCIarity/degenfi). While the debt fund allows Farmers to raise capital on-chain and originate loans with institutional credit borrowers.Borrowers and investors alike can access professional Farmerss to assist them with their capital needs.

## Motivations

This project is inspired by both [Maple Finance](https://www.maple.finance/) and [TokenSets](https://www.tokensets.com/). These protocol had interesting designs to allow portfolio managers with a suite of tools to manage capital on-chain. In an effort to continue looking for a challenge to learn and apply what I've learned in Scrypto, I decided to explore how these protocols would potentially look like if it was re-imagined in an asset-oriented approach. This challenge brought lots of considerations I had to ponder when it came to access controls as the protocol is meant to be permissioned.  

## Challenges

The main challenges I came across with building this protocol were:

* Blueprint design considerations
* Authorization model considerations
* Calculations

I spent a lot of time considering how the Blueprints should be layed out. There are 4 different types of users that will be using this protocol:

1. Protocol admins
2. Farmers (Farmerss/portfolio managers)
3. Borrowers
4. Investors

Since this protocol is permissioned, I needed to lay out the Blueprints in a way where all of these parties can be provided with a suite of features while ensuring that there are proper access controls that ensures safety between the interactions of all the users involved. As such, this also meant that there were a lot of authorization model designs I had to consider. The authorization model when this project first began looked very different from how it ended. Yet, frankly, I am still unsure whether this would be the best design for this form of protocol. After reading about Scrypto 0.5 in the midst of this challenge, I'm sure some of the complexities in this design would have simplified quite drastically and elegantly. 

Additionally, there were a lot of different forms calculations to consider, i.e how the conversion between fund tokens to the underlying asset of the index fund worked and the interest calculations on the debt fund side. Spending a lot of time pondering how these things worked was a personal challenge of mine. I buttoned up the calculations as best as I can, but you may likely find inaccuracies. 

Due to my previous experience of building DeFi protocol from the last challenge, it had made building this protocol much easier. Although, I also didn't realize how ambitious the project I wished to build for this challenge was. There were a lot of different moving parts to keep track of and consider. As such, there are probably a lot things janky in this protocol as I didn't have enough time to test out this prototype to the capacity I feel satisfied with. Nevertheless, I still wanted to submit this as I'm sure as part of providing more Scrypto examples to the community, people may find this helpful. Notwithstanding, this challenge was a big learning opportunity.  

## Protocol Admin Features:

* **Permissioned user creation** - Because this protocol is permissioned and largely meant for institution. Protocol admins are able to approve prospective instutional Farmerss
and borrowers to the protocol.
  * This process is facilitated by allowing prospective institutional Farmerss and borrowers to create a temporary badge of their user type ("Farmers" or "Borrower").
  * After off-chain due-dillegence, the protocol administrator would then approve each respective user type by calling either `create_fund_manager` or `create_borrower` to which a Farmers badge or a Borrower badge will be deposited to their respective component vault. Additionally, a `FundManagerDashboard` or `BorrowerDashboard` will be instantiated for each respective party to use once they claim their associated badges.
  * The Farmers or Borrower may claim their respective badges created by the protocol administrator by calling `claim_badge` where they will deposit their Temporary Badge (which will be burnt) and retrieve their respective badges.
  * There are logic involved to ensure that this process is orderly, which you may view by heading over to the [./src/vault_protocol.rs](./src/vault_protocol.rs).

So far, this is the only thing the protocol admins can do. There currently isn't any features to allow protocol admins to reject or revoke badges.

## Farmers Features:

Once the Farmers has retrieved their respective badge. They are provided with these features:

* **Create Index Fund** - Allows the Farmers to create an Index Fund, which allows them to create a basket of tokens their fund will manage.
* **Create Debt Fund** - Allows the Farmers to create a Debt Fund, which is essentially a lending pool which they can manage to faciliate loan originations to institutional borrowers.

### Index Fund Features:

The Index Fund is what allows Farmerss to create a fund, rebalance, and manage a basket of assets and allow for flexibilities in the strategies they pursue.

These are the sets of features Farmerss can access to manage their Index Fund:

* **Integrate DEX** - Currently, only supports [RaDEX](https://github.com/radixdlt/scrypto-challenges/tree/main/1-exchanges/RaDEX). This will allow Farmerss to swap tokens (in order to rebalance their portfolio) and participate in Liquidity Provider incentives by supplying liquidity to already established liquidity pool. 
* **Integrate Lending** - Currently, only supports [DegenFi](https://github.com/PostNutCIarity/degenfi). This will allow Farmerss to exercise leveraged fund strategies and participate in DegenFi's protocol features.
* **Issue fund tokens** - With each Index Fund will come with its own tokens that represents the share of ownership of the fund to be sold to the secondary market. Farmerss and Investors can issue fund tokens by directly providing the underlying assets that the fund supports.
* **Redeem fund tokens** - When the Investor wishes to exit out of the fund, they may redeem their share of the fund's underlying asset by exchanging their fund tokens.
* **Buy fund tokens** - When the Farmers has created a pool on RaDEX that supports the exchange of their respective fund tokens, Investors may now be able to purchase the fund tokens in the secondary market.
* **Sell fund tokens** - Likewise, Investors can also sell their fund tokens in the secondary market through RaDEX.

### Debt Fund Features:

The Debt Fund is what allows Farmerss to create a fund where they can raise capital and originate loans for institutional Borrowers in a permissioned and orderly way.

Before going over the Debt Fund features, it may be helpful to provide context in how loan originations work. Borrowers in this protocol must first be credit-worthy institutional borrowers. The protocol admin does the first phase of due dilligence to ensure that the Borrower is credit worthy by performing off-chain due dilligence. Once the Borrower is accepted into the protocol, Borrowers may request a loan through the `BorrowerDashboard`. 

When the loan request is submitted, Farmerss may view loan requests that have been submitted. They may accept the loan request as is or negotiate with the Borrower off-chain. Underwriting and due-dilligence will be performed by the Farmers off-chain, and once an agreement has been set, Farmerss may instantiate a `FundingLocker` where the loan will be managed. 

These are the set of features Farmerss can access to manage their Debt Fund:

* **Supply Liquidity** - Allows Farmerss or Investors to supply liquidity to the fund to claim fees. 
* **Remove Liquidity** - Likewise, Farmerss or Investors can remove liquidity from the fund. Note that there are probably issues of removing liquidity when there is a loan active.
* **Instantiate Funding Locker** - The Debt Fund can originate loans by instantiating a funding locker. A Loan NFT is provided and contained in the `FundingLocker` component until the Borrower has deposited enough collateral for the Borrower to retrieve it. The Loan NFT is used to access the `FundingLocker` from the Borrower's side.
* **Fund Loan** - Once the Borrower has met their collateralization requirement, the Loan NFT is set to `ReadyToFund` to which the Farmers may begin funding the loan through their Debt Fund.
* **Approve Draw Request** - The Borrower must first provide a draw request for the Farmers to approve. The formal documentations for a draw request will be submitted to the Farmers off-chain. Once satisfied, the Farmers can begin approving the draw request.
* **Reject Draw Request** - The Farmers may reserve the right to reject the Borrower's draw request.
* **Update loan** - The Farmers can update the loan with the interest expense accrued for the month.
* **Transfer Fees** - For user experience purposes, since the Investor primarily only has permissioned access to the Debt Fund, it seems ideal for the Farmerss to transfer fees from their loan originations through the `FundingLocker` to the `DebtFund` component where Borrowers may then be able to directly claim the fees through the `DebtFund` component.
* **Claim Fees** - Allows Investors to claim fees based on their ownserhsip of the fund.
* **Transfer Liquidity** - Farmerss can transfer the repaid loan proceeds from their loan originations through the `FundingLocker` to the `DebtFund`.

## Borrower Fatures:

Borrowers are approved institutions that may wish to borrow undercollateralized loans on this protocol.

Once the Borrower has retrieved their respective badge. They are provided with these features:

* **Request New Loan** - Borrowers may request a new loan to be sent out to Farmers(s) in the protocol. 
* **Deposit Collateral** - Once a Borrower's loan request is approved, they may deposit collateral to meet their collateralization requirement before the loan is funded. Once met, Borrower's receive a Loan NFT where they can access the `FundingLocker` through their `BorrowerDashboard`.
* **Request Draw** - Borrower may request to draw from the loan from the Farmers. 
* **Receive Draw** - Once a draw request has been approved, the Borrower may receive the draw amount.
* **Make Payment** - The Borrower can make interest and principal payment through the `make_payment` method call.

## Investor Features:

The Investor user type in this protocol does not need to have a badge to access the protocol. There is an `InvestorDashboard` through which they can access features provided to them.

These features are:

* **Retrieve Index Funds** - Investors can query a list of Index Funds they wish to invest in.
* **Buy Fund Tokens** - Investors may purchase fund tokens from the respective Index Funds they choose.
* **Sell Fund Tokens** - Investors may sell fund tokens to exit out of their positions.
* **Issue Tokens** - Much like from the [Index Fund Features](#index-fund-features), Investors can issue fund tokens in exchange for the appropriate underlying supported assets by the Index Fund.
* **Redeem Tokens** - Investors can redeem their fund tokens in exchange for the underlying asset of the Index Fund if they wish to exit.
* **Retrieve Debt Fund List** - Investors may query a list of Debt Funds they wish to invest in.
* **Supply Liquidity** - Investors may supply liquidity to a Debt Fund to claim fees.
* **Remove Liquidity** - Investors may exit from the Debt Fund.
* **Claim Fees** - Investors can claim fees from the Debt Fund.

## Design details

### Blueprints Overview
Farmers Market is made up of 6 core blueprints. These blueprints are `FarmersMarket`, `FarmerDashboard`, `BorrowerDashboard`, `InvestorDashboard`, `IndexFund`, `DebtFund`, and `FundingLocker`.

#### FarmersMarket Blueprint
The `FarmersMarket` blueprint is the main blueprint that primarily used by the protocol administrator. Since the Farmer Market protocol is a permissioned protocol, the protocol administrator(s) reserves the right to select who can use their protocol. Therefore, membership can be granted through this blueprint. This blueprint also keeps a global record of all the index funds, debt funds, and funding lockers in the protocol, among other key important things. 

#### Dashboard Blueprints
Since the `FarmerDashboard`, `BorrowerDashboard`, and `InvestorDashboard` blueprints serve similar functions, I'll group them all into this section for brevity. These Dashboards provide a module for users to interact with the protocol in a permissioned way. Investors don't require badges to access the `InvestorDashboard`, so it's the most straightforward blueprint. The component provides a set of methods to allow Investory-type users to query a list of Index Funds/Debt Funds to invest in and allow them to interact with those funds by either buying, selling, issuing tokens, redeeming tokens, or supplying/exiting liquidity. 

Similarly, the `FarmerDashboard` and `BorrowerDashboard` does the same. However, since Farmers and Borrowers need to vetted by the protocol administrator, they do require badges and more detailed authorization model to interact with the protocol. The reason Farmers (or otherwise known as Farmerss) require badges is because the protocol is meant to allow institutional player to access frictionless liquidity of DeFi market, whilst allowing retail investors access to high quality professional managers. 

Likewise, Borrowers require badges to allow Farmers (Farmerss) to have access to high quality institutional borrowers, which would help propel DeFi to have more examples of what undercollateralized lending looks like while allowing Borrowers to have more flexibility and access to capital markets beyond traditional finance. 

In addition to requiring badges to access methods in `FarmerDashboard` and `BorrowerDashboard`, each of the components will have its own admin badges contained in their respective vaults as well. These admin badges is used to provide secure cross-blueprint calls as these components will instantiate other components as well. For example, the `FarmerDashboard` will have responsibility to instantiate both `IndexFund` and `DebtFund` where each will have its own authorization model. So the considerations of how these blueprints connect with one another in a controlled way was an interesting exercise for me. After reading about Scrypto 0.5 with the introduction of Local Components, I can see how the solution I came up with can be significantly simplified. In the meantime, this design was my best solution for this challenge.

#### IndexFund Blueprint

As mentioned, the `FarmerDashboard` has the responsibility of instantiating `IndexFund`. Although, Farmers will not use their Farmer Badge to access the `IndexFund` component. Once instantiated, the Farmer will receive another admin badge to access `IndexFund` methods. Ideally, we would have only one badge to access multiple permissioned components; however, wile not yet supported, there will be a situation where the owner of their index fund will want to sell the ownership to another entity. Having the ability for the owner to sell their index fund without having to give up their Farmer badge was my way of structuring it like so.  

#### DebtFund Blueprint

Similarly to `IndexFund`, the `DebtFund` blueprint is instantiated by the `FarmerDashboard` component. It has similar authorization model as `IndexFund` with a little added complexity. The idea of having debt funds is to allow Farmers to not only have equity investment strategy, but debt investment strategies as well. For institutional lending, Borrowers require more sophisticated mechanisms to their loan that are mostly not available in DeFi. Maple Finance is the only protocol I'm aware of that is providing this type of product to institutions. As such, the loans need to be faciliated in a more orderly way. This is where the `FundingLocker` comes in. 

The `BorrowerDashboard` allows Borrowers to request loans through Loan Request NFTs. The reason we have it as so is that (while not currently supported), we can have changes to loan requests on-chain. On Ethereum and how it's done on Maple Finance, borrowers cannot change their loan request, but instead have to submit another transaction/request and let lenders on the protocol know that the previous loan request is stale. 

So how the process works is that, again, Borrowers request loans through the `BorrowerDashboard`. When Farmers who manage a debt fund strategy finds a loan and a borrower they like, they will proceed with negotiations, underwriting, and due dilligence off-chain. Once the terms are agreed upon, the Farmer will instantiate a `FundingLocker`. The `FundingLocker` as I will provide more color in the next section is where the loan will be faciliated. With the instantiation of the `FundingLocker`, a Loan NFT will be deposited into the component vault. Borrowers will be alerted that their loan has been accepted, to which the Loan Request NFT will be modified with the `FundingLocker` component address to access the component. Before the loan is funded, Borrowers must first provide enough collateralization upon the agreed terms. Once the collateralization requirement is met, the Loan Request NFT will be burnt and Borrowers will receive the Loan NFT. The Loan NFT will act as a badge to access the `FundingLocker`. 

On the Farmers side, there will be a badge minted to access the `FundingLocker` as well, but in an attempt to condense the amount of badges that users have to provide to access different types of permissioned components, the Farmer doesn't directly receive the badge to access the `FundingLocker`, instead, the badge will be deposited in the `DebtFund` component's vault where the Farmer can access the `FundingLocker` methods through the `DebtFund` component. While the Farmer doesn't have direct possession of the `FundingLocker` badge, because the Farmer has direct ownership of the `DebtFund`, it can be viewed that loans originated in the `DebtFund` are deemed to be the debt fund's assets. The Farmer may wish to sell individual loans (not yet supported) or sell the entirety of the fund to another entity (not yet supported).

#### FundingLocker Blueprint

As mentioned in the previous section the `DebtFund` instantiates the `FundingLocker`. The reason we have `FundingLocker` is to have a structured way of dispursing funds to the Borrower. This allows the Farmer to mitigate risk and faciliate the loan in a controlled way while the Borrower may benefit from access frictionless and programmatic capital on-chain.  

## Examples

### Getting Started

To get started let's make sure to have any data cleared.

```sh
resim reset
```

Firstly, we need to create our accounts. We can easily do this by pasting the follow commands and creating our environment variables.

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

We've created 4 accounts and they're identified by:

* `ACC_ADDRESS1` - Protocol Admin
* `ACC_ADDRESS2` - Farmer
* `ACC_ADDRESS3` - Investor
* `ACC_ADDRESS4` - Borrower  

Next we make sure that we've set `ACC_ADDRESS1` as the default account as it will be the protocol admin who first launches the protocol.

We can do so by running the following command:

```sh
resim set-default-account $ACC_ADDRESS1 $PRIV_KEY1
```

Let's publish our package and create our environment variable for our components and resources to use in our examples.

```sh
PK_OP=$(resim publish ".")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
M_OP=$(resim run "./transactions/farmersmarket.rtm")
export ORACLE=$(echo "$M_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export FARMER_COMPONENT=$(echo "$M_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export PROTOCOL_ADMIN=$(echo "$M_OP" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
R_OP=$(resim run "./transactions/radex.rtm")
export RADEX=$(echo "$R_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
D_OP=$(resim run "./transactions/degenfi.rtm")
export DEGENFI=$(echo "$D_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '3q;d')
export FLASH=$(echo "$D_OP" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
```

So here, we've instantiated the `PriceOracle`,`FarmersMarket`, `RaDEX`, and `DegenFi` components along with their component address so we can call its respective methods.

We've also set an environment variable for the badge: `PROTOCOL_ADMIN` that the Protocol Admin receives when first instantiating the protocol.

This badge is what allows the Protocol Admin to control the protocol as Farmers Market is a permissioned protocol.


### Example 1: Creating badges and permitting users

In this protocol, Farmers ans Borrowers must be accepted to be permitted into the protocol. In this example, we'll walk through how that is done.

First, let's head over to `ACC_ADDRESS2`, the Farmer.

```sh
resim set-default-account $ACC_ADDRESS2 $PRIV_KEY2
```

the prospective Farmer will first have to create a temporary badge to apply as a Farmer.

We can do so with the following transaction manifest file:

```sh
C_FMTB=$(resim run "./transactions/create_f_tb.rtm")
export F_TB=$(echo "$C_FMTB" | sed -nr "s/.* The resource address of your temporary badge is: ([[:alnum:]_]+)/\1/p")
```

So what just happened? 

The method
```rust
  pub fn create_temporary_badge(
      &mut self,
      name: String,
      user_type: UserType
  ) -> Bucket
```

Calls to have the prospective farmer to input their name (entity name) and the type of user they'd wish to apply for.  

If we view the transaction manifest file [./transactions/create_fm_tb.rtm]("./transactions/create_f_tb.rtm) we have the following:

```sh
CALL_METHOD
    ComponentAddress("${FARMER_COMPONENT}")
    "create_temporary_badge"
    "Farmer"
    Enum("Farmer");
```

We've selected to have `ACC_ADDRESS2` to become a Farmer and their name to simply be "Farmer".

If we use the following command:

```sh
resim show $ACC_ADDRESS2
```
```sh
Resources:
└─ { amount: 1, resource address: 03be0a1f999e73bdf435975b007351539451bf2bb14b67df5c8ea3, name: "Temporary Badge NFT", symbol: "TBNFT" }
   └─ NonFungible { id: 8c990d8838b707ad294cbc50471938c3, immutable_data: Struct("Farmer"), mutable_data: Struct(Enum("Farmer"), Enum("Pending")) }
```

We'll see in its wallet is the Temporary Badge with an Enum that says "Pending". This badge indicates that `ACC_ADDRESS2` is requesting to be a Farmer user type and is awaiting for approval. 

Before we get there, for convenience while our client is already on `ACC_ADDRESS2` lets have it mint and transfer tokens that we'll use for the examples.

```sh
M_OP=$(resim run "./transactions/mint_tokens.rtm")
export USD=$(echo "$M_OP" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export BTC=$(echo "$M_OP" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export DOGE=$(echo "$M_OP" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '3q;d')
export XRD=030000000000000000000000000000000000000000000000000004
resim run ./transactions/transfer_liquidity.rtm
```

Let's also set the price of these tokens to $1.

```sh
resim run ./transactions/set_price.rtm
```

Now that's out of the way, let's move over to `ACC_ADDRESS4`, the Borrower, and have it do the same as `ACC_ADDRESS2`, but for the Borrower role.

```sh
resim set-default-account $ACC_ADDRESS4 $PRIV_KEY4
C_BTB=$(resim run "./transactions/create_b_tb.rtm")
export B_TB=$(echo "$C_BTB" | sed -nr "s/.* The resource address of your temporary badge is: ([[:alnum:]_]+)/\1/p")
```

Now that `ACC_ADDRESS2` and `ACC_ADDRESS4` has temporary badges for the Farmer and Borrower role, let's move to the Protocol Admin to have it approve the users.

```sh
resim set-default-account $ACC_ADDRESS1 $PRIV_KEY1
```

For the farmer we will run this transaction manifest file:

```sh
C_F=$(resim run "./transactions/create_f.rtm")
export FDASHBOARD=$(echo "$C_F" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
```

If we view `ACC_ADDRESS2` account, we'll see the Temporary Badge has been changed to "Approved":
```sh
Resources:
├─ { amount: 1, resource address: 03be0a1f999e73bdf435975b007351539451bf2bb14b67df5c8ea3, name: "Temporary Badge NFT", symbol: "TBNFT" }
│  └─ NonFungible { id: 8c990d8838b707ad294cbc50471938c3, immutable_data: Struct("Farmer"), mutable_data: Struct(Enum("Farmer"), Enum("Approved")) }
```

Let's do the same for the `ACC_ADDRESS4`
```sh
C_B=$(resim run "./transactions/create_b.rtm")
export BDASHBOARD=$(echo "$C_B" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
```

So here not only have we approved `ACC_ADDRESS2` and `ACC_ADDRESS4` Temporary Badge, but we also had the Protocol Admin instanstiate the `FarmerDashboard` and `BorrowerDashboard` so each respective users can access their respective features.

While the Investor role does not require a badge, we also need to instantiate `InvestorDashboard` for them to use as well.

```sh
ID=$(resim run "./transactions/investor_dashboard.rtm")
export IDASHBOARD=$(echo "$ID" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
```

Now that we have things set up, let's see what the Farmers Market protocol can do!

### Example 2: Creating an Index Fund

Since `ACC_ADDRESS2` has been approved as a Farmer, we can now have it create our first Index Fund.

But before we can create one, we need to be able to access the `FarmerDashboard` and we can't do so without having the Farmer Badge.

Let's head over to `ACC_ADDRESS2` and have the Farmer claim their badge.

```sh
resim set-default-account $ACC_ADDRESS2 $PRIV_KEY2
C_CB=$(resim run "./transactions/claim_f_badge.rtm")
export F_BADGE=$(echo "$C_CB" | sed -nr "s/.* The resource address of your NFT is: ([[:alnum:]_]+)/\1/p")
```

When we look into its account we can now see the Farmer badge in its account:
```sh
Resources:
├─ { amount: 1, resource address: 03a9703c8f1a9a86621f9c67903a814e9d9767133c0bce4e454e54, name: "Farmer NFT", symbol: "F_NFT" }
│  └─ NonFungible { id: 180fc07741f24545d9d5797aabb293e5, immutable_data: Struct("Farmer", HashMap<Tuple, ComponentAddress>(), HashMap<ResourceAddress, ComponentAddress>()), mutable_data: Struct() }
```

We can now create our first Index Fund by running the following transaction manifest file:

```sh
C_IF=$(resim run "./transactions/new_index_fund.rtm")
export INDEX=$(echo "$C_IF" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export INDEX_BADGE=$(echo "$C_IF" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
```

So how does this Index Fund work and what is it composed of? 

Instantiating the Index Fund requires the Farmer to pass in these arguments:

```rust
impl IndexFund {
    
    pub fn new(
        fund_name: String,
        fund_ticker: String,
        fee_to_pool: Decimal,
        starting_share_price: Decimal,
        tokens: HashMap<ResourceAddress, Decimal>,
        price_oracle_address: ComponentAddress,
    ) -> (ComponentAddress, Bucket)
```

Farmers can name their Index Fund, set its token symbol, the fee collected for the pool, the starting share price, the token they wish to build the fund around and its starting weight.

Right now, it's an empty fund with a description of what the Farmer intends to do with the Index Fund. You saw earlier that we were minting tokens. For the purposes of this example, we'll be supplying this fund with `$DOGE`, `$BTC`, `$XRD`, and `$USD` coins.

We can do so by issuing tokens like so:

```sh
resim run ./transactions/issue_tokens.rtm
```
```sh
Logs: 19
├─ [INFO ] Cumulative value of the tokens passed: 5000
├─ [INFO ] Token Address: 030000000000000000000000000000000000000000000000000004
├─ [INFO ] Token weight: 0.25
├─ [INFO ] Amount of tokens passed: 1250
├─ [INFO ] Amount to mint: 1250
├─ [INFO ] Token Address: 03239e6b50205527dff639aa174c9065e1bc7d5401bc90102fb9d0
├─ [INFO ] Token weight: 0.25
├─ [INFO ] Amount of tokens passed: 1250
├─ [INFO ] Amount to mint: 1250
├─ [INFO ] Token Address: 038e6a15236a431a8c90504ebb16717ec47847b8021e153f025fb0
├─ [INFO ] Token weight: 0.25
├─ [INFO ] Amount of tokens passed: 1250
├─ [INFO ] Amount to mint: 1250
├─ [INFO ] Token Address: 0398a76d0eca910998086765cbfcf37cbf4c84e8fc11309d245bc9
├─ [INFO ] Token weight: 0.25
├─ [INFO ] Amount of tokens passed: 1250
├─ [INFO ] Amount to mint: 1250
├─ [INFO ] ["Radish Index" Fund]: Amount of "$RADSH" tokens issued: 5000
└─ [INFO ] ["Radish Index" Fund]: The resource address of "$RADSH" token is: 03ad4df374709f8dcad169e85c9624277a191575386fe60edbbbae
```

We see some information about how the tokens are issued to allow us to keep track of the calculation between the fund tokens and underlying asset.

Let's run the following transaction manifest file to view what was deposited and the current weighting of the Index Fund.

```sh
resim run ./transactions/view_token_weights.rtm
```
```sh
Logs: 5
├─ [INFO ] ["Radish Index" Fund]: The token weights are:
├─ [INFO ] Token Address: 038e6a15236a431a8c90504ebb16717ec47847b8021e153f025fb0 | Token Amount: 1250 | Token Value: 1250 | Token Weight: 0.25
├─ [INFO ] Token Address: 030000000000000000000000000000000000000000000000000004 | Token Amount: 1250 | Token Value: 1250 | Token Weight: 0.25
├─ [INFO ] Token Address: 0398a76d0eca910998086765cbfcf37cbf4c84e8fc11309d245bc9 | Token Amount: 1250 | Token Value: 1250 | Token Weight: 0.25
└─ [INFO ] Token Address: 03239e6b50205527dff639aa174c9065e1bc7d5401bc90102fb9d0 | Token Amount: 1250 | Token Value: 1250 | Token Weight: 0.25
```

We can now see what the fund is composed of.

Before we go further let's create an environment variable for $RADSH, our fund tokens.

```sh
export RADSH=03ad4df374709f8dcad169e85c9624277a191575386fe60edbbbae
```

To allow the Investor to purchase shares of our fund, we have to first create a liquidity pool to where the Investor can purchase the fund tokens from. If you remember, we instantiated `RaDEX` earlier. So let's create liquidity pools for our fund tokens and its underlying assets.

```sh
resim run ./transactions/new_liquidity_pool.rtm
```
```sh
Logs: 7
├─ [INFO ] [Pool Creation]: Creating new pool between tokens: 03ad4df374709f8dcad169e85c9624277a191575386fe60edbbbae-030000000000000000000000000000000000000000000000000004, of name: $RADSH-XRD, Ratio: 5000:5000
├─ [INFO ] [Pool Creation]: Creating new pool between tokens: 038e6a15236a431a8c90504ebb16717ec47847b8021e153f025fb0-03239e6b50205527dff639aa174c9065e1bc7d5401bc90102fb9d0, of name: 038e6a15236a431a8c90504ebb16717ec47847b8021e153f025fb0-03239e6b50205527dff639aa174c9065e1bc7d5401bc90102fb9d0, Ratio: 5000:5000
├─ [INFO ] [Pool Creation]: Creating new pool between tokens: 0398a76d0eca910998086765cbfcf37cbf4c84e8fc11309d245bc9-038e6a15236a431a8c90504ebb16717ec47847b8021e153f025fb0, of name: 0398a76d0eca910998086765cbfcf37cbf4c84e8fc11309d245bc9-038e6a15236a431a8c90504ebb16717ec47847b8021e153f025fb0, Ratio: 5000:5000
├─ [INFO ] [Pool Creation]: Creating new pool between tokens: 038e6a15236a431a8c90504ebb16717ec47847b8021e153f025fb0-030000000000000000000000000000000000000000000000000004, of name: 038e6a15236a431a8c90504ebb16717ec47847b8021e153f025fb0-XRD, Ratio: 5000:5000
├─ [INFO ] [Pool Creation]: Creating new pool between tokens: 0398a76d0eca910998086765cbfcf37cbf4c84e8fc11309d245bc9-03239e6b50205527dff639aa174c9065e1bc7d5401bc90102fb9d0, of name: 0398a76d0eca910998086765cbfcf37cbf4c84e8fc11309d245bc9-03239e6b50205527dff639aa174c9065e1bc7d5401bc90102fb9d0, Ratio: 5000:5000
├─ [INFO ] [Pool Creation]: Creating new pool between tokens: 03239e6b50205527dff639aa174c9065e1bc7d5401bc90102fb9d0-030000000000000000000000000000000000000000000000000004, of name: 03239e6b50205527dff639aa174c9065e1bc7d5401bc90102fb9d0-XRD, Ratio: 5000:5000
└─ [INFO ] [Pool Creation]: Creating new pool between tokens: 0398a76d0eca910998086765cbfcf37cbf4c84e8fc11309d245bc9-030000000000000000000000000000000000000000000000000004, of name: 0398a76d0eca910998086765cbfcf37cbf4c84e8fc11309d245bc9-XRD, Ratio: 5000:5000
```

Let's also have the Index Fund enable the DEX feature to allow the Farmer to rebalance the fund and the Investor to purchase `$RADSH`.

```sh
resim run ./transactions/integrate_dex.rtm
```
```sh
Logs: 1
└─ [INFO ] ["Radish Index" Fund]: RaDEx has been integrated! You may now use its controls.
```

### Example 3: Purchasing shares of Radish Index Fund

Now that we have that set up, we can now have the Investor purchase some Radish Index Fund shares.

```sh
resim set-default-account $ACC_ADDRESS3 $PRIV_KEY3
```

In the `InvestorDashboard`, Investors can retrieve a list of Index Fund that they wish to purchase from. We can see the list of available Index Fund by running the following transaction manifest file:

```sh
resim run ./transactions/retrieve_index_funds_lists.rtm
```
```
Instruction Outputs:
├─ HashMap<Tuple, ComponentAddress>(Tuple("Radish Index", "$RADSH"), ComponentAddress("02e57b6509378f049b62122a8e7b09d168f353a1fd8957847ce479"))
```

We now see the Radish Index Fund that we had the Farmer create earlier. Now let's have the Investor purchase some shares.

```sh
resim run ./transactions/buy_fund_tokens.rtm
```
```sh

```

Simulating a price increase, let's set the price of `$RADSH` to $2 to see what options the Investor can do.

Since this is a simple method call let's run the following command

```sh
resim call-method $ORACLE set_price $RADSH 2
```

Now that the price of `$RADSH` has appreciated, the investor has the following options: 1. Sell the tokens or 2. Redeem the tokens for the underlying asset.

Since the sell tokens feature is fairly straightforward... let's showcase what it looks like when we redeem the tokens for the underlying asset.

Let's run the following transaction manifest file:

```sh
resim run ./transactions/redeem_tokens.rtm
```





## Future work and improvements

This prototype is not production ready yet. While there needs to be many more testing and iterations to do for this to be prototype (I can definitely see some areas of improvements after the fact of building this lending protocol), I certainly would not get anywhere this close without the ease of Scrypto, the Radix Engine, and the Transaction Manifest. Here are a few things I have in mind that I'd like to explore more with this prototype:

* Researching risk analysis tools to quantify the risk of the protocol with various lending markets.
* Researching a better user experience for the liquidation mechanism (and user experience overall).
* Implement a more robust price oracle.
* Design better calculation mechanics to ensure accuracy.
* Research, implement, and experiment with securitization designs.
* Research and experiment more clever usage of flash loans.
* Research & design protocol economics
* Interest accrual calculations.

## Conclusion

This is my first attempt of developing a lending protocol or developing anything at all for that matter on my own; carrying out from design to implementation. It was a tremendous learning experience and incredibly fun visualizing assets being moved around in this protocol due its asset-orientedness. There may have been different parts of this design that could have been implemented better. I suppose you live and you learn. Major thanks to Florian, Omar, Peter Kim, Rock Howard, Clement, and Miso for talking out ideas with me and helping me out along the way. 

## License 

This work is licensed under Apache 2.0 and the license file is provided [here](./LICENSE).


