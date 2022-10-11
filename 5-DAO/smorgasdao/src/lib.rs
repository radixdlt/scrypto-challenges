#![allow(rustdoc::private_intra_doc_links)]

//! SmorgasDAO, a DAO with a number of configuration options that
//! plugs into any well-behaving component on the ledger to turn it
//! into a DAO.
//!
//! Of the contained modules only smorgasdao is needed to have a
//! functional DAO. The others exist for documentation and testing
//! purposes only.
//!
//! If you're building a package for deployment on ledger we recommend
//! you remove all but the smorgasdao module, to minimize the size of
//! the binary.
//!
//! # Understanding What This Does
//!
//! We recommend reading first the doc for the [smorgasdao module],
//! and then the [intermediary module] to gain a good understanding of
//! how the Radfolio functions.
//!
//! [radfolio module]: crate::smorgasdao
//! [investment vehicle module]: crate::intermediary
//!
//! In a nutshell, the `SmorgasDAO` component stores admin badges for
//! other components and calls them as its administrators would, but
//! only after consensus has been reached on a proposal in the
//! DAO. You can use this to turn a centralized admin team based
//! project into a decentralized one.
//!
//! # Bootstrapping
//!
//! In order to get started you need to put the [SmorgasDao] blueprint
//! on the ledger, and then call its [instantiate_smorgasdao]
//! function. This completed, you are now ready to start adding admin
//! badges to it.
//!
//! [SmorgasDao]: crate::smorgasdao::SmorgasDao_impl::SmorgasDao
//! [instantiate_smorgasdao]: crate::smorgasdao::SmorgasDao_impl::SmorgasDao::instantiate_smorgasdao
//!
//! # Test suite
//!
//! NOTE that due to time constraints the current test suite only
//! tests a single path through the DAO, and so other paths than this
//! must be considered suspect. (They have never been run.)
//!
//! What is tested is a DAO with anonmyous proposers and anonymous
//! voters. Use of identity badges has not been tested.
//!
//! The test suite is provided in the project source tree, in the
//! `tests` directory. In order to run these tests you must have the
//! resim tool available and you **must** run the tests
//! single-threaded: The tests use resim and that simulator
//! constitutes a shared global resource that prevents them from
//! running (successfully) in parallel. The following command line is
//! suitable for this:
//!
//! `cargo test -- --test-threads=1`
//!
//! On the author's system it takes around 40 seconds to run these
//! tests.
//!
//! Note that running the tests will remove any existing data inside
//! your simulator as `resim reset` is called several times during
//! execution.
//!
//! # Transaction manifests
//!
//! A set of transaction manifests is provided in the `rtm` directory
//! in the project source. Each is named identically to the Scrypto
//! method it covers. The functional part of these transaction
//! manifests is provided as inline documentation here in the web
//! docs, but if you open the actual file it will contain further
//! details as to its use. Each transaction manifest runs a complete
//! sequence of obtaining necessary resources and then calling the
//! method in question, before finally depositing any funds to the
//! user.
//!
//! There are three subdirectories in the `rtm` directory:
//!
//! `rtm/smorgasdao/` These are the manifests for the SmorgasDao
//! component itself. These are the ones you will deal with the most.
//!
//! `rtm/intermediary/` There is a manifest here for instantiating the
//! Intermediary component.
//!
//! `rtm/controlled/` This contains manifests for instantiating and
//! reading the demo controlled component that is used in the test
//! sutie.
//!
//! Note that we only provide manifests for functions that are
//! actually used in execution of the test suite provided. Since the
//! test suite is incomplete so is the manifest library at this point.
//!
//! # Development environment
//!
//! This project has been developed on Ubuntu Linux and while the
//! author expects everything in here to work on other platforms, if
//! you're having weird problems with it maybe this is the reason.
//!
//! This project has been developed for Scrypto v0.6.0.
//!
//! # License etc.
//!
//! This software is intended for entering into the **Radix Scrypto
//! DAO Challenge,** and the author cedes such rights as is necessary
//! to do so, ref. the challenge's official rules which are at time of
//! writing available
//! [here.](https://www.radixdlt.com/post/scrypto-dao-challenge-is-live)
//!
//! The author can be reached at `scryptonight@proton.me`


mod smorgasdao;
mod intermediary;
mod controlled;
