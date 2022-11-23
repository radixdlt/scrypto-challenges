//! The main blueprint of RadSense with which users are going to interact.
//! It offers methods for registering users (advertisers, ad slot providers, ad brokers), ads and ad slots
//! and for subsequently managing those entities. It is also used by aad brokers to create invoices.

use crate::dao_kit::code_execution_system::CodeExecution;
use crate::dao_kit::component_address_repo::ComponentAddressRepoComponent;
use crate::dao_kit::membership_system::DaoMember;
use crate::dao_kit::simple_dao_system::*;
use crate::dao_kit::voting_system::*;
use crate::invoice::{InvoiceAddresses, InvoiceComponent, InvoiceConfig};
use crate::utils::{Minter, NonFungibleConfig};
use derive_new::new;
use scrypto::address::Bech32Encoder;
use scrypto::prelude::*;

blueprint! {

    struct RadSense {
        /// A struct with all relevant RadSense resource addresses
        rsa: RadSenseAddresses,
        /// The central authority of the RadSense component that is used to min user, ads and ad slots
        minter: Minter,
        /// A central vault for all funds that are handled by this component. Contains XRD.
        funds: Vault,
        /// The addresses of the DAO system that handles arbitration cases for RadSense
        arbitration_dao_addresses: DaoSystemAddresses,
        /// A vault holding the admin badge that the RadSense component needs to invoke the arbitration DAO
        arbitration_dao_admin_badge: Vault,
        /// A set of URLs that point to the tracking APIs of the arbitrators. These APIs will be called whenever an
        /// ad is assigned to an ad slot and when that ad is clicked. Arbitrators can use this information to check
        /// whether the invoices created by ad brokers are correct.
        arbitrator_tracking_api_urls: HashSet<String>,
        /// This is needed as a workaround for https://github.com/radixdlt/radixdlt-scrypto/issues/483
        component_address_repo: ComponentAddress,
    }

    impl RadSense {
        /// Instantiates a new RadSense component.
        ///
        /// # Arguments
        /// - `initial_arbitrators` - A list of user names and their resp. account component addresses. These users will
        /// be the initial members of the arbitration DAO that handles any disputes between RadSense users.
        /// - `arbitrator_tracking_api_urls` - A set of URLs that point to the tracking APIs of the arbitrators
        /// - `kyc_resources` - A set of KYC resources (i.e. KYC providers) that RadSense accepts. Users can provide a
        /// proof of any of those resources to demonstrate that they have been KYCed. The set may be empty in which case
        /// the KYC feature will not be supported.
        ///
        /// # Returns
        /// 0. The instantiated, local RadSense component
        /// 1. A struct containing all relevant RadSense resource addresses
        /// 2. A struct containing the relevant addresses of the arbitration DAO
        pub fn instantiate(
            initial_arbitrators: Vec<(String, ComponentAddress)>,
            arbitrator_tracking_api_urls: HashSet<String>,
            kyc_resources: BTreeSet<ResourceAddress>,
        ) -> (RadSenseComponent, RadSenseAddresses, DaoSystemAddresses) {
            // Create a minting authority
            let minter =
                Minter::new(ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE).initial_supply(dec!(1)));

            // Create non-fungible resource for the three user types (ad broker, advertisers and ad slot providers)
            let ad_broker_resource = minter.new_non_fungible_resource("Ad Broker Badge", NonFungibleConfig::new());
            let advertiser_resource = minter.new_non_fungible_resource("Advertiser Badge", NonFungibleConfig::new());
            let ad_slot_provider_resource =
                minter.new_non_fungible_resource("Ad Slot Provider Badge", NonFungibleConfig::new());

            // Create a non-fungible resource for advertisements
            let ad_resource = minter
                .new_non_fungible_resource("Ad Token", NonFungibleConfig::new().updatable_non_fungible_data(true));

            // WARNING! Hardcoding the NetworkDefinition to adapanet here is of course very bad and will produces
            // wrong addresses when used on any network other than alphanet. I'm doing it anyway because I only need the
            // metadata on alphanet for the frontend.
            //
            // FIXME A proper feature for encoding addresses into metadata fields has been announced. Wait for that
            //  feature and use it
            let bech32_encoder = Bech32Encoder::new(&NetworkDefinition::adapanet());

            // Create a non fungible resource for ad slots
            let ad_slot_resource = minter.new_non_fungible_resource(
                "Ad Slot Token",
                NonFungibleConfig::new()
                    .metadata("AdBrokerResource", bech32_encoder.encode_resource_address(&ad_broker_resource))
                    .metadata("AdvertiserResource", bech32_encoder.encode_resource_address(&advertiser_resource))
                    .metadata(
                        "AdSlotProviderResource",
                        bech32_encoder.encode_resource_address(&ad_slot_provider_resource),
                    )
                    .metadata("AdResource", bech32_encoder.encode_resource_address(&ad_resource)),
            );

            // Create a fungible resource that acts as an access badge for this component
            let access_badge_resource =
                minter.new_fungible_resource("RadSense Internal Access Badge", DIVISIBILITY_NONE);

            // Wrap all addresses in a struct
            let rsa = RadSenseAddresses {
                ad_slot_resource,
                ad_resource,
                advertiser_resource,
                ad_slot_provider_resource,
                ad_broker_resource,
                access_badge_resource,
                kyc_resources,
            };

            // Instantiate a DAO that will be uses to arbitrate invoice disputes
            let initial_arbitrators = initial_arbitrators
                .into_iter()
                .map(|(arbitrator_name, arbitrator_account_address)| {
                    (DaoMember::new(arbitrator_name, None), arbitrator_account_address)
                })
                .collect();
            let component_address_repo = ComponentAddressRepoComponent::instantiate_global();
            let (arbitration_dao_addresses, arbitration_dao_admin_badge) =
                SimpleDaoSystemComponent::instantiate_global(initial_arbitrators, component_address_repo);

            // Instantiate the RadSense component
            let component = Self {
                rsa: rsa.clone(),
                minter,
                funds: Vault::new(RADIX_TOKEN),
                arbitration_dao_addresses: arbitration_dao_addresses.clone(),
                arbitration_dao_admin_badge: Vault::with_bucket(arbitration_dao_admin_badge),
                arbitrator_tracking_api_urls,
                component_address_repo,
            }
            .instantiate();

            (component, rsa, arbitration_dao_addresses)
        }

        /// Instantiates a new RadSense component and globalizes it.
        ///
        /// # Arguments
        /// - `initial_arbitrators` - A list of user names and their resp. account component addresses. These users will
        /// be the initial members of the arbitration DAO that handles any disputes between RadSense users.
        /// - `arbitrator_tracking_api_urls` - A set of URLs that point to the tracking APIs of the arbitrators
        /// - `kyc_resources` - A set of KYC resources (i.e. KYC providers) that RadSense accepts. Users can provide a
        /// proof of any of those resources to demonstrate that they have been KYCed. The set may be empty in which case
        /// the KYC feature will not be supported.
        ///
        /// # Returns
        /// 0. The component address of the instantiated and globalized RadSense component
        /// 1. A struct containing all relevant RadSense resource addresses
        /// 2. A struct containing the relevant addresses of the arbitration DAO
        pub fn instantiate_global(
            initial_arbitrators: Vec<(String, ComponentAddress)>,
            arbitrator_tracking_api_urls: HashSet<String>,
            kyc_resources: BTreeSet<ResourceAddress>,
        ) -> (ComponentAddress, RadSenseAddresses, DaoSystemAddresses) {
            let (mut component, rsa, arbitration_dao_addresses) =
                Self::instantiate(initial_arbitrators, arbitrator_tracking_api_urls, kyc_resources);

            let access_rules = AccessRules::new()
                .method("register", rule!(allow_all))
                .method("create_invoice", rule!(allow_all))
                .method("claim_ad_budget", rule!(require(rsa.access_badge_resource)))
                .method("deposit_ad_budget", rule!(allow_all))
                .method("redeposit_ad_budget", rule!(require(rsa.access_badge_resource)))
                .method("create_invoice_arbitration_vote", rule!(require(rsa.access_badge_resource)))
                .method(
                    "add_arbitrator_tracking_api_url",
                    rule!(require(arbitration_dao_addresses.dao_system_admin_badge_resource)),
                )
                .method("create_proposal", rule!(require(arbitration_dao_addresses.membership_resource)));
            component.add_access_check(access_rules);

            (component.globalize(), rsa, arbitration_dao_addresses)
        }

        /// Handles the given [RegistrationRequest] allowing the caller to register a user ([Advertiser],
        /// [AdSlotProvider] or [AdBroker]), an [Ad] or and [AdSLot].
        ///
        /// # Arguments
        /// - `request` - The registration request to handle
        ///
        /// # Returns
        /// 1. A bucket containing a NFR for the registered user, ad or ad slot
        /// 2. The ID of the returned NFR (this is needed by the web frontend)
        ///
        /// **Access control:** Can be called by anyone, but the caller might have to provide a proof inside the request
        /// struct if registering an ad or ad slot.
        pub fn register(&mut self, request: RegistrationRequest) -> (Bucket, NonFungibleId) {
            let bucket = match request {
                // Handle user registration request
                ///////////////////////////////////
                RegistrationRequest::User { role, kyc_proof } => {
                    let user_resource = match &role {
                        UserRole::Advertiser(_) => self.rsa.advertiser_resource,
                        UserRole::AdSlotProvider(_) => self.rsa.ad_slot_provider_resource,
                        UserRole::AdBroker(AdBroker { fee_ratio, .. }) => {
                            assert!(
                                *fee_ratio >= dec!(0) && *fee_ratio <= dec!(1),
                                "broker_fee_ratio must be in range [0..1]"
                            );
                            self.rsa.ad_broker_resource
                        }
                    };

                    // Get an optional KYC token address
                    let kyc_token_address = kyc_proof.and_then(|kyc_proof| self.validate_kyc(kyc_proof));

                    // Mint a new user NFR
                    self.minter.mint_non_fungible(user_resource, User::new(role, kyc_token_address))
                }

                // Handle AdSlot registration request
                /////////////////////////////////////
                RegistrationRequest::AdSlot { size_constraints, tags, owner_user_badge, approved_broker_user_ids } => {
                    assert!(!approved_broker_user_ids.is_empty(), "Field approved_broker_user_ids cannot be empty");
                    self.assert_brokers_exist(approved_broker_user_ids.iter());
                    let user_id = get_user_id(self.rsa.ad_slot_provider_resource, owner_user_badge);
                    // Mint a new AdSlot NFR
                    self.minter.mint_non_fungible(
                        self.rsa.ad_slot_resource,
                        AdSlot::new(size_constraints, tags, user_id, approved_broker_user_ids),
                    )
                }

                // Handle Ad registration request
                /////////////////////////////////
                RegistrationRequest::Ad {
                    media,
                    link_url,
                    hover_text,
                    cost_per_click,
                    tags,
                    size_constraints,
                    owner_user_badge,
                    ad_broker_user_id,
                    max_cost_per_day,
                    budget,
                } => {
                    assert!(cost_per_click > dec!(0), "Field cost_per_click must be > 0");
                    // Make sure the referenced broker actually exists
                    self.assert_brokers_exist(vec![&ad_broker_user_id]);
                    // Take note of the budget amount
                    let budget_amount = budget.amount();
                    // And transfer the payment into the funds of this component
                    self.funds.put(budget);

                    // Finally, get the advertiser's ID and mint the Ad NFR
                    let advertiser_user_id = get_user_id(self.rsa.advertiser_resource, owner_user_badge);
                    self.minter.mint_non_fungible(
                        self.rsa.ad_resource,
                        Ad {
                            media,
                            link_url,
                            hover_text,
                            cost_per_click,
                            tags,
                            size_constraints,
                            advertiser_user_id,
                            ad_broker_user_id,
                            max_cost_per_day,
                            budget: budget_amount,
                        },
                    )
                }
            };

            // Mint a new Ad NFR
            let non_fungible_id = bucket.non_fungible_id();
            (bucket, non_fungible_id)
        }

        /// Allows an ad broker to create a new invoice. As invoices are likely to contain hundreds if not thousand of
        /// items, they are not well suited to be represented by non fungible resources. Instead they are represented by
        /// a special Invoice component.
        ///
        /// # Arguments
        /// - `ad_broker_user_badge` - A proof containing the badge of an ad broker user
        ///
        /// # Returns
        /// 1. The component address of the newly created invoice component
        /// 2. A struct containing all relevant resource addresses that are used by the invoice component
        ///
        /// **Access control:** Can only be called by ad broker users. According proof must be passed by intent.
        pub fn create_invoice(&self, ad_broker_user_badge: Proof) -> (ComponentAddress, InvoiceAddresses) {
            // Verify the caller is an ad broker user
            let ad_broker_user_badge = ad_broker_user_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(self.rsa.ad_broker_resource, dec!(1)))
                .expect("Invalid proof of being an AdBroker provided");

            // Get the user data from the proof
            let user = ad_broker_user_badge.non_fungible::<User>().data();
            // Determine the fee ratio
            let broker_fee_ratio = match user.role {
                UserRole::AdBroker(AdBroker { fee_ratio, .. }) => fee_ratio,
                _ => panic!("Invalid execution path"),
            };

            // Create a new invoice config
            let invoice_config = InvoiceConfig::new(
                self.rsa.clone(),
                ad_broker_user_badge.non_fungible_id(),
                2 * 24 * 7, // confirmation_period in epochs (~1 week with an assumed epoch duration of 30m)
                broker_fee_ratio,
            );

            // Instantiate a new invoice component
            InvoiceComponent::instantiate_global(
                invoice_config,
                Runtime::actor().as_component().0,
                self.minter.mint(self.rsa.access_badge_resource, dec!(1)),
                self.arbitration_dao_addresses.dao_system_admin_badge_resource,
            )
        }

        /// Claims a specified amount of an ad's budget, withdrawing it from this component's vault. The budget field
        /// of the Ad NFR will be updated accordingly.
        ///
        /// This method will be called by an Invoice component when an ad broker adds new AdCost items to the invoice.
        ///
        /// # Arguments
        /// - `ad_id` - The ID of the ad for which the budget is claimed
        /// - `amount` - The amount to claim. This must not be higher than the ad's budget!
        ///
        /// # Return
        /// A result containing a bucket with the claimed funds or an [BudgetExceededError] if the ad budget was
        /// exceeded.
        ///
        /// **Access control:** A access badge of this component must be provided. This badge is only distributed to
        /// Invoice components, thus only Invoice components can call this method.
        pub fn claim_ad_budget(
            &mut self,
            ad_id: NonFungibleId,
            amount: Decimal,
        ) -> Result<Bucket, BudgetExceededError> {
            // Get the Ad NFR
            let rm = borrow_resource_manager!(self.rsa.ad_resource);
            let mut ad: Ad = rm.get_non_fungible_data(&ad_id);

            // Reduce the ad's budget. If the Ad has an insufficient budget, immediately return an error
            ad.reduce_budget(amount)?;

            // Save the ad with it's reduced budget
            self.minter.update_non_fungible(self.rsa.ad_resource, &ad_id, ad);

            // Take the requested funds out of this component's vault and return them
            Ok(self.funds.take(amount))
        }

        /// Deposits funds to the benefit of an Ad's budget into this component.
        ///
        /// # Arguments
        /// - `ad_proof` - Proof of the Ad to the budget of which the supplied funds should be added
        /// - `funds` - The funds by which the Ad's budget should be increased
        ///
        /// **Access control:** Can be called by anyone who is in possession of an Ad NFR.
        pub fn deposit_ad_budget(&mut self, ad_proof: Proof, funds: Bucket) {
            // Verify the caller owns the ad for which funds should be deposited
            let ad_proof = ad_proof
                .validate_proof(ProofValidationMode::ValidateContainsAmount(self.rsa.ad_resource, dec!(1)))
                .expect("Invalid AdProof provided");
            // Load the Ad data
            let mut ad: Ad = ad_proof.non_fungible().data();
            // Increase the budget
            ad.increase_budget(funds.amount());
            // Update the modified Ad NFR
            self.minter.update_non_fungible(self.rsa.ad_resource, &ad_proof.non_fungible_id(), ad);
            // Put the funds into this component's vault
            self.funds.put(funds);
        }

        /// Redeposit funds to the benefit of an Ad's budget into this component.
        ///
        /// This method will be called by an Invoice component when the invoice has been rejected and user funds must
        /// be returned back from the Invoice component to the RadSense component.
        ///
        /// # Arguments
        /// - `ad_id` - The ID of the Ad to the budget of which the funds will be redeposited. The budget field of the
        /// Ad will be updated accordingly.
        /// - `funds` - The funds that will be redeposited
        ///
        /// **Access control:** Can only be called by Invoice components. The access badge of the RadSense component
        /// must be provided.
        pub fn redeposit_ad_budget(&mut self, ad_id: NonFungibleId, funds: Bucket) {
            // Update the ad's budget and save it to the ledger
            self.minter.modify_non_fungible_data(self.rsa.ad_resource, &ad_id, |ad: &mut Ad| {
                ad.increase_budget(funds.amount());
            });
            // Put the funds into this component's vault
            self.funds.put(funds);
        }

        /// Creates a new vote (via the VotingSystem of the arbitration DAO) to either
        /// - reject the invoice as invalid, in which case all advertisers must reclaim their funds and the ad broker
        /// must create a corrected invoice, or
        /// - accept it as valid, deeming any objections as minor or insubstantial. In this case the invoice will be
        /// "force" accepted and ad slot providers can subsequently claim their earnings.
        ///
        /// # Arguments
        /// - `invoice_address` - The component address of the invoice for which the arbitration vote should be created
        ///
        /// # Return
        /// A struct that represents the arbitration vote
        ///
        /// **Access control:** Can only be called by Invoice components. The access badge of the RadSense component
        /// must be provided.
        pub fn create_invoice_arbitration_vote(&self, invoice_address: ComponentAddress) -> Vote {
            let mut options = HashMap::new();

            // This hack is needed because of a bug in Scrypto
            let component_address_repo: ComponentAddressRepoComponent = self.component_address_repo.into();
            let invoice_address_lookup = component_address_repo.create_lookup(invoice_address);

            // Insert an option that, when chosen by the arbitrator's vote, will force accept the invoice
            options.insert(
                "force_accept_invoice".to_owned(),
                VoteOption {
                    description: Some("Ignore all objections and force accept the invoice".to_owned()),
                    code_executions: vec![CodeExecution::MethodCall {
                        component: invoice_address_lookup.clone(),
                        method: "force_accept_invoice".to_string(),
                        args: vec![],
                        required_badges: vec![self.rsa.access_badge_resource],
                    }],
                    requirement: WinRequirement::AbsolutRatio(dec!("0.5000000001")),
                },
            );

            // Insert an option that, when chosen by the arbitrator's vote, will reject the invoice
            options.insert(
                "reject_invoice".to_owned(),
                VoteOption {
                    description: Some("Sustain the objections and reject the invoice".to_owned()),
                    code_executions: vec![CodeExecution::MethodCall {
                        component: invoice_address_lookup,
                        method: "reject_invoice".to_string(),
                        args: vec![],
                        required_badges: vec![self.rsa.access_badge_resource],
                    }],
                    requirement: WinRequirement::Fallback,
                },
            );

            // Create a vote config passing in the two options
            let vote_config = VoteConfig {
                name: format!("Arbitrate invoice {invoice_address}"),
                description: None,
                options,
                evaluation_settings: EvaluationSettings {
                    voting_deadline: VotingDeadline::SoftEpochDeadline(Runtime::current_epoch() + 100),
                    allow_multiple_winning_options: false,
                },
                voting_power_resource: self.arbitration_dao_addresses.membership_resource,
            };

            // Get the VotingSystem component and create the vote
            let vs: VotingSystemComponent = self.arbitration_dao_addresses.voting_system_component.into();
            self.arbitration_dao_admin_badge.authorize(|| vs.create_vote(vote_config))
        }

        /// Adds the given arbitrator tracking API URL to the list of arbitrator tracking API urls.
        ///
        /// # Arguments
        /// - `arbitrator_tracking_api_url` - A URL that point to an arbitrators tracking API
        ///
        /// **Access control:** this method requires the arbitration DAO's admin badge for authorization. To invoke it,
        /// create an appropriate proposal and execute it once it has been approved.
        pub fn add_arbitrator_tracking_api_url(&mut self, arbitrator_tracking_api_url: String) {
            self.arbitrator_tracking_api_urls.insert(arbitrator_tracking_api_url);
        }

        /// Creates a general purpose proposal by which changes to the arbitration DAO can be effected (e.g. adding or
        /// removing members).
        ///
        /// # Arguments:
        /// - `name` - A short name for the proposal
        /// - `description` - An optional and possibly longer description of the proposal
        /// - `code_executions` - Code executions that should be run if the proposal is accepted
        ///
        /// # Return
        /// A struct representing the proposal
        ///
        /// **Access control:** This method can only be called by members of the arbitration dao. Proof of a membership
        /// badge must be provided via the AuthZone.
        pub fn create_proposal(
            &self,
            name: String,
            description: Option<String>,
            code_executions: Vec<CodeExecution>,
        ) -> Vote {
            let vs: VotingSystemComponent = self.arbitration_dao_addresses.voting_system_component.into();

            self.arbitration_dao_admin_badge.authorize(|| {
                vs.create_proposal(
                    name,
                    description,
                    VotingDeadline::HardEpochDeadline(Runtime::current_epoch() + 100),
                    WinRequirement::AbsolutRatio(dec!("0.6666666667")),
                    code_executions,
                    self.arbitration_dao_addresses.membership_resource,
                )
            })
        }

        /// Validates the given `kyc_proof`. The proof must be of a KYC resource that was specified when the RadSense
        /// component was instantiated.
        ///
        /// # Arguments
        /// - `kyc_proof` - Proof of an accepted KYC resource
        ///
        /// # Returns
        /// An Option of either some NonFungibleAddress if the KYC proof could be validated or None if the KYC feature
        /// is not enabled.
        ///
        /// **Panics** if the provided proof is not of a supported KYC resource.
        fn validate_kyc(&self, kyc_proof: Proof) -> Option<NonFungibleAddress> {
            let kyc_resources = &self.rsa.kyc_resources; // KYC not supported, return None
            if kyc_resources.is_empty() {
                None
            } else {
                let kyc_proof = kyc_proof
                    .validate_proof(ProofValidationMode::ValidateResourceAddressBelongsTo(kyc_resources.to_owned()))
                    .expect("Invalid KYC Proof provided");
                assert_eq!(kyc_proof.amount(), dec!(1), "Proof of a single KYC token must be provided");
                Some(NonFungibleAddress::new(kyc_proof.resource_address(), kyc_proof.non_fungible_id()))
            }
        }

        /// Asserts that the ad brokers referenced by the supplied ID iterator exist.
        ///
        /// # Arguments
        /// - `broker_ids` - An iterator of broker IDs
        fn assert_brokers_exist<'a, I>(&self, broker_ids: I)
        where
            I: IntoIterator<Item = &'a NonFungibleId>,
        {
            let rm = borrow_resource_manager!(self.rsa.ad_broker_resource);
            for broker_id in broker_ids {
                assert!(rm.non_fungible_exists(broker_id), "The referenced broker does not exist: {}", broker_id);
            }
        }
    }
}

