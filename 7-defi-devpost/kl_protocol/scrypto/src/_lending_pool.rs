use scrypto::prelude::*;
use std::cmp::*;

#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Copy, Clone)]
pub struct InterestTypeState {
    pub interest_type: u8,
    pub total_loan: Decimal,
    pub total_loan_share: Decimal,
    pub interest_rate: Decimal,
    pub interest_updated_at: i64,
}

#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct LendingPoolState {
    pub liquidation_threshold: Decimal,
    pub liquidation_close_factor: Decimal,
    pub liquidation_spread: Decimal,

    //
    pub last_price: Decimal,
    pub last_price_update: i64,
}

#[derive(NonFungibleData)]
pub struct FlashloanTerm {
    pub amount_due: Decimal,
    pub loan_amount: Decimal,
    pub fees: Decimal,
}

#[derive(NonFungibleData)]
struct AuthBadgeData {}

external_component! {
    PriceOracleComponentTarget {
        fn get_price(&self,  quote: ResourceAddress) -> Option<Decimal>;
    }
}

external_component! {
    InterestFactoryTarget {
        fn get_loan_interest_rate(
            &self,
            interest_type: u8,
            pool_ressource_address:ResourceAddress,
            available_liquidity_amount:Decimal,
            total_loan_amount:Decimal
        ) -> Decimal;
    }
}

#[blueprint]
mod lending_pool {

    struct LendingPool {
        pool_component_badge: Vault,

        liquidity: Vault,

        collaterals: Vault,

        collected_fees: Vault,

        insurance_reserve: Vault,

        pool_resource_address: ResourceAddress,

        pool_share_resource_address: ResourceAddress,

        flashloan_term_resource_address: ResourceAddress,

        interest_factory_address: ComponentAddress,

        oracle_address: ComponentAddress,

        loan_state_lookup: HashMap<u8, InterestTypeState>,

        last_price: Decimal,

        last_price_update: i64,

        flashloan_fee_rate: Decimal,

        liquidation_threshold: Decimal,

        liquidation_spread: Decimal,

        liquidation_close_factor: Decimal,

        last_available_liquidity: Decimal,

        last_pool_share_supply: Decimal,

        last_collateral_amount: Decimal,
    }

