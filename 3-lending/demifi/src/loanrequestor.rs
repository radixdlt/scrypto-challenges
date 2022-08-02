//! Manages the negotiation and issuing of a loan.
//!
//! The LoanRequestor component handles the negotiation of terms for a
//! loan. A prospective borrower creates a LoanRequest by specifying
//! the terms he wants, and lenders may then inspect these terms and
//! decide wehether to pledge funds towards the loan. The request is
//! represented by a LoanRequest NFT which will typically be deposited
//! to the borrower's wallet. (The borrower needs to be able to build
//! transactions using this NFT as proof and so if placed in other
//! locations it may not be usable at all.)
//!
//! # Multiple lenders
//!
//! The component allows a loan request to be filled by anywhere from
//! one to any number of lenders so that, for example, a $100 loan
//! request might be filled by one person pledging $50, another
//! pledging $25 and five others pledging $5 each.
//!
//! # Minimum pledge
//!
//! The borrower can specify a minimum pledge amount and if so,
//! pledges below this number will not be accepted. This feature is
//! offered in recognition that the larger the number of lenders he
//! has to relate to the harder it is for the borrower to manage the
//! social aspects of the loan. Note that notwithstanding this
//! limitation, a lender will always be allowed to fill the last
//! remainder of the loan request such that e.g. if $100 is requested
//! with a minimum pledge of $50 and $80 has already been pledged
//! another lender can still pledge the final $20.
//!
//! # Managing pledges
//!
//! Outside of the lock periods (see below) lenders can withdraw their
//! pledges from a loan request at any time. A lender can increase his
//! pledge by simply pledging more, or decrease it by withdrawing all
//! of it and re-pledging a smaller amount.
//!
//! # Converting the request into a loan
//!
//! The first time the sum of active pledges towards a request reaches
//! the amount originally requested, the "loan filled" lock period
//! starts (see below) and the borrower can choose to convert the loan
//! request into an actual loan. This will burn the LoanRequest NFT
//! which gets replaced ny a new Loan NFT the lifecycle of which is
//! governed by the LoanAcceptor component. If the "loan filled" lock
//! period expires the borrower can still convert the request into a
//! loan, but at this point lenders can withdraw their pledges again
//! and if they do then it cannot be converted into a loan until more
//! pledges arrive to make up the difference. (The "loan filled" lock
//! will not trigger more than once for any given LoanRequest.)
//!
//! # Cancelling a request
//!
//! The borrower can cancel a loan request at any time. If he does
//! then no more funds can be pledged towards it and lenders who have
//! already pledged can immediately start withdrawing their funds
//! (lock periods no longer apply in this case).
//!
//! Note that because we allow lenders to go below the minimum pledge
//! when they're filling the remainder of the request, it is possible
//! to get several lenders who are below the minimum pledge when we
//! are outside of the lock periods. This can happen when lenders
//! leave and enter in such a way that there have been several
//! occasions in which someone came in to "fill the remainder". The
//! best way to avoid this happening is to ensure that you convert
//! your request into a loan before the "loan filled" lock period
//! ends. If your request has in your view become too full of tiny
//! pledges then your best remedy is probably to cancel the request
//! and start a new one.
//!
//! # Lock periods
//!
//! In order to provide some stability for the borrower (and cut down
//! on trolling), the borrower can specify two lock periods for
//! pledges:
//!
//! ## The pledge lock period
//!
//! The pledge lock period is an amount of time from the loan request
//! being published on ledger to it allowing lenders to withdraw their
//! pledges from it. During this period lenders can pledge funds into
//! the loan request but must wait until the lock period is over
//! before they can withdraw them. The author suggests that a suitable
//! pledge lock period might be anywhere from a few days to a couple
//! of weeks. It is expected that lenders will be reluctant to pledge
//! to a request with a very long pledge lock period and that front
//! ends will make this aspect readily visible to them, so that
//! borrowers are incentivized to keeping the lock periods within
//! reason.
//!
//! ## The "loan filled" lock period
//!
//! The "loan filled" lock period is an amount of time starting when
//! the full requested amount has been pledged to the loan request,
//! during which lenders will (again) not be able to withdraw their
//! funds. This gives the borrower some time to discover that the
//! request is now ready and he can make the transaction to convert it
//! into a loan (this same transaction also pays him the borrowed
//! funds). The author suggests that a reasonable "loan filled" lock
//! period may be anywhere from one to three days.
//!
//! If the two lock periods overlap then the longest lock period
//! applies.
//!
//! ---
//!
//! **Front-end design advice:** When either of the lock periods gets
//! long (e.g. months) the UI should give a clear warning about this,
//! and when they are absurd (e.g. years) the UI might flat out warn
//! that this looks like a scam.
//!
//! ---
//!
//! # Timekeeping
//!
//! Note that this blueprint was written for Scrypto v0.4 and at this
//! point there is no accurate timekeeping mechanism available to
//! smart contracts. The only option is epoch counting which is very
//! inaccurate and certainly unsuitable for financial apps such as
//! this. This being the case the author has not added sophisticated
//! support for policing too long lock periods etc. in the smart
//! contracts themselves. The timing system will necessarily need to
//! be revamped at a later point anyway if and when Scrypto starts
//! offering more accurate timekeeping. (Or perhaps we start using a
//! time oracle, which is outside the scope of the current development
//! goal.)
//!
//! ---
//!
//! **Future configuration possibilities:** In future work we might
//! offer the following configuration options for the Loan Requestor
//! instance itself:
//!
//! - Max lock period durations
//! - Fixed lock periods (i.e. the borrower can't choose)
//!
//! ---
//!
//! # Loan request options
//!
//! When creating a loan request, the borrower has a number of options
//! open to him. These place limits and terms on both the loan request
//! negotiation and also on the loan itself once (if) it is created.
//!
//! request_token and request_amount: The key parameters of the
//! request, this is the loan that the borrower is asking for.
//!
//! loan_purpose_summary: A short text describing the purpose of the
//! loan, this is the borrower's first marketing blurb to prospective
//! lenders.
//!
//! loan_purpose_url: The borrower might give a URL here where he goes
//! into more detail about the loan, himself, etc.
//!
//! promise_payment_intervals: The amount of time before the borrower
//! promises to pay the first installment on the loan, and also the
//! amount of time between installments.
//!
//! promise_installments: The total number of installments the
//! borrower promises to pay.
//!
//! promise_amount_per_installment: The number of tokens the borrower
//! promises to pay in each installment. This is in the same token as
//! the loan itself.
//!
//! ---
//!
//! **Future configuration possibilities:** In future we might offer
//! repayment to be in a different token than the loan itself.
//!
//! ---
//!
//! **Front-end design advice:** The UI should be careful to advice
//! lenders about the financial implications to themselves of going
//! into a loan, e.g. by clearly displaying the APY implied by the
//! terms offered. If the borrower is promising less in repayment than
//! the sum of the loan itself the UI should warn the user. The UI
//! should take the facilitator fee into account when doing all this
//! (see [crate::loanacceptor] for more on this parameter).
//!
//! ---
//!
//! minimum_share: The minimum pledge that is accepted to this loan
//! (see above for more details).
//!
//! pledge_lock_epochs, loan_filled_lock_epochs: See above for
//! details.
//!
//! # Requestor / Acceptor / Participants
//!
//! You will need to create at least one LoanRequestor instance for
//! each Participants catalog (see [crate::participants]) you want to
//! offer loans to. (The author expects that normally a single
//! Participants catalog should get the job done --- but I don't know
//! your business.)
//!
//! Each LoanRequestor instance needs to be paired with one
//! LoanAcceptor instance (see [crate::loanacceptor]). The requestor
//! manages the loan request negotiation, and the acceptor converts
//! the request into an actual loan and manages the repayment etc. of
//! that loan.
//!
//! # Load balancing
//!
//! A single LoanRequestor/LoanAcceptor instance pair should be all
//! you need to manage any number of loans. However you can connect a
//! single Participants catalog to multiple requestor/acceptor pairs
//! if you want. One reason for doing this might be to distribute load
//! across multiple shard groups if your service becomes wildly
//! successful.  In this case it is the job of your front-end to
//! distribute users' loan requests among the available LoanRequestor
//! instances to achieve the load balancing.
//!
//! While adding requestor/acceptor pairs is easy and can be done
//! dynamically, there is currently no provision for removing
//! them. (Other than your front-end could stop sending them new
//! requests).
//!
//! Note that if you use multiple requestor/acceptor pairs then users
//! might notice that the NFTs they receive are from different series
//! (i.e. they have varying ResourceAddresses). This should not affect
//! them much so long as you take into account in your front-end that
//! each LoanRequestor and LoanAcceptor instance has its own NFT
//! address.

