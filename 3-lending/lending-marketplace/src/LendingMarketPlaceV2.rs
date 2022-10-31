use crate::dutch_auction::DutchAuction;
use sbor::*;
use scrypto::prelude::*;

blueprint! {
    struct LendingMarketPlace {
        /// The liquidity pool
        liquidity_pool: Vault,
        /// the  epoch for when the liqudation period starts
        liquidation_period: u64,
        /// User state
        users: LazyMap<ResourceAddress, User>,
        /// nft valut ( todo)
        nft_vaults: HashMap<ResourceAddress, Vault>,
        /// This is the vault which stores the payment of the NFTs once it has been made. This vault may contain XRD or
        payment_vault: Vault,
        /// price for the nft to be sold
        price: Decimal,
        token_address: ResourceAddress,


    }

    impl LendingMarketPlace {
        /// Creates a lending pool
        pub fn instantiate_LendingMarketPlace(reserve_address: ResourceAddress ) -> ComponentAddress {

            // Create a new HashMap of vaults and aggregate all of the tokens in the buckets into the vaults of this
            // HashMap. This means that if somebody passes multiple buckets of the same resource, then they would end
            // up in the same vault.

            let mut nft_vaults: HashMap<ResourceAddress, Vault> = HashMap::new();
            // for bucket in non_fungible_tokens.into_iter() {
            //     nft_vaults
            //         .entry(bucket.resource_address())
            //         .or_insert(Vault::new(bucket.resource_address()))
            //         .put(bucket)
            // }


            // borrower badage
            let borrower_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Borrower Badge")
                .metadata(
                    "description",
                    "An Borrower badge used to authenticate the owner of the NFT(s).",
                )
                .metadata("symbol", "OWNER")
                .initial_supply(1);

            //access rule
            let access_rule: AccessRule = rule!(require(borrower_badge.resource_address()));
            let access_rules: AccessRules = AccessRules::new()
                .method("repay", access_rule.clone())
                .default(rule!(allow_all));

            Self {
                liquidity_pool: Vault::new(reserve_address),
                liquidation_period:50u64,
                users: LazyMap::new(),
                nft_vaults,
                payment_vault: Vault::new(reserve_address),
                price: dec!("30"),
                token_address: reserve_address,

            }
            .instantiate()
            .globalize()
        }



        /// Registers a new user
        pub fn new_user(&self) -> Bucket {
            ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "LendingMarketPlace User Badge")
                .initial_supply(1)
        }

        /// Deposits into the liquidity pool
        pub fn deposit(&mut self, user_auth: Proof, reserve_tokens: Bucket) {
            let user_id = Self::get_user_id(user_auth);
            let amount = reserve_tokens.amount();

            // Update user state
            let user = match self.users.get(&user_id) {
                Some(mut user) => {
                    user.on_deposit(amount);
                    user
                }
                None => User {
                    deposit_balance: amount,
                    borrow_balance: Decimal::zero(),
                    deposit_last_update: Runtime::current_epoch(),
                    borrow_last_update: Runtime::current_epoch(),
                },
            };

            // Commit state changes
            self.users.insert(user_id, user);
            self.liquidity_pool.put(reserve_tokens);
        }

        /// Redeems the underlying assets, partially or in full.
        pub fn redeem(&mut self, user_auth: Proof, amount: Decimal) -> Bucket {
            let user_id = Self::get_user_id(user_auth);

            // Update user state
            let mut user = self.get_user(user_id);
            let to_return_amount = user.on_redeem(amount);

            debug!(
                "LP balance: {}, redeemded: {}",
                self.liquidity_pool.amount(),
                to_return_amount
            );

            // Commit state changes
            self.users.insert(user_id, user);
            self.liquidity_pool.take(to_return_amount)
        }

        /// Borrows 30XRD frin the pool by collateralizing the underlying assets.
        pub fn borrow(&mut self, user_auth: Proof ,non_fungible_tokens: Vec<Bucket> ) -> Bucket {
            let user_id = Self::get_user_id(user_auth);
            for bucket in non_fungible_tokens.into_iter() {
                self.nft_vaults
                    .entry(bucket.resource_address())
                    .or_insert(Vault::new(bucket.resource_address()))
                    .put(bucket)
            }

            // Update user state
            let mut user = self.get_user(user_id);
            user.on_borrow(self.price);

            // Commit state changes
            self.users.insert(user_id, user);
            self.liquidity_pool.take(self.price)
        }

        /// Repays a loan, partially or in full.
        pub fn repay(&mut self, user_auth: Proof, mut repaid: Bucket) -> (Bucket, Vec<Bucket>) {
            let user_id = Self::get_user_id(user_auth);

            // Update user state
            let mut user = self.get_user(user_id);
            let to_return_amount = user.on_repay(repaid.amount());
            let to_return = repaid.take(to_return_amount);

            // Taking out all of the tokens from the vaults and returning them back to the caller.
            let resource_addresses: Vec<ResourceAddress> =
                self.nft_vaults.keys().cloned().collect();
            let mut tokens: Vec<Bucket> = Vec::new();
            for resource_address in resource_addresses.into_iter() {
                tokens.push(
                    self.nft_vaults
                        .get_mut(&resource_address)
                        .unwrap()
                        .take_all(),
                )
            }

            // Commit state changes
            self.users.insert(user_id, user);
            self.liquidity_pool.put(repaid);

            return (to_return, tokens);
        }

        /// Liquidates one user's position, if it's under collateralized.
        pub fn liquidate(&mut self, user_id: ResourceAddress, repaid: Bucket) -> () {
            let mut user = self.get_user(user_id);

            // Check time passed  size
            // assert!(
            //     repaid.amount() <= user.borrow_balance * self.max_liquidation_percent,
            //     "Max liquidation percent exceeded."
            // );


            let starting_price = dec!("90");
            let ending_price = dec!("40");
            let starting_epoch = Runtime::current_epoch() ;
            let ending_epoch = starting_epoch + self.liquidation_period;

            let resource_addresses: Vec<ResourceAddress> =
                self.nft_vaults.keys().cloned().collect();
            let mut tokens: Vec<Bucket> = Vec::new();
            for resource_address in resource_addresses.into_iter() {
                tokens.push(
                    self.nft_vaults
                        .get_mut(&resource_address)
                        .unwrap()
                        .take_all(),
                )
            }

            let (auction, bucket) = DutchAuction::instantiate_dutch_auction(tokens, self.token_address, starting_price, ending_price, self.liquidation_period);




            // Update user state and remove the nft badage

            // let to_return_amount = user.on_liquidate(repaid.amount());
            // let to_return = self.liquidity_pool.take(to_return_amount);

            // Commit state changes
            self.users.insert(user_id, user);
            // to_return
        }

        /// Returns the current state of a user.
        pub fn get_user(&self, user_id: ResourceAddress) -> User {
            match self.users.get(&user_id) {
                Some(user) => user,
                _ => panic!("User not found"),
            }
        }

        /// Parse user id from a proof.
        fn get_user_id(user_auth: Proof) -> ResourceAddress {
            assert!(user_auth.amount() > dec!("0"), "Invalid user proof");
            user_auth.resource_address()
        }
    }
}

