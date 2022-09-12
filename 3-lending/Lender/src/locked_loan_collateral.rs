use scrypto::prelude::*;
use std::cmp;

/// A loan contract is a record of the terms of a loan, including the start date, duration, installment
/// frequency, interest rate, late payment penalty rate, amount per installment, amount borrowed, and
/// collateral evaluation.
/// 
/// Properties:
/// 
/// * `start_date`: The date when the loan contract was created.
/// * `duration`: The duration of the loan in epochs.
/// * `installment_frequency`: The frequency of the installments. For example, if the frequency is 1,
/// then the installments are paid every epoch. If the frequency is 2, then the installments are paid
/// every 2 epochs.
/// * `interest_rate`: The interest rate per epoch.
/// * `late_payment_penalty_rate`: The penalty rate per epoch when the installment is overdue.
/// * `amount_per_installment`: The amount that the borrower has to pay per installment.
/// * `amount_borrowed`: The amount of money the borrower wants to borrow.
/// * `collateral_evaluation`: The value of the collateral at the time of the loan.
#[derive(Debug, Describe, Encode, Decode, TypeId, Clone)]
pub struct LoanContract
{ 
    start_date : u64,
    duration : u64, // in epochs
    installment_frequency : u64, // in epochs
    interest_rate : Decimal, // per epoch
    late_payment_penalty_rate : Decimal, // per epoch when installment is overdue
    amount_per_installment : Decimal, 
    amount_borrowed : Decimal,
    collateral_evaluation : Decimal
}

// cannot be modified once created
impl LoanContract {
    pub fn create(start_date : u64, duration : u64, installment_frequency : u64, 
        interest_rate : Decimal, late_payment_penalty_rate : Decimal, amount_per_installment : Decimal, 
        amount_borrowed : Decimal, collateral_evaluation : Decimal) -> LoanContract {
        return LoanContract {
            start_date, duration, installment_frequency, interest_rate,late_payment_penalty_rate, amount_per_installment, 
            amount_borrowed, collateral_evaluation
        }
    }

    pub fn get_start_date(&self) -> u64 {
        return self.start_date
    }
    pub fn get_duration(&self) -> u64 {
        return self.duration
    }
    pub fn get_installment_frequency(&self) -> u64 {
        return self.installment_frequency
    }
    pub fn get_interest_rate(&self) -> Decimal {
        return self.interest_rate
    }
    pub fn get_late_payment_penalty_rate(&self) -> Decimal {
        return self.late_payment_penalty_rate
    }
    pub fn get_amount_per_installment(&self) -> Decimal {
        return self.amount_per_installment
    }
    #[allow(dead_code)]
    pub fn get_amount_borrowed(&self) -> Decimal {
        return self.amount_borrowed
    }
    pub fn get_collateral_evaluation(&self) -> Decimal {
        return self.collateral_evaluation
    }
}


/// A LoanRefundStatus is a struct that keeps track of the payments made by the borrower
/// 
/// Properties:
/// 
/// * `unlock_threshold`: The amount of tokens that need to be refunded before the loan is unlocked.
/// * `refunded_tokens`: The amount of tokens that have been refunded to the lender.
/// * `last_update`: The last time the loan status was updated.
#[derive(Debug, Describe, Encode, Decode, TypeId, Clone)]
pub struct LoanRefundStatus
{
    pub unlock_threshold : Decimal,
    pub refunded_tokens : Decimal,
    pub last_update : u64,
}

#[derive(NonFungibleData)]
pub struct NFTLoan
{ 
    pub loan_contract : LoanContract,
    #[scrypto(mutable)]
    pub loan_refund_status : LoanRefundStatus
}

