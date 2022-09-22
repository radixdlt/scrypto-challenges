#![allow(rustdoc::private_intra_doc_links)]

//! ArchDAO, an autonomous organization on the Radix ledger.
//!
//! Of the contained modules only archdao is needed for the dao management. 
//! The others exist for testing purposes only.
//!
//! 
//! # What This Does
//!
//! the `ArchDAO` component implements a convicting voting system for
//! the fund and execute proposal projects. Each proposal is the first stage of a project.
//! A proposal begins with an idea, then it gets voted, then funded and then executed.
//!
//! The main ArchDAO component has no knowledge of what the proposal are about neither it is 
//! its primary interest, its goal is to let users vote for their preferred proposal to make it a real project.
//! Then it concerns about tokens getting in and out.
//!
//! # Bootstrapping
//!
//! In order to get started you need to put the [ArchDAO] blueprint on
//! the ledger, and then call its [instantiate_archdao]
//! function. This completed, you are now ready to start adding
//! proposals to it.
//!
//!
//! We provide one mock propoposal for testing
//! purposes, in the [archproposalMock] blueprint. Call its
//! [instantiate_archproposal_mock] function any number of times to
//! create mocks to then add them to your ArchDAO.
//!
//!
//! # Test suite
//!
//! A comprehensive test suite is provided in the project source tree,
//! in the `tests` directory. In order to run these tests you must
//! have the resim tool available.
//!
//! `cargo test -- --test-threads=1 --nocapture`
//!
//!
//! # Transaction manifests
//!
//! A complete set of transaction manifests is provided in the `rtm`
//! directory in the project source. Each is named identically to the
//! Scrypto method it covers. The functional part of these transaction
//! manifests is provided as inline documentation here in the web
//! docs. Each transaction manifest runs a complete
//! sequence of obtaining necessary resources and then calling the
//! method in question, before finally depositing any resource to the
//! user.
//!
//! There are four subdirectories in the `rtm` directory:
//!
//! `rtm/archdao/` These are the manifests for the ArchDAO component
//! itself.
//!
//! `rtm/archproposal/` These manifests are used for reading data
//! from proposal
//!
//! `rtm/mock/` This contains a manifest for instantiating the mock proposal.
//!
//! # Development environment
//!
//!
//! This project has been developed for Scrypto v0.4.1.
//!
//! # License etc.
//!
//! This software is intended for entering into the **Radix Scrypto
//! Portfolio Management and Yield Farming Challenge,** and the author
//! cedes such rights as is necessary to do so, ref. the challenge's
//! official rules which are at time of writing available
//! [here.](https://www.radixdlt.com/post/portfolio-management-and-yield-farming-challenge-is-live)
//!
mod archdao;
mod archproposal;
mod archproposalmock;

