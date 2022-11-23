use radix_engine::engine::{RejectionError, RuntimeError};
use radix_engine::ledger::*;
use radix_engine::transaction::{TransactionOutcome, TransactionReceipt, TransactionResult};
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::signing::EcdsaSecp256k1PrivateKey;

use rad_sense::dao_kit::simple_dao_system::DaoSystemAddresses;
use rad_sense::dao_kit::voting_system::{Vote, VoteState, VotingPower};
use rad_sense::invoice::{InvoiceAddresses, InvoiceItem, InvoiceState};
use rad_sense::rad_sense::{
    AdBroker, AdSlotProvider, Advertiser, Media, RadSenseAddresses, RegistrationRequest, SizeConstraints, UserRole,
};

/// Simulates some example usage of RadSense
#[test]
fn simulate_example_transaction_flow() {
    // Create a testing environment
    let mut store: TypedInMemorySubstateStore = TypedInMemorySubstateStore::with_bootstrap();
    let mut env = TestEnv::new(&mut store);

    // Create an account for John
    let johns_account = env.new_account();

    // Instantiate the RadSense component
    let initial_arbitrators = vec![("John".to_owned(), johns_account.account_component)];
    let arbitrator_tracking_api_urls = ["https://some-api-url.io".to_owned()].into();

    env.instantiate_rad_sense(&johns_account, initial_arbitrators, arbitrator_tracking_api_urls).unwrap();

    // Register an AdBroker
    let user_role = UserRole::AdBroker(AdBroker::new(
        "https://broker.api".to_string(),
        "https://broker-tracking.api".to_string(),
        dec!("0.1"),
    ));
    let ad_broker_id = env.register_user(&johns_account, user_role).unwrap().1;

    // Register an Advertiser
    let user_role = UserRole::Advertiser(Advertiser::new(Some("https://advertiser-tracking.api".to_string())));
    let advertiser_id = env.register_user(&johns_account, user_role).unwrap().1;

    // Register an AdSlotProvider
    let user_role =
        UserRole::AdSlotProvider(AdSlotProvider::new(Some("https://adslotprovider-tracking.api".to_string())));
    let ad_slot_proivder_id = env.register_user(&johns_account, user_role).unwrap().1;

    // Register an Ad
    let ad = AdConfig {
        image_url: "https://api.ociswap.com/icons/128x128/ociswap.png".to_string(),
        link_url: "https://ociswap.com/".to_string(),
        hover_text: "SWAP THE MEOW-Y WAY!".to_string(),
        cost_per_click: dec!(1),
        tags: ["dex", "exchange", "defi"].map(String::from).to_vec(),
        size_constraints: SizeConstraints::Flexible {
            min_width: 64,
            min_height: 64,
            max_width: 128,
            max_height: 128,
            aspect_ratio: "1x1".to_string(),
        },
        owner_user_id: advertiser_id.clone(),
        ad_broker_user_id: ad_broker_id.clone(),
        max_cost_per_day: dec!(50),
        budget: dec!(1000),
    };
    let ad_id = env.register_ad(&johns_account, ad).unwrap().1;

    let ad_slot = AdSlotConfig {
        size_constraints: SizeConstraints::Flexible {
            min_width: 50,
            min_height: 50,
            max_width: 500,
            max_height: 500,
            aspect_ratio: "1x1".to_string(),
        },
        tags: ["finance", "crypto", "bitcoin"].map(String::from).to_vec(),
        owner_user_id: ad_slot_proivder_id,
        approved_broker_user_ids: vec![ad_broker_id.clone()],
    };
    let ad_slot_id = env.register_ad_slot(&johns_account, ad_slot).unwrap().1;

    let (invoice_address, _invoice_addresses) = env.create_invoice(&johns_account, ad_broker_id.clone()).unwrap();

    let invoice_items = vec![
        InvoiceItem::AdCost { ad_id: ad_id.clone(), amount: dec!(500) },
        InvoiceItem::AdCost { ad_id: ad_id.clone(), amount: dec!(400) },
        InvoiceItem::AdSlotRevenue { ad_slot_id: ad_slot_id.clone(), amount: dec!(900) },
    ];
    let (ad_cost_ids, ad_slot_revenue_ids) =
        env.add_invoice_items(&johns_account, ad_broker_id.clone(), invoice_address, invoice_items).unwrap();
    assert_eq!(ad_cost_ids.len(), 2);
    assert_eq!(ad_slot_revenue_ids.len(), 1);

    env.publish_invoice(
        &johns_account,
        ad_broker_id.clone(),
        invoice_address,
        "invoice-proof.pdf".to_owned(),
        "12345".to_owned(),
    )
    .unwrap();

    env.confirm_ad_costs(&johns_account, invoice_address, ad_cost_ids, ad_id.clone()).unwrap();

    env.mark_invoice_as_accepted(&johns_account, invoice_address)
        .expect_err("Should fail, because the confirmation deadline has not passed yet");
    env.test_runner.set_current_epoch(2 * 24 * 7 + 1); // Advance the epoch to after the invoice's confirmation deadline

    let invoice_state = env.mark_invoice_as_accepted(&johns_account, invoice_address).unwrap();
    assert!(matches!(invoice_state, InvoiceState::Accepted { .. }));

    // Claim the ad slot earnings and assert that it amounts to 810 XRD (90% of the payment; 10% broker fee deducted)
    env.claim_ad_slot_earnings(
        &johns_account,
        invoice_address,
        ad_slot_revenue_ids.clone(),
        ad_slot_id.clone(),
        dec!(810),
    )
    .unwrap();
    env.claim_fees(&johns_account, invoice_address, ad_broker_id, dec!(90)).unwrap();
}

