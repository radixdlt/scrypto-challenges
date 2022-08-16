use scrypto::prelude::*;

#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub struct User {
    pub funds_managed: HashMap<String, ComponentAddress>,
    pub funds_invested: HashMap<String, ComponentAddress>,
}

#[derive(Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub enum FundType {
    IndexFund,
    DebtFund,
}

/// This is an NFT that represents the loan terms. We can consider this NFT as loan documents and hopefully in the future can
/// be represented as legal documents or a digital representation of a legal document. This NFT is given to the borrower.
/// For now its purpose is to simply tract the health factor of the loan. If the loan is in bad health, liquidators can
/// query the liquidation component to evaluate bad loans and liquidate the loan's collateral. Another purpose is to track
/// the status of the loan to update the user's credit report. In the future, there may be interesting use cases that
/// we can explore to securitize the loans or package them together.
#[derive(NonFungibleData, Describe, Encode, Decode, TypeId)]
pub struct Loan {
    #[scrypto(mutable)]
    pub borrower_id: NonFungibleId,
    #[scrypto(mutable)]
    pub lender_id: NonFungibleId,
    #[scrypto(mutable)]
    pub principal_loan_amount: Decimal,
    pub asset: ResourceAddress,
    pub collateral: ResourceAddress,
    #[scrypto(mutable)]
    pub collateral_percent: Decimal,
    #[scrypto(mutable)]
    pub annualized_interest_rate: Decimal,
    pub term_length: u64,
    pub payment_frequency: PaymentFrequency,
    pub origination_fee: Decimal,
    pub origination_fee_charged: Decimal,
    #[scrypto(mutable)]
    pub annualized_interest_expense: Decimal,
    #[scrypto(mutable)]
    pub remaining_balance: Decimal,
    pub draw_limit: Decimal,
    pub draw_minimum: Decimal,
    #[scrypto(mutable)]
    pub last_draw: u64,
    #[scrypto(mutable)]
    pub collateral_amount: Decimal,
    #[scrypto(mutable)]
    pub collateral_amount_usd: Decimal,
    #[scrypto(mutable)]
    pub health_factor: Decimal,
    #[scrypto(mutable)]
    pub loan_status: Status,
}

#[derive(NonFungibleData, Describe, Encode, Decode, TypeId)]
pub struct FundingLockerAdmin {
}

#[derive(NonFungibleData, Describe, Encode, Decode, TypeId)]
pub struct DrawRequest {
    pub amount: Decimal,
}

/// Organize better
#[derive(TypeId, Encode, Decode, Describe, Debug, PartialEq)]
pub enum Status {
    AwaitingCollateral,
    ReadyToFund,
    Unfunded,
    Funded,
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

#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId)]
pub struct AuctionAuth {
    #[scrypto(mutable)]
    pub amount_due: Decimal,
    pub collateral_due: Decimal,
    pub collateral_address: ResourceAddress,
}

#[derive(TypeId, Encode, Decode, Describe, Debug, PartialEq)]
pub enum PaymentFrequency {
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(TypeId, Encode, Decode, Describe, Debug, PartialEq)]
pub enum RequestStatus {
    Pending,
    Approved,
    Modified,
}

#[derive(TypeId, Encode, Decode, Describe, Debug, PartialEq)]
pub enum BorrowerBadges {
    LoanRequestNFT,
    Borrower,
}

#[derive(PartialEq)]
pub enum BorrowerBadgeContainer {
    LoanRequestNFTContainer(LoanRequest),
    BorrowerContainer(Borrower),
}

#[derive(TypeId, Encode, Decode, Describe, Debug, PartialEq)]
pub enum Badges {
    FundManager,
    Investor,
    Borrower,
    TemporaryBadge,
    LoanRequestNFT,
}

pub enum BadgeContainer {
    FundManagerContainer(FundManager),
    InvestorContainer(Investor),
    BorrowerContainer(Borrower),
    TemporaryBadgeContainer(TemporaryBadge),
    LoanRequestNFTContainer(LoanRequest),
}

#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub struct FundManager {
    pub name: String,
    pub managed_index_funds: HashMap<(String, String), ComponentAddress>,
    pub managed_debt_funds: HashMap<ResourceAddress, ComponentAddress>,
}

#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub struct Investor {
    pub funds_invested: HashMap<ComponentAddress, Decimal>,
    pub liquidity_pools: HashMap<ComponentAddress, Decimal>,
}

#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub struct Borrower {
    pub name: String,
    pub loan_requests: BTreeSet<NonFungibleId>,
    pub loans: BTreeSet<NonFungibleId>,
}

#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub struct TemporaryBadge {
    pub name: String,
    #[scrypto(mutable)]
    pub user_type: UserType,
    #[scrypto(mutable)]
    pub status: RequestStatus,
}

#[derive(Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub enum UserType {
    FundManager,
    Borrower,
}


#[derive(NonFungibleData, Debug, Describe, Encode, Decode, TypeId, PartialEq)]
pub struct LoanRequest {
    #[scrypto(mutable)]
    pub asset: ResourceAddress,
    #[scrypto(mutable)]
    pub loan_amount: Decimal,
    #[scrypto(mutable)]
    pub collateral_address: ResourceAddress,
    #[scrypto(mutable)]
    pub collateral_percent: Decimal,
    #[scrypto(mutable)]
    pub term_length: u64,
    #[scrypto(mutable)]
    pub annualized_interest_rate: Decimal,
    #[scrypto(mutable)]
    pub payment_frequency: PaymentFrequency,
    #[scrypto(mutable)]
    pub borrower: NonFungibleId,
    #[scrypto(mutable)]
    pub status: RequestStatus,
    #[scrypto(mutable)]
    pub loan_nft_id: Option<NonFungibleId>,
    #[scrypto(mutable)]
    pub funding_locker_address: Option<ComponentAddress>,
}

