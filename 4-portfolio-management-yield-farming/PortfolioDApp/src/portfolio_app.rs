use scrypto::prelude::*;
use crate::lending_app::LendingApp;
use crate::trading_app::TradingApp;
use crate::utils::*;

// Here, we define the data that will be present in
// each of the user . 
#[derive(NonFungibleData)]
struct UserHistory {
    #[scrypto(mutable)]
    username: ResourceAddress,
    #[scrypto(mutable)]   
    positive_operation: u32,    
    #[scrypto(mutable)]   
    negative_operation: u32, 
    #[scrypto(mutable)]   
    expert: bool          
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

        /// The resource definition of UserHistory token.
        username_nft_resource_def: ResourceAddress,
        /// Vault with admin badge for managine UserHistory NFT.
        username_nft_admin_badge: Vault,       
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
            trading_app: ComponentAddress) -> ComponentAddress {

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

            let username_nft = ResourceBuilder::new_non_fungible()
                .metadata("name", "Username History")
                .mintable(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(user_mint_badge.resource_address())), LOCKED)
                .no_initial_supply();

            return Self {
                main_pool: Vault::new(xrd_address),
                token1_pool: Vault::new(token_1_address),
                lending_app: lending_app,
                trading_app: trading_app,
                username_nft_resource_def: username_nft,
                username_nft_admin_badge: Vault::with_bucket(user_mint_badge),
            }
            .instantiate()
            // .add_access_check(rules)
            .globalize();            
        }



        pub fn register(&mut self,address: ResourceAddress) -> Bucket {

            let nft = self.username_nft_admin_badge.authorize(|| {
                let resource_manager = borrow_resource_manager!(self.username_nft_resource_def);
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

        pub fn fund_portfolio(&mut self, starting_tokens: Bucket) {
            info!("=== FUND PORTFOLIO OPERATION START === ");
                self.main_pool.put(starting_tokens);
        }

       /// # Execute a buy operation by means of the portfolio.
       pub fn buy(&mut self,xrd_tokens: Decimal
        )   {
            let trading_app: TradingApp = self.trading_app.into();
            // let value: String = "yes".to_string();
            self.token1_pool.put(trading_app.buy(self.main_pool.take(xrd_tokens)));
        }

        pub fn sell(&mut self,tokens: Decimal
        )   {
            let trading_app: TradingApp = self.trading_app.into();
            self.main_pool.put(trading_app.sell(self.token1_pool.take(tokens)));
        }


        pub fn register_for_lending(&mut self,address: ResourceAddress) -> Bucket {
            info!("Registering for lending with {} ", address) ;
            let lending_app: LendingApp = self.lending_app.into();
            return lending_app.register();
        }


        pub fn lend(
            &mut self,
            tokens: Bucket,
            ticket: Proof,
        ) -> Bucket {
            info!("Lending ");
            let lending_app: LendingApp = self.lending_app.into();
            return lending_app.lend_money(tokens, ticket);
        }

            // This is a pseudorandom function and not a true random number function.
    // pub fn get_random(&self) -> u128 {
    //     let multiplier = self.players.clone().into_iter()
    //         .map(|(_,p)| p.guess)
    //         .reduce(|a,b| a * b).unwrap_or(1);

    //     Runtime::generate_uuid() / multiplier
    // }

    // let random_number = (self.get_random() % 6) + 1;
    

    }
}