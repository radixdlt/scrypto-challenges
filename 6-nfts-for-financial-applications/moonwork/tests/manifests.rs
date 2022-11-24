use ::moonwork::dispute::*;
use radix_engine::ledger::*;
use radix_engine::model::ResourceManager;
use radix_engine::transaction::CommitResult;
use radix_engine::transaction::TransactionReceipt;
use radix_engine::types::*;
use scrypto::core::NetworkDefinition;
use scrypto::resource::Bucket;
use scrypto::resource::NonFungibleData;
use scrypto::resource::Proof;
use scrypto::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

pub fn claim_client_work_refund(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service: &MoonWorkService,
    client_account: ComponentAddress,
    client_public_key: EcdsaSecp256k1PublicKey,
) -> radix_engine::transaction::TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(service.moon_work_resources.client_badge, client_account)
        .pop_from_auth_zone(|builder, proof| {
            builder
                .call_method(
                    service.moon_work_component,
                    "claim_client_work_refund",
                    args!(Proof(proof)),
                )
                .call_method(
                    client_account,
                    "deposit_batch",
                    args!(Expression::entire_worktop()),
                )
        })
        .build();
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![client_public_key.into()]);
    receipt
}

pub fn remove_expired_promotions(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service: &MoonWorkService,
    promotion_service: &PromotionService,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(
            service.moon_work_resources.service_admin_badge,
            service.service_admin_component_address,
        )
        .call_method(
            promotion_service.component,
            "remove_expired_promotions",
            args!(),
        )
        .build();
    test_runner
        .execute_manifest_ignoring_fee(manifest, vec![service.service_admin_public_key.into()])
}

pub fn promote_contractor(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service: &MoonWorkService,
    promotion_service: &PromotionService,
    contractor_account: ComponentAddress,
    contractor_public_key: EcdsaSecp256k1PublicKey,
    work_resource_amount: u64,
    accolade_resource_amount: u64,
) -> radix_engine::transaction::TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_amount(
            work_resource_amount.into(),
            service.moon_work_resources.completed_work_resource,
            contractor_account,
        )
        .pop_from_auth_zone(|builder, work_completed_proof| {
            builder
                .create_proof_from_account_by_amount(
                    accolade_resource_amount.into(),
                    service.moon_work_resources.contractor_accolade_resource,
                    contractor_account,
                )
                .pop_from_auth_zone(|builder, accolades_proof| {
                    builder
                        .create_proof_from_account(
                            service.moon_work_resources.contractor_badge,
                            contractor_account,
                        )
                        .create_proof_from_auth_zone_by_amount(
                            dec!(1),
                            service.moon_work_resources.contractor_badge,
                            |builder, proof| {
                                builder
                                    .call_method(
                                        promotion_service.component,
                                        "promote_contractor",
                                        args!(
                                            Proof(work_completed_proof),
                                            Proof(accolades_proof),
                                            Proof(proof)
                                        ),
                                    )
                                    .drop_all_proofs()
                            },
                        )
                })
        })
        .build();
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![contractor_public_key.into()]);
    receipt
}

pub struct PromotionService {
    pub component: ComponentAddress,
    pub contractor_promotion: ResourceAddress,
}

pub fn create_promotion_service(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    resources: &MoonWorkResource,
    account_component: ComponentAddress,
    component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
) -> PromotionService {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(resources.service_admin_badge, account_component)
        .call_method(component, "create_promotion_service", args!())
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);

    receipt.expect_commit_success();

    let result = receipt.expect_commit();

    PromotionService {
        component: result.entity_changes.new_component_addresses[0],
        contractor_promotion: result.entity_changes.new_resource_addresses[0],
    }
}

pub fn create_work_to_complete_dispute(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service: &MoonWorkService,
    client_account: ComponentAddress,
    client_public_key: EcdsaSecp256k1PublicKey,
    contractor_account: ComponentAddress,
    contractor_public_key: EcdsaSecp256k1PublicKey,
    last_created_work_id: u64,
) -> radix_engine::transaction::TransactionReceipt {
    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();
    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();
    register_as_client(
        test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );
    register_as_contractor(
        test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );
    create_work_to_finish_work(
        test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        last_created_work_id + 1,
    );
    claim_contractor_compensation(
        test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );
    create_new_work(
        test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );
    let work_id = NonFungibleId::from_u64(last_created_work_id + 2);
    request_work(
        test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );
    accept_contractor_for_work(
        test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );
    create_new_dispute(
        test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id.clone(),
    );
    let dispute_id = NonFungibleId::from_u64(1);
    join_and_decide_dispute(
        test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );
    join_and_decide_dispute(
        test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );
    let receipt = complete_dispute(
        test_runner,
        &service,
        client_account,
        client_public_key,
        dispute_id,
    );

    receipt
}