use scrypto::prelude::*;

use crate::participants::Participant;

/// This is the NFT data for a loan request. It is used for
/// negotiating the loan with prospective lenders.
///
/// If the loan request is cancelled the owner can burn it once
/// everyone has pulled their funds out, and if the loan request gets
/// converted to a loan it is automatically burnt.
#[derive(NonFungibleData)]
struct LoanRequest {
    /// If this is set to true then the loan request has been
    /// cancelled.
    #[scrypto(mutable)]
    cancelled: bool,

    /// This is the token the borrower requests (e.g. XRD).
    request_token: ResourceAddress,

    /// This is the amount of tokens the borrower wants to borrow.
    request_amount: Decimal,

    /// You cannot pledge less than the minimum share, except if
    /// you're the last to fill a loan.
    minimum_share: Decimal,

    /// The time this loan request was created on ledger.
    request_start_epoch: u64,

    /// The number of epochs after request creation before lenders can
    /// withdraw any pledges they have made.
    pledge_lock_epochs: u64,

    /// The epoch in which the full request was first filled. This
    /// starts the "loan filled" lock period.
    #[scrypto(mutable)]
    loan_filled_epoch: Option<u64>,

    /// The number of epochs after the request is first filled that
    /// must pass before lenders can withdraw their pledges.
    loan_filled_lock_epochs: u64,

