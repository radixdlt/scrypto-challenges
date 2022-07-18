//! # [GroundLending](./ground_lending/blueprint/struct.GroundLending.html): Make a Ground for your Web 3 Finance!
//!
//! Ground Lending is the core blueprint of the Ground Finance package, provide collateral-free lending solution to maximize capital efficiency for borrowers and earn rates for lenders, allow on-chain "bank level" earning tracker while protecting lender's privacy, ensuring security and dynamic, transparent interest rate at the same time.
//!
//! ## Main Features
//!
//! The blueprint is for web3 organizations to instantiate and manage a collateral-free lending protocol on-chain. 
//!
//! The blueprint utilized the Credit Service from GroundCredit blueprint, the Oracle solution from NeuRacle blueprint 
//! and the business DAO solution from GroundBusinessDAO blueprint:
//!
//! - The Credit Service is for the protocol to keep track and update the borrower's credit data: current debt (include initial debt, debt interest and extra debt by late repayment), credit score, credit due time, credit start time.
//!
//! - The Oracle solution is for the protocol to keep track on the passage of time, to see which repayment is on-time (or late) and which lending accounts are eligible for the interest from borrowers, enable "bank level" earning tracker for lenders.
//!
//! - The DAO solution is to run the protocol by collective actions, reduce human "bias" in the lending protocol. 
//!
//! The DAO also provide a "risk-backed" method called "compensate" which will compensate lenders a part of their lending, taken directly from the DAO treasury in case of cooperated loan defaults.
//!
//! ## Protocol Entities:
//! 1. **Protocol Operator**: Main manager of the protocol (can also be a DAO). Through the blueprint's method, *protocol operator* is allowed to:
//! - Change the DAO component address the protocol is using.
//! - Change the Oracle component address the protocol is using.
//! - Funding the Oracle account from a badge received from that Oracle.
//! - Change the protocol's revolving credit interest rates.
//! - Change the protocol's fee and compensate rate.
//! - Change the protocol's tolerance threshold (the minimum remained percent in protocol's vault allowed for user to take a loan).
//! - Take the protocol's fee.
//! - Deposit a stable coin bucket into the protocol's vault to support the protocol in case of loan default.
//!
//! 2. **Lenders**: Any wallet address (permissionless) wish to lend the protocol their stable coin to maximize earn rates. Through the blueprint's method, *lenders* are allowed to:
//! - Lend an amount of stable coins into the protocol to earn interest and get the Account badge.
//! - Withdraw part of (or all) the return amount from the Account badge.
//! - Take the compensate amount from the DAO running this protocol in the worse case of cooperated loan default.
//!
//! 2. **Borrowers**: Permissioned wallet address (require ID SBT and Credit SBT) can make an automated collateral-free 
//! loan through this blueprint to maximize capital efficiency. 
//! Through the blueprint's method, *borrowers* are allowed to:
//! - Use the revolving credit SBT to take the revolving loan
//! - Use the installment credit badge to take the installment loan and change credit SBT into installment type.
//! - Get the current total debt (the debt is increased if user's late on repayment).
//! - Repay part of the current debt or repay in full.

use scrypto::prelude::*;
use neuracle::neuracle::*;
use ground_business::ground_business_dao::*;
use ground_id::{Identity};
use crate::utils::*;
use crate::ground_credit::*;

const MONTH: u64 = 60 * 60 * 24 * 30;
const YEAR: u64 = 60 * 60 * 24 * 365;

/// The struct keep track of lender's data.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct Lender {

    /// The current user's lending amount. The amount can be decreased when lender make a withdrawal.
    /// 
    /// The amount will also automatically increase over time when borrowers repay their loan.
    /// 
    /// If lenders make a withdrawal before eligible interests are concluded by borrower's repayments, 
    /// they won't get the interest from the withdrawal amount.
    lending_amount: Decimal,
    /// The user's lending start time.
    /// 
    /// This is to keep track of the start time of the lender's account.
    /// 
    /// This data is fixed for each lending account.
    /// 
    /// Lenders are only eligible for interest from the loans made after this time data.
    start_time: u64

}

impl Lender {
    pub fn interest(&mut self, interest: Decimal) {
        self.lending_amount *= interest
    }
}

/// The NFT keep track of user's lending account on the protocol.
/// 
/// ## Uses:
/// To get new Account NFT, user use the "new_lending_account" method with a bucket of stable coin.
/// 
/// Lenders can also use the NFT to withdraw the return amount (lending amount + interest) from the protocol. Method: "withdraw" or "withdraw_all".
#[derive(NonFungibleData)]
pub struct Account {}

/// On-chain revolving credit interest rate of the lending protocol.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct Interest {

    /// Interest rate when the repayment is made on-time
    pub interest_rate: Decimal,
    /// Interest rate when the user is late on repayment.
    pub interest_rate_late: Decimal

}

impl Interest {

    pub fn check_rate(&self) {
        assert_rate(self.interest_rate);
        assert_rate(self.interest_rate_late)
    }

    pub fn rate_aggregrate(&mut self) {
        self.interest_rate = self.interest_rate / dec!("100");
        self.interest_rate_late = self.interest_rate_late / dec!("100");
    }
}

/// The struct store on-chain interest rate of the lending protocol based by revolving credit type.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct RevolvingCreditInterestRates {

    /// Yearly revolving credit interest rate.
    pub yearly: Interest,
    /// Monthly revolving credit interest rate.
    pub monthly: Interest
    
}

impl RevolvingCreditInterestRates {

    pub fn check_rates(&self) {
        self.yearly.check_rate();
        self.monthly.check_rate()
    }

    pub fn rates_aggregrate(&mut self) {
        self.yearly.rate_aggregrate();
        self.monthly.rate_aggregrate()
    }
}

