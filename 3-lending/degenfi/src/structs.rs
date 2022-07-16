/// This module contains all the structs used in this protocol.
use scrypto::prelude::*;

/// User NFT is an NFT that represents users for this protocol. This NFT contains all the records of the user
/// interacting with this protocol. It can be seen as a credit report for the user. It is also used for authorization
/// that this user belongs to the protocol and access protocol features. Users themselves do not have permission to
/// change the data contained within the NFT. It is a non-transferable token or otherwise known as a "Soul Bound Token"
/// or "SBT" for short. The reason to contain deposit, collateral, and borrow balance as a HashMap is for better flexibility
/// and user experience. Especially when it comes to repaying loans. When a loan is paid off, users do not have to worry about
/// sending the wrong NFT, the protocol will simply look at the SBT token and find the loan that the user wants to pay off.
#[derive(NonFungibleData, Describe, Encode, Decode, TypeId)]
pub struct User {
    #[scrypto(mutable)]
    pub credit_score: u64,
    #[scrypto(mutable)]
    pub deposit_balance: HashMap<ResourceAddress, Decimal>,
    #[scrypto(mutable)]
    pub collateral_balance: HashMap<ResourceAddress, Decimal>,
    #[scrypto(mutable)]
    pub borrow_balance: HashMap<ResourceAddress, Decimal>,
    #[scrypto(mutable)]
    pub open_loans: HashMap<ResourceAddress, NonFungibleId>,
    #[scrypto(mutable)]
    pub closed_loans: HashMap<ResourceAddress, NonFungibleId>,
    #[scrypto(mutable)]
    pub defaults: u64,
    #[scrypto(mutable)]
    pub paid_off: u64,
}

/// This is an NFT that represents the loan terms. We can consider this NFT as loan documents and hopefully in the future can
/// be represented as legal documents or a digital representation of a legal document. This NFT is given to the borrower.
/// For now its purpose is to simply tract the health factor of the loan. If the loan is in bad health, liquidators can
/// query the liquidation component to evaluate bad loans and liquidate the loan's collateral. Another purpose is to track
/// the status of the loan to update the user's credit report. In the future, there may be interesting use cases that
/// we can explore to securitize the loans or package them together.
#[derive(NonFungibleData, Describe, Encode, Decode, TypeId)]
pub struct Loan {
    pub asset: ResourceAddress,
    pub collateral: ResourceAddress,
    pub principal_loan_amount: Decimal,
    pub interest_rate: Decimal,
    pub origination_fee: Decimal,
    pub origination_fee_charged: Decimal,
    pub owner: NonFungibleId,
    #[scrypto(mutable)]
    pub remaining_balance: Decimal,
    #[scrypto(mutable)]
    pub interest_expense: Decimal,
    #[scrypto(mutable)]
    pub last_update: u64,
    #[scrypto(mutable)]
    pub collateral_amount: Decimal,
    #[scrypto(mutable)]
    pub collateral_amount_usd: Decimal,
    #[scrypto(mutable)]
    pub health_factor: Decimal,
    #[scrypto(mutable)]
    pub loan_status: Status,
}

#[derive(TypeId, Encode, Decode, Describe, Debug, PartialEq)]
pub enum Status {
    PaidOff,
    Defaulted,
    Current,
}

#[derive(NonFungibleData, Debug)]
pub struct FlashLoan {
    pub amount_due: Decimal,
    pub asset: ResourceAddress,
    pub borrow_count: u8,
}