//! HareSwap is a (prototype) decentralized exchange platform that implements
//! part of the Swap protocol to create a peer-to-peer protocol for trading any
//! resource (fungible and non-fungible) on the Radix Ledger.
//! 
//! Yes, inspired by [AirSwap](https://about.airswap.io). :)
//! 
//! This implementation is an adaptation a subset of the Swap protocol and Request-for-Quote
//! interactions described here:
//! - Swap Protocol: <https://www.airswap.io/whitepaper.htm>
//! - Request-for-Quote: <https://about.airswap.io/technology/request-for-quote>
//! 
mod account;
mod maker;
mod model;
mod requirement;
mod transporter;

/// The api used for off-ledger operations
pub mod api {
    pub use super::model::*;
    pub use super::requirement::*;
    pub use super::transporter::authentication::{sign, verify, VerifyError};
    pub use super::transporter::voucher::Voucher;
}
