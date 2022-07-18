//! # [GroundCredit](./ground_credit/blueprint/struct.GroundCredit.html): Make a Credit Ground for your journey into Web 3!
//!
//! Ground Credit is the blueprint for any organization to help users build a credit Ground in Web3 Society by utilizing SBT characteristics. 
//! 
//! Ground Credit also help lending protocol operators to use the Credit Service, allow automated credit scoring and debt-tracking through credit user's data.
//!
//! ## Main Features:
//!
//! The blueprint is for web3 organizations to manage user's credit through making use of Soul Bound Tokens (SBTs). 
//!
//! The blueprint included installment type credit, allow [TrueFi](https://truefi.io/) level credit. 
//!
//! The blueprint also included two revolving credit types: "Monthly" and "Yearly", allow on-chain "consumer level" credit for borrowers.
//!
//! ## Protocol entities:
//!
//! 1. **Credit service operator**: Main manager of the protocol. Through the blueprint's method, *Credit service operator* is allowed to:
//! - Issue new Credit SBT for users (for user who wish to migrate his off-chain credit history). (Require off-chain process)
//! - Review installment credit request. (Require off-chain process)
//! - List, delist a lending protocol to use the Credit service. (Require off-chain process if the protocols weren't run by the same entity)
//! - Blacklist, whitelist credit users who have issue with the ID SBT (wrong income, trust score) or have a large loan default. (Require off-chain process)
//! - Change the credit degrade and restore rate when credit users have late (or on-time) repayment frequency.
//!
//! Service operator is also required to protect user's private data.
//!
//! 2. **Credit users**: Verified unique identity on web3 who wish to use on-chain credit or take a loan. Through the blueprint's method, *Credit users* are allowed to:
//! - Use the ID SBT to take new credit SBT.
//! - Change credit type ("Monthly" or "Yearly") (Require no-debt credit status).
//! - Check the maximum credit and current credit allowance.
//! - Request an installment credit.
//! - Take the installment credit badge after the request has passed.
//!
//! 3. **Lending protocols**: Listed lending protocols can use this blueprint for on-chain credit service. Through the blueprint's method, *Lending protocols* are allowed to:
//! - Automatically evaluate user's credit score through late (or on-time) repayment frequency. 
//! - Edit user's current debt or the credit's due time.
//! - Let protocol users use the installment credit badge to change credit into installment type (Require no-debt credit status).
//! - Let protocol users stop using installment credit and change the credit back into revolving type.

use scrypto::prelude::*;
use ground_id::*;

/// The SBT keep track of an user's credit data. 
/// 
/// ## Uses:
/// 
/// If the user has off-chain credit history, the data can be feeded on-chain through an off-chain process 
/// to aggregrate their new on-chain credit score. 
/// 
/// If the user has an on-chain unique identity and don't want to use the off-chain credit history, he can demand new credit SBT with the default data.
/// 
/// The user's credit data is used on-chain given a high possibility that user would be comfortable 
/// with publicing it as long as their other personal data (name, age, location,...) is protected.
/// 
/// The user ID SBT's trust factor score would not taken in regard any data related to the user's credit history. However, if that user has crime history related to credit (loan fraud), the trust factor score would be aggregated accordingly.
#[derive(NonFungibleData)]
pub struct Credit {

    /// A workaround way for restrictive proof.
    #[scrypto(mutable)]
    pub data: CreditData

}

/// A workaround way for restrictive proof.
#[derive(TypeId, Encode, Decode, Describe, Clone, Copy)]
pub struct CreditData {
    /// Credit score is needed to algorithmically calculate the maximum credit allowance for user and assessed by late (or on-time) payment frequency.
    /// 
    /// An user's credit score will be ranged from 0 to 100. 
    /// 
    /// Default credit score is equal to the user ID's trust score.
    pub credit_score: Decimal,
    /// An user's credit type can be "Revolving" Credit type or "Installment" Credit type.
    /// 
    /// Default credit type is the "Revolving" "Monthly" Credit type.
    // #[scrypto(mutable)]
    pub credit_type: CreditType,
    /// User's current debt start time. This is for the protocol to keep track of user's debt start time.
    /// 
    /// Default debt start time is 0.
    // #[scrypto(mutable)]
    pub current_debt_start_time: u64,
    /// User's current debt (include the original debt, not included the debt interest or extra debt on late repayment).
    /// 
    /// Default debt is 0.
    // #[scrypto(mutable)]
    pub current_debt: Decimal,
    /// User's debt interest on current debt.
    /// 
    /// Default debt interest is 0
    pub debt_interest: Decimal,
    /// User's debt due time.
    /// 
    /// Default due time is 0.
    // #[scrypto(mutable)]
    pub due_time: u64,
    /// The NFT data show the extra debt when user is late on repayment.
    /// 
    /// Default extra debt is 0,
    pub extra_debt: Decimal,
    /// User's accumulated repaid amount. 
    /// 
    /// This amount will be reset when it reached the maximum credit amount.
    /// 
    /// When the amount reset, user will got their credit score restored (if they has credit score < 100).
    /// 
    /// Default repaid amount is 0.
    // #[scrypto(mutable)]
    pub repaid_amount_accumulated: Decimal
}

