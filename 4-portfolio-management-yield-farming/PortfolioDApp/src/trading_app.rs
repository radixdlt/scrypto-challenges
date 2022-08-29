use scrypto::prelude::*;
// use rand::Rng;

blueprint! {
    #[derive(Debug)]
    struct TradingApp {
        /// The reserve for main pool
        main_pool: Vault,

        /// The reserve for trading token1 main pool
        token1_pool: Vault,

        /// The reserve for trading token2 main pool
        token2_pool: Vault,

        /// The reserve for trading token3 main pool
        token3_pool: Vault,

        // Starting epoch of trading app
        last_epoch: u64,
        //current simulated price of first pair
        token1_starting_value: u64,
        //current simulated price of second pair
        token2_starting_value: u64,
        //current simulated price of third pair
        token3_starting_value: u64,        

        //vault for trading fees
        trading_fees: Vault,
    }


    impl TradingApp {
        /// Creates a TradingApp component and returns the component address
        pub fn create_market(token_a_address: ResourceAddress, token_b_address: ResourceAddress, 
                            token_c_address: ResourceAddress, token_d_address: ResourceAddress) -> ComponentAddress {

            // Get the starting epoch .
            let last_epoch = Runtime::current_epoch();

            let token1_starting_value: u64 = "40".parse().expect("Not a number!");
            let token2_starting_value: u64 = "10".parse().expect("Not a number!");
            let token3_starting_value: u64 = "5".parse().expect("Not a number!");

            // Create the admin badges
            // let admin_badge: Bucket = ResourceBuilder::new_fungible() 
            // .divisibility(DIVISIBILITY_NONE)
            // .metadata("name", "Admin Badge")
            // .initial_supply(1);

            // // Define the access rules for this blueprint.
            // let access_rules = AccessRules::new()
            // .method("fund_market", rule!(require(admin_badge.resource_address()))) 
            // .method("fund_token1", rule!(require(admin_badge.resource_address()))) 
            // .method("fund_token2", rule!(require(admin_badge.resource_address()))) 
            // .method("fund_token3", rule!(require(admin_badge.resource_address()))) 
            // .method("fund_token4", rule!(require(admin_badge.resource_address()))) 
            // .default(rule!(allow_all));


            // Instantiate our tradingapp component
            let tradingapp = Self {
                main_pool: Vault::new(token_a_address),
                token1_pool: Vault::new(token_b_address),
                token2_pool: Vault::new(token_c_address),
                token3_pool: Vault::new(token_d_address),
                last_epoch: last_epoch,
                token1_starting_value: token1_starting_value,
                token2_starting_value: token2_starting_value,
                token3_starting_value: token3_starting_value,
                trading_fees: Vault::new(token_a_address),
            }
            .instantiate();
            //.add_access_check(access_rules);
            // Return the new Tradingapp component
            //(tradingapp.globalize(),admin_badge)
            tradingapp.globalize()
        }

        pub fn fund(&mut self) {
            info!("=== FUND OPERATION START === ");
        }

        //the following methods should be replace by 'fund_market'
        pub fn fund_token1(&mut self, starting_tokens: Bucket) {
            info!("=== FUND ONLY TOKEN 1 OPERATION START === ");
                self.main_pool.put(starting_tokens);
        }
        pub fn fund_token2(&mut self, starting_tokens: Bucket) {
            info!("=== FUND ONLY TOKEN 2 OPERATION START === ");
                self.token1_pool.put(starting_tokens);
        }
        pub fn fund_token3(&mut self, starting_tokens: Bucket) {
            info!("=== FUND ONLY TOKEN 3 OPERATION START === ");
                self.token2_pool.put(starting_tokens);
        }
        pub fn fund_token4(&mut self, starting_tokens: Bucket) {
            info!("=== FUND ONLY TOKEN 4 OPERATION START === ");
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

        //use xrd tokens to buy the tokens you desire specifying its resource address
        pub fn buy_generic(&mut self, xrd_tokens: Bucket, token_to_buy: ResourceAddress) -> Bucket {
            info!("=== BUY OPERATION START === ");
            let token_received = xrd_tokens.amount();
            self.main_pool.put(xrd_tokens);
            let mut returned_bucket = Bucket::new(token_to_buy);

            // Return the tokens along with NFT
            if self.token1_pool.resource_address()==token_to_buy {
                let how_many = token_received / self.token1_starting_value;
                info!("N. token1 to buy: {}", how_many);
                returned_bucket.put(self.token1_pool.take(how_many))
            } else if self.token2_pool.resource_address()==token_to_buy {
                let how_many = token_received / self.token2_starting_value;
                info!("N. token2 to buy: {}", how_many);              
                returned_bucket.put(self.token2_pool.take(how_many))
            } else if self.token3_pool.resource_address()==token_to_buy {
                let how_many = token_received / self.token3_starting_value;
                info!("N. token3 to buy: {}", how_many);
                returned_bucket.put(self.token3_pool.take(how_many))
            } 
            info!("=== BUY OPERATION END === ");
            returned_bucket
        }

        //sell tokens and receives xrd
        pub fn sell_generic(&mut self, tokens: Bucket) -> Bucket {
            info!("=== SELL OPERATION START === ");
            let price = self.current_price(RADIX_TOKEN, tokens.resource_address());
            let current_value = price;
            let how_many = (tokens.amount() * current_value).round(2,RoundingMode::TowardsPositiveInfinity);
            info!("N. xrd to receive: {}", how_many);
            let returned_tokens = self.main_pool.take(how_many);
            //put the tokens received in the correct vault
            if tokens.resource_address()==self.token1_pool.resource_address() {
                self.token1_pool.put(tokens);
            } else if tokens.resource_address()==self.token2_pool.resource_address() {
                self.token2_pool.put(tokens);
            } else if tokens.resource_address()==self.token3_pool.resource_address() {
                self.token3_pool.put(tokens);
            } 

            info!("=== SELL OPERATION END === ");
            // Return the tokens 
            returned_tokens
        }

        //buy from the token1_pool (should be replace by buy_generic method)
        pub fn buy(&mut self, xrd_tokens: Bucket) -> Bucket {
            info!("=== BUY OPERATION START === ");

            let how_many = (xrd_tokens.amount() / self.token1_starting_value).round(2,RoundingMode::TowardsPositiveInfinity);
            info!("N. to buy: {}", how_many);

            self.main_pool.put(xrd_tokens);

            // Return the tokens 
            let return_toked = self.token1_pool.take(how_many);

            info!("=== BUY OPERATION END === ");
            return_toked
        }

        //sell from the token1_pool (should be replace by sell_generic method)
        pub fn sell(&mut self, tokens: Bucket) -> Bucket {
            info!("=== SELL OPERATION START === ");
            let price = self.current_price(RADIX_TOKEN, tokens.resource_address());
            let current_value = price;

            let how_many = (tokens.amount() * current_value).round(2,RoundingMode::TowardsPositiveInfinity);
            info!("N. xrd to receive: {}", how_many);
            let xrd_tokens = self.main_pool.take(how_many);
            self.token1_pool.put(tokens);
            
            // Return the tokens 
            xrd_tokens
        }

        //here we simulate the change in price each time the epoch changes
        //this should return the price respect of token_a/token_b pair
        //TODO For the scope of this demo this method instead return the price respect of token_a/token1:resource address
        pub fn current_price(&mut self, _token_a_address: ResourceAddress, _token_b_address: ResourceAddress) -> u64 {
            let current = Runtime::current_epoch();
            //info!("Current epoch {} vs last epoch {}", current, self.last_epoch);
            let mut value: u64 = 0;

            //if epoch has changed then I change also the price of the asset
            if current > self.last_epoch {
                //max move is 10%
                let random_number = self.get_random() % 10 + 1;
                let random_direction = self.get_random() % 2;
                info!("The random movement is: {} and direction is {} ", random_number, random_direction);
                if random_direction==0 { 
                    self.token1_starting_value = self.token1_starting_value - (self.token1_starting_value*(random_number as u64)/100);
                    self.token2_starting_value = self.token2_starting_value - (self.token2_starting_value*(random_number as u64)/100);
                    self.token3_starting_value = self.token3_starting_value - (self.token3_starting_value*(random_number as u64)/100);
                } 
                else { 
                    self.token1_starting_value = self.token1_starting_value + (self.token1_starting_value*(random_number as u64)/100);
                    self.token2_starting_value = self.token2_starting_value + (self.token2_starting_value*(random_number as u64)/100);
                    self.token3_starting_value = self.token3_starting_value + (self.token3_starting_value*(random_number as u64)/100);
                } 
                
                info!("New price for {} is : {} ", self.token1_pool.resource_address(), self.token1_starting_value);
                info!("New price for {} is : {} ", self.token2_pool.resource_address(), self.token2_starting_value);
                info!("New price for {} is : {} ", self.token3_pool.resource_address(), self.token3_starting_value);
                self.last_epoch = current;
            } 
            
            if _token_b_address==self.token1_pool.resource_address() {
                info!("Current price of {}/{} is {} ", _token_a_address, _token_b_address , self.token1_starting_value);
                value = self.token1_starting_value;
            } else if _token_b_address==self.token2_pool.resource_address() {
                info!("Current price of {}/{} is {} ", _token_a_address, _token_b_address , self.token2_starting_value);
                value = self.token2_starting_value;
            } else if _token_b_address==self.token3_pool.resource_address() {
                info!("Current price of {}/{} is {} ", _token_a_address, _token_b_address , self.token3_starting_value);
                value = self.token3_starting_value;
            } 

            value
        }

        // withdraw all the fees collected 
        pub fn withdraw(&mut self) -> Bucket {
            info!("=== WITHDRAW FEES === ");

            let xrd_tokens_from_fees = self.trading_fees.take_all();

            // Return the tokens 
            xrd_tokens_from_fees
        }


        // This is a pseudorandom function and not a true random number function.
        pub fn get_random(&self) -> u128 {
            Runtime::generate_uuid() 
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