#[derive(Debug, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub struct User {
    /// The user's deposit balance
    pub deposit_balance: Decimal,
    /// Last update timestamp
    pub deposit_last_update: u64,

    /// The user's borrow balance
    pub borrow_balance: Decimal,
    /// Last update timestamp
    pub borrow_last_update: u64,
}

impl User {
    // pub fn get_collateral_ratio(&self) -> Option<Decimal> {
    //     if self.borrow_balance.is_zero() {
    //         None
    //     } else {
    //         let collateral = self.deposit_balance
    //             + self.deposit_balance * self.deposit_interest_rate * self.deposit_time_elapsed();

    //         let loan = self.borrow_balance
    //             + self.borrow_balance * self.borrow_interest_rate * self.borrow_time_elapsed();

    //         Some(collateral / loan)
    //     }
    // }

    // pub fn check_collateral_ratio(&self, min_collateral_ratio: Decimal) {
    //     let collateral_ratio = self.get_collateral_ratio();
    //     if let Some(ratio) = collateral_ratio {
    //         assert!(
    //             ratio >= min_collateral_ratio,
    //             "Min collateral ratio does not meet"
    //         );
    //     }
    // }

    pub fn on_deposit(&mut self, amount: Decimal) {
        // Increase principle balance by interests accrued
        // let interest =
        //     self.deposit_balance * self.deposit_interest_rate * self.deposit_time_elapsed();
        // self.deposit_balance += interest;
        self.deposit_last_update = Runtime::current_epoch();

        // // Calculate the aggregated interest of previous deposits & the new deposit
        // self.deposit_interest_rate = (self.deposit_balance * self.deposit_interest_rate
        //     + amount * interest_rate)
        //     / (self.deposit_balance + amount);

        // Increase principle balance by the amount.
        self.deposit_balance += amount;
    }

    pub fn on_redeem(&mut self, amount: Decimal) -> Decimal {
        // Deduct withdrawn amount from principle
        self.deposit_balance -= amount;

        // Calculate the amount to return
        // amount + amount * self.deposit_interest_rate * self.deposit_time_elapsed()
        amount
    }

    pub fn on_borrow(&mut self, amount: Decimal) {
        // Increase borrow balance by interests accrued
        // let interest = self.borrow_balance * self.borrow_interest_rate * self.borrow_time_elapsed();
        // self.borrow_balance += interest;
        // self.borrow_last_update = Runtime::current_epoch();

        // // Calculate the aggregated interest of previous borrows & the new borrow
        // self.borrow_interest_rate = (self.borrow_balance * self.borrow_interest_rate
        //     + amount * interest_rate)
        //     / (self.borrow_balance + amount);

        // Increase principle balance by the amount.
        self.borrow_balance += amount;
    }

    pub fn on_repay(&mut self, amount: Decimal) -> Decimal {
        // Increase borrow balance by interests accrued
        // let interest = self.borrow_balance * self.borrow_interest_rate * self.borrow_time_elapsed();
        // self.borrow_balance += interest;
        self.borrow_last_update = Runtime::current_epoch();

        // Repay the loan
        if self.borrow_balance < amount {
            let to_return = amount - self.borrow_balance;
            self.borrow_balance = Decimal::zero();
            // self.borrow_interest_rate = Decimal::zero();
            to_return
        } else {
            self.borrow_balance -= amount;
            Decimal::zero()
        }
    }

    pub fn on_liquidate(&mut self, amount: Decimal) -> Decimal {
        let changes = self.on_repay(amount);
        assert!(changes == 0.into());

        // TODO add exchange rate here when collaterals and borrows are different

        // let to_return = amount * (bonus_percent + 1);
        let to_return = amount;
        self.deposit_balance -= to_return;
        to_return
    }

    fn deposit_time_elapsed(&self) -> u64 {
        // +1 is for demo purpose only
        Runtime::current_epoch() - self.deposit_last_update + 1
    }

    fn borrow_time_elapsed(&self) -> u64 {
        // +1 is for demo purpose only
        Runtime::current_epoch() - self.borrow_last_update + 1
    }
}
