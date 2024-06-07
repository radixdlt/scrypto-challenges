mod constants;
mod events;
mod icon_urls;

use crate::constants::*;
use crate::events::*;
use crate::icon_urls::*;

use scrypto::prelude::*;
use scrypto_avltree::AvlTree;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct OwnerBadgeData {
    name: String,
}

//region decimal Converters trait

pub trait Converters {
    fn to_i64(&self) -> i64;

    fn to_u64(&self) -> u64;

    fn round_down_to_u64(&self) -> u64;

    fn round_down_no_digits(&self) -> Decimal;
}

impl Converters for Decimal {
    fn to_i64(&self) -> i64 {
        let full_integer: I192 = self.0 / Decimal::ONE.0;
        full_integer.to_i64().expect("Overflow: value too large")
    }

    fn to_u64(&self) -> u64 {
        let full_integer: I192 = self.0 / Decimal::ONE.0;
        full_integer.to_u64().expect("Overflow: value too large")
    }

    fn round_down_to_u64(&self) -> u64 {
        let rounded: Decimal = self.checked_round(0, RoundingMode::ToZero).unwrap();

        let full_integer: I192 = rounded.0 / Decimal::ONE.0;
        full_integer.to_u64().expect("Overflow: value too large")
    }

    fn round_down_no_digits(&self) -> Decimal {
        self.checked_round(0, RoundingMode::ToZero)
            .expect("Rounding to nearest integer failed")
    }
}

//endregion

//region YieldClaim struct and impl

/// Defines the detailed claim data stored within each Super Yield NFT.
#[derive(NonFungibleData, ScryptoSbor, Clone, Copy, PartialEq, Eq)]
pub struct YieldClaim {
    /// The hour relative to the start of the token sale when the NFT was minted.
    pub hour_of_mint: u64,
    /// Indicates the amount of SUPER tokens that were minted with this NFT.
    pub n_super_minted: u64,
    /// Indicates the amount of SUPERt tokens that were minted with this NFT, these tokens represent a share of XRD stored within the component's Native Pool.
    pub n_trust_minted: Decimal,
}

impl fmt::Display for YieldClaim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "YieldClaim {{hour_of_mint: {}, n_super_minted: {}, n_trust_minted: {} }}",
            self.hour_of_mint, self.n_super_minted, self.n_trust_minted
        )
    }
}

//endregion

#[blueprint]
#[events(
    SaleDetailEvent,
    CreateYieldNFTEvent,
    BurnYieldNFTEvent,
    WithdrawalCalculationEvent,
    SplitNFTEvent,
    ClaimYieldEvent,
    ShowSuperMintedEvent,
    YieldUpdateEvent
)]

mod super_iyo {
    enable_method_auth! {
        roles {
            db_updater => updatable_by: [OWNER];
        },
        methods {

            //Starting Token Sale
            start_sale => restrict_to: [SELF, OWNER];
            calculate_withdrawal_epochs => restrict_to: [SELF, OWNER];

            //Metadata Updaters
            set_trustfund_metadata => restrict_to: [SELF, OWNER];
            update_super_metadata => restrict_to: [SELF, OWNER];
            update_trust_metadata => restrict_to: [SELF, OWNER];
            update_yield_metadata => restrict_to: [SELF, OWNER];
            update_nft_manager_metadata => restrict_to: [SELF, OWNER];

            //Ending Token Sale
            end_sale => restrict_to: [SELF, OWNER];
            check_if_sale_complete => restrict_to: [SELF, OWNER];

            //Buying Functions
            deposit => PUBLIC;
            create_yield_nft => restrict_to: [SELF, OWNER];
            get_checked_nft_id => restrict_to: [SELF, OWNER];
            add_receipt_to_db => restrict_to: [SELF, OWNER];

            //Things to do w/ yield tokens
            split_yield_nft => PUBLIC;

            //Claiming Yield
            claim_yield => PUBLIC;
            check_and_burn_nft => restrict_to: [SELF, OWNER];

            //Vested withdrawal functions
            vested_withdraw => restrict_to: [OWNER];

            //AVL Tree Updaters
            update_dbs_with => restrict_to: [SELF, OWNER, db_updater];
            update_dbs_to_now => restrict_to: [SELF, OWNER, db_updater];
            update_hourly_super_minted => restrict_to: [SELF, OWNER];
            update_yield_generated => restrict_to: [SELF, OWNER, db_updater];

            //AVL Tree Viewers
            show_hourly_yield_generated => PUBLIC;
            show_hourly_super_minted => PUBLIC;

            //Helpers
            calculate_yield_curve_for_hour => restrict_to: [SELF, OWNER];
            divide_decimal_into_n_weighted_decimals => restrict_to: [SELF, OWNER];
            divide_integer_into_n_integers => restrict_to: [SELF, OWNER];
            find_positive_non_zero_multiple_of_10 => restrict_to: [SELF, OWNER];
            hours_since_start => restrict_to: [SELF, OWNER];
            //check_update_checklist => restrict_to: [SELF, OWNER];
            nft_local_id_to_u64 => restrict_to: [SELF, OWNER];
        }
    }

    struct Super {
        icons: Icons,
        active_colors: IconUrls,

        // Owner Vesting Vault
        vesting_vault: Vault,
        vested_withdrawal_amount: Decimal,
        unclaimed_super_yield: Decimal,

        pool: Global<OneResourcePool>,

        // ResourceManagers
        super_manager: ResourceManager,
        super_yield_manager: ResourceManager,
        yield_nft_manager: ResourceManager,

        // AVLTree
        yield_nft_db: AvlTree<u64, YieldClaim>, // <nft_local_addy, nft_data>
        yield_generated_db: AvlTree<u64, Decimal>, // <nft_local_addy, yield_generated_so_far>
        vested_withdrawals_db: AvlTree<u64, bool>, // <hour_of_withdrawal, withdrawal_used>
        hourly_super_minted: AvlTree<u64, u64>, // <hour, total_n_super>
        hour_updated_checklist: AvlTree<u64, bool>,

        // Token Sale Status flags
        time_sale_start: Instant,
        time_sale_end: Instant,
        dbs_updated_up_to_before_hour: u64,
        sale_details: SaleDetailEvent,
    }

