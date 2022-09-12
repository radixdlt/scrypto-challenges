//! Manages repayment of a loan.
//!
//! The LoanAcceptor component is used to manage a loan that has
//! already been paid out to the borrower. Its main function is to
//! receive periodic payments from the borrower and divide these up
//! between the lenders. It also has a system for taking a loan into
//! arrears when a payment is late, and for the lenders to be able to
//! get together and take the loan back out of arrears if/when they
//! feel that the situation has been settled.
//!
//! # Facilitators and fees
//!
//! If the lending system (by which we mean the collection of a
//! Participants catalog, a LoanRequestor and a LoanAcceptor) is being
//! managed by some central agency then they may have set themselves
//! up as facilitator of any resulting loans and may be receiving a
//! fee from each installment that is being paid. The fee is measured
//! in basis points, with one basis point being one hundredth of a
//! percent.
//!
//! # Repayment
//!
//! The borrower is expected to honour the obligations he set forth in
//! his LoanRequest, and pay the promised amounts at the promised
//! intervals. To do so he must call the pay_installment method
//! delivering it a full installment each time.
//!
//! Note that there is no limitation on who exactly can pay an
//! installment on the loan. In micro finance it is not unusual for
//! groups of borrowers to get together and form a sort of informal
//! insurance group, with all the members agreeing to cover for
//! eachother in case one of them hits upon hard times. The existence
//! of such a group of course makes it easier for them to attract
//! lenders. This blueprint caters to that dynamic primarily by
//! allowing anyone anywhere to be repaying people's loans.
//!
//! The payment is then divvied up between the facilitator (if any)
//! and all the lenders in this loan. Each lender will receive a
//! portion of the installment (after the facilitator takes his cut)
//! proportional to how much he pledged into the loan in the first
//! place.
//!
//! In the event that there is some rounding artifact in the process
//! (due to the max precision of the Decimal type) one of the lenders
//! may receive slightly more or slightly less than might otherwise be
//! expected but it is unlikely that this will produce a noticable
//! deviation in any direction.
//!
//! # Claiming rewards
//!
//! The facilitator may call the claim_facilitator_rewards method to
//! claim his rewards.
//!
//! A lender may call the claim_lender_rewards method to claim his
//! rewards.
//!
//! # Late payments and loans in arrears
//!
//! When a loan has a late payment it is said to be in technical
//! arrears. When a loan has had its internal arrears flag set it is
//! said to be in formal arrears. You could exit technical arrears by
//! making payment, but doing so will set the formal arrears flag
//! which is harder to get rid of. Technical arrears is formalized by
//! either calling pay_installment or a lender calling update_arrears.
//!
//! So once a loan has entered into technical arrears it will not exit
//! out of arrears again even if the late payment is then
//! made. Instead the borrower must enter into negotiations with his
//! lenders to convince them to manually clear the formal arrears
//! state. This negotiation itself is a social process and outside the
//! scope of this blueprint. Once the lenders are happy with the
//! situation however, if they all call the approve_clear_arrears
//! method this will remove the formal arrears state from the loan.
//!
//! Note that if after clearing the formal arrears flag the loan is
//! still in technical arrears (i.e. the current installment is
//! overdue), it will very soon find itself in formal arrears
//! again. It therefore is not very useful to negotiate arrears
//! clearing with your lenders until you've submitted all your late
//! payments.
//!
//! A lender who wants to withdraw his approval may call the
//! disapprove_clear_arrears method in order to do so.
//!
//! A lender who observes that the loan is technically in arrears but
//! has not had its internal flag set yet may call the update_arrears
//! method to have it set. (Note that this method checks to see if the
//! loan is technically in arrears before changing its state so a
//! lender cannot use this to set formal arrears on a loan that isn't
//! in fact in arrears.)
//!
//! It is expected that in a micro finance situation your reputation
//! is everything and to have one of your loans be in arrears is a
//! serious black mark against you as a borrower. This provides
//! compelling incentive for a borrower in arrears to work out a deal
//! with the lenders, using these mechanisms to assist in those
//! proceedings.
//!
//! ---
//! 
//! **Front-end design advice:** In addition to Participant-based
//! sponsorships and endorsements, showing your user that a
//! prospective borrower has loans that remain in arrears is critical
//! to informing him of the risk of his investments. This information
//! should be displayed to him as a priority.
//!
//! ---