/// A struct that describes a registration request for either a user, an ad or an ad slot.
#[derive(TypeId, Describe, Encode, Decode, Debug)]
pub enum RegistrationRequest {
    /// A request to register a new user
    User {
        /// The role of the user
        role: UserRole,
        /// An optional proof of a KYC token that the user possesses.
        /// Other users will likely trust a user more if they have provide a KYC proof.
        kyc_proof: Option<Proof>,
    },
    /// A request to register a new ad slot
    AdSlot {
        /// Size constraints that describe the ad slot. These constraints must be taken into account by ad brokers when
        /// they assign ads to this slot.
        size_constraints: SizeConstraints,
        /// Tags that describe the ad slot (i.e. the website on which it lives). This information may be used by ad
        /// brokers when assigning ads to this slot.
        tags: Vec<String>,
        /// Proof of an ad slot provider badge. The ID of the ad slot provider user will be associated with the new
        /// ad slot.
        owner_user_badge: Proof,
        /// A list of IDs of ad brokers that are allowed to assign ads to the new ad slot
        approved_broker_user_ids: Vec<NonFungibleId>,
    },
    /// A request to register a new ad
    Ad {
        /// A media item. This media item will be rendered in an ad slot
        media: Media,
        /// A URL to which users will be sent if they click the ad. This will typically be a landing page such as the
        /// advertisers homepage or their shopping site.
        link_url: String,
        /// A text that will be displayed when hovering over the media
        hover_text: String,
        /// The cost the advertiser is willing to pay each time a user clicks their ad.
        cost_per_click: Decimal,
        /// Tags that describe the ad and help ad brokers target it to users.
        tags: Vec<String>,
        /// Size constraints that describe the ad and must be taken into consideration by ad brokers when they assign
        /// the ad to ad slots
        size_constraints: SizeConstraints,
        /// Proof of a advertiser badge. The ad will be associated with this advertiser.
        owner_user_badge: Proof,
        /// The ID of an ad broker that is allowed to assign this ad to ad slots. This ad broker is also authorized to
        /// send the advertisers invoices that settle on the Radix network.
        ad_broker_user_id: NonFungibleId,
        /// The maximum budget per day
        max_cost_per_day: Decimal,
        /// Funding for this ad. This budget can also be increased later
        budget: Bucket,
    },
}