pub struct TestEnv<'s, S: ReadableSubstateStore + WriteableSubstateStore> {
    pub test_runner: TestRunner<'s, S>,
    pub admin_account: Account,
    pub package_address: PackageAddress,
    pub rad_sense_component: Option<ComponentAddress>,
    pub rsa: Option<RadSenseAddresses>,
    pub arbitration_dao_addresses: Option<DaoSystemAddresses>,
}

/// RadSense functions/methods
impl<'s, S: ReadableSubstateStore + WriteableSubstateStore> TestEnv<'s, S> {
    fn instantiate_rad_sense(
        &mut self,
        actor: &Account,
        initial_arbitrators: Vec<(String, ComponentAddress)>,
        arbitrator_tracking_api_urls: HashSet<String>,
    ) -> Result<(), TransactionError> {
        let kyc_resources: BTreeSet<ResourceAddress> = BTreeSet::new();
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_function(
                self.package_address,
                "RadSense",
                "instantiate_global",
                args!(initial_arbitrators, arbitrator_tracking_api_urls, kyc_resources),
            )
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        let output: (ComponentAddress, RadSenseAddresses, DaoSystemAddresses) = receipt.get_output(1)?;

        self.rad_sense_component = Some(output.0);
        self.rsa = Some(output.1);
        self.arbitration_dao_addresses = Some(output.2);

        Ok(())
    }

    fn register_user(
        &mut self,
        actor: &Account,
        user_role: UserRole,
    ) -> Result<(Bucket, NonFungibleId), TransactionError> {
        let request = RegistrationRequest::User { role: user_role, kyc_proof: None };
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(self.rad_sense_component.unwrap(), "register", args!(request))
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }

    fn register_ad(&mut self, actor: &Account, ad: AdConfig) -> Result<(Bucket, NonFungibleId), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account_by_ids(
                &BTreeSet::from([ad.owner_user_id.clone()]),
                self.rsa().advertiser_resource,
                actor.account_component,
            )
            .pop_from_auth_zone(|builder, proof_id| {
                builder
                    .withdraw_from_account_by_amount(ad.budget, RADIX_TOKEN, actor.account_component)
                    .take_from_worktop_by_amount(ad.budget, RADIX_TOKEN, |builder, bucket_id| {
                        builder.call_method(
                            self.rad_sense_component(),
                            "register",
                            args!(ad.into_create_request(Proof(proof_id), Bucket(bucket_id))),
                        )
                    })
            })
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(5)
    }

    fn register_ad_slot(
        &mut self,
        actor: &Account,
        ad_slot: AdSlotConfig,
    ) -> Result<(Bucket, NonFungibleId), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account_by_ids(
                &BTreeSet::from([ad_slot.owner_user_id.clone()]),
                self.rsa().ad_slot_provider_resource,
                actor.account_component,
            )
            .pop_from_auth_zone(|builder, proof_id| {
                builder.call_method(
                    self.rad_sense_component(),
                    "register",
                    args!(ad_slot.into_create_request(Proof(proof_id))),
                )
            })
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(3)
    }

    fn create_invoice(
        &mut self,
        actor: &Account,
        ad_broker_id: NonFungibleId,
    ) -> Result<(ComponentAddress, InvoiceAddresses), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account_by_ids(
                &BTreeSet::from([ad_broker_id]),
                self.rsa().ad_broker_resource,
                actor.account_component,
            )
            .pop_from_auth_zone(|builder, proof_id| {
                builder.call_method(self.rad_sense_component(), "create_invoice", args!(Proof(proof_id)))
            })
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(3)
    }
}

