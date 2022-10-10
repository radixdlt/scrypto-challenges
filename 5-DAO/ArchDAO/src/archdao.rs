//! ArchDAO implements a crypto dao that serves for decentralized governance
//! of business proposal from the initial stage to the approval and the execution stage
//! 
//! Proposers receive fungible vote tokens in return for their
//! registering, with vote tokens representing shares of decision of the
//! whole proposal approval process. Right to vote can be transferred and whoever holds 
//! them can at any time vote for a propostal. 
//! 
//! Proposers can also fund the ArchDao with XRD tokens and receive back ARCH tokens, 
//! as the projects funded by ArchDAO makes profits, the value of
//! each ARCH token will increase.
//! 
//! The ArchDAO's managers can dynamically add and remove proposal
//! , or ask to verify if the proposal trigger is satisfied 
//! for promoting a proposal into a into a project ready to be funded.
//! 
//! The ArchDAO can charge protocol fees, intended at producing profits
//! for the ArchDAO's managers. Some or all of these protocol fees can go
//! towards rewards for both the managers and the voters.
//! 
//! Both the managers and the voters can earn rewards when the project gets executed and
//! when the execution produces a profit.
//!
//! 
//! # Overview of functions and methods
//!
//! To be done
//!
//! ## Instantiation
//!
//! [instantiate_archdao()][blueprint::ArchDAO::instantiate_archdao]
//! Creates a new ArchDAO instance.
//!
//!
//! TODO
//!
//! # Main Features
//!
//! TODO
//!
//! ## Proposals project
//!
//! TODO
//!
//!
//! ## Protocol Fees and DAO Fees
//!
//! ArchDAO allows you to set protocol fees when instantiating it. In
//! order to provide stability it is not possible
//! to change the protocol fees after creation. You can set one
//! fee for voting and another fee for removing votes; Also other different
//! fees are for proposal's approval.
//!
//!
//! ## Approval Cycles
//!
//! Every now and then the DAO will need to approve some of the 
//! proposal that have been voted. You can set an interval for
//! this to happen in, and if that many epochs have passed without
//! approvals taking place one will be run on the next `vote` or
//! `remove vote` call to the DAO.
//!
//! Admin can force approvals by calling
//! [force_approvals].
//!
//!
//! ## Minimum Deposit
//!
//! You can set a minimum and a maximu deposit for users registering sending XRD to get back voting tokens.
//!
//! ## Vote Tokens
//!
//! The DAO uses vote tokens for giving proposers the right to vote.
//! if you own vote tokens (getting from the admins or bought by yourself) then you are part of the DAO. 
//! These vote tokens let you vote for one or more available prosposals.
//!
//! Vote Tokens are fungible tokens and can be freely traded outside of
//! the fund. 
//!
//! ArchDAO automatically mints and burns vote tokens in response
//! to account registering and unregistering in the DAO. This is authorized
//! through a separate minting badge which is held internally to the
//! ArchDAO component.
//!
//! ## Proposal Project
//!
//! Our Proposal Project are the project that are voted
//!
//! Seen from the perspective of the ArchDAO component, proposal project are components
//! that you can put funds into after they are approved, and their executions will hopely
//! generate profits they give back to you.
//!
//! Once you have voted a set of proposals you think are perfect for 
//! your needs then, if you want, you can also decide how much fund them, otherwise different parties like 
//! venture capitalis will fund the project for its execution.
//!
//! When distributing votes to the different projects,
//! ArchDAO will then calculate their value using a convicting system of voting, 
//! this means that the more the vote remain in the project the more 
//! will be estimated (for at least 1000epoch), the same when you remove the vote, the removed
//! vote will continue to be calculated for at least the some amount of epochs.
//!
//! ### Trouble-shooting Proposal Project
//!
//! TODO
//! 
//! ### Remove a Proposal Project
//! 
//! TODO
//!

use scrypto::prelude::*;