/// A struct grouping together all the relevant resource addresses of the RadSense component.
#[derive(TypeId, Describe, Encode, Decode, Clone, Debug)]
pub struct RadSenseAddresses {
    /// The address of the AdSlot NFRs
    pub ad_slot_resource: ResourceAddress,
    /// The address of the Ad NFRs
    pub ad_resource: ResourceAddress,
    /// The address of the Advertiser user NFRs
    pub advertiser_resource: ResourceAddress,
    /// The address of the AdSlotProvider user NFRs
    pub ad_slot_provider_resource: ResourceAddress,
    /// The address of the AdBroker user NFRs
    pub ad_broker_resource: ResourceAddress,
    /// The access badge resource that is required to invoke some of this component's methods and some methods of
    /// Invoice components
    pub access_badge_resource: ResourceAddress,
    /// A set of resource addresses that are accepted as KYC badges by this component
    pub kyc_resources: BTreeSet<ResourceAddress>,
}

/// A user of RadSense
#[derive(NonFungibleData, new, Debug)]
pub struct User {
    /// The role of the user: advertiser, ad slot provider or ad broker
    role: UserRole,
    /// An optional address of a KYC NFR. If this is present, the user is KYCed.
    kyc_token: Option<NonFungibleAddress>,
}

/// The role a user of the RadSense component takes on
#[derive(TypeId, Describe, Encode, Decode, new, Debug)]
pub enum UserRole {
    /// Users with this role are advertisers that want to advertise their products or services and do so by
    /// publishing ads to the RadSense component.
    Advertiser(Advertiser),
    /// Users with this role have websites on which ads may be displayed. They can register ad slots for all those
    /// spaces on their websites where ads should be displayed.
    AdSlotProvider(AdSlotProvider),
    /// Users with this role act as intermediaries between advertisers and ad slot providers. They have the important
    /// task of assigning ads to ad slots.
    AdBroker(AdBroker),
}

