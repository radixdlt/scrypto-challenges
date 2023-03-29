use scrypto::prelude::*;

// TODO functions
// 1. Fund trader's wallet
// 2. Withdraw funds from trader's wallet
// 3. Trade XRD for other tokens
// 4. Trade other tokens for XRD
// 5. Poll price for people's vaults
// 6. Fund TradeX wallet
// 7. Withdraw funds from TradeX wallet
// 8. Add new RadiSwap pool to TradeX

#[derive(NonFungibleData)]
struct TraderBadge {}

// Define the functions on the Radiswap blueprint
external_blueprint! {
  RadiswapPackageTarget {
    fn instantiate_pool(a_tokens: Bucket, b_tokens: Bucket, lp_initial_supply: Decimal, lp_symbol: String, lp_name: String, lp_url: String, fee: Decimal) -> (ComponentAddress, Bucket);
  }
}

// Define the methods on instantiated components
external_component! {
  RadiswapComponentTarget {
    fn add_liquidity(&mut self, a_tokens: Bucket, b_tokens: Bucket) -> (Bucket, Bucket);
    fn remove_liquidity(&mut self, lp_tokens: Bucket) -> (Bucket, Bucket);
    fn swap(&mut self, input_tokens: Bucket) -> Bucket;
    fn get_pair(&self) -> (ResourceAddress, ResourceAddress);
    fn get_pair_sizes(&self) -> (Decimal, Decimal, Decimal);
  }
}

#[blueprint]
mod trade_x {
    // use radix_engine::types::GlobalAddress;

    struct TradeX {
        tradex_vaults: HashMap<ResourceAddress, Vault>,
        approved_radiswap_pools: Vec<ComponentAddress>,
        tradex_wallets: HashMap<NonFungibleLocalId, HashMap<ResourceAddress, Vault>>,
        tradex_lending_balances: HashMap<NonFungibleLocalId, HashMap<ResourceAddress, Decimal>>,
        internal_admin_badge: Vault,
        traders_badge: ResourceAddress,
        commission: Decimal,
        xrd_resource_address: ResourceAddress,
        standard_radiswap_pools:
            HashMap<ResourceAddress, HashMap<ResourceAddress, ComponentAddress>>,
    }

    impl TradeX {
        pub fn instantiate_tradex(
            funds: Vec<Bucket>,
            approved_radiswap_pools: Vec<ComponentAddress>,
            commission: Decimal,
            xrd_resource_address: ResourceAddress,
            standard_radiswap_pools_from: Vec<ResourceAddress>,
            standard_radiswap_pools_to: Vec<ResourceAddress>,
            standard_radiswap_pools_addr: Vec<ComponentAddress>,
        ) -> (ComponentAddress, Bucket) {
            let ownership_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Ownership Badge")
                .metadata(
                    "description",
                    "An ownership badge used to authenticate the creator of TradeX.",
                )
                .metadata("symbol", "OWNER")
                .mint_initial_supply(1);

            // Creating the internal admin badge which will be used to manager the bidder badges
            let internal_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Internal Admin Badge")
                .metadata("description", "A badge used to manage the bidder badges")
                .metadata("symbol", "IADMIN")
                .mint_initial_supply(1);

            // Creating the bidder's badge which will be used to track the bidder's information and bids.
            let trader_badge_resource_address: ResourceAddress =
                ResourceBuilder::new_uuid_non_fungible()
                    .metadata("name", "Trader Badge")
                    .metadata(
                        "description",
                        "A badge provided to trader to access their TradeX wallets",
                    )
                    .metadata("symbol", "TRADER")
                    .mintable(
                        rule!(require(internal_admin_badge.resource_address())),
                        LOCKED,
                    )
                    .burnable(
                        rule!(require(internal_admin_badge.resource_address())),
                        LOCKED,
                    )
                    .updateable_non_fungible_data(
                        rule!(require(internal_admin_badge.resource_address())),
                        LOCKED,
                    )
                    .create_with_no_initial_supply();

            let access_rule: AccessRule = rule!(require(ownership_badge.resource_address()));
            let access_rules: AccessRules = AccessRules::new()
                .method("fund_vault", access_rule.clone(), AccessRule::DenyAll)
                .method(
                    "add_standard_radiswap_pool",
                    access_rule.clone(),
                    AccessRule::DenyAll,
                )
                .method(
                    "add_approved_radiswap_pool",
                    access_rule.clone(),
                    AccessRule::DenyAll,
                )
                .method(
                    "withdraw_from_vault",
                    access_rule.clone(),
                    AccessRule::DenyAll,
                )
                .default(rule!(allow_all), AccessRule::DenyAll);
            let mut tradex_vaults: HashMap<ResourceAddress, Vault> = HashMap::new();
            for bucket in funds {
                tradex_vaults
                    .entry(bucket.resource_address())
                    .or_insert(Vault::new(bucket.resource_address()))
                    .put(bucket)
            }

            let standard_radiswap_pools: HashMap<
                ResourceAddress,
                HashMap<ResourceAddress, ComponentAddress>,
            > = standard_radiswap_pools_from
                .into_iter()
                .zip(standard_radiswap_pools_to.into_iter())
                .zip(standard_radiswap_pools_addr.into_iter())
                .map(|((from, to), addr)| (from, (to, addr)))
                .fold(HashMap::new(), |mut acc, (from, (to, addr))| {
                    acc.entry(from).or_insert(HashMap::new()).insert(to, addr);
                    return acc;
                });

            let mut trade_x: TradeXComponent = Self {
                tradex_vaults,
                approved_radiswap_pools,
                tradex_wallets: HashMap::new(),
                tradex_lending_balances: HashMap::new(),
                internal_admin_badge: Vault::with_bucket(internal_admin_badge),
                traders_badge: trader_badge_resource_address,
                commission,
                xrd_resource_address,
                standard_radiswap_pools,
            }
            .instantiate();
            trade_x.add_access_check(access_rules);
            let trade_x: ComponentAddress = trade_x.globalize();

            (trade_x, ownership_badge)
        }