blueprint! {

    struct GroundLending {

        /// Component controller badge
        controller_badge: Vault,
        /// The lending account NFT address
        account_nft: ResourceAddress,
        /// Lending protocol revolving credit interest rates.
        interest_rates: RevolvingCreditInterestRates, 
        /// Vault keep the total remain amount from the lenders's return amount (lending amount + interest) subtract the total unpaid credit.
        /// 
        /// Borrowers will take the whitelisted credit amount from this vault and make repayment into the vault.
        vault: Vault,
        /// The total return amount. 
        /// 
        /// total_return = total_deposited + total_interest.
        total_return: Decimal,
        /// Fee vault for GroundFi operator to maintain their service.
        fee_vault: Vault,
        /// Fee percent for lenders when they made a withdrawal. (%)
        fee: Decimal,
        /// The map keep track of protocol lender's data.
        /// 
        /// **Syntax**:
        /// ```HashMap<lender_account_id, lender_data>```
        lenders: HashMap<NonFungibleId, Lender>,
        /// Initial minimum remaining rate percent allowed for any credit request.
        /// 
        /// ```remaining_rate = vault_remain / total_return```
        tolerance_threshold: Decimal,
        /// The on-using credit service component address
        credit_service: ComponentAddress,
        /// The on-using Oracle ```(component_address, oracle_user_badge)```
        oracle: (ComponentAddress, Vault),
        /// The DAO Component Address
        dao: Option<ComponentAddress>,
        /// The compensate rate in case of loan default
        compensate_rate: Decimal,
        /// The mainnet time of the protocol. This is for calculate the protocol APY rate
        mainnet: u64

    }

    impl GroundLending {
        
        /// This function will create new GroundLending component.
        /// 
        /// ### Input: 
        /// - name: the organization's name.
        /// - admin_badge: the organization admin badge. (the component holding admin badge can be a multisig account or a DAO component).
        /// - interest_rates: The initial lending interest rates.
        /// 
        /// Syntax: ```Struct(Struct({yearly_interest_rate}, {yearly_interest_rate_late}), Struct({monthly_interest_rate}, {monthly_interest_rate_late}))```
        /// - stablecoin: the fiat-backed stable coin address will be used on GroundFi protocol.
        /// - fee: initial fee percent for GroundFi operator. (%)
        /// - tolerance_threshold: initial minimum percent allowed remaining rate of the total return amount for any credit request. (%)
        /// - credit_service: initial credit service component address.
        /// - oracle: initial oracle component address and the time data badge.
        /// - dao: the DAO run this lending protocol (None if this is a protocol run by an individual).
        /// - compensate_rate: initial compensate rate of the protocol.
        /// - mainnet: the mainnet time of the protocol.
        /// 
        /// ```remaining rate = vault remain / total return```
        /// ### Output: Component address and the controller badge resource address (for test purpose).
        pub fn new(
            name: String, 
            admin_badge: ResourceAddress,
            mut interest_rates: RevolvingCreditInterestRates,
            stablecoin: ResourceAddress, 
            fee: Decimal, 
            tolerance_threshold: Decimal,
            credit_service: ComponentAddress,
            oracle: (ComponentAddress, Bucket),
            dao: Option<ComponentAddress>,
            compensate_rate: Decimal,
            mainnet: u64
        ) -> (ComponentAddress, ResourceAddress) {

            interest_rates.check_rates();
            interest_rates.rates_aggregrate();
            assert_rate(tolerance_threshold); assert_rate(compensate_rate); assert_rate(fee); 

            let controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", name.clone() + "'s Lending Protocol Controller Badge")
                .initial_supply(dec!(1isize));

            let account_nft = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s Lending NFT")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("use_dao", rule!(require(admin_badge)))
                .method("use_oracle", rule!(require(admin_badge)))
                .method("change_interest_rates", rule!(require(admin_badge)))
                .method("change_fee", rule!(require(admin_badge)))
                .method("change_tolerance_threshold", rule!(require(admin_badge)))
                .method("change_compensate_rate", rule!(require(admin_badge)))
                .method("withdraw_fee", rule!(require(admin_badge)))
                .method("withdraw_extra", rule!(require(admin_badge)))
                .default(rule!(allow_all));

            let controller_badge_resource_address = controller_badge.resource_address();

            let comp = Self {

                controller_badge: Vault::with_bucket(controller_badge),
                account_nft: account_nft,
                interest_rates: interest_rates,
                vault: Vault::new(stablecoin),
                total_return: Decimal::zero(),
                fee_vault: Vault::new(stablecoin),
                fee: fee / dec!("100"),
                lenders: HashMap::new(),
                tolerance_threshold: tolerance_threshold / dec!("100"),
                credit_service: credit_service,
                oracle: (oracle.0, Vault::with_bucket(oracle.1)),
                dao: dao,
                compensate_rate: compensate_rate / dec!("100"),
                mainnet: mainnet

            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            return (comp, controller_badge_resource_address)
        }

        /// This method is for the protocol operator to change oracle using.
        /// 
        /// Ground Lending will use NeuRacle.
        pub fn use_oracle(&mut self, oracle: ComponentAddress, data_badge: Bucket) -> Bucket {
            self.oracle.0 = oracle;
            let bucket = self.oracle.1.take_all();
            self.oracle.1.put(data_badge);
            bucket
        }

        /// This method is for the protocol operator to refund the oracle account.
        pub fn refund_oracle_account(&self, bucket: Bucket) -> Bucket {
            let neuracle: NeuRacle = self.oracle.0.into();
            let data_proof = self.oracle.1.create_proof();
            neuracle.refund_account(data_proof, bucket)
        }

        /// This method is for users to lend their stable coin to the protocol and get the lending nft to become protocol's lenders.
        /// 
        /// Input: the stablecoin bucket
        /// 
        /// Output: The Lending Account NFT.
        pub fn new_lending_account(&mut self, stablecoin: Bucket) -> Bucket {

            assert!(stablecoin.resource_address() == self.vault.resource_address(), "Wrong resource!");

            let neuracle: NeuRacle = self.oracle.0.into();
            let data_proof = self.oracle.1.create_proof();
            let current = neuracle.get_data(data_proof);
            let current: u64 = current.parse().expect("Wrong data!");

            let amount = stablecoin.amount();
            self.vault.put(stablecoin);
            info!("You have lent {} stable coin to the protocol", amount);
            self.total_return += amount;
            let id = NonFungibleId::random();
            self.lenders.insert(id.clone(), Lender{ lending_amount: amount, start_time: current });
            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.account_nft)
                    .mint_non_fungible(
                        &id,
                        Account {}
                )
            })

        }

        /// Assert if the protocol's vault contain enough repayment for lender or not.
        pub fn assert_protocol_vault(&self, amount: Decimal) {
            let remain = self.vault.amount();
            assert!(amount <= remain, "Current there are only {} stablecoin left on the vault, please try get your compensation instead!", remain);
        }

        /// This method is for lenders to withdraw their return amount from the protocol
        /// 
        /// Input: Lending Account NFT proof and withdraw amount
        /// 
        /// Output: lender's withdrawal
        pub fn withdraw(&mut self, account_proof: Proof, amount: Decimal) -> Bucket {

            assert!(account_proof.resource_address() == self.account_nft, "Wrong resource");

            self.assert_protocol_vault(amount);

            let id = account_proof.non_fungible::<Account>().id();

            if let Some(lender) = self.lenders.get_mut(&id) {

                let fee = amount * self.fee;

                let return_amount = amount - fee;

                info!("Your current account have {} stable coins", lender.lending_amount);

                info!("Withdrawing {} stable coins", amount);

                assert!(lender.lending_amount >= amount, "Your account amount is not enough!");

                lender.lending_amount -= amount;

                self.total_return -= amount;

                let mut bucket = self.vault.take(amount);

                self.deposit_fee(bucket.take(fee));

                info!("You have paid {} protocol fee and withdrawed {} stable coins from the protocol", fee, return_amount);

                bucket

            } else {panic!("The protocol don't have your lender account.")}
            
        }

        /// This method is for lenders to burn their lending account NFTs and withdraw all their return amount from the protocol
        /// 
        /// Input: Account NFT bucket
        /// 
        /// Output: Lender's withdrawal
        pub fn withdraw_all(&mut self, account_badges: Bucket) -> Bucket {

            assert!(account_badges.resource_address() == self.account_nft, "Wrong resource");

            let account_badges_data = account_badges.non_fungibles::<Account>();

            let mut withdraw_amount = Decimal::ZERO;

            for account_badge in account_badges_data {

                let id = account_badge.id();

                if let Some(lender) = self.lenders.remove(&id) {

                    withdraw_amount += lender.lending_amount;
    
    
                } else {panic!("The protocol don't have your lender account.")}

            };

            let fee = withdraw_amount * self.fee;
    
            self.total_return -= withdraw_amount;

            self.controller_badge.authorize(|| {
                account_badges.burn()
            });

            self.assert_protocol_vault(withdraw_amount);

            let return_amount = withdraw_amount - fee;

            let mut bucket = self.vault.take(withdraw_amount);

            self.deposit_fee(bucket.take(fee));

            info!("Your current account have {} stable coins", withdraw_amount);
    
            info!("You have paid {} protocol fee and withdrawed all {} stable coins from the protocol", fee, return_amount);

            bucket

        }

        // The following method will not work...
        // /// This method is for the permissioned borrower to take a loan from their revolving credit.
        // /// ### Input: 
        // /// - id_proof: the Identity SBT proof.
        // /// - credit_sbt: the Credit SBT proof.
        // /// - amount: the amount borrower wish to loan.
        // /// ### Output: 
        // /// borrower's loan by amount.
        // pub fn revolving_credit(&mut self, id_proof: Proof, credit_sbt: Proof, amount: Decimal) -> Bucket {
            
        //     assert!(amount > Decimal::zero(), "Wrong data provided!");

        //     let credit_service: GroundCredit = self.credit_service.into();

        //     assert!(self.vault.amount() / self.total_return > self.tolerance_threshold, "Currently you cannot take your credit from this protocol, please come back later.");

        //     let credit_data = credit_sbt.non_fungible::<Credit>().data();

        //     let (interest_rate, time) = match credit_data.credit_type {

        //         CreditType::Revolving(revolving_types) => {
        //             match revolving_types {
        //                 RevolvingTypes::Monthly => {(self.interest_rates.monthly.interest_rate, MONTH)}

        //                 RevolvingTypes::Yearly => {(self.interest_rates.yearly.interest_rate, YEAR)}
        //             }
        //         }

        //         _ => {panic!("Wrong credit type!")}
        //     };

        //     let neuracle: NeuRacle = self.oracle.0.into();
        //     let data_proof = self.oracle.1.create_proof();
        //     let current = neuracle.get_data(data_proof);
        //     let current: u64 = current.parse().unwrap();

        //     let due_time = credit_data.due_time;

        //     assert!(due_time == 0 || due_time > current, "Your credit is overdue, please repay your loan first!");

        //     let (_, allowance) = credit_service.get_revolving_credit_amount(id_proof.clone(), credit_sbt.clone());
            
        //     assert!(allowance >= amount, "Your current credit allowance is not enough!");

        //     let increase_debt_interest = amount * interest_rate;

        //     credit_service.update_debt(credit_sbt.clone(), self.controller_badge.create_proof(), credit_data.current_debt + amount, credit_data.debt_interest + increase_debt_interest, Decimal::ZERO);

        //     if due_time == 0 {
        //         credit_service.update_debt_time(credit_sbt.clone(), self.controller_badge.create_proof(), current + time, current);
                
        //     } else {
                
        //     }

        //     credit_sbt.drop(); id_proof.drop();

        //     info!("You have taken a {} loan from your credit", amount);

        //     self.vault.take(amount)
            
        // }

        // /// This method is for the permissioned borrower to take their installment loan.
        // /// ### Input: 
        // /// - id_proof: the Identity SBT proof.
        // /// - credit_sbt: the Credit SBT proof.
        // /// - installment_credit_badge: the Installment Credit Badge.
        // /// ### Output: 
        // /// All the borrower's installment loan.
        // pub fn installment_credit(&mut self, id_proof: Proof, credit_sbt: Proof, installment_credit_badge: Bucket) -> Bucket {

        //     let credit_service: GroundCredit = self.credit_service.into();

        //     assert!(self.vault.amount() / self.total_return > self.tolerance_threshold, "Currently you cannot take your credit from this protocol, please come back later.");

        //     let neuracle: NeuRacle = self.oracle.0.into();
        //     let data_proof = self.oracle.1.create_proof();
        //     let current = neuracle.get_data(data_proof);
        //     let current: u64 = current.parse().unwrap();

        //     let amount = credit_service.use_installment_credit(id_proof, credit_sbt, self.controller_badge.create_proof(), installment_credit_badge, current);

        //     info!("You have taken a {} loan from your installment credit.", amount);

        //     self.vault.take(amount)

        // }

        // /// This method is for the permissioned borrower to repay their loan.
        // /// ### Input: 
        // /// - id_proof: the Identity SBT proof.
        // /// - credit_sbt: the Credit SBT proof.
        // /// - repayment: the repayment stablecoin bucket.
        // /// ### Output: 
        // /// Remainder of borrower stablecoin bucket.
        // /// 
        // /// From this method, when the borrower repaid their debt in full, 
        // /// the method will automatically increase lender's lending amount if they're eligible for the interest 
        // /// (the borrow is made after lender's create the lending account).
        // /// 
        // /// Borrower can also make a period installment repayment in advance, their credit data will automatically updated through the method.
        // pub fn repay(&mut self, id_proof: Proof, credit_proof: Proof, mut repayment: Bucket) -> Bucket {

        //     assert!(repayment.resource_address() == self.vault.resource_address(), "Wrong resource.");

        //     let credit_service: GroundCredit = self.credit_service.into();

        //     credit_service.check_id_and_credit(id_proof.clone(), credit_proof.clone());

        //     let credit_data = credit_proof.non_fungible::<Credit>().data();

        //     match credit_data.credit_type {

        //         CreditType::Installment(_) => {

        //             let mut total_repaid = Decimal::ZERO;

        //             let mut new_debt = Decimal::ZERO;

        //             let mut new_debt_interest = Decimal::ZERO;
                    
        //             let mut new_extra_debt = Decimal::ZERO;

        //             let mut amount = repayment.amount();

        //             while new_debt > Decimal::ZERO {

        //                 (new_debt, new_debt_interest, new_extra_debt) = self.get_total_debt(credit_proof.clone());

        //                 if new_debt + new_debt_interest <= amount {

        //                     self.vault.put(repayment.take(new_debt + new_debt_interest));

        //                     amount -= new_debt + new_debt_interest;

        //                     total_repaid += new_debt;

        //                     let debt_start = credit_data.current_debt_start_time;

        //                     let mut eligible_return = Decimal::ZERO;
                            
        //                     self.lenders.values().for_each(|lender| {
        //                         if lender.start_time < debt_start {
        //                             eligible_return += lender.lending_amount
        //                         }
        //                     });

        //                     let interest = Decimal::ONE + new_debt_interest / eligible_return;

        //                     self.lenders.values_mut().for_each(|lender| {
        //                         if &lender.start_time < &debt_start {
        //                             lender.lending_amount *= interest
        //                         }
        //                     });

        //                     if new_extra_debt != Decimal::ZERO {

        //                         if new_extra_debt <= amount {

        //                             self.deposit_fee(repayment.take(new_extra_debt));
        //                             new_extra_debt = Decimal::ZERO;
        //                             amount -= new_extra_debt;
        //                             total_repaid += new_extra_debt;
        //                             new_debt = credit_service.update_installment_credit_data(credit_proof.clone(), self.controller_badge.create_proof(), true);
                                    
    
        //                         } else {
    
        //                             self.deposit_fee(repayment.take(amount));
        //                             total_repaid += amount;
        //                             new_extra_debt -= amount;
        //                             break
    
        //                         }

        //                     } else {
        //                         new_debt = credit_service.update_installment_credit_data(credit_proof.clone(), self.controller_badge.create_proof(), false);
        //                     }
        
        //                 } else {
            
        //                     self.vault.put(repayment.take(amount));
        //                     total_repaid += amount;

        //                     if amount <= new_debt {
        //                         new_debt -= amount
        //                     } else {
        //                         let remain = amount - new_debt;
        //                         new_debt = Decimal::ONE;
        //                         new_debt_interest -= remain
        //                     }

        //                     break
            
        //                 };
                        
        //             }

        //             if new_debt + new_debt_interest + new_extra_debt > Decimal::ZERO {
        //                 credit_service.update_debt(credit_proof.clone(), self.controller_badge.create_proof(), new_debt, new_debt_interest, new_extra_debt);
        //                 info!("You have repaid {} token for the protocol.", total_repaid)
        //             }
        //         }

        //         CreditType::Revolving(_) => {

        //             let (current_debt, debt_interest, extra_debt) = self.get_total_debt(credit_proof.clone());

        //             let mut amount = repayment.amount();

        //             let (new_debt, new_debt_interest, new_extra_debt, total_repaid) = if current_debt + debt_interest <= amount {

        //                 if current_debt + debt_interest != Decimal::ZERO {
                            
        //                     self.vault.put(repayment.take(current_debt + debt_interest));

        //                     amount -= current_debt + debt_interest;

        //                     let debt_start = credit_data.current_debt_start_time;

        //                     let mut eligible_return = Decimal::ZERO;
                            
        //                     self.lenders.values().for_each(|lender| {
        //                         if lender.start_time < debt_start {
        //                             eligible_return += lender.lending_amount
        //                         }
        //                     });

        //                     let interest = Decimal::ONE + debt_interest / eligible_return;

        //                     self.lenders.values_mut().for_each(|lender| {
        //                         if &lender.start_time < &debt_start {
        //                             lender.lending_amount *= interest
        //                         }
        //                     });
        //                 } 

        //                 if extra_debt != Decimal::ZERO {

        //                     if extra_debt <= amount {

        //                         self.deposit_fee(repayment.take(extra_debt));
        //                         (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, current_debt + debt_interest + extra_debt)
    
        //                     } else {
    
        //                         self.deposit_fee(repayment.take(amount));
        //                         (Decimal::ZERO, Decimal::ZERO, extra_debt - amount, current_debt + debt_interest + amount)
    
        //                     }
                            
        //                 } else {
        //                     credit_service.update_revolving_credit_repaid_accumulate(id_proof.clone(), credit_proof.clone(), self.controller_badge.create_proof(), current_debt + debt_interest);
        //                     (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, current_debt + debt_interest)
        //                 }
    
        //             } else {
        
        //                 self.vault.put(repayment.take(amount));

        //                 if extra_debt == Decimal::ZERO {
        //                     credit_service.update_revolving_credit_repaid_accumulate(id_proof.clone(), credit_proof.clone(), self.controller_badge.create_proof(), amount);
        //                 }

        //                 if amount <= current_debt {
        //                     (current_debt - amount, debt_interest, extra_debt, amount)
        //                 } else {
        //                     let remain = amount - current_debt;
        //                     (Decimal::ZERO, debt_interest - remain, extra_debt, amount)
        //                 }
        
        //             };

        //             credit_service.update_debt(credit_proof.clone(), self.controller_badge.create_proof(), new_debt, new_debt_interest, new_extra_debt);

        //             if new_debt + new_debt_interest + new_extra_debt == Decimal::ZERO {
                        
        //                 credit_service.update_debt_time(credit_proof.clone(), self.controller_badge.create_proof(), 0, 0);
        //                 info!("You have repaid all your current debt.")

        //             } else {

        //                 info!("You have repaid {} token for the protocol.", total_repaid)

        //             }
        //         }
        //     } 

        //     id_proof.drop(); credit_proof.drop();

        //     return repayment

        // }

        // /// This method is for the permissioned borrower to get their total debt.
        // /// ### Input: 
        // /// - credit_sbt: the Credit SBT proof.
        // /// ### Output: 
        // /// The borrower's initial debt, debt interest, and extra debt from late repayment.
        // /// 
        // /// From this method, if the protocol see the borrower are late on repayment for the first time, 
        // /// the protocol will automatically degrade the borrower's credit score, 
        // /// calculate the extra debt from late repayment and update borrower's debt data.
        // pub fn get_total_debt(&self, credit_proof: Proof) -> (Decimal, Decimal, Decimal) {

        //     let data = credit_proof.non_fungible::<Credit>().data();

        //     let neuracle: NeuRacle = self.oracle.0.into();
        //     let data_proof = self.oracle.1.create_proof();
        //     let current = neuracle.get_data(data_proof);
        //     let current: u64 = current.parse().expect("Wrong data!");

        //     let due_time = data.due_time;

        //     let current_debt = data.current_debt;

        //     let mut extra_debt = data.extra_debt;
            
        //     let debt_interest = data.debt_interest;

        //     assert!(current_debt + debt_interest + extra_debt != Decimal::ZERO, "You currently don't have any debt!");

        //     if due_time <= current && extra_debt == Decimal::ZERO {

        //         let credit_service: GroundCredit = self.credit_service.into();

        //         credit_service.degrade_credit(credit_proof.clone(), self.controller_badge.create_proof());

        //         extra_debt = match data.credit_type {

        //             CreditType::Revolving(types) => {

        //                 match types {

        //                     RevolvingTypes::Monthly => {

        //                         let number = (Decimal::from(current - due_time)  / Decimal::from(MONTH)).ceiling().to_string().parse().expect("Cannot parse Decimal to u8");
        //                         let rate = self.interest_rates.monthly.interest_rate_late;
        //                         let mutiply = expo(rate, number);
        //                         current_debt * (mutiply - Decimal::ONE)

        //                     }

        //                     RevolvingTypes::Yearly => {
        //                         let number = (Decimal::from(current - due_time)  / Decimal::from(YEAR)).ceiling().to_string().parse().expect("Cannot parse Decimal to u8");
        //                         let rate = dec!("1") + self.interest_rates.yearly.interest_rate_late;
        //                         let mutiply = expo(rate, number);
        //                         current_debt * (mutiply - Decimal::ONE)
        //                     }

        //                 }
        //             }

        //             CreditType::Installment(data) => {

        //                 let number = (Decimal::from(current - due_time) / Decimal::from(data.period_length)).ceiling().to_string().parse().expect("Cannot parse Decimal to u8");
        //                     let rate = data.interest_rate_late;
        //                     let mutiply = expo(rate, number);
        //                     current_debt * (mutiply - Decimal::ONE)

        //             }
        //         };

        //         credit_service.update_debt(credit_proof.clone(), self.controller_badge.create_proof(), current_debt, debt_interest, extra_debt);

        //     }

        //     credit_proof.drop();

        //     info!("Your current debt is {}.", current_debt + debt_interest + extra_debt);

        //     (current_debt, debt_interest, extra_debt)
        // }

        /// This method is for borrowers to take a loan from their revolving credit.
        /// ### Input: 
        /// - id_proof: the Identity SBT proof.
        /// - credit_sbt: the Credit SBT proof.
        /// - amount: the amount borrower wish to loan.
        /// ### Output: 
        /// borrower's loan by amount.
        pub fn revolving_credit(&mut self, id_proof: Proof, credit_sbt: Proof, amount: Decimal) -> Bucket {
            
            assert!(amount > Decimal::zero(), "Wrong data provided!");
            
            let credit_service: GroundCredit = self.credit_service.into();

            assert!(self.total_return != Decimal::ZERO, "Currently there's no resource on the protocol's vault, please come back later.");

            assert!((self.vault.amount() - amount) / self.total_return > self.tolerance_threshold, "Currently you cannot take your credit from this protocol, please come back later or try reducing your loan amount.");

            credit_service.full_check(id_proof.non_fungible::<Credit>().id(), id_proof.resource_address(), credit_sbt.non_fungible::<Credit>().id(), credit_sbt.resource_address());

            let credit_data = credit_sbt.non_fungible::<Credit>().data().data;

            let id_data = id_proof.non_fungible::<Identity>().data().data;

            let (interest_rate, time) = match credit_data.credit_type {

                CreditType::Revolving(revolving_types) => {
                    match revolving_types {
                        RevolvingTypes::Monthly => {(self.interest_rates.monthly.interest_rate, MONTH)}

                        RevolvingTypes::Yearly => {(self.interest_rates.yearly.interest_rate, YEAR)}
                    }
                }

                _ => {panic!("Wrong credit type!")}
            };

            let neuracle: NeuRacle = self.oracle.0.into();
            let data_proof = self.oracle.1.create_proof();
            let current = neuracle.get_data(data_proof);
            let current: u64 = current.parse().unwrap();

            let due_time = credit_data.due_time;

            assert!(due_time == 0 || due_time > current, "Your credit is overdue, please repay your loan first!");

            let (_, allowance) = credit_service.get_revolving_credit_amount_by_data(id_data, credit_data);
            
            assert!(allowance >= amount, "Out of credit, you have to repay your debt first!");

            let increase_debt_interest = amount * interest_rate;

            let credit_sbt = self.update_debt(credit_sbt, self.controller_badge.create_proof(), credit_data.current_debt + amount, credit_data.debt_interest + increase_debt_interest, Decimal::ZERO);

            let credit_sbt = if due_time == 0 {

                self.update_debt_time(credit_sbt, self.controller_badge.create_proof(), current + time, current)
                
            } else {

                credit_sbt
                
            };

            credit_sbt.drop(); id_proof.drop();

            info!("You have taken a {} stable coins loan from your credit", amount);

            self.vault.take(amount)
            
        }

        /// This method is for borrowers to take their installment loan after they got the installment credit badge.
        /// ### Input: 
        /// - id_proof: the Identity SBT proof.
        /// - credit_sbt: the Credit SBT proof.
        /// - installment_credit_badge: the Installment Credit Badge.
        /// ### Output: 
        /// All the borrower's installment loan.
        pub fn installment_credit(&mut self, id_proof: Proof, credit_sbt: Proof, installment_credit_badge: Bucket) -> Bucket {

            assert!(self.total_return != Decimal::ZERO, "Currently there's no resource on the protocol's vault, please come back later.");

            let credit_service: GroundCredit = self.credit_service.into();

            credit_service.full_check(id_proof.non_fungible::<Credit>().id(), id_proof.resource_address(), credit_sbt.non_fungible::<Credit>().id(), credit_sbt.resource_address());

            let neuracle: NeuRacle = self.oracle.0.into();
            let data_proof = self.oracle.1.create_proof();
            let current = neuracle.get_data(data_proof);
            let current: u64 = current.parse().unwrap();

            let amount = self.use_installment_credit(id_proof, credit_sbt, self.controller_badge.create_proof(), installment_credit_badge, current);

            assert!((self.vault.amount() - amount) / self.total_return > self.tolerance_threshold, "Currently you cannot take your credit from this protocol, please come back later.");

            info!("You have taken a {} stable coins loan from your installment credit.", amount);

            self.vault.take(amount)

        }

        /// This method is for borrowers to repay their loan.
        /// ### Input: 
        /// - id_proof: the Identity SBT proof.
        /// - credit_sbt: the Credit SBT proof.
        /// - repayment: the repayment stablecoin bucket.
        /// ### Output: 
        /// Remainder of borrower stablecoin bucket.
        /// 
        /// From this method, when the borrower begin repaid their interest, 
        /// the method will automatically increase lender's lending amount if they're eligible for the interest 
        /// (if the borrow is made after lenders create their lending account).
        /// 
        /// Borrower can also make a period installment repayment in advance, their credit data will automatically updated through the method.
        pub fn repay(&mut self, mut id_proof: Proof, mut credit_proof: Proof, mut repayment: Bucket) -> Bucket {

            assert!(repayment.resource_address() == self.vault.resource_address(), "Wrong resource.");

            let credit_service: GroundCredit = self.credit_service.into();

            credit_service.check_id_and_credit_by_data(id_proof.non_fungible::<Credit>().id(), id_proof.resource_address(), credit_proof.non_fungible::<Credit>().id(), credit_proof.resource_address());

            let credit_data = credit_proof.non_fungible::<Credit>().data().data;

            let credit_proof = match credit_data.credit_type {

                CreditType::Installment(_) => {

                    let mut total_repaid = Decimal::ZERO;

                    let mut amount = repayment.amount();

                    let (mut new_debt, mut new_debt_interest, mut new_extra_debt) = (Decimal::ONE, Decimal::ONE, Decimal::ONE);

                    while new_debt > Decimal::ZERO {

                        (new_debt, new_debt_interest, new_extra_debt, credit_proof) = self.get_total_debt(credit_proof);

                        if new_debt + new_debt_interest <= amount {

                            amount -= new_debt + new_debt_interest;

                            total_repaid += new_debt + new_debt_interest;

                            repayment = self.protocol_interest(self.controller_badge.create_proof(), repayment, new_debt, new_debt_interest, credit_data.current_debt_start_time);

                            if new_extra_debt != Decimal::ZERO {

                                if new_extra_debt <= amount {

                                    self.deposit_fee(repayment.take(new_extra_debt));
                                    new_extra_debt = Decimal::ZERO;
                                    amount -= new_extra_debt;
                                    total_repaid += new_extra_debt;
                                    (new_debt, new_debt_interest, credit_proof) = self.update_installment_credit_data(credit_proof, self.controller_badge.create_proof(), true);
                                    
    
                                } else {
    
                                    self.deposit_fee(repayment.take(amount));
                                    total_repaid += amount;
                                    new_extra_debt -= amount;
                                    break
    
                                }

                            } else {
                                (new_debt, new_debt_interest, credit_proof) = self.update_installment_credit_data(credit_proof, self.controller_badge.create_proof(), false);
                            }
        
                        } else if new_debt < amount {
            
                            self.vault.put(repayment.take(new_debt));

                            total_repaid += new_debt;

                            let remain = amount - new_debt;

                            repayment = self.protocol_interest(self.controller_badge.create_proof(), repayment, new_debt, remain, credit_data.current_debt_start_time);

                            new_debt = Decimal::ZERO;

                            new_debt_interest -= remain;

                            break
            
                        } else {
                            self.vault.put(repayment.take(amount));
                            total_repaid += amount;
                            new_debt -= amount;
                            break
                        }
                        
                    }

                    if new_debt + new_debt_interest + new_extra_debt > Decimal::ZERO {
                        credit_proof = self.update_debt(credit_proof, self.controller_badge.create_proof(), new_debt, new_debt_interest, new_extra_debt);
                        info!("You have repaid {} stable coins for the protocol.", total_repaid)
                    };

                    credit_proof
                }

                CreditType::Revolving(_) => {

                    let (current_debt, debt_interest, extra_debt, mut credit_proof) = self.get_total_debt(credit_proof);

                    let mut amount = repayment.amount();

                    let (new_debt, new_debt_interest, new_extra_debt, total_repaid) = if current_debt + debt_interest <= amount {

                        if current_debt + debt_interest != Decimal::ZERO {

                            amount -= current_debt + debt_interest;

                            repayment = self.protocol_interest(self.controller_badge.create_proof(), repayment, current_debt, debt_interest, credit_data.current_debt_start_time);

                        } 

                        if extra_debt != Decimal::ZERO {

                            if extra_debt <= amount {

                                self.deposit_fee(repayment.take(extra_debt));
                                (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, current_debt + debt_interest + extra_debt)
    
                            } else {
    
                                self.deposit_fee(repayment.take(amount));
                                (Decimal::ZERO, Decimal::ZERO, extra_debt - amount, current_debt + debt_interest + amount)
    
                            }
                            
                        } else {
                            (id_proof, credit_proof) = self.update_revolving_credit_repaid_accumulate(id_proof, credit_proof, self.controller_badge.create_proof(), current_debt + debt_interest);
                            (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, current_debt + debt_interest)
                        }
    
                    } else if current_debt < amount {

                        if extra_debt == Decimal::ZERO {
                            (id_proof, credit_proof) = self.update_revolving_credit_repaid_accumulate(id_proof, credit_proof, self.controller_badge.create_proof(), amount);
                        }

                        let remain = amount - current_debt;

                        repayment = self.protocol_interest(self.controller_badge.create_proof(), repayment, current_debt, remain, credit_data.current_debt_start_time);

                        (Decimal::ZERO, debt_interest - remain, extra_debt, amount)
                        
                    } else {
        
                        self.vault.put(repayment.take(amount));

                        if extra_debt == Decimal::ZERO {
                            (id_proof, credit_proof) = self.update_revolving_credit_repaid_accumulate(id_proof, credit_proof, self.controller_badge.create_proof(), amount);
                        }

                        (current_debt - amount, debt_interest, extra_debt, amount)
        
                    };

                    credit_proof = self.update_debt(credit_proof, self.controller_badge.create_proof(), new_debt, new_debt_interest, new_extra_debt);

                    if new_debt + new_debt_interest + new_extra_debt == Decimal::ZERO {
                        
                        credit_proof = self.update_debt_time(credit_proof, self.controller_badge.create_proof(), 0, 0);
                        info!("You have repaid all your current debt.")

                    } else {

                        info!("You have repaid {} stable coins for the protocol.", total_repaid)

                    };

                    credit_proof
                }
            };

            id_proof.drop(); credit_proof.drop();

            return repayment

        }

        /// This method is for the permissioned borrower to get their total debt.
        /// ### Input: 
        /// - credit_sbt: the Credit SBT proof.
        /// ### Output: 
        /// The borrower's initial debt, debt interest, and extra debt from late repayment.
        /// 
        /// From this method, if the protocol see the borrower are late on repayment for the first time, 
        /// the protocol will automatically degrade the borrower's credit score, 
        /// calculate the extra debt from late repayment and update borrower's debt data.
        pub fn get_total_debt(&self, mut credit_proof: Proof) -> (Decimal, Decimal, Decimal, Proof) {

            let data = credit_proof.non_fungible::<Credit>().data().data;

            let neuracle: NeuRacle = self.oracle.0.into();
            let data_proof = self.oracle.1.create_proof();
            let current = neuracle.get_data(data_proof);
            let current: u64 = current.parse().expect("Wrong data!");

            let due_time = data.due_time;

            let current_debt = data.current_debt;

            let mut extra_debt = data.extra_debt;
            
            let debt_interest = data.debt_interest;

            assert!(current_debt + debt_interest + extra_debt != Decimal::ZERO, "You currently don't have any debt!");

            if due_time <= current && extra_debt == Decimal::ZERO {

                credit_proof = self.degrade_credit(credit_proof, self.controller_badge.create_proof());

                extra_debt = match data.credit_type {

                    CreditType::Revolving(types) => {

                        match types {

                            RevolvingTypes::Monthly => {

                                let number = (Decimal::from(current - due_time)  / Decimal::from(MONTH)).ceiling().to_string().parse().expect("Cannot parse Decimal to u8");
                                let rate = self.interest_rates.monthly.interest_rate_late;
                                let mutiply = power(rate, number);
                                current_debt * (mutiply - Decimal::ONE)

                            }

                            RevolvingTypes::Yearly => {
                                let number = (Decimal::from(current - due_time)  / Decimal::from(YEAR)).ceiling().to_string().parse().expect("Cannot parse Decimal to u8");
                                let rate = dec!("1") + self.interest_rates.yearly.interest_rate_late;
                                let mutiply = power(rate, number);
                                current_debt * (mutiply - Decimal::ONE)
                            }

                        }
                    }

                    CreditType::Installment(data) => {

                        let number = (Decimal::from(current - due_time) / Decimal::from(data.period_length)).ceiling().to_string().parse().expect("Cannot parse Decimal to u8");
                            let rate = data.interest_rate_late;
                            let mutiply = power(rate, number);
                            current_debt * (mutiply - Decimal::ONE)

                    }
                };

                credit_proof = self.update_debt(credit_proof, self.controller_badge.create_proof(), current_debt, debt_interest, extra_debt);

            } else {
                info!("Your current debt is {}.", current_debt + debt_interest + extra_debt);
            }
            
            (current_debt, debt_interest, extra_debt, credit_proof)
        }

        /// This method is to check the protocol controller proof.
        pub fn check_protocol(&self, protocol_proof: Proof) {
            assert!(protocol_proof.resource_address() == self.controller_badge.resource_address(), "This protocol is not allowed to use on-chain credit service.");
            protocol_proof.drop();
        }

        /// This method is for protocols to check and update user's installment credit data
        /// before user take the installment loan.
        ///
        /// The method can only be self called. (Unless Scrypto allow create a different resource with the same address as the protocol's controller badge).
        /// ### Input: 
        /// - id_sbt_proof: The Proof of the user's Credit SBT.
        /// - credit_proof: the Proof of the user's Credit SBT.
        /// - protocol_proof: The Proof of the protocol's controller badge.
        /// - installment_credit_badge: The Bucket of user's Installment Credit NFT.
        /// - current: Current time data fed in through the protocol. (unix)
        /// ### Output: 
        /// Return the installment credit amount.
        pub fn use_installment_credit(&self, id_sbt_proof: Proof, credit_proof: Proof, protocol_proof: Proof, installment_credit_badge: Bucket, current: u64) -> Decimal {

            self.check_protocol(protocol_proof);

            let credit = credit_proof.non_fungible::<Credit>();

            let data = credit.data().data;

            let id_sbt = id_sbt_proof.non_fungible::<Identity>().id();

            id_sbt_proof.drop(); 

            let credit_service: GroundCredit = self.credit_service.into();

            credit_service.check_installment_credit(installment_credit_badge.resource_address());

            let installment_data = installment_credit_badge.non_fungible::<InstallmentCredit>().data();

            let installment_id = installment_data.sbt_id;

            assert!(id_sbt == installment_id, "Wrong Installment Credit Badge provided.");

            assert!(data.due_time == 0, "You have to repay all your current debt first.");

            let mut installment_data = installment_data.data;

            installment_data.period_counter = 1;
            
            let current_debt = installment_data.total_loan / installment_data.period_max;

            let debt_interest = current_debt * installment_data.interest_rate;

            let due_time = current + installment_data.period_length;

            self.controller_badge.authorize(|| { 
                installment_credit_badge.burn();
                credit.update_data(Credit {
                    data: CreditData {
                        credit_type: CreditType::Installment(installment_data),
                        current_debt_start_time: current,
                        current_debt,
                        debt_interest,
                        due_time,
                        ..data
                    }
                })
            });

            info!("You have changed your credit type into Installment Credit. Your current debt is: {}. Your debt will over due in: {} (unix time)", current_debt + debt_interest, due_time);

            credit_proof.drop();

            installment_data.total_loan
                    
        }

        /// This method is for protocols to check and update user's installment credit data
        /// after user repaid the period loan.
        /// 
        /// The method can only be self called. (Unless Scrypto allow create a different resource with the same address as the protocol's controller badge).
        /// ### Input: 
        /// - credit_proof: the Proof of the user's Credit SBT.
        /// - protocol_proof: The Proof of the protocol's controller badge.
        /// - late: input if the repayment is late or not.
        /// ### Output:
        /// - Update the installment credit data based on user's repayment.
        /// - Return the next period debt amount. (include the origin debt and the interest)
        pub fn update_installment_credit_data(&self, credit_proof: Proof, protocol_proof: Proof, late: bool) -> (Decimal, Decimal, Proof) {

            self.check_protocol(protocol_proof);

            let credit_service: GroundCredit = self.credit_service.into();

            let credit_scoring_rates = credit_service.credit_scoring_rate();

            let credit = credit_proof.non_fungible::<Credit>();

            let mut data = credit.data().data;

            let mut installment_data = match data.credit_type {

                CreditType::Installment(data) => {data}
                _ => {panic!("Wrong credit type!")}

            };

            if !late {
                data.credit_score += credit_scoring_rates.monthly.restore_rate
            };

            if installment_data.period_counter < installment_data.period_max {

                installment_data.period_counter += 1;

                let current_debt = installment_data.total_loan / installment_data.period_max;

                let debt_interest = current_debt * installment_data.interest_rate;

                let period_length = installment_data.period_length;

                info!("You have repaid all the current period debt from your installment credit, your installment period will be advanced by 1. Current period: {}", installment_data.period_counter);

                data = CreditData {
                        credit_type: CreditType::Installment(installment_data),
                        current_debt_start_time: data.due_time,
                        current_debt,
                        debt_interest,
                        due_time: data.due_time + period_length,
                        ..data
                    };

            } else if installment_data.period_counter == installment_data.period_max {

                installment_data.period_counter += 1;

                data = CreditData {
                        credit_type: CreditType::Revolving(RevolvingTypes::Monthly),
                        current_debt_start_time: 0,
                        current_debt: Decimal::ZERO,
                        debt_interest: Decimal::ZERO,
                        due_time: 0,
                        ..data
                    };

                info!("You have repaid all your installment loan, your credit will change into the Monthly Revolving Credit");

            } else {panic!("You have already repaid all the installment credit debt. Please consider changing your credit type into revolving credit.")};

            self.controller_badge.authorize(|| { 
                credit.update_data(Credit {
                    data: CreditData {..data}
                })
            });

            return (data.current_debt, data.debt_interest, credit_proof)

        }

        /// This method is for lending protocol to update the debt amount of an user.
        /// 
        /// The method can only be self called. (Unless Scrypto allow create a different resource with the same address as the protocol's controller badge).
        /// ### Input: 
        /// - credit_proof: the user's credit proof.
        /// - protocol_proof: the protocol controller's proof.
        /// - current_debt: new debt amount.
        /// - extra_debt: new extra debt amount.
        /// ### Output: 
        /// update the credit debt amount.
        pub fn update_debt(&self, credit_proof: Proof, protocol_proof: Proof, current_debt: Decimal, debt_interest: Decimal, extra_debt: Decimal) -> Proof {

                self.check_protocol(protocol_proof);
                let credit = credit_proof.non_fungible::<Credit>();
                let data = credit.data().data;
    
                self.controller_badge.authorize(|| {
                    credit.update_data(
                        Credit {
                            data: CreditData {
                                current_debt,
                                debt_interest,
                                extra_debt,
                                ..data
                            }
                        }
                    )
                });

                info!("Your current debt is {}", current_debt + debt_interest + extra_debt);

                credit_proof
    
        }

        /// This method is for lending protocol to update credit due time of an user.
        /// 
        /// The method can only be self called. (Unless Scrypto allow create a different resource with the same address as the protocol's controller badge).
        /// ### Input: 
        /// - credit_proof: the user's credit proof.
        /// - protocol_proof: the protocol controller's proof.
        /// - due_time: new credit loan due time
        /// - current_debt_start_time: new credit loan start time.
        /// ### Output: 
        /// update the user's credit due time
        pub fn update_debt_time(&self, credit_proof: Proof, protocol_proof: Proof, due_time: u64, current_debt_start_time: u64) -> Proof {

                self.check_protocol(protocol_proof);
                let credit = credit_proof.non_fungible::<Credit>();
                let data = credit.data().data;
                self.controller_badge.authorize(|| {
                    credit.update_data(
                        Credit {
                            data: CreditData {
                                due_time,
                                current_debt_start_time,
                                ..data
                            }
                        }
                    )
                });

                if due_time != 0 {info!("Your debt will be over due in {} (unix time)", due_time);}

                credit_proof
    
        }

        /// This method is for lending protocol to degrade credit score of an user.
        /// 
        /// The method can only be self called. (Unless Scrypto allow create a different resource with the same address as the protocol's controller badge).
        /// ### Input: 
        /// - credit_proof: the user's credit proof.
        /// - protocol_proof: the protocol controller's proof.
        /// ### Output: 
        /// degrade the credit score
        pub fn degrade_credit(&self, credit_proof: Proof, protocol_proof: Proof) -> Proof {

            self.check_protocol(protocol_proof);
            let credit = credit_proof.non_fungible::<Credit>();
            let old_data = credit.data().data;

            let credit_service: GroundCredit = self.credit_service.into();

            let credit_scoring_rates = credit_service.credit_scoring_rate();

            let score = old_data.credit_score;

            let degrade_rate = match old_data.credit_type {

                CreditType::Revolving(ref types) => {

                    match types {

                        RevolvingTypes::Monthly => {credit_scoring_rates.monthly.degrade_rate}
                        RevolvingTypes::Yearly => {credit_scoring_rates.yearly.degrade_rate}

                    }
                    
                }

                _ => {credit_scoring_rates.monthly.degrade_rate}

            };

            let new_score = if score >= degrade_rate {
                score - degrade_rate
            } else { Decimal::zero() };

            self.controller_badge
            .authorize(|| { 
                credit.update_data(
                    Credit {
                        data: CreditData {
                            credit_score: new_score,
                            ..old_data
                        }
                    }
                )
            });

            info!("Your credit score has been degraded to {} because of late repayment", new_score);

            credit_proof

        }
    
        /// This method is for lending protocol to update repaid data and restore credit score of an user (if passed the maximum credit amount).
        /// 
        /// The method can only be self called. (Unless Scrypto allow create a different resource with the same address as the protocol's controller badge).
        /// ### Input: 
        /// - id_proof: the user's ID proof.
        /// - credit_proof: the user's credit proof.
        /// - protocol_proof: the protocol controller's proof.
        /// - amount: token amount the user has repaid.
        /// ### Output: 
        /// update repaid data and restore the credit score (if passed the maximum credit amount).
        pub fn update_revolving_credit_repaid_accumulate(&self, id_proof: Proof, credit_proof: Proof, protocol_proof: Proof, amount: Decimal) -> (Proof, Proof) {

            self.check_protocol(protocol_proof);

            let credit_service: GroundCredit = self.credit_service.into();

            let credit_scoring_rates = credit_service.credit_scoring_rate();

            let id_data = id_proof.non_fungible::<Identity>().data().data;

            let credit = credit_proof.non_fungible::<Credit>();

            let old_data = credit.data().data;

            let credit_service: GroundCredit = self.credit_service.into();

            let (maximum, _) = credit_service.get_revolving_credit_amount_by_data(id_data, old_data);

            let new_accumulated = old_data.repaid_amount_accumulated + amount;

            if new_accumulated >= maximum {

                let restore_rate = match old_data.credit_type {
                    CreditType::Revolving(ref types) => {

                        match types {
    
                            RevolvingTypes::Monthly => {credit_scoring_rates.monthly.restore_rate}
                            RevolvingTypes::Yearly => {credit_scoring_rates.yearly.restore_rate}
    
                        }
                        
                    }

                    _ => {panic!("Wrong credit type!")}
    
                };

                let score = old_data.credit_score;

                let new_score = if score <= (dec!("100") - restore_rate) {
                    score + restore_rate
                } else { dec!("100") };

                self.controller_badge
                .authorize(|| { 
                    credit.update_data(
                        Credit {
                            data: CreditData {
                                credit_score: new_score,
                                repaid_amount_accumulated: Decimal::zero(),
                                ..old_data
                            }
                        }
                    )
                });

                info!("Your credit score has been restored to {} because of your on-time repayment frequency", new_score);

            } else {
                self.controller_badge
                .authorize(|| { 
                    credit.update_data(
                        Credit {
                            data: CreditData {
                                repaid_amount_accumulated: new_accumulated,
                                ..old_data
                            }
                        }
                    )
                });
            }

            (id_proof, credit_proof)
            
        }

        /// This method is for lending protocol to update lender's interest based on their current account data stored on the component.
        /// 
        /// The method can only be self called. (Unless Scrypto allow create a different resource with the same address as the protocol's controller badge).
        pub fn protocol_interest(&mut self, protocol_proof: Proof, mut repayment: Bucket, current_debt: Decimal, interest: Decimal, debt_start: u64) -> Bucket {

            self.check_protocol(protocol_proof);

            let mut eligible_return = Decimal::ZERO;
                            
                for lender in self.lenders.values() {
                    if lender.start_time <= debt_start {
                        eligible_return += lender.lending_amount
                    }
                };

                let mut interest_rate = Decimal::ONE;

                if eligible_return != Decimal::ZERO {

                    self.total_return += interest;

                    self.vault.put(repayment.take(current_debt + interest));

                    interest_rate += interest / eligible_return;

                    for lender in self.lenders.values_mut() {

                        let start_time = lender.start_time;

                        if start_time <= debt_start {

                            lender.interest(interest_rate)

                        };

                    };
                } else {
                    self.vault.put(repayment.take(current_debt));
                    self.deposit_fee(repayment.take(interest));
                }

            return repayment

        }

        /// This method is for the protocol operator to make this protocol run by a DAO
        pub fn use_dao(&mut self, dao: ComponentAddress) {
            self.dao = Some(dao);
        }

        /// This method is to deposit the protocol fee into the fee vault or directly into the DAO treasury.
        pub fn deposit_fee(&mut self, fee: Bucket) {
            match self.dao {
                None => {self.fee_vault.put(fee)}
                Some(dao) => {
                    let dao: GroundBusinessDAO = dao.into();
                    dao.deposit(fee)
                }
            }
        }

        /// This method is for any volunteer or the DAO to deposit a bucket into the protocol's vault to support the protocol in case of loan default.
        pub fn deposit(&mut self, bucket: Bucket) {
            self.vault.put(bucket)
        }

        /// This method is for lenders to take their compensation from the DAO treasury in the worst case of cooperated loan default.
        pub fn compensate(&mut self, lender_bucket: Bucket) -> Bucket {

            match self.dao {
                None => {panic!("The protocol isn't run by a DAO, please consider contact the protocol operator!")}
                Some(dao) => {

                    let dao: GroundBusinessDAO = dao.into();

                    assert!(lender_bucket.resource_address() == self.account_nft, "Wrong resource");

                    let lender_accounts = lender_bucket.non_fungibles::<Account>();

                    let mut actual_return = Decimal::ZERO;

                    for account in lender_accounts {

                        let amount = self.lenders.remove(&account.id()).unwrap().lending_amount;

                        actual_return += amount;

                    }

                    let compensate = actual_return * self.compensate_rate;

                    self.total_return -= actual_return;

                    self.controller_badge.authorize(|| {
                        lender_bucket.burn()
                    });

                    let bucket = dao.compensate(self.controller_badge.create_proof(), compensate);

                    info!("You have been compensated {} stable coins from the DAO operated the protocol.", compensate);

                    bucket
                }
            }
        }

        /// This method is for the protocol withdraw the extra resource from protocol's vault.
        /// 
        /// If after some compensate but the borrowers still repay their loan, there will be extra resources on the vault.
        /// 
        /// For now, lenders can take compensate but the DAO run the protocol cannot take the extra resources on the vault
        /// because there has been none practice of feeding a transaction manifest into scrypto (can only feeding methods).
        /// 
        /// So, even if we run a concept from the DAO to call this method, there is no way to take the Bucket from the Component WorkTop yet,
        /// thus might result in ResourceCheckFailure error because of dangling bucket.
        pub fn withdraw_extra(&mut self) -> Bucket {
            assert!(self.vault.amount() > self.total_return, "The protocol's has no extra resource");
            let amount = self.vault.amount() - self.total_return;
            info!("You have withdrawed {} protocol's extra stable coins", amount);
            self.vault.take(amount)
        }

        pub fn change_interest_rates(&mut self, mut interest_rates: RevolvingCreditInterestRates) {
            interest_rates.check_rates();
            interest_rates.rates_aggregrate();
            self.interest_rates = interest_rates
        }

        pub fn change_fee(&mut self, fee: Decimal) {
            assert_rate(fee);
            self.fee = fee / dec!("100")
        }

        pub fn change_tolerance_threshold(&mut self, tolerance_threshold: Decimal) {
            assert_rate(tolerance_threshold);
            self.tolerance_threshold = tolerance_threshold / dec!("100")
        }

        pub fn change_compensate_rate(&mut self, compensate_rate: Decimal) {
            assert_rate(compensate_rate);
            self.compensate_rate = compensate_rate / dec!("100")
        }

        pub fn withdraw_fee(&mut self) -> Bucket {
            self.fee_vault.take_all()
        }
    }
}