/// Invoice functions/methods
impl<'s, S: ReadableSubstateStore + WriteableSubstateStore> TestEnv<'s, S> {
    fn add_invoice_items(
        &mut self,
        actor: &Account,
        ad_broker_id: NonFungibleId,
        invoice_address: ComponentAddress,
        invoice_items: Vec<InvoiceItem>,
    ) -> Result<(Vec<NonFungibleId>, Vec<NonFungibleId>), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account_by_ids(
                &BTreeSet::from([ad_broker_id]),
                self.rsa().ad_broker_resource,
                actor.account_component,
            )
            .call_method(invoice_address, "add_invoice_items", args!(invoice_items))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(2)
    }

    fn publish_invoice(
        &mut self,
        actor: &Account,
        ad_broker_id: NonFungibleId,
        invoice_address: ComponentAddress,
        proof_document_url: String,
        proof_document_hash: String,
    ) -> Result<(), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account_by_ids(
                &BTreeSet::from([ad_broker_id]),
                self.rsa().ad_broker_resource,
                actor.account_component,
            )
            .call_method(invoice_address, "publish", args!(proof_document_url, proof_document_hash))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(2)
    }

    fn confirm_ad_costs(
        &mut self,
        actor: &Account,
        invoice_address: ComponentAddress,
        ad_cost_ids: Vec<NonFungibleId>,
        ad_id: NonFungibleId,
    ) -> Result<(), TransactionError> {
        let ad_cost_ids: HashSet<NonFungibleId> = ad_cost_ids.into_iter().collect();
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account_by_ids(&BTreeSet::from([ad_id]), self.rsa().ad_resource, actor.account_component)
            .pop_from_auth_zone(|builder, proof_id| {
                builder.call_method(invoice_address, "confirm_ad_costs", args!(ad_cost_ids, Proof(proof_id)))
            })
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(3)
    }

    fn mark_invoice_as_accepted(
        &mut self,
        actor: &Account,
        invoice_address: ComponentAddress,
    ) -> Result<InvoiceState, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(invoice_address, "mark_invoice_as_accepted_or_disputed", args!())
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }

    fn claim_ad_slot_earnings(
        &mut self,
        actor: &Account,
        invoice_address: ComponentAddress,
        ad_slot_revenue_ids: Vec<NonFungibleId>,
        ad_slot_id: NonFungibleId,
        expected_revenue: Decimal,
    ) -> Result<Bucket, TransactionError> {
        let ad_slot_revenue_ids: HashSet<NonFungibleId> = ad_slot_revenue_ids.into_iter().collect();
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account_by_ids(
                &BTreeSet::from([ad_slot_id]),
                self.rsa().ad_slot_resource,
                actor.account_component,
            )
            .pop_from_auth_zone(|builder, proof_id| {
                builder.call_method(
                    invoice_address,
                    "claim_ad_slot_earnings",
                    args!(ad_slot_revenue_ids, Proof(proof_id)),
                )
            })
            .assert_worktop_contains_by_amount(expected_revenue, RADIX_TOKEN)
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(3)
    }

    fn claim_fees(
        &mut self,
        actor: &Account,
        invoice_address: ComponentAddress,
        ad_broker_id: NonFungibleId,
        expected_fee: Decimal,
    ) -> Result<Bucket, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account_by_ids(
                &BTreeSet::from([ad_broker_id]),
                self.rsa().ad_broker_resource,
                actor.account_component,
            )
            .call_method(invoice_address, "claim_fees", args!())
            .assert_worktop_contains_by_amount(expected_fee, RADIX_TOKEN)
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(2)
    }
}

impl<'s, S: ReadableSubstateStore + WriteableSubstateStore> TestEnv<'s, S> {
    pub fn new(store: &'s mut S) -> Self {
        let mut test_runner = TestRunner::new(false, store);
        let (public_key, private_key, account_component) = test_runner.new_account();
        let admin_account = Account { public_key, private_key, account_component };

        let package_address = test_runner.compile_and_publish(this_package!());
        let rad_sense_component = None;
        let rsa = None;
        let arbitration_dao_addresses = None;

        Self { test_runner, admin_account, package_address, rad_sense_component, rsa, arbitration_dao_addresses }
    }

    pub fn new_account(&mut self) -> Account {
        let (public_key, private_key, account_component) = self.test_runner.new_account();
        Account { public_key, private_key, account_component }
    }

    #[allow(unused)]
    pub fn get_balance(&mut self, account: &Account, resource: ResourceAddress) -> Decimal {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(account.account_component, "balance", args!(resource))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![account.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.expect_commit_success();
        receipt.output(1)
    }

    fn rad_sense_component(&self) -> ComponentAddress {
        self.rad_sense_component.unwrap()
    }

    fn rsa(&self) -> &RadSenseAddresses {
        self.rsa.as_ref().unwrap()
    }
}

// Voting system methods
impl<'s, S: ReadableSubstateStore + WriteableSubstateStore> TestEnv<'s, S> {
    pub fn cast_vote(
        &mut self,
        actor: &Account,
        dao_system_addresses: &DaoSystemAddresses,
        vote_id: &NonFungibleId,
        option: String,
    ) -> Result<(Vote, Option<Bucket>), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account(dao_system_addresses.membership_resource, actor.account_component)
            .pop_from_auth_zone(|builder, proof_id| {
                builder.call_method(
                    dao_system_addresses.voting_system_component,
                    "cast_vote",
                    args!(vote_id.to_owned(), option, VotingPower::NonFungible(Proof(proof_id))),
                )
            })
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(3)
    }

