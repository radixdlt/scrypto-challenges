# Trading dApp 

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Trading dApp is a proof-of-concept protocol of a 'simulated' trading application operating on crypto tokens pair. It has been  built on the Radix ledger using v0.4.1 of Scrypto: the smart contract language of the Radix ledger.   
  
# Abstract 

The Trading dApp is a simulated simple trading application built to be used from the Portfolio dApp blueprint.

It contains a fixed number of vaults, exactly four vaults for trading on the pairs xrd/btc, xrd/eth, xrd/leo and a function that simulates the movements in their prices as epoch advances.

Trading dApp has only some simple rules:
- buy_generic(amount, ResourceAddress) -> a buy order is issued using for the amount specified and the resource address is the token that will be bought,
- sell_generic(bucket) -> a sell order is issued using the amount in the bucket and its  resource address


# Integration Test

The portfolio_dapp.sh is a bash script that contains all the functions and methods tested, also buy/sell methods of this simple blueprint

# Unit Test

Execute 'scrypto test' 

# TODO 

Implement a LazyMap containing ResourceAddress and Vaults instead of fixed crypto token pairs