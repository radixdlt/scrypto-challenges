use scrypto::prelude::*;
use crate::lending_app::LendingApp;
use crate::trading_app::TradingApp;
use crate::utils::*;

//fee for each operation executed by the portfolio
const FIXED_FEE: i32 = 100;

// Here, we define the data that will contain the number of positive/negative operation of the user 
#[derive(NonFungibleData)]
struct UserHistory {
    #[scrypto(mutable)]
    username: ComponentAddress,
    #[scrypto(mutable)]   
    positive_operation: u32,    
    #[scrypto(mutable)]   
    negative_operation: u32, 
    #[scrypto(mutable)]   
    expert: bool          
}

// Here, we define the operation data detail of a single operation 
#[derive(TypeId, Encode, Decode, Describe,NonFungibleData,Copy, Clone)]
struct OperationDetail {
    username: ComponentAddress,
    operation_id: u128,    
    xrd_tokens: Decimal,    
    current_price: u64,
    token_a_address: ResourceAddress, 
    token_b_address: ResourceAddress,
    num_token_b_received: Decimal,
    date_opened: u64, 
    date_closed: Option<u64>,     
    current_standing: Option<bool>,    
    number_of_request_for_autoclosing: Option<u32>,  
    current_requestor_for_closing: Option<ResourceAddress>   
}
//method used for logging data 
impl ToString for OperationDetail {
    fn to_string(&self) -> String {
        return format!("{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{:?}|{:?}|{:?}", 
        self.username,
        self.operation_id,    
        self.xrd_tokens,  
        self.current_price,  
        self.token_a_address,    
        self.token_a_address,    
        self.num_token_b_received,
        self.date_opened, 
        self.date_closed,     
        self.current_standing,    
        self.number_of_request_for_autoclosing,
        self.current_requestor_for_closing
        );
    }
}

// Here, we define the data of the user fundings 
// xrd_tokens: amount of xrd funded in the portfolio
// in_progress: funded active/terminated
// total_amount: amount of the main vault at the time of funding
// epoch_funded: epoch at the time of funding
#[derive(NonFungibleData)]
struct UserAccountFundingData {
    #[scrypto(mutable)]
    xrd_tokens: Decimal,
    #[scrypto(mutable)]   
    in_progress: bool,
    #[scrypto(mutable)]
    total_amount: Decimal,
    #[scrypto(mutable)]
    funded_ratio: Decimal,
    #[scrypto(mutable)]
    epoch_funded: u64                    
}