// LockedLoanCollateral is a structure that encapsulated the logic and calculation for repaying loans and keeps the deposit locked until the amount required for unlock is met
// The threshold is computed based on the interest rate and is adapted each time a new deposit is made based on early/late installment payments
// If the borrower didn't manage to restore the threshold amount until the installment date, then he will have to pay penalty fees 
// to account for the additional time the lender is without his funds
// Because we have collateral we increase the debt even more as a penalty method if the borrower doesn't meet his deadlines until the loan passes its deadline
// and the collateral evaluation is less than the remaining debt. We liquidate the loan when the borrower does not have any more reasons to continue repaying because the 
// amount he must repay is larger than the collateral
/// 
/// Properties:
/// 
/// * `auth_vault`: Vault - the vault that contains the authority token for the borrower
/// * `collateral`: The vault that contains the collateral that the borrower deposited.
/// * `loan_contract`: LoanContract - the loan contract that contains the loan data
/// * `loan_refund_status`: LoanRefundStatus - the status of the loan refund
/// * `lender_badge_resource`: The address of the lender's badge resource.
/// * `borrower_nft_resource`: ResourceAddress - the vault can only be unlocked with the badge if all
/// the money were restored or if the loan expired
/// * `loan_contract_nft_id`: The ID of the loan NFT.
#[derive(Debug, Describe, Encode, Decode, TypeId)]
pub struct LockedLoanCollateral
{
    auth_vault: Vault,
    collateral : Vault,
    loan_contract : LoanContract,
    loan_refund_status : LoanRefundStatus,
    lender_badge_resource : ResourceAddress,
    borrower_nft_resource : ResourceAddress,
    loan_contract_nft_id : NonFungibleId
}

impl LockedLoanCollateral 
{
    pub fn instantiate(client_collateral: Bucket, amount_borrowed : Decimal, custom_loan_contract : LoanContract, loan_id : NonFungibleId, lender_badge : ResourceAddress) -> (Self, Bucket)
    {
        assert!(client_collateral.amount() > Decimal::zero(), "Deposit provided is empty");
        assert!(custom_loan_contract.get_interest_rate() > Decimal::zero(), "No lender badge provided");
        assert!(custom_loan_contract.get_late_payment_penalty_rate() > Decimal::zero(), "No lender badge provided");

        let auth_token = ResourceBuilder::new_fungible()
        .divisibility(DIVISIBILITY_NONE)
        .metadata("name", "Admin authority for loaning")
        .burnable(rule!(allow_all), LOCKED)
        .initial_supply(Decimal::one());

        // Create an NFT resource 
        let borrower_badge_resource = ResourceBuilder::new_non_fungible()
        .mintable(rule!(require(auth_token.resource_address())), LOCKED) // only the owner can mint new tickets
        .burnable(rule!(allow_all), LOCKED) // only the owner can mint new tickets
        .metadata("LoanBorrowerBadge", "Contains loan data")
        .updateable_non_fungible_data(rule!(require(lender_badge)), LOCKED) // nobody can change the lottery numbers once created
        .no_initial_supply();

        let max_tokens_for_unlock = amount_borrowed + amount_borrowed * custom_loan_contract.get_interest_rate();

        let borrower_nft = auth_token.authorize(|| {
            borrow_resource_manager!(borrower_badge_resource)
                .mint_non_fungible(&loan_id, NFTLoan { 
                    loan_contract : custom_loan_contract.clone(),
                    loan_refund_status : LoanRefundStatus { unlock_threshold: max_tokens_for_unlock, refunded_tokens: Decimal::zero(), last_update : custom_loan_contract.get_start_date() }
                })
        });

        let component = Self {
            collateral: Vault::with_bucket(client_collateral),
            auth_vault : Vault::with_bucket(auth_token),
            lender_badge_resource : lender_badge,
            loan_contract: custom_loan_contract.clone(),
            loan_refund_status : LoanRefundStatus { unlock_threshold: max_tokens_for_unlock, refunded_tokens: Decimal::zero(), last_update : custom_loan_contract.get_start_date() },
            borrower_nft_resource: borrower_badge_resource,
            loan_contract_nft_id : loan_id
        };
        
        return (component, borrower_nft)
    }
    
    fn get_num_installments_to_date(&self) -> u64 {
        let duration = cmp::min( scrypto::prelude::Runtime::current_epoch() - self.loan_contract.get_start_date(), self.loan_contract.get_duration());
        return duration / self.loan_contract.get_installment_frequency()
    }

