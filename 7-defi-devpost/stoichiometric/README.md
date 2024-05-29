# SUSD: Stoichiometric USD

Stoichiometric is a Collateralized Debt Position (CDP) Stablecoin ecosystem which integrates 3 main features: a DAO, a Stablecoin, an AMM.

# Table of contents
1. [DAO](#DAO)
2. [DEX](#DEX)
3. [Issuer](#Issuer)
4. [Tests](#Tests)
5. [Frontend](#Frontend)
6. [Backend](#Backend)
7. [References](#References)


## DAO
The DAO controls the whole stablecoin ecosystem. It has admin badge access to the DEX and the Stabelcoin lender. Users 
can make the following decisions:
- Change the voting period for the DAO
- Change the minimum amount of votes to make a proposal valid
- Change the parameters of a Lender
- Allow a new token to be used as collateral
- Give a non-fungible resource that allows the minting and burning of the stablecoin. This enables new protocols to enter 
- the stablecoin ecosystem
- Recall a stablecoin minter. This enables the users to remove a protocol from the ecosystem.

## DEX
The DEX is vastly inspired by TraderJoe, which is basically Uniswap v3 but with a constant-sum AMM between each tick. 
The DEX is built so that every pair has to include SUSD. The goal of the DEX is to concentrate as much liquidity around 
SUSD as possible. Therefore, the DEX pairs graph is a star with SUS at the center. This enables in theory to make SUSD 
more stable.

# Issuer
The stablecoin lender enables users to lock allowed collateral against SUSD. The liquidation process differs from 
MakerDAO, DAI and AAVE in the sense that it uses a constant-product AMM curve to decide the amount of collateral to be 
liquidated when the liquidation threshold is exceeded.

# Tests
Tests are written in the `tests` package and can be launched using:
```
cargo test -- --test-threads=1
```
Tests failed are not documented because they are mostly for development purposes.

# Frontend
The frontend directory includes the code to run the frontend.


# Backend
The backend directory includes the code needed to decode the gateway API.

# References
- Uniswap v3 whitepaper: https://uniswap.org/whitepaper-v3.pdf
- TraderJoe docs: https://docs.traderjoexyz.com/
- Curve's stablecoin withepaper: https://github.com/curvefi/curve-stablecoin/blob/master/doc/curve-stablecoin.pdf
- MakerDao docs: https://makerdao.com/en/whitepaper/
 

 ## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.

