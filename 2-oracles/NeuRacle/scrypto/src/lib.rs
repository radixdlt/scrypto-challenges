//! NeuRacle provide decentralized, trustless data validation service 
//! to bring off-chain data into Radix Ledger.
//! 
//! NeuRacle is inspired by [Komodo Trustless Oracles](https://komodoplatform.com/en/blog/the-promise-of-smart-contracts-and-the-oracle-problem/)
//! to use Consensus Models for validating data 
//! off-chain and bring on-chain.
//! 
//! However, not every Consensus solved the Blockchain trilemma. 
//! The more security, decentralized that Oracles has, 
//! the harder it come to finality on each data validation,
//! destroy the liveness of data stream.
//!
//! Thus, NeuRacle will utilize the atomic-composability and 
//! unlimited scalability trait of Radix Ledger to bring data validation
//! to the next level.
//! 
//! This is only a NeuRacle prototype that work with-out sharding.

mod validator;
mod utilities;
mod neura_stable_coin;
mod neuracle;
mod neura_token;