/// Type of the credit.
/// 
/// Currently there are 2 credit types: Revolving Credit and Installment Credit.
#[derive(TypeId, Encode, Decode, Describe, Clone, Copy)]
pub enum CreditType {
    Revolving(RevolvingTypes),
    Installment(InstallmentCreditData)
}

/// Type of revolving credit.
/// 
/// Can have more revolving credit type in the future.
#[derive(TypeId, Encode, Decode, Describe, Clone, Copy)]
pub enum RevolvingTypes {
    /// Maximum credit amount is calculated by user's estimated yearly income data. Credit due time will be about one year since the loan is taken.
    Yearly,
    /// **Monthly**: Maximum credit amount is calculated by user's estimated monthly income data. Credit due time will be about one month since the loan is taken.
    Monthly
}

/// On-chain scoring rate of the credit.
#[derive(TypeId, Encode, Decode, Describe, Clone, Copy)]
pub struct CreditScoring {

    /// Credit degrade rate when user had late repayment frequency.
    pub degrade_rate: Decimal,
    /// Credit restore rate when user had on-time repayment frequency.
    pub restore_rate: Decimal

}

impl CreditScoring {
    pub fn check_rate(&self) {
        assert_rate(self.degrade_rate);
        assert_rate(self.restore_rate);
    }
}

/// The struct store on-chain scoring rate of the credit based by credit type.
#[derive(TypeId, Encode, Decode, Describe, Clone, Copy)]
pub struct CreditScoringRates {

    pub yearly: CreditScoring,
    pub monthly: CreditScoring
    
}

impl CreditScoringRates {

    pub fn check_rates(&self) {
        self.yearly.check_rate();
        self.monthly.check_rate()
    }
}

/// The NFT badge keeping track of an user's installment loan request.
/// 
/// ## Uses:
/// The installment loan can be job-support loan, business loan, house loan, education loan,......
/// 
/// After make a request, user has to wait for a review from the service operator.
/// 
/// After the request has passed, user can use this NFT to get their installment credit badge to change their credit type.
#[derive(NonFungibleData)]
pub struct RequestInstallmentCredit {}


/// The struct store on-chain installment credit data.
/// 
/// These are the data that the credit service operator agreed on. They can even be ensured through an off-chain legal process.
#[derive(TypeId, Encode, Decode, Describe, Clone, Copy)]
pub struct InstallmentCreditData {

    /// User's total loan.
    pub total_loan: Decimal,
    /// The installment credit interest rate.
    pub interest_rate: Decimal,
    /// The installment credit interest rate when user is late on repayment.
    pub interest_rate_late: Decimal,
    /// The time before user has to repay the next installment debt.
    pub period_length: u64,
    /// The maturity period time when user don't have to repay installment debt anymore.
    pub period_max: u8,
    /// The period counter.
    pub period_counter: u8

}

/// The NFT badge allow users to change credit type into an installment credit.
/// 
/// ## Uses:
/// After got the installment credit badge, user can take the loan at any time from any lending protocol that the credit service operator listed to start using installment credit.
/// 
/// User's credit will change into Installment Credit type and user has to repay both the loan and the interest in many periods as agreed with the credit service operator.
/// 
/// When using installment credit, user cannot use any other credit type.
#[derive(NonFungibleData)]
pub struct InstallmentCredit {
    /// The user's Credit SBT ID. 
    /// 
    /// This data is to make sure the one requested and used the installment credit is the same wallet address.
    pub sbt_id: NonFungibleId,
    /// Store the installment credit data.
    pub data: InstallmentCreditData
}

