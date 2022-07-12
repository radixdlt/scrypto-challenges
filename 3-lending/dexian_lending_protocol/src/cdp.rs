use scrypto::prelude::*;

#[derive(NonFungibleData)]
struct CollateralDebtPosition{
    borrow_token: ResourceAddress,
    collateral_token: ResourceAddress,
    
    #[scrypto(mutable)]
    total_borrow: Decimal,
    #[scrypto(mutable)]
    total_repay: Decimal,
    
    #[scrypto(mutable)]
    normalized_borrow: Decimal,
    #[scrypto(mutable)]
    collateral_amount: Decimal,
    #[scrypto(mutable)]
    borrow_amount: Decimal,
    #[scrypto(mutable)]
    repay_amount: Decimal,
    #[scrypto(mutable)]
    last_update_epoch: u64
}