use crate::moonwork::DisputeDecision;
use crate::users::{Client, Contractor};
use crate::work::{Work, WorkStatus};
use scrypto::prelude::*;

// Assuming an epoch interval is 30mins, 5 days expiry time
const EXPIRATION_TIME: u64 = 240;

#[derive(Debug, Describe, TypeId, Encode, Decode, PartialEq, Eq)]
pub enum DisputeSide {
    Contractor,
    Client,
}

#[derive(Debug, NonFungibleData)]
pub struct Dispute {
    pub work: NonFungibleAddress,
    pub expiration: u64,
    pub raised_by: DisputeSide,
    pub contractor: NonFungibleId,
    pub client: NonFungibleId,
    #[scrypto(mutable)]
    pub client_documents: Vec<NonFungibleId>,
    #[scrypto(mutable)]
    pub contractor_documents: Vec<NonFungibleId>,
    #[scrypto(mutable)]
    pub participant_contractors: HashMap<NonFungibleId, DisputeSide>,
    #[scrypto(mutable)]
    pub participant_clients: HashMap<NonFungibleId, DisputeSide>,
}

#[derive(Debug, NonFungibleData)]
pub struct DisputeDocument {
    pub submitted_by: DisputeSide,
    pub dispute_id: NonFungibleId,
    pub document_title: String,
    pub document_url: String,
}

#[derive(Debug, Describe, TypeId, Encode, Decode, Clone, Copy)]
pub struct ContractorCriteria {
    pub jobs_completed: u64,
}

#[derive(Debug, Describe, TypeId, Encode, Decode, Clone, Copy)]
pub struct ClientCriteria {
    pub jobs_paid_out: u64,
}

#[derive(Debug, Describe, TypeId, Encode, Decode, Clone, Copy)]
pub struct ParticipantCriteria {
    pub participant_limit: u64,
    pub contractor: ContractorCriteria,
    pub client: ClientCriteria,
}

