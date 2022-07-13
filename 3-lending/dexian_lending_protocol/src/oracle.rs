use scrypto::prelude::*;

blueprint! {
    struct PriceOracle{
        usdc: ResourceAddress,
        usdt: ResourceAddress
    }

    /// dummy oracle
    impl PriceOracle{

        const EPOCH_OF_YEAR: u64 = 15017;
        
        pub fn new(usdt: ResourceAddress, usdc: ResourceAddress) -> ComponentAddress {
            Self{
                usdc,
                usdt
            }.instantiate.globalize()
        }

        pub fn get_price_quote_in_xrd(res_addr: ResourceAddress) -> Decimal {
            // Simulate changes in the market environment (time) to return different quotes
            // the actual application needs to use the real quote source and price
            if Runtime::current_epoch() > EPOCH_OF_YEAR {
                match res_addr{
                    Decimal::ONE => RADIX_TOKEN,
                    Decimal::ONE / Decimal::from("0.6") => USDC,
                    Decimal::ONE / Decimal::from("0.61") => USDT,
                }
            }
            else{
                match res_addr{
                    Decimal::ONE => RADIX_TOKEN,
                    Decimal::ONE / Decimal::from("0.06") => USDC,
                    Decimal::ONE / Decimal::from("0.061") => USDT,
                }
            }
        }
    }
}