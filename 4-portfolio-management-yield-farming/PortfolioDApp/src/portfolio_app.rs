use scrypto::prelude::*;
use crate::lending_app::LendingApp;
use crate::trading_app::TradingApp;
use crate::utils::*;

const FIXED_FEE: i32 = 10;

// Here, we define the data that will be present in
// each of the user . 
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

// definition of operation 
#[derive(TypeId, Encode, Decode, Describe,NonFungibleData)]
struct OperationHistory {
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

impl ToString for OperationHistory {
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

// Here, we define the data that will be present in
// each of the lending ticket NFTs.
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

        lending_app: ComponentAddress,

        trading_app: ComponentAddress,

        /// The resource definition of User Account Trading History token.
        user_account_trading_nft_resource_def: ResourceAddress,
        /// Vault with admin badge for managine UserHistory NFT.
        user_account_trading_history_nft_admin_badge: Vault,    
        
        //positions opened/closed
        positions: Vec<OperationHistory>,
        //vault containing lending nft for using the lendingapp component
        lending_nft_vault: Vault,

        //the resource for tracking user account fundings
        user_account_funding_nft_resource_def: ResourceAddress,

        //vault containing borrowing nft for using the lendingapp component
        borrowing_nft_vault: Vault,
        
        //vault containing LND 
        lnd_vault: Vault,

        amount_funded: Decimal
    }

    

    // resim call-function $package TradingApp create_market $xrd $btc $eth $leo
//procedo con il funding del market
// resim call-method $component fund_market 1000,$xrd 1000,$btc 1000,$eth 1000,$leo

    impl Portfolio {
        /// Instantiates a new Portfolio component. 
        pub fn new(
            xrd_address: ResourceAddress, 
            token_1_address: ResourceAddress,
            lending_app: ComponentAddress,
            trading_app: ComponentAddress,
            lending_nft_resource_def: ResourceAddress,
            borrowing_nft_resource_def: ResourceAddress,
            loan_tokens_resource_def: ResourceAddress) -> ComponentAddress {

            // let rules = AccessRules::new()
            // .method("issue_new_credit_sbt", rule!(require(admin_badge)))
            // .method("review_installment_credit_request", rule!(require(admin_badge)))
            // .method("list_protocol", rule!(require(admin_badge)))
            // .method("delist_protocol", rule!(require(admin_badge)))
            // .method("blacklist", rule!(require(admin_badge)))
            // .method("whitelist", rule!(require(admin_badge)))
            // .method("change_credit_scoring_rate", rule!(require(admin_badge)))
            // .default(rule!(allow_all));

            let user_mint_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "User Mint Badge")
                .initial_supply(1);

            // let rules = AccessRules::new()
            //     .method("sell", rule!(require(user_mint_badge.resource_address())))
            //     .default(rule!(allow_all));                          

            let user_account_trading_history_nft = ResourceBuilder::new_non_fungible()
                .metadata("name", "User Account Trading History")
                .mintable(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Create the non fungible resource that will represent the lendings
            let user_account_funding_nft: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "User Account Funding Data NFTs")
                .mintable(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), MUTABLE(rule!(require(user_mint_badge.resource_address()))))
                .no_initial_supply();                 

