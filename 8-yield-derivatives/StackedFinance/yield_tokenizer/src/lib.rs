use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub enum Expiry {
    TwelveMonths,
    EighteenMonths,
    TwentyFourMonths,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct YieldTokenData {
    underlying_lsu_resource: ResourceAddress,
    underlying_lsu_amount: Decimal,
    redemption_value_at_start: Decimal,
    yield_claimed: Decimal,
    maturity_date: UtcDateTime,
}

#[blueprint]
mod yield_tokenizer {
    struct YieldTokenizer {
        pt_rm: ResourceManager,
        yt_rm: ResourceManager,
        maturity_date: UtcDateTime,
        lsu_validator_component: Global<Validator>,
        lsu_address: ResourceAddress,
        lsu_vault: FungibleVault,
    }

    impl YieldTokenizer {
        pub fn instantiate_yield_tokenizer(
            expiry: Expiry,
            accepted_lsu: ResourceAddress,
        ) -> Global<YieldTokenizer> {

            let maturity_date = match expiry {
                Expiry::TwelveMonths => {
                    let current_time = Clock::current_time_rounded_to_seconds();
                    UtcDateTime::from_instant(&current_time.add_days(365).unwrap()).ok().unwrap()
                },
                Expiry::EighteenMonths => {
                    let current_time = Clock::current_time_rounded_to_seconds();
                    UtcDateTime::from_instant(&current_time.add_days(547).unwrap()).ok().unwrap()
                },
                Expiry::TwentyFourMonths => {
                    let current_time = Clock::current_time_rounded_to_seconds();
                    UtcDateTime::from_instant(&current_time.add_days(730).unwrap()).ok().unwrap()
                },
            };

        let (address_reservation, component_address) =
            Runtime::allocate_component_address(YieldTokenizer::blueprint_id());

            let validator_name = Self::retrieve_validator_name(accepted_lsu);

            //Truncate the name incase it's too long
            let truncated_name = if validator_name.len() > 16 {
                &validator_name[..16]
            } else {
                &validator_name
            }.trim().to_string();

            //Truncate the name fort a symbol
            let truncated_symbol = if validator_name.len() > 6 {
                &validator_name[..6]
            } else {
                &validator_name
            }.trim().to_string();

            let pt_rm: ResourceManager = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata! {
                    init {
                        "name" => truncated_name.clone() + " Principal Token", locked;
                        "symbol" => truncated_symbol.clone() + "PT", locked;
                        "yield_tokenizer_component" => GlobalAddress::from(component_address), locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(allow_all);
                    // minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            
            let yt_rm: ResourceManager = 
                ResourceBuilder::new_ruid_non_fungible::<YieldTokenData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => truncated_name.clone() + " Yield Receipt", locked;
                        "symbol" => truncated_symbol.clone() + "YT", locked;
                        "yield_tokenizer_component" => GlobalAddress::from(component_address), locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(allow_all);
                    burner_updater => rule!(deny_all);
                })
                .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                    non_fungible_data_updater => rule!(require(global_caller(component_address)));
                    non_fungible_data_updater_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

                let lsu_validator_component = Self::retrieve_validator_component(accepted_lsu);

                assert_eq!(Self::validate_lsu(accepted_lsu), true, "Not an LSU!");
            

            Self {
                pt_rm,
                yt_rm,
                maturity_date,
                lsu_validator_component,
                lsu_address: accepted_lsu,
                lsu_vault: FungibleVault::new(accepted_lsu),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }

        fn retrieve_validator_component(
            lsu_address: ResourceAddress
        ) -> Global<Validator> {
            let metadata: GlobalAddress = 
                ResourceManager::from(lsu_address)
                .get_metadata("validator")
                .unwrap()
                .unwrap_or_else(||
                    Runtime::panic(String::from("Not an LSU!"))
                );
            ComponentAddress::try_from(metadata)
                .unwrap()
                .into()
        }

        fn retrieve_validator_name(
            input_lsu_address: ResourceAddress
        ) -> String {
            let metadata: GlobalAddress = 
                ResourceManager::from(input_lsu_address)
                .get_metadata("validator")
                .unwrap()
                .unwrap_or_else(||
                    Runtime::panic(String::from("Not an LSU!"))
                );
            let validator_address = 
                ComponentAddress::try_from(metadata).unwrap();
            let validator: Global<Validator> = 
                Global::from(validator_address);
            let validator_name: String = 
                validator
                .get_metadata("name")
                .unwrap()
                .unwrap_or_else(||
                    Runtime::panic(String::from("No name metadata!"))
                );
    
            // Return validator name for the token
            return validator_name;
        }

        fn validate_lsu(
            input_lsu_address: ResourceAddress
        ) -> bool {
            let metadata: GlobalAddress = 
                ResourceManager::from(input_lsu_address)
                .get_metadata("validator")
                .unwrap()
                .unwrap_or_else(||
                    Runtime::panic(String::from("Not an LSU!"))
                );
            let validator_address = 
                ComponentAddress::try_from(metadata).unwrap();
            let validator: Global<Validator> = 
                Global::from(validator_address);
            let lsu_address: GlobalAddress = 
                validator
                .get_metadata("pool_unit")
                .unwrap()
                .unwrap_or_else(||
                    Runtime::panic(String::from("Not an LSU!"))
                );
            
            input_lsu_address == ResourceAddress::try_from(lsu_address).unwrap()
        }

        /// Tokenizes the LSU to its PT and YT.
        ///
        /// # Arguments
        ///
        /// * `lsu_token`: [`FungibleBucket`] - A fungible bucket of LSU tokens to tokenize.
        ///
        /// # Returns
        ///
        /// * [`FungibleBucket`] - A fungible bucket of PT.
        /// * [`NonFungibleBucket`] - A non fungible bucket of YT.
        pub fn tokenize_yield(
            &mut self, 
            lsu_token: FungibleBucket
        ) -> (FungibleBucket, NonFungibleBucket) {
            assert_ne!(self.check_maturity(), true, "The expiry date has passed!");
            assert_eq!(lsu_token.resource_address(), self.lsu_address);

            let lsu_amount = lsu_token.amount();
            let redemption_value = 
                self.lsu_validator_component
                    .get_redemption_value(lsu_token.amount());

            let pt_bucket = 
                self.pt_rm.mint(lsu_amount).as_fungible();
            let yt_bucket = 
                self.yt_rm
                .mint_ruid_non_fungible(
                    YieldTokenData {
                        underlying_lsu_resource: self.lsu_address,
                        underlying_lsu_amount: lsu_amount,
                        redemption_value_at_start: redemption_value,
                        yield_claimed: Decimal::ZERO,
                        maturity_date: self.maturity_date
                    }
                ).as_non_fungible();
            
            self.lsu_vault.put(lsu_token);

            return (pt_bucket, yt_bucket)
        }

        /// Redeems the underlying LSU from PT and YT.
        ///
        /// # Arguments
        ///
        /// * `pt_bucket`: [`FungibleBucket`] - A fungible bucket of PT.
        /// * `yt_bucket`: [`NonFungibleBucket`] - A non fungible bucket of YT.
        /// * `yt_redeem_amount`: [`Decimal`] - Desired amount of YT to redeem.
        ///
        /// # Returns
        ///
        /// * [`FungibleBucket`] - A fungible bucket of the owed LSU.
        /// * [`Option<NonFungibleBucket>`] - Returns a non fungible bucket of YT
        /// if not all is redeemed.
        pub fn redeem(
            &mut self, 
            pt_bucket: FungibleBucket, 
            yt_bucket: NonFungibleBucket, 
            yt_redeem_amount: Decimal,
        ) -> (FungibleBucket, Option<NonFungibleBucket>) {
            let mut data: YieldTokenData = yt_bucket.non_fungible().data();    
            assert!(data.underlying_lsu_amount >= yt_redeem_amount);            
            assert_eq!(pt_bucket.amount(), yt_redeem_amount);
            assert_eq!(pt_bucket.resource_address(), self.pt_rm.address());
            assert_eq!(yt_bucket.resource_address(), self.yt_rm.address());

            let lsu_bucket = self.lsu_vault.take(pt_bucket.amount());

            let option_yt_bucket: Option<NonFungibleBucket> = if data.underlying_lsu_amount > yt_redeem_amount {
                data.underlying_lsu_amount -= yt_redeem_amount;
                Some(yt_bucket)
            } else {
                yt_bucket.burn();
                None
            };

            pt_bucket.burn();

            return (lsu_bucket, option_yt_bucket)
        }

        /// Redeems the underlying LSU from PT.
        /// 
        /// Can only redeem from PT if maturity date has passed.
        ///
        /// # Arguments
        ///
        /// * `pt_bucket`: [`FungibleBucket`] - A fungible bucket of PT.
        ///
        /// # Returns
        ///
        /// * [`FungibleBucket`] - A fungible bucket of the owed LSU.
        pub fn redeem_from_pt(
            &mut self,
            pt_bucket: FungibleBucket,
        ) -> FungibleBucket {
            // To redeem PT only, must wait until after maturity.
            assert_eq!(
                self.check_maturity(), 
                true, 
                "The Principal Token has not reached its maturity!"
            );
            assert_eq!(pt_bucket.resource_address(), self.pt_rm.address());

            let bucket_of_lsu = self.lsu_vault.take(pt_bucket.amount());
            pt_bucket.burn();

            return bucket_of_lsu
        }

        /// Claims owed yield for the period.
        ///
        /// # Arguments
        ///
        /// * `yt_proof`: [`NonFungibleProof`] - A non fungible proof of YT.
        ///
        /// # Returns
        ///
        /// * [`Bucket`] - A bucket of the Unstake NFT.
        /// Note: https://docs.radixdlt.com/docs/validator#unstake-nft
        pub fn claim_yield(
            &mut self, 
            yt_proof: NonFungibleProof,
        ) -> Bucket {
            // Can no longer claim yield after maturity.
            assert_ne!(
                self.check_maturity(), 
                true, 
                "The yield token has reached its maturity!"
            );
            
            let checked_proof = 
                yt_proof.check(self.yt_rm.address());
            let mut data: YieldTokenData = 
                checked_proof.non_fungible().data();

            // Calc yield owed (redemption value) based on difference of current redemption 
            // value and redemption value at start.
            let yield_owed = 
                self.calc_yield_owed(&data);
            
            // Calc amount of LSU to redeem to achieve yield owed.
            let required_lsu_for_yield_owed = 
                self.calc_required_lsu_for_yield_owed(yield_owed);

            // Burn the yield token by the amount of LSU required to redeem.
            data.underlying_lsu_amount -= required_lsu_for_yield_owed;
            data.yield_claimed += yield_owed;

            // LSU amount decreases but redemption value is the same
            let required_lsu_bucket = 
                self.lsu_vault.take(required_lsu_for_yield_owed);

            self.lsu_validator_component.unstake(required_lsu_bucket.into())
        }

        /// Calculates earned yield of YT.
        ///
        /// # Arguments
        ///
        /// * `data`: [`&YieldTokenData`] - The `NonFungibleData` of YT.
        ///
        /// # Returns
        ///
        /// * [`Decimal`] - The calculated earned yield from YT for the current period.
        fn calc_yield_owed(
            &self,
            data: &YieldTokenData,
        ) -> Decimal {
            let redemption_value = 
            self.lsu_validator_component
                .get_redemption_value(data.underlying_lsu_amount);

            info!("Redemption Value: {:?}", redemption_value);

            let redemption_value_at_start = 
                data.redemption_value_at_start;

            info!("Redemption Value: {:?}", redemption_value_at_start);

            assert!(
                redemption_value > redemption_value_at_start, 
                "No rewards earned yet."
            );

            redemption_value
            .checked_sub(redemption_value_at_start)
            .unwrap()
        }

        /// Calculates the required LSU to redeem yield earned for the period.
        ///
        /// # Arguments
        ///
        /// * `yield_owed`: [`Decimal`] - The redemption value of the yield owed.
        ///
        /// # Returns
        ///
        /// * [`Decimal`] - The required LSU amount to redeem yield owed.
        fn calc_required_lsu_for_yield_owed(
            &self, 
            yield_owed: Decimal
        ) -> Decimal {
            let total_xrd_staked = self.lsu_validator_component.total_stake_xrd_amount();
            let total_lsu_supply = self.lsu_validator_component.total_stake_unit_supply();

            total_xrd_staked
                .checked_div(total_lsu_supply)
                .and_then(|result| yield_owed.checked_mul(result))
                .unwrap()
        }

        /// Retrieves the `ResourceAddress` of PT.
        /// 
        /// # Returns
        ///
        /// * [`ResourceAddress`] - The address of PT.
        pub fn pt_address(&self) -> ResourceAddress {
            self.pt_rm.address()
        }

        /// Retrieves the `ResourceAddress` of YT.
        /// 
        /// # Returns
        ///
        /// * [`ResourceAddress`] - The address of YT.
        pub fn yt_address(&self) -> ResourceAddress {
            self.yt_rm.address()
        }

        /// Retrieves the `ResourceAddress` of the underlying LSU.
        /// 
        /// # Returns
        ///
        /// * [`ResourceAddress`] - The address of the underlying LSU.
        pub fn underlying_resource(&self) -> ResourceAddress {
            self.lsu_address
        }

        /// Retrieves the maturity date.
        /// 
        /// # Returns
        ///
        /// * [`UtcDateTime`] - The maturity date.
        pub fn maturity_date(&self) -> UtcDateTime {
            self.maturity_date
        }

        /// Checks whether maturity date has been reached.
        pub fn check_maturity(&self) -> bool {
            Clock::current_time_comparison(
                self.maturity_date.to_instant(), 
                TimePrecision::Second, 
                TimeComparisonOperator::Gte
            )
        }
    }
}