blueprint! {

    struct GroundCredit {

        /// Component controller badge
        controller_badge: Vault,
        /// The credit SBT address
        credit_sbt: ResourceAddress,
        /// Request book for keeping track of installment credit request.
        /// 
        /// **Format**: 
        ///
        /// `LazyMap<request_NFT_ID, (identity SBT address, installment_credit_data, status)>`
        request_book: LazyMap<NonFungibleId, (NonFungibleId, InstallmentCreditData, bool)>,
        /// Request id counter
        request_id_counter: u64,
        /// Request badge Resource Address
        request_badge: ResourceAddress,
        /// Installment Credit Badge Resource Address
        installment_credit_badge: ResourceAddress,
        /// Installment Credit Badge vault.
        /// 
        /// After the service operator pass a installment credit request, user can take the Installment Credit Badge from this vault.
        installment_credit_badge_vault: Vault,
        /// The on-using identity service.
        identity_service: ComponentAddress,
        /// The black listed ID SBTs which are not allowed to use credit because of variable reasons:
        /// 
        /// late repayment frequency, loan scam, change of income, change of trust factor score,...
        blacklist: Vec<NonFungibleId>,
        /// Credit scoring rates. 
        credit_scoring_rates: CreditScoringRates,
        /// Listed protocols can use the credit service.
        authorized_protocol: Vec<ResourceAddress>,
        /// List of credit user.
        /// 
        /// **Format**: 
        /// 
        /// `LazyMap<Identity SBT ID, Credit SBT ID>`
        credit_list: LazyMap<NonFungibleId, NonFungibleId>

    }

    impl GroundCredit {
        
        /// This function will create new GroundCredit component.
        /// 
        /// ### Input: 
        /// - name: the organization's name.
        /// - admin_badge: the organization admin badge address. (the component holding admin badge can be a multisig account or a DAO component).
        /// - credit_scoring_rates: credit scoring rates, syntax: 
        /// 
        /// ```Struct(Struct({yearly_degrade_rate}, {yearly_restore_rate}), Struct({monthly_degrate_rate}, {monthly_restore_rate}))```
        /// - identity_service: the Identity service component address which the ground credit component initializer use.
        /// ### Output: 
        /// Component address.
        pub fn new(name: String, admin_badge: ResourceAddress, credit_scoring_rates: CreditScoringRates, identity_service: ComponentAddress) -> ComponentAddress {

            credit_scoring_rates.check_rates();

            let controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", name.clone() + "'s Credit Service Controller Badge")
                .initial_supply(dec!(1isize));

            let authorized_protocol = vec![controller_badge.resource_address()];

            let credit_sbt = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s Credit SBT")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .updateable_non_fungible_data(rule!(deny_all), MUTABLE(rule!(require_any_of(authorized_protocol.clone()))))
                .no_initial_supply();
            
            let request_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s Installment Credit Request Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let installment_credit_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name +"'s Installment Credit Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), MUTABLE(rule!(require(controller_badge.resource_address()))))
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("issue_new_credit_sbt", rule!(require(admin_badge)))
                .method("review_installment_credit_request", rule!(require(admin_badge)))
                .method("list_protocol", rule!(require(admin_badge)))
                .method("delist_protocol", rule!(require(admin_badge)))
                .method("blacklist", rule!(require(admin_badge)))
                .method("whitelist", rule!(require(admin_badge)))
                .method("change_credit_scoring_rate", rule!(require(admin_badge)))
                .default(rule!(allow_all));

            let comp = Self {

                controller_badge: Vault::with_bucket(controller_badge),
                credit_sbt: credit_sbt,
                request_book: LazyMap::new(),
                request_id_counter: 0,
                request_badge: request_badge,
                installment_credit_badge: installment_credit_badge,
                installment_credit_badge_vault: Vault::new(installment_credit_badge),
                identity_service: identity_service,
                blacklist: Vec::new(),
                credit_scoring_rates: credit_scoring_rates,
                authorized_protocol: authorized_protocol,
                credit_list: LazyMap::new()

            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            return comp
        }

        /// This is a method for any user has on-chain unique identity to create new credit SBT.
        /// 
        /// Default credit score = trust score.
        /// 
        /// Input: the proof of user's Identity SBT.
        /// 
        /// Output: the credit SBT has it type modified
        pub fn get_new_credit_sbt(&self, id_sbt: Proof) -> Bucket {

            let id_sbt = self.check_id(id_sbt);

            let sbt_id = id_sbt.non_fungible::<Identity>().id();

            assert!(matches!(self.credit_list.get(&sbt_id), None), "You already has a credit SBT");

            let trust_score = id_sbt.non_fungible::<Identity>().data().data.trust_factor;

            id_sbt.drop();

            let id = NonFungibleId::random();

            info!("You got new Credit SBT no.{}", id.clone());
            
            self.credit_list.insert(sbt_id, id.clone());

            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.credit_sbt)
                    .mint_non_fungible(
                        &id,
                        Credit {

                            data: CreditData {
                                credit_type: CreditType::Revolving(RevolvingTypes::Monthly),
                                credit_score: trust_score,
                                current_debt_start_time: 0,
                                current_debt: Decimal::zero(),
                                debt_interest: Decimal::zero(),
                                due_time: 0,
                                extra_debt: Decimal::zero(),
                                repaid_amount_accumulated: Decimal::zero()
                            }
                        }
                )
            })
        } 

        /// This method is for the service operator to issue new Credit SBT after an off-chain process 
        /// for users that already has off-chain credit history. The data can be fed in through an Oracle.
        /// ### Input: 
        /// - sbt_id: the user's Identity SBT ID.
        /// - credit_score: aggregrated off-chain credit score of the user.
        /// ### Output: 
        /// The new Credit SBT.
        pub fn issue_new_credit_sbt(&self, sbt_id: NonFungibleId, credit_score: Decimal) -> Bucket {

            assert!(matches!(self.credit_list.get(&sbt_id), None), "This Identity already has a credit SBT");

            assert_rate(credit_score);

            let id = NonFungibleId::random();

            info!("Issued new Credit SBT no.{}", id.clone());

            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.credit_sbt)
                    .mint_non_fungible(
                        &id,
                        Credit {

                            data: CreditData {
                                credit_type: CreditType::Revolving(RevolvingTypes::Monthly),
                                credit_score: credit_score,
                                current_debt_start_time: 0,
                                current_debt: Decimal::zero(),
                                debt_interest: Decimal::zero(),
                                due_time: 0,
                                extra_debt: Decimal::zero(),
                                repaid_amount_accumulated: Decimal::zero()
                            }

                        }
                )
            })
        } 

        /// This method is for user to change their credit type. 
        /// ### Input: 
        /// - id_sbt: The Proof of the user's Credit SBT.
        /// - credit_sbt: the Proof of the user's Credit SBT.
        /// ### Output: 
        /// The new Credit SBT.
        pub fn change_credit_type(&self, id_proof: Proof, credit_sbt: Proof) {

            let (id_proof, credit_sbt) = self.check_id_and_credit(id_proof, credit_sbt);

            id_proof.drop();

            let credit = credit_sbt.non_fungible::<Credit>();

            let data = credit.data().data;

            assert!(data.due_time == 0, "You have to repay all your current debt first.");

            match data.credit_type {
                CreditType::Revolving(revolving_types) => {
                    match revolving_types {
                        RevolvingTypes::Monthly => {
                            self.controller_badge.authorize(|| {
                                credit.update_data(
                                    Credit {
                                        data: CreditData {
                                            credit_type: CreditType::Revolving(RevolvingTypes::Yearly),
                                            repaid_amount_accumulated: Decimal::zero(),
                                            ..data
                                        }
                                    }
                                )
                            });
                            info!("You have changed your credit into a yearly credit.")
                        }

                        RevolvingTypes::Yearly => {
                            self.controller_badge.authorize(|| {
                                credit.update_data(
                                    Credit {
                                        data: CreditData {
                                            credit_type: CreditType::Revolving(RevolvingTypes::Monthly),
                                            repaid_amount_accumulated: Decimal::zero(),
                                            ..data
                                        }
                                    }
                                )
                            });
                            info!("You have changed your credit into a monthly credit.")
                        }
                    }
                }

                _ => {

                    panic!("Wrong credit type.")
                }
            }

            credit_sbt.drop();

        }

        /// A workaround method for restrictive proof.
        pub fn get_revolving_credit_amount_by_data(&self, id_data: IdentityData, data: CreditData) -> (Decimal, Decimal) {
            
            assert!(id_data.trust_factor >  Decimal::zero(), "You're not allowed to use credit.");
            assert!(data.credit_score >  Decimal::zero(), "Your credit score has degraded to 0, you're not allowed to use credit.");

            match data.credit_type {
                CreditType::Revolving(types) => {

                    assert!(id_data.income > Decimal::zero(), "You don't have an income. Please consider using installment credit service.");

                    let yearly_maximum_credit = id_data.income * (id_data.trust_factor / 100) * (data.credit_score / 100);

                    let maximum_credit = match types {
                        RevolvingTypes::Monthly => {yearly_maximum_credit / dec!("12")}
                        RevolvingTypes::Yearly => {yearly_maximum_credit}
                    };

                    let mut allowance = maximum_credit - data.current_debt;

                    if allowance < Decimal::ZERO {
                        allowance = Decimal::ZERO
                    }

                    info!("Your current credit allowance is: {}", allowance);

                    (maximum_credit, allowance)

                }
                _ => {panic!("You're using an installment credit. You cannot take the revolving credit.")}
            }
        }

        /// This method is for users to get their maximum credit and current credit allowance.
        /// 
        /// The maximum credit amount is calculated by a cubic function with the income, id trust score and credit score as the params.
        /// 
        /// The current credit allowance = maximum credit - current debt.
        /// 
        /// ### Input: 
        /// - id_sbt: The Proof of the user's Credit SBT.
        /// - credit_sbt: the Proof of the user's Credit SBT.
        /// ### Output: 
        /// - User's maximum credit amount and current allowance.
        pub fn get_revolving_credit_amount(&self, id_proof: Proof, credit_sbt: Proof) -> (Decimal, Decimal) {

            let (id_proof, credit_sbt) = self.check_id_and_credit(id_proof, credit_sbt);

            // let data = credit_sbt.non_fungible::<Credit>().data();

            // let id_data = id_proof.non_fungible::<Identity>().data();

            let data = credit_sbt.non_fungible::<Credit>().data().data;

            let id_data = id_proof.non_fungible::<Identity>().data().data;

            // assert!(id_data.trust_factor >  Decimal::zero(), "You're not allowed to use credit.");
            // assert!(data.credit_score >  Decimal::zero(), "Your credit score has degraded to 0, you're not allowed to use credit.");
            // assert!(data.extra_debt == Decimal::zero(), "You have to repay your debt first!");

            // id_proof.drop(); credit_sbt.drop();

            // match data.credit_type {
            //     CreditType::Revolving(types) => {

            //         assert!(id_data.income > Decimal::zero(), "You don't have an income. Please consider using installment credit service.");

            //         let yearly_maximum_credit = id_data.income * (id_data.trust_factor / 100) * (data.credit_score / 100);

            //         let maximum_credit = match types {
            //             RevolvingTypes::Monthly => {yearly_maximum_credit / dec!("12")}
            //             RevolvingTypes::Yearly => {yearly_maximum_credit}
            //         };

            //         assert!(maximum_credit >= data.current_debt + data.debt_interest, "Out of credit, you have to repay your debt first!");

            //         let allowance = maximum_credit - data.current_debt - data.debt_interest;

            //         info!("Your current credit allowance is: {}", allowance);

            //         (maximum_credit, allowance)

            //     }
            //     _ => {panic!("You're using an installment credit. You cannot take the revolving credit.")}
            // }

            self.get_revolving_credit_amount_by_data(id_data, data)

        }
      
        /// This method is for users to request an installment loan.
        /// 
        /// ### Input: 
        /// - id_sbt: The Proof of the user's Identity SBT.
        /// - total_loan: the user's installment loan amount request.
        /// - interest_rate: the user's installment loan interest rate request.
        /// - interest_rate_late: the user's installment loan interest rate when user is late on repayment.
        /// - period_length: the length of each period that user has to repay the part of the loan.
        /// - period_max: the total period number that user wish to pay the loan.
        /// ### Output: 
        /// The Installment credit request badge.
        pub fn request_installment_credit(&mut self, id_sbt: Proof, total_loan: Decimal, interest_rate: Decimal, interest_rate_late: Decimal, period_length: u64, period_max: u8) -> (Bucket, u64) {

            let id_sbt = self.check_id(id_sbt);

            let sbt_id = id_sbt.non_fungible::<Identity>().id();

            id_sbt.drop();

            let request_id = self.request_id_counter;

            let id = NonFungibleId::from_u64(request_id);

            let interest_rate = interest_rate / dec!("100");

            let interest_rate_late = interest_rate_late / dec!("100");

            self.request_book.insert(id.clone(), (sbt_id.clone(), InstallmentCreditData {total_loan, interest_rate, interest_rate_late, period_length, period_max, period_counter: 0}, false));

            info!("Created a new installment credit request no.{} by the user ID {}", id, sbt_id);

            self.request_id_counter += 1;

            (self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.request_badge)
                    .mint_non_fungible(&id, RequestInstallmentCredit {})
            }), request_id)

        }

        /// This method is for the service operator to allow an user's installment loan request.
        /// 
        /// The organization will go through an off-chain process to allow user's installment loan.
        /// 
        /// This process will likely be ensured by an agreement between both parties.
        /// 
        /// The agreement can be supported by an off-chain legal document for future legal action.
        /// ### Input: 
        /// - id: the request ID.
        /// - is_ok: the request has reach agreement or not.
        /// ### Output: 
        /// The organization will put the agreed installment loan badge into the component for users to take the loan themselves.
        pub fn review_installment_credit_request(&mut self, id: u64, is_ok: bool) {

            let request_id = NonFungibleId::from_u64(id);

            let result = self.request_book.get(&request_id);

            assert!(result.is_some(),
                "The request book doesn't contain this request id."
            );

            let (sbt_id, data, mut status) = result.unwrap();

            assert!(!status,
                "This request is already reviewed."
            );

            status = true;

            self.request_book.insert(request_id.clone(), (sbt_id.clone(), data, status));

            if is_ok {

                info!("The installment credit request no.{} has passed.", request_id);

                self.controller_badge.authorize(|| {
                    self.installment_credit_badge_vault.put(
                    borrow_resource_manager!(self.installment_credit_badge)
                        .mint_non_fungible(&request_id, InstallmentCredit {

                            sbt_id: sbt_id,
                            data: data

                        }))
                })
                
            } else {
                info!("The installment credit request no.{} has been rejected.", request_id);
            }

        }

        /// This method is for users to get the installment credit badge from the component.
        /// ### Input: 
        /// - request_badge: the request badge bucket
        /// - id_proof: the user's ID SBT proof
        /// ### Output: 
        /// Return None if the request has rejected and the installment credit badge if the request has passed
        pub fn get_installment_credit_badge(&mut self, request_badge: Bucket, id_proof: Proof) -> Option<Bucket> {

            assert!(request_badge.resource_address() == self.request_badge, "Wrong resource!");

            self.check_id(id_proof);

            let request_id = request_badge.non_fungible::<Request>().id();

            let (_, _, status) = self.request_book.get(&request_id).unwrap();

            assert!(status,
                "The organization haven't reviewed your request yet."
            );

            self.controller_badge.authorize(|| {
                request_badge.burn()
            });

            if self.installment_credit_badge_vault.non_fungible_ids().contains(&request_id) {
                info!("Your installment credit request no.{} has passed.", request_id);
                Some(self.installment_credit_badge_vault.take_non_fungible(&request_id))
            } else {
                info!("Your installment credit request no.{} has been rejected.", request_id);
                None
            }

        }

        // /// This method is for protocols to check and update user's installment credit data
        // /// before user take the installment loan.
        // /// ### Input: 
        // /// - id_sbt_proof: The Proof of the user's Credit SBT.
        // /// - credit_proof: the Proof of the user's Credit SBT.
        // /// - protocol_proof: The Proof of the protocol's controller badge.
        // /// - installment_credit_badge: The Bucket of user's Installment Credit NFT.
        // /// - current: Current time data fed in through the protocol. (unix)
        // /// ### Output: 
        // /// Return the installment credit amount.
        // /// 
        // /// Notice: This will not work with restrictive proof
        // pub fn use_installment_credit(&self, id_sbt_proof: Proof, credit_proof: Proof, protocol_proof: Proof, installment_credit_badge: Bucket, current: u64) -> Decimal {

        //     self.check_protocol(protocol_proof);

        //     let credit = credit_proof.non_fungible::<Credit>();

        //     let data = credit.data();

        //     let id_sbt = id_sbt_proof.non_fungible::<Identity>().id();

        //     id_sbt_proof.drop(); 

        //     assert!(installment_credit_badge.resource_address() == self.installment_credit_badge, "Wrong resource!");

        //     let installment_data = installment_credit_badge.non_fungible::<InstallmentCredit>().data();

        //     let installment_id = installment_data.sbt_id;

        //     assert!(id_sbt == installment_id, "Wrong Installment Credit Badge provided.");

        //     assert!(data.due_time == 0, "You have to repay all your current debt first.");

        //     let mut installment_data = installment_data.data;

        //     installment_data.period_counter = 1;
            
        //     let current_debt = installment_data.total_loan / installment_data.period_max;

        //     let debt_interest = current_debt * installment_data.interest_rate;

        //     let due_time = current + installment_data.period_length;

        //     self.controller_badge.authorize(|| { 
        //         installment_credit_badge.burn();
        //         credit.update_data(Credit {
        //             credit_type: CreditType::Installment(installment_data),
        //             current_debt_start_time: current,
        //             current_debt,
        //             debt_interest,
        //             due_time,
        //             ..data
        //         })
        //     });

        //     info!("You have changed your credit type into Installment Credit. Your current debt is: {}. Your debt will over due in: {} (unix time)", current_debt, due_time);

        //     credit_proof.drop();

        //     installment_data.total_loan
                    
        // }

        // /// This method is for protocols to check and update user's installment credit data
        // /// after user repaid the period loan.
        // /// ### Input: 
        // /// - credit_proof: the Proof of the user's Credit SBT.
        // /// - protocol_proof: The Proof of the protocol's controller badge.
        // /// - late: input if the repayment is late or not.
        // /// ### Output:
        // /// - Update the installment credit data based on user's repayment.
        // /// - Return the next period debt amount.
        // /// 
        // /// Notice: This will not work with restrictive proof
        // pub fn update_installment_credit_data(&self, credit_proof: Proof, protocol_proof: Proof, late: bool) -> Decimal {

        //     self.check_protocol(protocol_proof);

        //     let credit = credit_proof.non_fungible::<Credit>();

        //     let mut data = credit.data();

        //     let mut installment_data = match data.credit_type {

        //         CreditType::Installment(data) => {data}
        //         _ => {panic!("Wrong credit type!")}

        //     };

        //     if !late {
        //         data.credit_score += self.credit_scoring_rates.monthly.restore_rate
        //     };

        //     if installment_data.period_counter < installment_data.period_max {

        //         installment_data.period_counter += 1;

        //         let current_debt = installment_data.total_loan / installment_data.period_max;

        //         let debt_interest = current_debt * installment_data.interest_rate;

        //         let period_length = installment_data.period_length;

        //         info!("You have repaid all the current period debt from your installment credit, your period will be advanced by 1. Current period: {}", installment_data.period_counter);

        //         data = Credit {
        //             credit_type: CreditType::Installment(installment_data),
        //             current_debt_start_time: data.due_time,
        //             current_debt,
        //             debt_interest,
        //             due_time: data.due_time + period_length,
        //             ..data
        //         };

        //     } else if installment_data.period_counter == installment_data.period_max {

        //         installment_data.period_counter += 1;

        //         data = Credit {
        //             credit_type: CreditType::Revolving(RevolvingTypes::Monthly),
        //             current_debt_start_time: 0,
        //             current_debt: Decimal::ZERO,
        //             debt_interest: Decimal::ZERO,
        //             due_time: 0,
        //             ..data
        //         };

        //         info!("You have repaid all your installment loan, your credit will change into the Monthly Revolving Credit");

        //     } else {panic!("You have already repaid all the installment credit debt. Please consider changing your credit type into revolving credit.")};

        //     self.controller_badge.authorize(|| { 
        //         credit.update_data(Credit {
        //             ..data
        //         })
        //     });

        //     credit_proof.drop();

        //     return data.current_debt

        // }

        // /// This method is for the service operator to allow a protocol using on-chain credit service.
        // /// 
        // /// Input: The protocol controller badge resource address
        // pub fn list_protocol(&mut self, protocol_controller_address: ResourceAddress) {

        //     self.authorized_protocol.push(protocol_controller_address);

        //     info!("listed the lending protocol with controller badge address {}", protocol_controller_address);

        // }

        // /// This method is for the service operator to deny a protocol using on-chain credit service.
        // /// All protocols is denied by default.
        // /// 
        // /// Input: The protocol controller badge resource address
        // pub fn delist_protocol(&mut self, protocol_controller_address: ResourceAddress) {

        //     let index = self.authorized_protocol.iter().position(|x| *x == protocol_controller_address);

        //     match index {
        //         None => {info!("Doesn't have this protocol on the list.")}
        //         Some(x) => {

        //             self.authorized_protocol.remove(x);

        //             info!("delisted the lending protocol with controller badge address {}", protocol_controller_address);

        //         }
        //     }
        // }

        /// This method is for the service operator to allow a protocol using on-chain credit service.
        /// 
        /// Input: The protocol controller badge resource address
        pub fn list_protocol(&mut self, protocol_controller_address: ResourceAddress) {

            self.authorized_protocol.push(protocol_controller_address);

            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.credit_sbt)
                .set_updateable_non_fungible_data(rule!(require_any_of(self.authorized_protocol.clone())));
                borrow_resource_manager!(self.installment_credit_badge)
                .set_burnable(rule!(require_any_of(self.authorized_protocol.clone())))
            });

            info!("listed the lending protocol with controller badge address {}", protocol_controller_address);

        }

        /// This method is for the service operator to deny a protocol using on-chain credit service.
        /// 
        /// Input: The protocol controller badge resource address
        /// 
        /// All the protocol are denied by default
        pub fn delist_protocol(&mut self, protocol_controller_address: ResourceAddress) {

            let index = self.authorized_protocol.iter().position(|x| *x == protocol_controller_address);

            match index {
                None => {info!("Doesn't have this protocol on the list.")}
                Some(x) => {

                    self.authorized_protocol.remove(x);

                    info!("delisted the lending protocol with controller badge address {}", protocol_controller_address);

                }
            }

            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.credit_sbt)
                .set_updateable_non_fungible_data(rule!(require_any_of(self.authorized_protocol.clone())));
                borrow_resource_manager!(self.installment_credit_badge)
                .set_burnable(rule!(require_any_of(self.authorized_protocol.clone())));
            });

        }
        // /// This method is to check if the protocol is listed or not.
        // /// 
        // /// Input: The protocol controller badge proof.
        // pub fn check_protocol(&self, protocol_proof: Proof) {
        //     assert!(self.authorized_protocol.contains(&protocol_proof.resource_address()), "This protocol is not allowed to use on-chain credit service.");
        //     protocol_proof.drop();
        // }

        // /// This method is for lending protocol to update the debt amount of an user.
        // /// ### Input: 
        // /// - credit_proof: the user's credit proof.
        // /// - protocol_proof: the protocol controller's proof.
        // /// - current_debt: new debt amount.
        // /// - extra_debt: new extra debt amount.
        // /// ### Output: 
        // /// update the credit debt amount.
        // /// 
        // /// Notice: This will not work with restrictive proof
        // pub fn update_debt(&self, credit_proof: Proof, protocol_proof: Proof, current_debt: Decimal, debt_interest: Decimal, extra_debt: Decimal) {

        //     self.check_protocol(protocol_proof);
        //     let credit = credit_proof.non_fungible::<Credit>();
        //     let data = credit.data();

        //     self.controller_badge.authorize(|| {
        //         credit.update_data(
        //             Credit {
        //                 current_debt,
        //                 debt_interest,
        //                 extra_debt,
        //                 ..data
        //             }
        //         )
        //     });
        //     credit_proof.drop();
        //     info!("Your current debt is {}", current_debt +  extra_debt)

        // }

        // /// This method is for lending protocol to update credit due time of an user.
        // /// ### Input: 
        // /// - credit_proof: the user's credit proof.
        // /// - protocol_proof: the protocol controller's proof.
        // /// - due_time: new credit loan due time
        // /// - current_debt_start_time: new credit loan start time.
        // /// ### Output: 
        // /// update the user's credit due time
        // /// 
        // /// Notice: This will not work with restrictive proof
        // pub fn update_debt_time(&self, credit_proof: Proof, protocol_proof: Proof, due_time: u64, current_debt_start_time: u64) {

        //     self.check_protocol(protocol_proof);
        //     let credit = credit_proof.non_fungible::<Credit>();
        //     self.controller_badge.authorize(|| {
        //         credit.update_data(
        //             Credit {
        //                 due_time,
        //                 current_debt_start_time,
        //                 ..credit.data()
        //             }
        //         )
        //     });
        //     credit_proof.drop();
        //     info!("Your debt will be over due in {} (unix time)", due_time)

        // }

        // /// This method is for lending protocol to degrade credit score of an user.
        // /// ### Input: 
        // /// - credit_proof: the user's credit proof.
        // /// - protocol_proof: the protocol controller's proof.
        // /// ### Output: 
        // /// degrade the credit score
        // /// 
        // /// Notice: This will not work with restrictive proof
        // pub fn degrade_credit(&self, credit_proof: Proof, protocol_proof: Proof) {

        //     assert!(credit_proof.resource_address() == self.credit_sbt, "Wrong resource!");
        //     self.check_protocol(protocol_proof);
        //     let credit = credit_proof.non_fungible::<Credit>();
        //     let old_data = credit.data();

        //     let score = old_data.credit_score;

        //     let degrade_rate = match old_data.credit_type {

        //         CreditType::Revolving(ref types) => {

        //             match types {

        //                 RevolvingTypes::Monthly => {self.credit_scoring_rates.monthly.degrade_rate}
        //                 RevolvingTypes::Yearly => {self.credit_scoring_rates.yearly.degrade_rate}

        //             }
                    
        //         }

        //         _ => {self.credit_scoring_rates.monthly.degrade_rate}

        //     };

        //     let new_score = if score >= degrade_rate {
        //         score - degrade_rate
        //     } else { Decimal::zero() };

        //     self.controller_badge
        //     .authorize(|| { 
        //         credit.update_data(
        //             Credit {
        //                 credit_score: new_score,
        //                 ..old_data
        //             }
        //         )
        //     });

        //     info!("Your credit score has been degraded to {} because of late repayment", new_score);

        //     credit_proof.drop()

        // }

        // /// This method is for lending protocol to update repaid data and restore credit score of an user (if passed the maximum credit amount).
        // /// ### Input: 
        // /// - id_proof: the user's ID proof.
        // /// - credit_proof: the user's credit proof.
        // /// - protocol_proof: the protocol controller's proof.
        // /// - amount: token amount the user has repaid.
        // /// ### Output: 
        // /// update repaid data and restore the credit score (if passed the maximum credit amount).
        // /// 
        // /// Notice: This will not work with restrictive proof
        // pub fn update_revolving_credit_repaid_accumulate(&self, id_proof: Proof, credit_proof: Proof, protocol_proof: Proof, amount: Decimal) {

        //     self.check_protocol(protocol_proof);
        //     let (maximum, _) = self.get_revolving_credit_amount(id_proof, credit_proof.clone());
            
        //     let credit = credit_proof.non_fungible::<Credit>();
        //     let old_data = credit.data();

        //     let new_accumulated = old_data.repaid_amount_accumulated + amount;

        //     if new_accumulated >= maximum {

        //         let restore_rate = match old_data.credit_type {
        //             CreditType::Revolving(ref types) => {

        //                 match types {
    
        //                     RevolvingTypes::Monthly => {self.credit_scoring_rates.monthly.restore_rate}
        //                     RevolvingTypes::Yearly => {self.credit_scoring_rates.yearly.restore_rate}
    
        //                 }
                        
        //             }

        //             _ => {panic!("Wrong credit type!")}
    
        //         };

        //         let score = old_data.credit_score;

        //         let new_score = if score <= (dec!("100") - restore_rate) {
        //             score + restore_rate
        //         } else { dec!("100") };

        //         self.controller_badge
        //         .authorize(|| { 
        //             credit.update_data(
        //                 Credit {
        //                     credit_score: new_score,
        //                     repaid_amount_accumulated: Decimal::zero(),
        //                     ..old_data
        //                 }
        //             )
        //         });

        //         info!("Your credit score has been restored to {} because of your on-time repayment frequency", new_score);

        //     } else {
        //         self.controller_badge
        //         .authorize(|| { 
        //             credit.update_data(
        //                 Credit {
        //                     repaid_amount_accumulated: new_accumulated,
        //                     ..old_data
        //                 }
        //             )
        //         });
        //     }

        //     credit_proof.drop()
            
        // }

        /// This method is for the service operator to blacklist an ID SBT.
        pub fn blacklist(&mut self, id: NonFungibleId) {
            assert!(!matches!(self.credit_list.get(&id), None), "This ID haven't got a credit account yet!");
            info!("ID address {} has been blacklisted", id.clone());
            self.blacklist.push(id);
        }

        /// This method is for the service operator to whitelist an ID SBT.
        /// 
        /// All ID SBTs is whitelisted by default.
        pub fn whitelist(&mut self, id: NonFungibleId) {

            let index = self.blacklist.iter().position(|x| *x == id);

            match index {
                None => {info!("Doesn't have this ID on the blacklist.")}
                Some(x) => {

                    self.blacklist.remove(x);

                    info!("ID address {} has been whitelisted", id.clone());

                }
            }
        }

        /// This method is to check if the SBT address is blacklisted or not.
        pub fn check_id(&self, id_proof: Proof) -> Proof {
            let identity_service: GroundID = self.identity_service.into();
            identity_service.check_resource(id_proof.resource_address());
            assert!(!self.blacklist.contains(&id_proof.non_fungible::<Identity>().id()), "You're not allowed to use credit. Please contact your credit issuer.");
            id_proof
        }

        /// This method is to check the SBT proof and the credit proof, see if they match (on the same wallet) or not.
        pub fn check_id_and_credit(&self, id_proof: Proof, credit_proof: Proof) -> (Proof, Proof) {
            let id_proof = self.check_id(id_proof);
            let sbt_id = id_proof.non_fungible::<Identity>().id();
            let credit = credit_proof.non_fungible::<Credit>();
            assert!(credit_proof.resource_address() == self.credit_sbt, "Wrong resource!");
            assert!(credit.id() == self.credit_list.get(&sbt_id).unwrap(), "Wrong credit SBT!");
            (id_proof, credit_proof)
        }

        /// Workaround method...
        pub fn full_check(&self, id: NonFungibleId, id_resource: ResourceAddress, credit_id: NonFungibleId, credit_resource: ResourceAddress) {
            let id = self.check_id_and_credit_by_data(id, id_resource, credit_id, credit_resource);
            assert!(!self.blacklist.contains(&id), "You're not allowed to use credit. Please contact your credit issuer.");
        }

        pub fn check_id_and_credit_by_data(&self, id: NonFungibleId, id_resource: ResourceAddress, credit_id: NonFungibleId, credit_resource: ResourceAddress) -> NonFungibleId {
            let identity_service: GroundID = self.identity_service.into();
            identity_service.check_resource(id_resource);
            assert!(credit_resource == self.credit_sbt, "Wrong resource!");
            assert!(credit_id == self.credit_list.get(&id).unwrap(), "Wrong credit SBT!");
            id
        }

        pub fn check_installment_credit(&self, resource_address: ResourceAddress) {
            assert!(resource_address == self.installment_credit_badge, "Wrong resource!");
        }

        /// The method for the service operators to change on-chain credit scoring rates. syntax:
        /// 
        /// ```Struct(Struct({yearly_degrade_rate}, {yearly_restore_rate}), Struct({monthly_degrate_rate}, {monthly_restore_rate}))```
        pub fn change_credit_scoring_rate(&mut self, credit_scoring_rates: CreditScoringRates) {
            credit_scoring_rates.check_rates();
            self.credit_scoring_rates = credit_scoring_rates
        }

        pub fn credit_scoring_rate(&self) -> CreditScoringRates {
            self.credit_scoring_rates
        }
    }
}