/// The data of an advertiser
#[derive(TypeId, Describe, Encode, Decode, new, Debug)]
pub struct Advertiser {
    /// An optional URL to a tracking API that is hosted by the advertiser. This API will invoked every time one of
    /// their ads is displayed in an ad slot or clicked on. Advertisers can provide this URL if they want to track
    /// how often and where their ads are displayed and how often they get clicked. They may also want to provide this
    /// so that they can track their costs and check invoices created by the ad broker. However, if they trust their
    /// broker enough, they do  not have to provide an URL.
    tracking_api_url: Option<String>,
}

/// The data of an ad slot provider
#[derive(TypeId, Describe, Encode, Decode, new, Debug)]
pub struct AdSlotProvider {
    /// An optional URL to a tracking API that is hosted by the ad slot provider. This API will invoked every time an
    /// ad is displayed in one of their ad slots or clicked on. Ad slot providers can provide this URL if they want to
    /// track how often ads are displayed in their ad slots. This information is necessary for calculating the revenue
    /// their ad slots have generated if they would like to check their ad broker's invoices. If they trust their
    /// brokers, they do not have to provide an URL.
    tracking_api_url: Option<String>,
}

/// The data of an ad broker
#[derive(TypeId, Describe, Encode, Decode, new, Debug)]
pub struct AdBroker {
    /// The URL to the operational API of the ad broker. This API will be invoked every time an ad slot is rendered and
    /// an ad must be selected that should be displayed in this ad slot.
    broker_api_url: String,
    /// The URL to the tracking API of the ad broker. As with advertisers and ad slot providers this API will be invoked
    /// every time an ad has been displayed in an ad slot or was clicked. The ad broker must provide this URL because
    /// it is their responsibility to keep track of ad costs and ad slot revenues and create invoices for their users
    /// (advertisers and ad slot providers).
    tracking_api_url: String,
    /// The fee ratio the ad broker is deducting from the payments of advertisers before sending it along to ad slot
    /// providers. Must be in range [0,1].
    fee_ratio: Decimal,
}