use scrypto::prelude::*;

use crate::participants::Participant;

/// This is the NFT data for a Loan. It is used for managing the loan
/// after its principal has been paid out and repayments are expected
/// to start rolling in.
///
/// The Loan NFT will stay with the borrower forever as a record of
/// his borrowing history.
#[derive(NonFungibleData)]
struct Loan {
    /// If the loan is in formal arrears this is set to true. Note
    /// that a loan can be in technical arrears even if this has not
    /// been set to true.
    #[scrypto(mutable)]
    in_arrears: bool,

    /// When the loan is in format arrears, lender votes for taking it
    /// out of arrears are stored here.
    #[scrypto(mutable)]
    arrears_votes: HashSet<NonFungibleId>,

    /// The principal of the loan.
    loan_amount: Decimal,

    /// Borrower provided brief summary of the purpose of the loan
    loan_purpose_summary: String,

    /// Borrower provided link to more about the loan
    loan_purpose_url: String,

    /// The token the loan is in, e.g., RADIX_TOKEN (XRD)
    loan_token: ResourceAddress,

    /// When the loan started. The principal was paid to the lender at
    /// this time.
    loan_start_epoch: u64,

    /// The Participant id of the borrower.
    borrower_id: NonFungibleId,

    /// Overivew of the lenders to the loan and how much of the
    /// principal each contributed.
    lenders: HashMap<NonFungibleId, Decimal>,

    /// How many installments in total the borrower is expected to
    /// pay.
    installment_total_count: u64,

    /// How many of the installments are still to be paid.
    #[scrypto(mutable)]
    installments_remaining: u64,

    /// The number of epochs between each installment.
    epochs_per_installment: u64,

    /// The amount of tokens to pay on each installment.
    amount_per_installment: Decimal,
}