// Here, we define the proposal data 
#[derive(TypeId, Encode, Decode, Describe,NonFungibleData, Clone)]
pub struct Proposal {
    proposal: String,
    trigger: u64,
    proposal_id: u128,    
    epoch_opened: u64, 
    epoch_approved: Option<u64>, 
    epoch_funded: Option<u64>, 
    epoch_closed: Option<u64>,
    votes: HashMap<ResourceAddress,Vote>
}

// Here, we define the proposal data 
#[derive(TypeId, Encode, Decode, Describe,NonFungibleData, Clone, Copy)]
pub struct Vote {
    epoch_opened: u64, 
    amount: Decimal
}


blueprint! {
    struct ArchDAO {    
        /// This is the token we're working on.
        archdao_token: ResourceAddress,
        /// Vote token to be given to voters
        vote_address: ResourceAddress,

        /// Admin badges are returned to whoever created us and are
        /// used for calling restricted methods.
        admin_badge_address: ResourceAddress,

        /// Our map of proposal opened for approval.
        proposal: HashMap<ComponentAddress, Proposal>,         
        ///this map contains votes for each component, for each account
        all_votes: HashMap<ComponentAddress, HashMap<ComponentAddress,Vote>>,  

        /// Available funds for starting the proposals.
        free_funds_for_proposals: Vault,

        /// Every this often we automatically run a full maintenance
        /// approval process, in response to a vote proposal
        // proposal_update_interval_epochs: u64,
        proposal_update_interval_epochs: u64,
        /// The last time we ran a full maintenance cycle, whether
        /// forced or automatic.
        last_update_epoch: u64,

        /// We don't accept deposits below this value.
        minimum_deposit: Decimal,
        /// The protocol fee to charge on deposits.
        deposit_fee_bps: Option<Decimal>,
        /// The protocol fee to charge on withdrawals.
        withdraw_fee_bps: Option<Decimal>,

        /// The badge we use to mint and burn our vote tokens. It only exists in this vault.
        vote_mint_badge: Vault,
        /// The badge we use to control the proposals. It only lives in this vault.
        /// (We can't use the `vote_mint_badge` for this since we don't
        /// want to give the proposal project the power to mint and
        /// burn vote tokens.)
        proposals_control_badge: Vault,

        /// Protocol fees collected.
        fees: Vault,

        /// OLD FIELDS
        /// We try to keep our `free_funds` at this level relative to
        /// the total funds we have under management.
        free_funds_target_percent: Decimal,        

        /// 
        /// These proposal project are temporarily taken out of
        /// use. They remain in `proposal` but get skipped in most
        /// of our logic.
        stopped_proposal: HashSet<ComponentAddress>,
    }

    impl ArchDAO {

        /// Creates a new ArchDAO, returning to you any admin badges
        /// that were created in the process.
        ///
        /// Will panic if it detects errors in the input parameters.
        ///
        /// Refer to [main module documentation][crate::archdao] for
        /// an overview of parameters not explained here and how they
        /// relate to each other.
        ///
        ///
        /// `ArchDAO vote tokens` is the token one that
        /// is visible to your voters since they will all have
        /// vote tokens.
        ///
        /// You can control the quantity of admin badges you will
        /// receive with `admin_badge_quantity`. You may want several
        /// if you have a large admin team
        ///
        /// ---
        ///
        /// **Access control:** Anyone can instantiate archdao.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/instantiate_archdao.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/instantiate_archdao.rtm")]
        /// ```
        pub fn instantiate_archdao(
            archdao_token: ResourceAddress,
            free_funds_target_percent: Decimal,
            proposal_update_interval_epochs: u64,
            minimum_deposit: Decimal,
            admin_badge_name: Option<String>,
            admin_badge_quantity: u64,
            vote_name: Option<String>,
            deposit_fee_bps: Option<Decimal>,
            withdraw_fee_bps: Option<Decimal>,
            vote_mint_badge_name: Option<String>,
            proposal_control_badge_name: Option<String>,
        ) -> (ComponentAddress, ResourceAddress, Bucket, ResourceAddress, Bucket)
        {
            // ArchDAO::assert_minimum_deposit //TODO

            // The admin_badge is used for controlling our approvals and parameters, 
            // and for triggering DAO management
            let admin_badges = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", admin_badge_name.unwrap_or(
                    "ArchDAO admin badge".to_string()))
                .initial_supply(admin_badge_quantity);
            let admin_res = admin_badges.resource_address();

            // This is kept in a bucket in self, for automatic minting
            // and burning of vote tokens.
            let vote_mint_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", vote_mint_badge_name.unwrap_or(
                    "ArchDAO vote token mint badge".to_string()))
                .initial_supply(1);

            // This is kept in a bucket in self, for manipulating our
            // proposal project.
            let proposal_control_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", proposal_control_badge_name.unwrap_or(
                    "ArchDAO proposal project control badge".to_string()))
                .initial_supply(1);

            // These token represent one's right to vote into the DAO
            let votes = ResourceBuilder::new_fungible()
                .mintable(rule!(require(vote_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(vote_mint_badge.resource_address())), LOCKED)
                .metadata("name", vote_name.unwrap_or(
                    "ArchDAO vote tokens".to_string()))
                .initial_supply(0);

            let mut archdao = 
                Self {
                    archdao_token,
                    vote_address: votes.resource_address(),
                    admin_badge_address: admin_badges.resource_address(),
                    proposal: HashMap::new(),
                    all_votes: HashMap::new(),
                    stopped_proposal: HashSet::new(),
                    free_funds_for_proposals: Vault::new(archdao_token),
                    free_funds_target_percent,
                    proposal_update_interval_epochs,
                    last_update_epoch: 0,
                    minimum_deposit,
                    deposit_fee_bps,
                    withdraw_fee_bps,
                    vote_mint_badge: Vault::with_bucket(vote_mint_badge),
                    proposals_control_badge: Vault::with_bucket(proposal_control_badge),
                    fees: Vault::new(archdao_token),
                }
            .instantiate();

            archdao.add_access_check(
                    AccessRules::new()
                    // In order to stay on the safe side we default to
                    // requiring the admin badge, and individually
                    // specify those methods that are either available
                    // to all or have custom access control. This way
                    // we won't accidentally leave sensitive methods
                    // open.
                        .default(rule!(require(admin_res)))
                        .method("deposit", rule!(allow_all))
                        .method("withdraw", rule!(allow_all))
                        .method("read_proposal_for_approval", rule!(allow_all))         
                        .method("read_proposal_control_badge_address", rule!(allow_all))     
                        .method("vote_proposal", rule!(allow_all))   
                        .method("list_proposal", rule!(allow_all))    
                        .method("register", rule!(allow_all))    
                        // .method("approve_proposal", rule!(allow_all))    
                    );
                    
                let archdaocopy = archdao.globalize();

            (
                archdaocopy,
                admin_badges.resource_address(),
                admin_badges,
                votes.resource_address(),
                votes,
            )
        }


        /// Deposits funds into ArchDAO, returning a number of
        /// vote tokens representing the votes you get
        ///
        /// Will panic if you try to deposit the wrong type of token,
        /// if you're depositing too few or too many tokens, and under various
        /// other error conditions.
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone sending us
        /// tokens of the correct type
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/deposit.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/deposit.rtm")]
        /// ```
        pub fn deposit(&mut self, funds: Bucket) -> Bucket  {
            assert!(funds.resource_address() == self.archdao_token,
                    "Wrong token type sent");
            assert!(funds.amount() >= self.minimum_deposit,
                    "Send at least the minimum {} token deposit", self.minimum_deposit);

            // self.charge_fees //TODO
            let cmgr: &ResourceManager = borrow_resource_manager!(self.vote_address);

            // We mint a number of new votes equal to the value of
            // the deposit. 
            let total = self.calc_total_funds();
            let mint_q = if total.is_zero()
            { funds.amount() } else { (cmgr.total_supply() / total ) * funds.amount()};

            let votes = self.vote_mint_badge.authorize(|| {
                borrow_resource_manager!(self.vote_address).mint(mint_q)
            });

            self.free_funds_for_proposals.put(funds);
            // self.maintain_proposals(false); //TODO

            votes
        }

        /// Unregistering from ArchDAO by depositing vote tokens. 
        /// The tokens will be burnt and funds returned.
        ///
        /// Will panic if insufficient free funds are available. 
        ///
        /// Will also panic if you try to deposit the wrong vote tokens
        /// type and under various error conditions.
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone sending us
        /// vote tokens of the correct type
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/withdraw.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/withdraw.rtm")]
        /// ```
        pub fn withdraw(&mut self, vote_tokens: Bucket) -> Bucket {
            assert!(vote_tokens.resource_address() == self.vote_address,
                    "Wrong vote token type");

            let cmgr: &ResourceManager = borrow_resource_manager!(self.vote_address);
            // We receive a number of tokens proportional to our
            // ownership% in the vote tokens.
            //
            // Note that if free_funds does not have sufficient tokens
            // then this call fails and the user needs to wait for
            // free_funds to refill, possibly making a smaller
            // withdrawal in the meantime.
            let bucket_out = self.free_funds_for_proposals.take(self.value_of(vote_tokens.amount(), cmgr));
            // self.charge_fees(self.withdraw_fee_bps, self.withdraw_fee_partner_bps,
            //                  &mut bucket_out, &partner);
            self.vote_mint_badge.authorize(||  {
                vote_tokens.burn();
            });

            // self.maintain_proposals(false); //TODO

            bucket_out
        }


        /// Registering on the platform sendi xrd bucket and getting back xrd tokens
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/register.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/register.rtm")]
        /// ```
        pub fn register(&mut self, xrd_bucket: Bucket) -> Bucket {
            info!("register START");  
            // mint vote
            let vote_bucket = self.vote_mint_badge.authorize(|| {
                borrow_resource_manager!(self.vote_address).mint(xrd_bucket.amount())
            });

            //deposit xrd token
            self.free_funds_for_proposals.put(xrd_bucket);
             
            vote_bucket
        }  

        

        /// Adds a proposal project to the DAO
        ///
        /// Will panic if we already have that exact proposal
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/add_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/add_proposal.rtm")]
        /// ```
        pub fn add_proposal(&mut self,
                                      proposal_project: ComponentAddress, proposition: String) {
            info!("add_proposal START"); 
            // create and save a record of the book being borrowed
            let proposal = Proposal { 
                proposal: proposition,
                trigger: 60,
                proposal_id: Runtime::generate_uuid(),    
                epoch_opened: Runtime::current_epoch(), 
                epoch_approved: None, 
                epoch_funded: None, 
                epoch_closed: None,
                votes: HashMap::new()
             };                           
             
            assert!(self.proposal.insert(proposal_project, proposal).is_none(),
                    "The requested proposal is already set up for approval");
        }        
         

        /// Vote for a proposal
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/vote_proposal_old.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/vote_proposal.rtm")]
        /// ```
        pub fn vote_proposal(&mut self,
                        proposal_project: ComponentAddress, vote_bucket: Bucket, account: ComponentAddress) {
            info!("vote_proposal START"); 
            let vote = Vote { 
                epoch_opened: Runtime::current_epoch(), 
                amount: vote_bucket.amount()
                };                           
            
            let mut inner_votes: HashMap<ComponentAddress, Vote> = HashMap::new();
            inner_votes.insert(account,vote);

            //TODO controllare se ci sono già voti per il progetti
            //ed in tal caso aggiungere i voti alla mappa interna
            //altrimenti vengono persi i voti precedenti
            if self.all_votes.get(&proposal_project).is_none() {
                self.all_votes.insert(proposal_project, inner_votes);
            } else {
                let mappa = self.all_votes.get_mut(&proposal_project).unwrap();
                //ci sono già voti sul progetto
                mappa.insert(account, vote);//allora inserisco altri voti provenienti dallo stesso account
                // all_votes.insert(proposal_project, mappa);
            }

            //put the bucket received in the proposal component vault
            // self.iv_control_badge.authorize(
            //     ||
                    borrow_component!(proposal_project).call::<Option<Bucket>>(
                        "add_votes",
                        args!(vote_bucket));
        }              



        /// List all the available proposals
        ///
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/vote_proposal_old.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/list_proposal.rtm")]
        /// ``` 
        pub fn list_proposal(&mut self) -> HashMap<ComponentAddress,Decimal> {
            info!("list_proposal START"); 
            let mut totals: HashMap<ComponentAddress,Decimal> = HashMap::new();
            info!("Start list_proposal: "); 
            
            for (component_address, all_vote) in self.all_votes.iter() {
                let mut tot: Decimal = Decimal::zero();
                for (account_address, &vote) in all_vote.iter() {
                    info!("Value of proposal/account {}: {}: {}", component_address,account_address, self.sum(&vote)); 
                    tot = tot + self.sum(&vote);
                    totals.insert(*component_address, tot);
                }
                info!("Total Value: {}", tot); 
            }           
            totals
        }      

        /// Approve proposal
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/approve_proposal.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/approve_proposal.rtm")]
        /// ``` 
        pub fn approve_proposal(&mut self)  {
            info!("approve_proposal START"); 

            assert!(
                    self.free_funds_for_proposals.amount()>dec!(100),
                    "Minimum level for starting approvals is 100, please fund the dao!"
                );

            let totals: HashMap<ComponentAddress,Decimal> = self.list_proposal();

            for (proposal_address, &amount) in totals.iter() {
                info!("Amount of votes to send to proposal {}: {}", proposal_address, amount); 

                // let resource_manager: &ResourceManager = borrow_resource_manager!(self.archdao_token);
                //TODO this is how to mint the vote token
                // let bucket: Bucket = self.vote_mint_badge.authorize(|| resource_manager.mint(amount));

                // ERROR, I don't want to fund the projects now
                // now I want to see votes accumulating in the proposals
                let bucket: Bucket = self.free_funds_for_proposals.take(amount);
                info!("Ready to send {} {} token to {}", amount, self.free_funds_for_proposals.resource_address(),  proposal_address); 
                self.fund_proposal(proposal_address,bucket);
            }
        }    

        /// Puts in the main vault the fundings for the approved projects 
        pub fn fund_approved_projects(&mut self,  bucket: Bucket)  {
            self.free_funds_for_proposals.put(bucket);
        }
        
        /// Puts in the project internal vault the fundings for the its execution
        fn fund_proposal(&self, project: &ComponentAddress, bucket: Bucket)  {
            // borrow_component!(*project).call::<Decimal>(
            //     "add_funds",
            //     args!(bucket));
            self.proposals_control_badge.authorize(
                ||
                    borrow_component!(*project).call::<Option<Bucket>>(
                        "add_funds",
                        args!(bucket)));                
        }
    

        /// Calculate the total value of the vote, checking for how long the vote has been given
        fn sum(&self, vote: &Vote) -> Decimal {
            info!("Start sum vote: "); 
            let mut tot = Decimal::zero();
            let diff: Decimal = Decimal::from(Runtime::current_epoch()-vote.epoch_opened); 
            if diff < dec!("100") {
                tot = (diff/dec!("100"))*vote.amount; 
            }
            if diff >= dec!("100") {
                tot = vote.amount; 
            }            
            Decimal::from(tot)
        }


        /// Utility functions
        /// 
        /// 
        /// Withdraws any accrued protocol fees.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/withdraw_protocol_fees.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/withdraw_protocol_fees.rtm")]
        /// ```
        pub fn withdraw_protocol_fees(&mut self) -> Bucket {
            self.fees.take_all()
        }

        /// Calculates the current value of some number of vote tokens.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/value_of_vote_tokens.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/value_of_vote_tokens.rtm")]
        /// ```
        pub fn value_of_vote_tokens(&self, amount: Decimal) -> Decimal {
            let cmgr: &ResourceManager = borrow_resource_manager!(self.vote_address);
            self.value_of(amount, cmgr)
        }


        /// Calculates how many vote tokens are currently in existence.
        /// Each vote token represents a right to vote for a proposal.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_total_votes.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_total_votes.rtm")]
        /// ```
        pub fn read_total_votes(&self) -> Decimal {
            let cmgr: &ResourceManager = borrow_resource_manager!(self.vote_address);
            cmgr.total_supply()
        }

        /// Returns our `archdao_token`.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_archdao_token.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_archdao_token.rtm")]
        /// ```
        pub fn read_archdao_token(&self) -> ResourceAddress {
            self.archdao_token
        }

        /// Returns the address of our ownership vote token.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_vote_tokens_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_vote_token_address.rtm")]
        /// ```
        pub fn read_vote_token_address(&self) -> ResourceAddress {
            self.vote_address
        }

        /// Returns the address of our admin badges.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_admin_badge_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_admin_badge_address.rtm")]
        /// ```
        pub fn read_admin_badge_address(&self) -> ResourceAddress {
            self.admin_badge_address
        }



        /// Returns how many epochs must pass after an approval process
        /// cycle takes place again.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_approvals_update_interval_epochs.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_approvals_update_interval_epochs.rtm")]
        /// ```
        pub fn read_approvals_update_interval_epochs(&self) -> u64 {
            self.proposal_update_interval_epochs
        }

        /// Returns the last epoch on which a an approval process
        /// was performed.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_last_update_epoch.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_last_update_epoch.rtm")]
        /// ```
        pub fn read_last_update_epoch(&self) -> u64 {
            self.last_update_epoch
        }

        /// Returns the minimum deposit we accept from proposers.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_minimum_deposit.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_minimum_deposit.rtm")]
        /// ```
        pub fn read_minimum_deposit(&self) -> Decimal {
            self.minimum_deposit
        }

        /// Returns the deposit fee
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_deposit_fee_bps.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_deposit_fee_bps.rtm")]
        /// ```
        pub fn read_deposit_fee_bps(&self) -> Option<Decimal> {
            self.deposit_fee_bps
        }

        /// Returns the withdraw fee
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_withdraw_fee_bps.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_withdraw_fee_bps.rtm")]
        /// ```
        pub fn read_withdraw_fee_bps(&self) -> Option<Decimal> {
            self.withdraw_fee_bps
        }


        /// Returns the address of the badge we use for minting and
        /// burning vote tokens. The only such badge in existence is held
        /// within the component itself.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_vote_mint_badge_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_vote_mint_badge_address.rtm")]
        /// ```
        pub fn read_vote_mint_badge_address(&self) -> ResourceAddress {
            self.vote_mint_badge.resource_address()
        }

         /// Returns the address of the badge we use for controlling
        /// our proposal project. The only such badge in existence
        /// is held within the component itself.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_proposal_control_badge_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_proposal_control_badge_address.rtm")]
        /// ```
        pub fn read_proposal_control_badge_address(&self) -> ResourceAddress {
            self.proposals_control_badge.resource_address()
        }        

        /// TODO
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/archdao/read_proposal_for_approval.rtm`
        /// ```text
        #[doc = include_str!("../rtm/archdao/read_proposal_for_approval.rtm")]
        /// ```
        pub fn read_proposal_for_approval(&self) -> HashMap<ComponentAddress, Proposal> {
            self.proposal.clone()
        }        

        /// Calculates the value of a given number of vote tokens.
        fn value_of(&self, amount: Decimal, manager: &ResourceManager) -> Decimal {
            let total = manager.total_supply();
            if total.is_zero() { return amount; }
            else { return amount * (self.calc_total_funds() / total); }
        }

        /// Calculates the total funds we have
        fn calc_total_funds(&self) -> Decimal {
            let total: Decimal = self.free_funds_for_proposals.amount();
            total
        }        


    }
}
