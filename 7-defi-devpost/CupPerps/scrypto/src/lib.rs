use scrypto::prelude::*;

#[blueprint]
mod prop_perps {

    /// Cup Perps are described in detail in a pdf inside of this repo
    /// this implementation has a freely-manipulated oracle
    /// only to be used for purposes of demonstration 
    /// 
    /// Recommended to read the pdf first for some context
    struct CupPerp {
        /// eg. BTC/USD
        /// is stored in token metadata, better UX
        pair: String,

        /// parameters that can be freely changed 
        /// deployed version has: 5, 0.75
        leverage: Decimal,
        funding_coeff: Decimal,

        /// unfortunately apart from long/short_cup 
        /// we also have to store _value
        /// as the offchain query doesn't provide that information
        /// i.e. doesn't give per-vault values but rather per-resource
        long_cup: Vault,
        long_cup_value: Decimal, 

        short_cup: Vault,
        short_cup_value: Decimal,

        /// amount of total long/short LP tokens minted
        long_cup_lp: Decimal,
        short_cup_lp: Decimal,

        /// unused due to oracle being freely-set
        last_update: u64,
        /// last tick's exchange rate
        /// used to calculate Delta
        last_exrate: Decimal,

        /// unused due to oracle being freely-set 
        oracle_update: u64,
        // toy version has the oracle be a variable
        // able to be updated with set_oracle(n)
        oracle_exrate: Decimal,

        /// resource management variables 
        long_lp_badge: Vault,
        long_lp_resource: ResourceAddress,

        short_lp_badge: Vault,
        short_lp_resource: ResourceAddress,

        /// what asset is used to bet on the pair
        /// useful for off-chain verification 
        /// as any asset could be used for these perps
        stable_coin: ResourceAddress
    }

    impl CupPerp {

        /// Instantiates the Component
        /// 
        /// The initial deposit's lp is not issued, to prevent division by zero,
        /// but the initial deposit may be worth close to 0
        /// followed by the actual liqidity providers
        /// 
        /// # Arguments 
        /// 
        /// * `pair` - String shown in LP token metadata
        /// * `leverage` - The leverage of both cups' exposure when at equilibrium
        /// * `funding_coeff` - Scales the funding rebate (funding * cup size ratio)
        /// * `exch` - Initial pair exchange rate
        /// * `deposit` - Liquidity to be locked in the pair forever
        /// 
        /// ```
        /// // resim 0.8.0
        /// > resim call-function [package] CupPerp instantiate_pair "5x BTC/USD" 5 0.75 28345 10,[xrd]
        /// 
        /// // wallet-sdk 0.6.0-beta.11
        /// .withdrawFromAccountByAmount(
        ///     [account], 
        ///     100, 
        ///     [xrd address])
        /// .takeFromWorktop(
        ///     [xrd address],
        ///     "buck")
        /// .callFunction(
        ///     [package], 
        ///     "CupPerp", 
        ///     "instantiate_pair", 
        ///     [String("RAND/USD"), Decimal(1000), Bucket("buck")])
        /// .callMethod(
        ///      [account], 
        ///      "deposit_batch", 
        ///      [Expression("ENTIRE_WORKTOP")])
        /// .build()
        /// .toString();
        /// ```
        pub fn instantiate_pair(
            pair: String, 
            leverage: Decimal,
            funding_coeff: Decimal,
            exch: Decimal, 
            mut deposit: Bucket) 
            -> ComponentAddress {

            debug!("INSTANTIATING PAIR");
            let long_lp_badge: Vault = Vault::with_bucket(ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1));

            let short_lp_badge: Vault = Vault::with_bucket(ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1));

            let mut name = "CupPerp Long ".to_owned();
            name.push_str(&*pair);
            // reminder to self: don't ever
            // set custom divisibility (other than none)
            let long_lp_resource = ResourceBuilder::new_fungible()
                .mintable(rule!(require(long_lp_badge.resource_address())), LOCKED)
                .burnable(rule!(require(long_lp_badge.resource_address())), LOCKED)
                .metadata("name", name)
                // doesn't work currently, wallet doesn't implement icons yet
                .metadata("icon", "https://www.iconsdb.com/icons/preview/green/up-xxl.png")
                .create_with_no_initial_supply();