blueprint! {

    struct LoanAcceptor {
        /// We only interact with Participants from this catalog,
        /// others are refused access to our services
        participants_nft_addr: ResourceAddress,

        /// The LoanRequestor with this admin badge address has the
        /// exclusive right to create new loans for us to manage.
        requestor_admin_addr: ResourceAddress,

        /// If set, the facilitator is able to collect any facilitator
        /// fees that have accrued.
        facilitator: Option<NonFungibleId>,

        /// Fee, in basis points, taken from each installment paid to
        /// loans we manage.
        facilitator_fee: Decimal,

        /// The NFT ResourceAddress of our Loan NFTs.
        loan_nft_address: ResourceAddress,

        /// Admin badge used internally for managing our Loan NFTs.
        admin_badge: Vault,

        /// Vaults holding funds repaid to lenders. The outer map is
        /// by Participant id, and the inner map is by token type so
        /// that, in principle, you would look up
        /// lender_rewards\[Bob]\[XRD] to get to Bob's cumulated
        /// XRD-demoninated rewards.
        lender_rewards: HashMap<NonFungibleId, HashMap<ResourceAddress, Vault>>,

        /// Vaults holding funds taken as fees, with one vault per
        /// token type.
        facilitator_rewards: HashMap<ResourceAddress, Vault>,
    }

    impl LoanAcceptor {

        /// Creates a new LoanAcceptor instance.
        ///
        /// A LoanAcceptor must be tied to an already existing
        /// LoanRequestor (see [crate::loanrequestor]), and it must be
        /// associated with a Participants catalog (see
        /// [crate::participants]).
        ///
        /// The associated LoanRequestor is given the privilege to
        /// call [LoanAcceptor::create_loan] to make us create new
        /// Loan NFTs for tracking repayments. The requestor must
        /// present a Proof of its admin badge to do so, the address
        /// of which gets passed to this function.
        ///
        /// The access control of the methods herein is closely tied
        /// to the Participant id of the calling party, and usually a
        /// Proof must be provided that you are a valid
        /// Participant. We only allow Participants from the catalog
        /// associated with the participants_nft_addr passed to us
        /// here.
        ///
        /// You can optionally establish a facilitator for the
        /// acceptor, and a fee which will be taken from all
        /// installments paid to loans managed by us. If you do not
        /// provide a facilitator then there can also not be a
        /// facilitator fee.
        ///
        /// Two new resources are created by this function: An admin
        /// badge we hold on to for managing our NFTs, and a new NFT
        /// series for our Loan NFTs. Both receive default names
        /// unless you override those names by giving them in
        /// admin_badge_name and nft_resource_name.
        ///
        /// This function panics if the input data doesn't make sense
        /// to it.
        ///
        /// The return tuple contains, in order:
        ///
        /// 0. the address of the new LoanAcceptor instance
        ///
        /// 1. the resource address for the Loan NFT series that was
        /// created.
        ///
        /// ---
        ///
        /// **Access control:** The supplied NFT and admin addresses
        /// as well as the facilitator id dictate who gets to interact
        /// with the instance created.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/instantiate_loan_acceptor.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/instantiate_loan_acceptor.rtm")]
        /// ```
        pub fn instantiate_loan_acceptor(participants_nft_addr: ResourceAddress,
                                         requestor_admin_addr: ResourceAddress,
                                         facilitator: Option<NonFungibleId>,
                                         facilitator_fee: Decimal,
                                         admin_badge_name: Option<String>,
                                         nft_resource_name: Option<String>)
                                         -> (ComponentAddress, ResourceAddress)
        {
            if facilitator.is_some() {
                // Don't allow facilitator fee greater than 100%
                assert!(facilitator_fee <= dec!("10000"),
                        "Facilitator fee is too large: {}", facilitator_fee);
                // Negative fee is also very fishy
                assert!(facilitator_fee >= Decimal::zero(),
                        "Facilitator fee must be positive: {}", facilitator_fee);
            } else {
                assert!(facilitator_fee.is_zero(),
                        "Cannot have facilitator fee without a facilitator");
            }
            let badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", admin_badge_name.unwrap_or("Loan NFT control badge".to_string()))
                .initial_supply(1);
            let nft_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", nft_resource_name.unwrap_or("Loan NFT".to_string()))
                .mintable(rule!(require(badge.resource_address())), LOCKED)
                .burnable(rule!(require(badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(badge.resource_address())), LOCKED)
                .no_initial_supply();
            let acceptor = 
                Self {
                    participants_nft_addr,
                    requestor_admin_addr,
                    facilitator,
                    facilitator_fee,
                    loan_nft_address: nft_resource,
                    admin_badge: Vault::with_bucket(badge),
                    lender_rewards: HashMap::new(),
                    facilitator_rewards: HashMap::new(),
                }.instantiate().globalize();

            // All methods that require access control in this blueprint
            // handle this themselves through the Proof instances provided
            // to them.
            
            (acceptor, nft_resource)
        }

        /// Creates a new loan when called by our associated
        /// LoanRequestor.
        ///
        /// The data from the original LoanRequest is passed in here
        /// so it can be duplicated in the Loan NFT.
        ///
        /// We return a tuple containing the newly created Loan NFT
        /// and separately the id of that NFT.
        ///
        /// ---
        ///
        /// **Access control:** Requires that the requestor_admin
        /// proof corresponds to our LoanRequestor instance's admin
        /// badge.
        ///
        /// **Transaction manifest:** This method is only ever called
        /// by a different component. Since users are not meant to
        /// call it directly no transaction manifest is provided.
        pub fn create_loan(&mut self,
                           requestor_admin: Proof,
                           loan_amount: Decimal,
                           loan_token: ResourceAddress,
                           epochs_per_installment: u64,
                           installments: u64,
                           amount_per_installment: Decimal,
                           loan_purpose_summary: String,
                           loan_purpose_url: String,
                           borrower_id: NonFungibleId,
                           lenders: HashMap<NonFungibleId, Decimal>) -> (Bucket, NonFungibleId)
        {
            assert_eq!(self.requestor_admin_addr, requestor_admin.resource_address(),
                       "Unsupported Requestor instance");

            let loan_nfid: NonFungibleId = NonFungibleId::random();
            let loan_nft: Bucket = self.admin_badge.authorize(||
                borrow_resource_manager!(self.loan_nft_address)
                    .mint_non_fungible(
                        &loan_nfid, 
                        Loan {
                            lenders,
                            borrower_id,
                            in_arrears: false,
                            arrears_votes: HashSet::new(),
                            loan_amount,
                            loan_token,
                            loan_purpose_summary,
                            loan_purpose_url,
                            loan_start_epoch: Runtime::current_epoch(),
                            installment_total_count: installments,
                            installments_remaining: installments,
                            epochs_per_installment,
                            amount_per_installment,
                        }
                    )
            );
            if self.facilitator.is_some()
                && self.facilitator_fee > Decimal::zero()
                && !self.facilitator_rewards.contains_key(&loan_token)
            {
                self.facilitator_rewards.insert(loan_token,
                                                Vault::new(loan_token));
            }
            (loan_nft, loan_nfid)
        }

        /// Used to make a downpayment on a loan.
        ///
        /// When called with at least as many funds as are needed to
        /// pay a full installment on the loan, will distribute those
        /// funds to the vaults of the facilitator (if any) and
        /// lenders.
        ///
        /// If the payment comes late then the loan will be put in a
        /// formal state of arrears.
        ///
        /// This method will panic if all installments have already
        /// been paid, or if the payment is too low or of the wrong
        /// type.
        ///
        /// We return any change that remains after making the payment.
        ///
        /// ---
        ///
        /// **Access control:** Anyone can pay installments, not just
        /// the borrower, and they don't even need to present a
        /// Participant proof to do so.
        ///
        /// *pecunia non olet*
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/pay_installment.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/pay_installment.rtm")]
        /// ```
        pub fn pay_installment(&mut self, loan_nfid: NonFungibleId, mut payment: Bucket) -> Bucket {
            let (loan_nfid, _, mut loan_data) =
                self.retrieve_loan_from_id(loan_nfid);

            assert_ne!(0, loan_data.installments_remaining,
                       "All installments are already paid");
            assert_eq!(loan_data.loan_token, payment.resource_address(),
                       "Wrong token type");
            if self.check_enter_arrears(&loan_data) {
                if !loan_data.in_arrears {
                    // Old votes are no longer valid: this is a new arrears situation
                    loan_data.arrears_votes.clear();
                }
                loan_data.in_arrears = true;
                info!("This loan is in arrears")
            }

            // Only take as much as promised, the rest will be returned.
            // Also, this ensures we are paid enough.
            let mut installment = payment.take(loan_data.amount_per_installment);

            // Pay the facilitator
            if self.facilitator.is_some() && self.facilitator_fee != Decimal::zero() {
                let facilitator_fee = (self.facilitator_fee / dec!("10000")) // convert from bps
                    * loan_data.amount_per_installment;
                self.facilitator_rewards.get_mut(&loan_data.loan_token).unwrap()
                    .put(installment.take(facilitator_fee));
            }

            // Pay the lenders
            let total_pot = installment.amount();
            let mut countdown = loan_data.lenders.len();

            // Note there is special treatment of the final element in this loop:
            // That lender receives whatever the remainder is, to account for any
            // rounding artifacts.
            for (lender_nfid, pledge) in loan_data.lenders.iter_mut() {
                let lender_rewards_map =
                    self.lender_rewards.entry(lender_nfid.clone())
                    .or_insert(HashMap::new());
                if  !lender_rewards_map.contains_key(&loan_data.loan_token) {
                    lender_rewards_map.insert(loan_data.loan_token,
                                              Vault::new(loan_data.loan_token));
                }
                let lender_vault =
                    lender_rewards_map.get_mut(&loan_data.loan_token).unwrap();
                countdown -= 1;
                let mut current_reward: Option<Decimal> = None;
                if countdown != 0 {
                    // Give to each lender proportional to their pledge
                    current_reward = Some(total_pot * *pledge / loan_data.loan_amount);
                }
                lender_vault.put(if let Some(payout) = current_reward
                                 { installment.take(payout) } else
                                 { installment.take(installment.amount()) });
            }

            loan_data.installments_remaining -= 1;
            self.save_loan_data(&loan_nfid, loan_data);

            assert!(installment.is_empty());
            // It's already empty but we need to get rid of the
            // bucket.
            payment.put(installment);

            payment // return the change
        }

        /// A lender calls this to claim his accumulated rewards.
        ///
        /// All lender rewards owed to the lender, of all token types
        /// and from all loans under the management of this
        /// LoanAcceptor instance, are returned from the method.
        ///
        /// ---
        ///
        /// **Access control:** Only returns the funds of the lender
        /// represented by the supplied proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/claim_lender_rewards.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/claim_lender_rewards.rtm")]
        /// ```
        pub fn claim_lender_rewards(&mut self, lender: Proof) -> Vec<Bucket> {
            let (lender_nfid, _, _) =
                self.check_and_retrieve_participant(lender);

            let mut rewards: Vec<Bucket> = Vec::new();
            let lender_map = self.lender_rewards.get_mut(&lender_nfid).unwrap();
            for (_, reward) in lender_map {
                rewards.push(reward.take_all());
            }
            rewards
        }

        /// The facilitator calls this to claim his accumulated
        /// rewards.
        ///
        /// This method will panic if there is no facilitator.
        ///
        /// All facilitator rewards owed to the facilitator, of all
        /// token types and from all loans under the management of
        /// this LoanAcceptor instance, are returned from the method.
        ///
        /// ---
        ///
        /// **Access control:** The proof provided must be the
        /// facilitator of this LoanAcceptor instance.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/claim_facilitator_rewards.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/claim_facilitator_rewards.rtm")]
        /// ```
        // Access control: 
        pub fn claim_facilitator_rewards(
            &mut self, facilitator: Proof) -> Vec<Bucket>
        {
            let (facilitator_nfid, _, _) =
                self.check_and_retrieve_participant(facilitator);
            assert_eq!(self.facilitator.as_ref().unwrap(), &facilitator_nfid,
                       "You are not the facilitator");

            let mut buckets: Vec<Bucket> = Vec::new();
            for (_, vault) in self.facilitator_rewards.iter_mut() {
                buckets.push(vault.take_all());
            }
            buckets
        }

        /// Lenders use this method to vote for clearing formal
        /// arrears.
        ///
        /// Adds a vote towards clearing the arrears status of the
        /// loan. If this is the final needed vote the formal arrears
        /// status is cleared and all votes reset.
        ///
        /// Note that if the loan is still in technical arrears (that
        /// is, the current payment is late) it will tend to go back
        /// into formal arrears again before long.
        ///
        /// This method will panic if the loan does not have its
        /// formal arrears flag set.
        ///
        /// ---
        ///
        /// **Access control:** The lender proof must be one of the
        /// lenders in the named loan.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/approve_clear_arrears.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/approve_clear_arrears.rtm")]
        /// ```
        pub fn approve_clear_arrears(&mut self, lender: Proof, loan_nfid: NonFungibleId) {
            let (loan_nfid, _, mut loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            let (lender_nfid, _, _) =
                self.check_and_retrieve_participant(lender);
            assert!(loan_data.lenders.contains_key(&lender_nfid),
                    "You are not a lender to this loan");

            assert!(loan_data.in_arrears,
                    "This loan is not in arrears");
            
            loan_data.arrears_votes.insert(lender_nfid);
            if loan_data.arrears_votes.len() == loan_data.lenders.len() {
                loan_data.in_arrears = false;
                loan_data.arrears_votes.clear();
            }
            self.save_loan_data(&loan_nfid, loan_data);
        }

        /// Lenders use this method to withdraw a vote for clearing
        /// formal arrears.
        ///
        /// ---
        ///
        /// **Access control:** The lender proof must be one of the
        /// lenders in the named loan.
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/disapprove_clear_arrears.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/disapprove_clear_arrears.rtm")]
        /// ```
        pub fn disapprove_clear_arrears(&mut self, lender: Proof, loan_nfid: NonFungibleId) {
            let (loan_nfid, _, mut loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            let (lender_nfid, _, _) =
                self.check_and_retrieve_participant(lender);
            assert!(loan_data.lenders.contains_key(&lender_nfid),
                    "You are not a lender to this loan");

            loan_data.arrears_votes.remove(&lender_nfid);
            self.save_loan_data(&loan_nfid, loan_data);
        }

        /// Lenders use this to update the formal arrears status of a
        /// loan.
        ///        
        /// Causes the arrears status of the named loan to be
        /// evaluated and if found to be in technical arrears the
        /// formal arrears flag will be set on the loan.
        ///
        /// Note that under normal circumstances it should never be
        /// necessary to call this method: The
        /// [LoanAcceptor::is_in_arrears] method returns true whether
        /// there is formal or technical arrears, and it's not
        /// possible to take a loan which is in technical arrears out
        /// of arrears without automatically having it get put into
        /// formal arrears. (The latter is handled by
        /// [LoanAcceptor::pay_installment]). The only real effect of
        /// calling this method is to update the [Loan::in_arrears] field in
        /// the Loan NFT data structure. The method is nevertheless
        /// provided in case it proves useful to third-party loan
        /// tracking tools etc.
        ///
        /// ---
        ///
        /// **Access control:** The lender proof must be of a lender
        /// to the named loan
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/update_arrears.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/update_arrears.rtm")]
        /// ```
        pub fn update_arrears(&mut self, lender: Proof, loan_nfid: NonFungibleId) {
            let (loan_nfid, _, mut loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            let (lender_nfid, _, _) =
                self.check_and_retrieve_participant(lender);
            assert!(loan_data.lenders.contains_key(&lender_nfid),
                    "You are not a lender to this loan");

            if !loan_data.in_arrears && self.int_should_be_in_arrears(&loan_data) {
                // Old votes are no longer valid: this is a new arrears situation
                loan_data.arrears_votes.clear();
                loan_data.in_arrears = true;
                self.save_loan_data(&loan_nfid, loan_data);
            }
        }

        /// Checks if the loan is currently in arrears.
        ///
        /// Returns true if the loan is currently in formal or
        /// technical arrears.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/is_in_arrears.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/is_in_arrears.rtm")]
        /// ```
        pub fn is_in_arrears(&self, loan_nfid: NonFungibleId) -> bool {
            let (_, _, loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            self.int_should_be_in_arrears(&loan_data)
        }

        /// Retrieves some of the data of the Loan NFT.
        ///
        /// Returns a tuple with the following data (in order):
        ///
        /// 0. Loan amount (principal)
        /// 1. Loan token (e.g. XRD)
        /// 2. Start epoch of the loan
        /// 3. Total number of installments expected
        /// 4. Number of installments remaining
        /// 5. Number of epochs between each installment
        /// 6. Amount paid per installment
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/read_loan_data.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_loan_data.rtm")]
        /// ```
        // Access control: Read only, anyone can call this
        pub fn read_loan_data(&self, loan_nfid: NonFungibleId)
                              -> (Decimal, ResourceAddress, u64, u64, u64, u64, Decimal) {
            let (_, _, loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            (loan_data.loan_amount,
             loan_data.loan_token,
             loan_data.loan_start_epoch,
             loan_data.installment_total_count,
             loan_data.installments_remaining,
             loan_data.epochs_per_installment,
             loan_data.amount_per_installment)
        }

        /// Retrieves purpose data of the Loan NFT.
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
        /// `rtm/loanacceptor/read_loan_purpose.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_loan_purpose.rtm")]
        /// ```
        pub fn read_loan_purpose(&self, loan_nfid: NonFungibleId)
                              -> (String, String) {
            let (_, _, loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            (loan_data.loan_purpose_summary,
             loan_data.loan_purpose_url)
        }

        /// Reads the current state of arrears voting for a loan.
        ///
        /// Returns a set containing the Participant ids of all
        /// lenders who have voted to clear arrears. Any lender who is
        /// not represented here has not approved the action (yet).
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/read_loan_arrears_votes.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_loan_arrears_votes.rtm")]
        /// ```
        pub fn read_loan_arrears_votes(&self, loan_nfid: NonFungibleId)
                                       -> HashSet<NonFungibleId> {
            let (_, _, loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            loan_data.arrears_votes
        }
        
        /// Retrieves the Participant address of a loan's borrower.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/read_borrower.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_borrower.rtm")]
        /// ```
        pub fn read_borrower(&self, loan_nfid: NonFungibleId)
                                -> NonFungibleAddress {
            let (_, _, loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            NonFungibleAddress::new(self.participants_nft_addr, loan_data.borrower_id)
        }

        /// Retrieves the lender list of a loan.
        ///
        /// Returns a map where the all the lenders' Participant ids
        /// are keys and the value is how much that lender contributed
        /// to the principal.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/read_lenders.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_lenders.rtm")]
        /// ```
        pub fn read_lenders(&self, loan_nfid: NonFungibleId)
                            -> HashMap<NonFungibleId, Decimal> {
            let (_, _, loan_data) =
                self.retrieve_loan_from_id(loan_nfid);
            loan_data.lenders
        }
        
        /// Retrieves the resource address of our Participant
        /// catalog's NFTs
        ///
        /// This resource address is used extensively in this
        /// blueprint for access control.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/read_participants_nft_addr.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_participants_nft_addr.rtm")]
        /// ```
        pub fn read_participants_nft_addr(&self) -> ResourceAddress {
            self.participants_nft_addr
        }

        /// Retrieves the Participant id of the facilitator (if any)
        /// of this LoanAcceptor instance.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/read_facilitator.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_facilitator.rtm")]
        /// ```
        pub fn read_facilitator(&self) -> Option<NonFungibleAddress> {
            if let Some(nfid) = self.facilitator.as_ref()
            { Some(NonFungibleAddress::new(self.participants_nft_addr, nfid.clone())) } else { None }
        }

        /// Retrieves the facilitator fee of this LoanAcceptor
        /// instance.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/read_facilitator_fee.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_facilitator_fee.rtm")]
        /// ```
        pub fn read_facilitator_fee(&self) -> Decimal {
            self.facilitator_fee
        }

        /// Retrieves the resource address of the Loan NFTs used by
        /// this LoanAcceptor instance.
        ///
        /// ---
        ///
        /// **Access control:** Read only, anyone can call this
        ///
        /// **Transaction manifest:**
        /// `rtm/loanacceptor/read_loan_nft_addr.rtm`
        /// ```text
        #[doc = include_str!("../rtm/loanacceptor/read_loan_nft_addr.rtm")]
        /// ```
        pub fn read_loan_nft_addr(&self) -> ResourceAddress {
            self.loan_nft_address
        }

        //
        // Internal utility methods follow
        //

        /// Checks if this loan either is or should currently be in arrears.
        fn int_should_be_in_arrears(&self, loan_data: &Loan) -> bool {
            loan_data.in_arrears ||
                loan_data.installments_remaining > 0 &&
                self.check_enter_arrears(loan_data)
        }

        /// Checks if the current installment is overdue, in which case
        /// the loan ought to go into arrears. Does not take into account
        /// that the loan may already be paid in full.
        fn check_enter_arrears(&self, loan_data: &Loan) -> bool {
            loan_data.loan_start_epoch
                + ((loan_data.installment_total_count - loan_data.installments_remaining) + 1)
                * loan_data.epochs_per_installment
                < Runtime::current_epoch()
        }

        /// Produces a resource manager and participant from a
        /// Participant id; also returns the id itself.
        fn retrieve_participant_from_id(&self, nfid: NonFungibleId) 
                                        -> (NonFungibleId, &ResourceManager, Participant)
        {
            let nft_manager = borrow_resource_manager!(self.participants_nft_addr);
            let data = nft_manager.get_non_fungible_data(&nfid);
            (nfid, nft_manager, data)
        }

        /// Asserts that the Proof is for a Participant NFT of the
        /// catalog we're connected to, and returns useful objects for
        /// working with it.
        fn check_and_retrieve_participant(&self, nft: Proof)
                                          -> (NonFungibleId, &ResourceManager, Participant)
        { 
           assert_eq!(
                nft.resource_address(),
                self.participants_nft_addr,
                "Unsupported participant NFT"
            );
            assert_eq!(nft.amount(), dec!("1"),
                       "Use only one participant NFT at a time");
            let nfid = nft
                .non_fungible_ids()
                .into_iter()
                .collect::<Vec<NonFungibleId>>()[0]
                .clone();
            self.retrieve_participant_from_id(nfid)
        }

        /// Produces a resource manager and participant from a
        /// Loan id; also returns the id itself.
        fn retrieve_loan_from_id(&self, nfid: NonFungibleId) 
                                 -> (NonFungibleId, &ResourceManager, Loan)
        {
            let nft_manager = borrow_resource_manager!(self.loan_nft_address);
            let data = nft_manager.get_non_fungible_data(&nfid);
            (nfid, nft_manager, data)
        }

        /// Writes the mutable part of the Loan NFT data to the
        /// ledger.
        fn save_loan_data(&self, non_fungible_id: &NonFungibleId, data: Loan)
        {
            self.admin_badge.authorize(||  {
                borrow_resource_manager!(self.loan_nft_address)
                    .update_non_fungible_data(&non_fungible_id, data);
            });
        }

    }
}
