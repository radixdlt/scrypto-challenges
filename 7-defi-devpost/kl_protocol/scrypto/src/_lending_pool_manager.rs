use crate::_comon::*;
use crate::_extended_cdp::*;
use crate::_lending_pool::*;
use scrypto::prelude::*;

//
#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Copy, Clone)]
pub struct CollateralPostion {
    pub position_id: u128,
    pub pool_resource_address: ResourceAddress,
    pub pool_share_resource_address: ResourceAddress,
    pub pool_component_address: ComponentAddress,
    pub pool_share: Decimal,
}

#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct DebtPostion {
    pub position_id: u128,
    pub pool_resource_address: ResourceAddress,
    pub pool_component_address: ComponentAddress,
    pub loan_share: Decimal,
    pub interest_type: u8,
}

#[derive(
    NonFungibleData, LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone,
)]
pub struct CollaterizedDebtPositionData {
    #[mutable]
    pub delegator_id: Option<NonFungibleLocalId>,
    #[mutable]
    pub delegated_ids: Vec<NonFungibleLocalId>,
    #[mutable]
    pub collaterals: HashMap<u128, CollateralPostion>,
    #[mutable]
    pub debts: HashMap<u128, DebtPostion>,
}

// REFENRECENCE TO AN EXCHANGE COMPONENTS
external_component! {
    ExchangeComponentTarget {
        fn swap(&self,  to_swap:Bucket, to_get: ResourceAddress) -> Bucket;
    }
}

// REFENRECENCE TO LENDINGPOOL COMPONENTS
external_component! {
    LendingPoolComponentTarget {

        fn add_liquidity(&mut self, assets: Bucket) -> Bucket;

        fn remove_liquidity(&mut self, pool_shares: Bucket) -> Bucket;

        fn take_flash_loan(&mut self, loan_amount: Decimal) -> (Bucket, Bucket);

        fn repay_flash_loan(&mut self, loan_repayment: Bucket,  loan_terms: Bucket) -> Bucket;

        //

        fn add_collected_fees(&mut self, tokens: Bucket);

        fn remove_collected_fees(&mut self, amount: Decimal) -> Bucket;

        fn insurance_deposit(&mut self, tokens: Bucket) ;

        fn insurance_withdraw(&mut self, amount: Decimal) -> Bucket;

    }

}

#[blueprint]
mod _lending_pool_manager {

    struct LendingPoolManager {
        lending_market_component_badge: Vault,

        cdp_resource_address: ResourceAddress,

        lending_pool_registry: ComponentAddressRegistery,

        exchange_component_address: ComponentAddress,

        debt_position_counter: u128,
        collateral_position_counter: u128,
        cdp_position_counter: u64,

        admin_badge_resource_address: ResourceAddress,

        // ! Store User CDP DATA for easy Access Only for tests purpose
        cdp_data_lookup: HashMap<NonFungibleLocalId, CollaterizedDebtPositionData>,
    }