pub fn cancel_dispute(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service: &MoonWorkService,
    badge: ResourceAddress,
    contractor_account: ComponentAddress,
    contractor_public_key: EcdsaSecp256k1PublicKey,
    dispute_id: NonFungibleId,
) -> radix_engine::transaction::TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(badge, contractor_account)
        .pop_from_auth_zone(|builder, proof| {
            builder
                .call_method(
                    service.work_components.dispute_component,
                    "cancel_dispute",
                    args!(dispute_id, Proof(proof)),
                )
                .drop_all_proofs()
        })
        .build();
    let receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![contractor_public_key.into()]);
    receipt
}

pub fn complete_dispute_as_admin(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service: &MoonWorkService,
    dispute_id: NonFungibleId,
    dispute_side: DisputeSide,
) -> radix_engine::transaction::TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(
            service.moon_work_resources.service_admin_badge,
            service.service_admin_component_address,
        )
        .call_method(
            service.work_components.dispute_component,
            "complete_dispute_as_admin",
            args!(dispute_id, dispute_side),
        )
        .drop_all_proofs()
        .build();
    let receipt = test_runner
        .execute_manifest_ignoring_fee(manifest, vec![service.service_admin_public_key.into()]);
    receipt
}

pub fn remove_work(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service: &MoonWorkService,
    account_component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    work_id: NonFungibleId,
) -> radix_engine::transaction::TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account_by_ids(
            &BTreeSet::from_iter([work_id]),
            service.work_components.work_resource,
            account_component,
        )
        .pop_from_auth_zone(|builder, work| {
            builder
                .call_method(
                    service.work_components.work_component,
                    "remove_work",
                    args!(Proof(work)),
                )
                .drop_all_proofs()
        })
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);
    receipt
}

pub fn complete_dispute(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service: &MoonWorkService,
    account: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    dispute_id: NonFungibleId,
) -> radix_engine::transaction::TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(service.moon_work_resources.client_badge, account)
        .pop_from_auth_zone(|builder, proof| {
            builder
                .call_method(
                    service.work_components.dispute_component,
                    "complete_dispute",
                    args!(dispute_id, Proof(proof)),
                )
                .drop_all_proofs()
        })
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);
    receipt
}