    /// It calculates the penalty for late payments by iterating over all the installments and adding the
    /// penalty for each installment that is late
    /// 
    /// Arguments:
    /// 
    /// * `current_epoch`: the current epoch number
    /// 
    /// Returns:
    /// 
    /// The amount of tokens that the borrower has to pay to the lender for being late in order to pay the loan.
    fn get_late_installment_penalty(&self, current_epoch : u64) -> Decimal
    {
        let mut sum_epochs_late = Decimal::zero();

        let epochs_since_last_update = current_epoch - self.loan_refund_status.last_update;

        for i in 0..self.get_num_installments_to_date() {
            if self.loan_refund_status.refunded_tokens >= self.loan_contract.get_amount_per_installment() * Decimal::from(i) {
                continue;
            }

            let cur_installment_end_epoch = self.loan_contract.get_start_date() + i * self.loan_contract.get_installment_frequency();
            sum_epochs_late += Decimal::from(current_epoch - cur_installment_end_epoch);
            // if we made a payment ealier and we already computed the amount lost we use the epochs_since_last_update so that we don't apply penaly for them
            if cur_installment_end_epoch < current_epoch -  epochs_since_last_update {
                sum_epochs_late -= current_epoch - cur_installment_end_epoch - epochs_since_last_update;
            }
        }

        // TODO: if a borrower payed half of an installment amount and he is late, we should apply the penalty only for that half and not for the whole amount
        return sum_epochs_late * self.loan_contract.get_late_payment_penalty_rate() * self.loan_contract.get_amount_per_installment();
    }

    /// It calculates the amount of tokens that the user saved by paying installments in advance (the unlock threshold will be reduced)
    /// 
    /// Arguments:
    /// 
    /// * `current_epoch`: the current epoch
    /// * `num_deposited_tokens`: the amount of tokens deposited by the user
    /// 
    /// Returns:
    /// 
    /// The amount of tokens that the user saved by paying installments in advance.
    fn get_advance_payment_savings(&self, current_epoch : u64, num_deposited_tokens : Decimal) -> Decimal {
        if current_epoch - self.loan_contract.get_start_date() > self.loan_contract.get_duration() {
            return Decimal::zero()
        }

        let nb_repayed_installments = (self.loan_refund_status.refunded_tokens + num_deposited_tokens)  / self.loan_contract.get_amount_per_installment();
        let nb_installments_to_date = Decimal::from(self.get_num_installments_to_date());

        if nb_repayed_installments <= nb_installments_to_date {
            return Decimal::zero()
        }
        let nb_installments_already_refunded = num_deposited_tokens / self.loan_contract.get_amount_per_installment();
        let nb_installments_payed_in_advance = nb_installments_already_refunded - cmp::max(nb_installments_already_refunded, nb_installments_to_date);

        if nb_installments_payed_in_advance > Decimal::zero() {
            return nb_installments_payed_in_advance * Decimal::from(self.loan_contract.get_duration() - current_epoch) * self.loan_contract.get_interest_rate();
        }
        return Decimal::zero()
    }


    /// Function called when the borrower want to repay the loan
    /// It takes in a number of tokens deposited by the borrower, a proof of the lender's badge, and a proof
    /// of the loan NFT, and returns a tuple of the updated loan refund status and the number of tokens that
    /// overflowed the refund threshold
    /// 
    /// Arguments:
    /// 
    /// * `num_deposited_tokens`: The amount of tokens that the borrower is depositing to repay the loan.
    /// * `lender_auth`: Proof of the lender's badge
    /// * `loan_nft`: Proof of the loan NFT
    pub fn repay(&mut self, num_deposited_tokens : Decimal, lender_auth: Proof, loan_nft : Proof) -> (LoanRefundStatus, Decimal)
    {
        assert!(lender_auth.amount() > Decimal::zero() && lender_auth.resource_address() == self.lender_badge_resource, "invalid lender badge proof");
        assert!(loan_nft.amount() > Decimal::zero() && loan_nft.resource_address() == self.borrower_nft_resource, "invalid loan nft");
        assert!(loan_nft.non_fungible::<NFTLoan>().id() == self.loan_contract_nft_id, "loan nft id does not match with the current loan id");

        let current_epoch : u64 = scrypto::prelude::Runtime::current_epoch();

        let mut deposited_tokens_overflow = Decimal::zero();
        if self.loan_refund_status.refunded_tokens + num_deposited_tokens > self.loan_refund_status.unlock_threshold {
            deposited_tokens_overflow = self.loan_refund_status.refunded_tokens + num_deposited_tokens - self.loan_refund_status.unlock_threshold;
        }
            
        // update the status of the refund
        self.loan_refund_status.unlock_threshold += self.get_late_installment_penalty(current_epoch) - self.get_advance_payment_savings(current_epoch, num_deposited_tokens);
        self.loan_refund_status.refunded_tokens += num_deposited_tokens - deposited_tokens_overflow;
        self.loan_refund_status.last_update = current_epoch; // we don't really need this but it makes things easier to calculate

        return (self.loan_refund_status.clone(), deposited_tokens_overflow)
    }
    