    impl Super {
        /// Initializes and returns a new instance of the Super component.
        /// This function sets up the initial state, including badges, resource managers, and
        /// databases required for the component's operation.
        ///
        /// # Arguments
        /// * `dapp_definition_addy` - DApp definition address for the DApp.
        ///
        /// # Returns
        /// A tuple containing:
        /// * A `NonFungibleBucket` for the owner badge.
        /// * A `Global<Super>` instance representing the newly created Super component.
        /// * A `NonFungibleBucket` for the database updater badge.
        pub fn new(
            dapp_definition_addy: ComponentAddress,
        ) -> (NonFungibleBucket, Global<Super>, NonFungibleBucket) {

            let dapp_definition_addy_vec: Vec<GlobalAddress> =
                vec![GlobalAddress::new_or_panic(dapp_definition_addy.into())];

            let logos: Icons = Icons::new();

            let current_colors: IconUrls = logos.black.clone();

            //region Component Rules and address reservations

            let (addy_reservation, component_addy): (GlobalAddressReservation, ComponentAddress) =
                Runtime::allocate_component_address(Super::blueprint_id());

            let access_rule_component: AccessRule = rule!(require(global_caller(component_addy)));

            let global_component_caller_badge: NonFungibleGlobalId =
                NonFungibleGlobalId::global_caller_badge(component_addy);

            let global_component_addy: ResourceAddress =
                global_component_caller_badge.resource_address();

            let owner_role_component: OwnerRole = OwnerRole::Fixed(access_rule_component.clone());

            //endregion

            //region Minting Owner badge and owner access rules

            let badge_owner: NonFungibleBucket =
                ResourceBuilder::new_integer_non_fungible(owner_role_component.clone())
                    .metadata(metadata!(
                    init {
                        "name" => "SUPER Owner Badge".to_owned(), locked;
                        "key_image_url" => current_colors.nft.to_owned(), updatable;
                        "dapp_definitions" => dapp_definition_addy_vec.to_owned(), updatable;
                    }))
                    .mint_roles(mint_roles!(
                    minter => access_rule_component.clone();
                    minter_updater => access_rule_component.clone();
                    ))
                    .mint_initial_supply(vec![(
                        0u64.into(),
                        OwnerBadgeData {
                            name: "SHAH".to_owned(),
                        },
                    )]);

            let badge_owner_addy: ResourceAddress = badge_owner.resource_address();

            let owner_badge_global_id: NonFungibleGlobalId =
                NonFungibleGlobalId::global_caller_badge(badge_owner_addy);

            let access_component_or_owner_badge: AccessRule = rule!(require_any_of(vec![
                owner_badge_global_id.clone(),
                global_component_caller_badge.clone()
            ]));
            let owner_component_or_owner_badge: OwnerRole =
                OwnerRole::Fixed(access_component_or_owner_badge.clone());

            //endregion

            //region Creating a AVLTree that contains all epochs where vested funds will partially unlock

            let withdrawal_epochs: AvlTree<u64, bool> = AvlTree::new();

            //endregion

            //region Creating One Resource Pool (The lump sum will be stored here)

            let pool_one_resource: Global<OneResourcePool> =
                Blueprint::<OneResourcePool>::instantiate(
                    owner_component_or_owner_badge.clone(),
                    rule!(require(global_component_caller_badge.clone())),
                    XRD,
                    None,
                );

            let pool_addy: ComponentAddress = pool_one_resource.address();

            //Note:
            // The Resource Address for the one resource pool's LP token is inaccessible until this component is globalized.
            // With this in mind, the instantiated value for the trust_token_addy is incorrect.
            // It is corrected in the function update_trust_metadata(), which is called from start_sale()

            //endregion

            //region Creating Manager for SUPER (principal) and SUPERy (yield)

            let super_manager: ResourceManager =
                ResourceBuilder::new_fungible(owner_component_or_owner_badge.clone())
                    .metadata(metadata! {
                        roles {
                            metadata_locker => access_rule_component.clone();
                            metadata_locker_updater => access_rule_component.clone();
                            metadata_setter => access_rule_component.clone();
                            metadata_setter_updater => access_rule_component.clone();
                        },
                        init {
                            "name" => "SUPER".to_owned(), locked;
                            "symbol" => "SUPER".to_owned(), locked;
                            "icon_url" => current_colors.super_s.to_owned(), updatable;
                            "dapp_definitions" => dapp_definition_addy_vec.to_owned(), updatable;
                        }
                    })
                    .mint_roles(mint_roles!(
                        minter => rule!(require(global_caller(component_addy)));
                        minter_updater => rule!(require(global_caller(component_addy)));
                    ))
                    .divisibility(0)
                    .burn_roles(burn_roles!(
                    burner => rule!(require(global_caller(component_addy)));
                    burner_updater => rule!(require(global_caller(component_addy)));
                    ))
                    .create_with_no_initial_supply();

            let super_empty_bucket: Bucket = super_manager.create_empty_bucket();
            let super_resource_addy: ResourceAddress = super_empty_bucket.resource_address();
            super_empty_bucket.drop_empty();

            let super_yield_manager: ResourceManager = ResourceBuilder::new_fungible(
                owner_component_or_owner_badge.clone(),
            )
                .metadata(metadata! {
                roles {
                    metadata_locker => access_rule_component.clone();
                    metadata_locker_updater => access_rule_component.clone();
                    metadata_setter => access_rule_component.clone();
                    metadata_setter_updater => access_rule_component.clone();
                },
                init {
                        "name" => "SUPER Yield Token".to_owned(), locked;
                        "symbol" => "SUPERy".to_owned(), locked;
                        "icon_url" => current_colors.super_y.to_owned(), updatable;
                        "dapp_definitions" => dapp_definition_addy_vec.to_owned(), updatable;
                    }
            })
                .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_addy)));
                minter_updater => rule!(require(global_caller(component_addy)));
            ))
                .divisibility(DIVISIBILITY_MAXIMUM)
                .burn_roles(burn_roles!(
            burner => rule!(require(global_caller(component_addy)));
            burner_updater => rule!(require(global_caller(component_addy)));
            ))
                .create_with_no_initial_supply();

            let super_yield_empty_bucket: Bucket = super_yield_manager.create_empty_bucket();
            let super_yield_resource_addy: ResourceAddress =
                super_yield_empty_bucket.resource_address();
            super_yield_empty_bucket.drop_empty();

            //endregion

            //region Creating Yield nft Manager

            let nft_manager: ResourceManager = ResourceBuilder::new_integer_non_fungible::<
                YieldClaim,
            >(owner_component_or_owner_badge.clone())
                .metadata(metadata!(
                roles {
                    metadata_setter => access_rule_component.clone();
                    metadata_setter_updater => access_rule_component.clone();
                    metadata_locker => access_rule_component.clone();
                    metadata_locker_updater => access_rule_component.clone();
                },
                init {
                    "name" => "SUPER Yield NFT".to_owned(), locked;
                    "Current Hour" => 0u64.to_owned(), updatable;
                    "Current Time" => 0u64.to_owned(), updatable;
                    "Total Amount" => dec!("0.0").to_owned(), updatable;
                    "dapp_definitions" => dapp_definition_addy_vec.to_owned(), updatable;
                    "icon_url" => current_colors.nft.to_owned(), updatable;
                    "key_image_url" => current_colors.nft.to_owned(), updatable;
                }
            ))
                .mint_roles(mint_roles!(
                minter => access_rule_component.clone();
                minter_updater => access_rule_component.clone();
            ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                non_fungible_data_updater => access_rule_component.clone();
                non_fungible_data_updater_updater => rule!(deny_all);
            ))
                .burn_roles(burn_roles!(
                burner => access_rule_component.clone();
                burner_updater => rule!(deny_all);
            ))
                .create_with_no_initial_supply();

            let nft_empty_bucket: Bucket = nft_manager.create_empty_bucket();
            let nft_resource_addy: ResourceAddress = nft_empty_bucket.resource_address();
            nft_empty_bucket.drop_empty();

            //endregion

            //region Creating Yield NFT db and db_updater badge, that will be sent to off-ledger automated wallet who will activate it once per hour

            let badge_db_updater: NonFungibleBucket =
                ResourceBuilder::new_integer_non_fungible(owner_role_component.clone())
                    .metadata(metadata!(
                        init {
                            "name" => "SUPER Updater Badge".to_owned(), locked;
                            "key_image_url" => current_colors.nft.to_owned(), updatable;
                            "dapp_definitions" => dapp_definition_addy_vec.to_owned(), updatable;
                        }))
                    .mint_roles(mint_roles!(
                        minter => access_rule_component.clone();
                        minter_updater => access_rule_component.clone();
                    ))
                    .mint_initial_supply(vec![(
                        0u64.into(),
                        OwnerBadgeData {
                            name: "Updater".to_owned(),
                        },
                    )]);

            let db_updater_resource_addy: ResourceAddress = badge_db_updater.resource_address();

            //endregion

            let new_super_event: SaleDetailEvent = SaleDetailEvent {
                dapp_definition_caddy: dapp_definition_addy_vec.to_owned(),

                component_caddy: component_addy,
                pool_caddy: pool_addy,

                owner_badge_raddy: badge_owner_addy,
                component_badge_raddy: global_component_addy,
                db_updater_raddy: db_updater_resource_addy,

                super_raddy: super_resource_addy,
                super_y_raddy: super_yield_resource_addy,
                super_t_raddy: super_yield_resource_addy,
                yield_nft_raddy: nft_resource_addy,

                sale_started: false,
                sale_completed: false,

                sale_start_time_unix: 0,
                sale_start_time_utc: "Sale hasn't begun".to_owned(),

                sale_end_time_unix: 0,
                sale_end_time_utc: "Sale hasn't begun".to_owned(),
            };

            let component: Global<Super> = Self {
                //Icon stuff
                icons: logos.clone(),
                active_colors: current_colors.clone(),

                //Components and (c)omp Addresses
                pool: pool_one_resource,

                //vesting vault and relevant shit
                vesting_vault: Vault::new(XRD),
                vested_withdrawal_amount: dec!("0"),
                unclaimed_super_yield: dec!("0"),

                //Token Managers
                super_manager,
                super_yield_manager,
                yield_nft_manager: nft_manager,

                //Databases (db)
                yield_nft_db: AvlTree::new(),
                yield_generated_db: AvlTree::new(),
                vested_withdrawals_db: withdrawal_epochs,
                hourly_super_minted: AvlTree::new(),
                hour_updated_checklist: AvlTree::new(),

                // Token Sale Timings and Status
                time_sale_start: Instant::new(0),
                time_sale_end: Instant::new(0),
                dbs_updated_up_to_before_hour: 0,
                sale_details: new_super_event.to_owned(),
            }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Fixed(rule!(require(badge_owner_addy))))
                .roles(roles! {
                db_updater => rule!(require(db_updater_resource_addy));
            })
                .with_address(addy_reservation)
                .metadata(metadata!(
                init {
                    "name" => "SUPER_IYO", updatable;
                    "dapp_definition" => dapp_definition_addy_vec.to_owned(), updatable;
                    "icon_url" => current_colors.ww.clone().to_owned(), updatable;
                }
            ))
                .globalize();

            Runtime::emit_event(new_super_event.to_owned());

            (badge_owner, component, badge_db_updater)
        }

        //region Starting Token Sale

        /// Initiates the token sale and configures the necessary parameters and schedules.
        ///
        /// This function marks the start of the token sale by setting up the trust fund metadata with the provided fee, initializing the start and end times of the sale, and preparing the withdrawal epochs based on the sale's duration.
        ///
        /// # Arguments
        /// * `fee` - A bucket containing 1 XRD, for setting up the trust fund. This fee will be handled by `set_trustfund_metadata`.
        ///
        /// # Returns
        /// A bucket of the fee passed in.
        ///
        pub fn start_sale(&mut self, fee: Bucket) -> Bucket {
            let return_fee: Bucket = self.set_trustfund_metadata(fee);

            self.time_sale_start = Clock::current_time_rounded_to_seconds();

            self.time_sale_end = Clock::current_time_rounded_to_seconds()
                .add_days(SALE_DURATION_DAYS as i64)
                .unwrap();

            self.calculate_withdrawal_epochs();

            info!(
                "Sale started on: {}",
                self.time_sale_start.seconds_since_unix_epoch
            );

            info!(
                "Sale ends on: {}",
                self.time_sale_end.seconds_since_unix_epoch
            );

            self.super_manager.set_mintable(AccessRule::AllowAll);

            //Updating and emitting self.sale_details
            self.sale_details.sale_started = true;
            self.sale_details.sale_completed = false;

            self.sale_details.sale_start_time_unix = self.time_sale_start.seconds_since_unix_epoch;
            self.sale_details.sale_start_time_utc =
                UtcDateTime::from_instant(&self.time_sale_start)
                    .unwrap()
                    .to_string();

            self.sale_details.sale_end_time_unix = self.time_sale_end.seconds_since_unix_epoch;
            self.sale_details.sale_end_time_utc = UtcDateTime::from_instant(&self.time_sale_end)
                .unwrap()
                .to_string();

            Runtime::emit_event(self.sale_details.to_owned());

            return_fee
        }

        /// Calculates and schedules the withdrawal epochs based on the token sale start time.
        ///
        /// This function iterates through the specified number of vesting weeks and calculates the specific epochs (in Unix time)
        /// at which vested funds become available for withdrawal. Each epoch is calculated by adding a multiple of the vesting period
        /// in days to the sale start time. The result is stored in the vested withdrawals db with an initial state set to `false`,
        /// indicating that the funds for that epoch are not yet withdrawn.
        pub fn calculate_withdrawal_epochs(&mut self) {
            let mut withdrawal_epoch_vector: Vec<String> = Vec::new();

            for i in 0..WEEKS_VESTED {
                let epoch: u64 = self
                    .time_sale_start
                    .to_owned()
                    .add_days((i * DAYS_PER_VEST_PERIOD) as i64)
                    .unwrap()
                    .seconds_since_unix_epoch as u64;

                self.vested_withdrawals_db.insert(epoch.clone(), false);

                withdrawal_epoch_vector.insert(i as usize, epoch.clone().to_string());
            }

            Runtime::emit_event(WithdrawalCalculationEvent {
                withdraw_epochs: withdrawal_epoch_vector,
            });
        }

        //endregion Starting Token Sale

        //region Metadata Updaters

        /// Sets **initial metadata** for the trust fund.
        ///
        /// This function configures the metadata for the trust fund pool using the specified fee. It sets up
        /// the trust fund's name, description, and other attributes to ensure transparency and
        /// trustworthiness.
        ///
        /// # Arguments
        /// * `fee` - A bucket containing 1 XRD, which will be returned
        ///
        /// # Returns
        /// Bucket containing the fee passed in
        pub fn set_trustfund_metadata(&mut self, fee: Bucket) -> Bucket {

            self.pool
                .set_metadata("name".to_owned(), "SUPER Trust Fund".to_owned());
            self.pool.set_metadata("description".to_owned(), "A Trust Fund for the SUPER token to foster trust between developers and investors. This ensures you can reclaim part of your investment if the project's direction doesn't meet your expectations.".to_owned());
            self.pool
                .set_metadata("icon_url".to_owned(), self.active_colors.super_t.to_owned());
            self.pool.set_metadata(
                "dapp_definition".to_owned(),
                self.sale_details.dapp_definition_caddy.to_owned(),
            );

            let pool_token: Bucket = self.pool.contribute(fee);
            self.sale_details.super_t_raddy = pool_token.resource_address();

            let pool_token_manager: ResourceManager = ResourceManager::from(self.sale_details.super_t_raddy);

            pool_token_manager.set_metadata("name".to_owned(), "SUPER Trust Token".to_owned());
            pool_token_manager.set_metadata("symbol".to_owned(), "SUPERt".to_owned());
            pool_token_manager
                .set_metadata("icon_url".to_owned(), self.active_colors.super_t.to_owned());
            pool_token_manager.set_metadata(
                "dapp_definitions".to_owned(),
                self.sale_details.dapp_definition_caddy.to_owned(),
            );

            let return_fee: Bucket = self.pool.redeem(pool_token);

            return_fee
        }

        /// Updates the metadata of the SUPER token.
        /// This function modifies the metadata entry specified by `entry_to_update` with the new value `value_to_update_to`.
        /// If the entry is for "icon_url" or "info_url", it treats the value as a URL; otherwise, it treats it as a string.
        ///
        /// # Arguments
        /// * `entry_to_update` - The metadata entry to update (e.g., "icon_url", "name").
        /// * `value_to_update_to` - The new value for the metadata entry.
        pub fn update_super_metadata(
            &mut self,
            entry_to_update: String,
            value_to_update_to: String,
        ) {
            if (entry_to_update == "icon_url") | (entry_to_update == "info_url") {
                let url: UncheckedUrl = Url::of(value_to_update_to);
                self.super_manager
                    .set_metadata(entry_to_update.to_owned(), url);
            } else {
                self.super_manager
                    .set_metadata(entry_to_update.to_owned(), value_to_update_to);
            }
        }

        /// Updates the metadata of the SUPERt token.
        /// This function modifies the metadata entry specified by `entry_to_update` with the new value `value_to_update_to`.
        /// If the entry is for "icon_url" or "info_url", it treats the value as a URL; otherwise, it treats it as a string.
        ///
        /// # Arguments
        /// * `entry_to_update` - The metadata entry to update (e.g., "icon_url", "name").
        /// * `value_to_update_to` - The new value for the metadata entry.
        pub fn update_trust_metadata(
            &mut self,
            entry_to_update: String,
            value_to_update_to: String,
        ) {
            if self.sale_details.super_t_raddy == self.sale_details.super_y_raddy {
                panic!("Run self.start_sale() first!")
            }

            let pool_token_manager: ResourceManager = ResourceManager::from(self.sale_details.super_t_raddy);

            if (entry_to_update == "icon_url") | (entry_to_update == "info_url") {
                let url: UncheckedUrl = Url::of(value_to_update_to);
                pool_token_manager.set_metadata(entry_to_update.to_owned(), url);
            } else {
                pool_token_manager.set_metadata(entry_to_update.to_owned(), value_to_update_to);
            }
        }

        /// Updates the metadata of the SUPERy token.
        /// This function modifies the metadata entry specified by `entry_to_update` with the new value `value_to_update_to`.
        /// If the entry is for "icon_url" or "info_url", it treats the value as a URL; otherwise, it treats it as a string.
        ///
        /// # Arguments
        /// * `entry_to_update` - The metadata entry to update (e.g., "icon_url", "name").
        /// * `value_to_update_to` - The new value for the metadata entry.
        pub fn update_yield_metadata(
            &mut self,
            entry_to_update: String,
            value_to_update_to: String,
        ) {
            if (entry_to_update == "icon_url") | (entry_to_update == "info_url") {
                let url: UncheckedUrl = Url::of(value_to_update_to);
                self.super_yield_manager
                    .set_metadata(entry_to_update.to_owned(), url);
            } else {
                self.super_yield_manager
                    .set_metadata(entry_to_update.to_owned(), value_to_update_to);
            }
        }

        /// Updates the metadata of the SUPER NFT.
        /// This function modifies the metadata entry specified by `entry_to_update` with the new value `value_to_update_to`.
        /// If the entry is for "icon_url" or "info_url", it treats the value as a URL; otherwise, it treats it as a string.
        ///
        /// # Arguments
        /// * `entry_to_update` - The metadata entry to update (e.g., "icon_url", "name").
        /// * `value_to_update_to` - The new value for the metadata entry.
        pub fn update_nft_manager_metadata(&mut self) {
            let current_time: i64 =
                Clock::current_time_rounded_to_seconds().seconds_since_unix_epoch;

            let time_after_sale_start: u64 =
                (current_time - self.time_sale_start.seconds_since_unix_epoch) as u64;

            let hour_after_sale_start: u64 = time_after_sale_start / TIME_SECONDS_PER_HOUR;

            let total_super: u64 = self
                .super_manager
                .total_supply()
                .unwrap_or(dec!(0))
                .to_u64();

            //let total_trust: Decimal = self.pool.get_vault_amount();

            self.yield_nft_manager
                .set_metadata("Total SUPER Minted", total_super);
            self.yield_nft_manager
                .set_metadata("Current Time", time_after_sale_start);
            self.yield_nft_manager
                .set_metadata("Current Hour", hour_after_sale_start);
        }
        //endregion Metadata Updaters

        //region Ending Token Sale

        /// Ends the token sale and finalizes the sale details.
        /// This function sets the SUPER token to non-mintable, marks the sale as completed,
        /// calculates the vested withdrawal amount, and emits the updated sale details event.
        pub fn end_sale(&mut self) {
            // Set SUPER tokens to non-mintable
            self.super_manager.set_mintable(AccessRule::DenyAll);

            // Mark the sale as completed
            self.sale_details.sale_completed = true;

            // Calculate the total vested amount and the vested withdrawal amount
            let total_vested: Decimal = self.vesting_vault.amount();
            self.vested_withdrawal_amount = total_vested / Decimal::from(WEEKS_VESTED);

            // Emit the updated sale details event
            Runtime::emit_event(self.sale_details.to_owned());

            // Ensure the total vested amount is correctly divided into weekly withdrawals
            assert_eq!(
                Decimal::from(WEEKS_VESTED) * self.vested_withdrawal_amount,
                total_vested,
                "Total vested is not == to total being withdrawn."
            );
        }

        /// Checks if the token sale is complete and ends the sale if it is.
        /// This function compares the current time with the sale end time and calls `end_sale`
        /// if the current time is past the sale end time.
        pub fn check_if_sale_complete(&mut self) {
            // Get the current time rounded to seconds
            let now: Instant = Clock::current_time_rounded_to_seconds();

            // Check if the current time is greater than the sale end time
            let check_sale_complete: bool =
                now.compare(self.time_sale_end, TimeComparisonOperator::Gt);

            // Log the current sale completion status
            info!("Token sale completed = {}", self.sale_details.sale_completed);

            // If the sale is complete, end the sale
            if check_sale_complete {
                self.end_sale();
            }
        }

        //endregion Ending Token Sale

        //region Buying Functions


        /// Processes a deposit during the token sale.
        /// This function handles the payment, splits it into vested and trust fund amounts,
        /// mints SUPER tokens, creates a Yield NFT, and updates the relevant vaults and pools.
        ///
        /// # Arguments
        /// * `payment` - A bucket containing the payment in XRD tokens.
        ///
        /// # Returns
        /// A tuple containing:
        /// * The remaining payment bucket.
        /// * The contribution bucket to the trust fund.
        /// * The bucket containing minted SUPER tokens.
        /// * The bucket containing the created Yield NFT.
        pub fn deposit(&mut self, mut payment: Bucket) -> (Bucket, Bucket, Bucket, Bucket) {

            // Get the current time rounded to seconds
            let now: Instant = Clock::current_time_rounded_to_seconds();
            let now_integer: i64 = now.seconds_since_unix_epoch;

            // Get the start time of the sale
            let time_start: i64 = self.time_sale_start.to_owned().seconds_since_unix_epoch;

            // Calculate the time since the sale began
            let time_since_sale_began: u64 = (now_integer - time_start) as u64;

            // Check if the sale is complete
            self.check_if_sale_complete();

            // Ensure the sale is not completed
            assert!(
                !self.sale_details.sale_completed,
                "Token sale is finished, buy on secondary market!"
            );

            // Ensure the sale has started
            assert!(
                self.sale_details.sale_started,
                "Token sale hasn't started! Please wait"
            );

            // Ensure the payment is made with XRD tokens
            assert_eq!(payment.resource_address(), XRD, "Please pay with XRD.");

            // Find the nearest positive non-zero multiple of 10 for the payment amount
            let payment_amount: Decimal =
                self.find_positive_non_zero_multiple_of_10(payment.amount());

            // Calculate the vested amount and trust fund amount
            let amount_vested: Decimal = FRACTION_VESTED
                .checked_mul(payment_amount)
                .unwrap()
                .checked_round(0, RoundingMode::ToNearestMidpointToEven)
                .unwrap();


            let amount_trust_fund: Decimal = FRACTION_TRUST_FUND
                .checked_mul(payment_amount)
                .unwrap()
                .checked_round(0, RoundingMode::ToNearestMidpointToEven)
                .unwrap();

            // Ensure the payment is correctly split into vested and trust fund amounts
            assert_eq!(
                amount_vested.checked_add(amount_trust_fund).unwrap(),
                payment_amount,
                "Payment isn't being split right broski"
            );

            // Put the vested amount into the vesting vault
            self.vesting_vault.put(payment.take(amount_vested));

            // Contribute the remaining XRD to the trust fund pool
            let contribution: Bucket = self.pool.contribute(payment.take(amount_trust_fund));
            let trust_token_amount: Decimal = contribution.amount();

            // Mint SUPER tokens for the payment amount
            let minted_tokens: Bucket = self.super_manager.mint(payment_amount);
            let payment_int: u64 = payment_amount.to_u64();

            // Create a Yield NFT for the payment
            let (yield_nft, _): (Bucket, u64) = self.create_yield_nft(
                payment_int,
                trust_token_amount,
                time_since_sale_began,
                false,
            );

            // Return any remaining payment, the trust fund contribution, minted tokens, and the Yield NFT
            (payment, contribution, minted_tokens, yield_nft)
        }


        /// Creates a Yield NFT with the specified parameters.
        /// This function mints a new Yield NFT with the provided SUPER and trust amounts,
        /// records the NFT details in the database, and emits an event indicating the creation.
        ///
        /// # Arguments
        /// * `super_amount` - The amount of SUPER tokens minted with the NFT.
        /// * `trust_amount` - The amount of trust tokens (SUPERt) minted with the NFT.
        /// * `time_after_sale_start` - The time after the sale started, in seconds.
        /// * `splitting_nft` - A boolean indicating if the NFT is being created as part of a split.
        ///
        /// # Returns
        /// A tuple containing:
        /// * A `Bucket` with the minted Yield NFT.
        /// * A `u64` representing the ID of the newly created NFT.
        pub fn create_yield_nft(
            &mut self,
            super_amount: u64,
            trust_amount: Decimal,
            time_after_sale_start: u64,
            splitting_nft: bool,
        ) -> (Bucket, u64) {

            // Calculate the hour after the sale started
            let hour_after_sale_start: u64 = time_after_sale_start / TIME_SECONDS_PER_HOUR;

            // Create the YieldClaim data for the NFT
            let receipt_data: YieldClaim = YieldClaim {
                hour_of_mint: hour_after_sale_start,
                n_super_minted: super_amount,
                n_trust_minted: trust_amount,
            };

            // Set metadata for the NFT
            let manager: ResourceManager = self.yield_nft_manager;
            manager.set_metadata("Amount", super_amount);
            manager.set_metadata("Hour of Mint", hour_after_sale_start);

            // Get a valid NFT ID, ensuring no collisions
            let checked_time: u64 = self.get_checked_nft_id(time_after_sale_start);

            // Mint the new Yield NFT
            let nft: Bucket = self.yield_nft_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(checked_time.to_owned()),
                receipt_data.to_owned(),
            );

            // Add the NFT details to the database
            let inserted_nft_id: u64 = self.add_receipt_to_db(checked_time, receipt_data.clone());

            // Ensure the inserted NFT ID matches the checked time
            assert_eq!(
                checked_time, inserted_nft_id,
                "Key dont match w/ checked key"
            );

            // Update databases and emit events
            if !splitting_nft {
                // Update the databases with the new NFT details
                self.update_dbs_with(
                    Some(receipt_data.n_super_minted.to_owned()),
                    Some(receipt_data.hour_of_mint.to_owned()),
                );

                // Emit event for the creation of the Yield NFT
                Runtime::emit_event(CreateYieldNFTEvent {
                    nft_id: inserted_nft_id.to_owned(),
                    hour_of_mint: receipt_data.hour_of_mint.to_owned(),
                    n_super_minted: receipt_data.n_super_minted.to_owned(),
                    n_trust_minted: receipt_data.n_trust_minted.to_owned(),
                });
            } else {
                // Update the databases without specific details
                self.update_dbs_to_now();

                // Emit event for the creation of the Yield NFT as part of a split
                Runtime::emit_event(CreateYieldNFTEvent {
                    nft_id: inserted_nft_id.to_owned(),
                    hour_of_mint: receipt_data.hour_of_mint.to_owned(),
                    n_super_minted: receipt_data.n_super_minted.to_owned(),
                    n_trust_minted: receipt_data.n_trust_minted.to_owned(),
                });
            }

            // Return the minted NFT and its ID
            (nft, checked_time)
        }


        /// Ensures the NFT ID is unique by checking against existing IDs in the database.
        /// This function increments the provided NFT ID until a unique ID is found that does not
        /// already exist in the `yield_nft_db` database.
        ///
        /// # Arguments
        /// * `nft_id` - The initial NFT ID to check.
        ///
        /// # Returns
        /// A `u64` representing the first unique NFT ID that is not already used.
        pub fn get_checked_nft_id(&mut self, nft_id: u64) -> u64 {
            let mut key: u64 = nft_id;

            // Increment the key until a unique one is found
            while self.yield_nft_db.get(&key).is_some() {
                key += 1; // Increment the key if the current key is already used
            }

            key
        }

        /// Adds a new YieldClaim entry to the database, ensuring a unique key is used.
        /// This function checks the provided NFT ID for uniqueness, increments it if necessary,
        /// and then inserts the YieldClaim data into the `yield_nft_db` and `yield_generated_db` databases.
        ///
        /// # Arguments
        /// * `nft_id` - The initial NFT ID to check and insert.
        /// * `nft_data` - The YieldClaim data to insert.
        ///
        /// # Returns
        /// A `u64` representing the unique NFT ID used for the insertion.
        pub fn add_receipt_to_db(&mut self, nft_id: u64, nft_data: YieldClaim) -> u64 {
            let mut key: u64 = nft_id;
            let value: YieldClaim = nft_data;

            // Loop until an unused key is found
            while self.yield_nft_db.get(&key).is_some() {
                key += 1; // Increment the key if the current key is already used
            }

            // Insert the new value at the unused key
            self.yield_nft_db.insert(key, value);
            self.yield_generated_db.insert(key, dec!(0));

            // Return the key that was used for insertion
            key
        }

        //endregion Buying Functions

        //region Things to do w/ yield tokens

        /// Splits a Yield NFT into multiple smaller NFTs.
        /// This function checks the validity of the provided Yield NFT, splits it into the specified
        /// number of smaller NFTs, updates the relevant databases, and emits an event for the split.
        ///
        /// # Arguments
        /// * `yield_nft` - The original Yield NFT to be split.
        /// * `number_of_splits` - The number of smaller NFTs to create.
        ///
        /// # Returns
        /// A tuple containing:
        /// * A `Bucket` with the first newly created NFT.
        /// * A `Bucket` with the rest of the newly created NFTs.
        pub fn split_yield_nft(&mut self, yield_nft: NonFungibleBucket, number_of_splits: u64) -> (Bucket, Bucket) {

            // Ensure the provided NFT is a Yield NFT
            assert_eq!(
                yield_nft.resource_address(),
                self.sale_details.yield_nft_raddy,
                "Please send yield nft"
            );

            // Create a proof for the NFT and check it
            let nft_proof: NonFungibleProof = yield_nft.create_proof_of_all();
            let checked_nft: CheckedNonFungibleProof = nft_proof.check(self.sale_details.yield_nft_raddy);

            // Get the time of minting and data from the NFT
            let time_of_mint: u64 = self.nft_local_id_to_u64(checked_nft.non_fungible_local_id());
            let data: YieldClaim = yield_nft.non_fungible().data();

            // Ensure the NFT can be split into the specified number of parts
            assert!(
                data.n_super_minted >= number_of_splits,
                "Your max split is {}",
                data.n_super_minted
            );

            // Remove the yield generated for this NFT from the database
            let total_yield_generated: Decimal =
                self.yield_generated_db.remove(&time_of_mint).unwrap();

            let mut created_nft_ids: Vec<u64> = Vec::new();

            // Divide the SUPER and trust amounts for the splits
            let (super_first_nft, super_rest_nfts): (u64, u64) =
                self.divide_integer_into_n_integers(data.n_super_minted, number_of_splits);

            let (trust_first_nft, trust_rest_nfts): (Decimal, Decimal) = self
                .divide_decimal_into_n_weighted_decimals(
                    data.n_trust_minted,
                    super_first_nft,
                    super_rest_nfts,
                    number_of_splits,
                );

            let (yield_first_nft, yield_rest_nfts): (Decimal, Decimal) = self
                .divide_decimal_into_n_weighted_decimals(
                    total_yield_generated,
                    super_first_nft,
                    super_rest_nfts,
                    number_of_splits,
                );

            // Create the first split NFT
            let (first_nft, first_nft_local_id): (Bucket, u64) =
                self.create_yield_nft(super_first_nft, trust_first_nft, time_of_mint, true);

            // Update the yield generated database for the first NFT
            self.yield_generated_db
                .insert(first_nft_local_id, yield_first_nft);

            created_nft_ids.insert(0, first_nft_local_id);

            // Create a bucket to hold the rest of the split NFTs
            let mut split_nfts: Bucket = Bucket::new(yield_nft.resource_address());

            // Create the rest of the split NFTs
            for split_number in 1..number_of_splits {
                let new_time: u64 = &time_of_mint + split_number + 1;

                let (rest_nft, local_id): (Bucket, u64) =
                    self.create_yield_nft(super_rest_nfts, trust_rest_nfts, new_time, true);

                split_nfts.put(rest_nft);

                self.yield_generated_db.insert(local_id, yield_rest_nfts);

                {
                    let index: usize = usize::try_from(split_number).unwrap_or_else(|e| {
                        eprintln!("Error converting u64 to usize: {}", e);
                        usize::MAX
                    });

                    created_nft_ids.insert(index, local_id);
                }
            }

            // Remove the original NFT data from the database
            let removed_data: YieldClaim = self.yield_nft_db.remove(&time_of_mint).unwrap();

            // Ensure the original NFT data matches the removed data
            assert_eq!(
                data.hour_of_mint, removed_data.hour_of_mint,
                "nft data ain't matching broski"
            );
            assert_eq!(
                data.n_super_minted, removed_data.n_super_minted,
                "nft data ain't matching broski"
            );

            // Burn the original NFT
            checked_nft.drop();
            yield_nft.burn();

            // Get the IDs and data for the newly created NFTs
            let first_nft_id: NonFungibleLocalId = first_nft.as_non_fungible().non_fungible_local_id();
            let first_nft_data: YieldClaim = first_nft.as_non_fungible().non_fungible().data();

            let rest_nft_ids: Vec<NonFungibleLocalId> = split_nfts.as_non_fungible().non_fungible_local_ids().to_owned().into_iter().collect();
            let rest_nft_data: YieldClaim = split_nfts.as_non_fungible().non_fungibles().first().unwrap().data();

            // Emit an event for the split
            Runtime::emit_event(SplitNFTEvent {
                burnt_nft_id: time_of_mint,
                burnt_nft_data: data,
                first_nft_id: first_nft_id,
                first_nft_data: first_nft_data,
                rest_nft_ids: rest_nft_ids,
                rest_nft_data: rest_nft_data
            });

            // Return NFTs
            (first_nft, split_nfts)
        }

        //endregion Things to do w/ yield tokens

        //region Claiming Yield

        /// Claims the yield for a Yield NFT by redeeming trust fund tokens and minting SUPERy tokens.
        /// This function verifies the provided NFT and trust fund tokens, updates databases, redeems the trust fund tokens,
        /// mints the yield tokens, and emits an event for the yield claim.
        ///
        /// # Arguments
        /// * `nft` - The Yield NFT for which the yield is being claimed.
        /// * `trust_fund_tokens` - A bucket containing the trust fund tokens for redemption.
        ///
        /// # Returns
        /// A tuple containing:
        /// * A `Bucket` with the newly minted SUPERy tokens.
        /// * A `Bucket` with any remaining trust fund tokens.
        pub fn claim_yield(
            &mut self,
            nft: NonFungibleBucket,
            mut trust_fund_tokens: Bucket,
        ) -> (Bucket, Bucket) {

            //region Running all necessary checks (time, amount tokens, token_addy, nft_addy, nft_id, updating dbs)

            // Update the databases to the current state
            self.update_dbs_to_now();

            // Get the current time in hours since the sale started
            let now: u64 = self.hours_since_start();

            // Ensure the yield can only be claimed after the sale finishes
            assert!(
                now > SALE_DURATION_DAYS * 24,
                "Please wait til after the sale finishes to claim yield"
            );

            // Ensure the yield can only be claimed after the sale finishes
            assert_eq!(
                trust_fund_tokens.resource_address(),
                self.sale_details.super_t_raddy,
                "Send trust tokens"
            );

            let trust_fund_amount: Decimal = trust_fund_tokens.amount();

            // Check and burn the provided NFT
            let (nft_id, _nft_data, trust_amount_to_return): (u64, YieldClaim, Decimal) =
                self.check_and_burn_nft(nft, trust_fund_amount);

            //endregion

            // Get the amount of yield tokens to mint from the database
            let amount_to_mint: Decimal = *self.yield_generated_db.get(&nft_id).unwrap();

            // Calculate the amount of trust fund tokens to return
            let return_trust_fund_tokens: Bucket = trust_fund_tokens.take(trust_amount_to_return);

            // Redeem the provided trust fund tokens for XRD
            let trust_fund_redemption: Bucket = self.pool.redeem(trust_fund_tokens);

            let amount_trust_fund_redemption: Decimal = trust_fund_redemption.amount();

            // Deposit the redeemed XRD into the vesting vault
            self.vesting_vault.put(trust_fund_redemption);

            // Emit an event for the yield claim
            Runtime::emit_event(ClaimYieldEvent {
                hour_of_claim: now,
                super_y_minted: amount_to_mint,
                nft_id,
                trust_fund_redemption_amount: amount_trust_fund_redemption,
            });

            // Mint the yield tokens and return the minted tokens and any remaining trust fund tokens
            (
                self.super_yield_manager.mint(amount_to_mint),
                return_trust_fund_tokens,
            )
        }

        /// Checks the validity of a Yield NFT and burns it.
        /// This function verifies the provided NFT against the stored data, ensures that the trust fund amount is sufficient,
        /// and then burns the NFT, emitting an event for the burn.
        ///
        /// # Arguments
        /// * `nft` - The Yield NFT to check and burn.
        /// * `trust_fund_amount_in` - The amount of trust fund tokens provided for the NFT.
        ///
        /// # Returns
        /// A tuple containing:
        /// * A `u64` representing the ID of the burned NFT.
        /// * A `YieldClaim` struct with the data of the burned NFT.
        /// * A `Decimal` representing the amount of trust fund tokens to return to the user.
        pub fn check_and_burn_nft(
            &self,
            nft: NonFungibleBucket,
            trust_fund_amount_in: Decimal,
        ) -> (u64, YieldClaim, Decimal) {

            // Create a proof for the NFT and check it
            let nft_proof: NonFungibleProof = nft.create_proof_of_all();
            let checked_nft: CheckedNonFungibleProof = nft_proof.check(self.sale_details.yield_nft_raddy);

            // Get the local ID and data from the NFT
            let local_id: u64 = self.nft_local_id_to_u64(checked_nft.non_fungible_local_id());
            let nft_data: YieldClaim = checked_nft.as_non_fungible().non_fungible().data();

            // Retrieve the matching NFT data from the database
            let matching_nft: YieldClaim = match self.yield_nft_db.get(&local_id) {
                Some(nft) => *nft,
                None => panic!("Couldn't find NFT in db"),
            };

            let trust_fund_amount_nft: Decimal = nft_data.n_trust_minted;

            //region asserting each value from db

            // Ensure the trust fund amounts match
            assert_eq!(
                matching_nft.n_trust_minted, nft_data.n_trust_minted,
                "Mismatch in n_trust_minted values"
            );

            // Ensure the provided trust fund amount is sufficient
            assert!(
                trust_fund_amount_in >= nft_data.n_trust_minted,
                "Amount of trust fund tokens ain't enough."
            );

            // Ensure the SUPER amounts match
            assert_eq!(
                matching_nft.n_super_minted, nft_data.n_super_minted,
                "Mismatch in n_super_minted values"
            );

            // time is checked when finding nft in the db,

            //endregion

            // Figuring out the number of trust_fund_tokens to return to user:
            let amount_trust_tokens_to_return: Decimal = trust_fund_amount_in
                .checked_sub(trust_fund_amount_nft)
                .unwrap();

            if amount_trust_tokens_to_return.is_negative() {
                panic!("Send more SUPERt");
            };

            // Emit an event for the NFT burn
            Runtime::emit_event(BurnYieldNFTEvent {
                nft_id: local_id,
                hour_of_mint: nft_data.hour_of_mint,
                n_super_minted: nft_data.n_super_minted,
                n_trust_minted: nft_data.n_trust_minted,
            });

            // Drop the checked proof and burn the NFT
            checked_nft.drop();
            nft.burn();

            // Return the local ID, NFT data, and the amount of trust fund tokens to return
            (local_id, nft_data, amount_trust_tokens_to_return)
        }

        //endregion Claiming Yield

        //region Vested withdrawal functions

        /// Processes a vested withdrawal from the vesting vault.
        /// This function checks the vesting schedule, calculates the allowed withdrawals,
        /// and withdraws the appropriate amount from the vesting vault.
        ///
        /// # Returns
        /// A `Bucket` containing the withdrawn XRD.
        pub fn vested_withdraw(&mut self) -> Bucket {

            // Ensure the token sale is complete before allowing withdrawals
            assert!(!self.sale_details.sale_completed, "Token Sale is not yet complete!");

            // Update the databases to the current state
            self.update_dbs_to_now();

            let mut withdrawals_allowed: u64 = 0;
            let mut used_withdrawals: u64 = 0;

            // Iterate through the vesting withdrawals database to determine allowed withdrawals
            for (withdraw_date, used, _) in self.vested_withdrawals_db.range(..) {
                if (!used)
                    && (Clock::current_time_is_at_or_after(
                    Instant::new(withdraw_date as i64),
                    TimePrecision::Minute,
                ))
                {
                    withdrawals_allowed += 1;
                } else if used {
                    used_withdrawals += 1;
                }
            }

            // Create a new bucket for the withdrawal
            let mut withdrawal: Bucket = Bucket::new(XRD);

            // If all withdrawals have been used and no more are allowed, take all remaining funds
            if (used_withdrawals == WEEKS_VESTED) && (withdrawals_allowed == 0) {
                withdrawal.put(self.vesting_vault.take_all());
            }

            // Calculate the amount to withdraw based on allowed withdrawals
            let withdrawal_amount: Decimal =
                Decimal::from(withdrawals_allowed) * self.vested_withdrawal_amount;

            // Withdraw the calculated amount from the vesting vault
            withdrawal.put(self.vesting_vault.take(withdrawal_amount));

            withdrawal
        }

        //endregion Vested withdrawal functions

        //region AVL Tree Updaters

        /// Updates the hourly SUPER minted data with the specified amount and hour.
        /// This function updates the database with the amount of SUPER tokens minted for a specific hour.
        /// If no amount or hour is provided, it defaults to 0 and the current hour since the sale started.
        ///
        /// # Arguments
        /// * `amount` - An optional `u64` representing the amount of SUPER tokens minted.
        /// * `hour` - An optional `u64` representing the number of hours since the sale started.
        pub fn update_dbs_with(&mut self, amount: Option<u64>, hour: Option<u64>) {
            let amount: u64 = amount.unwrap_or(0);

            let hours_elapsed: u64 = hour.unwrap_or_else(|| self.hours_since_start());

            // Update the hourly SUPER minted data
            self.update_hourly_super_minted(hours_elapsed, amount);
        }

        /// Updates the databases to the current state.
        /// This function ensures the databases are up-to-date by calling `update_dbs_with`
        /// and `update_yield_generated` to update the hourly SUPER minted data and the yield generated.
        pub fn update_dbs_to_now(&mut self) {

            //just in case a new nft was minted or burnt within the hour:
            self.update_dbs_with(None, None);

            // Once amount fractions are up-to-date, yield_generated can be calculated 
            // and then updated in a separate database using:
            self.update_yield_generated();
        }

        /// Updates the hourly SUPER minted data.
        /// This function updates the amount of SUPER tokens minted for the given hour.
        /// If the hour is not already in the database, it fills in any missing hours
        /// and sets the new total amount minted.
        ///
        /// # Note
        /// There may be a memory issue with this function, since as the trees get longer it gets
        /// more expensive to access them using .range(..).last()
        ///
        /// # Arguments
        /// * `hours_since_start` - The number of hours since the sale started.
        /// * `amount` - The amount of SUPER tokens minted in the current hour.
        pub fn update_hourly_super_minted(&mut self, hours_since_start: u64, amount: u64) {
            // If the key does not exist in the db, this will return None.
            if let Some(mut data_for_hour) = self.hourly_super_minted.get_mut(&hours_since_start) {
                {
                    // If key exists, update by adding amount.
                    *data_for_hour += amount;
                }

                return;
            }

            // If a key does not exist for this hour, insert a new key-value pair with the given hour and amount.
            /* OLD APPROACH
            let last_super_minted: (u64, u64, Option<u64>) = self
                .hourly_super_minted
                .range(..)
                .last()
                .unwrap_or((0, 0, None)); 
                
            let last_hour_updated: u64 = last_super_minted.0;

            let total_amount: u64 = last_super_minted.1;
            */

            // Retrieve the last updated hour and total amount using a more efficient approach
            let (last_hour_updated, total_amount) = match self.hourly_super_minted.range(..).last() {
                Some((last_hour, total_amount, _)) => (last_hour, total_amount),
                None => (0, 0),
            };

            // Insert the total amount for each hour up to the current hour
            for hour in last_hour_updated..=hours_since_start {
                //info!("At hour {} total SUPER minted = {}", hour, total_amount);
                self.hourly_super_minted.insert(hour, total_amount);
            }

            // Calculate the new total amount and insert it for the current hour
            let new_total: u64 = total_amount + amount;
            self.hourly_super_minted
                .insert(hours_since_start, new_total);
        }

        /// Calculates and updates the yield generated for each SUPER token.
        /// This function updates the `yield_per_super_db` and `yield_generated_db` for the hour
        /// before the current hour. It calculates the yield based on the yield curve and the amount of
        /// SUPER tokens minted, then updates the yield generated for each NFT.
        ///
        /// # Note
        /// This function iterates over the range of hours from the last updated hour to the current hour,
        /// ensuring all intermediate hours are updated accordingly.
        pub fn update_yield_generated(&mut self) {

            // Get the current hour since the sale started
            let now_hour: u64 = self.hours_since_start();

            // Iterate through each hour from the last updated hour to the current hour
            for current_hour in self.dbs_updated_up_to_before_hour..=now_hour {

                // Calculate the yield tokens minted for the current hour
                let yield_tokens_minted: Decimal =
                    self.calculate_yield_curve_for_hour(current_hour);

                // Get the amount of SUPER minted in the current hour
                let super_minted_in_hour: u64 =
                    *self.hourly_super_minted.get(&current_hour).unwrap();

                // Calculate the yield per SUPER token for the current hour
                let yield_per_super_for_hour: Decimal = yield_tokens_minted
                    .checked_div(super_minted_in_hour)
                    .unwrap();

                // Update the yield generated for each NFT
                self.yield_generated_db.range_mut(..).for_each(
                    |(nft_id, yield_generated, next_nft_id): (&u64, &mut Decimal, Option<u64>)| {

                        let nft_data: YieldClaim = *self.yield_nft_db.get(nft_id).unwrap();
                        let hour_minted: u64 = nft_data.hour_of_mint;
                        let super_minted: u64 = nft_data.n_super_minted;

                        // Update the yield generated for the current NFT if it was minted before the current hour
                        if current_hour >= hour_minted {
                            let yield_generated_this_hour: Decimal =
                                yield_per_super_for_hour.checked_mul(super_minted).unwrap();

                            //info!("Yield_generated updated from {}", yield_generated);

                            {
                                *yield_generated = yield_generated
                                    .checked_add(yield_generated_this_hour)
                                    .unwrap();
                            }

                            //info!("to {}", yield_generated);
                            //info!("Hour: {}, NFT: {}, Yield: {}", current_hour, nft_id, yield_generated);
                        }

                        // Continue iterating or break if this is the last entry
                        match next_nft_id {
                            Some(_x) => scrypto_avltree::IterMutControl::Continue,
                            None => scrypto_avltree::IterMutControl::Break,
                        }
                    },
                );
            }

            // Update the last updated hour and mark the hour as updated in the checklist
            self.dbs_updated_up_to_before_hour = now_hour + 1;
            self.hour_updated_checklist.insert(now_hour, true);
            //info!("Yield db Updated up to hour {}", self.dbs_updated_up_to_before_hour);
        }

        /*
        pub fn update_yield_curve_ten_more_hours(&mut self) {
            if (self.yield_curve_updated_to_before_hour > 2689
        }
        */

        //endregion AVL Tree Updaters

        //region AVL Tree Viewers

        /// Emits events to show the hourly SUPER minted data.
        /// This function iterates through the `hourly_super_minted` database and emits an event
        /// for each hour, showing the amount of SUPER tokens minted.
        pub fn show_hourly_super_minted(&mut self) {
            for (key, val, _next_key) in self.hourly_super_minted.range(..) {
                // Emit an event for each hour with the amount of SUPER minted
                Runtime::emit_event(ShowSuperMintedEvent {
                    time: key,
                    n_super: val,
                });
            }
        }

        /// Emits events to show the hourly yield generated data.
        /// This function iterates through the `yield_generated_db` database and emits an event
        /// for each NFT, showing the yield generated up to the current hour.
        pub fn show_hourly_yield_generated(&mut self) {
            // Emit an event for each NFT with the yield generated
            for (key, val, _next_key) in self.yield_generated_db.range(..) {
                Runtime::emit_event(YieldUpdateEvent {
                    time: self.hours_since_start(),
                    nft_id: key,
                    yield_generated: val,
                });

                // Log the yield generated for each NFT
                info!("NFT ID {}, yield generated = {}", key, val);
            }
        }

        //endregion AVL Tree Viewers

        //region Helper functions

        /// Ensures that a given decimal is a positive non-zero multiple of 10.
        ///
        ///
        /// # Examples
        /// ```rust, ignore
        /// let number = dec!("258");
        /// let adjusted_number = self.find_positive_non_zero_multiple_of_10(number);
        /// assert_eq!(adjusted_number, dec!(250));
        /// ```
        ///
        /// # Panics
        /// This function will panic if:
        /// - The input `number` is not positive.
        /// - The division or multiplication operation fails, which can happen if the input number is extremely large.
        pub fn find_positive_non_zero_multiple_of_10(&self, number: Decimal) -> Decimal {
            assert!(number.is_positive(), "Positive payment required");

            let new_num: Decimal = number
                .checked_div(10u64)
                .unwrap()
                .round_down_no_digits()
                .checked_mul(10u64)
                .unwrap();

            assert_ne!(new_num, dec!("0.0"), "Pay more than 10");

            new_num
        }

        /// Calculates the number of hours elapsed since the start of a sale.
        ///
        /// This function computes the difference between the current time and the sale start time,
        /// then converts this difference into hours. The current time is rounded to the nearest second
        /// before the calculation to ensure consistency.
        ///
        /// # Returns
        /// - `u64`: The number of hours that have elapsed since `self.time_sale_start`.
        pub fn hours_since_start(&self) -> u64 {
            (Clock::current_time_rounded_to_seconds().seconds_since_unix_epoch
                - self.time_sale_start.to_owned().seconds_since_unix_epoch) as u64
                / TIME_SECONDS_PER_HOUR
        }

        /// Calculates the yield curve at a given time `t` using:
        /// ```rust, ignore
        ///
        /// f(t) = πt + [ (807305e) / (et + 1) ]
        /// ```
        ///
        /// where:
        ///
        /// `f(t)` = yield at time `t`
        ///
        /// `e` = Euler's number, 2.718...  
        /// `π` = Pi, 3.141...  
        /// `t` = time
        ///
        /// # Arguments
        /// * `t_int` - The time `t` as an `u64`, representing the point in time at which the yield is calculated.
        ///
        /// # Returns
        /// Returns a `Decimal` representing the yield value at the given time `t`.
        ///
        ///
        pub fn calculate_yield_curve_for_hour(&self, hour: u64) -> Decimal {
            // f(t) =   πt      +   [ (807305e) / (et + 1) ]
            //      =   term_1  +   [ (term_2_numerator) / (term_2_denominator_1 + term_2_denominator_2) ]
            //      =   term_1  +   [ term_2_numerator / term_2_denominator ]
            //      =   term_1  +   [ term_2 ]

            let term_1: Decimal = PI.checked_mul(hour).unwrap();

            let term_2_numerator: Decimal = EULER.checked_mul(807305).unwrap();

            let term_2_denominator_1: Decimal = EULER.checked_mul(hour).unwrap();
            let term_2_denominator_2: Decimal = dec!("1.0");

            let term_2_denominator: Decimal = term_2_denominator_1
                .checked_add(term_2_denominator_2)
                .unwrap();

            let term_2: Decimal = term_2_numerator.checked_div(term_2_denominator).unwrap();

            let f_x: Decimal = term_1.checked_add(term_2).unwrap();

            f_x
        }

        /// Divides an integer into `n` almost equal parts.
        ///
        /// # Arguments
        /// * `number` - The `u64` number to be divided.
        /// * `n` - The number of parts to divide `number` into.
        ///
        /// # Returns
        /// Returns a tuple of two `u64` values:
        /// - The first value is the first part of the division, potentially adjusted for any remainder.
        /// - The second value is the size of each of the remaining `n-1` parts.
        ///
        /// # Examples
        /// Assuming a scenario where you want to divide 90 into 4 parts:
        /// ```rust, ignore
        /// let number: u64 = 90;
        /// let parts: u64 = 4;
        /// let (first_part, other_parts) = divide_integer_into_n_integers(number, parts);
        ///
        /// assert_eq!(first_part, 24);
        /// assert_eq!(other_parts, 22);
        /// assert_eq!(first_part + other_parts * (parts - 1), number);
        pub fn divide_integer_into_n_integers(&self, number: u64, n: u64) -> (u64, u64) {
            let mut first_number: u64 = number / n;

            let rest_number: u64 = first_number;

            if first_number * n != number {
                // Take whatever is left from rest * (n-1) and make it equal to the first
                first_number = number - rest_number * (n - 1);
            }

            assert_eq!(
                first_number + rest_number * (n - 1),
                number,
                "when i split {number} into {n} parts, I fucked up, I got first_num = {first_number}, rest_num = {rest_number}");

            (first_number, rest_number)
        }

        /// Divides a scrypto decimal into `n` almost equal parts.
        ///
        /// # Arguments
        /// * `number` - The `Decimal` number to be divided.
        /// * `n` - The number of parts to divide `number` into.
        ///
        /// # Returns
        /// Returns a tuple of two `Decimal` values:
        /// - The first value is the first part of the division, potentially adjusted for any remainder.
        /// - The second value is the size of each of the remaining `n-1` parts.
        ///
        /// # Examples
        /// Assuming a scenario where you want to divide 90 into 4 parts:
        /// ```rust, ignore
        /// let number: Decimal = dec!(90);
        /// let parts: u64 = 4;
        /// let (first_part, other_parts) = divide_decimal_into_n_weighted_decimals(number, parts);
        ///
        /// assert_eq!(first_part, 22.5);
        /// assert_eq!(other_parts, 22.5);
        /// assert_eq!(first_part + other_parts * (parts - 1), number);
        /// ```
        ///
        /// # Panics
        /// This function will panic if:
        /// - The division by `n` results in an error or overflow.
        /// - The recalculations for adjusting remainders fail due to underflow or other arithmetic errors.
        pub fn divide_decimal_into_n_weighted_decimals(
            &self,
            number: Decimal,
            first_weight: u64,
            rest_weight: u64,
            n: u64,
        ) -> (Decimal, Decimal) {
            let total_weight: u64 = first_weight + ((n - 1) * rest_weight);

            let first_weight_fraction: Decimal = Decimal::from(first_weight)
                .checked_div(total_weight)
                .unwrap();
            let rest_weight_fraction: Decimal = Decimal::from(rest_weight)
                .checked_div(total_weight)
                .unwrap();

            let first_number: Decimal = number.checked_mul(first_weight_fraction).unwrap();
            let rest_number: Decimal = number.checked_mul(rest_weight_fraction).unwrap();

            assert_eq!(
                first_number.checked_add(rest_number.checked_mul(n-1).unwrap()).unwrap().checked_round(8, RoundingMode::ToNearestMidpointToEven).unwrap(),
                number.checked_round(8, RoundingMode::ToNearestMidpointToEven).unwrap(),
                "when i split the decimal {number} into {n} weighted parts, I fucked up, I got first_num = {first_number}, rest_num = {rest_number}");

            (first_number, rest_number)
        }
        /*
                pub fn check_update_checklist(&self) {
                    let mut missing_hours: Vec<u64> = Vec::new();

                    let total_hours: u64 = WEEKS_VESTED * TIME_HOURS_PER_WEEK;

                    let expected_length: usize = total_hours as usize;

                    let actual_length: usize = self.hour_updated_checklist.get_length();

                    if expected_length == actual_length {
                        info!("Length is good");
                        return;
                    } else {
                        info!("Discrepancy found: expected {} hours, found {} entries.", total_hours, actual_length);
                    }

                    // Iterate over all entries in the tree
                    let mut entries: scrypto_avltree::NodeIterator<u64, bool> = self.hour_updated_checklist.range(..);
                    let mut expected_hour: u64 = 0; // Initialize expected_hour

                    let mut current_entry: Option<(u64, bool, Option<u64>)> = entries.next();

                    while expected_hour < total_hours {
                        match current_entry {

                            Some((key, _value, _next_key)) if key.to_owned() > expected_hour => {
                                // If the current key is greater than the expected hour, hours are missing
                                missing_hours.extend(expected_hour..key.to_owned());
                                expected_hour = key.to_owned() + 1;  // Skip to the next expected hour after current key
                                current_entry = entries.next();  // Move to the next entry
                            },
                            Some((key, _value, _next_key)) => {
                                expected_hour = key + 1;  // Match found, move to the next hour
                                current_entry = entries.next();
                            },
                            None => {
                                // No more entries, all remaining hours are missing
                                missing_hours.extend(expected_hour..total_hours);
                                break;
                            }
                        }
                    }

                    // Log discrepancies if there are missing hours
                    if !missing_hours.is_empty() {
                        for _hour in missing_hours {
                            info!("Missing log for hour {}", hour);
                        }
                    } else {
                        info!("All hours are correctly logged in the checklist.");
                    }

                }
        */

        /// Converts a NonFungibleLocalID<Integer> to a u64 integer
        pub fn nft_local_id_to_u64(&self, nft_local_id: NonFungibleLocalId) -> u64 {
            nft_local_id
                .to_string()
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u64>()
                .expect("Failed to get non fungible local id")
        }
        //endregion Helper functions
    }
}
