//! The Transporter redeems [voucher::SealedVoucher]s for NonFungibles
//! effectively moving assets onto the ledger (by minting them into existance)
pub mod authentication;
pub mod blueprint;
pub mod decoder;
pub mod voucher;