    impl LendingPoolManager {
        // Define a public function "new" that takes in an oracle and exchange address and returns a tuple of ComponentAddress and Bucket.
        pub fn instantiate(
            exchange_component_address: ComponentAddress,
        ) -> (ComponentAddress, Bucket) {
            // Mint the LendingPoolManager Collaterized Debt Position Authority badge
            let lending_market_component_badge = Vault::with_bucket(
                ResourceBuilder::new_uuid_non_fungible().mint_initial_supply([(AuthBadgeData {})]),
            );

            // Create an NFT  for the lending market admin badge and mint an initial supply of 1.
            let pool_admin_badge = ResourceBuilder::new_uuid_non_fungible()
                .metadata("internal_tag", "lendind_market_admin_badge")
                .metadata("name", "KLP Admin Badge")
                .metadata("description", "KL Protocol administrator authority badge")
                .mint_initial_supply([(AuthBadgeData {})]);

            // Create an integer non-fungible resource for the collaterized debt position (CDP), mintable and burnable only by the CDP admin badge, and updateable only by the same badge.
            let cdp_resource_address = ResourceBuilder::new_integer_non_fungible()
                .metadata("internal_tag", "cdp_resource")
                .metadata("name", "KL Protocol CDP")
                .metadata(
                    "description",
                    "Represente a KL Protocole Collaterized Debt Position",
                )
                .mintable(
                    rule!(require(lending_market_component_badge.resource_address())),
                    LOCKED,
                )
                .burnable(
                    rule!(require(lending_market_component_badge.resource_address())),
                    LOCKED,
                )
                .updateable_non_fungible_data(
                    rule!(require(lending_market_component_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply();

            // Define an access rule that requires the lending market admin badge for the "create_market" and "auto_liquidate" methods.
            let component_manager_rule =
                rule!(require(lending_market_component_badge.resource_address()));

            let admin_rule = rule!(require(pool_admin_badge.resource_address()));

            // Define the access rules for the component, allowing only the defined admin_rule for the "create_market" and "auto_liquidate" methods.
            let access_rules = AccessRules::new()
                // Available publicly
                .method("create_cdp", AccessRule::AllowAll, LOCKED)
                .method("add_liquidity", AccessRule::AllowAll, LOCKED)
                .method("remove_liquidity", AccessRule::AllowAll, LOCKED)
                .method("take_flash_loan", AccessRule::AllowAll, LOCKED)
                .method("repay_flash_loan", AccessRule::AllowAll, LOCKED)
                // Available publicly but will require holding a CDP NFT
                .method("create_delegated_cdp", AccessRule::AllowAll, LOCKED)
                .method("new_collateral", AccessRule::AllowAll, LOCKED)
                .method("add_collateral", AccessRule::AllowAll, LOCKED)
                .method("remove_collateral", AccessRule::AllowAll, LOCKED)
                .method("borrow", AccessRule::AllowAll, LOCKED)
                .method("borrow_more", AccessRule::AllowAll, LOCKED)
                .method("repay", AccessRule::AllowAll, LOCKED)
                .method("change_interest_type", AccessRule::AllowAll, LOCKED)
                // available publicly but will fail on healthy CDP
                .method("liquidate", AccessRule::AllowAll, LOCKED)
                // Pool creation,  auto liquidation and fees removal requires an admin badge
                // auto liquidqtion will still fail  on healthy CDP
                .method("create_lending_pool", admin_rule.clone(), LOCKED)
                .method("auto_liquidate", admin_rule.clone(), LOCKED)
                .method("remove_collected_fees", admin_rule.clone(), LOCKED)
                .method("insurance_deposit", admin_rule.clone(), LOCKED)
                // ! maybe a debat here - but make sens to lock them to the compenent badge for now
                // .method("add_collected_fees", admin_rule.clone(), LOCKED)
                // .method("insurance_withdraw", admin_rule.clone(), LOCKED)
                //
                // Lock all method by default for internal or cross component calls
                .default(component_manager_rule.clone(), LOCKED);

            // Instantiate our component with the previously created resources and addresses.
            let mut component = Self {
                lending_market_component_badge,
                cdp_resource_address,
                exchange_component_address,

                // Lookups matching lendingPools wwith  resources and pool share resources addresses
                lending_pool_registry: ComponentAddressRegistery::new(),

                debt_position_counter: 0u128,
                collateral_position_counter: 0u128,
                cdp_position_counter: 0u64,

                admin_badge_resource_address: pool_admin_badge.resource_address(),

                cdp_data_lookup: HashMap::new(),
            }
            .instantiate();

            // Add the access rules to the component.
            component.add_access_check(access_rules);

            let component_address = component.globalize();

            // LendingPoolComponentTarget::at(component_address).get_pool_state(true);

            // Return the globalized ComponentAddress and the lending market admin badge.
            (component_address, pool_admin_badge)
        }

        pub fn create_lending_pool(
            &mut self,

            // Pool Resources infos
            pool_resource_address: ResourceAddress,
            pool_share_resource_symbol: String,
            pool_share_resource_name: String,
            pool_share_resource_icon: String,

            // LendingPool Parameters
            flashloan_fee: Decimal,
            liquidation_threshold: Decimal,
            liquidation_spread: Decimal,
            liquidation_close_factor: Decimal,

            //
            interest_factory_address: ComponentAddress,
            price_feed_address: ComponentAddress,

            //
            internal_tag: String,
        ) {
            // Generate a new lending pool component and associated admin badge and pool share resource address
            let (lending_pool_component_address, pool_share_resource_address) =
                LendingPoolComponent::new(
                    self.lending_market_component_badge.resource_address(),
                    self.admin_badge_resource_address,
                    pool_resource_address.clone(),
                    pool_share_resource_symbol,
                    pool_share_resource_name,
                    pool_share_resource_icon,
                    flashloan_fee,
                    liquidation_threshold,
                    liquidation_spread,
                    liquidation_close_factor,
                    interest_factory_address,
                    price_feed_address,
                    internal_tag,
                );

            self.lending_pool_registry
                .regester_component(pool_resource_address, lending_pool_component_address);

            self.lending_pool_registry
                .regester_addresses(pool_resource_address, pool_share_resource_address);
        }

        // This function generates a new Collateralized Debt Position (CDP) for the LendingPoolManager component.
        // It creates a new unique ID for the CDP and mints it using the LendingPoolManager component badge.
        // The CDP is then returned to the user.
        pub fn create_cdp(&mut self) -> Bucket {
            // Get the resource manager for the CDP resource
            let cdp_resource_manager = borrow_resource_manager!(self.cdp_resource_address);

            let cdp_local_id = NonFungibleLocalId::Integer(self.get_new_cdp_id().into());

            // Mint a new CDP using the CDP admin badge
            self.lending_market_component_badge.authorize(|| {
                let cdp_data = CollaterizedDebtPositionData {
                    delegator_id: None,
                    delegated_ids: [].to_vec(),
                    debts: HashMap::new(),
                    collaterals: HashMap::new(),
                };

                self.cdp_data_lookup
                    .insert(cdp_local_id.clone(), cdp_data.clone());

                cdp_resource_manager.mint_non_fungible(&cdp_local_id, cdp_data)
            })
        }

        pub fn create_delegated_cdp(&mut self, cdp_proof: Proof) -> Bucket {
            let delegator_cdp_id = self._validate_cdp_proof(cdp_proof);

            let cdp_resource_manager = borrow_resource_manager!(self.cdp_resource_address);

            let mut delegator_cdp_data: CollaterizedDebtPositionData =
                cdp_resource_manager.get_non_fungible_data(&delegator_cdp_id);

            assert!(
                delegator_cdp_data.delegator_id.is_none(),
                "Delegated CDP can not create delegated CDP",
            );

            let delegated_cdp_id = NonFungibleLocalId::Integer(self.get_new_cdp_id().into());

            delegator_cdp_data
                .delegated_ids
                .push(delegated_cdp_id.clone());

            let delegated_cdp_data = CollaterizedDebtPositionData {
                delegator_id: Some(delegator_cdp_id.clone()),
                delegated_ids: [].to_vec(),
                debts: HashMap::new(),
                collaterals: HashMap::new(),
            };

            self.cdp_data_lookup
                .insert(delegator_cdp_id.clone(), delegator_cdp_data.clone());

            self.cdp_data_lookup
                .insert(delegated_cdp_id.clone(), delegated_cdp_data.clone());

            self.lending_market_component_badge.authorize(|| {
                cdp_resource_manager
                    .update_non_fungible_data(&delegator_cdp_id, delegator_cdp_data);
                cdp_resource_manager.mint_non_fungible(&delegated_cdp_id, delegated_cdp_data)
            })
        }

        /*

        LendingPool PUBLIC PROXY METHODS

        Proxy methods for lending and flashloan feature. They use the ComponentRegistery instance to target relevent LendingPool

        */

        pub fn add_liquidity(&mut self, assets: Bucket) -> Bucket {
            let mut lending_pool_component = self._get_pool(assets.resource_address());

            lending_pool_component.add_liquidity(assets)
        }

        pub fn remove_liquidity(&mut self, pool_shares: Bucket) -> Bucket {
            let mut lending_pool_component = self._get_pool(pool_shares.resource_address());

            lending_pool_component.remove_liquidity(pool_shares)
        }

        pub fn take_flash_loan(
            &mut self,
            asstes_resource_address: ResourceAddress,
            amount: Decimal,
        ) -> (Bucket, Bucket) {
            let mut lending_pool_component = self._get_pool(asstes_resource_address);

            lending_pool_component.take_flash_loan(amount)
        }

        pub fn repay_flash_loan(
            &mut self,
            asstes_resource_address: ResourceAddress,
            loan_repayment: Bucket,
            loan_terms: Bucket,
        ) -> Bucket {
            let mut lending_pool_component = self._get_pool(asstes_resource_address);

            lending_pool_component.repay_flash_loan(loan_repayment, loan_terms)
        }

        /*

        LendingPool RESTRICTED PROXY METHODS

        Proxy methods that requires and admin badge or with access scope restricted to cross component calls.
        They use the ComponentRegistery instance to target relevent LendingPool.

        */

        pub fn add_collected_fees(&mut self, tokens: Bucket) {
            let mut lending_pool_component = self._get_pool(tokens.resource_address());

            lending_pool_component.add_collected_fees(tokens);
        }

        pub fn remove_collected_fees(
            &mut self,
            asstes_resource_address: ResourceAddress,
            amount: Decimal,
        ) -> Bucket {
            let mut lending_pool_component = self._get_pool(asstes_resource_address);

            lending_pool_component.remove_collected_fees(amount)
        }

        pub fn insurance_deposit(&mut self, tokens: Bucket) {
            let mut lending_pool_component = self._get_pool(tokens.resource_address());

            lending_pool_component.insurance_deposit(tokens);
        }

        pub fn insurance_withdraw(
            &mut self,
            resource_address: ResourceAddress,
            amount: Decimal,
        ) -> Bucket {
            let mut lending_pool_component = self._get_pool(resource_address);

            lending_pool_component.insurance_withdraw(amount)
        }

        /*

        ExtendedCollaterizedDebtPosition PUBLIC PROXY METHODS

        Proxy methods publicly available form the LendingPoolManager.
        They  require a valide CDP that will be use to generate an ExtendedCollaterizedDebtPosition then uuse it to performe request action.
        All Borrowing related action are done through the ExtendedCollaterizedDebtPosition. it put togather all required info and references to check the CDP health
        and decide the right action to to or not todo.

        */

        //
        // Create a new Collateral position and provision it
        pub fn new_collateral(&mut self, cdp_proof: Proof, assets: Bucket) {
            let (ra, pool_shre_ra) = self
                .lending_pool_registry
                .get_resource_address(assets.resource_address());

            let component_ra = self
                .lending_pool_registry
                .get_component_address(assets.resource_address());

            let mut extended_cdp = self._get_extended_cdp(self._validate_cdp_proof(cdp_proof));

            let new_collateral_position = CollateralPostion {
                position_id: self.get_new_collateral_id(),
                pool_resource_address: ra,
                pool_share_resource_address: pool_shre_ra,
                pool_component_address: component_ra,
                pool_share: dec!(0),
            };

            self.lending_market_component_badge.authorize(|| {
                extended_cdp.new_collateral(assets, new_collateral_position);
            });

            self._save_cdp(extended_cdp);
        }

        // Increase the collateral position of a known posision_id with the supply assets
        pub fn add_collateral(&mut self, cdp_proof: Proof, assets: Bucket, position_id: u128) {
            let mut extended_cdp = self._get_extended_cdp(self._validate_cdp_proof(cdp_proof));

            self.lending_market_component_badge.authorize(|| {
                extended_cdp.add_collateral(assets, position_id);
            });

            self._save_cdp(extended_cdp);
        }

        // Decrease the collateral position of a known posision_id by the given amount
        pub fn remove_collateral(
            &mut self,
            cdp_proof: Proof,
            position_id: u128,
            pool_share_amount: Decimal,
            get_pool_share: bool,
        ) -> Bucket {
            let mut extended_cdp = self._get_extended_cdp(self._validate_cdp_proof(cdp_proof));

            extended_cdp.chek_health_factor(true);

            let tokens = self.lending_market_component_badge.authorize(|| {
                extended_cdp.remove_collateral(position_id, pool_share_amount, get_pool_share)
            });

            extended_cdp.chek_health_factor(true);

            self._save_cdp(extended_cdp);

            tokens
        }

        //
        //
        pub fn borrow(
            &mut self,
            cdp_proof: Proof,
            resource_address: ResourceAddress,
            amount: Decimal,
            interest_type: u8,
        ) -> Bucket {
            let mut extended_cdp = self._get_extended_cdp(self._validate_cdp_proof(cdp_proof));

            extended_cdp.chek_health_factor(true);

            let component_ra = self
                .lending_pool_registry
                .get_component_address(resource_address);

            let new_debt_position = DebtPostion {
                position_id: self.get_debt_new_id(),
                pool_resource_address: resource_address,
                pool_component_address: component_ra,
                interest_type,
                loan_share: dec!(0),
            };

            let loan = self
                .lending_market_component_badge
                .authorize(|| extended_cdp.borrow(interest_type, amount, new_debt_position));

            extended_cdp.chek_health_factor(true);

            self._save_cdp(extended_cdp);

            loan
        }

        //
        //
        pub fn borrow_more(
            &mut self,
            cdp_proof: Proof,
            position_id: u128,
            amount: Decimal,
        ) -> Bucket {
            let mut extended_cdp = self._get_extended_cdp(self._validate_cdp_proof(cdp_proof));

            extended_cdp.chek_health_factor(true);

            let loan = self
                .lending_market_component_badge
                .authorize(|| extended_cdp.borrow_more(amount, position_id));

            extended_cdp.chek_health_factor(true);

            self._save_cdp(extended_cdp);

            loan
        }

        //
        //
        pub fn repay(&mut self, cdp_proof: Proof, payment: Bucket, position_id: u128) -> Bucket {
            let mut extended_cdp = self._get_extended_cdp(self._validate_cdp_proof(cdp_proof));

            let remainer = self
                .lending_market_component_badge
                .authorize(|| extended_cdp.repay(payment, position_id));

            self._save_cdp(extended_cdp);

            remainer
        }

        //
        //
        pub fn change_interest_type(
            &mut self,
            cdp_proof: Proof,
            position_id: u128,
            interest_type: u8,
            new_interest_type: u8,
        ) {
            let mut extended_cdp = self._get_extended_cdp(self._validate_cdp_proof(cdp_proof));

            let remainer = self.lending_market_component_badge.authorize(|| {
                extended_cdp.change_interest_type(position_id, interest_type, new_interest_type)
            });

            self._save_cdp(extended_cdp);

            remainer
        }

        //
        //
        pub fn liquidate(
            &mut self,
            cdp_id: NonFungibleLocalId,
            mut loan_payment: Bucket,
            debt_position_id: u128,
            collateral_position_id: u128,
        ) -> (Bucket, Bucket) {
            let mut extended_cdp = self._get_extended_cdp(cdp_id);

            extended_cdp.can_be_liquidated();

            // Step 2: Get collateral position
            let cp = match extended_cdp
                .collateral_positions
                .get(&collateral_position_id)
            {
                Some(c) => c,
                None => panic!("Collateral position not found"),
            };

            // Step 3: Get debt position
            let dp = match extended_cdp.debt_positions.get(&debt_position_id) {
                Some(c) => c,
                None => panic!("Debt position not found"),
            };

            // Step 4: Calculate max loan value
            let max_loan =
                dp.loan_share / dp.get_loan_share_ratio() * dp.pool_state.liquidation_close_factor;
            let max_loan_value = max_loan * dp.pool_state.last_price;

            // Step 5: Calculate max collateral value
            let max_collateral = cp.pool_share / cp.get_pool_share_ratio();
            let max_collateral_value = max_collateral * cp.pool_state.last_price
                / (dec!(1) + cp.pool_state.liquidation_spread);

            // Step 6: Calculate max payment value
            let max_payment_value = loan_payment.amount();

            // Step 7: Calculate base value
            let mut base_value = std::cmp::min(max_loan_value, max_collateral_value);
            base_value = std::cmp::min(base_value, max_payment_value);

            // Step 8: Calculate payment amount
            let payment_amount = base_value / dp.pool_state.last_price;

            // Step 9: Get collateral to be liquidated
            let liquidated_collateral_amount = base_value / cp.pool_state.last_price
                * (dec!(1) + cp.pool_state.liquidation_spread);

            let returned_collateral = self.lending_market_component_badge.authorize(|| {
                extended_cdp.remove_collateral(
                    collateral_position_id,
                    liquidated_collateral_amount,
                    false,
                )
            });

            // Step 10: Repay the loan
            let remainer = self.lending_market_component_badge.authorize(|| {
                extended_cdp.repay(loan_payment.take(payment_amount), debt_position_id)
            });

            // Step 11: Update CDP
            self._save_cdp(extended_cdp);

            // Step 12: Return remaining loan payment and collateral
            loan_payment.put(remainer);
            (loan_payment, returned_collateral)
        }

        /*

        ADMIN FUNCTION

        PoolManager Method that require an admin badge

        */

        //
        //
        pub fn auto_liquidate(&mut self, cdp_id: NonFungibleLocalId) {
            let exchange: ExchangeComponentTarget = self.exchange_component_address.into();

            let mut extended_cdp = self._get_extended_cdp(cdp_id);

            extended_cdp.can_be_auto_liquidated();

            let debt = &extended_cdp.clone().debt_positions;
            let collaterals = &extended_cdp.clone().collateral_positions;

            // debug!("debts:{}, collateral:{}", debt.len(), collaterals.len());

            for (_, dp) in debt {
                let mut n: u64 = (dec!(1) / dp.pool_state.liquidation_close_factor)
                    .floor()
                    .0
                    .into();
                n = n / 1000000000000000000u64 + 1;

                let mut i: u64 = 0;

                let mut liquidation_value = dp.loan_value;

                let max_loan_value = dp.loan_value * dp.pool_state.liquidation_close_factor;

                while i < n {
                    for (collateral_position_id, cp) in collaterals {
                        let liquidated_collateral_value = std::cmp::min(
                            cp.collateral_solvency_value,
                            std::cmp::min(max_loan_value, liquidation_value),
                        );

                        liquidation_value -= liquidated_collateral_value;

                        let liquidated_collateral_amount =
                            liquidated_collateral_value / cp.pool_state.last_price;

                        let liquidation_bonus_amount =
                            liquidated_collateral_amount * cp.pool_state.liquidation_spread;

                        ComponentAuthZone::push(self.lending_market_component_badge.create_proof());

                        let mut liquidated_collaterals = extended_cdp.remove_collateral(
                            *collateral_position_id,
                            liquidated_collateral_amount * cp.get_pool_share_ratio(),
                            false,
                        );

                        let liquidation_bonus =
                            liquidated_collaterals.take(liquidation_bonus_amount);

                        //

                        let remainer = extended_cdp.repay(
                            exchange.swap(liquidated_collaterals, dp.pool_resource_address),
                            dp.position_id,
                        );

                        self.add_collected_fees(remainer);

                        self.insurance_deposit(liquidation_bonus);

                        ComponentAuthZone::pop();

                        //
                        if extended_cdp.chek_health_factor(false) || liquidation_value == dec!(0) {
                            break;
                        }
                    }

                    //
                    if extended_cdp.chek_health_factor(false) || liquidation_value == dec!(0) {
                        break;
                    }

                    i = i + 1;
                }

                //
                if extended_cdp.chek_health_factor(false) || liquidation_value == dec!(0) {
                    break;
                }
            }

            debug!("{:?}", extended_cdp);

            self._save_cdp(extended_cdp);
        }

        // All debt and collateral are global idetified with an icremental ID.
        // This method increment the global counter for each new debt or collateral position
        pub fn get_debt_new_id(&mut self) -> u128 {
            self.debt_position_counter += 1;
            self.debt_position_counter
        }

        pub fn get_new_collateral_id(&mut self) -> u128 {
            self.collateral_position_counter += 1;
            self.collateral_position_counter
        }

        pub fn get_new_cdp_id(&mut self) -> u64 {
            self.cdp_position_counter += 1;
            self.cdp_position_counter
        }

        /* UTILITY PRIVATE METHODS */

        //
        // Get a CDP NFT data and  Hydrate it to produce a HydratedCollaterizedDebtPosition that will be use all over the LendingPoolMnanager
        fn _get_extended_cdp(
            &self,
            cdp_id: NonFungibleLocalId,
        ) -> ExtendedCollaterizedDebtPosition {
            self.lending_market_component_badge.authorize(|| {
                ExtendedCollaterizedDebtPosition::create_extended_cdp(
                    cdp_id,
                    self.cdp_resource_address,
                    self.lending_pool_registry.clone(),
                )
            })
        }

        //
        // Get en reference to the relevent LendingPool component from pool_resource_address
        fn _get_pool(&self, resource_address: ResourceAddress) -> LendingPoolComponentTarget {
            let component_address = self
                .lending_pool_registry
                .get_component_address(resource_address);

            LendingPoolComponentTarget::at(component_address)
        }

        // Validate provided CDP NFT Proof and return it's LocalId
        fn _validate_cdp_proof(&self, cdp: Proof) -> NonFungibleLocalId {
            let validated_cdp = cdp
                .validate_proof(self.cdp_resource_address)
                .expect("Wrong badge provided.");
            validated_cdp.non_fungible_local_id()
        }

        // Get a dry CDP NFT data a store it on leager
        fn _save_cdp(&mut self, mut extended_cdp: ExtendedCollaterizedDebtPosition) {
            // let cdp_id = extended_cdp.clone().cdp_id;
            let cdps = extended_cdp.get_cdps();

            for (cdp_id, cdp_data) in cdps {
                self.lending_market_component_badge.authorize(|| {
                    borrow_resource_manager!(self.cdp_resource_address)
                        .update_non_fungible_data(&cdp_id, cdp_data.clone());
                });

                // !
                self.cdp_data_lookup.insert(cdp_id, cdp_data);
            }
        }
    }
}