        pub fn fund_vault(&mut self, funds: Bucket) {
            if self.tradex_vaults.contains_key(&funds.resource_address()) {
                self.tradex_vaults
                    .get_mut(&funds.resource_address())
                    .unwrap()
                    .put(funds);
            } else {
                self.tradex_vaults
                    .insert(funds.resource_address(), Vault::with_bucket(funds));
            }
        }

        pub fn withdraw_from_vault(&mut self, amount: Decimal, address: ResourceAddress) -> Bucket {
            let funds: Bucket = self.tradex_vaults.get_mut(&address).unwrap().take(amount);

            funds
        }

        // ------------------------------Trader functions start here-------------------------------------------------
        pub fn create_and_fund_wallet(&mut self, funds: Bucket) -> Bucket {
            assert!(!funds.is_empty(), "funds is empty");

            let funds_resource_address: ResourceAddress = funds.resource_address();
            let funds_amount: Decimal = funds.amount();
            // check if funds.resource_address() is in tradex_vaults
            assert!(
                self.tradex_vaults.contains_key(&funds_resource_address),
                "funds.resource_address() is not in tradex_vaults"
            );

            assert_eq!(
                funds_resource_address, self.xrd_resource_address,
                "funds.resource_address() is not XRD"
            );

            // check if vault has enough funds
            assert!(
                self.tradex_vaults
                    .get(&funds_resource_address)
                    .unwrap()
                    .amount()
                    >= funds_amount,
                "tradex_vaults does not have enough funds"
            );

            let traders_badge: Bucket = self.internal_admin_badge.authorize(|| {
                let traders_resource_manager: &mut ResourceManager =
                    borrow_resource_manager!(self.traders_badge);
                traders_resource_manager.mint_uuid_non_fungible(TraderBadge {})
            });

            self.tradex_wallets
                .insert(traders_badge.non_fungible_local_id(), HashMap::new());
            self.tradex_lending_balances
                .insert(traders_badge.non_fungible_local_id(), HashMap::new());

            self.tradex_wallets
                .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .insert(funds_resource_address, Vault::with_bucket(funds));

            // Take equal amount of funds from the vault and put it in the wallet
            let margin_funds: Bucket = self
                .tradex_vaults
                .get_mut(&funds_resource_address)
                .unwrap()
                .take(funds_amount);

            self.tradex_wallets
                .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .get_mut(&funds_resource_address)
                .unwrap()
                .put(margin_funds);

            self.tradex_lending_balances
                .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .insert(funds_resource_address, funds_amount);

            // Returning the bidder's badge back to the caller
            traders_badge
        }