/// Non fungible data describing an Advertisement
#[derive(NonFungibleData, Debug)]
pub struct Ad {
    /// A media item. This media item will be rendered in an ad slot
    media: Media,
    /// A URL to which users will be sent if they click the ad. This will typically be a landing page such as the
    /// advertisers homepage or their shopping site.
    link_url: String,
    /// A text that will be displayed when hovering over the media
    hover_text: String,
    /// The cost the advertiser is willing to pay each time a user clicks their ad.
    cost_per_click: Decimal,
    /// Tags that describe the ad and help ad brokers target it to users.
    tags: Vec<String>,
    /// Size constraints that describe the ad and must be taken into consideration by ad brokers when they assign
    /// the ad to ad slots
    size_constraints: SizeConstraints,
    /// The ID of the advertiser user that created this ad
    advertiser_user_id: NonFungibleId,
    /// The ID of an ad broker that is allowed to assign this ad to ad slots. This ad broker is also authorized to
    /// send the advertisers invoices that settle on the Radix network.
    ad_broker_user_id: NonFungibleId,
    /// The maximum budget per day
    max_cost_per_day: Decimal,
    /// The budget of this ad. This value will be updated when ad brokers create an invoice and reference this ad on it.
    /// Ad brokers must keep track of this budget when they assign this ad to ad slots. It is their responsibility to
    /// not exceed this budget.
    #[scrypto(mutable)]
    budget: Decimal,
}

