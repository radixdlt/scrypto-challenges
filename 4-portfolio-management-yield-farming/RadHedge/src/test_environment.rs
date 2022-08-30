use price_oracle::*;
use radex::faucet::*;
use radex::radex::*;
use scrypto::prelude::*;

// Create Testenvironment
// Sinnvolles Setup aufbauen TODO

blueprint! {
    struct TestEnvironment {
        /// Stores the price-oracle component used for this test environment.
        oracle: PriceOracle,
        /// Stores the address of the price-oracle
        oracle_address: ComponentAddress,
        /// Stores the admin badge of the price oracle.
        oracle_admin_badge: Vault,
        /// Stores the decentralized exchange (DEX) thats being used for this test environment.
        dex: RaDEX,
        /// Stores the address of the decentralized exchange (DEX) thats being used for this test environment.
        dex_address: ComponentAddress,
        /// Stores the faucet component used to create some crypto currencies thats being used for this test environment.
        faucet: Faucet,
        /// Stores the address of the faucet used to create some crypto currencies thats being used for this test environment.
        faucet_address: ComponentAddress,
        /// Store all tracking tokens for the liquidity pools.
        lp_tracking_tokens:Vec<Vault>,
    }

    impl TestEnvironment {
        /// Creates a test environment for the investment pool.
        ///
        /// This test environment included a price oracle, a decentralized exchange (RaDEX) and a faucet to create some token.
        /// It also creates the necessary liquidity pools and funds them appropriately.
        ///
        ///
        /// This function doesn't take any arguments.
        ///
        /// # Returns:
        ///
        /// * (Vec<Bucket>) -All buckets with newly created currencies.
        #[allow(clippy::vec_init_then_push)]
        pub fn create_test_environment() -> Vec<Bucket> {
            // Instantiate the price oracle with one admin.
            let (oracle_admin_badge_bucket, oracle_address) = PriceOracle::instantiate_oracle(1);
            let oracle: PriceOracle = oracle_address.into();

            // Instantiate the RaDEX.
            let dex_address = RaDEX::new();
            let dex: RaDEX = dex_address.into();

            // Instantiate the faucet.
            let (faucet_address, mut currencies): (ComponentAddress, Vec<Bucket>) =
                Faucet::instantiate_faucet();
            let faucet: Faucet = faucet_address.into();

            // // Instantiate liqudity pools for the RaDEX.
            // let mut bucket_btc : Bucket = Bucket::new(currencies[0].resource_address());
            // bucket_btc.put(currencies[0]);

            // let mut bucket_eth : Bucket = Bucket::new(currencies[1].resource_address());
            // bucket_eth.put(currencies[1]);

            // let mut bucket_usdt : Bucket = Bucket::new(currencies[2].resource_address());
            // bucket_usdt.put(currencies[2]);

            // let mut bucket_bnb : Bucket = Bucket::new(currencies[3].resource_address());
            // bucket_bnb.put(currencies[3]);

            // let mut bucket_ada : Bucket = Bucket::new(currencies[4].resource_address());
            // bucket_ada.put(currencies[4]);
            // TODO: If time is left: Keep tracking tokens. Keep currencies in vaults. Take out all liquidity for next epoch. Refund pools with new liquidity.

            // Keep all lp_tracking_tokens in this vec.
            let mut lp_tracking_tokens: Vec<Vault> = Vec::new();

            // Create a lp with btc and usdt: 5_000:100_000_000 (btc = 20000USDT) and keep tracking tokens.
            lp_tracking_tokens.push(Vault::with_bucket(dex.new_liquidity_pool(currencies[0].take(5_000), currencies[2].take(100_000_000))));

            // Create a lp with btc and eth: 1_000:10_000 (eth = 2000USDT) and keep tracking tokens.
            lp_tracking_tokens.push(Vault::with_bucket(dex.new_liquidity_pool(currencies[0].take(1_000), currencies[1].take(10_000))));

            // Create a lp with eth and usdt: 10_000: 20_000_000 (eth = 2000USDT) and keep tracking tokens.
            lp_tracking_tokens.push(Vault::with_bucket(dex.new_liquidity_pool(currencies[1].take(10_000), currencies[2].take(20_000_000))));

            // Create a lp with bnb and usdt: 100_000: 30_000_000 (bnb = 300USDT) and keep tracking tokens.
            lp_tracking_tokens.push(Vault::with_bucket(dex.new_liquidity_pool(currencies[3].take(100_000), currencies[2].take(30_000_000))));

            // Create a lp with ada and usdt: 40_000_000:10_000_000 (ada = 0.4USDT) and keep tracking tokens.
            lp_tracking_tokens.push(Vault::with_bucket(dex.new_liquidity_pool(currencies[4].take(5_000), currencies[2].take(100_000_000))));

            // Instantiate the test environment.
            Self {
                oracle,
                oracle_address,
                oracle_admin_badge: Vault::with_bucket(oracle_admin_badge_bucket),
                dex,
                dex_address,
                faucet,
                faucet_address,
                lp_tracking_tokens,
            }
            .instantiate()
            .globalize();

            info!(
                    "[Test Environment Creation]: Created a new test environment with price oracle [{}], RaDEX: [{}] and faucet [{}].",
                    oracle_address,
                    dex_address,
                    faucet_address
                );

            currencies

        }

        /// Returns the oracle component address.
        pub fn oracle_address(&self) -> ComponentAddress {
            self.oracle_address
        }

        /// Returns the dex component address.
        pub fn dex_address(&self) -> ComponentAddress {
            self.dex_address
        }

        /// Returns the faucet component address.
        pub fn faucet_address(&self) -> ComponentAddress {
            self.faucet_address
        }

        // Change oracle price. TODO
    }
}
