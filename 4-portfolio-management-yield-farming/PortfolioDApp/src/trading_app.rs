use scrypto::prelude::*;
use rand::Rng;

blueprint! {
    #[derive(Debug)]
    struct TradingApp {
        /// The reserve for main pool
        main_pool: Vault,

        /// The reserve for trading token1 main pool
        token1_pool: Vault,

        /// The reserve for trading token1 main pool
        token2_pool: Vault,

        /// The reserve for trading token1 main pool
        token3_pool: Vault
    }


    impl TradingApp {
        /// Creates a TradingApp component and returns the component address
        pub fn create_market(token_a_address: ResourceAddress, token_b_address: ResourceAddress, 
                            token_c_address: ResourceAddress, token_d_address: ResourceAddress) -> ComponentAddress {

            // Instantiate our tradingapp component
            let tradingapp = Self {
                main_pool: Vault::new(token_a_address),
                token1_pool: Vault::new(token_b_address),
                token2_pool: Vault::new(token_c_address),
                token3_pool: Vault::new(token_d_address),
            }
            .instantiate();
            // Return the new Tradingapp component
            tradingapp.globalize()
        }

        pub fn fund(&mut self) {
            info!("=== FUND OPERATION START === ");
        }

        //the following methods should be replace by 'fund_market'
        pub fn fund_token1(&mut self, starting_tokens: Bucket) {
            info!("=== FUND TOKEN 1 OPERATION START === ");
                self.main_pool.put(starting_tokens);
        }
        pub fn fund_token2(&mut self, starting_tokens: Bucket) {
            info!("=== FUND TOKEN 2 OPERATION START === ");
                self.token1_pool.put(starting_tokens);
        }
        pub fn fund_token3(&mut self, starting_tokens: Bucket) {
            info!("=== FUND TOKEN 3 OPERATION START === ");
                self.token2_pool.put(starting_tokens);
        }
        pub fn fund_token4(&mut self, starting_tokens: Bucket) {
            info!("=== FUND TOKEN 4 OPERATION START === ");
                self.token3_pool.put(starting_tokens);
        }                        

        //the following method should replace the previous ones
        pub fn fund_market(&mut self, starting_tokens: Bucket,token_index1: Bucket,token_index2: Bucket,token_index3: Bucket) {
            info!("=== FUND ALL OPERATION START === ");
                self.main_pool.put(starting_tokens);
                self.token1_pool.put(token_index1);
                self.token2_pool.put(token_index2);
                self.token3_pool.put(token_index3);
        }

        //buy generic, with resource address
        pub fn buy_generic(&mut self, xrd_tokens: Bucket, token_to_buy: ResourceAddress) -> Bucket {
            info!("=== BUY OPERATION START === ");
            let token_received = xrd_tokens.amount();
            self.main_pool.put(xrd_tokens);
            let mut return_toked = Bucket::new(token_to_buy);

            info!("=== BUY OPERATION END === ");
            // Return the tokens along with NFT
            if self.token1_pool.resource_address()==token_to_buy {
                let current_value: Decimal = dec!("0,04");
                let how_many = token_received / current_value;
                info!("N. to buy1: {}", how_many);
                return_toked = self.token1_pool.take(how_many);
            } else if self.token2_pool.resource_address()==token_to_buy {
                let current_value: Decimal = dec!("40");
                let how_many = token_received / current_value;
                info!("N. to buy2: {}", how_many);                
                return_toked = self.token2_pool.take(how_many);
            } else if self.token3_pool.resource_address()==token_to_buy {
                let current_value: Decimal = dec!("10");
                let how_many = token_received / current_value;
                info!("N. to buy3: {}", how_many);
                return_toked = self.token3_pool.take(how_many);
            }

            return_toked
        }

        //buy from the token1_pool (should be replace by buy_generic method)
        pub fn buy(&mut self, xrd_tokens: Bucket) -> Bucket {
            info!("=== BUY OPERATION START === ");
            let current_value: Decimal = dec!("40");

            let how_many = xrd_tokens.amount() / current_value;
            info!("N. to buy: {}", how_many);

            self.main_pool.put(xrd_tokens);

            info!("=== BUY OPERATION END === ");
            // Return the tokens along with NFT
            let return_toked = self.token1_pool.take(how_many);

            return_toked
        }

        // pub fn current_price(&mut self, token_a_address: ResourceAddress, token_b_address: ResourceAddress)  {
        //     info!("=== GENERATE NUMBER === ");
        //     let current = Runtime::current_epoch();
        //     println!("Current epoch: {}", current);
        //     let secret_number = rand::thread_rng().gen_range(-10..10);
        //     println!("The secret number is: {}", secret_number);
        // }

        //sell from the token1_pool (should be replace by sell_generic method)
        pub fn sell(&mut self, tokens: Bucket) -> Bucket {
            info!("=== SELL OPERATION START === ");
            let current_value: Decimal = dec!("45");

            let how_many = tokens.amount() * current_value;
            info!("N. xrd to receive: {}", how_many);
            let xrd_tokens = self.main_pool.take(how_many);
            self.token1_pool.put(tokens);
            
            // Return the tokens along with NFT
            xrd_tokens
        }

        //returns the pool size/address
        pub fn token1_pool_size(&self) -> Decimal {
            return self.token1_pool.amount();
        }
        pub fn token1_pool_address(&self) -> ResourceAddress {
            return self.token1_pool.resource_address();
        }           
        pub fn token2_pool_size(&self) -> Decimal {
            return self.token2_pool.amount();
        }
        pub fn token2_pool_address(&self) -> ResourceAddress {
            return self.token2_pool.resource_address();
        }               
        pub fn token3_pool_size(&self) -> Decimal {
            return self.token3_pool.amount();
        }      
        pub fn token3_pool_address(&self) -> ResourceAddress {
            return self.token3_pool.resource_address();
        }                       
        //returns the main pool size
        pub fn main_pool_size(&self) -> Decimal {
            return self.main_pool.amount();
        }
    
    }
}