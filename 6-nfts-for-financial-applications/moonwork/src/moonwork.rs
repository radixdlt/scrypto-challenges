use crate::dispute::{DisputeServiceComponent, DisputeSide, ParticipantCriteria};
use crate::promotion::PromotionServiceComponent;
use crate::users::{Client, Contractor};
use crate::work::WorkServiceComponent;
use scrypto::prelude::*;

#[derive(Debug, NonFungibleData)]
struct CompletedWork {
    category_resource: ResourceAddress,
    work_id: NonFungibleId,
    total_compensation: Decimal,
}

#[derive(Debug, Describe, TypeId, Encode, Decode, PartialEq, Eq)]
#[repr(u64)]
pub enum Accolade {
    NewMoon = 0,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
}

impl Accolade {
    pub fn get_accolade(jobs_completed: u64) -> Accolade {
        if jobs_completed >= Self::FullMoon as u64 * 10 {
            Self::FullMoon
        } else {
            let accolade_index = jobs_completed / 10;
            unsafe { std::mem::transmute(accolade_index) }
        }
    }
}

#[derive(Debug, NonFungibleData)]
pub struct ContractorAccolades {
    pub accolade: Accolade,
    pub work_ids: Vec<NonFungibleAddress>,
}

#[derive(Debug, Describe, TypeId, Encode, Decode, PartialEq, Eq)]
pub enum DisputeDecision {
    Won,
    Lost,
}

#[derive(Debug, NonFungibleData)]
pub struct DisputeOutcome {
    pub work: NonFungibleAddress,
    pub decision: DisputeDecision,
}

