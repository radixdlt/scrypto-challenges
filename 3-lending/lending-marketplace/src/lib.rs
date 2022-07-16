use scrypto::prelude::*;

blueprint! {
    struct LendingMarketPlace {
        /// The liquidity pool
        liquidity_pool: Vault,
        /// The min collateral ratio that a user has to maintain
        min_collateral_ratio: Decimal,
        /// The max percent of liquidity pool one can borrow
        max_borrow_percent: Decimal,
        /// The max percent of debt one can liquidate
        max_liquidation_percent: Decimal,
        /// Liquidation bonus
        liquidation_bonus: Decimal,
        /// User state
        users: LazyMap<ResourceAddress, User>,
        /// The interest rate of deposits, per epoch
        deposit_interest_rate: Decimal,
        /// The (stable) interest rate of loans, per epoch
        borrow_interest_rate: Decimal,

    }

    impl LendingMarketPlace {
        /// Creates a lending pool, with single collateral.
        pub fn new(reserve_address : &ResourceAddress) -> ComponentAddress {
            Self {
                liquidity_pool: Vault::new(reserve_address),
                min_collateral_ratio: dec!("1.2"),
                max_borrow_percent: dec!("0.3"),
                max_liquidation_percent: dec!("0.5"),
                liquidation_bonus: dec!("0.05"),
                users: LazyMap::new(),
                deposit_interest_rate: dec!("0.01"),
                borrow_interest_rate: dec!("0.02"),
            }
            .inistantiate()
            .globalize()
        }
    }

    // Reigster new user 
    pub fn new_user(&self) -> Bucket {
        ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .metadata("name", "AutoLend User Badge")
            .initial_supply(1)
    }

     /// Deposits into the liquidity pool and start earning interest.
     pub fn deposit(&mut self, user_auth: Proof, reserve_tokens: Bucket) {
        let user_id = Self::get_user_id(&user_auth);
        let amount = reserve_tokens.balance();

        // update user state 
        let deposit_interest_rate = self.deposit_interest_rate;
        let user =  match self.user.get(&user_id) {
            some( mut user) => {
                user.on_deposit(amount, deposit_interest_rate);
                user
            }
            None => User{
                deposit_balance: amount,
                borrow_balance: Decimal::zero(),
                deposit_interest_rate,
                borrow_interest_rate: Decimal::zero(),
                deposit_last_update: Runtime::current_epoch(),
                borrow_last_update: Runtime::current_epoch(),
            }
        };

        // commit state change 
        self.user.insert(user_id, user);
        self.liquidity_pool.put(reserve_tokens);
     }
     
     /// Redeems the underlying assets, partially or in full 
     pub fn redeem(&mut self, user_auth: Proof, requested: Deciaml) -> Bucket {
        let user_id = self::get_user_id(user_auth);

            assets!(
                requested <= self.liquidity_pool.amount() * self.max_borrow_percent , 
                "max borrow percent exceeded"
            );

            // update
            let borrow_interest_rate = self.borrow_interest_rate;
            let mut user = self.get_user(user_id);
            user.on_borrow(requested, borrow_interest_rate);
            user.check_collateral_ratio(self.min_collateral_ratio);

            // commit staTE change
            self.users.insert(user_id, user);
            self.liquidity_pool.withdraw(requested);
     }

     // repays a loan , partially or in full 
        pub fn repay(&mut self, user_auth: Proof, mut repaid: Bucket) -> Bucket {
            let user_id = self::get_user_id(user_auth);
    
            
                // update
                let mut user = self.get_user(user_id);
                let to_return_amount = user.on_repay(repaid.amount());
                let to_return = repaid.take(to_return_amount);
    
                // commit staTE change
                // Commit state changes
                self.users.insert(user_id, user);
                self.liquidity_pool.put(repaid);
                to_return
        }
    // Liquadate on user postion if its under collateral ratio
        pub fn liquidate(&mut self, user_id: ResourceAddress, repaid Bucket) -> Bucket {
            let mut user = self.get_user(user_id)

            //check if user is under collateral ratio
            let collateral_ratio = user.get_collateral_ratio();
            if let Some(ratio) = collateral_ratio{
                assets!(
                    ratio <= self.min_collateral_ratio,
                    "Liquidation not allowed."
                );
            } else {
                panic!("No borrow from user")
            }

            //check liqudation size 
            assets!(
                repaid.amount() <= user.borrow_balance * self.max_liquidation_percent,
                "Max liquidation percent exceeded."
            );

            let to_return_amount = user.on_liquidate(repaid.amount(), self.max_liquidation_percent);
            let to_return = self.liquidity_pool.take(to_return_amount);

            // commit state changes
            self.users.insert(user_id, user);
            to_return

            }

            /// return current state of the user 
            pub fn get_user(&self, user_id: ResourceAddress) -> User {
                match self.users.get(&user_id) {
                    Some(user) => user,
                    _ => panic!("User not found"),
                }
            }

            // return the deposit interest rate per epoch 
            pub fn set_deposit_interest_rate(&mut self, rate: Decimal) {
                self.deposit_interest_rate = rate;
            }

            // return the borrow interest rate per epoch
            pub fn set_borrow_interest_rate(&mut self, rate: Decimal) {
                self.borrow_interest_rate = rate;
            }

            //parse user id from a proof.
            fn get_user_id(user_auth: Proof) -> ResourceAddress {
                assert!(user_auth.amount() > dec!("0"), "Invalid user proof");
                user_auth.resource_address()
            }


        }

}