    /// If the loan is finished and the borrower didn't pay enough to unlock his collateral, then the lender
    /// can liquidate the collateral
    /// 
    /// Arguments:
    /// 
    /// * `lender_auth`: Proof - the proof of the lender's badge
    /// 
    /// Returns:
    /// 
    /// The liquidate function returns a Bucket.
    pub fn liquidate(&mut self, lender_auth: Proof) -> Bucket {
        assert!(lender_auth.amount() > Decimal::zero() && lender_auth.resource_address() == self.lender_badge_resource, "invalid lender badge proof");

        let current_epoch : u64 = scrypto::prelude::Runtime::current_epoch();
        if current_epoch - self.loan_contract.get_start_date() <= self.loan_contract.get_duration()  { // loan is not finished, cannot liquidate
            return Bucket::new(self.collateral.resource_address())
        }

        if self.loan_refund_status.refunded_tokens >= self.loan_refund_status.unlock_threshold { // the borrower paid enough tokens to unlock, cannot liquidate
            return Bucket::new(self.collateral.resource_address())
        }

        // if the client didn't pay enough to cover his collateral, then we liquidate it to cover for the losses
        // Even if the client is late to refund the loan, we should not liquidate instantly
        // He still has the right to refund everything given that he pays the late payment penalty fees
        // besides the additional borrowed tokens and interest
        // For example a client uses an expensive NFT that gets evaluated at half the price
        // it is in his interest, even if he is late, to pay the fees in order to unlock the NFT
        if self.loan_refund_status.unlock_threshold - self.loan_refund_status.refunded_tokens >= self.loan_contract.get_collateral_evaluation() {
            return self.collateral.take_all()
        }
        return Bucket::new(self.collateral.resource_address()) // still in penalization period
    }
    
    /// If the loan is finished and the borrower repayed the borrowed amount + interest and
    /// penalization, then the borrower can unlock his collateral and get it back
    /// 
    /// Arguments:
    /// 
    /// * `lender_auth`: Proof of the lender's badge
    /// * `loan_nft`: Proof of the loan NFT
    /// 
    /// Returns:
    /// 
    /// The collateral is being returned to the lender.
    pub fn unlock_collateral(&mut self, lender_auth: Proof, loan_nft : Proof) -> Bucket
    {
        assert!(lender_auth.amount() > Decimal::zero() && lender_auth.resource_address() == self.lender_badge_resource, "invalid lender badge proof");
        assert!(loan_nft.amount() > Decimal::zero() && loan_nft.resource_address() == self.borrower_nft_resource, "invalid loan nft");
        assert!(loan_nft.non_fungible::<NFTLoan>().id() == self.loan_contract_nft_id, "loan nft id does not match with the current loan id");
        assert!(self.is_finished(), "loan not finished yet");
        // the unlock threshold for this loan was not met! Borrower did not repay the borrowed amount + interest and penalization
        if self.loan_refund_status.refunded_tokens >= self.loan_refund_status.unlock_threshold {
            return self.collateral.take_all()
        }

        return Bucket::new(self.collateral.resource_address());
    }

    /// If the loan has finished and the collateral evaluation is less than the amount of tokens
    /// that have been refunded, then the loan cannot be liquidated
    /// 
    /// Returns:
    /// 
    /// a boolean value.
    pub fn can_be_liquidated(&self) -> bool {
        let current_epoch : u64 = scrypto::prelude::Runtime::current_epoch();
        if current_epoch - self.loan_contract.get_start_date() <= self.loan_contract.get_duration()  { // loan is not finished yet
            return false
        }

        if self.loan_refund_status.unlock_threshold - self.loan_refund_status.refunded_tokens >= self.loan_contract.get_collateral_evaluation() {
            return true
        }

        return false
    }
    
    pub fn get_loan_contract(&self) -> LoanContract {
        return self.loan_contract.clone(); // return a copy of the loan data
    }


    pub fn get_id(&self) -> NonFungibleId { 
        return self.loan_contract_nft_id.clone();
    }

    pub fn is_finished(&self) -> bool {
        if self.loan_refund_status.refunded_tokens >= self.loan_refund_status.unlock_threshold {
            return true
        }

        return self.can_be_liquidated();
    }

}