            return Self {
                main_pool: Vault::new(xrd_address),
                token1_pool: Vault::new(token_1_address),
                lending_app: lending_app,
                trading_app: trading_app,
                user_account_trading_nft_resource_def: user_account_trading_history_nft,
                user_account_trading_history_nft_admin_badge: Vault::with_bucket(user_mint_badge),
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



        pub fn register(&mut self,address: ComponentAddress) -> Bucket {
            let nft = self.user_account_trading_history_nft_admin_badge.authorize(|| {
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
            nft
        }

        pub fn fund_portfolio(&mut self, starting_tokens: Bucket) -> Bucket {
            info!("=== FUND PORTFOLIO OPERATION START === ");
            let how_many: Decimal = starting_tokens.amount();
            self.main_pool.put(starting_tokens);
            let total_amount: Decimal = self.main_pool.amount();
            // The ratio funded compared to the total amount.
            let funded_ratio = how_many * dec!("100") / total_amount;
            let epoch_funded: u64 = Runtime::current_epoch();
            //update the total funded in the portfolio
            self.amount_funded = self.amount_funded  + how_many;

            let portfolio_id = get_non_fungible_id();             
            // Create a NFT. TODO: If this already exist this needs to be updated
            let user_account_funding_nft = self.user_account_trading_history_nft_admin_badge.authorize(|| {
                borrow_resource_manager!(self.user_account_funding_nft_resource_def)
                    .mint_non_fungible(&portfolio_id, UserAccountFundingData {xrd_tokens: how_many,  in_progress: true, total_amount, funded_ratio, epoch_funded  })
                });             
            user_account_funding_nft
        }

        pub fn withdraw_portfolio(&mut self, user_account_trading_nft: Proof) -> Bucket {
            info!("=== WITHDRAW PORTFOLIO OPERATION START === ");

            // Get the data associated with the Lending NFT and update the variable values
            let non_fungible: NonFungible<UserAccountFundingData> = user_account_trading_nft.non_fungible();
            let mut portfolio_nft_data = non_fungible.data();                
            
            let starting_epoch: u64 = portfolio_nft_data.epoch_funded;
            let actual_epoch: u64 = Runtime::current_epoch();
            let total_amount_at_the_time_of_funding: Decimal = portfolio_nft_data.total_amount;
            let total_amount_at_the_time_of_withdraw: Decimal = self.main_pool.amount();

            //update the total funded in the portfolio
            info!(" Amount of funded tokens in the portfolio {} " , self.amount_funded);  
            info!(" Amount of yours funded tokens in the portfolio {} " , portfolio_nft_data.xrd_tokens);  

            let portfolio_tokens_value: Decimal = self.portfolio_value();
            let total = portfolio_tokens_value+total_amount_at_the_time_of_withdraw;
            info!(" Portfolio amount at time of funding {} and actual {} " , total_amount_at_the_time_of_funding, total);  
            // The ratio of increment/decrease of the main pool.
            let diff_ratio: Decimal = ((total / self.amount_funded) * dec!("100") )-dec!(100);
            info!(" Portfolio increase/decrease ratio  {} " , diff_ratio);  

            //the amount of tokens to be returned with increase or decrease
            let diff_tokens = portfolio_nft_data.xrd_tokens * (dec!("100") + diff_ratio) / dec!("100");
         
            info!(" you got {} from {} in {} epoch " , diff_tokens , portfolio_nft_data.xrd_tokens , (actual_epoch-starting_epoch));            
            //return the tokens to the user account
            let to_be_returned: Bucket = self.main_pool.take(diff_tokens);

            //update the total funded in the portfolio
            self.amount_funded = self.amount_funded - total_amount_at_the_time_of_funding;
            info!(" Updated Amount of funded tokens  {} " , self.amount_funded);  

            // // Update the data on that NFT globally
            portfolio_nft_data.in_progress = false;
            portfolio_nft_data.xrd_tokens = Decimal::zero();
            portfolio_nft_data.total_amount = Decimal::zero();
            portfolio_nft_data.epoch_funded = Runtime::current_epoch();

            // portfolio_nft_data.xrd_tokens - tokens_to_withdraw;
            self.user_account_trading_history_nft_admin_badge.authorize(|| {
                borrow_resource_manager!(self.user_account_funding_nft_resource_def).update_non_fungible_data(&non_fungible.id(), portfolio_nft_data);
            });

            // // Burn the badge to only allow one call to approve per badge
            // self.username_nft_admin_badge.authorize(|| {
            //     ticket.burn();
            // });
            // self.username_nft_admin_badge.authorize(|| {
            //     borrow_resource_manager!(self.portfolio_nft_resource_def).burn(Bucket::new(ticket));
            // });
            to_be_returned
        }

       /// # Execute a buy operation by means of the portfolio.
       pub fn buy(&mut self,xrd_tokens: Decimal, user_account: ComponentAddress, token_to_buy: ResourceAddress)   {
            assert!(
                self.main_pool.amount() >= xrd_tokens,
                "Main vault has not sufficient tokens to buy ! Please fund portfolio !"
            );   
            let trading_app: TradingApp = self.trading_app.into();
            self.token1_pool.put(trading_app.buy(self.main_pool.take(xrd_tokens)));
            
            let current_price = trading_app.current_price(RADIX_TOKEN,token_to_buy);
            let how_many = xrd_tokens / current_price;

            let trade1 = OperationHistory {
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

            self.positions.push(trade1);

            // let user_account_history_id = get_non_fungible_id();             
            // // Create a NFT. Note that this contains the number of lending and the level arwarded
            // let user_account_history_nft = self.user_account_trading_history_nft_admin_badge.authorize(|| {
            //     borrow_resource_manager!(self.user_account_funding_nft_resource_def)
            //         .mint_non_fungible(&user_account_history_id, trade1)
            //     });             
            // user_account_history_nft                   
        }

        pub fn sell(&mut self,tokens: Decimal
        )   {
            let trading_app: TradingApp = self.trading_app.into();
            self.main_pool.put(trading_app.sell(self.token1_pool.take(tokens)));
        }

        pub fn portfolio_value(&self) -> Decimal {
            info!("Position size inside portfolio {}", self.positions.len());
            let trading_app: TradingApp = self.trading_app.into();

            // let mut total: Decimal = Decimal::zero();
            // for inner_position in &self.positions {
            //     info!("Position Id {}", inner_position.operation_id);          
            //     info!("Ready to get current price ");
            //     let updated_value = trading_app.current_price(inner_position.token_a_address,inner_position.token_b_address);
            //     info!("Xrd used for trade {} at Current price {:?} ", inner_position.xrd_tokens, updated_value);
            //     total = total + (inner_position.xrd_tokens*updated_value);
            //     info!("Calculated value {:?}", total);
            // }      
            
            let total: Decimal = self.token1_pool.amount()*(trading_app.current_price(RADIX_TOKEN,self.token1_pool.resource_address()));
            info!("Added value from token1 vault {:?}", total);

            total
        }

        pub fn portfolio_total_value(&self) -> Decimal {
            info!("Position size inside portfolio {}", self.positions.len());
            let trading_app: TradingApp = self.trading_app.into();
            
            let mut total: Decimal = self.portfolio_value();
            info!("Added value from token1 vault {:?}", total);
            let totalxrd: Decimal = self.main_pool.amount();
            info!("Value in main vault {:?}", totalxrd);
            total = total + totalxrd;
            info!("Grandtotal {:?}", total);

            total
        }
   

        pub fn position(&self) -> Vec<u128> {
            info!("Position size {}", self.positions.len());
            let trading_app: TradingApp = self.trading_app.into();

            let mut losing_positions: Vec<u128> = Vec::new();
            let mut result: Decimal = Decimal::zero();
            for inner_position in &self.positions {
                info!("Inner Position {}", inner_position.to_string());    
                info!("Position Id {}", inner_position.operation_id);          

                info!("Ready to get current price ");
                let updated_value = trading_app.current_price(inner_position.token_a_address,inner_position.token_b_address);
                info!("Xrd used for trade {}", inner_position.xrd_tokens);
                info!("Starting price {:?}", inner_position.current_price );
                info!("Current price {:?}", updated_value );
                let net_result = updated_value.wrapping_sub(inner_position.current_price);
                info!("Position net result {:?}", net_result);

                if net_result >= 0 {
                    let trade1 = OperationHistory {
                        username: inner_position.username,
                        operation_id: inner_position.operation_id,    
                        xrd_tokens: inner_position.xrd_tokens,    
                        current_price: inner_position.current_price,    
                        token_a_address: RADIX_TOKEN,
                        token_b_address: inner_position.token_b_address,
                        num_token_b_received: inner_position.num_token_b_received,
                        date_opened: inner_position.date_opened,
                        date_closed: None,
                        current_requestor_for_closing: None, 
                        current_standing: None,
                        number_of_request_for_autoclosing: None,
                    };
                    losing_positions.push(inner_position.operation_id);
                };

            }        

            losing_positions
        }

        pub fn close_position(&mut self, operation_id: u128)  {
            info!("Position size {}", self.positions.len());
            let mut amount_to_sell: Decimal = Decimal::zero();
            for inner_position in &self.positions {
                info!("Inner Position {}", inner_position.to_string());           
                info!("Position Id {}", inner_position.operation_id);    

                if inner_position.operation_id==operation_id {
                    amount_to_sell = inner_position.num_token_b_received.clone();
                }
            }    
            info!("Ready to close position {}", operation_id);
            self.sell(amount_to_sell);    
        }


        pub fn register_for_lending(&mut self)  {
            info!("Registering for lending ") ;
            info!("Vault for Lending NFT, accept resource address : {:?} ", self.lending_nft_vault.resource_address());
            let lending_app: LendingApp = self.lending_app.into();
            let bucket: Bucket = lending_app.register();
            self.lending_nft_vault.put(bucket);
        }
        pub fn register_for_borrowing(&mut self) {
            info!("Registering for borrowing ") ;
            info!("Vault for Borrowing NFT, accept resource address : {:?} ", self.borrowing_nft_vault.resource_address());
            let lending_app: LendingApp = self.lending_app.into();
            let bucket: Bucket = lending_app.register_borrower();
            self.borrowing_nft_vault.put(bucket);            
        }     

        pub fn lend(&mut self,tokens: Bucket) -> Bucket {
            info!("Lending ");
            let lending_app: LendingApp = self.lending_app.into();
            let proof: Proof = self.lending_nft_vault.create_proof_by_amount(dec!(1));
            
            return lending_app.lend_money(tokens, proof);
        }

        pub fn take_back(&mut self, lnd_tokens: Bucket) -> Bucket {
            info!("Take back ");
            let lending_app: LendingApp = self.lending_app.into();
            let proof: Proof = self.lending_nft_vault.create_proof_by_amount(dec!(1));
            return lending_app.take_money_back(lnd_tokens, proof);
        }

    }
}