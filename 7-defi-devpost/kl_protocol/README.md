# KL Protocol: Lending protocol built on Radix DLT

## Overview
  
KL Protocol is a decentralized lending protocol built on Radix DLT with some key differences: the protocol works more as a pool, so the interest rate that matters is the borrowing interest rate. The protocol could take a small cut on the generated interest, but all the rest is paid back to the lender.

From a technical point of view, building on Radix unlocks some features that improve implementation and help produce a safe place for lenders and borrowers. The protocol is based around some key assumptions and choices that set it apart:

### AMM like pool and loan share

Lending pools are Single Token Liquidity pools with mechanics similar to those used by decentralized exchanges Automated Market Maker for tracking shares of pooled liquidity. So each depositor gets “Pool Share” representing a share of the pool and assures that all depositors fairly receive a share of collected interest. A pool-to-share ratio is calculated at each deposit and is also used to calculate withdrawal corresponding to pooled tokens at any time.

Borrowed amounts are tracked using the "loan share" concept. Similar to the lending "pool share" , a "loan share" will be calculated each time a borrower takes a loan, and the protocol will track the total amount of assets borrowed. The loan share will be used with a loan share ratio (similar to pool share ratio) to ensure the amount each borrower will have to repay.

The logic behind a pool share is quite simple and can be modeled using a question like this one :

> "If I want to add 100 units in the pool while there are 150 units already in the pool corresponding to 90 shares, how many shares will I get?" 

The answer is  100 times 90/150 = 60 shares and the pool will have 250 units. Our pool to share (or loan to share) in that exemple is 90/150. from there, my share amount remains constant until I decide to remove my unit from the pool. keeping input from previous exemple and supposing that the pool accrued 90 more units from interest and now topping now at 350 units. In that case the amount I will get will be the my shares divided by the current pool to share ratio. So if no new shares are added, I will get 60 / (150/350) = 136 units. Is it fair ? Well, let's check : I got 36 more units that represent 40% of the amount accrued in the pool during the period my unit was in. 40% is exactly the portion of the pool the moment I joined (100/250), So it seems fair.If the pool is empty then the initial pool to share is one (meaning that each unit i'm about to add represents my shares as I'm alone at that moment.

So, with the concept of pool and loan share, to accrue interest to the pool, we simply have to increase the total borrowed amount by the necessary amount based on the interest rate (yes, one operation) while the borrower keeps only the constant amount on loan share he has taken as loan.

### Pool Share as collateral
    

The protocol accepts only “Pool Share” tokens as collateral. If a user would like to add a whitelisted asset directly as collateral, the protocol will add the assets to the corresponding pool to get Pool Share tokens to use as collateral. This ensures that deposited collateral generates yield without specific logic and helps simplify the protocol.

### CDP NFT

Collateralized Debt positions (CDP) are represented by a Non-Fungible token. Only the protocol has the authority to update or burn that NFT. It is required for any operation on loans and collaterals. The CDP NFT also allows multiple assets collateral, backing multiple assets loans. 

## B. System description

With  assumption above and with the help of Scrypto and the Radix Engin, it was fairly straightforward. We could say that learning rust gives more challenges to the actual implementation.

  

At the end we have all standard features of the lending protocol like flashoan for debt and collateral swap, variable or fixed interest rate (via an external interest rate factory component) and more … Implemented in less than 3 weeks including the prof of concept user interface.

The protocol has 3 critical parts: the CDP NFT / HydratedCDP, the LendingPool component and the LendingPoolManager component.



![Bleuprints overview](https://res.cloudinary.com/daisvxhyu/image/upload/v1679330327/kl_protocol_system_description_vtmkfw.png)
  

### 1.  Collateralized Debt Position (CDP) and HydratedCDP Struct

A Collateralized Debt Position (CDP) is a non-fungible token (NFT) that represents a borrower's loan and collateral position in the lending market. To start, a borrower mints a CDP NFT, which is then used as the basis for their interaction with the LendingPoolManager component. Borrowers can use the CDP to take out loans and deposit collateral on any lending pool component that accepts the same asset type. This means that the same CDP can be used across all markets, allowing borrowers to have a diversified portfolio of loans across different assets.
The CDP token contains only those that are relevant (e.i that need the LendingPoolManager component authority to be updated) to be stored on chain . For now it will be loan amounts and collateral amounts. all other information like interest rate, loan status, current assets price and so on, will be put together in a wrapper struct called HydratedCDP. The HydratedCDP token is updated to reflect the borrower's loan and collateral positions across all computations where the CDP is supposed to be used.
It also provides helper methods to interact with relevant pools efficiently.
When done, components can use the HydratedCDP to easily produce a “Dry” CDP that can be stored in the CDP NFT.

### 2.  LendingPool Blueprint

The LendingPool component holds a whitelisted asset and handles all logic related to operation around this asset. Lenders can freely interact with a lending pool but borrowing related operations need an authority badge. This badge will be held by a “proxy” or master component ensuring that each operation can or can not be performed and also ensure that each change on loan and collateral status is properly mentioned and tracked in relevant places like the CDP NFT.

The lendingPool contains two vaults: one for lenders to deposit funds and another for borrowers to deposit collateral.

  

The Market component contains several methods that are used by borrowers and lenders. Borrowers can use the take_loan method to take out a loan, the repay_loan method to repay a loan, the add_collateral method to add collateral to their CDP, and the remove_collateral method to remove collateral from their CDP. Lenders can use the add_liquidity method to provide liquidity to the Market component and earn interest, and the remove_liquidity method to withdraw their funds and interest earned. Flash loan feature is also handled directly by the LendingPool component. Like adding and removing liquidity, flash loans are exposed directly to the users.

  

Additionally, the Market component includes an update_interest_rate method that is used to dynamically calculate the interest rate for loans using the concept of “Loan Share”. The interest rate is handled by an InteresFactory component. This component has a method that takes the pool state as input and returns back an interest rate. So from there any scenario is possible. We could base the interstate on several factors, including the current supply and demand for the asset on the market, the collateral-to-loan ratio or external factor provided via an oracle.

We also have a reference to an external component (like a price oracle) to feed price information to the pool.

  

### 3.  LendingPoolManager Blueprint

  

LendingMarke is the master component of the KL Protocol lending market. It contains a list of all the created LendingPool components and all required authority badges to operate them. As stated above all borrowing activity is handled by this component so it exposes some proxymethods to the user that validate presented CDP before any operation.

  

Another feature made available by the LendingPoolManager is liquidation. Liquidation happens when a CDP is considered unhealthy using a calculated health factor. When a CDP is considered unhealthy any one can liquidate associated collaterals to cover the loan without proving the CDP NFT.

  

There are two types of liquidation: The manual liquidation, that is available to any user and auto_liquidation that is reserved to admin batch liquidation. Both have different mechanics. Manual liquidation targets a specific collateral that the liquidator wants and provides required loan payment to get the liquidation to happen. On the other end, auto_liquidation analyzes the whole CDP and performs direct asset sales to improve the health factor.