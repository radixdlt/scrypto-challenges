use scrypto::prelude::*;
use scrypto_math::*;
use crate::liquidity_curve::*;

/// 365 days in seconds
const PERIOD_SIZE: Decimal = dec!(31536000);

/// Retrieves before-trade calculations for the 
/// exchange rate.
#[derive(ScryptoSbor, Clone)]
pub struct MarketCompute {
    rate_scalar: Decimal,
    rate_anchor: PreciseDecimal,
}

/// The `NonFungibleData` of the YieldToken NFT from
/// `YieldTokenizer` blueprint. We require the `NonFungibleData`
/// to perform YT ---> LSU swaps.
#[derive(ScryptoSbor, NonFungibleData)]
pub struct YieldTokenData {
    underlying_lsu_resource: ResourceAddress,
    underlying_lsu_amount: Decimal,
    redemption_value_at_start: Decimal,
    yield_claimed: Decimal,
    maturity_date: UtcDateTime,
}

/// The transient flash loan NFT which has `NonFungibleData` to track the resource 
/// and amount of the flash loan. The data here must be enforced to ensure that
/// the flash loan NFT can be burnt and therefore guarantee repayment.
#[derive(ScryptoSbor, NonFungibleData)]
pub struct FlashLoanReceipt {
    pub resource: ResourceAddress,
    pub amount: Decimal,
}

#[blueprint]
mod yield_amm {

    // The associated YieldTokenizer package and component which is used to verify associated PT, YT, and 
    // LSU asset. It is also used to perform YT <---> LSU swaps.
    extern_blueprint! {
        "package_tdx_2_1p4vfemgll9y7ykuhrsfymdyuxcd5wr4stpncle8t2we8aptff440u8",
        YieldTokenizer {
            fn tokenize_yield(
                &mut self, 
                amount: FungibleBucket
            ) -> (FungibleBucket, NonFungibleBucket);
            fn redeem(
                &mut self, 
                principal_token: FungibleBucket, 
                yield_token: NonFungibleBucket,
                yt_redeem_amount: Decimal
            ) -> (FungibleBucket, Option<NonFungibleBucket>);
            fn pt_address(&self) -> ResourceAddress;
            fn yt_address(&self) -> ResourceAddress;
            fn underlying_resource(&self) -> ResourceAddress;
            fn maturity_date(&self) -> UtcDateTime;
        }
    }
/* 
    const TOKENIZER: Global<YieldTokenizer> = global_component! (
        YieldTokenizer,
        "component_tdx_2_1crsv9p2jz5649s3e5uhvexenkevx84703c7ysdrmqct2yzgvvjnptj"
    );
    */

    struct YieldAMM {
        /// The native pool component which manages liquidity reserves. 
        pool_component: Global<TwoResourcePool>,
        /// The ResourceManager of the flash loan FlashLoanReceipt, which is used
        /// to ensure flash loans are repaid.
        flash_loan_rm: ResourceManager,
        /// The expiration date of the market. Once the market has expired,
        /// no more trades can be made.
        maturity_date: UtcDateTime,
        /// The initial scalar root of the market. This is used to calculate
        /// the scalar value. It determins the slope of the curve and becomes
        /// less sensitive as the market approaches maturity. The higher the 
        /// scalar value the more flat the curve is, the lower the scalar value
        /// the more steep the curve is.
        scalar_root: Decimal,
        /// The fee rate of the market. This is the fee rate charged on trades.
        fee_rate: PreciseDecimal,
        /// The reserve fee rate.
        reserve_fee_percent: Decimal,
        /// The natural log of the implied rate of the last trade.
        last_ln_implied_rate: PreciseDecimal,
        /// The LSU Address of the underlying
        lsu_address: ResourceAddress,
        /// The component Address fo the yield tokenizer
        tokenizer_component_address: ComponentAddress
    }