pub fn join_and_decide_dispute(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    badge: ResourceAddress,
    service: &MoonWorkService,
    dispute_id: NonFungibleId,
    dispute_side: DisputeSide,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(badge, account)
        .pop_from_auth_zone(|builder, proof| {
            builder
                .call_method(
                    service.work_components.dispute_component,
                    "join_and_decide_dispute",
                    args!(dispute_id, dispute_side, Proof(proof)),
                )
                .drop_all_proofs()
        })
        .build();

    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub struct MoonWorkService {
    pub service_admin_public_key: EcdsaSecp256k1PublicKey,
    pub service_admin_component_address: ComponentAddress,
    pub moon_work_resources: MoonWorkResource,
    pub moon_work_component: ComponentAddress,
    pub work_components: WorkComponents,
}

pub fn create_moon_work_service_with_work_category(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
) -> MoonWorkService {
    let (service_admin_public_key, _private_key, service_admin_component_address) =
        test_runner.new_account();
    let (moonwork_component, moonwork_resources) = create_moonwork_service(
        test_runner,
        service_admin_component_address,
        service_admin_public_key,
    );
    register_as_client(
        test_runner,
        moonwork_component,
        service_admin_component_address,
        service_admin_public_key,
        "admin",
    );
    let create_new_work_receipt = create_new_category(
        test_runner,
        moonwork_resources.service_admin_badge,
        service_admin_component_address,
        service_admin_public_key,
        moonwork_component,
    );
    let work_components = get_work_components(create_new_work_receipt.expect_commit());

    MoonWorkService {
        service_admin_public_key,
        service_admin_component_address,
        moon_work_resources: moonwork_resources,
        moon_work_component: moonwork_component,
        work_components,
    }
}

pub fn create_work_to_finish_work(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    client_account: ComponentAddress,
    contractor_account: ComponentAddress,
    contractor_username: &str,
    moon_work_resources: &MoonWorkResource,
    work_components: &WorkComponents,
    client_public_key: EcdsaSecp256k1PublicKey,
    contractor_public_key: EcdsaSecp256k1PublicKey,
    id: u64,
) {
    create_new_work(
        test_runner,
        client_account,
        client_public_key,
        moon_work_resources.client_badge,
        work_components,
    );
    request_work(
        test_runner,
        contractor_account,
        contractor_public_key,
        moon_work_resources.contractor_badge,
        work_components.work_component,
        NonFungibleId::from_u64(id),
    );
    accept_contractor_for_work(
        test_runner,
        work_components.work_component,
        moon_work_resources.client_badge,
        client_account,
        client_public_key,
        work_components.work_resource,
        NonFungibleId::from_bytes(contractor_username.as_bytes().to_vec()),
        NonFungibleId::from_u64(id),
    );
    finish_work(
        test_runner,
        moon_work_resources,
        client_account,
        client_public_key,
        contractor_account,
        contractor_public_key,
        NonFungibleId::from_u64(id),
        work_components,
    );
}

pub fn submit_document(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    badge: ResourceAddress,
    account: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    work_components: &WorkComponents,
    dispute_id: NonFungibleId,
    dispute_document_title: &str,
    dispute_document_url: &str,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(badge, account)
        .pop_from_auth_zone(|builder, client_proof| {
            builder
                .call_method(
                    work_components.dispute_component,
                    "submit_document",
                    args!(
                        dispute_id,
                        dispute_document_title,
                        dispute_document_url,
                        Proof(client_proof)
                    ),
                )
                .drop_all_proofs()
        })
        .build();

    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub fn create_new_dispute(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    badge: ResourceAddress,
    account: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    work_components: &WorkComponents,
    work_id: NonFungibleId,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(badge, account)
        .pop_from_auth_zone(|builder, client_proof| {
            builder
                .call_method(
                    work_components.dispute_component,
                    "create_new_dispute",
                    args!(work_id, work_components.work_resource, Proof(client_proof)),
                )
                .drop_all_proofs()
        })
        .build();

    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub fn claim_contractor_compensation(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    resources: &MoonWorkResource,
    moonwork_component: ComponentAddress,
    contractor_account: ComponentAddress,
    contractor_public_key: EcdsaSecp256k1PublicKey,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(resources.contractor_badge, contractor_account)
        .pop_from_auth_zone(|builder, contractor_proof| {
            builder
                .call_method(
                    moonwork_component,
                    "claim_contractor_compensation",
                    args!(Proof(contractor_proof)),
                )
                .call_method(
                    contractor_account,
                    "deposit_batch",
                    args!(Expression::entire_worktop()),
                )
        })
        .build();
    test_runner.execute_manifest_ignoring_fee(manifest, vec![contractor_public_key.into()])
}

pub fn finish_work(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    resources: &MoonWorkResource,
    client_account: ComponentAddress,
    client_public_key: EcdsaSecp256k1PublicKey,
    contractor_account: ComponentAddress,
    contractor_public_key: EcdsaSecp256k1PublicKey,
    work_id: NonFungibleId,
    work_components: &WorkComponents,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(resources.client_badge, client_account)
        .pop_from_auth_zone(|builder, client_proof| {
            builder
                .create_proof_from_account(resources.contractor_badge, contractor_account)
                .pop_from_auth_zone(|builder, contractor_proof| {
                    builder
                        .create_proof_from_account_by_ids(
                            &BTreeSet::from_iter([work_id]),
                            work_components.work_resource,
                            client_account,
                        )
                        .pop_from_auth_zone(|builder, work_proof| {
                            builder.call_method(
                                work_components.work_component,
                                "finish_work",
                                args!(
                                    Proof(work_proof),
                                    Proof(client_proof),
                                    Proof(contractor_proof)
                                ),
                            )
                        })
                })
                .drop_all_proofs()
        })
        .build();

    test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![client_public_key.into(), contractor_public_key.into()],
    )
}

pub fn accept_contractor_for_work(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    work_component: ComponentAddress,
    client_resource: ResourceAddress,
    client_account: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    work_resource: ResourceAddress,
    contractor: NonFungibleId,
    work_id: NonFungibleId,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(client_resource, client_account)
        .pop_from_auth_zone(|builder, client_proof| {
            builder
                .create_proof_from_account_by_ids(
                    &BTreeSet::from_iter([work_id]),
                    work_resource,
                    client_account,
                )
                .pop_from_auth_zone(|builder, work_id| {
                    builder.call_method(
                        work_component,
                        "accept_contractor_for_work",
                        args!(Proof(work_id), contractor, Proof(client_proof)),
                    )
                })
                .drop_all_proofs()
        })
        .build();
    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub fn request_work(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account_component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    contractor_resource: ResourceAddress,
    work_component: ComponentAddress,
    work_id: NonFungibleId,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .withdraw_from_account_by_amount(dec!(1), RADIX_TOKEN, account_component)
        .create_proof_from_account(contractor_resource, account_component)
        .pop_from_auth_zone(|builder, proof_id| {
            builder
                .call_method(
                    work_component,
                    "request_work",
                    args!(work_id, Proof(proof_id)),
                )
                .call_method(
                    account_component,
                    "deposit_batch",
                    args!(Expression::entire_worktop()),
                )
        })
        .build();
    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub fn create_new_work(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account_component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    client_resource: ResourceAddress,
    work_components: &WorkComponents,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .withdraw_from_account_by_amount(dec!(1), RADIX_TOKEN, account_component)
        .create_proof_from_account(client_resource, account_component)
        .pop_from_auth_zone(|builder, proof_id| {
            builder
                .take_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
                    builder.call_method(
                        work_components.work_component,
                        "create_new_work",
                        args!(
                            dec!(1),
                            "Develop a dex",
                            "Create a multi pair decentralised exchange.",
                            vec!["rustlang", "scrypto"],
                            Bucket(bucket_id),
                            Proof(proof_id)
                        ),
                    )
                })
                .call_method(
                    account_component,
                    "deposit_batch",
                    args!(Expression::entire_worktop()),
                )
        })
        .build();
    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub fn create_new_category(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    service_admin_badge: ResourceAddress,
    account_component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    component: ComponentAddress,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .create_proof_from_account(service_admin_badge, account_component)
        .call_method(component, "create_new_category", args!("Development & IT"))
        .drop_all_proofs()
        .build();

    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub struct WorkComponents {
    pub work_component: ComponentAddress,
    pub work_resource: ResourceAddress,
    pub dispute_component: ComponentAddress,
    pub dispute_resource: ResourceAddress,
    pub dispute_document_resource: ResourceAddress,
}

pub fn get_work_components(result: &CommitResult) -> WorkComponents {
    let work_component = result.entity_changes.new_component_addresses[0];
    let dispute_component = result.entity_changes.new_component_addresses[1];
    let work_resource = result.entity_changes.new_resource_addresses[0];
    let dispute_resource = result.entity_changes.new_resource_addresses[1];
    let dispute_document_resource = result.entity_changes.new_resource_addresses[2];

    WorkComponents {
        work_component,
        work_resource,
        dispute_component,
        dispute_resource,
        dispute_document_resource,
    }
}

pub fn get_account_balance(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    account_component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    resource: ResourceAddress,
) -> Decimal {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(account_component, "balance", args!(resource))
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);

    receipt.output::<Decimal>(1)
}

pub struct MoonWorkResource {
    pub service_admin_badge: ResourceAddress,
    pub contractor_badge: ResourceAddress,
    pub client_badge: ResourceAddress,
    pub completed_work_resource: ResourceAddress,
    pub disputed_outcome_resource: ResourceAddress,
    pub contractor_accolade_resource: ResourceAddress,
}

pub fn get_moonwork_resources(commit_result: &CommitResult) -> MoonWorkResource {
    let service_admin_badge = commit_result.entity_changes.new_resource_addresses[1];
    let contractor_badge = commit_result.entity_changes.new_resource_addresses[2];
    let client_badge = commit_result.entity_changes.new_resource_addresses[3];
    let completed_work_resource = commit_result.entity_changes.new_resource_addresses[4];
    let disputed_outcome_resource = commit_result.entity_changes.new_resource_addresses[5];
    let contractor_accolade_resource = commit_result.entity_changes.new_resource_addresses[6];

    MoonWorkResource {
        service_admin_badge,
        contractor_badge,
        client_badge,
        completed_work_resource,
        disputed_outcome_resource,
        contractor_accolade_resource,
    }
}

pub fn register_as_contractor(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    component: ComponentAddress,
    account_component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    username: &str,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(component, "register_as_contractor", args!(username))
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub fn register_as_client(
    test_runner: &mut TestRunner<TypedInMemorySubstateStore>,
    component: ComponentAddress,
    account_component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
    username: &str,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_method(component, "register_as_client", args!(username))
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()])
}

pub fn create_moonwork_service<'a>(
    test_runner: &'a mut TestRunner<TypedInMemorySubstateStore>,
    account_component: ComponentAddress,
    public_key: EcdsaSecp256k1PublicKey,
) -> (ComponentAddress, MoonWorkResource) {
    let package_address = test_runner.compile_and_publish(this_package!());
    let service_fee = dec!(1);
    let minimum_work_payment = dec!(1);
    let dispute_participant_criteria = ParticipantCriteria {
        participant_limit: 1,
        contractor: ContractorCriteria { jobs_completed: 1 },
        client: ClientCriteria { jobs_paid_out: 1 },
    };

    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .call_function(
            package_address,
            "MoonWorkService",
            "create",
            args!(
                service_fee,
                minimum_work_payment,
                dispute_participant_criteria
            ),
        )
        .call_method(
            account_component,
            "deposit_batch",
            args!(Expression::entire_worktop()),
        )
        .build();
    let service_receipt =
        test_runner.execute_manifest_ignoring_fee(manifest, vec![public_key.into()]);

    service_receipt.expect_commit_success();

    let resources = get_moonwork_resources(service_receipt.expect_commit());
    let service_component = service_receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];
    (service_component, resources)
}