blueprint!{
    /// The Portfolio blueprint 
    struct Portfolio{

        /// The reserve for main pool
        main_pool: Vault,

        /// The reserve for trading token1 main pool
        token1_pool: Vault,
        /// The reserve for trading token2 main pool
        token2_pool: Vault,
        /// The reserve for trading token2 main pool
        token3_pool: Vault,                

        //address of the lendingapp blueprint 
        lending_app: ComponentAddress,
        //address of the tradingapp blueprint 
        trading_app: ComponentAddress,

        /// The resource definition of User Account Trading History token.
        user_account_trading_nft_resource_def: ResourceAddress,
        /// Vault with admin badge for managine User Account Trading History NFT.
        user_account_nft_admin_badge: Vault,    
        
        //Vector containing positions opened/closed
        positions: Vec<OperationDetail>,

        //vault containing lending nft for using the lendingapp component
        lending_nft_vault: Vault,

        //the resource for tracking user account fundings
        user_account_funding_nft_resource_def: ResourceAddress,

        //vault containing borrowing nft for using the lendingapp component
        borrowing_nft_vault: Vault,
        
        //vault containing LND received from the lendingapp component
        lnd_vault: Vault,

        //Decimal to contain the total amount funded in the portfolio (useful for calculate the ratio)
        amount_funded: Decimal
    }

    impl Portfolio {
        /// Instantiates a new Portfolio component. 
        pub fn new(
            xrd_address: ResourceAddress, 
            token_1_address: ResourceAddress,
            token_2_address: ResourceAddress,
            token_3_address: ResourceAddress,
            lending_app: ComponentAddress,
            trading_app: ComponentAddress,
            lending_nft_resource_def: ResourceAddress,
            borrowing_nft_resource_def: ResourceAddress,
            loan_tokens_resource_def: ResourceAddress) -> ComponentAddress {

            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .initial_supply(1);

            // let rules = AccessRules::new()
            //     .method("sell", rule!(require(admin_badge.resource_address())))
            //     .method("close_all_positions", rule!(require(admin_badge.resource_address())))        
            //     .method("reset_positions", rule!(require(admin_badge.resource_address())))          
            //     .default(rule!(allow_all));                          

            let user_account_trading_history_nft = ResourceBuilder::new_non_fungible()
                .metadata("name", "User Account Trading History")
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Create the non fungible resource that will represent the lendings
            let user_account_funding_nft: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "User Account Funding Data NFTs")
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), MUTABLE(rule!(require(admin_badge.resource_address()))))
                .no_initial_supply();                 

            return Self {
                main_pool: Vault::new(xrd_address),
                token1_pool: Vault::new(token_1_address),
                token2_pool: Vault::new(token_2_address),
                token3_pool: Vault::new(token_3_address),
                lending_app: lending_app,
                trading_app: trading_app,
                user_account_trading_nft_resource_def: user_account_trading_history_nft,
                user_account_nft_admin_badge: Vault::with_bucket(admin_badge),
                positions: Vec::new(),
                lending_nft_vault: Vault::new(lending_nft_resource_def),
                user_account_funding_nft_resource_def: user_account_funding_nft,
                borrowing_nft_vault: Vault::new(borrowing_nft_resource_def),
                lnd_vault: Vault::new(loan_tokens_resource_def),
                amount_funded: dec!(0),
            }
            .instantiate()
            // .add_access_check(rules)
            .globalize();            
        }


        // anyone as to register itself before using the portfolio component
        // and receives an NFT
        pub fn register(&mut self,address: ComponentAddress) -> (Bucket, Bucket) {
            let nft = self.user_account_nft_admin_badge.authorize(|| {
                let resource_manager = borrow_resource_manager!(self.user_account_trading_nft_resource_def);
                resource_manager.mint_non_fungible(
                    // The NFT id
                    &get_non_fungible_id(),
                    // The NFT data
                    UserHistory {
                        username: address,
                        positive_operation: 0,
                        negative_operation: 0,
                        expert: false,
                    },
                )
            });

            let portfolio_id = get_non_fungible_id();        
            let epoch_registered: u64 = Runtime::current_epoch();     
            // Create a NFT. TODO: If this already exist this needs to be updated
            let user_account_funding_nft = self.user_account_nft_admin_badge.authorize(|| {
                borrow_resource_manager!(self.user_account_funding_nft_resource_def)
                    .mint_non_fungible(&portfolio_id, UserAccountFundingData {xrd_tokens: dec!("0"), in_progress: false, total_amount: dec!("0"), funded_ratio: dec!("0"), epoch_funded: epoch_registered  })
                });             

            (nft,user_account_funding_nft)
        }

        // anyone has to fund tokens in the portfolio 
        // before starting to operate on behalf ot itself 
        // and on behalf of anyone else
        pub fn fund_portfolio(&mut self, starting_tokens: Bucket, user_account_funding_nft: Proof)  {
            info!("=== FUND PORTFOLIO OPERATION START === ");
            let how_many: Decimal = starting_tokens.amount();

            // Get the data associated with the User Account Trading History NFT and update the variable values
            let non_fungible: NonFungible<UserAccountFundingData> = user_account_funding_nft.non_fungible();
            let mut portfolio_nft_data = non_fungible.data();  
            assert!(portfolio_nft_data.in_progress==false, "You have already funded the portfolio! If you want fund more please withdraw first and then fund again!");      
            // let diff = dec!("100000")-self.main_pool.amount();
            // assert!(self.main_pool.amount()+how_many<=dec!("100000"), "Maximum amount fundable in the portfolio is 100.000! Max you can fund is {}" , diff);      

            //put in the main vault
            self.main_pool.put(starting_tokens);
            let total_amount: Decimal = self.main_pool.amount();
            // The ratio funded compared to the total amount.
            let funded_ratio = how_many * dec!("100") / total_amount;
            let epoch_funded: u64 = Runtime::current_epoch();
            //update the total funded in the portfolio
            self.amount_funded += how_many;

            //update the nft data 
            portfolio_nft_data.in_progress = true;
            portfolio_nft_data.xrd_tokens = how_many;
            portfolio_nft_data.total_amount = total_amount;
            portfolio_nft_data.funded_ratio = funded_ratio;
            portfolio_nft_data.epoch_funded = epoch_funded;
            // portfolio_nft_data.xrd_tokens - tokens_to_withdraw;
            self.user_account_nft_admin_badge.authorize(|| {
                borrow_resource_manager!(self.user_account_funding_nft_resource_def).update_non_fungible_data(&non_fungible.id(), portfolio_nft_data);
            });  
        }

        //anyone can withdraw its fundings even with opened position
        //at the time of withdraw a reward/penalty is calculated based on the total portfolio management results
        //any open position can be closed by anyone else later
        pub fn withdraw_portfolio(&mut self, user_account_funding_nft: Proof) -> Bucket {
            info!("=== WITHDRAW PORTFOLIO OPERATION START === ");

            // Get the data associated with the User Account Funding History NFT and update the variable values
            let non_fungible: NonFungible<UserAccountFundingData> = user_account_funding_nft.non_fungible();
            let mut portfolio_nft_data = non_fungible.data();
            //assert that a funding is in progress, otherwise stop the withdraw                
            assert!(portfolio_nft_data.in_progress==true, "You have nothing to withdraw!!");      
            
            let starting_epoch: u64 = portfolio_nft_data.epoch_funded;
            let actual_epoch: u64 = Runtime::current_epoch();

            //update the total funded in the portfolio
            info!(" Amount of funded tokens in the portfolio {} " , self.amount_funded);  
            info!(" Amount of yours funded tokens in the portfolio {} " , portfolio_nft_data.xrd_tokens);  
            //total portfolio value at now 
            let total = self.portfolio_total_value();
            // The ratio of increment/decrease of the main pool.
            let diff_ratio: Decimal = ((total / self.amount_funded) * dec!("100") )-dec!(100);
            info!(" Portfolio increase/decrease ratio  {} " , diff_ratio);  

            //the amount of tokens to be returned to the user account with increase or decrease
            let diff_tokens = portfolio_nft_data.xrd_tokens * (dec!("100") + diff_ratio) / dec!("100");
         
            info!(" you got {} from {} in {} epoch " , diff_tokens , portfolio_nft_data.xrd_tokens , (actual_epoch-starting_epoch));            
            //return the tokens to the user account
            let to_be_returned: Bucket = self.main_pool.take(diff_tokens);

            //update the total funded in the portfolio
            self.amount_funded = self.amount_funded - diff_tokens;
            info!(" Updated Amount of funded tokens  {} " , self.amount_funded);  

            // // Update the data on that NFT globally
            portfolio_nft_data.in_progress = false;
            portfolio_nft_data.xrd_tokens = Decimal::zero();
            portfolio_nft_data.total_amount = Decimal::zero();
            portfolio_nft_data.epoch_funded = Runtime::current_epoch();

            // portfolio_nft_data.xrd_tokens - tokens_to_withdraw;
            self.user_account_nft_admin_badge.authorize(|| {
                borrow_resource_manager!(self.user_account_funding_nft_resource_def).update_non_fungible_data(&non_fungible.id(), portfolio_nft_data);
            });

            to_be_returned
        }

       // Execute a buy operation by means of the portfolio.
       pub fn buy(&mut self,xrd_tokens: Decimal, user_account: ComponentAddress, token_to_buy: ResourceAddress, user_account_funding_nft: Proof)   {
            assert!(
                self.main_pool.amount() >= xrd_tokens,
                "Main vault has not sufficient tokens to buy ! Please fund portfolio !"
            );   
            // Get the data associated with the User Account Funding History NFT and update the variable values
            let non_fungible: NonFungible<UserAccountFundingData> = user_account_funding_nft.non_fungible();
            let portfolio_nft_data = non_fungible.data();       
            assert!(portfolio_nft_data.in_progress, "You first need to fund the portfolio then you can operate on behalf of it!!");      
            assert!(
                portfolio_nft_data.xrd_tokens*10 >= xrd_tokens,
                "You can use max 10x leverage !!"
            );   

            let trading_app: TradingApp = self.trading_app.into();
            //buy by using the trading app
            if token_to_buy==self.token1_pool.resource_address() {
                info!("Buy tokens1 {}", token_to_buy);
                //Take the xrd tokens from the main vault for buying by using the trading app, getting back tokens and put in the pool1 vault
                self.token1_pool.put(trading_app.buy_generic(self.main_pool.take(xrd_tokens), token_to_buy));
            } else if token_to_buy==self.token2_pool.resource_address() {
                info!("Buy tokens2 {}", token_to_buy);
                self.token2_pool.put(trading_app.buy_generic(self.main_pool.take(xrd_tokens), token_to_buy));
            } else if token_to_buy==self.token3_pool.resource_address() {
                info!("Buy tokens3 {}", token_to_buy);
                self.token3_pool.put(trading_app.buy_generic(self.main_pool.take(xrd_tokens), token_to_buy));
            } 
            
            let current_price = trading_app.current_price(RADIX_TOKEN,token_to_buy);
            let how_many = xrd_tokens / current_price;

            let trade1 = OperationDetail {
                username: user_account,
                operation_id: Runtime::generate_uuid(),    
                xrd_tokens: xrd_tokens,    
                current_price: current_price,    
                token_a_address: RADIX_TOKEN,
                token_b_address: token_to_buy,
                num_token_b_received: how_many,
                date_opened: Runtime::current_epoch(),
                date_closed: None,
                current_requestor_for_closing: None, 
                current_standing: None,
                number_of_request_for_autoclosing: None,
            };     
            //add a position to the list
            self.positions.push(trade1);
                
        }

        // Execute a sell operation by means of the portfolio.
        // Method can be executed only using an admin badge 
        pub fn sell(&mut self,tokens: Decimal, token_to_sell: ResourceAddress
        )   {
            let trading_app: TradingApp = self.trading_app.into();

            if token_to_sell==self.token1_pool.resource_address() {
                //get the token from token1_pool and sell by using the trading app, getting back xrd tokens in the main vault
                self.main_pool.put(trading_app.sell_generic(self.token1_pool.take(tokens)));
            } else if token_to_sell==self.token2_pool.resource_address() {
                self.main_pool.put(trading_app.sell_generic(self.token2_pool.take(tokens)));
            } else if token_to_sell==self.token3_pool.resource_address() {
                self.main_pool.put(trading_app.sell_generic(self.token3_pool.take(tokens)));
            } 
        }

        // Reset the a position, needs a badge 
        pub fn reset_positions(&mut self)  {
            info!("Position size {}", self.positions.len());
            //replace the Vec with the new one 
            self.positions = Vec::new();
            // info!("No open positions now {}", self.positions.len());
        }

        // Calculate the value of the other vault (getting the token current price from the tradingapp component)
        pub fn portfolio_value(&self) -> Decimal {
            info!("Position size inside portfolio {}", self.positions.len());
            let trading_app: TradingApp = self.trading_app.into();
            
            let total1: Decimal = self.token1_pool.amount()*(trading_app.current_price(RADIX_TOKEN,self.token1_pool.resource_address()));
            let total2: Decimal = self.token2_pool.amount()*(trading_app.current_price(RADIX_TOKEN,self.token2_pool.resource_address()));
            let total3: Decimal = self.token3_pool.amount()*(trading_app.current_price(RADIX_TOKEN,self.token3_pool.resource_address()));                        
            info!("{:?} tokens are valued xrd {:?}", self.token1_pool.amount(), total1);
            info!("{:?} tokens are valued xrd {:?}", self.token2_pool.amount(), total2);
            info!("{:?} tokens are valued xrd {:?}", self.token3_pool.amount(), total3);

            total1+total2+total3
        }

        // Calculate the total value of the portfolio (main vault + other vault + lnd vault)
        pub fn portfolio_total_value(&self) -> Decimal {

            //trading vault are calculated at current price
            let mut total: Decimal = self.portfolio_value();
            //total liquidity
            let totalxrd: Decimal = self.main_pool.amount();
            info!("Value in main vault {:?}", totalxrd);
            //total xrd in lending pool
            let totallnd: Decimal = self.lnd_vault.amount();
            info!("Value in lnd vault {:?}", totallnd);            
            total = total + totalxrd + totallnd;
            info!("Grandtotal {:?}", total);

            total
        }
   
        // calculate the list of open position (positionId can be used for closing)
        pub fn position(&self) -> Vec<u128> {
            info!("Position size {}", self.positions.len());
            let trading_app: TradingApp = self.trading_app.into();

            let mut losing_positions: Vec<u128> = Vec::new();
            for inner_position in &self.positions {
                info!("Inner Position {}", inner_position.to_string());    
                info!("Position Id {}", inner_position.operation_id);          

                info!("Ready to get current price ");
                let updated_value = trading_app.current_price(inner_position.token_a_address,inner_position.token_b_address);
                info!("Xrd used for trade {}", inner_position.xrd_tokens);
                info!("Starting price {:?}", inner_position.current_price );
                info!("Current price {:?}", updated_value );
                let net_result: Decimal = inner_position.xrd_tokens*Decimal::from(updated_value)/Decimal::from(inner_position.current_price)-inner_position.xrd_tokens;
                info!("Position net result {:?}", net_result);

                if net_result <= Decimal::ZERO {
                    info!("Found a suffering position , ID = {}", inner_position.operation_id);
                    losing_positions.push(inner_position.operation_id);
                };

            }        
            info!("Losing Position size {}", losing_positions.len());
            losing_positions
        }

        // Close a position, method can be executed by anyone register with the portfolio by using the positionId 
        pub fn close_position(&mut self, operation_id: u128)  {
            info!("Position size {}", self.positions.len());
            let mut amount_to_sell: Decimal = Decimal::zero();
            let mut token_to_sell: ResourceAddress = RADIX_TOKEN;
            let mut remaining_positions: Vec<OperationDetail> = Vec::new();
            let trading_app: TradingApp = self.trading_app.into();

            for inner_position in &self.positions {     
                info!("Position Id {}", inner_position.operation_id);    

                if inner_position.operation_id==operation_id {
                    amount_to_sell = inner_position.num_token_b_received.clone();
                    token_to_sell = inner_position.token_b_address.clone();

                    //let's check if the position is losing yet, otherwise we can close it
                    let updated_value = trading_app.current_price(inner_position.token_a_address,inner_position.token_b_address);
                    let net_result: Decimal = inner_position.xrd_tokens*Decimal::from(updated_value)/Decimal::from(inner_position.current_price)-inner_position.xrd_tokens;
                    info!("Updated position net result {:?}", net_result);
                    if net_result >= Decimal::ZERO {
                        info!("The position is not anymore losing  , ID = {}", inner_position.operation_id);
                        remaining_positions.push(inner_position.clone());
                    };

                } else {
                    remaining_positions.push(inner_position.clone());
                }
            }    
            info!("Ready to close position {}", operation_id);
            //replace the Vec with the new one with the closed position is missing
            self.positions = remaining_positions;

            info!("Position size after removing the closed position {}", self.positions.len());
            //execute the sell operation using the trading component
            self.sell(amount_to_sell, token_to_sell);    
        }

        // Close all the position, method can be executed only using an admin badge 
        pub fn close_all_positions(&mut self)  {
            info!("Position size {}", self.positions.len());
            let mut _amount_to_sell: Decimal = Decimal::zero();
            let mut _token_to_sell: ResourceAddress = RADIX_TOKEN;
            let mut _remaining_positions: Vec<OperationDetail> = Vec::new();
            for inner_position in &self.positions {     
                info!("Position Id {} Amount {} Component address that started the operation {} "
                    ,inner_position.operation_id
                    ,inner_position.xrd_tokens
                    ,inner_position.username);    
                _amount_to_sell = inner_position.num_token_b_received.clone();
                _token_to_sell = inner_position.token_b_address.clone()
                // self.sell(amount_to_sell,token_to_sell);   
            }    
            //replace the Vec with the new one with the closed position is missing
            // self.positions = Vec::new();
            // info!("No open positions now {}", self.positions.len());
        }

        //not implemented
        //method can be executed only using an admin badge 
        // pub fn refund_all(&mut self)  {
        //     let address: ComponentAddress;
        //     let component = borrow_component!(address);
        //     component.call::<()>("deposit", args![refund])
        // }   

        //Method using the LendingApp component
        //Here, the platform register itself to be able to lend tokens to the LendingApp component
        pub fn register_for_lending(&mut self)  {
            info!("Registering for lending ") ;
            info!("Vault for Lending NFT, accept resource address : {:?} ", self.lending_nft_vault.resource_address());
            let lending_app: LendingApp = self.lending_app.into();
            let bucket: Bucket = lending_app.register();
            self.lending_nft_vault.put(bucket);
        }
        //Here, the platform register itself to be able to borrow tokens from the LendingApp component
        pub fn register_for_borrowing(&mut self) {
            info!("Registering for borrowing ") ;
            info!("Vault for Borrowing NFT, accept resource address : {:?} ", self.borrowing_nft_vault.resource_address());
            let lending_app: LendingApp = self.lending_app.into();
            let bucket: Bucket = lending_app.register_borrower();
            self.borrowing_nft_vault.put(bucket);            
        }     
        //Here, the user account can order the platform component to lend tokens to the LendingApp component
        pub fn lend(&mut self,xrd_tokens: Decimal)  {
            info!("Lending ");
            let lending_app: LendingApp = self.lending_app.into();

            let xrd_to_lend: Bucket = self.main_pool.take(xrd_tokens);
            let proof: Proof = self.lending_nft_vault.create_proof_by_amount(dec!(1));
            
            //return lending_app.lend_money(tokens, proof);
            self.lnd_vault.put(lending_app.lend_money(xrd_to_lend, proof));
        }
        //Here, the user account can order the platform component to get back tokens from the LendingApp component
        pub fn take_back(&mut self, lnd_tokens: Decimal)  {
            info!("Take back ");

            let xrd_to_get_back: Bucket = self.lnd_vault.take(lnd_tokens);

            let lending_app: LendingApp = self.lending_app.into();
            let proof: Proof = self.lending_nft_vault.create_proof_by_amount(dec!(1));
            
            //return lending_app.take_money_back(xrd_to_get_back, proof);
            self.main_pool.put(lending_app.take_money_back(xrd_to_get_back, proof));
        }

    }
}