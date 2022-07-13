use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct CollateralDebtPosition{
    pub borrow_token: ResourceAddress,
    pub collateral_token: ResourceAddress,
    
    #[scrypto(mutable)]
    pub total_borrow: Decimal,
    #[scrypto(mutable)]
    pub total_repay: Decimal,
    
    #[scrypto(mutable)]
    pub normalized_borrow: Decimal,
    #[scrypto(mutable)]
    pub collateral_amount: Decimal,
    #[scrypto(mutable)]
    pub borrow_amount: Decimal,
    #[scrypto(mutable)]
    pub last_update_epoch: u64
}