impl Ad {
    pub fn advertiser_user_id(&self) -> &NonFungibleId {
        &self.advertiser_user_id
    }
    pub fn ad_broker_user_id(&self) -> &NonFungibleId {
        &self.ad_broker_user_id
    }

    /// Increases the ad's budget by the given `amount`
    pub fn increase_budget(&mut self, amount: Decimal) {
        self.budget += amount;
    }

    /// Reduces the ad's budget by the given `amount`. If this would exceed the budget an error is returned.
    pub fn reduce_budget(&mut self, amount: Decimal) -> Result<(), BudgetExceededError> {
        if self.budget >= amount {
            self.budget -= amount;
            Ok(())
        } else {
            Err(BudgetExceededError)
        }
    }
}

/// Describes some media item
#[derive(TypeId, Describe, Encode, Decode, new, Debug)]
pub enum Media {
    /// The media item is an image
    Image {
        /// The source URL where the image is hosted
        source_url: String,
    },
    /// The media item is a video
    Video {
        /// The source URL where the video is hosted
        source_url: String,
        /// The length of the video in seconds
        length_seconds: u16,
    },
}

/// Error representing that an ad's budget has been exceeded
#[derive(TypeId, Describe, Encode, Decode, Debug)]
pub struct BudgetExceededError;