        pub fn fund_existing_wallet(&mut self, funds: Bucket, traders_badge: Proof) {
            let funds_resource_address: ResourceAddress = funds.resource_address();
            let funds_amount: Decimal = funds.amount();
            assert!(
                self.tradex_vaults.contains_key(&funds_resource_address),
                "[Fund Existing Wallet]: funds.resource_address() is not in tradex_vaults"
            );

            assert_eq!(
                funds_resource_address, self.xrd_resource_address,
                "[Fund Existing Wallet]: Only XRD can be used to fund the wallet"
            );

            // check if vault has enough funds
            assert!(
                self.tradex_vaults
                    .get(&funds_resource_address)
                    .unwrap()
                    .amount()
                    >= funds.amount(),
                "tradex_vaults does not have enough funds"
            );

            let traders_badge: ValidatedProof = traders_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.traders_badge,
                    dec!("1"),
                ))
                .expect("[Fund Existing Wallet]: Invalid proof provided");

            assert_eq!(
                traders_badge.resource_address(),
                self.traders_badge,
                "[Fund Existing Wallet]: Badge provided is not a valid trader's badge"
            );
            assert_eq!(
                    traders_badge.amount(), Decimal::one(),
                    "[Fund Existing Wallet]: This method requires that exactly one trader's badge is passed to the method"
                );

            if self
                .tradex_wallets
                .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .contains_key(&funds.resource_address())
            {
                self.tradex_wallets
                    .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                    .unwrap()
                    .get_mut(&funds.resource_address())
                    .unwrap()
                    .put(funds);

                let current_lent_amount: Decimal = self
                    .tradex_lending_balances
                    .get(&traders_badge.non_fungible::<TraderBadge>().local_id())
                    .unwrap()
                    .get(&funds_resource_address)
                    .unwrap()
                    .clone();

                self.tradex_lending_balances
                    .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                    .unwrap()
                    .insert(funds_resource_address, current_lent_amount + funds_amount);
            } else {
                self.tradex_wallets
                    .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                    .unwrap()
                    .insert(funds.resource_address(), Vault::with_bucket(funds));

                self.tradex_lending_balances
                    .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                    .unwrap()
                    .insert(funds_resource_address, funds_amount);
            }

            // Take equal amount of funds from the vault and put it in the wallet
            let margin_funds: Bucket = self
                .tradex_vaults
                .get_mut(&funds_resource_address)
                .unwrap()
                .take(funds_amount);