    impl YieldAMM {
        /// Instantiates a Yield AMM DEX. The basic implementation of the DEX only allows one
        /// asset pair to be traded, 
        pub fn instantiate_yield_amm(
            /* Rules */
            owner_role: OwnerRole,
            /* Initial market values */
            // The initial scalar root of the market which determines the initial
            // steepness of the curve (high slippage at the ends of the curve).
            scalar_root: Decimal,
            // The trading fee charged on each trade.
            fee_rate: Decimal,
            // The asset reserve fee. 
            reserve_fee_percent: Decimal,
            // Component Address of an instance of yield tokenizer
            tokenizer_component_address: ComponentAddress,
        ) -> Global<YieldAMM> {
            assert!(scalar_root > Decimal::ZERO);
            assert!(fee_rate > Decimal::ZERO);
            assert!(reserve_fee_percent > Decimal::ZERO && reserve_fee_percent < Decimal::ONE);

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(YieldAMM::blueprint_id());
            let global_component_caller_badge =
                NonFungibleGlobalId::global_caller_badge(component_address);

            let flash_loan_rm: ResourceManager = 
                ResourceBuilder::new_ruid_non_fungible::<FlashLoanReceipt>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Flash Loan FlashLoanReceipt", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(deny_all);
                    depositor_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            let tokenizer = Self::get_component(tokenizer_component_address);
            let pt_address= tokenizer.pt_address();
            let lsu_address = tokenizer.underlying_resource();

            let pool_component = 
                Blueprint::<TwoResourcePool>::instantiate(
                owner_role.clone(),
                rule!(require(global_component_caller_badge)),
                (pt_address, lsu_address),
                None,
            );

            let fee_rate = PreciseDecimal::from(fee_rate.ln().unwrap());
            let maturity_date = tokenizer.maturity_date();

            Self {
                pool_component,
                flash_loan_rm,
                maturity_date,
                scalar_root,
                fee_rate,
                reserve_fee_percent,
                last_ln_implied_rate: PreciseDecimal::ZERO,
                lsu_address: lsu_address,
                tokenizer_component_address: tokenizer_component_address
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }

        // First set the natural log of the implied rate here.
        // We also set optional inital anchor rate as the there isn't an anchor rate yet until we have the implied rate.
        // The initial anchor rate is determined by a guess on the interest rate which trading will be most capital efficient.
        pub fn set_initial_ln_implied_rate(
            &mut self, 
            initial_rate_anchor: PreciseDecimal
        ) {
            assert_eq!(
                self.last_ln_implied_rate, 
                PreciseDecimal::ZERO,
                "Initial Ln Implied Rate has already been set"
            );

            let time_to_expiry = self.time_to_expiry();

            let rate_scalar = calc_rate_scalar(
                self.scalar_root,
                time_to_expiry
            );

            self.last_ln_implied_rate = self.get_ln_implied_rate( 
                time_to_expiry,
                rate_scalar,
                initial_rate_anchor
            );

            info!("Implied Rate: {:?}", self.last_ln_implied_rate.exp().unwrap());
        }

        pub fn get_market_implied_rate(&mut self) -> PreciseDecimal {
            self.last_ln_implied_rate.exp().unwrap()
        }
        
        pub fn get_vault_reserves(&self) -> IndexMap<ResourceAddress, Decimal> {
            self.pool_component.get_vault_amounts()
        }

        /// Adds liquidity to pool reserves.
        /// 
        /// # Arguments
        ///
        /// * `lsu_tokens`: [`FungibleBucket`] - A fungible bucket of LSU token supply.
        /// * `principal_token`: [`FungibleBucket`] - A fungible bucket of principal token supply.
        ///
        /// # Returns
        /// 
        /// * [`Bucket`] - A bucket of `pool_unit`.
        /// * [`Option<Bucket>`] - An optional bucket of any remainder token.
        pub fn add_liquidity(
            &mut self, 
            lsu_token: FungibleBucket, 
            principal_token: FungibleBucket
        ) -> (Bucket, Option<Bucket>) {
            self.pool_component.contribute((lsu_token.into(), principal_token.into()))
        }

        /// Redeems pool units for the underlying pool assets.
        /// 
        /// # Arguments
        ///
        /// * `pool_units`: [`FungibleBucket`] - A fungible bucket of `pool_units` tokens to
        /// to redeem for underlying pool assets. 
        ///
        /// # Returns
        /// 
        /// * [`Bucket`] - A bucket of PT.
        /// * [`Bucket`] - A bucket of LSU tokens.
        pub fn remove_liquidity(
            &mut self, 
            pool_units: FungibleBucket
        ) -> (Bucket, Bucket) {
            self.pool_component.redeem(pool_units.into())
        }

        /// Swaps the given PT for LSU tokens.
        /// 
        /// # Arguments
        ///
        /// * `principal_token`: [`FungibleBucket`] - A fungible bucket of PT tokens to
        /// to swap for LSU. 
        ///
        /// # Returns
        ///
        /// * [`FungibleBucket`] - A bucket of LSU tokens.
        pub fn swap_exact_pt_for_lsu(
            &mut self, 
            principal_token: FungibleBucket
        ) -> FungibleBucket {
            assert_ne!(self.check_maturity(), true, "Market has reached its maturity");

            let tokenizer = Self::get_component(self.tokenizer_component_address);
            assert_eq!(principal_token.resource_address(), tokenizer.pt_address());

            let time_to_expiry = self.time_to_expiry();

            // Calcs the rate scalar and rate anchor with the current market state
            let market_compute = self.compute_market(time_to_expiry);

            // Calcs the the swap
            let lsu_to_account = self.calc_trade(
                principal_token.amount().checked_neg().unwrap(), 
                time_to_expiry,
                market_compute.clone()
            );

            info!(
                "[swap_exact_pt_for_lsu] All-in Exchange rate: {:?}", 
                principal_token.amount().checked_div(lsu_to_account).unwrap()
            );

            // Deposit all given PT tokens to the pool.
            self.pool_component.protected_deposit(principal_token.into());

            // Withdraw the amount of LSU tokens from the pool.
            let owed_lsu_bucket = self.pool_component.protected_withdraw(
                tokenizer.underlying_resource(), 
                lsu_to_account, 
                WithdrawStrategy::Rounded(RoundingMode::ToZero)
            );

            // Saves the new implied rate.
            self.last_ln_implied_rate = 
                self.get_ln_implied_rate(
                    time_to_expiry, 
                    market_compute.rate_scalar, 
                    market_compute.rate_anchor
                );

            info!(
                "[swap_exact_pt_for_lsu] LSU Returned: {:?}", 
                owed_lsu_bucket.amount()
            );

            return owed_lsu_bucket.as_fungible()
        }

        /// Swaps the given PT for LSU tokens.
        ///
        /// # Arguments
        ///
        /// * `lsu_token`: [`FungibleBucket`] - A fungible bucket of LSU tokens to
        /// swap for PT.
        /// * `desired_pt_amount`: [`Decimal`] - The amount of PT the user
        /// wants.
        ///
        /// # Returns
        ///
        /// * [`FungibleBucket`] - A bucket of PT.
        /// * [`FungibleBucket`] - A bucket of any remaining LSU tokens.
        /// 
        /// Notes:
        /// I believe it needs to be calculated this way because formula for trades is easier 
        /// based on PT being swapped in/ou but not for LSUs.
        /// 
        /// Challengers have room for improvements to approximate required LSU better such that it equals
        /// the LSU sent in. 
        pub fn swap_exact_lsu_for_pt(
            &mut self, 
            mut lsu_token: FungibleBucket, 
            desired_pt_amount: Decimal
        ) -> (FungibleBucket, FungibleBucket) {
            assert_ne!(self.check_maturity(), true, "Maturity date has lapsed");
            let tokenizer = Self::get_component(self.tokenizer_component_address);
            assert_eq!(lsu_token.resource_address(), tokenizer.underlying_resource());

            let time_to_expiry = self.time_to_expiry();

            // Calcs the rate scalar and rate anchor with the current market state
            let market_compute = self.compute_market(time_to_expiry);

            // Calcs the swap
            let required_lsu = self.calc_trade(
                desired_pt_amount,
                time_to_expiry,
                market_compute.clone()
            );

            // Assert the amount of LSU sent in is at least equal to the required
            // LSU needed for the desired PT amount.
            assert!(lsu_token.amount() >= required_lsu);

            info!(
                "[swap_exact_lsu_for_pt] All-in Exchange rate: {:?}", 
                desired_pt_amount.checked_div(required_lsu).unwrap()
            );

            // Only need to take the required LSU, return the rest.
            let required_lsu_bucket = lsu_token.take(required_lsu);

            info!(
                "[swap_exact_lsu_for_pt] Required LSU: {:?}", 
                required_lsu_bucket.amount()
            );

            // Deposit the required LSU to the pool.
            self.pool_component.protected_deposit(required_lsu_bucket.into());

            // Withdraw the desired PT amount.
            let owed_pt_bucket = self.pool_component.protected_withdraw(
                tokenizer.pt_address(), 
                desired_pt_amount, 
                WithdrawStrategy::Rounded(RoundingMode::ToZero)
            );

            // Saves the new implied rate of the trade.
            self.last_ln_implied_rate = 
                self.get_ln_implied_rate(
                    time_to_expiry, 
                    market_compute.rate_scalar, 
                    market_compute.rate_anchor
                );

            info!("[swap_exact_lsu_for_pt] Owed PT: {:?}", owed_pt_bucket.amount());

            return (owed_pt_bucket.as_fungible(), lsu_token)
        }   

        /// Swaps the given LSU token for YT (Buying YT)
        /// 
        /// # Arguments
        ///
        /// * `bucket`: [`FungibleBucket`] - A fungible bucket of LSU tokens to
        /// swap for YT.
        ///
        /// # Returns
        ///
        /// * [`FungibleBucket`] - A bucket of YT.
        /// 
        /// Note: In practice, the way an amount of YT can be determined given an
        /// LSU is by calculating the price of PT and YT based on P(PT) + P(YT) = LSU
        /// relationship. However, doing so require complex approximation algorithm
        /// which isn't covered in this implementation.
        pub fn swap_exact_lsu_for_yt(
            &mut self, 
            lsu_token: FungibleBucket
        ) -> (NonFungibleBucket, FungibleBucket) {
            assert_ne!(
                self.check_maturity(), 
                true, 
                "Market has reached its maturity"
            );

            // There would be an algorithm to estimate the PT that can be
            // swapped for LSU to determine the price of PT as this would
            // determine the amount of LSU one can borrow and pay back.
            // let est_max_pt_in = dec!(0); 
            // let time_to_expiry = self.time_to_expiry();

            // let lsu_amount = self.calc_trade(
            //     est_max_pt_in.checked_neg().unwrap(),
            //     time_to_expiry,
            //     market_compute.clone()
            // );   

            // let price_of_pt = lsu_amount.checked_div(max_pt_in).unwrap();
            // let price_of_yt = dec!(1).checked_sub(price_of_pt).unwrap();
            // let amount_of_yt = lsu_token.amount().checked_div(price_of_yt).unwrap();
            
            // let required_lsu_to_borrow =
            //     amount_of_yt.checked_sub(lsu_token.amount())
            //     .unwrap(); 

            // Get amount of YT per lsu based on above calculation
            // let (lsu_flash_loan, flash_loan_receipt) = self.flash_loan(
            //     lsu_token.resource_address(), 
            //     required_lsu_to_borrow
            // );
            
            // lsu_token.put(lsu_flash_loan);

            let mut tokenizer = Self::get_component(self.tokenizer_component_address);

            // Mints PT and YT token from all of the LSU
            let (principal_token, yield_token) = 
                tokenizer.tokenize_yield(lsu_token);

            // Swaps the PTs for lsu to return the amount from step 2
            // Determine how many PTs needed to swap enough lsus to repay the flash loan.
            let lsu_token = self.swap_exact_pt_for_lsu(principal_token);

            // let optional_return_bucket = self.flash_loan_repay(lsu_token, flash_loan_receipt);
            
            return (yield_token, lsu_token)
            
        }
        
        /// Swaps the given YT for LSU tokens (Selling YT):
        ///
        /// 1. Seller sends YT into the swap contract.
        /// 2. Contract borrows an equivalent amount of PT from the pool.
        /// 3. The YTs and PTs are used to redeem LSU.
        /// 4. Contract calculates the required LSU to swap back to PT.
        /// 5. A portion of the LSU is sold to the pool for PT to return the amount from step 2.
        /// 6. The remaining LSU is sent to the seller.
        ///
        /// # Arguments
        ///
        /// * `yield_token`: [`FungibleBucket`] - A fungible bucket of LSU tokens to
        /// swap for YT.
        /// * `amount_yt_to_swap_in`: [Decimal] - Amount of YT to swap in.
        ///
        /// # Returns
        ///
        /// * [`FungibleBucket`] - A bucket of LSU.
        /// * [`Option<NonFungibleBucket>`] - A bucket of YT if not all were used.
        /// * [`Option<FungibleBucket>`] - A bucket of unused LSU.
        pub fn swap_exact_yt_for_lsu(
            &mut self, 
            yield_token: NonFungibleBucket,
            amount_yt_to_swap_in: Decimal,
        ) -> (FungibleBucket, Option<NonFungibleBucket>, Option<FungibleBucket>) {
            assert_ne!(self.check_maturity(), true, "Market has reached its maturity");
            let mut tokenizer = Self::get_component(self.tokenizer_component_address);
            assert_eq!(yield_token.resource_address(), tokenizer.yt_address());
            
            // Need to borrow the same amount of PT as YT to redeem LSU
            let data: YieldTokenData = yield_token.non_fungible().data();
            let underlying_lsu_amount = data.underlying_lsu_amount;
            assert!(underlying_lsu_amount >= amount_yt_to_swap_in);
            let pt_flash_loan_amount = amount_yt_to_swap_in;

            // Borrow equivalent amount of PT from the pool - enough to get LSU
            let (pt_flash_loan, flash_loan_receipt) = 
                self.flash_loan(
                    tokenizer.pt_address(), 
                    pt_flash_loan_amount
                );

            // Combine PT and YT to redeem LSU
            let (mut lsu_token, option_yt_bucket) = 
                tokenizer.redeem(pt_flash_loan, yield_token, amount_yt_to_swap_in);

            // Retrieve flash loan requirements to ensure enough can be swapped back to repay
            // the flash loan.
            let flash_loan_data: FlashLoanReceipt = 
                flash_loan_receipt.as_non_fungible().non_fungible().data();

            let desired_pt_amount = flash_loan_data.amount;

            let time_to_expiry = self.time_to_expiry();
            let market_compute = self.compute_market(time_to_expiry);

            // Portion of lsu is sold to the pool for PT to return the borrowed PT
            let required_lsu = self.calc_trade(
                    desired_pt_amount,
                    time_to_expiry,
                    market_compute.clone()
                );

            info!(
                "[swap_exact_yt_for_lsu] All-in Exchange rate: {:?}", 
                desired_pt_amount.checked_div(required_lsu).unwrap()
            );

            info!(
                "[swap_exact_yt_for_lsu] All-in Exchange rate: {:?}", 
                required_lsu.checked_div(desired_pt_amount).unwrap()
            );

            let required_lsu_bucket = lsu_token.take(required_lsu);

            let (pt_flash_loan_repay, returned_lsu) = 
                self.swap_exact_lsu_for_pt(required_lsu_bucket, desired_pt_amount);

            lsu_token.put(returned_lsu);
            
            let optional_return_bucket = self.flash_loan_repay(pt_flash_loan_repay, flash_loan_receipt);

            self.last_ln_implied_rate = self.get_ln_implied_rate(
                time_to_expiry, 
                market_compute.rate_scalar, 
                market_compute.rate_anchor
            );

            info!("[swap_exact_yt_for_lsu] LSU Returned: {:?}", lsu_token.amount());

            return (lsu_token, option_yt_bucket, optional_return_bucket)
        }

        fn compute_market(
            &self,
            time_to_expiry: i64
        ) -> MarketCompute {

            let proportion = calc_proportion(
                dec!(0),
                self.get_vault_reserves()[0],
                self.get_vault_reserves()[1]
            );

            let rate_scalar = calc_rate_scalar(
                self.scalar_root, 
                time_to_expiry
            );

            let rate_anchor = calc_rate_anchor(
                self.last_ln_implied_rate,
                proportion,
                time_to_expiry,
                rate_scalar
            );

            MarketCompute {
                rate_scalar,
                rate_anchor,
            }
        }

        /// Calculates the the trade based on the direction of the trade.
        /// 
        /// This method retrieves the exchange rate, 
        fn calc_trade(
            &mut self,
            net_pt_amount: Decimal,
            time_to_expiry: i64,
            market_compute: MarketCompute
        ) -> Decimal {

            let proportion = calc_proportion(
                net_pt_amount,
                self.get_vault_reserves()[0],
                self.get_vault_reserves()[1]
            );

            // Calcs exchange rate based on size of the trade (change)
            let pre_fee_exchange_rate = calc_exchange_rate(
                proportion,
                market_compute.rate_anchor,
                market_compute.rate_scalar
            );

            let pre_fee_amount = 
                net_pt_amount
                .checked_div(pre_fee_exchange_rate)
                .unwrap()
                .checked_neg()
                .unwrap();

            let fee = calc_fee(
                self.fee_rate,
                time_to_expiry,
                net_pt_amount,
                pre_fee_exchange_rate,
                pre_fee_amount
            );

            // Fee allocated to the asset reserve
            let net_asset_fee_to_reserve =
                fee
                .checked_mul(self.reserve_fee_percent)
                .unwrap();

            // Trading fee allocated to the reserve based on the direction
            // of the trade.
            let trading_fee = 
                fee
                .checked_sub(net_asset_fee_to_reserve)
                .unwrap();

            let net_amount = 
            // If this is [swap_exact_pt_to_lsu] then pre_fee_lsu_to_account is negative and
            // fee is positive so it actually adds to the net_lsu_to_account.
                pre_fee_amount
                .checked_sub(trading_fee)
                .unwrap();

            // Net amount can be negative depending on direciton of the trade.
            // However, we want to have net amount to be positive to be able to 
            // perform the asset swap.
            let net_amount = if net_amount < PreciseDecimal::ZERO {
                // LSU ---> PT
                net_amount
                .checked_add(net_asset_fee_to_reserve)
                .and_then(|result| result.checked_abs())
                .unwrap()
            } else {
                // PT ---> LSU
                net_amount
                .checked_sub(net_asset_fee_to_reserve)
                .unwrap()
            };

            return Decimal::try_from(net_amount).ok().unwrap()
            

        }

        
        /// Takes a flash loan of a resource and amount from pool reserves.
        /// 
        /// This method mints a transient `FlashLoanReceipt` NFT which must be burnt.
        ///
        /// # Arguments
        ///
        /// * `resource`: [`ResourceAddress`] - The resource to borrow.
        /// * `amount`: [`Decimal`] - The amount to borrow.
        /// wants.
        ///
        /// # Returns
        ///
        /// * [`FungibleBucket`] - A fungible bucket of requested loan.
        /// * [`NonFungibleBucket`] - A non fungible bucket of the flash loan receipt NFT.
        /// 
        /// Note: This method is private due to the way implied rates are saved. 
        fn flash_loan(
            &mut self, 
            resource: ResourceAddress, 
            amount: Decimal
        ) -> (FungibleBucket, NonFungibleBucket) {
            
            let flash_loan_receipt = self.flash_loan_rm.mint_ruid_non_fungible(
                FlashLoanReceipt {
                    resource,
                    amount,
                }
            )
            .as_non_fungible();

            let flash_loan = self.pool_component.protected_withdraw(
                resource, 
                amount, 
                WithdrawStrategy::Rounded(RoundingMode::ToZero)
            )
            .as_fungible();
        
            return (flash_loan, flash_loan_receipt)
        }

        /// Repays flash loan
        ///
        /// # Arguments
        ///
        /// * `flash_loan`: [`FungibleBucket`] - A fungible bucket of the flash
        /// loan repayment.
        /// * `flash_loan_receipt`: [`NonFungibleBucket`] - A non fungible bucket
        /// of the flash loan receipt NFT.
        ///
        /// # Returns
        ///
        /// * [`Option<FungibleBucket>`] - An option fungible bucket of repayment 
        /// overages.
        fn flash_loan_repay(
            &mut self, 
            mut flash_loan: FungibleBucket, 
            flash_loan_receipt: NonFungibleBucket
        ) -> Option<FungibleBucket> {
            let mut flash_loan_receipt_data: FlashLoanReceipt = flash_loan_receipt.as_non_fungible().non_fungible().data();
            let flash_loan_repay = flash_loan.take(flash_loan_receipt_data.amount);
            flash_loan_receipt_data.amount -= flash_loan_repay.amount();

            assert_eq!(self.flash_loan_rm.address(), flash_loan_receipt.resource_address());
            assert_eq!(flash_loan.resource_address(), flash_loan_receipt_data.resource);
            assert_eq!(flash_loan_receipt_data.amount, Decimal::ZERO);

            self.pool_component.protected_deposit(flash_loan_repay.into());

            flash_loan_receipt.burn();

            return Some(flash_loan)
        }

        /// Retrieves current market implied rate.
        fn get_ln_implied_rate(
            &mut self, 
            time_to_expiry: i64, 
            rate_scalar: Decimal,
            rate_anchor: PreciseDecimal
        ) -> PreciseDecimal {

            let proportion = calc_proportion(
                dec!(0),
                self.get_vault_reserves()[0],
                self.get_vault_reserves()[1]
            );

            let exchange_rate = calc_exchange_rate(
                proportion,
                rate_anchor,
                rate_scalar
            );

            // exchangeRate >= 1 so its ln >= 0
            let ln_exchange_rate = exchange_rate.ln().unwrap();

            let ln_implied_rate = 
                ln_exchange_rate.checked_mul(PERIOD_SIZE)
                .and_then(|result| result.checked_div(time_to_expiry))
                .unwrap();

            return ln_implied_rate
        }

        pub fn time_to_expiry(&self) -> i64 {
            self.maturity_date.to_instant().seconds_since_unix_epoch 
                - Clock::current_time_rounded_to_seconds().seconds_since_unix_epoch
        }

        /// Checks whether maturity has lapsed
        pub fn check_maturity(&self) -> bool {
            Clock::current_time_comparison(
                self.maturity_date.to_instant(), 
                TimePrecision::Second, 
                TimeComparisonOperator::Gte
            )
        }

        //Get the component
        pub fn get_component(component_address: ComponentAddress) -> Global<YieldTokenizer> {

            let yield_tokenizer: Global<YieldTokenizer> = component_address.into();
        
            return yield_tokenizer
        }
    }
}



