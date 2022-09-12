//! A decentralized micro-finance implementation.
//!
//! Provides three blueprints which when used together allow you to
//! offer micro-finance services on the ledger:
//!
//! - A [Participant catalog](crate::participants) for tracking
//! lenders and borrowers,
//!
//! - a [Loan requesting service](crate::loanrequestor) for
//! negotiating terms for a loan,
//!
//! - and a [Loan manager](crate::loanacceptor) for managing repayment
//! of a loan.
//!
//! # What is micro-finance?
//!
//! Micro-finance first seriously entered the public consciousness
//! when Muhammed Yunus won the 2006 Nobel Peace Prize for having
//! developed, with Grameen Bank, a lending paradigm that was suitable
//! for financing local entrepeneurs in destitute areas of the world.
//!
//! Since then micro-finance has had its ups and downs and remains an
//! important source of finance in many developing nations.
//!
//! A key characteristic of a micro-finance loan is that the borrower
//! is generally poor and owns very little. They are consequently not
//! able to offer any collateral, and if their business venture should
//! fail they have nothing they could sell to make their creditors
//! whole. An investor into such an enterprise must therefore develop
//! strategies for managing the risk inherent in the paradigm, such
//! as:
//!
//! - Carefully consider each borrower and their reputation and
//! history. (We provide the Participants blueprint which can be used
//! to track reputation on-ledger, and we provide a formal arrears
//! flag that can be used to judge the borrower's performance in past
//! loans.)
//!
//! - Carefully consider the business idea that the borrower
//! proposes. This author gently advices not to invest into loans that
//! are intended for consumption rather than for some commercial
//! enterprise.
//!
//! - Spread their capital across multiple projects to reduce the
//! risk of losing everything in one failed venture. (We allow each
//! loan to be financed by multiple lenders to facilitate each lender
//! being able to part-finance many different loans instead of needing
//! to go all in on one.)
//!
//! - Ensure that the borrower does not exist in a vacuum but that
//! there is a local support network they can lean on to help them
//! manage both financial and practical matters in relation to their
//! business venture. (Our LoanAcceptor blueprint provides for an
//! optional "facilitator" which can be paid fees to support such an
//! operation. This will tend to incentivize local businesses to pick
//! up this important role -- each such local business will set up its
//! own requestor/acceptor pair so they can set their own facilitator
//! fee but they might all share the same Participant catalog.)
//!
//! In one report, micro-finance loans average less than £50 for the
//! whole loan; others put them at $100 to $500. Lenders in existing
//! solutions are known to put amounts such as $5 into loans and
//! display considerable pride in being able to help someone from
//! across the world realize their dreams.
//!
//! The blueprints herein are designed to cater to everything from £50
//! to $25,000 and beyond, but it is worth keeping in mind that to
//! large parts of the world a $100 loan can be life changing; and in
//! other parts of the world lending someone $5 towards that ambition
//! can be the simplest thing.
//!
//! # General workflow
//!
//! The general flow in obtaining and servicing a loan is as
//! follows. (This skips over a lot of options and detail in the
//! interest of clarity.)
//!
//! 1. Obtain a Participant NFT by calling
//! [Participants::new_participant][w1].
//!
//! 2. Start a new loan request by calling
//! [LoanRequest::request_loan][w2].
//!
//! 3. Other users pledge funds to your loan by calling
//! [LoanRequest::pledge_loan][w3].
//!
//! 4. When the loan request is fully funded you convert it into a
//! loan, and receive the principal, by calling
//! [LoanRequest::start_loan][w4].
//!
//! 5. You then service your loan by periodically calling
//! [LoanAcceptor::pay_installment][w5].
//!
//! 6. Your lenders claim their share of your repayments by calling
//! [LoanAcceptor::claim_lender_rewards][w6].
//!
//! When all installments have been paid, everyone is happy.
//!
//! There are mechanisms for handling late payments, facilitator fees,
//! and other, but they are beyond the scope of this bird's eye view
//! of the process.
//!
//! [w1]: crate::participants::blueprint::Participants::new_participant
//! [w2]: crate::loanrequestor::blueprint::LoanRequestor::request_loan
//! [w3]: crate::loanrequestor::blueprint::LoanRequestor::pledge_loan
//! [w4]: crate::loanrequestor::blueprint::LoanRequestor::start_loan
//! [w5]: crate::loanacceptor::blueprint::LoanAcceptor::pay_installment
//! [w6]: crate::loanacceptor::blueprint::LoanAcceptor::claim_lender_rewards
//!
//! # Bootstrapping
//!
//! In order to get started you will need to perform the following
//! steps. You are essentially setting up a full-fledged loan service
//! in doing this.
//!
//! 1. Create a new Participants instance by calling
//! [Participants::instantiate_participant_catalog][b1].  (Or use
//! someone else's but we are keeping it simple here.)
//!
//! 2. Create a new LoanRequestor instance by calling
//! [LoanRequestor::instantiate_requestor][b2], passing in the
//! Participants NFT address you just obtained.
//!
//! 3. Create a new LoanAcceptor instance by calling
//! [LoanAcceptor::instantiate_loan_acceptor][b3], passing in the
//! Participants NFT address and the admin address of the
//! LoanRequestor you created.
//!
//! 4. Activate the LoanRequestor by calling
//! [LoanRequestor::set_loan_acceptor][b4] with the address of the
//! LoanAcceptor you just created.
//!
//! If you are in doubt about this process, you might take a look at
//! the `test_loanacceptor_scenario_2` test case in the `tests`
//! directory, which does all of the above very early on.
//!
//! [b1]: crate::participants::blueprint::Participants::instantiate_participant_catalog
//! [b2]: crate::loanrequestor::blueprint::LoanRequestor::instantiate_requestor
//! [b3]: crate::loanacceptor::blueprint::LoanAcceptor::instantiate_loan_acceptor
//! [b4]: crate::loanrequestor::blueprint::LoanRequestor::set_loan_acceptor
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
//!     cargo test -- --test-threads=1
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
//! A complete set of transaction manifests is provided in the `rtm`
//! directory in the project source. They are organized into one
//! subdirectory for each of the three blueprints and are named
//! identically to the Scrypto method they cover. The functional part
//! of these transaction manifests is provided as inline documentation
//! here in the web docs, but if you open the actual file it will
//! contain further details as to its use. Each transaction manifest
//! runs a complete sequence of obtaining necessary resources and then
//! calling the method in question, before finally depositing any
//! funds to the user.
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
//! # License etc.
//!
//! This software is intended for entering into the Radix Scrypto
//! Lending Challenge, and the author cedes such rights as is
//! necessary to do so, ref. the challenge's official rules which are
//! at time of writing available
//! [here.](https://scrypto-lending.devpost.com/rules)
//!
//! The author can be reached at `scryptonight@proton.me`

mod loanacceptor;
mod loanrequestor;
mod participants;