    impl LendingPool {
        // Creates a PoolManager component and returns the component address
        pub fn new(
            // pool_admin_badge: ResourceAddress,
            pool_manager_component_badge: ResourceAddress,
            pool_admin_badge: ResourceAddress,
            pool_resource_address: ResourceAddress,
            pool_share_resource_symbol: String,
            pool_share_resource_name: String,
            pool_share_resource_icon: String,

            flashloan_fee_rate: Decimal,
            liquidation_threshold: Decimal,
            liquidation_spread: Decimal,
            liquidation_close_factor: Decimal,

            interest_factory_address: ComponentAddress,
            oracle_address: ComponentAddress,

            internal_tag: String,
        ) -> (ComponentAddress, ResourceAddress) {
            // Badge needed for all internal operation that require this component authority
            let pool_component_badge = Vault::with_bucket(
                ResourceBuilder::new_uuid_non_fungible().mint_initial_supply([(AuthBadgeData {})]),
            );

            //  Define Pool share resource
            let pool_share_resource_address = ResourceBuilder::new_fungible()
                .metadata(
                    "internal_tag",
                    format!("{}_pool_share_resource", internal_tag),
                )
                .metadata("symbol", pool_share_resource_symbol)
                .metadata("name", pool_share_resource_name)
                .metadata("icon", pool_share_resource_icon)
                .updateable_metadata(rule!(require(pool_admin_badge)), LOCKED)
                .mintable(
                    rule!(require(pool_component_badge.resource_address())),
                    LOCKED,
                )
                .burnable(
                    rule!(require(pool_component_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply();

            // Define a "transient" resource which can never be deposited once created, only burned
            let flashloan_term_resource_address = ResourceBuilder::new_uuid_non_fungible()
                .metadata("internal_tag", format!("{}_flashloan_term", internal_tag))
                .metadata("name", "KLP Flashoan terme")
                .metadata(
                    "description",
                    "Promise NFT for KL Protocol Flashloans - must be returned to be burned!",
                )
                .mintable(
                    rule!(require(pool_component_badge.resource_address())),
                    LOCKED,
                )
                .burnable(
                    rule!(require(pool_component_badge.resource_address())),
                    LOCKED,
                )
                .restrict_deposit(AccessRule::DenyAll, LOCKED)
                .create_with_no_initial_supply();

            // Instatiate the pool withh access rules

            let mut liquidity_pool = Self {
                pool_component_badge,

                liquidity: Vault::new(pool_resource_address),
                collaterals: Vault::new(pool_share_resource_address),
                collected_fees: Vault::new(pool_resource_address),
                insurance_reserve: Vault::new(pool_resource_address),

                pool_resource_address,
                pool_share_resource_address,
                flashloan_term_resource_address,

                loan_state_lookup: HashMap::from([
                    (
                        0u8,
                        InterestTypeState {
                            interest_type: 0u8,
                            total_loan_share: Decimal::zero(),
                            interest_rate: Decimal::zero(),
                            interest_updated_at: 0,
                            total_loan: Decimal::zero(),
                        },
                    ),
                    (
                        1u8,
                        InterestTypeState {
                            interest_type: 1u8,
                            total_loan_share: Decimal::zero(),
                            interest_rate: Decimal::zero(),
                            interest_updated_at: 0,
                            total_loan: Decimal::zero(),
                        },
                    ),
                    (
                        2u8,
                        InterestTypeState {
                            interest_type: 2u8,
                            total_loan_share: Decimal::zero(),
                            interest_rate: Decimal::zero(),
                            interest_updated_at: 0,
                            total_loan: Decimal::zero(),
                        },
                    ),
                ]),

                flashloan_fee_rate,
                liquidation_threshold,
                liquidation_spread,
                liquidation_close_factor,

                //
                interest_factory_address,
                oracle_address,
                last_price: Decimal::zero(),
                last_price_update: 0,
                last_available_liquidity: Decimal::zero(),
                last_collateral_amount: Decimal::zero(),
                last_pool_share_supply: Decimal::zero(),
            }
            .instantiate();

            liquidity_pool.metadata("internal_tag", format!("{}_lending_pool", internal_tag));

            let admin_rule = rule!(require(pool_manager_component_badge));

            let access_rules = AccessRules::new()
                // Allow All methods publicly accessible for lending and flashloan
                .method("add_liquidity", AccessRule::AllowAll, LOCKED)
                .method("remove_liquidity", AccessRule::AllowAll, LOCKED)
                .method("take_flash_loan", AccessRule::AllowAll, LOCKED)
                .method("repay_flash_loan", AccessRule::AllowAll, LOCKED)
                //Allow All methods publicly accessible for pool state infos
                .method("get_pool_share_ratio", AccessRule::AllowAll, LOCKED)
                .method("get_loan_share_ratio", AccessRule::AllowAll, LOCKED)
                .method("get_pool_state", AccessRule::AllowAll, LOCKED)
                .method("update_price", AccessRule::AllowAll, LOCKED)
                .method("update_all_interest", AccessRule::AllowAll, LOCKED)
                // All methods are lock by default
                .default(admin_rule.clone(), LOCKED);

            liquidity_pool.add_access_check(access_rules);

            let component_address = liquidity_pool.globalize();

            // Return the new LiquidityPool component_address, as well as the Admin badge and LP resource address
            (component_address.clone(), pool_share_resource_address)
        }

        /*

        USERS METHODS

        Following methods ar publicly accessible. they implement the lending features and also flash loan features

        */

        pub fn add_liquidity(&mut self, assets: Bucket) -> Bucket {
            self.update_all_interest();

            let tokens_amount = assets.amount();

            let supply_to_mint = tokens_amount * self.get_pool_share_ratio();

            self.liquidity.put(assets);

            let pool_shares = self.pool_component_badge.authorize(|| {
                let lp_resource_manager =
                    borrow_resource_manager!(self.pool_share_resource_address);

                let pool_shares = lp_resource_manager.mint(supply_to_mint);

                pool_shares
            });

            //
            self._update_state();

            pool_shares
        }

        pub fn remove_liquidity(&mut self, pool_shares: Bucket) -> Bucket {
            self.update_all_interest();

            // We need to add total amount of loan for an accurate calulation of liquidity provided and earned by lenders.
            // Total Loan represent what hadbeen borrow + accrued interest
            let pool_amount = self.liquidity.amount() + self._total_loan_amount();

            let to_remove = min(
                pool_shares.amount() / self.get_pool_share_ratio(),
                pool_amount,
            );
            let removed_assets = self.liquidity.take(to_remove);

            self.pool_component_badge.authorize(|| {
                pool_shares.burn();
            });

            //
            self._update_state();

            removed_assets
        }

        pub fn take_flash_loan(&mut self, loan_amount: Decimal) -> (Bucket, Bucket) {
            assert!(
                loan_amount <= self.liquidity.amount(),
                "Not enough liquidity to supply this loan!"
            );

            let fees = loan_amount * (self.flashloan_fee_rate);

            // Mint the loan term. it can be deposited in any caccount so, it will need to be return with the repayment and burn for the transaction to be able to suuceed
            let loan_terms = self.pool_component_badge.authorize(|| {
                borrow_resource_manager!(self.flashloan_term_resource_address).mint_non_fungible(
                    &NonFungibleLocalId::random(),
                    FlashloanTerm {
                        amount_due: fees + loan_amount,
                        fees,
                        loan_amount,
                    },
                )
            });
            (self.liquidity.take(loan_amount), loan_terms)
        }

        pub fn repay_flash_loan(
            &mut self,
            mut loan_repayment: Bucket,
            loan_terms: Bucket,
        ) -> Bucket {
            // Verify we are being sent at least the amount due
            let terms: FlashloanTerm = loan_terms.non_fungible().data();
            assert!(
                loan_repayment.amount() >= terms.amount_due,
                "Insufficient repayment given for your loan!"
            );

            self.liquidity.put(loan_repayment.take(terms.amount_due));

            // We have our payment; we can now burn the transient token
            self.liquidity.authorize(|| loan_terms.burn());

            // Return the change to the work top
            loan_repayment
        }

        /*

        INFORMATION INQUERY

        Following methods ars also publicly available. the expose to over component pool_share_ratio, the loan_share_ratio and other usefull state information

        */

        // Get the most recent pool_share_ratio
        pub fn get_pool_share_ratio(&self) -> Decimal {
            let pool_amount = self.liquidity.amount() + self._total_loan_amount();

            if pool_amount != Decimal::zero() {
                borrow_resource_manager!(self.pool_share_resource_address).total_supply()
                    / pool_amount
            } else {
                Decimal::ONE
            }
        }

        // Get the most recent loan_share_ratio  for a specific
        pub fn get_loan_share_ratio(&self, interest_type: u8) -> Decimal {
            let interest_type_state = self._get_interest_type_state(interest_type);

            if interest_type_state.total_loan != Decimal::ZERO {
                interest_type_state.total_loan_share / interest_type_state.total_loan
            } else {
                Decimal::ONE
            }
        }

        // Get the pool state including
        pub fn get_pool_state(&mut self) -> LendingPoolState {
            self.update_price();

            let state = LendingPoolState {
                liquidation_threshold: self.liquidation_threshold,
                liquidation_close_factor: self.liquidation_close_factor,
                liquidation_spread: self.liquidation_spread,

                last_price: self.last_price,
                last_price_update: self.last_price_update,
            };

            state
        }

        /*  ADMIN METHODS

        The following methods can be call only with an admin badge. Thus badge is controlled by an highe level component holding this badge.
        they help handling logic of the pool at a global level and do not get involve in per-user operation.

        */

        // Handle request to increase polled  collateral.
        // The methode expect pool_share but if supply tokens are pool main resources, it call add_liquidity to get pool_share then provision pool_shares as collateral
        pub fn add_collateral(&mut self, assets: Bucket) -> Decimal {
            let pool_shares: Bucket;
            if assets.resource_address() == self.pool_share_resource_address {
                pool_shares = assets;
            } else {
                pool_shares = self.add_liquidity(assets);
            }

            let pool_share_amount = pool_shares.amount();

            self.collaterals.put(pool_shares);

            //
            self._update_state();

            pool_share_amount
        }

        // Handle request to decrease polled collateral.
        // if get_pool_share is set to true, it return pool_share otherwise it will also call remove_liquidity
        pub fn remove_collateral(
            &mut self,
            pool_share_amount: Decimal,
            get_pool_share: bool,
        ) -> Bucket {
            // let mut pool_shares_amount = if get_pool_share {
            //     amount
            // } else {
            //     amount * self.get_pool_share_ratio()
            // };

            let max_pool_share_amount = min(self.collaterals.amount(), pool_share_amount);

            let mut removed_assets = self.collaterals.take(max_pool_share_amount);

            if !get_pool_share {
                removed_assets = self.remove_liquidity(removed_assets);
            }

            //
            self._update_state();

            removed_assets
        }

        // Handle request to increse borowed amount.
        // it remove request liquidity and updated the pool loan state per interest type base
        pub fn borrow(&mut self, amount: Decimal, interest_type: u8) -> (Bucket, Decimal) {
            // as collateral are pool shares, we should  be able to be withdraw at any time
            // so we deduct they value from the liquidity pool
            let borrow_limit = min(
                amount,
                self.liquidity.amount() - self.collaterals.amount() / self.get_pool_share_ratio(),
            );

            let (actual_amount, loan_share) = self._update_loan_share(interest_type, borrow_limit);

            let loan = self.liquidity.take(actual_amount);

            //
            self._update_state();

            (loan, loan_share)
        }

        // Handle request to decrese borowed amount.
        // it add back liquidity comming from repayments and updated the pool loan state per interest type base
        pub fn repay(&mut self, mut payment: Bucket, interest_type: u8) -> (Bucket, Decimal) {
            let (actual_amount, loan_share) =
                self._update_loan_share(interest_type, -payment.amount());

            // returned actual_amount and loan_shares should be negative or 0
            self.liquidity.put(payment.take(-actual_amount));

            //
            self._update_state();

            // Send back positive loan_share to evoid confusion at higher level in the stack
            (payment, loan_share.abs())
        }

        // Each interest_type is track by it's own set of total_loan and loan_shares.
        // so changing the interest rate is achive by repaying one loan and borrowing the equivelent amount with the new interest rate.
        // i practice we only update loan and loan shares of both interest rate
        pub fn move_loan_share(
            &mut self,
            from_interest_type: u8,
            interest_type_dest: u8,
            amount: Decimal,
        ) {
            if from_interest_type == interest_type_dest {
                return;
            }
            self._update_loan_share(from_interest_type, -amount);
            self._update_loan_share(interest_type_dest, amount);
        }

        /*

        ADMIN FUNCTIONS


        */

        // Deposit or withdraw collected fee
        pub fn add_collected_fees(&mut self, tokens: Bucket) {
            self.collected_fees.put(tokens);
        }
        pub fn remove_collected_fees(&mut self, amount: Decimal) -> Bucket {
            self.collected_fees.take(amount)
        }

        // Deposit or withdraw collected insurance reserve. For now the reserve comme from liquidation bonus.
        //
        pub fn insurance_deposit(&mut self, tokens: Bucket) {
            self.insurance_reserve.put(tokens);
        }
        pub fn insurance_withdraw(&mut self, amount: Decimal) -> Bucket {
            self.insurance_reserve.take(amount)
        }

        pub fn update_price(&mut self) -> Decimal {
            let before = self.last_price_update;
            let now: i64 = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;

            let period_in_minute = (now - before) / 60;

            // debonce interest update up to a minute
            if period_in_minute == 0 && before != 0 {
                return self.last_price;
            }

            let price = match PriceOracleComponentTarget::at(self.oracle_address)
                .get_price(self.pool_resource_address)
            {
                Some(p) => p,
                None => panic!("Missing price"),
            };

            self.last_price = price;
            self.last_price_update = now;

            price
        }

        pub fn update_interest(&mut self, interest_type: u8) {
            let mut interest_type_state = self._get_interest_type_state(interest_type);

            let before = interest_type_state.interest_updated_at;
            let now: i64 = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let period_in_minute = (now - before) / 60;

            if period_in_minute == 0 {
                return;
            }

            let interest_rate = InterestFactoryTarget::at(self.interest_factory_address)
                .get_loan_interest_rate(
                    interest_type,
                    self.pool_resource_address,
                    self.liquidity.amount(),
                    self._total_loan_amount(),
                );

            interest_type_state.interest_rate = interest_rate;
            interest_type_state.interest_updated_at = now;

            // this is to speedup the time for test perpose: 1 year -> 1 week
            interest_type_state.total_loan = interest_type_state.total_loan
                * (dec!(1) + (interest_rate / 525600)).powi(period_in_minute);

            self.loan_state_lookup
                .insert(interest_type, interest_type_state);
        }

        pub fn update_all_interest(&mut self) {
            for (interest_type, _) in &self.loan_state_lookup.clone() {
                self.update_interest(*interest_type);
            }
        }

        /* UTILITY METHODS */

        fn _update_loan_share(&mut self, interest_type: u8, amount: Decimal) -> (Decimal, Decimal) {
            let mut interest_type_state = self._get_interest_type_state(interest_type);

            // Making sur that supplied assets are not more that what had been borrowed

            let actual_amount = if (interest_type_state.total_loan + amount) < dec!(0) {
                -interest_type_state.total_loan
            } else {
                amount
            };

            let loan_share = actual_amount * self.get_loan_share_ratio(interest_type);

            interest_type_state.total_loan = interest_type_state.total_loan + actual_amount;

            interest_type_state.total_loan_share =
                interest_type_state.total_loan_share + loan_share;

            self.loan_state_lookup
                .insert(interest_type, interest_type_state);

            (actual_amount, loan_share)
        }

        fn _get_interest_type_state(&self, interest_type: u8) -> InterestTypeState {
            let interest_type_state = match self.loan_state_lookup.clone().remove(&interest_type) {
                Some(s) => s,
                None => panic!("Interest type not found"),
            };

            interest_type_state
        }

        fn _total_loan_amount(&self) -> Decimal {
            let mut total_loan: Decimal = dec!(0);

            for (_, interest_type_state) in &self.loan_state_lookup {
                total_loan = total_loan + interest_type_state.total_loan;
            }

            total_loan
        }

        fn _update_state(&mut self) {
            self.last_available_liquidity = self.liquidity.amount();
            self.last_collateral_amount = self.collaterals.amount();
            self.last_pool_share_supply =
                borrow_resource_manager!(self.pool_share_resource_address).total_supply();
        }
    }
}