            self.tradex_wallets
                .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .get_mut(&funds_resource_address)
                .unwrap()
                .put(margin_funds);
        }

        pub fn trade(
            &mut self,
            radiswap_pool_address: ComponentAddress,
            amount_to_trade_out: Decimal,
            funds_resource_address: ResourceAddress,
            traders_badge: Proof,
        ) {
            self.poll_all_traders_health();
            let traders_badge: ValidatedProof = traders_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.traders_badge,
                    dec!("1"),
                ))
                .expect("[Withdraw Payment]: Invalid proof provided");

            assert_eq!(
                traders_badge.resource_address(),
                self.traders_badge,
                "[Withdraw Payment]: Badge provided is not a valid trader's badge"
            );
            assert_eq!(
                    traders_badge.amount(), Decimal::one(),
                    "[Withdraw Payment]: This method requires that exactly one trader's badge is passed to the method"
                );

            self._trade(
                radiswap_pool_address,
                amount_to_trade_out,
                funds_resource_address,
                traders_badge.non_fungible::<TraderBadge>().local_id(),
            );
        }

        pub fn withdraw_payment(
            &mut self,
            amount: Decimal,
            address: ResourceAddress,
            traders_badge: Proof,
        ) -> Bucket {
            self.poll_all_traders_health();
            let traders_badge: ValidatedProof = traders_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.traders_badge,
                    dec!("1"),
                ))
                .expect("[Withdraw Payment]: Invalid proof provided");

            assert_eq!(
                traders_badge.resource_address(),
                self.traders_badge,
                "[Withdraw Payment]: Badge provided is not a valid trader's badge"
            );
            assert_eq!(
                    traders_badge.amount(), Decimal::one(),
                    "[Withdraw Payment]: This method requires that exactly one trader's badge is passed to the method"
                );

            assert!(
                self.tradex_wallets
                    .get(&traders_badge.non_fungible::<TraderBadge>().local_id())
                    .unwrap()
                    .contains_key(&address),
                "[Withdraw Payment]: address is not in tradex_wallets"
            );

            // check if wallet has enough funds
            assert!(
                self.tradex_wallets
                    .get(&traders_badge.non_fungible::<TraderBadge>().local_id())
                    .unwrap()
                    .get(&address)
                    .unwrap()
                    .amount()
                    >= (Decimal::from(2) + self.commission) * amount,
                "tradex_wallets does not have enough funds"
            );

            // Take equal amount of funds from the wallet and put it in the vault
            let margin_funds: Bucket = self
                .tradex_wallets
                .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .get_mut(&address)
                .unwrap()
                .take(amount * (Decimal::from(1) + self.commission));

            self.tradex_vaults
                .get_mut(&address)
                .unwrap()
                .put(margin_funds);

            let current_lent_amount: Decimal = self
                .tradex_lending_balances
                .get(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .get(&address)
                .unwrap()
                .clone();

            self.tradex_lending_balances
                .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .insert(
                    address,
                    current_lent_amount - (amount * (Decimal::from(1) + self.commission)),
                );

            let returned_funds: Bucket = self
                .tradex_wallets
                .get_mut(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .get_mut(&address)
                .unwrap()
                .take(amount);

            return returned_funds;
        }

        pub fn check_wallets(&self, traders_badge: Proof) -> (Vec<ResourceAddress>, Vec<Decimal>) {
            let traders_badge: ValidatedProof = traders_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.traders_badge,
                    dec!("1"),
                ))
                .expect("[Withdraw Payment]: Invalid proof provided");

            assert_eq!(
                traders_badge.resource_address(),
                self.traders_badge,
                "[Withdraw Payment]: Badge provided is not a valid trader's badge"
            );
            assert_eq!(
                    traders_badge.amount(), Decimal::one(),
                    "[Withdraw Payment]: This method requires that exactly one trader's badge is passed to the method"
                );

            let mut resource_ids: Vec<ResourceAddress> = Vec::new();
            let mut resource_amounts: Vec<Decimal> = Vec::new();

            for (resource_address, bucket) in self
                .tradex_wallets
                .get(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .iter()
            {
                resource_ids.push(resource_address.clone());
                resource_amounts.push(bucket.amount());
            }

            (resource_ids, resource_amounts)
        }

        pub fn show_lending_balance(
            &self,
            traders_badge: Proof,
        ) -> (Vec<ResourceAddress>, Vec<Decimal>) {
            let traders_badge: ValidatedProof = traders_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.traders_badge,
                    dec!("1"),
                ))
                .expect("[Withdraw Payment]: Invalid proof provided");

            assert_eq!(
                traders_badge.resource_address(),
                self.traders_badge,
                "[Withdraw Payment]: Badge provided is not a valid trader's badge"
            );
            assert_eq!(
                    traders_badge.amount(), Decimal::one(),
                    "[Withdraw Payment]: This method requires that exactly one trader's badge is passed to the method"
                );

            let mut resource_ids: Vec<ResourceAddress> = Vec::new();
            let mut resource_amounts: Vec<Decimal> = Vec::new();

            for (resource_address, value) in self
                .tradex_lending_balances
                .get(&traders_badge.non_fungible::<TraderBadge>().local_id())
                .unwrap()
                .iter()
            {
                resource_ids.push(resource_address.clone());
                resource_amounts.push(*value);
            }

            (resource_ids, resource_amounts)
        }
        // -----------------------------------------------------------------------------------------------------------
        pub fn add_standard_radiswap_pool(
            &mut self,
            pool_address: ComponentAddress,
            resource_address_a: ResourceAddress,
            resource_address_b: ResourceAddress,
        ) {
            if !self
                .standard_radiswap_pools
                .contains_key(&resource_address_a)
            {
                self.standard_radiswap_pools
                    .insert(resource_address_a, HashMap::new());
            }

            self.standard_radiswap_pools
                .get_mut(&resource_address_a)
                .unwrap()
                .insert(resource_address_b, pool_address);

            if !self
                .standard_radiswap_pools
                .contains_key(&resource_address_b)
            {
                self.standard_radiswap_pools
                    .insert(resource_address_b, HashMap::new());
            }

            self.standard_radiswap_pools
                .get_mut(&resource_address_b)
                .unwrap()
                .insert(resource_address_a, pool_address);
        }

        pub fn show_standard_radiswap_pools(
            &self,
            a: ResourceAddress,
            b: ResourceAddress,
        ) -> ComponentAddress {
            self.standard_radiswap_pools
                .get(&a)
                .unwrap()
                .get(&b)
                .unwrap()
                .clone()
        }

        pub fn add_approved_radiswap_pool(&mut self, pool_address: ComponentAddress) {
            self.approved_radiswap_pools.push(pool_address);
        }

        fn _trade(
            &mut self,
            radiswap_pool_address: ComponentAddress,
            amount_to_trade_out: Decimal,
            funds_resource_address: ResourceAddress,
            trader_id: &NonFungibleLocalId,
        ) {
            assert!(
                self.approved_radiswap_pools
                    .contains(&radiswap_pool_address),
                "[Trade Out XRD]: Radiswap pool is not approved"
            );

            assert!(
                self.tradex_wallets
                    .get(&trader_id)
                    .unwrap()
                    .contains_key(&funds_resource_address),
                "[Trade Out XRD]: funds_resource_address is not in tradex_wallets"
            );

            assert!(
                self.tradex_wallets
                    .get(&trader_id)
                    .unwrap()
                    .get(&funds_resource_address)
                    .unwrap()
                    .amount()
                    >= amount_to_trade_out,
                "tradex_wallets does not have enough funds"
            );

            let input_tokens: Bucket = self
                .tradex_wallets
                .get_mut(&trader_id)
                .unwrap()
                .get_mut(&funds_resource_address)
                .unwrap()
                .take(amount_to_trade_out);

            let output_tokens: Bucket =
                RadiswapComponentTarget::at(radiswap_pool_address).swap(input_tokens);

            if self
                .tradex_wallets
                .get(&trader_id)
                .unwrap()
                .contains_key(&output_tokens.resource_address())
            {
                self.tradex_wallets
                    .get_mut(&trader_id)
                    .unwrap()
                    .get_mut(&output_tokens.resource_address())
                    .unwrap()
                    .put(output_tokens);
            } else {
                self.tradex_wallets.get_mut(&trader_id).unwrap().insert(
                    output_tokens.resource_address(),
                    Vault::with_bucket(output_tokens),
                );
            }
        }

        pub fn poll_all_traders_health(&mut self) {
            let mut trader_ids: Vec<NonFungibleLocalId> = Vec::new();

            for (trader_id, _) in self.tradex_wallets.iter() {
                trader_ids.push((*trader_id).clone());
            }
            for trader_id in trader_ids {
                self.poll_trader_health(trader_id);
            }
        }

        pub fn poll_trader_health(&mut self, trader_id: NonFungibleLocalId) {
            let mut current_xrd_balance: Decimal = Decimal::zero();
            // iterate over each tradex wallet and check if the trader has enough funds to cover the margin
            for (_resource_address, vault) in self.tradex_wallets.get(&trader_id).unwrap().iter() {
                current_xrd_balance += self
                    ._get_tradex_wallet_value_in_xrd(&vault.resource_address(), &vault.amount());
            }

            let current_lent_amount: Decimal = self
                .tradex_lending_balances
                .get(&trader_id)
                .unwrap()
                .get(&self.xrd_resource_address)
                .unwrap()
                .clone();

            if current_lent_amount > Decimal::zero()
                && ((Decimal::from("1") + self.commission) * current_lent_amount)
                    >= current_xrd_balance
            {
                // if the trader does not have enough funds to cover the margin, then we need to liquidate the trader
                self.liquidate_trader(trader_id);
            }
        }

        fn liquidate_trader(&mut self, trader_id: NonFungibleLocalId) {
            let mut vaults: Vec<(ResourceAddress, Decimal)> = Vec::new();

            for (_resource_address, vault) in self.tradex_wallets.get(&trader_id).unwrap().iter() {
                vaults.push((vault.resource_address(), vault.amount()));
            }

            for (resource_address, amount) in vaults {
                let input_tokens_resource_address: ResourceAddress = resource_address;
                if input_tokens_resource_address == self.xrd_resource_address {
                    continue;
                }
                assert!(
                    self.standard_radiswap_pools
                        .contains_key(&self.xrd_resource_address)
                        && self
                            .standard_radiswap_pools
                            .get(&self.xrd_resource_address)
                            .unwrap()
                            .contains_key(&input_tokens_resource_address)
                        || self
                            .standard_radiswap_pools
                            .contains_key(&input_tokens_resource_address)
                            && self
                                .standard_radiswap_pools
                                .get(&input_tokens_resource_address)
                                .unwrap()
                                .contains_key(&self.xrd_resource_address),
                    "[Trade Out XRD]: No standard radiswap pool for this asset"
                );

                let mut pool_address: ComponentAddress;
                if self
                    .standard_radiswap_pools
                    .contains_key(&self.xrd_resource_address)
                    && self
                        .standard_radiswap_pools
                        .get(&self.xrd_resource_address)
                        .unwrap()
                        .contains_key(&input_tokens_resource_address)
                {
                    pool_address = self
                        .standard_radiswap_pools
                        .get(&self.xrd_resource_address)
                        .unwrap()
                        .get(&input_tokens_resource_address)
                        .unwrap()
                        .clone();
                } else {
                    pool_address = self
                        .standard_radiswap_pools
                        .get(&input_tokens_resource_address)
                        .unwrap()
                        .get(&self.xrd_resource_address)
                        .unwrap()
                        .clone();
                }

                self._trade(pool_address, amount, resource_address, &trader_id);
            }

            let xrd_balance: Decimal = self
                .tradex_wallets
                .get(&trader_id)
                .unwrap()
                .get(&self.xrd_resource_address)
                .unwrap()
                .amount();

            self.tradex_lending_balances
                .get_mut(&trader_id)
                .unwrap()
                .insert(self.xrd_resource_address.clone(), Decimal::zero());

            let liquidated_funds: Bucket = self
                .tradex_wallets
                .get_mut(&trader_id)
                .unwrap()
                .get_mut(&self.xrd_resource_address)
                .unwrap()
                .take(xrd_balance);

            self.tradex_vaults
                .get_mut(&self.xrd_resource_address)
                .unwrap()
                .put(liquidated_funds);
        }

        pub fn get_tradex_wallet_value_in_xrd(
            &self,
            input_tokens_resource_address: ResourceAddress,
            input_tokens_amount: Decimal,
        ) -> Decimal {
            self._get_tradex_wallet_value_in_xrd(
                &input_tokens_resource_address,
                &input_tokens_amount,
            )
        }

        fn _get_tradex_wallet_value_in_xrd(
            &self,
            input_tokens_resource_address: &ResourceAddress,
            input_tokens_amount: &Decimal,
        ) -> Decimal {
            if *input_tokens_resource_address == self.xrd_resource_address {
                return *input_tokens_amount;
            }
            assert!(
                self.standard_radiswap_pools
                    .contains_key(&self.xrd_resource_address)
                    && self
                        .standard_radiswap_pools
                        .get(&self.xrd_resource_address)
                        .unwrap()
                        .contains_key(input_tokens_resource_address)
                    || self
                        .standard_radiswap_pools
                        .contains_key(input_tokens_resource_address)
                        && self
                            .standard_radiswap_pools
                            .get(input_tokens_resource_address)
                            .unwrap()
                            .contains_key(&self.xrd_resource_address),
                "[Trade Out XRD]: No standard radiswap pool for this asset"
            );

            let mut pool_address: ComponentAddress;
            if self
                .standard_radiswap_pools
                .contains_key(&self.xrd_resource_address)
                && self
                    .standard_radiswap_pools
                    .get(&self.xrd_resource_address)
                    .unwrap()
                    .contains_key(input_tokens_resource_address)
            {
                pool_address = self
                    .standard_radiswap_pools
                    .get(&self.xrd_resource_address)
                    .unwrap()
                    .get(input_tokens_resource_address)
                    .unwrap()
                    .clone();
            } else {
                pool_address = self
                    .standard_radiswap_pools
                    .get(input_tokens_resource_address)
                    .unwrap()
                    .get(&self.xrd_resource_address)
                    .unwrap()
                    .clone();
            }

            let (a_pool_amount, b_pool_amount, fee) =
                RadiswapComponentTarget::at(pool_address).get_pair_sizes();

            let (a_pool_resource_address, _b_pool_resource_address) =
                RadiswapComponentTarget::at(pool_address).get_pair();

            let fee_amount = (*input_tokens_amount) * fee;

            let output_tokens_amount = if *input_tokens_resource_address == a_pool_resource_address
            {
                // Calculate how much of token B we will return
                let b_amount = b_pool_amount
                    - a_pool_amount * b_pool_amount
                        / (*input_tokens_amount - fee_amount + a_pool_amount);

                b_amount
            } else {
                // Calculate how much of token A we will return
                let a_amount = a_pool_amount
                    - a_pool_amount * b_pool_amount
                        / (*input_tokens_amount - fee_amount + b_pool_amount);

                a_amount
            };

            output_tokens_amount
        }
    }
}