pub fn get_non_fungible_data<T: NonFungibleData>(
    store: &TypedInMemorySubstateStore,
    resource: ResourceAddress,
    id: NonFungibleId,
) -> T {
    let nft_wrapper: Option<radix_engine::model::NonFungibleWrapper> = store
        .get_substate(&SubstateId::NonFungible(resource, id))
        .map(|s| s.substate)
        .map(|s| s.into());
    let nft = nft_wrapper.unwrap().0.unwrap();
    // there is currently a bug in the NonFungibleData decode method where whats supposed to be an
    // empty struct `Struct()` is decoding as `Struct(Vec<NonFungibleId>(), Vec<NonFungibleId>(), Map<NonFungibleId, Enum>(), Map<NonFungibleId, Enum>())`
    // So this is a workaround to make things work for now
    let mutable_data = match T::mutable_data_schema().matches(&Value::Struct { fields: vec![] }) {
        true => {
            ScryptoValue::from_value(Value::Struct { fields: vec![] })
                .unwrap()
                .raw
        }
        false => nft.mutable_data(),
    };
    let immutable_data = match T::immutable_data_schema().matches(&Value::Struct { fields: vec![] })
    {
        true => {
            ScryptoValue::from_value(Value::Struct { fields: vec![] })
                .unwrap()
                .raw
        }
        false => nft.immutable_data(),
    };
    T::decode(&immutable_data, &mutable_data).unwrap()
}

pub fn does_non_fungible_exist(
    store: &TypedInMemorySubstateStore,
    resource: ResourceAddress,
    id: NonFungibleId,
) -> bool {
    let nft_wrapper: Option<radix_engine::model::NonFungibleWrapper> = store
        .get_substate(&SubstateId::NonFungible(resource, id))
        .map(|s| s.substate)
        .map(|s| s.into());

    let unwrapped_substate = nft_wrapper.unwrap().0;

    match unwrapped_substate {
        Some(_) => true,
        None => false,
    }
}

pub fn get_total_supply(
    store: &TypedInMemorySubstateStore,
    resource_address: ResourceAddress,
) -> Decimal {
    let resource_manager: ResourceManager = store
        .get_substate(&SubstateId::ResourceManager(resource_address))
        .map(|s| s.substate)
        .map(|s| s.into())
        .unwrap();

    resource_manager.total_supply()
}

pub fn assert_error_message(
    receipt: &radix_engine::transaction::TransactionReceipt,
    error_message: &str,
) {
    let is_error_found = receipt
        .execution
        .application_logs
        .iter()
        .find(|(_, message)| message.contains(error_message));

    assert!(is_error_found.is_some());
}
