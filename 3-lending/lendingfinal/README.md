# MescaLend

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

MescaLend is a proof-of-concept AMM-inspired lending protocol built on the Radix ledger using v0.4.1 of Scrypto: the smart contract language of the Radix ledger. 

It is based off of Timeswap, a recently developed lending protocol on Polygon that uses a novel constant product automated market maker function to enable market driven, oracleless, and permissionless fixed interest rate lending for any digital asset. 

## Why MescaLend?

Current popular lending protocol designs, such as the designs of AAVE and Compound, are: 
  * **Insecure**, as they are susceptible to oracle manipulation explotis. (See [Venus Protocol](https://quillhashteam.medium.com/200-m-venus-protocol-hack-analysis-b044af76a1ae) and [Compound](https://cryptobriefing.com/compound-user-liquidated-49-million-price-oracle-blamed/))
  * **Not fully permisionless**, as they require governance to allow certain assets to be lent and borrowed. 
  * **Not capitally efficient**, as they require governance to adjust interest rate models and to decide collateral ratios.
  * **Not useful for long-term economic projects**, as their interest rates are variable and there is risk of liquidation, making decentralized lending less appealing for both corporations and individuals.  

MescaLend, attempts to solve these challenges by implementing a novel lending protocol design inspired more by AMMs like Uniswap than by traditional designs like AAVE or Compound.

The MescaLend design is: 
  * **Secure**, as it doesn't use price oracles, so oracle manipulation exploits are impossible. 
  * **Fully permissionless**, as governance proposals are not needed to approve assets to be lent or borrowed, so any digital asset can be lent/borrowed.
  * **Capitally efficient**, as the interest rate and collateral ratios for asset/collateral pairs are determined entirely by the market.
  * **Useful for long-term economic projects**, as interest rates are fixed for borrowers and there is no liquidation feature. 


## Goals
Piers Ridyard, CEO of Radix DLT, touched on in [Radix Technical AMA #10](https://youtu.be/OHCNZDKMjRk?t=2552) the idea of DeFi providing liquid markets for smaller than microcap long-tail assets. Currently, Automated Market Makers like Uniswap create liquid markets for long-tail assets to be bought and sold by utilizing constant product market making liquidity pools. 

Although trading of long-tail assets is currently available in DeFi, there is a significant absence of lending and borrowing markets for long-tail assets. 

The goal of MescaLend is to provide secure and capitally efficient lending and borrowing markets for not only large-cap assets, but also for smaller-than-microcap assets and everything in between. 

Imagine small-business-entities like your local mom and pop shops being able to more easily borrow money to expand or improve their business by borrowing against the equity of their business. 


## Features

The currenty implementation of MescaLend allows the following features:
 * Allows for the creation of lending pools betwen any two fungible tokens. 
 * Allows for the borrowing of asset tokens by putting up collateral tokens. 
 * Allows for the paying back of borrowed asset tokens. 
 * Allows for the lending of asset tokens. 
 * Allows for the adding of liquidity to lending pools.
 * Allows for the removing of liquidity to lending pools.

## How it works

MescaLend's lending pool design is based on Timeswap's, which essentially is a 3 variable constant product formula inspired by Uniswap’s Constant Product equation. 

The formula is:  X * Y * Z = K

X = Principal Pool
Y = Interest Rate Pool
Z = Collateral Factor Pool

The interplay between these three variables determine the interest rate and collateral ratio between the asset and collateral tokens of any lending pool.

No one explains Timeswap's design than Timeswap themselves. Check out the following resources from Timeswap to better understand their design: 
 * [Medium: Timeswap 101: A simplified explainer](https://medium.com/timeswap/timeswap-101-a-simplified-explainer-fe098a2ec378)
 * [Medium: Timeswap AMM — A Deep Dive](https://medium.com/timeswap/timeswap-amm-a-deep-dive-1293e57bb10f)
 * [Timeswap White Paper](https://timeswap.io/whitepaper.pdf)

## Why is this better on Radix? 

Timeswap has this concept of native tokens, which manages the economics of the lending pools. This design is perfect for implementing on the Radix ledger, as Scrypto and the Radix Engine is designed for asset-oriented programming. I was able to easily create these native tokens with just a few lines of code for each native token. 

See the following to better understand native tokens and how they relate to lending pool economics:
 * [Medium: Timeswap Native Tokens](https://medium.com/timeswap/timeswap-native-tokens-50a5da587be0)
 * [Gitbook: Native Tokens](https://timeswap.gitbook.io/timeswap/getting-started/tokens)

## Future Improvements

As MescaLend is a proof-of-concept and not a complete implementation of the Timeswap design, MescaLend can be significantly improved by: 
 * Implementing a feature for lenders to retrieve collateral when a borrower defaults.
 * Implementing fee incentives for liquidity providers.
 * Implementing a minimum interest rate for lenders and borrowers. 
 * Implementing a method that burns collateralised debt NFTs when debt is fully repaid. 

## License
This work is licensed under Apache 2.0

## Acknowledgements

As I recently started learning Scrypto and just discovered Timeswap, I have had many questions and challenges as I developed MescaLend. Several Radix and Timeswap community members have helped answer those questions. Thus I stand on the shoulder of giants. 

I give thanks to: 
 * Omar for his development of RaDEX - it has been so incredibly helpful studying it
 * Florian Pieper and talesofbeem for answering my questions on Telegram and Discord
 * “Timelord” Riccson Ngo for creating the novel and innovative design behind Timeswap
 * vindoor from Timeswap Discord for answering my various questions on Timeswap's design
 * Bent on Telegram for helping me with Git!