    pub fn evaluate_vote(
        &mut self,
        actor: &Account,
        dao_system_addresses: &DaoSystemAddresses,
        vote_id: &NonFungibleId,
    ) -> Result<Vote, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(dao_system_addresses.voting_system_component, "evaluate_vote", args!(*vote_id))
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }

    pub fn implement_vote(
        &mut self,
        actor: &Account,
        dao_system_addresses: &DaoSystemAddresses,
        vote: &Vote,
    ) -> Result<Vote, TransactionError> {
        let winning_options = if let VoteState::Decided { winning_option_names, .. } = &vote.state {
            winning_option_names
        } else {
            panic!("Vote must be in state Decided")
        };
        let has_code_executions =
            vote.config.options.iter().any(|(option_name, option)| {
                winning_options.contains(option_name) && !option.code_executions.is_empty()
            });

        let mut manifest = ManifestBuilder::new(&NetworkDefinition::simulator());
        manifest
            .create_proof_from_account(dao_system_addresses.membership_resource, actor.account_component)
            .pop_from_auth_zone(|builder, proof_id| {
                builder.call_method(
                    dao_system_addresses.voting_system_component,
                    "implement_vote",
                    args!(vote.id, Proof(proof_id)),
                )
            });
        if has_code_executions {
            manifest.take_from_worktop(
                dao_system_addresses.code_execution_resource,
                |builder, code_execution_bucket_id| {
                    builder.call_method(
                        dao_system_addresses.code_execution_system_component,
                        "execute_code",
                        args!(Bucket(code_execution_bucket_id)),
                    )
                },
            );
        }
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest.build(), vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        let output: Result<(Vote, Option<Bucket>), TransactionError> = receipt.get_output(3);
        output.map(|(vote, _)| vote)
    }
}

pub trait GetOutput {
    fn get_output<T>(self, nth: usize) -> Result<T, TransactionError>
    where
        T: Decode + TypeId;
}

impl GetOutput for TransactionReceipt {
    fn get_output<T>(self, nth: usize) -> Result<T, TransactionError>
    where
        T: Decode + TypeId,
    {
        match self.result {
            TransactionResult::Commit(c) => match c.outcome {
                TransactionOutcome::Success(x) => {
                    let encoded = &x[nth][..];
                    let decoded = scrypto_decode::<T>(encoded).expect("Wrong instruction output type!");
                    Ok(decoded)
                }
                TransactionOutcome::Failure(err) => Err(TransactionError::TransactionFailure(err)),
            },
            TransactionResult::Reject(err) => Err(TransactionError::TransactionRejected(err.error)),
        }
    }
}

#[derive(Debug)]
pub enum TransactionError {
    TransactionFailure(RuntimeError),
    TransactionRejected(RejectionError),
}

pub struct Account {
    pub public_key: EcdsaSecp256k1PublicKey,
    pub private_key: EcdsaSecp256k1PrivateKey,
    pub account_component: ComponentAddress,
}

struct AdConfig {
    image_url: String,
    link_url: String,
    hover_text: String,
    cost_per_click: Decimal,
    tags: Vec<String>,
    size_constraints: SizeConstraints,
    owner_user_id: NonFungibleId,
    ad_broker_user_id: NonFungibleId,
    max_cost_per_day: Decimal,
    budget: Decimal,
}

impl AdConfig {
    fn into_create_request(self, owner_user_badge: Proof, budget: Bucket) -> RegistrationRequest {
        RegistrationRequest::Ad {
            media: Media::new_image(self.image_url),
            link_url: self.link_url,
            hover_text: self.hover_text,
            cost_per_click: self.cost_per_click,
            tags: self.tags,
            size_constraints: self.size_constraints,
            owner_user_badge,
            ad_broker_user_id: self.ad_broker_user_id,
            max_cost_per_day: self.max_cost_per_day,
            budget,
        }
    }
}

struct AdSlotConfig {
    size_constraints: SizeConstraints,
    tags: Vec<String>,
    owner_user_id: NonFungibleId,
    approved_broker_user_ids: Vec<NonFungibleId>,
}

impl AdSlotConfig {
    fn into_create_request(self, owner_user_badge: Proof) -> RegistrationRequest {
        RegistrationRequest::AdSlot {
            size_constraints: self.size_constraints,
            tags: self.tags,
            owner_user_badge,
            approved_broker_user_ids: self.approved_broker_user_ids,
        }
    }
}