    /// The number of epochs before the first installment is due, and
    /// also the number of epochs between installments.
    promise_payment_intervals: u64,

    /// The number of installments the borrower promises to pay.
    promise_installments: u64,

    /// The amount of tokens the borrower promises to pay in each
    /// installment.
    promise_amount_per_installment: Decimal,

    /// A short blurb summarizing what the loan is for.
    loan_purpose_summary: String,

    /// A link to a deeper explanation of the loan.
    loan_purpose_url: String,

    /// The Participant id of the borrower who created this request.
    borrower_id: NonFungibleId,
}

blueprint! {

    struct LoanRequestor {
        /// The resource address of our LoanRequest NFTs
        nft_address: ResourceAddress,

        /// Our internal admin badge, used for managing LoanRequest
        /// NFTs.
        admin_badge: Vault,

        /// A temporary configuration badge used to bootstrap the
        /// requestor. The badge itself is burnt upon calling
        /// [LoanRequestor::set_loan_acceptor].
        config_badge_addr: ResourceAddress,

        /// We only interact with Participants from this catalog,
        /// others are refused access to our services
        participants_nft_address: ResourceAddress,

        /// The LoanAcceptor (see [crate::loanacceptor]) we use for
        /// converting loan requests into loans.
        loan_acceptor: Option<ComponentAddress>,

        /// The funds that have come in from prospective lenders.
        principals: HashMap<NonFungibleId, HashMap<NonFungibleId, Vault>>,
    }

    impl LoanRequestor {


        /// Creates a new LoanRequestor instance.
        ///
        /// A LoanRequestor must be associated with a Participants
        /// catalog (see [crate::participants]).
        ///
        /// The access control of the methods herein is closely tied
        /// to the Participant id of the calling party, and usually a
        /// Proof must be provided that you are a valid
        /// Participant. We only allow Participants from the catalog
        /// associated with the participants_nft_addr passed to us
        /// here.
        ///
        /// Three new resources are created by this function: An admin
        /// badge we hold on to for managing our NFTs, a configuration
        /// badge you will use to boostrap the new instance, and a new
        /// NFT series for our Loan NFTs. The admin badge and the NFT
        /// series both receive default names unless you override
        /// those names by giving them in admin_badge_name and
        /// nft_resource_name. You cannot name the configuration badge
        /// because it's too temporary to matter.
        ///
        /// The return tuple contains, in order:
        ///
        /// 0. The component address of the new LoanRequestor instance
        /// 1. The resource address of our LoanRequest NFTs
        /// 2. The resource address of our own admin badge (this needs
        /// to be passed to [instantiate_loan_acceptor][ila] as
        /// `requestor_admin_addr` when you create our companion
        /// acceptor.
        /// 3. A bucket containing the configuration badge.
        /// 4. The resource address of the configuration badge.
        ///
        /// Note that before the new instance can be functional you
        /// need to also call [LoanRequestor::set_loan_acceptor] with
        /// a reference to the acceptor to use. Before you do this
        /// most methods on the new instance will panic.
        ///
        /// [ila]: crate::loanacceptor::blueprint::LoanAcceptor::instantiate_loan_acceptor
        ///
        /// ---
        ///
        /// **Access control:** The supplied Participants NFT address
        /// dictates who gets to interact with the instance created.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/instantiate_requestor.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/instantiate_requestor.rtm")]
        /// ```
        pub fn instantiate_requestor(participants_nft_address: ResourceAddress,
                                     admin_badge_name: Option<String>,
                                     nft_resource_name: Option<String>)
                                     -> (ComponentAddress, ResourceAddress, ResourceAddress,
                                         Bucket, ResourceAddress)
        {
            // The admin_badge is mostly used for controlling our
            // LoanRequest NFTs
            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", admin_badge_name.unwrap_or(
                    "Loan Request NFT control badge".to_string()))
                .initial_supply(1);
            let admin_badge_addr = admin_badge.resource_address();

            // The config_badge is used, once, for setting our config,
            // then it gets burnt.
            let config_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Loan Request config badge")
                .burnable(rule!(require(admin_badge.resource_address())), LOCKED)
                .initial_supply(1);
            let config_badge_addr = config_badge.resource_address();

            // Our LoanRequest NFTs
            let nft_address = ResourceBuilder::new_non_fungible()
                .metadata("name", nft_resource_name.unwrap_or("Loan Request NFT".to_string()))
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            
            let requestor =
                Self {
                    nft_address,
                    admin_badge: Vault::with_bucket(admin_badge),
                    config_badge_addr,
                    participants_nft_address,
                    loan_acceptor: None,
                    principals: HashMap::new(),
                }
            .instantiate()
                .globalize();

            // All methods that require access control in this
            // blueprint handle this themselves through the Proof or
            // Bucket instances provided to them.

            (requestor, nft_address, admin_badge_addr,
             config_badge, config_badge_addr)
        }

        /// Establishes the LoanAcceptor we use to create loans.
        ///
        /// This method must be called on a new LoanRequestor instance
        /// before it will start working.
        ///
        /// The configuration badge must be supplied and it will be
        /// burnt since this is the only bit of configuration to do
        /// and it should only be done once.
        ///
        /// This method will panic if the acceptor has already been
        /// set.
        ///
        /// ---
        ///
        /// **Access control:** Requires our configuration badge to be
        /// provided in the bucket.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/set_loan_acceptor.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/set_loan_acceptor.rtm")]
        /// ```
        pub fn set_loan_acceptor(&mut self,
                                 config_badge: Bucket, loan_acceptor: ComponentAddress)
        {
            assert!(self.loan_acceptor.is_none(),
                    "Loan acceptor is already set");
            assert_eq!(config_badge.resource_address(), self.config_badge_addr,
                       "Provide the configuration badge so we can burn it");
            assert_eq!(Decimal::one(), config_badge.amount(),
                       "Exactly one configuration badge is needed");

            self.admin_badge.authorize(||  {
                config_badge.burn();
            });
            self.loan_acceptor = Some(loan_acceptor);
        }

        /// If you need money for anything, use this to create a new
        /// loan request.
        ///
        /// You specify in the call what terms you offer for the
        /// loan. These parameters map to the same in the
        /// [LoanRequest] struct.
        ///
        /// The method returns a bucket with the LoanRequest NFT
        /// created, and the id of that NFT.
        ///
        /// ---
        ///
        /// **Access control:** The borrower proof must be a
        /// recognized Participant
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/request_loan.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/request_loan.rtm")]
        /// ```
        pub fn request_loan(&mut self,
                            borrower: Proof,
                            request_token: ResourceAddress,
                            request_amount: Decimal,
                            minimum_share: Decimal,
                            pledge_lock_epochs: u64,
                            loan_filled_lock_epochs: u64,
                            promise_payment_intervals: u64,
                            promise_installments: u64,
                            promise_amount_per_installment: Decimal,
                            loan_purpose_summary: String,
                            loan_purpose_url: String) -> (Bucket, NonFungibleId) {
            assert!(self.loan_acceptor.is_some(),
                    "Loan acceptor is not set");
            assert!(request_amount > Decimal::zero(),
                    "Loan amount must be above zero");

            let (borrower_id, _, _) =
                self.check_and_retrieve_participant(borrower);

            let nfid: NonFungibleId = NonFungibleId::random();
            let nft: Bucket = self.admin_badge.authorize(||
                borrow_resource_manager!(self.nft_address)
                    .mint_non_fungible(
                        &nfid,
                        LoanRequest {
                            cancelled: false,
                            request_token,
                            request_amount,
                            minimum_share,
                            request_start_epoch: Runtime::current_epoch(),
                            pledge_lock_epochs,
                            loan_filled_epoch: None,
                            loan_filled_lock_epochs,
                            promise_payment_intervals,
                            promise_installments,
                            promise_amount_per_installment,
                            loan_purpose_summary,
                            loan_purpose_url,
                            borrower_id,
                        }
                    )
            );
            self.principals.insert(nfid.clone(), HashMap::new());
            (nft, nfid)
        }
        
        /// Call this method to cancel a loan request. That request
        /// can then never turn into a loan nor can more pledges be
        /// made towards it.
        ///
        /// If a cancelled request has had all pledged funds removed
        /// from it the LoanRequest NFT can be burnt by the owner
        /// calling the [LoanRequestor::burn] method.
        ///
        /// ---
        ///
        /// **Access control:** The request must belong to the borrower
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/cancel_request.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/cancel_request.rtm")]
        /// ```
        pub fn cancel_request(&mut self, borrower: Proof, request: Proof) {
            assert!(self.loan_acceptor.is_some(),
                    "Loan acceptor is not set");

            let (request_id, _, mut data) = self.check_and_retrieve_request(request);
            let (borrower_id, _, _) = self.check_and_retrieve_participant(borrower);
            assert_eq!(borrower_id, data.borrower_id,
                       "This is not your loan request");

            data.cancelled = true;
            self.save_request_data(&request_id, data);
        }

        /// Call this method to pledge funds towards a loan.
        ///
        /// The method will panic if the request has been cancelled,
        /// the loan request has already been filled, or if your
        /// pledge is too small.
        ///
        /// If you pledged more than was needed then the difference
        /// will be returned out of this method.
        ///
        /// ---
        ///
        /// **Access control:** Allows anyone who is a recognized Participant
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/pledge_loan.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/pledge_loan.rtm")]
        /// ```
        pub fn pledge_loan(&mut self, lender: Proof,
                           request_id: NonFungibleId, mut pledge: Bucket)
                           -> Bucket
        {
            assert!(self.loan_acceptor.is_some(),
                    "Loan acceptor is not set");

            let (request_id, _, mut request_data) =
                self.retrieve_request_from_id(request_id);
            assert_eq!(request_data.request_token, pledge.resource_address(),
                       "The borrower does not request this token type");
            assert!(!request_data.cancelled,
                    "This loan request has been cancelled by the borrower");

            let (lender_id, _, _) = self.check_and_retrieve_participant(lender);
            let pledge_map = self.principals.get_mut(&request_id).unwrap();
            let mut amount = pledge.amount();
            let tot_pledged = LoanRequestor::calc_total_pledge(&pledge_map);
            assert!(tot_pledged < request_data.request_amount,
                    "The loan is already fully pledged");
            let mut final_amount = false;
            if amount + tot_pledged >= request_data.request_amount {
                // Don't lend more than was asked
                amount = request_data.request_amount - tot_pledged;
                final_amount = true;
            }

            if !pledge_map.contains_key(&lender_id) {
                pledge_map.insert(lender_id.clone(),
                                  Vault::new(request_data.request_token));
            }
            let pledge_vault = pledge_map.get_mut(&lender_id).unwrap();
            pledge_vault.put(pledge.take(amount));
            assert!(final_amount || pledge_vault.amount() >= request_data.minimum_share,
                    "You must pledge at least {} tokens towards this loan",
                    request_data.minimum_share);

            if final_amount && request_data.loan_filled_epoch.is_none() {
                // The first time the loan gets filled we start the loan filled counter
                // during which time lenders cannot rescind.
                request_data.loan_filled_epoch = Some(Runtime::current_epoch());
                self.save_request_data(&request_id, request_data);
            }
            pledge // returns any change
        }

        /// Call this method to rescind the pledge you made to this
        /// loan.
        ///
        /// The method will panic if the request is in one of the lock
        /// periods. (Try again later.)
        ///
        /// Your pledged funds will be returned out of this method.
        ///
        /// ---
        ///
        /// **Access control:** Only the funds of the lender proof are
        /// rescinded
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/rescind_loan.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/rescind_loan.rtm")]
        /// ```
        pub fn rescind_loan(&mut self, lender: Proof, request_id: NonFungibleId)
                           -> Bucket
        {
            let (request_id, _, request_data) =
                self.retrieve_request_from_id(request_id);
            if  !request_data.cancelled {
                assert!(request_data.request_start_epoch + request_data.pledge_lock_epochs
                        <= Runtime::current_epoch(),
                        "This loan request is still in the pledge lock period");
                if request_data.loan_filled_epoch.is_some() {
                    assert!(request_data.loan_filled_epoch.unwrap()
                            + request_data.loan_filled_lock_epochs
                            <= Runtime::current_epoch(),
                            "This loan request is still in the loan filled lock period");
                }
            }

            let (lender_id, _, _) = self.check_and_retrieve_participant(lender);
            let pledge_map = self.principals.get_mut(&request_id).unwrap();
            pledge_map.get_mut(&lender_id).unwrap().take_all()
        }
        
        /// If your loan request is fully funded you can call this
        /// method to convert it into a loan. This returns to you the
        /// principal of the loan.
        ///
        /// You must pass in a bucket containing the LoanRequest NFT.
        ///
        /// The method will panic if the request has been cancelled or
        /// if it's not fully funded.
        ///
        /// When successful a new Loan NFT will be created and
        /// returned to you, and the LoanRequest you passed in will be
        /// burnt.
        ///
        /// The loan principal will be returned out of this method as
        /// will the new Loan NFT. Also the id of that NFT is
        /// returned.
        ///
        /// ---
        ///
        /// **Access control:** The borrower proof must be same as the
        /// request owner.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/start_loan.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/start_loan.rtm")]
        /// ```
        pub fn start_loan(&mut self, borrower: Proof, request: Bucket)
                          -> (Bucket, Bucket, NonFungibleId) {
            assert!(self.loan_acceptor.is_some(),
                    "Loan acceptor is not set");
            let (borrower_id, _, _) = self.check_and_retrieve_participant(borrower);
            let (reqid, _, request_data) = self.check_and_retrieve_request_from_bucket(&request);
            assert_eq!(request_data.borrower_id, borrower_id,
                       "This is not your loan");
            assert!(!request_data.cancelled,
                    "This loan request has been cancelled by the borrower");

            let pledge_map = self.principals.get_mut(&reqid).unwrap();
            assert!(request_data.request_amount <= LoanRequestor::calc_total_pledge(&pledge_map),
                    "Insufficient amount pledged towards loan");

            // Make a map of (lenders)=>(pledged amounts) to send to the acceptor
            let mut pledge_amounts: HashMap<NonFungibleId, Decimal> = HashMap::new();

            // Pull out all the pledged capital so we can return it to the caller
            let mut principal = Bucket::new(request_data.request_token);

            for (lender_id, lender_vault) in pledge_map {
                if lender_vault.amount() != Decimal::zero() {
                    pledge_amounts.insert(lender_id.clone(), lender_vault.amount());
                    principal.put(lender_vault.take_all());
                }
            }

            // create the Loan NFT
            let (loan_nft, loan_nfid) =
                borrow_component!(self.loan_acceptor.unwrap()).call::<(Bucket, NonFungibleId)>(
                    "create_loan",
                    args!(
                        self.admin_badge.create_proof(),
                        request_data.request_amount,
                        request_data.request_token,
                        request_data.promise_payment_intervals,
                        request_data.promise_installments,
                        request_data.promise_amount_per_installment,
                        request_data.loan_purpose_summary,
                        request_data.loan_purpose_url,
                        borrower_id,
                        pledge_amounts
                    ));

            // burn the LoanRequest, it has been replaced by the Loan
            self.admin_badge.authorize(||  {
                request.burn();
            });

            (principal, loan_nft, loan_nfid)
        }

        /// This burns a LoanRequest NFT.
        ///
        /// A LoanRequest that has been cancelled and has no remaining
        /// pledges towards it can be burnt by passing it to this
        /// method.
        ///
        /// The method panics if those preconditions do not hold.
        ///
        /// ---
        ///
        /// **Access control:** The borrower proof must be same as the
        /// request owner.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/burn.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/burn.rtm")]
        /// ```
        pub fn burn(&mut self, borrower: Proof, request: Bucket) {
            let (borrower_id, _, _) = self.check_and_retrieve_participant(borrower);
            let (reqid, _, request_data) = self.check_and_retrieve_request_from_bucket(&request);
            assert_eq!(request_data.borrower_id, borrower_id,
                       "This is not your loan request");
            assert!(request_data.cancelled,
                    "The request must be cancelled before you can burn it");

            let pledge_map = self.principals.get_mut(&reqid).unwrap();
            assert!(LoanRequestor::calc_total_pledge(&pledge_map) == Decimal::zero(),
                    "The request cannot be burned while there are still \
                     pledged funds associated with it");

            self.admin_badge.authorize(||  {
                request.burn();
            });
        }
        
        /// Retrieves purpose data of the LoanRequest NFT.
        ///
        /// Returns a tuple with the following data (in order):
        ///
        /// 0. Loan purpose summary
        /// 1. Loan purpose URL
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/read_loan_purpose.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/read_loan_purpose.rtm")]
        /// ```
        pub fn read_loan_purpose(&self, request_id: NonFungibleId) -> (String, String) {
            let (_, _, data) =
                self.retrieve_request_from_id(request_id);
            (data.loan_purpose_summary.to_string(), data.loan_purpose_url.to_string())
        }

        /// Checks if a loan request has been cancelled.
        ///
        /// Returns true if the request is cancelled. A request that
        /// has been cancelled will never become not cancelled again.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/read_is_cancelled.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/read_is_cancelled.rtm")]
        /// ```
        pub fn read_is_cancelled(&self, request_id: NonFungibleId) -> bool {
            let (_, _, data) =
                self.retrieve_request_from_id(request_id);
            data.cancelled
        }
        
        /// Retrieve the Participant id of the borrower behind the
        /// request.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/read_borrower_id.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/read_borrower_id.rtm")]
        /// ```
        pub fn read_borrower_id(&self, request_id: NonFungibleId) -> NonFungibleId {
            let (_, _, data) =
                self.retrieve_request_from_id(request_id);
            data.borrower_id
        }

        /// Retrieves data from a requests's NFT.
        ///
        /// This reads the bulk of a request's internal state, with
        /// the following data elements missing because of limitations
        /// in the Scrypto language (only up to 10-element tuples
        /// appear to be supported):
        ///
        /// - borrower_id
        /// - cancelled
        /// - loan_purpose_summary
        /// - loan_purpose_url
        ///
        /// Those have their own read methods that you can use.
        ///
        /// The return tuple contains return values in the following order:
        ///
        /// 0. request_token
        /// 1. request_amount
        /// 2. minimum_share
        /// 3. request_start_epoch
        /// 4. pledge_lock_epochs
        /// 5. loan_filled_epoch
        /// 6. loan_filled_lock_epochs
        /// 7. promise_payment_intervals
        /// 8. promise_installments
        /// 9. promise_amount_per_installment
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/read_data.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/read_data.rtm")]
        /// ```
        pub fn read_data(&self, request_id: NonFungibleId)
                         -> (ResourceAddress, Decimal, Decimal, u64, u64, Option<u64>,
                             u64, u64, u64, Decimal)
        {
            let (_, _, data) =
                self.retrieve_request_from_id(request_id);
            (data.request_token,
             data.request_amount,
             data.minimum_share,
             data.request_start_epoch,
             data.pledge_lock_epochs,
             data.loan_filled_epoch,
             data.loan_filled_lock_epochs,
             data.promise_payment_intervals,
             data.promise_installments,
             data.promise_amount_per_installment)
        }

        /// Retrieve LoanRequest NFT resource address for this
        /// LoanRequestor instance.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/read_request_nft_addr.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/read_request_nft_addr.rtm")]
        /// ```
        pub fn read_request_nft_addr(&self) -> ResourceAddress {
            self.nft_address
        }

        /// Retrieve Participants NFT resource address for this
        /// LoanRequestor instance.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/read_participants_nft_addr.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/read_participants_nft_addr.rtm")]
        /// ```
        pub fn read_participants_nft_addr(&self) -> ResourceAddress {
            self.participants_nft_address
        }

        /// Retrieve the component address of our companion
        /// LoanAcceptor instance.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanrequestor/read_loan_acceptor.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanrequestor/read_loan_acceptor.rtm")]
        /// ```
        pub fn read_loan_acceptor(&self) -> Option<ComponentAddress> {
            self.loan_acceptor
        }

        
        //
        // Internal methods follow
        //

        /// Totals up the pledges towards a loan and returns the sum
        fn calc_total_pledge(pledge_map: &HashMap<NonFungibleId, Vault>) -> Decimal {
            let mut total = Decimal::zero();
            for vault in pledge_map.values() {
                total += vault.amount();
            }
            total
        }

        /// Produces a resource manager and LoanRequest NFT data from
        /// a LoanRequest id; also returns the id itself.
        fn retrieve_request_from_id(&self, non_fungible_id: NonFungibleId) 
                                    -> (NonFungibleId, &ResourceManager, LoanRequest) {
            let nft_manager = borrow_resource_manager!(self.nft_address);
            let data = nft_manager.get_non_fungible_data(&non_fungible_id);
            (non_fungible_id, nft_manager, data)
        }

        /// Produces an, id, resource manager and LoanRequest NFT data
        /// from a LoanRequest in a bucket
        fn check_and_retrieve_request_from_bucket(&self, nft: &Bucket)
                                      -> (NonFungibleId, &ResourceManager, LoanRequest) { 
           assert_eq!(
                nft.resource_address(),
                self.nft_address,
                "Unsupported loan request NFT"
            );
            assert_eq!(nft.amount(), dec!("1"),
                       "Use only one loan request NFT at a time");
            let non_fungible_id = nft
                .non_fungible_ids()
                .into_iter()
                .collect::<Vec<NonFungibleId>>()[0]
                .clone();
            self.retrieve_request_from_id(non_fungible_id)
        }

        /// Asserts that the Proof is for a LoanRequest NFT of our
        /// series and returns useful objects for working with it.
        fn check_and_retrieve_request(&self, nft: Proof)
                                      -> (NonFungibleId, &ResourceManager, LoanRequest) { 
           assert_eq!(
                nft.resource_address(),
                self.nft_address,
                "Unsupported loan request NFT"
            );
            assert_eq!(nft.amount(), dec!("1"),
                       "Use only one loan request NFT at a time");
            let non_fungible_id = nft
                .non_fungible_ids()
                .into_iter()
                .collect::<Vec<NonFungibleId>>()[0]
                .clone();
            self.retrieve_request_from_id(non_fungible_id)
        }

        /// Asserts that the Proof is for a Participant NFT of the
        /// catalog we're connected to, and returns useful objects for
        /// working with it.
        fn check_and_retrieve_participant(&self, nft: Proof)
                                      -> (NonFungibleId, &ResourceManager, Participant) { 
           assert_eq!(
                nft.resource_address(),
                self.participants_nft_address,
                "Unsupported participant NFT"
            );
            assert_eq!(nft.amount(), dec!("1"),
                       "Use only one participant NFT at a time");
            let nfid = nft
                .non_fungible_ids()
                .into_iter()
                .collect::<Vec<NonFungibleId>>()[0]
                .clone();
            let nft_manager = borrow_resource_manager!(self.participants_nft_address);
            let data = nft_manager.get_non_fungible_data(&nfid);
            (nfid, nft_manager, data)
        }

        /// Writes the mutable part of the LoanRequest NFT data to the
        /// ledger.
        fn save_request_data(&self, non_fungible_id: &NonFungibleId, data: LoanRequest) {
            self.admin_badge.authorize(||  {
                borrow_resource_manager!(self.nft_address)
                    .update_non_fungible_data(&non_fungible_id, data);
            });
        }
    }
}
