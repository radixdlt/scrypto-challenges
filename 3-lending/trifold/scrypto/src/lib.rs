use scrypto::{prelude::*};


/// A representation of a approved borrower, with the current loan.
/// If there is no loan, then current_loan is 0 and loan_start_epoch is 0.
#[derive(NonFungibleData)]
pub struct Borrower {
    /// The borrower's friendly name
    pub name: String,
    /// The borrower's website
    pub website: String,
    /// The borrower's cryptocurrency address
    pub address: ComponentAddress,
    #[scrypto(mutable)]
    /// The borrower's current loan amount
    pub current_loan: Decimal,
    /// The epoch at which the borrower's loan started
    #[scrypto(mutable)]
    pub loan_start_epoch: u64,
}

#[derive(NonFungibleData)]
pub struct LoanRequestBadge {
    pub request_start_epoch: u64,
}

#[derive(NonFungibleData)]
pub struct LenderBadge {
    pub deposit_amount: Decimal
}


blueprint! {
    /// A system that manages the borrowers and their loans.
    struct Trifold {
        /// The main vault where liquidity is stored.
        main_vault: Vault,
        /// A stable vault which stores the amount of token deposited to calculate the amount to cash out
        stable_vault: Vault,
        /// the emergency lockdown vault
        emergency_vault: Vault,
        /// The badge that approves all protected actions
        internal_badge: Vault,
        /// The admin badge to approve borrowers
        admin_badge: ResourceAddress,
        /// the badge nft that allows borrowers to request a loan
        approved_borrower_badge: ResourceAddress,
        /// a nft that approves a loan
        approved_loan_badge: ResourceAddress,
        /// The virtual token that represents the loan amount
        virtual_token: ResourceAddress,
        /// The lockdown token
        lockdown_token: ResourceAddress,
        /// The virtual token that represents the size of the loan that borrowers can request
        karma_token: ResourceAddress,
        /// The interest rate per epoch of a loan
        interest_rate: Decimal,
        /// The period of time to vote on a loan request
        borrower_approval_request_vote_period: u64,

    }

    impl Trifold {
        /// Create a new trifold system.
        pub fn instantiate() -> (ComponentAddress, Bucket) {
            let internal_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);

            let admin_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Admin Badge")
                .mintable(rule!(require(internal_badge.resource_address())), MUTABLE(rule!(require(internal_badge.resource_address()))))
                .burnable(rule!(require(internal_badge.resource_address())), MUTABLE(rule!(require(internal_badge.resource_address()))))
                .divisibility(DIVISIBILITY_NONE)
                .no_initial_supply();



            let virtual_token: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "Loaned Radix")
                .metadata("symbol", "lnXRD")
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .no_initial_supply();
            
            let karma_token: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "Lending Karma")
                .metadata("symbol", "KARMA")
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let lockdown_token: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "Lockdown Token")
                .metadata("symbol", "LDT")
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let approved_borrower_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "Approved Borrower Badge")
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(internal_badge.resource_address())),
                    LOCKED,
                )
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();
            
            let approved_loan_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "Approved Loan Badge")
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(internal_badge.resource_address())),
                    LOCKED,
                )
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();
            
            let initial_admin_badge = internal_badge.authorize(|| -> Bucket {
                let badge = borrow_resource_manager!(admin_badge).mint(1);
                badge
            });

            let auth =  AccessRules::new()
                .method("approve_borrower", rule!(require(admin_badge)))
                .default(rule!(allow_all));

            (Self {
                main_vault: Vault::new(RADIX_TOKEN),
                stable_vault: Vault::new(virtual_token),
                emergency_vault: Vault::new(virtual_token),
                internal_badge: Vault::with_bucket(internal_badge),
                admin_badge: admin_badge,
                virtual_token,
                karma_token,
                lockdown_token,
                approved_borrower_badge,
                approved_loan_badge,
                interest_rate: dec!("0.1"),
                borrower_approval_request_vote_period: 10,
            }
            .instantiate()
            .add_access_check(auth)
            .globalize(), initial_admin_badge)
        }

        /// Deposit some amount of RADIX into the main vault.
        /// This will increase the supply of the main vault.
        /// This will also increase the supply of the stable vault.
        /// 
        /// # Arguments
        /// * `xrd` - The RADIX to deposit.
        /// 
        /// # Returns
        /// The lnXRD to redeem for the deposit.
        pub fn deposit(&mut self, xrd: Bucket) -> Bucket {
            assert!(self.is_in_safe_mode(), "The system is under lockdown");
            assert_eq!(xrd.resource_address(), RADIX_TOKEN, "The tokens must be XRD");
            
            let amount_to_mint = xrd.amount();
            
            self.main_vault.put(xrd);

            let lnxrd: Bucket = self.internal_badge.authorize(|| -> Bucket {
                // mint tokens to give to the lender
                let token: Bucket = borrow_resource_manager!(self.virtual_token).mint(amount_to_mint);
                // mint an identitical amount of tokens to store as log.
                self.stable_vault.put(borrow_resource_manager!(self.virtual_token).mint(amount_to_mint));
                token
            });




            lnxrd
        }

        /// Withdraw depositted RADIX from the main vault along with any profits.
        /// This will decrease the supply of the main vault.
        /// 
        /// # Arguments
        /// * `lnxrd` - The lnXRD to redeem for profits.
        /// 
        /// # Returns
        /// The withdrawn RADIX.
        pub fn withdraw(&mut self, lnxrd: Bucket) -> Bucket {
            assert!(self.is_in_safe_mode(), "The system is under lockdown");
            assert!(lnxrd.resource_address() == self.virtual_token, "The tokens must be lnXRD");
            
            let amount= lnxrd.amount();

            // propertionally determine how much xrd to reward, considering that interest has been earned.
            let amount_of_stable_token = self.stable_vault.amount();
            let amount_of_xrd = self.main_vault.amount();

            let amount_of_xrd_to_withdraw = amount_of_xrd / amount_of_stable_token * amount;

            assert!(amount_of_xrd_to_withdraw <= amount_of_xrd, "Not enough liquidity to withdraw right now, check back later.");
            
            self.internal_badge.authorize(|| {
                lnxrd.burn();
                self.stable_vault.take(amount).burn();
            });
            
            self.main_vault.take(amount_of_xrd_to_withdraw)
        }


        /// Take a loan from the pool
        //&/ You need to have a borrower badge to request a loan.
        /// 
        /// # Arguments
        /// * `karma` - The amount of karma to use to withdraw token.
        /// * `borrower_badge` - A proof of your borrower badge.
        /// 
        /// # Returns
        /// The loan and any remaining karma
        pub fn borrow(&mut self, karma: Bucket, borrower_badge: Proof) -> Bucket {
            assert!(self.is_in_safe_mode(), "The system is under lockdown");
            assert!(karma.resource_address() == self.karma_token, "The tokens must be KARMA");

            let amount = karma.amount();

            assert!(amount > dec!(0), "The amount to borrow must be greater than 0");
            // make sure the amount to borrow is not greater than the amount in the vault
            assert!(amount <= self.main_vault.amount(), "The amount to borrow must be less than or equal to the amount in the vault");
            // make sure the borrower badge is valid
            assert_eq!(borrower_badge.resource_address(), self.approved_borrower_badge, "The borrower badge is not valid");

            // make sure the borrower badge does not already have a loan
            let mut borrower_badge_data: Borrower = borrower_badge.non_fungible().data();

            assert_eq!(borrower_badge_data.current_loan, dec!(0), "The borrower badge already has a loan");
            assert_eq!(borrower_badge_data.loan_start_epoch, 0, "The borrower badge already has a loan");


            self.internal_badge.authorize(|| {
                karma.burn();
                borrower_badge_data.current_loan = amount;
                borrower_badge_data.loan_start_epoch = Runtime::current_epoch();
                borrower_badge.non_fungible().update_data(borrower_badge_data);
            });



            self.main_vault.take(amount)
        }

        /// Repay a loan to the pool
        /// You need to have a borrower badge to repay a loan.
        /// 
        /// # Arguments
        /// * `xrd` - The XRD to repay.
        /// * `borrower_badge` - A proof of your borrower badge.
        /// 
        /// # Returns
        /// Any karma earned and any leftover XRD.
        pub fn repay(&mut self, mut xrd: Bucket, borrower_badge: Proof) -> (Bucket, Bucket) {
            assert!(self.is_in_safe_mode(), "The system is under lockdown");
            assert!(xrd.resource_address() == RADIX_TOKEN, "The tokens must be XRD");
            // make sure the borrower badge is valid
            assert_eq!(borrower_badge.resource_address(), self.approved_borrower_badge, "The borrower badge is not valid");
            // make sure there is only one borrower badge
            assert_eq!{borrower_badge.amount(), Decimal::one(), "There must be only one borrower badge"};
            // make sure the borrower badge does not already have a loan
            let mut borrower_badge_data: Borrower = borrower_badge.non_fungible().data();
            assert_ne!(borrower_badge_data.current_loan, dec!(0), "The borrower badge doesn't have a loan");
            assert_ne!(borrower_badge_data.loan_start_epoch, 0, "The borrower badge doesn't have a loan");

            let mut amount_owed = borrower_badge_data.current_loan;
            
            for _ in borrower_badge_data.loan_start_epoch..Runtime::current_epoch() {
                amount_owed *= dec!(1) + self.interest_rate;
            }

            let amount_of_xrd = xrd.amount();

            if amount_of_xrd >= amount_owed {
                self.main_vault.put(xrd.take(amount_owed));
                self.internal_badge.authorize(|| {
                    borrower_badge_data.current_loan = dec!(0);
                    borrower_badge_data.loan_start_epoch = 0;
                    borrower_badge.non_fungible().update_data(borrower_badge_data);
                });
            } else {
                self.main_vault.put(xrd.take(amount_of_xrd));
                self.internal_badge.authorize(|| {
                    borrower_badge_data.current_loan = amount_owed - amount_of_xrd;
                    borrower_badge_data.loan_start_epoch = Runtime::current_epoch();
                    borrower_badge.non_fungible().update_data(borrower_badge_data);
                });
            }

            let karma_to_reward = amount_of_xrd;

            let karma = self.internal_badge.authorize(|| {
                let karma: Bucket = borrow_resource_manager!(self.karma_token).mint(karma_to_reward);
                karma
            });

            (karma, xrd)
        }
        

        
        pub fn approve_borrower(&mut self, borrower: ComponentAddress, name: String, website: String) -> Bucket {
            assert!(self.is_in_safe_mode(), "The system is under lockdown");
            self.internal_badge.authorize(|| {
                borrow_resource_manager!(self.approved_borrower_badge).mint_non_fungible(
                    &NonFungibleId::random(),
                    Borrower {
                        name,
                        website,
                        address: borrower,
                        current_loan: dec!(0),
                        loan_start_epoch: 0,
                    }
                )
            })
        }


        pub fn get_info(&self) -> String {
            let info = json::object!{
                virtual_token: self.virtual_token.to_string(),
                karma_token: self.karma_token.to_string(),
                approved_borrower_badge: self.approved_borrower_badge.to_string(),
                admin_badge: self.admin_badge.to_string(),
                lockdown_token: self.lockdown_token.to_string(),
            };

            info.dump()
        }

        pub fn lockdown_vote(&mut self, vote: Bucket) -> Bucket {
            assert!(vote.resource_address() == self.virtual_token, "The tokens must be lnXRD");
            let vote_amount = vote.amount();
            self.internal_badge.authorize(|| {
                self.emergency_vault.put(vote);
            });
            self.internal_badge.authorize(|| {
                borrow_resource_manager!(self.lockdown_token).mint(vote_amount)
            })
        }

        pub fn withdraw_lockdown(&mut self, vote: Bucket) -> Bucket {
            assert!(vote.resource_address() == self.lockdown_token, "The tokens must be lockdown tokens");
            let vote_amount = vote.amount();
            self.internal_badge.authorize(|| {
                vote.burn();
            });
            self.internal_badge.authorize(|| {
                self.emergency_vault.take(vote_amount)
            })
            
        }
        pub fn is_in_safe_mode(&self) -> bool {
            self.emergency_vault.amount() > (self.stable_vault.amount() / 2)
        }

    }
}