            name = "CupPerp Short ".to_owned();
            name.push_str(&*pair);
            let short_lp_resource = ResourceBuilder::new_fungible()
                .mintable(rule!(require(short_lp_badge.resource_address())), LOCKED)
                .burnable(rule!(require(short_lp_badge.resource_address())), LOCKED)
                .metadata("name", name)
                .metadata("icon", "https://www.iconsdb.com/icons/preview/red/down-xxl.png")
                .create_with_no_initial_supply();

            let stable_coin = deposit.resource_address();

            let amm = deposit.amount().clone()/2;
            let long_cup = Vault::with_bucket(deposit.take(deposit.amount()/2));
            let short_cup = Vault::with_bucket(deposit);
            let long_cup_lp = dec!(1000);
            let short_cup_lp = dec!(1000);

            Self {
                pair: pair,

                leverage: leverage,
                funding_coeff: funding_coeff,

                long_cup: long_cup,
                long_cup_value: amm,
                short_cup: short_cup,
                short_cup_value: amm,
                long_cup_lp: long_cup_lp,
                short_cup_lp: short_cup_lp,
                last_update: 0,
                last_exrate: exch,

                oracle_update: 0,
                oracle_exrate: exch,

                long_lp_badge: long_lp_badge,
                long_lp_resource: long_lp_resource,
                short_lp_badge: short_lp_badge,
                short_lp_resource: short_lp_resource,

                stable_coin: stable_coin
            }
            .instantiate()
            .globalize()
        }

        /// Updates the system state after any price change
        /// 
        /// ```
        /// // scrypto 0.8.0
        /// > resim call-method [component] update
        /// 
        /// // wallet-sdk 0.6.0-beta.11
        /// .callMethod(
        ///     [component],
        ///     "update",
        ///     [])
        /// .callMethod(
        ///      [account], 
        ///      "deposit_batch", 
        ///      [Expression("ENTIRE_WORKTOP")])
        /// .build()
        /// .toString();
        /// ```
        pub fn update(&mut self) {
            // assert!(oracle-update >= last_update)
            assert!(self.oracle_exrate > dec!(0));

            if self.last_exrate == self.oracle_exrate {
                return
            }            

            let ONE: Decimal = dec!(1);
            let ZERO: Decimal = dec!(0);

            let delta = (self.oracle_exrate / self.last_exrate - ONE) * self.leverage;
            let long_d = delta * self.long_cup.amount();
            let short_d = delta * self.short_cup.amount();
            let ratio = self.long_cup.amount() / self.short_cup.amount();

            let funding = self.funding_coeff * if ratio > ONE { 
                    ONE/ratio
                } else { 
                    ratio 
                };

            // funding rebate always goes to the smaller cup
            let adj_delta = if long_d.abs() > short_d.abs() { 
                    (if delta > ZERO { funding } else { ONE / funding }) * short_d.abs()
                } else { 
                    (if delta > ZERO { ONE / funding } else { funding }) * long_d.abs() 
                };

            if delta > ZERO {
                self.long_cup.put(self.short_cup.take(adj_delta));
            } else {
                self.short_cup.put(self.long_cup.take(adj_delta));
            }

            self.long_cup_value = self.long_cup.amount();
            self.short_cup_value = self.short_cup.amount();

            self.last_exrate = self.oracle_exrate;
            self.last_update = self.oracle_update;
        }
        
        /// Deposits the stable asset to the select cup
        /// 
        /// # Arguments 
        /// 
        /// * `side` - Which cup to deposit into? Long is true, Short is false. 
        /// * `funds` - Funds to be deposited
        /// 
        /// ```
        /// // resim 0.8.0
        /// > resim call-method [component] deposit true 10,[xrd]
        /// 
        /// // wallet-sdk 0.6.0-beta.11
        /// .withdrawFromAccountByAmount(
        ///     [account],
        ///     10,
        ///     [xrd address])
        /// .takeFromWorktop(
        ///     [xrd address],
        ///     "buck1")
        /// .callMethod(
        ///     [component],
        ///     "deposit",
        ///     [Bool(false), Bucket("buck1")])
        /// .callMethod(
        ///      [account], 
        ///      "deposit_batch", 
        ///      [Expression("ENTIRE_WORKTOP")])
        /// .build()
        /// .toString();
        /// ```
        pub fn deposit(&mut self, side: bool, funds: Bucket) -> Bucket {
            self.update();

            let lp_caller; let lp; let cup;
            let lp_badge; let lp_resource;
            if side {
                lp = &mut self.long_cup_lp;
                cup = &mut self.long_cup;
                lp_badge = &self.long_lp_badge;
                lp_resource = self.long_lp_resource;
            } else {
                lp = &mut self.short_cup_lp;
                cup = &mut self.short_cup;
                lp_badge = &self.short_lp_badge;
                lp_resource = self.short_lp_resource;
            }

            lp_caller = 
                *lp * ((cup.amount() + funds.amount()) / cup.amount() - 1);
            *lp += lp_caller;
            (*cup).put(funds);

            self.long_cup_value = self.long_cup.amount();
            self.short_cup_value = self.short_cup.amount();

            return lp_badge.authorize(|| 
                borrow_resource_manager!(lp_resource).mint(lp_caller));  
        }

        /// Withdraws the liquidity with an LP token
        /// 
        /// ```
        /// // scrypto 0.8.0
        /// > resim call-method [component] withdraw 1337,[ShortLp]
        /// 
        /// // wallet-sdk 0.6.0-beta.11
        /// .withdrawFromAccountByAmount(
        ///     [account],
        ///     420,
        ///     [LongLp address])
        /// .takeFromWorktop(
        ///     [LongLp address],
        ///     "buck1")
        /// .callMethod(
        ///     [component],
        ///     "withdraw",
        ///     [Bucket("buck1")])
        /// .callMethod(
        ///      [account], 
        ///      "deposit_batch", 
        ///      [Expression("ENTIRE_WORKTOP")])
        /// .build()
        /// .toString();
        /// ```
        pub fn withdraw(&mut self, funds: Bucket) -> Bucket {
            self.update();

            let payout; let lp; let cup; let lp_badge; 
            let lp_resource = funds.resource_address().clone();

            if lp_resource == self.long_lp_resource {
                lp = &mut self.long_cup_lp;
                cup = &mut self.long_cup;
                lp_badge = &self.long_lp_badge;
            } else {
                lp = &mut self.short_cup_lp;
                cup = &mut self.short_cup;
                lp_badge = &self.short_lp_badge;
            }

            assert!(funds.resource_address() == lp_resource);
            payout = funds.amount() / *lp * cup.amount();
            *lp -= funds.amount();
            lp_badge.authorize(|| funds.burn());

            if lp_resource == self.long_lp_resource {
                self.long_cup_value = cup.amount() - payout;
            } else {
                self.short_cup_value = cup.amount() - payout;
            }

            return (*cup).take(payout);
        }

        /// Sets the built-in oracle to a given number
        /// 
        /// # Arguments 
        /// 
        /// * `n` - Decimal exchange rate
        /// 
        /// ```
        /// // scrypto 0.8.0
        /// > resim call-method [component] set_oracle 4
        /// 
        /// // wallet-sdk 0.6.0-beta.11
        /// .callMethod(
        ///     [component],
        ///     "set_oracle",
        ///     [Decimal(5138008)])
        /// .callMethod(
        ///      [account], 
        ///      "deposit_batch", 
        ///      [Expression("ENTIRE_WORKTOP")])
        /// .build()
        /// .toString();
        /// ```
        pub fn set_oracle(&mut self, n: Decimal) {
            self.oracle_exrate = n;
        }

        /// Shows the current amount of tokens in both cups
        /// 
        /// ```
        /// // scrypto 0.8.0
        /// > resim call-method [component] get_lp_resource_addr 
        /// ```
        pub fn show_cups(&self) -> (Decimal, Decimal) {
            (self.long_cup.amount(), self.long_cup.amount())
        }

        /// Returns the ResourceAddress of both Lp tokens
        /// 
        /// ```
        /// // scrypto 0.8.0
        /// > resim call-method [component] get_lp_resource_addr 
        /// ```
        pub fn get_lp_resource_addr(&self) 
            -> (ResourceAddress, ResourceAddress) {
            return (
                self.long_lp_resource, 
                self.short_lp_resource)
        }

        /// Returns the total value of a given quantity of Lp tokens
        /// 
        /// # Arguments 
        /// 
        /// * `long` - Amount of LongLp tokens
        /// * `short` - Amount of ShortLp tokens 
        /// 
        /// ```
        /// // scrypto 0.8.0
        /// > resim call-method [component] value 7 0 
        /// ```
        pub fn value(&self, long: Decimal, short: Decimal) 
            -> Decimal {
            
            return 
                long / self.long_cup_lp 
                    * self.long_cup.amount()
              + short / self.short_cup_lp 
                    * self.short_cup.amount()
        }
    }
}