/// Non fungible data describing an ad slot
#[derive(NonFungibleData, new, Debug)]
pub struct AdSlot {
    /// Size constraints that describe the ad slot. These constraints must be taken into account by ad brokers when
    /// they assign ads to this slot.
    size_constraints: SizeConstraints,
    /// Tags that describe the ad slot (i.e. the website on which it lives). This information may be used by ad
    /// brokers when assigning ads to this slot.
    tags: Vec<String>,
    /// The ID of the ad slot provider user that created this ad slot
    ad_slot_provider_user_id: NonFungibleId,
    /// A list of IDs of ad brokers that are allowed to assign ads to the new ad slot
    approved_broker_user_ids: Vec<NonFungibleId>,
}

impl AdSlot {
    pub fn ad_slot_provider_user_id(&self) -> &NonFungibleId {
        &self.ad_slot_provider_user_id
    }
    pub fn approved_broker_user_ids(&self) -> &Vec<NonFungibleId> {
        &self.approved_broker_user_ids
    }
}

/// Describes a size constraint of an ad or an ad slot
#[derive(TypeId, Describe, Encode, Decode, new, Debug)]
pub enum SizeConstraints {
    /// The ad or slot has a fixed size
    Fixed {
        /// The width in px
        width: u16,
        /// The height in px
        height: u16,
    },
    /// The ad or slot has a flexible size. An ad for example could be scaled in a given size range as long as a
    /// certain aspect ratio is maintained.
    Flexible {
        /// The minimum width in px
        min_width: u16,
        /// The minimum height in px
        min_height: u16,
        /// The maximum width in px
        max_width: u16,
        /// The maximum height in px
        max_height: u16,
        /// An aspect ratio that must be maintained. E.g.: "16x9"
        aspect_ratio: String,
    },
}