blueprint! {
    struct DisputeService {
        client: ResourceAddress,
        contractor: ResourceAddress,
        dispute_resource: ResourceAddress,
        dispute_latest_id: u64,
        dispute_vault: Vault,
        dispute_document_resource: ResourceAddress,
        dispute_document_latest_id: u64,
        dispute_document_vault: Vault,
        work_resource: ResourceAddress,
        participant_incentive_vault: Vault,
        service_auth: Vault,
        service_component: ComponentAddress,
    }

    impl DisputeService {
        /// This creates a basic dispute system for a given work resource. This blueprint is
        /// designed in a way that is reusable. You will have to implement a service component to
        /// use this blueprint.
        ///
        /// Service Methods To Implement:
        /// ```ignore
        /// blueprint! {
        ///     struct Service {}
        ///
        ///     impl Service {
        ///         pub fn get_dispute_participant_criteria(&self) -> ParticipantCriteria {
        ///             todo!()
        ///         }
        ///
        ///         pub fn compensate_contractor(
        ///             &self,
        ///             client_id: NonFungibleId,
        ///             contractor_id: NonFungibleId,
        ///             total_compensation: Decimal,
        ///         ) -> ParticipantCriteria {
        ///             todo!()
        ///         }
        ///
        ///         pub fn complete_dispute_outcome(
        ///             &self,
        ///             dispute_side: DisputeSide,
        ///             contractor_id: NonFungibleId,
        ///             client_id: NonFungibleId,
        ///             dispute_decision: DisputeDecision,
        ///         ) {
        ///             todo!()
        ///         }
        ///
        ///         pub fn refund_client(&self, client_id: NonFungibleId, refund_amount: Decimal) {
        ///             todo!()
        ///         }
        ///     }
        /// }
        /// ```
        pub fn create(
            service_badge: Bucket,
            client: ResourceAddress,
            contractor: ResourceAddress,
            work_resource: ResourceAddress,
            service_component: ComponentAddress,
        ) -> ComponentAddress {
            let dispute_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", "Dispute")
                .metadata("service", "MoonWork")
                .mintable(rule!(require(service_badge.resource_address())), LOCKED)
                .burnable(rule!(require(service_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(service_badge.resource_address())),
                    LOCKED,
                )
                .no_initial_supply();

            let dispute_document_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", "Dispute Document")
                .metadata("service", "MoonWork")
                .mintable(rule!(require(service_badge.resource_address())), LOCKED)
                .burnable(rule!(require(service_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(service_badge.resource_address())),
                    LOCKED,
                )
                .no_initial_supply();

            let access_rules = AccessRules::new()
                .method(
                    "complete_dispute_as_admin",
                    rule!(require(service_badge.resource_address())),
                )
                .default(AccessRule::AllowAll);

            let mut component = Self {
                client,
                contractor,
                dispute_resource,
                dispute_latest_id: 0,
                dispute_document_resource,
                dispute_document_latest_id: 0,
                work_resource,
                dispute_vault: Vault::new(dispute_resource),
                dispute_document_vault: Vault::new(dispute_document_resource),
                participant_incentive_vault: Vault::new(RADIX_TOKEN),
                service_auth: Vault::with_bucket(service_badge),
                service_component,
            }
            .instantiate();

            component.add_access_check(access_rules);
            component.globalize()
        }

        /// Creates a new dispute for work that is only in progress and assigned to a contractor
        /// Work carried out may have been done unfairly and not with what was agreed.
        ///
        /// This method does the following:
        /// 1. Updates `Work` NFT `Work.work_status` with `InDispute` status
        /// 2. A `Dispute` NFT is minted with an expiration (`CURRENT_EPOCH + EXPIRATION_TIME`)
        /// 3. Dispute NFT is stored in a dispute vault
        ///
        /// # Panics:
        /// - if user proof provided is an invalid `Client` or `Contractor` `ResourceAddress`
        /// - if `work_resource` is invalid
        /// - if `Work.work_status` is not `InProgress`
        /// - if `Work.contractor_assigned` is not assigned to any `Contractor`
        /// - if `Work.client` or `Work.contractor_assigned` does not match the user `Proof`
        pub fn create_new_dispute(
            &mut self,
            work_id: NonFungibleId,
            work_resource: ResourceAddress,
            client_or_contractor: Proof,
        ) {
            let validated_client_or_contractor = client_or_contractor
                .validate_proof(ProofValidationMode::ValidateResourceAddressBelongsTo(
                    BTreeSet::from_iter([self.client, self.contractor]),
                ))
                .expect("unauthorized user");

            assert!(work_resource == self.work_resource, "invalid work");

            let work_resource_manager = borrow_resource_manager!(work_resource);

            let work_nft = work_resource_manager.get_non_fungible_data::<Work>(&work_id);

            assert!(
                work_nft.work_status == WorkStatus::InProgress,
                "work not in progress"
            );

            let is_client = work_nft.client == validated_client_or_contractor.non_fungible_id();

            let contractor_assigned = work_nft.contractor_assigned.expect("work not assigned yet");

            let is_contractor =
                contractor_assigned == validated_client_or_contractor.non_fungible_id();

            assert!(is_client || is_contractor, "unauthorized user");

            self.service_auth.authorize(|| {
                work_resource_manager.update_non_fungible_data(
                    &work_id,
                    Work {
                        client: work_nft.client.clone(),
                        total_compensation: work_nft.total_compensation,
                        work_title: work_nft.work_title,
                        work_description: work_nft.work_description,
                        work_status: WorkStatus::InDispute,
                        skills_required: work_nft.skills_required,
                        contractor_requests: work_nft.contractor_requests,
                        contractor_assigned: Some(contractor_assigned.clone()),
                    },
                );
            });

            let dispute_resource_manager = borrow_resource_manager!(self.dispute_resource);

            self.dispute_latest_id += 1;

            let id = NonFungibleId::from_u64(self.dispute_latest_id);

            let raised_by = match is_contractor {
                true => DisputeSide::Contractor,
                false => DisputeSide::Client,
            };

            let dispute = self.service_auth.authorize(|| {
                dispute_resource_manager.mint_non_fungible(
                    &id,
                    Dispute {
                        work: NonFungibleAddress::new(work_resource, work_id),
                        expiration: Runtime::current_epoch() + EXPIRATION_TIME,
                        participant_contractors: HashMap::new(),
                        participant_clients: HashMap::new(),
                        contractor: contractor_assigned,
                        client: work_nft.client,
                        contractor_documents: vec![],
                        client_documents: vec![],
                        raised_by,
                    },
                )
            });

            self.dispute_vault.put(dispute);
        }

        /// Cancels the dispute for a given work resource. This method does the following:
        ///
        /// 1. Updates `Work.work_status` back to `InProgress`
        /// 2. Burns the `Dispute` NFT
        ///
        /// # Panics:
        /// - if user proof is not a `Client` or `Contractor` `ResourceAddress`
        /// - if `Client` or `Contractor` is not part of the dispute
        pub fn cancel_dispute(&mut self, dispute_id: NonFungibleId, client_or_contractor: Proof) {
            let validated_client_or_contractor = client_or_contractor
                .validate_proof(ProofValidationMode::ValidateResourceAddressBelongsTo(
                    BTreeSet::from_iter([self.client, self.contractor]),
                ))
                .expect("unauthorized user");

            let dispute_resource_manager = borrow_resource_manager!(self.dispute_resource);

            let dispute_nft =
                dispute_resource_manager.get_non_fungible_data::<Dispute>(&dispute_id);

            match validated_client_or_contractor.resource_address() == self.client {
                true => {
                    let is_raised_by_client = dispute_nft.raised_by == DisputeSide::Client;
                    let is_matching_client_id =
                        dispute_nft.client == validated_client_or_contractor.non_fungible_id();
                    assert!(
                        is_raised_by_client && is_matching_client_id,
                        "invalid client"
                    )
                }
                false => {
                    let is_raised_by_contractor = dispute_nft.raised_by == DisputeSide::Contractor;
                    let is_matching_contractor_id =
                        dispute_nft.contractor == validated_client_or_contractor.non_fungible_id();
                    assert!(
                        is_raised_by_contractor && is_matching_contractor_id,
                        "invalid contractor"
                    );
                }
            };

            let work_resource_manager = borrow_resource_manager!(self.work_resource);

            let work_nft = work_resource_manager
                .get_non_fungible_data::<Work>(&dispute_nft.work.non_fungible_id());

            self.service_auth.authorize(|| {
                dispute_resource_manager.burn(self.dispute_vault.take_non_fungible(&dispute_id));

                work_resource_manager.update_non_fungible_data(
                    &dispute_nft.work.non_fungible_id(),
                    Work {
                        client: work_nft.client,
                        total_compensation: work_nft.total_compensation,
                        work_title: work_nft.work_title,
                        work_description: work_nft.work_description,
                        work_status: WorkStatus::InProgress,
                        skills_required: work_nft.skills_required,
                        contractor_requests: work_nft.contractor_requests,
                        contractor_assigned: work_nft.contractor_assigned,
                    },
                );
            })
        }

        /// Client or Contractor that is in dispute can present all evidence that supports their
        /// side of the dispute. This is used for participants to judge who they think is correct.
        ///
        /// Once again, leveraging NFTs, this method does the following:
        /// 1. Mints a `DisputeDocument` NFT which is public and viewable by anyone.
        ///
        /// # Panics:
        /// - if `Client` or `Contractor` is not the correct `ResourceAddress`
        /// - if `Client` or `Contractor` is not part of the dispute
        pub fn submit_document(
            &mut self,
            dispute_id: NonFungibleId,
            document_title: String,
            document_url: String,
            client_or_contractor: Proof,
        ) {
            let validated_client_or_contractor = client_or_contractor
                .validate_proof(ProofValidationMode::ValidateResourceAddressBelongsTo(
                    BTreeSet::from_iter([self.client, self.contractor]),
                ))
                .expect("unauthorized user");

            let dispute_resource_manager = borrow_resource_manager!(self.dispute_resource);

            let mut dispute =
                dispute_resource_manager.get_non_fungible_data::<Dispute>(&dispute_id);

            let is_client_part_of_dispute =
                dispute.client == validated_client_or_contractor.non_fungible_id();

            let is_contractor_part_of_dispute =
                dispute.contractor == validated_client_or_contractor.non_fungible_id();

            assert!(
                is_client_part_of_dispute || is_contractor_part_of_dispute,
                "unauthorized user"
            );

            let dispute_side = match is_client_part_of_dispute {
                true => DisputeSide::Client,
                false => DisputeSide::Contractor,
            };

            let dispute_document_resource_manager =
                borrow_resource_manager!(self.dispute_document_resource);

            self.dispute_document_latest_id += 1;

            let id = NonFungibleId::from_u64(self.dispute_document_latest_id);

            let dispute_document = self.service_auth.authorize(|| {
                dispute_document_resource_manager.mint_non_fungible(
                    &id,
                    DisputeDocument {
                        submitted_by: dispute_side,
                        dispute_id: dispute_id.clone(),
                        document_title,
                        document_url,
                    },
                )
            });

            match is_client_part_of_dispute {
                true => dispute
                    .client_documents
                    .push(dispute_document.non_fungible_id()),
                false => dispute
                    .contractor_documents
                    .push(dispute_document.non_fungible_id()),
            };

            self.service_auth.authorize(|| {
                dispute_resource_manager.update_non_fungible_data(
                    &dispute_id,
                    Dispute {
                        work: dispute.work,
                        expiration: dispute.expiration,
                        contractor: dispute.contractor,
                        client: dispute.client,
                        client_documents: dispute.client_documents,
                        contractor_documents: dispute.contractor_documents,
                        participant_contractors: dispute.participant_contractors,
                        participant_clients: dispute.participant_clients,
                        raised_by: dispute.raised_by,
                    },
                )
            });

            self.dispute_document_vault.put(dispute_document);
        }

        /// Removes a document from the dispute, this method does the following:
        ///
        /// 1. Update `Work.contractor_documents` or `Work.client_documents` depending on the user
        /// wanting to remove their document
        /// 2. `DisputeDocument` NFT with the corresponding NonFungibleId is burned from vault
        ///
        /// # Panics
        /// - if user proof is not a `Client` or `Contractor`
        /// - if valid user proof is not part of the dispute
        pub fn remove_document(&mut self, document_id: NonFungibleId, client_or_contractor: Proof) {
            let validated_client_or_contractor = client_or_contractor
                .validate_proof(ProofValidationMode::ValidateResourceAddressBelongsTo(
                    BTreeSet::from_iter([self.client, self.contractor]),
                ))
                .expect("unauthorized user");

            let dispute_document_resource_manager =
                borrow_resource_manager!(self.dispute_document_resource);

            let document_nft = dispute_document_resource_manager
                .get_non_fungible_data::<DisputeDocument>(&document_id);

            let dispute_resource_manager = borrow_resource_manager!(self.dispute_resource);

            let mut dispute_nft =
                dispute_resource_manager.get_non_fungible_data::<Dispute>(&document_nft.dispute_id);

            match validated_client_or_contractor.resource_address() == self.client {
                true => {
                    assert!(
                        dispute_nft.client == validated_client_or_contractor.non_fungible_id(),
                        "unauthorized user"
                    );
                    assert!(document_nft.submitted_by == DisputeSide::Client);
                    dispute_nft.client_documents = dispute_nft
                        .client_documents
                        .into_iter()
                        .filter(|cd| cd != &document_id)
                        .collect();
                }
                false => {
                    assert!(
                        dispute_nft.contractor == validated_client_or_contractor.non_fungible_id(),
                        "unauthorized user"
                    );
                    assert!(document_nft.submitted_by == DisputeSide::Contractor);
                    dispute_nft.contractor_documents = dispute_nft
                        .contractor_documents
                        .into_iter()
                        .filter(|cd| cd != &document_id)
                        .collect();
                }
            }

            self.service_auth.authorize(|| {
                dispute_document_resource_manager
                    .burn(self.dispute_document_vault.take_non_fungible(&document_id));

                dispute_resource_manager.update_non_fungible_data(
                    &document_nft.dispute_id,
                    Dispute {
                        work: dispute_nft.work,
                        expiration: dispute_nft.expiration,
                        raised_by: dispute_nft.raised_by,
                        contractor: dispute_nft.contractor,
                        client: dispute_nft.client,
                        client_documents: dispute_nft.client_documents,
                        contractor_documents: dispute_nft.contractor_documents,
                        participant_contractors: dispute_nft.participant_contractors,
                        participant_clients: dispute_nft.participant_clients,
                    },
                )
            })
        }

        /// Participants can join a dispute to decide either between on the side of the client or
        /// contractor. In order to participate, they must meet the `ParticipantCriteria` to vote.
        /// Equal amounts of Client and Contractor can join. If the participant limit is 2,
        /// 2 Client users and 2 Contractor can join.
        ///
        /// This method in detail does the following:
        /// 1. Check if willing participant meets requirements
        /// 2. `Dispute` NFT is updated with the participant
        ///
        /// # Panics
        /// - if user proof is not a `Client` or `Contractor` has an invalid `ResourceAddress`
        /// - if dispute has expired
        /// - if user is part of the dispute itself
        /// - if user has already joined the dispute
        /// - if user does not meet requirements
        pub fn join_and_decide_dispute(
            &self,
            dispute_id: NonFungibleId,
            side: DisputeSide,
            client_or_contractor: Proof,
        ) {
            let validated_client_or_contractor = client_or_contractor
                .validate_proof(ProofValidationMode::ValidateResourceAddressBelongsTo(
                    BTreeSet::from_iter([self.client, self.contractor]),
                ))
                .expect("unauthorized user");

            let dispute_resource_manager = borrow_resource_manager!(self.dispute_resource);

            let mut dispute =
                dispute_resource_manager.get_non_fungible_data::<Dispute>(&dispute_id);

            assert!(
                dispute.expiration >= Runtime::current_epoch(),
                "dispute expired"
            );

            let participant_criteria = borrow_component!(self.service_component)
                .call::<ParticipantCriteria>("get_dispute_participant_criteria", args!());

            match validated_client_or_contractor.resource_address() == self.client {
                true => {
                    assert!(
                        validated_client_or_contractor.non_fungible_id() != dispute.client,
                        "cannot participate in own dispute"
                    );
                    assert!(
                        dispute
                            .participant_clients
                            .get(&validated_client_or_contractor.non_fungible_id())
                            .is_none(),
                        "already joined dispute"
                    );
                    assert!(
                        dispute.participant_clients.len()
                            < participant_criteria.participant_limit as usize,
                        "participant limit reached"
                    );

                    let client_resource_manager = borrow_resource_manager!(self.client);
                    let client_nft = client_resource_manager.get_non_fungible_data::<Client>(
                        &validated_client_or_contractor.non_fungible_id(),
                    );

                    assert!(
                        client_nft.jobs_paid_out >= participant_criteria.client.jobs_paid_out,
                        "client criteria not met"
                    );

                    dispute
                        .participant_clients
                        .insert(validated_client_or_contractor.non_fungible_id(), side)
                }
                false => {
                    assert!(
                        validated_client_or_contractor.non_fungible_id() != dispute.contractor,
                        "cannot participate in own dispute"
                    );

                    assert!(
                        dispute
                            .participant_contractors
                            .get(&validated_client_or_contractor.non_fungible_id())
                            .is_none(),
                        "already joined dispute"
                    );
                    assert!(
                        dispute.participant_contractors.len()
                            < participant_criteria.participant_limit as usize,
                        "participant limit reached"
                    );

                    let contractor_resource_manager = borrow_resource_manager!(self.contractor);
                    let contractor_nft = contractor_resource_manager
                        .get_non_fungible_data::<Contractor>(
                            &validated_client_or_contractor.non_fungible_id(),
                        );

                    assert!(
                        contractor_nft.jobs_completed
                            >= participant_criteria.contractor.jobs_completed,
                        "contractor criteria not met"
                    );

                    dispute
                        .participant_contractors
                        .insert(validated_client_or_contractor.non_fungible_id(), side)
                }
            };

            self.service_auth.authorize(|| {
                dispute_resource_manager.update_non_fungible_data(
                    &dispute_id,
                    Dispute {
                        work: dispute.work,
                        expiration: dispute.expiration,
                        client: dispute.client,
                        contractor: dispute.contractor,
                        participant_contractors: dispute.participant_contractors,
                        participant_clients: dispute.participant_clients,
                        client_documents: dispute.client_documents,
                        contractor_documents: dispute.contractor_documents,
                        raised_by: dispute.raised_by,
                    },
                )
            });
        }

        /// If the dispute has expired and has a clear decision, either the `Client` or
        /// `Contractor` can complete the dispute. Similar to the actions of
        /// complete_dispute_as_admin, this methods does the following:
        ///
        /// 1. The associated `Work` NFT is marked as `Disputed`
        /// 2. If the contractor has won, it compensates the contractor as agreed
        /// 3. If the client has won, it refunds the client from the compensation agreed
        /// 4. In both cases, it calls the service component to create a `DisputeOutcome` NFT
        ///    for both the `Client` and `Contractor` indicating if they `Won` or `Lost`
        ///    respectively
        ///
        /// # Panics:
        /// - if user proof is not a `Client` or `Contractor` `ResourceAddress`
        /// - if `Work.work_status` is not `InDispute`
        /// - if dispute has not expired or dispute has not reached participant limit
        pub fn complete_dispute(&mut self, dispute_id: NonFungibleId, client_or_contractor: Proof) {
            let validated_client_or_contractor = client_or_contractor
                .validate_proof(ProofValidationMode::ValidateResourceAddressBelongsTo(
                    BTreeSet::from_iter([self.client, self.contractor]),
                ))
                .expect("unauthorized user");

            let participant_criteria = borrow_component!(self.service_component)
                .call::<ParticipantCriteria>("get_dispute_participant_criteria", args!());

            let dispute_resource_manager = borrow_resource_manager!(self.dispute_resource);

            let dispute = dispute_resource_manager.get_non_fungible_data::<Dispute>(&dispute_id);

            let work_resource_manager = borrow_resource_manager!(dispute.work.resource_address());

            let work_nft = work_resource_manager
                .get_non_fungible_data::<Work>(&dispute.work.non_fungible_id());

            assert!(
                work_nft.work_status == WorkStatus::InDispute,
                "work must be in dispute"
            );

            let is_client = dispute.client == validated_client_or_contractor.non_fungible_id();

            let is_contractor =
                dispute.contractor == validated_client_or_contractor.non_fungible_id();

            assert!(is_client || is_contractor, "unauthorized user");

            let is_dispute_expired = Runtime::current_epoch() > dispute.expiration;

            let participant_limit_reached = dispute.participant_contractors.len()
                == participant_criteria.participant_limit as usize
                && dispute.participant_clients.len()
                    == participant_criteria.participant_limit as usize;

            assert!(
                is_dispute_expired || participant_limit_reached,
                "requirements not met"
            );

            let clients_on_contractor_side = dispute
                .participant_clients
                .values()
                .into_iter()
                .filter(|client| **client == DisputeSide::Contractor)
                .count();
            let contractors_on_contractor_side = dispute
                .participant_contractors
                .values()
                .into_iter()
                .filter(|contractor| **contractor == DisputeSide::Contractor)
                .count();

            let total_participants =
                dispute.participant_clients.len() + dispute.participant_contractors.len();

            let total_on_contractor = clients_on_contractor_side + contractors_on_contractor_side;

            let total_on_client = total_participants - total_on_contractor;

            let has_contractor_won_dispute =
                ((Decimal::from(total_on_contractor) / Decimal::from(total_participants)) * 100)
                    > dec!(50);

            let has_client_won_dispute =
                ((Decimal::from(total_on_client) / Decimal::from(total_participants)) * 100)
                    > dec!(50);

            assert!(
                (has_client_won_dispute && !has_contractor_won_dispute)
                    || (!has_client_won_dispute && has_contractor_won_dispute),
                "split decision - admin decision"
            );

            self.service_auth.authorize(|| {
                work_resource_manager.update_non_fungible_data(
                    &dispute.work.non_fungible_id(),
                    Work {
                        client: work_nft.client,
                        total_compensation: work_nft.total_compensation,
                        work_title: work_nft.work_title,
                        work_description: work_nft.work_description,
                        work_status: WorkStatus::Disputed,
                        skills_required: work_nft.skills_required,
                        contractor_requests: work_nft.contractor_requests,
                        contractor_assigned: work_nft.contractor_assigned,
                    },
                );
            });

            let work_compensation = work_nft.total_compensation;

            match has_contractor_won_dispute {
                true => self.service_auth.authorize(|| {
                    borrow_component!(self.service_component).call::<()>(
                        "compensate_contractor",
                        args!(dispute.client, dispute.contractor, work_compensation),
                    );

                    borrow_component!(self.service_component).call::<()>(
                        "complete_dispute_outcome",
                        args!(
                            DisputeSide::Contractor,
                            dispute.contractor,
                            dispute.work,
                            DisputeDecision::Won
                        ),
                    );

                    borrow_component!(self.service_component).call::<()>(
                        "complete_dispute_outcome",
                        args!(
                            DisputeSide::Client,
                            dispute.client,
                            dispute.work,
                            DisputeDecision::Lost
                        ),
                    );
                }),
                false => {
                    self.service_auth.authorize(|| {
                        borrow_component!(self.service_component)
                            .call::<()>("refund_client", args!(dispute.client, work_compensation));

                        borrow_component!(self.service_component).call::<()>(
                            "complete_dispute_outcome",
                            args!(
                                DisputeSide::Contractor,
                                dispute.contractor,
                                dispute.work,
                                DisputeDecision::Lost
                            ),
                        );

                        borrow_component!(self.service_component).call::<()>(
                            "complete_dispute_outcome",
                            args!(
                                DisputeSide::Client,
                                dispute.client,
                                dispute.work,
                                DisputeDecision::Won
                            ),
                        );
                    });
                }
            };
        }

        /// Admins can intervene on disputes only if certain conditions are met. That is if the
        /// dispute has expired and does not have a clear overall decision.
        ///
        /// To encourage reusability, we actually rely on this blueprint being part of a service
        /// component, just like the work blueprint. This method does a few steps to complete
        /// dispute:
        ///
        /// 1. The associated `Work` NFT is marked as `Disputed`
        /// 2. If the contractor has won, it compensates the contractor as agreed
        /// 3. If the client has won, it refunds the client from the compensation agreed
        /// 4. In both cases, it calls the service component to create a `DisputeOutcome` NFT
        ///    for both the `Client` and `Contractor` indicating if they `Won` or `Lost`
        ///    respectively
        ///
        /// # Panics:
        /// - if user is not an admin
        /// - if work is not in dispute
        /// - if dispute has not expired or has a clear overall decision
        pub fn complete_dispute_as_admin(
            &mut self,
            dispute_id: NonFungibleId,
            dispute_side: DisputeSide,
        ) {
            let dispute_resource_manager = borrow_resource_manager!(self.dispute_resource);

            let dispute = dispute_resource_manager.get_non_fungible_data::<Dispute>(&dispute_id);

            let work_resource_manager = borrow_resource_manager!(dispute.work.resource_address());

            let work_nft = work_resource_manager
                .get_non_fungible_data::<Work>(&dispute.work.non_fungible_id());

            assert!(
                work_nft.work_status == WorkStatus::InDispute,
                "work must be in dispute"
            );

            let is_dispute_expired = Runtime::current_epoch() > dispute.expiration;

            let clients_on_contractor_side = dispute
                .participant_clients
                .values()
                .into_iter()
                .filter(|client| **client == DisputeSide::Contractor)
                .count();
            let contractors_on_contractor_side = dispute
                .participant_contractors
                .values()
                .into_iter()
                .filter(|contractor| **contractor == DisputeSide::Contractor)
                .count();

            let total_participants =
                dispute.participant_clients.len() + dispute.participant_contractors.len();

            let total_on_contractor = clients_on_contractor_side + contractors_on_contractor_side;

            let total_on_client = total_participants - total_on_contractor;

            let has_contractor_won_dispute =
                ((Decimal::from(total_on_contractor) / Decimal::from(total_participants)) * 100)
                    > dec!(50);

            let has_client_won_dispute =
                ((Decimal::from(total_on_client) / Decimal::from(total_participants)) * 100)
                    > dec!(50);

            let is_split_decision = (!has_client_won_dispute && !has_client_won_dispute)
                || (has_client_won_dispute && has_contractor_won_dispute);

            assert!(
                is_dispute_expired && is_split_decision,
                "conditions for admin action not met"
            );

            self.service_auth.authorize(|| {
                work_resource_manager.update_non_fungible_data(
                    &dispute.work.non_fungible_id(),
                    Work {
                        client: work_nft.client,
                        total_compensation: work_nft.total_compensation,
                        work_title: work_nft.work_title,
                        work_description: work_nft.work_description,
                        work_status: WorkStatus::Disputed,
                        skills_required: work_nft.skills_required,
                        contractor_requests: work_nft.contractor_requests,
                        contractor_assigned: work_nft.contractor_assigned,
                    },
                );
            });

            let work_compensation = work_nft.total_compensation;

            match dispute_side {
                DisputeSide::Contractor => self.service_auth.authorize(|| {
                    borrow_component!(self.service_component).call::<()>(
                        "compensate_contractor",
                        args!(dispute.client, dispute.contractor, work_compensation),
                    );

                    borrow_component!(self.service_component).call::<()>(
                        "complete_dispute_outcome",
                        args!(
                            DisputeSide::Contractor,
                            dispute.contractor,
                            dispute.work,
                            DisputeDecision::Won
                        ),
                    );

                    borrow_component!(self.service_component).call::<()>(
                        "complete_dispute_outcome",
                        args!(
                            DisputeSide::Client,
                            dispute.client,
                            dispute.work,
                            DisputeDecision::Lost
                        ),
                    );
                }),
                DisputeSide::Client => {
                    self.service_auth.authorize(|| {
                        borrow_component!(self.service_component)
                            .call::<()>("refund_client", args!(dispute.client, work_compensation));

                        borrow_component!(self.service_component).call::<()>(
                            "complete_dispute_outcome",
                            args!(
                                DisputeSide::Contractor,
                                dispute.contractor,
                                dispute.work,
                                DisputeDecision::Lost
                            ),
                        );

                        borrow_component!(self.service_component).call::<()>(
                            "complete_dispute_outcome",
                            args!(
                                DisputeSide::Client,
                                dispute.client,
                                dispute.work,
                                DisputeDecision::Won
                            ),
                        );
                    });
                }
            };
        }
    }
}
