//! # Ground Finance: Make a Finance Ground for your journey into Web 3
//! 
//! Ground Finance is a blueprint package with 2 main usecases: on-chain credit service; on-chain lending protocol.
//! 
//! ## Current on-chain problem of uncollateral lending protocols:
//! 
//! Most on-chain lending protocol recent day cannot do uncollateral lending, thus missed the 11 Tn market potential of uncolleteral lending. The problem came from the contradictory between "trust" characteristic of uncollateral lending with on-chain "trustless" characteristic.
//! 
//! Some new projects are trying to challenge the problem through permissioned methods: 
//! - [Aave](https://docs.aave.com/developers/guides/credit-delegation) credit delegation solution: push the "trust problem" to "lenders".
//! - [GoldFinch](https://docs.goldfinch.finance/goldfinch/goldfinch-overview) trust through consensus solution: solve the "trust problem" through a consensus from many permissioned entities (Backers, Auditors).
//! - [Centrifuge Tinlake](https://docs.centrifuge.io/getting-started/centrifuge-at-a-glance/) full permissioned solution for Investors, Issuers and Asset Originators, highly require off-chain "trust".
//! - [TrueFi](https://blog.trusttoken.com/introducing-truefi-the-defi-protocol-for-uncollateralized-lending-9bfd6594a48)  permissioned solution for institution borrowers, voted through by a DAO and provide a risk-backed solution for lenders.
//! 
//! Although Ground Finance also used permissioned solutions, it combined the best charateristic of these 4 uncollateral lending solutions and evolved them into on-chain "consumer level" credit and "bank-like" lending solution while protecting lender's privacy, ensuring security and dynamic, transparent interest rate at the same time.

mod utils;
pub mod ground_credit;
pub mod ground_lending;