/// Helper method for validating a user badge and extracting the user ID
///
/// # Arguments
/// - `required_user_resource` - The resource against which the provided `user_badge` should be validated
/// - `user_badge` - Proof of a user badge that should be validated and from which the user ID should be taken
///
/// # Returns
/// The user ID which is the NonFungibleId of the provided `user_badge` proof
///
/// **Panics** if the proof is invalid
fn get_user_id(required_user_resource: ResourceAddress, user_badge: Proof) -> NonFungibleId {
    user_badge
        .validate_proof(ProofValidationMode::ValidateContainsAmount(required_user_resource, dec!(1)))
        .expect("Invalid user badge")
        .non_fungible_id()
}

#[cfg(test)]
mod test {
    use super::*;
    use scrypto::values::ScryptoValue;

    /// A test that generates examples for `RegistrationRequest`s. These can be used when building manifests.
    #[test]
    fn create_register_ad_broker_request() {
        let mut requests = HashMap::new();

        requests.insert(
            "Register Advertiser",
            RegistrationRequest::User {
                role: UserRole::Advertiser(Advertiser {
                    tracking_api_url: Some("http://127.0.0.1:5173/mocks/tracking_api/Advertiser".to_owned()),
                }),
                kyc_proof: None,
            },
        );
        requests.insert(
            "Register AdSlotProvider",
            RegistrationRequest::User {
                role: UserRole::AdSlotProvider(AdSlotProvider {
                    tracking_api_url: Some("http://127.0.0.1:5173/mocks/tracking_api/AdSlotProvider".to_owned()),
                }),
                kyc_proof: None,
            },
        );
        requests.insert(
            "Register AdBroker",
            RegistrationRequest::User {
                role: UserRole::AdBroker(AdBroker {
                    broker_api_url: "http://127.0.0.1:5173/mocks/broker_api".to_owned(),
                    tracking_api_url: "http://127.0.0.1:5173/mocks/tracking_api/AdBroker".to_owned(),
                    fee_ratio: dec!("0.1"),
                }),
                kyc_proof: None,
            },
        );

        requests.insert(
            "Register Ad",
            RegistrationRequest::Ad {
                media: Media::new_image("https://api.ociswap.com/icons/128x128/ociswap.png".to_string()),
                link_url: "https://ociswap.com/".to_string(),
                hover_text: "SWAP THE MEOW-Y WAY!".to_string(),
                cost_per_click: dec!(1),
                tags: ["finance", "defi", "exchange"].map(String::from).to_vec(),
                size_constraints: SizeConstraints::new_fixed(128, 128),
                owner_user_badge: Proof(0),
                ad_broker_user_id: NonFungibleId::from_u32(1),
                max_cost_per_day: dec!(10),
                budget: Bucket(0),
            },
        );

        requests.insert(
            "Register AdSlot",
            RegistrationRequest::AdSlot {
                size_constraints: SizeConstraints::Fixed { width: 128, height: 128 },
                tags: ["finance", "news", "bitcoin"].map(String::from).to_vec(),
                owner_user_badge: Proof(42),
                approved_broker_user_ids: vec![NonFungibleId::from_u32(1)],
            },
        );

        for (request_name, request) in requests {
            println!("{request_name}: {}", ScryptoValue::from_typed(&request));
        }
    }
}
