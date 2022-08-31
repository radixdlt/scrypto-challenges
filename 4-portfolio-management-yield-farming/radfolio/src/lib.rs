#![allow(rustdoc::private_intra_doc_links)]

//! Radfolio, a portfolio management system for the Radix ledger.
//!
//! Of the contained modules only radfolio is needed for the portfolio
//! management system. The others exist for documentation and testing
//! purposes only.
//!
//! If you're building a package for deployment on ledger we recommend
//! you remove all but the radfolio module, to minimize the size of
//! the binary.
//!
//! # Understanding What This Does
//!
//! We recommend reading first the doc for the [radfolio module], and
//! then the [investment vehicle module] to gain a good understanding
//! of how the Radfolio functions.
//!
//! [radfolio module]: crate::radfolio
//! [investment vehicle module]: crate::investmentvehicle
//!
//! In a nutshell, the `Radfolio` component implements a general
//! purpose investment fund which invests into any number of different
//! `InvestmentVehicle` implementations. Each such investment vehicle
//! is an abstraction of some investment opportunity on the ledger,
//! and its internal workings can be anything from trivially simple to
//! magnificently complex.
//!
//! The intent of the design is to allow the implementor of an
//! investment vehicle to encapsulate almost any type of investment
//! opportunity in it, such that as wide an array of opportunities as
//! possible can be made available to the Radfolio. The main Radfolio
//! component therefore has very little knowledge of what is going on
//! inside of each investment vehicle, but only concerns itself with
//! money going in and money coming back out.
//!
//! # Bootstrapping
//!
//! In order to get started you need to put the [Radfolio] blueprint on
//! the ledger, and then call its [instantiate_radfolio]
//! function. This completed, you are now ready to start adding
//! investment vehicles to it.
//!
//! [Radfolio]: crate::radfolio::blueprint::Radfolio
//! [instantiate_radfolio]: crate::radfolio::blueprint::Radfolio::instantiate_radfolio
//!
//! We provide one mock investment vehicle you can use for testing
//! purposes, in the [InterestBearingMock] blueprint. Call its
//! [instantiate_interestbearing_mock] function any number of times to
//! create mocks to your liking, then add them to your fund.
//!
//! [InterestBearingMock]: crate::interestbearingmock::blueprint::InterestBearingMock
//! [instantiate_interestbearing_mock]: crate::interestbearingmock::blueprint::InterestBearingMock::instantiate_interestbearing_mock
//!
//! In a real live system you would instead obtain or develop
//! investment vehicles that interface towards real investment
//! opportunities on the ledger.
//!
//! # Test suite
//!
//! A comprehensive test suite is provided in the project source tree,
//! in the `tests` directory. In order to run these tests you must
//! have the resim tool available and you **must** run the tests
//! single-threaded: The tests use resim and that simulator
//! constitutes a shared global resource that prevents them from
//! running (successfully) in parallel. The following command line is
//! suitable for this:
//!
//! `cargo test -- --test-threads=1`
//!
//! On the author's system it takes around 30 seconds to run these
//! tests.
//!
//! Note that running the tests will remove any existing data inside
//! your simulator as `resim reset` is called several times during
//! execution.
//!
//! # Transaction manifests
//!
//! A complete set of transaction manifests is provided in the `rtm`
//! directory in the project source. Each is named identically to the
//! Scrypto method it covers. The functional part of these transaction
//! manifests is provided as inline documentation here in the web
//! docs, but if you open the actual file it will contain further
//! details as to its use. Each transaction manifest runs a complete
//! sequence of obtaining necessary resources and then calling the
//! method in question, before finally depositing any funds to the
//! user.
//!
//! There are four subdirectories in the `rtm` directory:
//!
//! `rtm/radfolio/` These are the manifests for the Radfolio component
//! itself. These are the ones you will deal with the most.
//!
//! `rtm/investmentvehicle/` These manifests are used for reading data
//! from investment vehicles. This can be useful when populating a
//! front-end with relevant data.
//!
//! `rtm/mock/` This contains a manifest for instantiating the mock
//! investment vehicle we use in our test suite.
//!
//! `rtm/participants/` Contains two manifests you will need if you
//! want to create Participant NFTs to identify partners.
//!
//! All these transaction manifests are actively used in execution of
//! the test suite provided and so they are known to be correct.
//!
//! # Development environment
//!
//! This project has been developed on Ubuntu Linux and while the
//! author expects everything in here to work on other platforms, if
//! you're having weird problems with it maybe this is the reason.
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
//! The author can be reached at `scryptonight@proton.me`
mod radfolio;
mod investmentvehicle;
mod interestbearingmock;

