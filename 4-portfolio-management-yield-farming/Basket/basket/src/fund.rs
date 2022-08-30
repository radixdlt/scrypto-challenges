use scrypto::prelude::*;
use crate::auction::Auction;

// import fake amm pool blueprint as a generic interface
import! { r#"
{
    "package_address": "",
    "blueprint_name": "AmmPool",
    "functions": [],
    "methods": [
        {
        "name": "swap",
        "mutability": "Mutable",
        "inputs": [
            {
            "type": "Custom",
            "name": "Bucket",
            "generics": []
            }
        ],
        "output": {
            "type": "Custom",
            "name": "Bucket",
            "generics": []
        }
        },
        {
        "name": "get_pair",
        "mutability": "Immutable",
        "inputs": [],
        "output": {
            "type": "Tuple",
            "elements": [
            {
                "type": "Custom",
                "name": "ResourceAddress",
                "generics": []
            },
            {
                "type": "Custom",
                "name": "ResourceAddress",
                "generics": []
            }
            ]
        }
        }
    ]
}
"# }

// nft that represents a stake
#[derive(NonFungibleData)]
pub struct StakeReceipt {
    investment: usize,                  // index of investment
    stake: Decimal,                     // amount of fund tokens staked
    #[scrypto(mutable)]
    price: Decimal,                     // price of tokens bought
    #[scrypto(mutable)]
    reward: Decimal,                    // reward for performance 
    #[scrypto(mutable)]
    status: Status,                     // status of stake 
}

// status of a stake 
// progresses in the order Staking -> Staked -> Unstaking -> Unstaked
#[derive(Encode, Decode, TypeId, Describe)]
pub enum Status {
    Staking,
    Staked,
    Unstaking,
    Unstaked,
}

#[derive(Encode, Decode, TypeId, Describe)]
pub struct Investment {
    amm_pool: ComponentAddress,                        // AMM pool used for mint/redeem
    tokens: Vault,                                     // tokens that make up the investment
    stake_pool: Vault,                                 // tokens that are staked to the investment
    buy_auction: ComponentAddress,                     // auction used to buy tokens for the investment
    buy_auction_badge: Vault,                          // badge to authorize buy auction
    sell_auction: ComponentAddress,                    // auction used to sell tokens for the investment
    sell_auction_badge: Vault,                         // badge to authorize sell auction
}

#[derive(Encode, Decode, TypeId, Describe)]
pub struct Stake {
    epoch: u64,                                        // epoch the stake was created
    stake_receipt: NonFungibleId,                      // id of the stake receipt the stake is for
}

#[derive(Encode, Decode, TypeId, Describe)]
pub struct Unstake {
    epoch: u64,                                        // epoch the unstake was created
    stake_receipt: NonFungibleId,                      // id of the stake receipt the unstake is for
}

blueprint! {
    struct Fund {
        name: String,                                  // name of the fund
        mutable: bool,                                 // if the fund is still mutable for the admin token
        denominator_token_address: ResourceAddress,    // the token that the fund values other tokens in terms of
        fund_token_address: ResourceAddress,           // fund token that represents a part ownership of the fund 
        stake_receipt_address: ResourceAddress,        // nft that represents a stake
        internal_badge: Vault,                         // badge used for all internal permission (minting, burning, etc.)
        fee_percent: Decimal,                          // percent of buying xrd transfered to stakers
        auction_delay: u64,                            // delay be before auctions are processed to allow the market to participate
        denominator_tokens: Vault,                     // denominator tokens reserve
        fund_tokens: Vault,                            // not staked fund tokens
        investments: Vec<Investment>,                  // list of investments in the fund
        staking_queue: Vec<Stake>,                     // list of stakes to process
        unstaking_queue: Vec<Unstake>,                 // list of unstakes to process
    }

    impl Fund {
        // instantiates fund
        // returns (fund_address, admin_badge)
        // admin_badge is used to add tokens to the fund
        pub fn new(
            name: String, 
            token_name: String, 
            token_symbol: String, 
            denominator_token_address: ResourceAddress, 
            auction_delay: u64, 
            fee_percent: Decimal) -> (ComponentAddress, Bucket) {

            // assert it is a valid delay
            assert!(
                auction_delay > 1,
                "Invalid auction_delay value."
            );
            
            // assert it is a valid fee percent
            assert!(
                fee_percent >= dec!(0) && fee_percent <= dec!(10),
                "Invalid fee_percent value."
            );

            // mint admin badge
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Fund Admin Badge")
                .metadata("symbol", "ADMIN")    
                .initial_supply(1);

            // mint internal badge used for minting and burning
            let internal_badge: Bucket = ResourceBuilder::new_fungible()
                .initial_supply(1);

            // create fund token resource manager
            let fund_token_address: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", token_name.clone())
                .metadata("symbol", token_symbol)
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // create stake receipt resource manager
            let stake_receipt_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", token_name + " Stake Receipt")
                .metadata("symbol", "STAKE")
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(internal_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // create method permissions
            let auth: AccessRules = AccessRules::new()
                .method("set_immutable", rule!(require(admin_badge.resource_address())))
                .method("add_investment", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            // instantiate and return
            let fund: ComponentAddress = Self {
                    name: name,
                    mutable: true,
                    denominator_token_address: denominator_token_address,
                    fund_token_address: fund_token_address,
                    stake_receipt_address: stake_receipt_address,
                    internal_badge: Vault::with_bucket(internal_badge),
                    fee_percent: fee_percent,
                    auction_delay: auction_delay,
                    denominator_tokens: Vault::new(denominator_token_address),
                    fund_tokens: Vault::new(fund_token_address),
                    investments: Vec::new(),
                    staking_queue: Vec::new(),
                    unstaking_queue: Vec::new(),
                }
                .instantiate()
                .add_access_check(auth)
                .globalize();

            (fund, admin_badge)
        }

        // makes the fund immutable so the admin can no longer change the fund
        pub fn set_immutable(&mut self) {
            // assert fund is mutable
            assert!(
                self.mutable,
                "Fund is not mutable."
            );

            self.mutable = false;
        }

        // adds a investment for the fund
        pub fn add_investment(&mut self, amm_pool: ComponentAddress) {
            // assert fund is mutable
            assert!(
                self.mutable,
                "Fund is not mutable."
            );

            // assert the amm pool is valid
            let pool: AmmPool = amm_pool.into();
            let pair: (ResourceAddress, ResourceAddress) = pool.get_pair();
            assert!(
                pair.1 == self.denominator_token_address,
                "Second token in pair must be denominator token."
            );

            // create auctions used for rebalancing
            let (buy_auction, buy_auction_badge): (ComponentAddress, Bucket) = Auction::new(pair.0, pair.1);
            let (sell_auction, sell_auction_badge): (ComponentAddress, Bucket) = Auction::new(pair.1, pair.0);

            // add investment
            self.investments.push(
                Investment {
                    amm_pool: amm_pool,
                    tokens: Vault::new(pair.0),
                    stake_pool: Vault::new(self.fund_token_address),
                    buy_auction: buy_auction,
                    buy_auction_badge: Vault::with_bucket(buy_auction_badge),
                    sell_auction: sell_auction,
                    sell_auction_badge: Vault::with_bucket(sell_auction_badge),
                }
            );
        }
        
        // mints fund tokens using denominator tokens
        // fund buys investment tokens using the denominator tokens in proportion to stakes or keeps denominator tokens if no stakers
        // returns fund tokens
        pub fn mint(&mut self, mut denominator_tokens: Bucket) -> Bucket {
            let denominator_amount: Decimal = denominator_tokens.amount();

            // if no current supply
            let fund_token_manager: &ResourceManager = borrow_resource_manager!(self.fund_token_address);
            let total_supply = fund_token_manager.total_supply();
            let mint_amount: Decimal = if total_supply == dec!(0) {
                denominator_amount
            } else {
                // if stakers
                let total_stake: Decimal = self.get_total_stake();
                let relative_value: Decimal = if total_stake > dec!(0) {
                    // buy tokens according to stake ratios
                    let mut value_sum: Decimal = dec!(0);
                    for investment in &mut self.investments {
                        let calculated_buy_amount = denominator_amount * investment.stake_pool.amount() / total_stake;
                        let buy_amount: Decimal = calculated_buy_amount.min(denominator_tokens.amount());
                        
                        if buy_amount > dec!(0) {
                            let pool: AmmPool = investment.amm_pool.into();
                            let tokens: Bucket = pool.swap(denominator_tokens.take(buy_amount));
                            value_sum += tokens.amount() / investment.tokens.amount() * investment.stake_pool.amount();
                            investment.tokens.put(tokens);
                        }
                    }

                    value_sum / total_stake
                } else {
                    denominator_amount / self.denominator_tokens.amount()
                };
                
                total_supply * relative_value
            };

            // put denominator tokens remainder in reserve
            self.denominator_tokens.put(denominator_tokens);

            // mint fund tokens
            let fund_tokens: Bucket = self.internal_badge.authorize(|| {
                fund_token_manager.mint(mint_amount)
            });

            // return minted tokens
            fund_tokens
        }

        // redeems fund tokens for denominator tokens
        // fund sells investment tokens for denominator tokens or takes denominator tokens from the reserve
        // returns denominator tokens
        pub fn redeem(&mut self, fund_tokens: Bucket) -> Bucket {
            // calculate amount of the fund owned by the tokens and take entitled amount from the reserve
            let fund_token_manager: &ResourceManager = borrow_resource_manager!(self.fund_token_address);
            let amount_ownership: Decimal = fund_tokens.amount() / fund_token_manager.total_supply();
            let mut denominator_tokens: Bucket = self.denominator_tokens.take(self.denominator_tokens.amount() * amount_ownership);
            
            // sell entitled amount of investments
            for investment in &mut self.investments {
                let sell_amount: Decimal = investment.tokens.amount() * amount_ownership;
                let pool: AmmPool = investment.amm_pool.into();
                let tokens: Bucket = pool.swap(investment.tokens.take(sell_amount));
                denominator_tokens.put(tokens);
            }
            
            // burn fund tokens
            self.internal_badge.authorize(|| {
                fund_tokens.burn()
            });

            // return tokens from reserve and sales
            denominator_tokens
        }

        
        // redeems fund tokens for investment tokens
        // returns vector of tokens
        pub fn redeem_for_tokens(&mut self, fund_tokens: Bucket) -> Vec<Bucket> {
            let mut tokens_vec: Vec<Bucket> = Vec::new();

            // calculate amount of the fund owned by the tokens and take entitled amount from the reserve
            let fund_token_manager: &ResourceManager = borrow_resource_manager!(self.fund_token_address);
            let amount_ownership: Decimal = fund_tokens.amount() / fund_token_manager.total_supply();
            let denominator_tokens: Bucket = self.denominator_tokens.take(self.denominator_tokens.amount() * amount_ownership);
            tokens_vec.push(denominator_tokens);
            
            // get entitled amount of investments
            for investment in &mut self.investments {
                let take_amount: Decimal = investment.tokens.amount() * amount_ownership;
                let tokens: Bucket = investment.tokens.take(take_amount);

                tokens_vec.push(tokens);
            }
            
            // burn fund tokens
            self.internal_badge.authorize(|| {
                fund_tokens.burn()
            });

            // return vector of tokens
            tokens_vec
        }

        // creates a stake for an investment
        // queues auction to buy tokens
        // returns stake receipt nft
        pub fn stake(&mut self, fund_tokens: Bucket, investment: usize) -> Bucket {
            // assert the investment exists in the fund
            assert!(
                investment < self.investments.len(),
                "Investment does not exist."
            );

            // deposit fund tokens
            let stake: Decimal = fund_tokens.amount();
            assert!(
                stake > dec!(0),
                "Stake must me greater than zero."
            );
            self.fund_tokens.put(fund_tokens);

            // mint the stake receipt
            let stake_manager: &ResourceManager = borrow_resource_manager!(self.stake_receipt_address);
            let id: NonFungibleId = NonFungibleId::random();
            let stake_receipt: Bucket = self.internal_badge.authorize(|| {
                stake_manager.mint_non_fungible(
                    &id,
                    StakeReceipt {
                        investment: investment,
                        stake: stake,
                        price: dec!(0),
                        reward: dec!(0),
                        status: Status::Staking,
                    }
                )
            });

            // add to staking queue  
            self.staking_queue.push(
                Stake {
                    epoch: Runtime::current_epoch(),
                    stake_receipt: id,
                }
            );

            // return the minted stake receipt
            stake_receipt
        }

        // starts unstaking for given staked stake receipt
        // queues auction to sell tokens
        // returns updated stake receipt nft
        pub fn unstake(&mut self, stake_receipt: Bucket) -> Bucket {
            // assert it is a valid stake receipt
            assert!(
                stake_receipt.resource_address() == self.stake_receipt_address,
                "Invalid stake receipt."
            );
            
            let stake_receipt_nft: NonFungible<StakeReceipt> = stake_receipt.non_fungible();
            match stake_receipt_nft.data().status {
                Status::Staking => {
                    panic!("Currently staking. Can not unstake.")
                },
                Status::Staked => {
                    // update state
                    let mut stake_receipt_data: StakeReceipt = stake_receipt_nft.data();
                    stake_receipt_data.status = Status::Unstaking;

                    self.internal_badge.authorize(|| {
                        stake_receipt_nft.update_data(stake_receipt_data);
                    });

                    // add to unstaking queue  
                    self.unstaking_queue.push(
                        Unstake {
                            epoch: Runtime::current_epoch(),
                            stake_receipt: stake_receipt_nft.id(),
                        }
                    );

                    // return updated stake receipt
                    stake_receipt
                },
                Status::Unstaking => {
                    panic!("Already unstaking.")
                },
                Status::Unstaked => {
                    panic!("Already unstaked. To collect tokens call collect_unstaked.")
                },
            }
        }

        // collects fund tokens for given unstaked stake receipt
        // returns fund tokens
        pub fn collect_unstaked(&mut self, stake_receipt: Bucket) -> Bucket {
            // assert it is a valid stake receipt
            assert!(
                stake_receipt.resource_address() == self.stake_receipt_address,
                "Invalid stake receipt."
            );
            
            let stake_receipt_nft: NonFungible<StakeReceipt> = stake_receipt.non_fungible();
            match stake_receipt_nft.data().status {
                Status::Staking => {
                    panic!("Currently staking. Can not unstake.")
                },
                Status::Staked => {
                    panic!("Currently staked. To unstake call unstake.")
                },
                Status::Unstaking => {
                    panic!("Already unstaking.")
                },
                Status::Unstaked => {
                    let stake_receipt_data: StakeReceipt = stake_receipt_nft.data();
                    let fund_tokens: Bucket = self.fund_tokens.take(stake_receipt_data.stake + stake_receipt_data.reward);

                    self.internal_badge.authorize(|| {
                        stake_receipt.burn();
                    });

                    fund_tokens
                },
            }
        }

        // processes stakes in staking queue
        pub fn process_staking(&mut self) {
            let stake_manager: &ResourceManager = borrow_resource_manager!(self.stake_receipt_address);
            let epoch: u64 = Runtime::current_epoch();

            // for each stake past delay
            let mut idx: usize = 0;
            while idx < self.staking_queue.len() && self.staking_queue[idx].epoch + self.auction_delay < epoch {
                let mut stake_receipt_data: StakeReceipt = stake_manager.get_non_fungible_data(&self.staking_queue[idx].stake_receipt);
                
                // if currently stakers
                let total_stake: Decimal = self.get_total_stake();
                if total_stake > dec!(0) {
                    // sell tokens to account for new stake
                    let change: Decimal = dec!(1) / total_stake - dec!(1) / (total_stake + stake_receipt_data.stake);
                    for investment in &mut self.investments {
                        let auction: Auction = investment.sell_auction.into();
                        let sell_amount: Decimal = change * investment.stake_pool.amount() * investment.tokens.amount();        
                        let (tokens, remainder): (Bucket, Bucket) = investment.sell_auction_badge.authorize(|| {
                            auction.auction(investment.tokens.take(sell_amount))
                        });

                        self.denominator_tokens.put(tokens);
                        investment.tokens.put(remainder);
                    }
                }

                let investment: &mut Investment = &mut self.investments[stake_receipt_data.investment];

                // buy tokens using available capital 
                let auction: Auction = investment.buy_auction.into();
                let mut denominator_amount: Decimal = self.denominator_tokens.amount();
                let (tokens, remainder): (Bucket, Bucket) = investment.buy_auction_badge.authorize(|| {
                    auction.auction(self.denominator_tokens.take_all())
                });

                // calculate price
                let tokens_amount: Decimal = tokens.amount();
                denominator_amount -= remainder.amount();
                let price: Decimal = if tokens_amount > dec!(0) {
                    denominator_amount / tokens_amount
                } else {
                    dec!(0)
                };

                // deposit tokens
                investment.tokens.put(tokens);
                self.denominator_tokens.put(remainder);

                if price > dec!(0) {    // auction was successful
                    // move stake to staking pool
                    investment.stake_pool.put(self.fund_tokens.take(stake_receipt_data.stake));
                    
                    // update stake receipt as staked
                    stake_receipt_data.status = Status::Staked;
                    stake_receipt_data.price = price;
                } else {                // auction was not successful
                    // update stake receipt as unstaked
                    stake_receipt_data.status = Status::Unstaked;
                }

                self.internal_badge.authorize(|| {
                    stake_manager.update_non_fungible_data(&self.staking_queue[idx].stake_receipt, stake_receipt_data);
                });

                // increment to go to next in queue
                idx += 1;
            }

            // remove processed stakes from queue
            self.staking_queue.drain(..idx);
        }

        // processes stakes in unstaking queue
        pub fn process_unstaking(&mut self) {
            let stake_manager: &ResourceManager = borrow_resource_manager!(self.stake_receipt_address);
            let epoch: u64 = Runtime::current_epoch();

            // for each unstake past delay
            let mut idx: usize = 0;
            while idx < self.unstaking_queue.len() && self.unstaking_queue[idx].epoch + self.auction_delay < epoch {
                let mut stake_receipt_data: StakeReceipt = stake_manager.get_non_fungible_data(&self.unstaking_queue[idx].stake_receipt);
                let investment: &mut Investment = &mut self.investments[stake_receipt_data.investment];

                // sell tokens to account for unstake
                let auction: Auction = investment.sell_auction.into();
                let sell_amount: Decimal = stake_receipt_data.stake / investment.stake_pool.amount() * investment.tokens.amount();
                let (tokens, remainder): (Bucket, Bucket) = investment.sell_auction_badge.authorize(|| {
                    auction.auction(investment.tokens.take(sell_amount))
                });

                // calculate price
                let denominator_amount: Decimal = tokens.amount();
                let tokens_amount: Decimal = sell_amount - remainder.amount();
                let price: Decimal = if tokens_amount > dec!(0) {
                    denominator_amount / tokens_amount
                } else {
                    dec!(0)
                };

                // deposit tokens
                self.denominator_tokens.put(tokens);
                investment.tokens.put(remainder);

                // remove stake from stake pool
                let mut fund_tokens: Bucket = investment.stake_pool.take(stake_receipt_data.stake);

                // if other stakers
                let total_stake: Decimal = self.get_total_stake();
                let relative_value: Decimal = if total_stake > dec!(0) {
                    // buy tokens according to stake ratios
                    let mut value_sum: Decimal = dec!(0);
                    for investment in &mut self.investments {
                        let calculated_buy_amount = denominator_amount * investment.stake_pool.amount() / total_stake;
                        let buy_amount: Decimal = calculated_buy_amount.min(self.denominator_tokens.amount());
                        
                        if buy_amount > dec!(0) {
                            let auction: Auction = investment.buy_auction.into();
                            let (tokens, remainder): (Bucket, Bucket) = investment.buy_auction_badge.authorize(|| {
                                auction.auction(self.denominator_tokens.take(buy_amount))
                            });
                            value_sum += tokens.amount() / investment.tokens.amount() * investment.stake_pool.amount();

                            investment.tokens.put(tokens);
                            self.denominator_tokens.put(remainder);
                        }
                    }

                    value_sum / total_stake
                } else {
                    dec!(1)
                };

                // reward for performance
                let reward: Decimal = if price == dec!(0) {     // missing price data
                    dec!(0)
                } else if price > stake_receipt_data.price {    // positive performance
                    let fund_token_manager: &ResourceManager = borrow_resource_manager!(self.fund_token_address);
                    let total_supply = fund_token_manager.total_supply();
                    
                    // mint tokens with value equal to fee_percent of gained value
                    let mint_amount: Decimal = (dec!(1) - stake_receipt_data.price / price) * total_supply * relative_value * self.fee_percent / dec!(100);
                    let fee_fund_tokens: Bucket = self.internal_badge.authorize(|| {
                        fund_token_manager.mint(mint_amount)
                    });

                    fund_tokens.put(fee_fund_tokens);

                    mint_amount
                } else {                                        // negative performance
                    // burn percent of tokens equal to percent performance                            
                    let burn_amount: Decimal = (dec!(1) - price / stake_receipt_data.price) * fund_tokens.amount();
                    let burn_fund_tokens: Bucket = fund_tokens.take(burn_amount);
                    self.internal_badge.authorize(|| {
                        burn_fund_tokens.burn()
                    });

                    -burn_amount
                };

                // put unstaked tokens in storage vault
                self.fund_tokens.put(fund_tokens);

                // update stake receipt
                stake_receipt_data.reward = reward;
                stake_receipt_data.status = Status::Unstaked;

                self.internal_badge.authorize(|| {
                    stake_manager.update_non_fungible_data(&self.unstaking_queue[idx].stake_receipt, stake_receipt_data);
                });

                // increment to go to next in queue
                idx += 1;
            }

            // remove processed unstakes from queue
            self.unstaking_queue.drain(..idx);
        }

        // returns total amount of staked tokens
        fn get_total_stake(&self) -> Decimal {
            let mut sum: Decimal = dec!(0);
            for investment in &self.investments {
                sum += investment.stake_pool.amount();
            }

            sum
        }
    }
}