blueprint! {
    struct MoonWorkService {
        contractor: ResourceAddress,
        client: ResourceAddress,
        admin_resource: ResourceAddress,
        moon_work_auth: Vault,
        // KeyValueStore<Client NonFungibleId, Vault> that stores work compensation
        client_work_vault: KeyValueStore<NonFungibleId, Vault>,
        // KeyValueStore<Client NonFungibleId, Vault> that stores work refunds as a result of
        // disputes or removed work
        client_withdrawable_vault: KeyValueStore<NonFungibleId, Vault>,
        // KeyValueStore<Contractor NonFungibleId, Vault> that stores all compensation as a result
        // of finished work
        contractor_vault: KeyValueStore<NonFungibleId, Vault>,
        // KeyValueStore<Contractor NonFungibleId, Vault> that stores `CompletedWork` NFT that is
        // later claimed
        completed_work_vault: KeyValueStore<NonFungibleId, Vault>,
        completed_work_resource: ResourceAddress,
        dispute_outcome_resource: ResourceAddress,
        // KeyValueStore<Client NonFungibleId, Vault> that stores all dispute outcomes as a result
        // of completed disputes
        client_dispute_outcome_vault: KeyValueStore<NonFungibleId, Vault>,
        // KeyValueStore<Conrtactor NonFungibleId, Vault> that stores all dispute outcomes as a result
        // of completed disputes
        contractor_dispute_outcome_vault: KeyValueStore<NonFungibleId, Vault>,
        contractor_accolade_resource: ResourceAddress,
        service_fee: Decimal,
        minimum_work_payment: Decimal,
        service_vault: Vault,
        dispute_participant_criteria: ParticipantCriteria,
    }

    impl MoonWorkService {
        /// Creates the whole MoonWork Service that encompasses the creation of Client, Contractor,
        /// WorkCompleted and DisputeOutcome NFT resources, all of which are soulbound NFTs.
        /// Alongside the resources, we have 2 additional components representing the work category
        /// system and dispute system for a given work resource.
        ///
        /// The purpose for leveraging all these NFT resources is to show a user's personal record
        /// as well as verifying their record by checking a Client's Work NFT balance or
        /// a Contractor's WorkCompleted NFT balance. And if there are any disputes, DisputeOutcome
        /// NFTs are also shown as a soulbound token in your wallet.
        ///
        /// - `service_fee` a service is taken only when work has successfully completed
        /// - `minimum_work_payment` this is required as a guard rail to stop disputes from being
        /// gamed
        /// - `dispute_participant_criteria` the number of participants on both sides, and
        /// requirements for clients and contractors to have work/jobs completed
        pub fn create(
            service_fee: Decimal,
            minimum_work_payment: Decimal,
            dispute_participant_criteria: ParticipantCriteria,
        ) -> (ComponentAddress, Bucket) {
            let moon_work_auth: Bucket = ResourceBuilder::new_fungible().initial_supply(1);

            let service_admin: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Admin Badge")
                .metadata("service", "MoonWork")
                .mintable(rule!(require(moon_work_auth.resource_address())), LOCKED)
                .burnable(rule!(require(moon_work_auth.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .initial_supply(1);

            let contractor = ResourceBuilder::new_non_fungible()
                .metadata("name", "Contractor")
                .metadata("service", "MoonWork")
                .mintable(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .updateable_non_fungible_data(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();

            let client = ResourceBuilder::new_non_fungible()
                .metadata("name", "Client")
                .metadata("service", "MoonWork")
                .mintable(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .updateable_non_fungible_data(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();

            let completed_work_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", "Completed Work")
                .metadata("service", "MoonWork")
                .mintable(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .restrict_withdraw(
                    rule!(require_any_of(vec![
                        service_admin.resource_address(),
                        moon_work_auth.resource_address()
                    ])),
                    LOCKED,
                )
                .no_initial_supply();

            let dispute_outcome_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", "Dispute Outcome")
                .metadata("service", "MoonWork")
                .mintable(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .restrict_withdraw(
                    rule!(require_any_of(vec![
                        service_admin.resource_address(),
                        moon_work_auth.resource_address()
                    ])),
                    LOCKED,
                )
                .no_initial_supply();

            let contractor_accolade_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", "Contractor Accolade")
                .metadata("service", "MoonWork")
                .mintable(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .burnable(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .updateable_non_fungible_data(
                    rule!(require_any_of(vec![
                        moon_work_auth.resource_address(),
                        service_admin.resource_address()
                    ])),
                    LOCKED,
                )
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();

            let access_rules = AccessRules::new()
                .method(
                    "deposit_compensation",
                    rule!(require(service_admin.resource_address())),
                )
                .method(
                    "create_new_category",
                    rule!(require(service_admin.resource_address())),
                )
                .method(
                    "finalise_work",
                    rule!(require(service_admin.resource_address())),
                )
                .method(
                    "update_dispute_participant_criteria",
                    rule!(require(service_admin.resource_address())),
                )
                .method(
                    "refund_client",
                    rule!(require(service_admin.resource_address())),
                )
                .method(
                    "complete_dispute_outcome",
                    rule!(require(service_admin.resource_address())),
                )
                .method(
                    "is_client_enabled",
                    rule!(require(service_admin.resource_address())),
                )
                .method(
                    "is_contractor_enabled",
                    rule!(require(service_admin.resource_address())),
                )
                .method(
                    "create_promotion_service",
                    rule!(require(service_admin.resource_address())),
                )
                .default(AccessRule::AllowAll);

            let mut component = Self {
                service_fee,
                minimum_work_payment,
                contractor: contractor,
                client: client,
                client_work_vault: KeyValueStore::new(),
                client_withdrawable_vault: KeyValueStore::new(),
                contractor_vault: KeyValueStore::new(),
                completed_work_vault: KeyValueStore::new(),
                completed_work_resource,
                dispute_outcome_resource,
                client_dispute_outcome_vault: KeyValueStore::new(),
                contractor_dispute_outcome_vault: KeyValueStore::new(),
                contractor_accolade_resource,
                admin_resource: service_admin.resource_address(),
                moon_work_auth: Vault::with_bucket(moon_work_auth),
                dispute_participant_criteria,
                service_vault: Vault::new(RADIX_TOKEN),
            }
            .instantiate();

            component.add_access_check(access_rules);
            let service_component = component.globalize();

            (service_component, service_admin)
        }

        /// This creates a basic promotion service with the requirements for contractors to promote
        /// themselves in the service. This is an example of how we would leverage the NFTs from
        /// `CompletedWork` and `ContractorAccolades` as a form of identity verification and
        /// requirements.
        ///
        /// # Panics:
        /// - if user is does not have admin in AuthZone
        pub fn create_promotion_service(&self) -> ComponentAddress {
            let service_auth = self
                .moon_work_auth
                .authorize(|| borrow_resource_manager!(self.admin_resource).mint(dec!(1)));

            PromotionServiceComponent::create(
                self.contractor,
                service_auth,
                Runtime::actor().as_component().0,
                self.completed_work_resource,
                self.contractor_accolade_resource,
                20, // requires 20 work completed
                3,  // requires all accolades up to ContractorAccolades::FirstQuarter
                10,
            )
        }

        /// Register as a contractor which mints a Contractor NFT. This is used as a badge for work
        /// and dispute services. The ID is the username/alias the user gives so no duplicate
        /// usernames/aliases are allowed.
        ///
        /// A few resources are setup in addition:
        /// 1. CompletedWork NFT is minted indicating that they have started their Contractor journey.
        /// 2. Creates a contractor vault for their earnings
        /// 3. Creates a work completed vault for all work successfully completed
        /// 4. Creates a dispute outcome vault for all dispute outcomes that has taken place
        ///
        /// # Panics
        /// - if username is taken
        pub fn register_as_contractor(&mut self, username: String) -> (Bucket, Bucket) {
            let id = NonFungibleId::from_bytes(username.as_bytes().to_vec());
            self.contractor_vault
                .insert(id.clone(), Vault::new(RADIX_TOKEN));
            self.completed_work_vault
                .insert(id.clone(), Vault::new(self.completed_work_resource));
            self.contractor_dispute_outcome_vault
                .insert(id.clone(), Vault::new(self.dispute_outcome_resource));
            self.moon_work_auth.authorize(|| {
                let contractor_badge = borrow_resource_manager!(self.contractor).mint_non_fungible(
                    &id,
                    Contractor {
                        jobs_completed: 0,
                        total_worth: Decimal::zero(),
                        disputed: 0,
                    },
                );

                let accolade_resource_manager =
                    borrow_resource_manager!(self.contractor_accolade_resource);

                let id = NonFungibleId::from_u64(
                    (accolade_resource_manager.total_supply() + 1)
                        .to_string()
                        .parse()
                        .unwrap(),
                );

                let accolade = accolade_resource_manager.mint_non_fungible(
                    &id,
                    ContractorAccolades {
                        accolade: Accolade::NewMoon,
                        work_ids: vec![],
                    },
                );

                (contractor_badge, accolade)
            })
        }

        /// Register as a Client which mints a Client NFT.
        ///
        /// A few resources are setup in addition:
        /// 1. Creates a client vault where all work created will have all payments upfront stored
        ///    in this vault
        /// 2. Creates a withdrawable vault where refunds as a result of disputes or removed work
        ///    are stored
        /// 3. Creates a dispute outcome vault for all dispute outcomes that has taken place
        ///
        /// # Panics
        /// - if username is taken
        pub fn register_as_client(&mut self, username: String) -> Bucket {
            let id = NonFungibleId::from_bytes(username.as_bytes().to_vec());
            self.client_work_vault
                .insert(id.clone(), Vault::new(RADIX_TOKEN));
            self.client_withdrawable_vault
                .insert(id.clone(), Vault::new(RADIX_TOKEN));
            self.client_dispute_outcome_vault
                .insert(id.clone(), Vault::new(self.dispute_outcome_resource));
            self.moon_work_auth.authorize(|| {
                borrow_resource_manager!(self.client).mint_non_fungible(
                    &id,
                    Client {
                        jobs_created: 0,
                        jobs_paid_out: 0,
                        total_paid_out: Decimal::zero(),
                        disputed: 0,
                    },
                )
            })
        }

        /// Gets the minimum payment amount, this is required for the work components which are
        /// used to enforce the work raised must be at or above the minimum work amount
        pub fn minimum_work_payment_amount(&self) -> Decimal {
            self.minimum_work_payment
        }

        pub fn is_client_enabled(&self, client_id: NonFungibleId) -> bool {
            let dispute_outcome_amount = self
                .client_dispute_outcome_vault
                .get(&client_id)
                .unwrap()
                .amount();

            dispute_outcome_amount == Decimal::zero()
        }

        pub fn is_contractor_enabled(&self, contractor_id: NonFungibleId) -> bool {
            let dispute_outcome_amount = self
                .contractor_dispute_outcome_vault
                .get(&contractor_id)
                .unwrap()
                .amount();

            dispute_outcome_amount == Decimal::zero()
        }

        /// When work is created, the compensation is dposited into the corresponding client vault
        ///
        /// # Panics:
        /// - if caller does not have a service badge resource in Auth Zone
        pub fn deposit_compensation(&mut self, client_id: NonFungibleId, compensation: Bucket) {
            self.client_work_vault
                .get_mut(&client_id)
                .unwrap()
                .put(compensation);
        }

        /// This is called when work has been successfully completed as a result of `Client` and
        /// `Contractor` agreeing. This method ensures the compensation from a client vault
        /// transfers the compensation to the contractor vault.
        ///
        /// # Panics:
        /// - if caller does not have a service badge resource in Auth Zone
        pub fn compensate_contractor(
            &mut self,
            client_id: NonFungibleId,
            contractor_id: NonFungibleId,
            total_compensation: Decimal,
        ) {
            let mut contractor_compensation = self
                .client_work_vault
                .get_mut(&client_id)
                .unwrap()
                .take(total_compensation);

            let service_fee_amount = (self.service_fee / 100) * contractor_compensation.amount();

            self.service_vault
                .put(contractor_compensation.take(service_fee_amount));

            self.contractor_vault
                .get_mut(&contractor_id)
                .unwrap()
                .put(contractor_compensation);
        }

        /// Due to a dispute or removing work, this method is called. As a result of this, the
        /// client work originally meant for a contractor's work is transferred to the client
        /// withdrawable vault
        ///
        /// # Panics:
        /// - if caller does not have a service badge resource in Auth Zone
        pub fn refund_client(&mut self, client_id: NonFungibleId, refund_amount: Decimal) {
            let client_work_refund_bucket = self
                .client_work_vault
                .get_mut(&client_id)
                .unwrap()
                .take(refund_amount);

            self.client_withdrawable_vault
                .get_mut(&client_id)
                .unwrap()
                .put(client_work_refund_bucket);
        }

        /// Assuming disputes have been handled and client has won the dispute or if the client
        /// created work and wanted to remove work, all the amounts are then transferred to
        /// a refund vault. This method does the following:
        ///
        /// 1. Get all refunds its owed as a result of the reasons said above
        /// 2. Returns a soulbound Dispute NFT with their record and result
        ///
        /// # Panics:
        /// - if `Client` proof has incorrect `ResourceAddress`
        pub fn claim_client_work_refund(&mut self, client: Proof) -> (Bucket, Bucket) {
            let validated_client = client
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.client,
                    dec!(1),
                ))
                .expect("unauthorized access");

            let client_refund = self
                .client_withdrawable_vault
                .get_mut(&validated_client.non_fungible_id())
                .unwrap()
                .take_all();

            let completed_dispute = self.moon_work_auth.authorize(|| {
                self.client_dispute_outcome_vault
                    .get_mut(&validated_client.non_fungible_id())
                    .unwrap()
                    .take_all()
            });

            let client_resource_manager = borrow_resource_manager!(self.client);

            let mut client = client_resource_manager
                .get_non_fungible_data::<Client>(&validated_client.non_fungible_id());

            client.disputed += completed_dispute
                .amount()
                .to_string()
                .parse::<u64>()
                .unwrap();

            self.moon_work_auth.authorize(|| {
                client_resource_manager
                    .update_non_fungible_data(&validated_client.non_fungible_id(), client);
            });

            (client_refund, completed_dispute)
        }

        /// When work has been completed successfully, we mint a WorkCompleted NFT to indicate
        /// this. Because this only happens when called by a work component, the `CompletedWork` is
        /// stored in a completed work vault
        ///
        /// # Panics:
        /// - if `Contractor` has an incorrect `ResourceAddress`
        pub fn finalise_work(
            &mut self,
            category_resource: ResourceAddress,
            work_id: NonFungibleId,
            total_compensation: Decimal,
            contractor: NonFungibleId,
        ) {
            let completed_work_resource_manager =
                borrow_resource_manager!(self.completed_work_resource);

            let id = NonFungibleId::from_u64(
                (completed_work_resource_manager.total_supply() + 1)
                    .to_string()
                    .parse()
                    .unwrap(),
            );

            let completed_work = self.moon_work_auth.authorize(|| {
                completed_work_resource_manager.mint_non_fungible(
                    &id,
                    CompletedWork {
                        category_resource,
                        work_id,
                        total_compensation,
                    },
                )
            });

            self.completed_work_vault
                .get_mut(&contractor)
                .unwrap()
                .put(completed_work);
        }

        /// This is the meat of the additional rewards system for Contractors. This method does
        /// a few things under the hood:
        ///
        /// 1. We return all payment compensation (XRD)
        /// 2. We return all `CompletedWork` NFTs
        /// 3. For every 10 work completed, we mint an `ContractorAccolades` NFT with all the Work
        ///    IDs used to win that accolade. Because of this, its a truly unique NFT for the
        ///    individual Contractor
        /// 4. We also return all `DisputeOutcome` NFTs for every dispute they Won/Lost
        ///    respectively
        pub fn claim_contractor_compensation(
            &mut self,
            contractor: Proof,
        ) -> (Bucket, Bucket, Bucket, Bucket) {
            let validated_contractor = contractor
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.contractor,
                    dec!(1),
                ))
                .expect("unauthorized access");
            let contractor_compensation = self
                .contractor_vault
                .get_mut(&validated_contractor.non_fungible_id())
                .unwrap()
                .take_all();

            let completed_work = self.moon_work_auth.authorize(|| {
                let completed_work = self
                    .completed_work_vault
                    .get_mut(&validated_contractor.non_fungible_id())
                    .unwrap()
                    .take_all();

                completed_work
            });

            let contractor_resource_manager = borrow_resource_manager!(self.contractor);

            let contractor_nft = contractor_resource_manager
                .get_non_fungible_data::<Contractor>(&validated_contractor.non_fungible_id());

            let mut current_acollade = Accolade::get_accolade(contractor_nft.jobs_completed);
            let mut new_accolades: Vec<ContractorAccolades> = vec![];
            let mut work_ids: Vec<NonFungibleAddress> = vec![];

            let mut updated_contractor = completed_work
                .non_fungibles::<CompletedWork>()
                .iter()
                .fold(contractor_nft, |mut contractor, work| {
                    let data = work.data();
                    contractor.jobs_completed = contractor.jobs_completed + 1;
                    contractor.total_worth = contractor.total_worth + data.total_compensation;

                    work_ids.push(work.address());

                    if current_acollade != Accolade::get_accolade(contractor.jobs_completed) {
                        new_accolades.push(ContractorAccolades {
                            accolade: Accolade::get_accolade(contractor.jobs_completed),
                            work_ids: work_ids.clone(),
                        });
                        current_acollade = Accolade::get_accolade(contractor.jobs_completed);
                        work_ids = vec![];
                    }

                    contractor
                });

            let contractor_accolade_resource_manager =
                borrow_resource_manager!(self.contractor_accolade_resource);

            let mut accolade_bucket = Bucket::new(self.contractor_accolade_resource);

            let completed_dispute = self.moon_work_auth.authorize(|| {
                self.contractor_dispute_outcome_vault
                    .get_mut(&validated_contractor.non_fungible_id())
                    .unwrap()
                    .take_all()
            });

            updated_contractor.disputed += completed_dispute
                .amount()
                .to_string()
                .parse::<u64>()
                .unwrap();

            self.moon_work_auth.authorize(|| {
                new_accolades.into_iter().for_each(|accolade| {
                    let id = NonFungibleId::from_u64(
                        (contractor_accolade_resource_manager.total_supply() + 1)
                            .to_string()
                            .parse()
                            .unwrap(),
                    );

                    let new_accolade =
                        contractor_accolade_resource_manager.mint_non_fungible(&id, accolade);
                    accolade_bucket.put(new_accolade);
                });
                contractor_resource_manager.update_non_fungible_data(
                    &validated_contractor.non_fungible_id(),
                    updated_contractor,
                );
            });

            (
                contractor_compensation,
                completed_work,
                accolade_bucket,
                completed_dispute,
            )
        }

        /// This is where we create 2 components to make sure we keep components scalable. For
        /// a new work category, this method:
        ///
        /// 1. Creates a new `WorkServiceComponent`
        /// 2. Creates a corresponding `DisputeServiceComponent`
        ///
        /// # Panics:
        /// - if admin badge is not in the Auth Zone
        pub fn create_new_category(
            &mut self,
            work_type: String,
        ) -> (ComponentAddress, ComponentAddress) {
            let mut service_auth = self
                .moon_work_auth
                .authorize(|| borrow_resource_manager!(self.admin_resource).mint(dec!(2)));

            let (moon_work_component_address, _, _) = Runtime::actor().as_component();

            let work_component = WorkServiceComponent::create(
                work_type,
                moon_work_component_address,
                service_auth.take(1),
                self.client,
                self.contractor,
            );

            let work_service: WorkServiceComponent = work_component.into();

            let dispute_service = DisputeServiceComponent::create(
                service_auth,
                self.client,
                self.contractor,
                work_service.get_work_resource(),
                moon_work_component_address,
            );

            (work_component, dispute_service)
        }

        /// Gets the participant criteria used for disputes
        pub fn get_dispute_participant_criteria(&self) -> ParticipantCriteria {
            self.dispute_participant_criteria
        }

        /// Updates the participant criteria used for disputes
        pub fn update_dispute_participant_criteria(
            &mut self,
            participant_criteria: ParticipantCriteria,
        ) {
            self.dispute_participant_criteria = participant_criteria;
        }

        /// This completes the outcome assuming the `DisputeServiceComponent` has reached a verdict
        /// for both `Client` and `Contractor`. This method simply mints a `DisputeOutcome` NFT
        /// which is then put into the corresponding `Client` or `Contractor` dispute outcome vault
        ///
        /// # Panics:
        /// - if admin badge is not in Auth Zone
        pub fn complete_dispute_outcome(
            &mut self,
            dispute_side: DisputeSide,
            client_or_contractor_id: NonFungibleId,
            work: NonFungibleAddress,
            decision: DisputeDecision,
        ) {
            let dispute_outcome_resource_manager =
                borrow_resource_manager!(self.dispute_outcome_resource);

            let id = NonFungibleId::from_u64(
                (dispute_outcome_resource_manager.total_supply() + 1)
                    .to_string()
                    .parse()
                    .unwrap(),
            );

            let completed_dispute = self.moon_work_auth.authorize(|| {
                dispute_outcome_resource_manager
                    .mint_non_fungible(&id, DisputeOutcome { work, decision })
            });

            match dispute_side {
                DisputeSide::Contractor => {
                    self.contractor_dispute_outcome_vault
                        .get_mut(&client_or_contractor_id)
                        .unwrap()
                        .put(completed_dispute);
                }
                DisputeSide::Client => {
                    self.client_dispute_outcome_vault
                        .get_mut(&client_or_contractor_id)
                        .unwrap()
                        .put(completed_dispute);
                }
            }
        }
    }
}
