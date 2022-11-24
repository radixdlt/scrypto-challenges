use crate::users::{Client, Contractor};
use scrypto::prelude::*;

#[derive(Debug, Describe, TypeId, Encode, Decode, PartialEq, Eq)]
pub enum WorkStatus {
    NotStarted,
    InProgress,
    Finished,
    InDispute,
    Disputed,
    Delisted,
}

#[derive(Debug, NonFungibleData)]
pub struct Work {
    pub client: NonFungibleId,
    pub total_compensation: Decimal,
    pub work_title: String,
    pub work_description: String,
    #[scrypto(mutable)]
    pub work_status: WorkStatus,
    pub skills_required: Vec<String>,
    #[scrypto(mutable)]
    pub contractor_requests: HashMap<NonFungibleId, bool>,
    #[scrypto(mutable)]
    pub contractor_assigned: Option<NonFungibleId>,
}

blueprint! {
    struct WorkService {
        contractor: ResourceAddress,
        client: ResourceAddress,
        work_resource: ResourceAddress,
        service_auth: Vault,
        service_component: ComponentAddress,
    }

    impl WorkService {
        /// Creates an independant work category.
        ///
        /// This is designed to be a reusable component so
        /// callable methods can be made by creating a service componet blueprint
        ///
        /// # ParentComponent Methods to Implement:
        /// ```ignore
        /// use scrypto::prelude::*;
        ///
        /// blueprint! {
        ///
        /// struct Service {
        /// }
        ///
        /// impl Service {
        ///     pub fn minimum_work_payment_amount(&self) -> Decimal {
        ///       todo!()
        ///     }
        ///     pub fn deposit_compensation(&self, contractor_id: NonFungibleId, xrd_payment: Bucket) {
        ///       todo!()
        ///     }
        ///     pub fn refund_client(&self, client_id: NonFungibleId, refund_amount: Bucket) {
        ///       todo!()
        ///     }
        ///     pub fn compensate_contractor(
        ///       &self,
        ///       client_id: NonFungibleId,
        ///       contractor_id: NonFungibleId,
        ///       total_compensation: Decimal
        ///     ) {
        ///       todo!()
        ///     }
        ///     pub fn finalise_work(
        ///       &self,
        ///       work_resource: ResourceAddress,
        ///       work_id: NonFungibleId,
        ///       total_compensation: Decimal,
        ///       contractor_id: NonFungibleId
        ///     ) {
        ///       todo!()
        ///     }
        ///     pub fn is_client_enabled(
        ///       &self,
        ///       client_id: NonFungibleId
        ///     ) -> bool {
        ///       todo!()
        ///     }
        ///     pub fn is_contractor_enabled(
        ///       &self,
        ///       contractor_id: NonFungibleId
        ///     ) -> bool {
        ///       todo!()
        ///     }
        ///   }
        /// }
        /// ```
        ///
        /// # Arguments:
        /// - `category` name of the work category, for example Development & IT, Accounting
        /// & Finance etc
        /// - `service_component` the parent component, in this case moonwork service, you will
        /// require certain methods to be implemented if you wanted your own service
        /// - `service_badge` the service badge is required in this service as we need access rules
        /// to update resources, it also has access to work resource. Alongside this, we need to be
        /// able to call moonwork service methods which are only callable by admin
        /// - `client` This resource must be an NFT
        /// - `contractor` This resource must be an NFT
        pub fn create(
            category: String,
            service_component: ComponentAddress,
            service_badge: Bucket,
            client: ResourceAddress,
            contractor: ResourceAddress,
        ) -> ComponentAddress {
            let work_resource = ResourceBuilder::new_non_fungible()
                .metadata("name", category)
                .mintable(rule!(require(service_badge.resource_address())), LOCKED)
                .burnable(rule!(require(service_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(service_badge.resource_address())),
                    LOCKED,
                )
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();

            let component = Self {
                contractor,
                client,
                work_resource,
                service_auth: Vault::with_bucket(service_badge),
                service_component,
            }
            .instantiate();

            component.globalize()
        }

        /// Simply returns the work resource address, required for the service component
        pub fn get_work_resource(&self) -> ResourceAddress {
            self.work_resource
        }

        /// Allows a client to create work for contractors to pick up. To keep the exchange as safe
        /// as possible, the client needs the payment amount upfront which is then stored in
        /// a client bucket. This method in summary does 2 things:
        /// 1. Mint a Work NFT which is then returned to the Client as a **soulbound** token
        /// 2. Takes XRD as the payment currency and is stored on a client payment vault which is
        ///    stored in a parent component
        ///
        /// # Arguments:
        /// - `total_compensation` - The total compensation that the work pays out to be. This is
        /// additionally validated by the xrd_payment bucket that is given matches the parameter
        /// - `work_title` - Metadata representing the title
        /// - `work_description` - Metadata representing a short description of the description of
        /// work
        /// - `skills_required` - Metadata list of keyword skillsets required by the contractor
        /// - `client` - A proof representing the client badge
        ///
        /// # Panics:
        /// - If uses a different resource as a proof
        /// - If `xrd_payment` is not a `RADIX_TOKEN` resource
        /// - If `xrd_payment` amount does not need minimum work payment
        /// - If `xrd_payment` does is not equal to `total_compensation`
        pub fn create_new_work(
            &self,
            total_compensation: Decimal,
            work_title: String,
            work_description: String,
            skills_required: Vec<String>,
            xrd_payment: Bucket,
            client: Proof,
        ) -> Bucket {
            let validated_client = self.validate_client(client).expect("unauthorized access");

            self.service_auth.authorize(|| {
                assert!(
                    borrow_component!(self.service_component).call::<bool>(
                        "is_client_enabled",
                        args!(validated_client.non_fungible_id())
                    ),
                    "disputes pending withdrawal"
                );
            });

            assert!(
                xrd_payment.resource_address() == RADIX_TOKEN,
                "invalid payment"
            );

            let minimum_work_payment = borrow_component!(self.service_component)
                .call::<Decimal>("minimum_work_payment_amount", args!());

            assert!(
                xrd_payment.amount() >= minimum_work_payment,
                "minimum payment not met"
            );
            assert!(xrd_payment.amount() == total_compensation, "invalid amount");

            self.update_client_jobs_created(&validated_client);

            let work = self.mint_new_work(
                &validated_client,
                total_compensation,
                work_description,
                work_title,
                skills_required,
            );

            self.service_auth.authorize(|| {
                borrow_component!(self.service_component).call::<()>(
                    "deposit_compensation",
                    args!(validated_client.non_fungible_id(), xrd_payment),
                );
            });

            work
        }

        /// Removes work only if it has not started.
        ///
        /// This method does 2 things:
        /// - Update the Work NFT Work Status to `Delisted`. This is so that we can update the frontend
        /// - Refunds the client by placing the total compensation amount to a client refund vault
        ///
        /// # Panics:
        /// - if work is not a valid resource address
        pub fn remove_work(&self, work: Proof) {
            let validated_work = self.validate_work(work).expect("invalid work");

            let work_nft = validated_work.non_fungible::<Work>().data();

            self.service_auth.authorize(|| {
                assert!(
                    borrow_component!(self.service_component)
                        .call::<bool>("is_client_enabled", args!(work_nft.client)),
                    "disputes pending withdrawal"
                );
            });

            self.update_work_delist(validated_work, work_nft);
        }

        /// Contractor requests for work that has been raised by the client
        ///
        /// This method updates Work NFT to include contractor `NonFungibleId` into
        ///    `Work.contractor_requests`
        ///
        /// # Panics:
        /// - if contractor proof is not the correct `ResourceAddress`
        /// - if work has not already started
        /// - if you try to request for a job you already requested
        pub fn request_work(&self, work_id: NonFungibleId, contractor: Proof) {
            let validated_contractor = self
                .validate_contractor(contractor)
                .expect("unauthorized access");

            self.service_auth.authorize(|| {
                assert!(
                    borrow_component!(self.service_component).call::<bool>(
                        "is_contractor_enabled",
                        args!(validated_contractor.non_fungible_id())
                    ),
                    "disputes pending withdrawal"
                );
            });

            let work_resource_manager = borrow_resource_manager!(self.work_resource);

            let work = work_resource_manager.get_non_fungible_data::<Work>(&work_id);

            assert!(
                work.work_status == WorkStatus::NotStarted,
                "work must not be started"
            );

            assert!(
                work.contractor_requests
                    .get(&validated_contractor.non_fungible_id())
                    .is_none(),
                "already requested job"
            );

            self.update_work_with_contractor_request(
                work_resource_manager,
                validated_contractor,
                work,
                work_id,
            );
        }

        /// Assuming work has been requested by contractors, we accept a specific contractor to
        /// take on the work.
        ///
        /// When client accepts a contractor for a particular work:
        /// 1. Updates `Work.contractor_assigned` to the contractor `NonFungibleId` given
        /// 2. Updates `Work.work_status` to `InProgress`
        ///
        /// # Panics:
        /// - if client has an incorrrect `ResourceAddress`
        /// - if contractor `NonFungibleId` is incorrect/does not exist
        pub fn accept_contractor_for_work(
            &self,
            work: Proof,
            contractor: NonFungibleId,
            client: Proof,
        ) {
            let validated_client = self.validate_client(client).expect("unauthorized access");
            let validated_work = self.validate_work(work).expect("incorrect work");

            self.service_auth.authorize(|| {
                assert!(
                    borrow_component!(self.service_component).call::<bool>(
                        "is_client_enabled",
                        args!(validated_client.non_fungible_id())
                    ),
                    "disputes pending withdrawal"
                );
            });

            self.update_work_with_assigned_contractor(contractor, validated_work);
        }

        /// If both client and contractor agrees that the work has been completed.
        /// This requires multi-signature from the client and contractor
        /// This method successfully finishes the work which does the following:
        ///
        /// 1. Updates the `Client` NFT `Client.jobs_paid_out` and `Client.total_paid_out` from
        ///    information of `Work` NFT
        /// 2. We compensate the contractor externally from the service component
        /// 3. Mint a `CompletedWork` NFT which is deposited to a WorkCompleted vault
        ///
        /// # Panics:
        /// - if work `ResourceAddress` is invalid
        /// - if client `ResourceAddress` is invalid
        /// - if contractor `ResourceAddress` is invalid
        pub fn finish_work(&mut self, work: Proof, client: Proof, contractor: Proof) {
            let validated_work = self.validate_work(work).expect("invalid work");

            let validated_client = self.validate_client(client).expect("unauthorized access");

            let validated_contractor = self
                .validate_contractor(contractor)
                .expect("unauthorized access");

            self.service_auth.authorize(|| {
                assert!(
                    borrow_component!(self.service_component).call::<bool>(
                        "is_client_enabled",
                        args!(validated_client.non_fungible_id())
                    ),
                    "disputes pending withdrawal"
                );
            });

            self.service_auth.authorize(|| {
                assert!(
                    borrow_component!(self.service_component).call::<bool>(
                        "is_contractor_enabled",
                        args!(validated_contractor.non_fungible_id())
                    ),
                    "disputes pending withdrawal"
                );
            });

            let client_resource_manager = borrow_resource_manager!(self.client);
            let work_resource_manager = borrow_resource_manager!(self.work_resource);
            let service_component = borrow_component!(self.service_component);

            let client_nft = client_resource_manager
                .get_non_fungible_data::<Client>(&validated_client.non_fungible_id());
            let work_nft = work_resource_manager
                .get_non_fungible_data::<Work>(&validated_work.non_fungible_id());

            self.service_auth.authorize(|| {
                self.compensate_contractor(
                    service_component,
                    &validated_client,
                    &validated_contractor,
                    &work_nft,
                );

                self.finalise_work(
                    service_component,
                    &validated_work,
                    &work_nft,
                    validated_contractor,
                );

                self.update_client_total_paid_out(
                    client_resource_manager,
                    validated_client,
                    client_nft,
                    &work_nft,
                );

                self.update_work_to_finished(work_resource_manager, validated_work, work_nft);
            });
        }

        /// Mints a new work from arguments given
        fn mint_new_work(
            &self,
            validated_client: &ValidatedProof,
            total_compensation: Decimal,
            work_description: String,
            work_title: String,
            skills_required: Vec<String>,
        ) -> Bucket {
            let work_resource_manager = borrow_resource_manager!(self.work_resource);
            let id = NonFungibleId::from_u64(
                (work_resource_manager.total_supply() + 1)
                    .to_string()
                    .parse()
                    .unwrap(),
            );
            let work = self.service_auth.authorize(|| {
                work_resource_manager.mint_non_fungible(
                    &id,
                    Work {
                        client: validated_client.non_fungible_id(),
                        total_compensation,
                        work_description,
                        work_title,
                        work_status: WorkStatus::NotStarted,
                        skills_required,
                        contractor_requests: HashMap::new(),
                        contractor_assigned: None,
                    },
                )
            });
            work
        }

        /// Update a work NFT to be marked as delisted
        fn update_work_delist(&self, validated_work: ValidatedProof, work_nft: Work) {
            self.service_auth.authorize(|| {
                borrow_resource_manager!(self.work_resource).update_non_fungible_data(
                    &validated_work.non_fungible_id(),
                    Work {
                        client: work_nft.client.clone(),
                        total_compensation: work_nft.total_compensation,
                        work_title: work_nft.work_title,
                        work_description: work_nft.work_description,
                        work_status: WorkStatus::Delisted,
                        skills_required: work_nft.skills_required,
                        contractor_requests: work_nft.contractor_requests,
                        contractor_assigned: work_nft.contractor_assigned,
                    },
                );

                borrow_component!(self.service_component).call::<()>(
                    "refund_client",
                    args!(work_nft.client, work_nft.total_compensation),
                );
            });
        }

        /// Update client to increment jobs created
        fn update_client_jobs_created(&self, validated_client: &ValidatedProof) {
            let client_resource_manager = borrow_resource_manager!(self.client);
            let client = client_resource_manager
                .get_non_fungible_data::<Client>(&validated_client.non_fungible_id());
            self.service_auth.authorize(|| {
                client_resource_manager.update_non_fungible_data(
                    &validated_client.non_fungible_id(),
                    Client {
                        jobs_created: client.jobs_created + 1,
                        total_paid_out: client.total_paid_out,
                        disputed: client.disputed,
                        jobs_paid_out: client.jobs_paid_out,
                    },
                );
            });
        }

        /// Validate the client for the correct `ResourceAddress`
        fn validate_client(
            &self,
            client: Proof,
        ) -> Result<ValidatedProof, (Proof, ProofValidationError)> {
            client.validate_proof(ProofValidationMode::ValidateContainsAmount(
                self.client,
                dec!(1),
            ))
        }

        /// Validate work ResourceAddress
        fn validate_work(
            &self,
            work: Proof,
        ) -> Result<ValidatedProof, (Proof, ProofValidationError)> {
            work.validate_proof(ProofValidationMode::ValidateContainsAmount(
                self.work_resource,
                dec!(1),
            ))
        }

        /// Validate that the use is a contractor
        fn validate_contractor(
            &self,
            contractor: Proof,
        ) -> Result<ValidatedProof, (Proof, ProofValidationError)> {
            contractor.validate_proof(ProofValidationMode::ValidateContainsAmount(
                self.contractor,
                dec!(1),
            ))
        }

        // Assign a contractor to the work NFT
        fn update_work_with_assigned_contractor(
            &self,
            contractor: NonFungibleId,
            validated_work: ValidatedProof,
        ) {
            let contractor_resource_manager = borrow_resource_manager!(self.contractor);
            contractor_resource_manager.get_non_fungible_data::<Contractor>(&contractor);
            let work_resource_manager = borrow_resource_manager!(self.work_resource);
            let work_nft = work_resource_manager
                .get_non_fungible_data::<Work>(&validated_work.non_fungible_id());
            self.service_auth.authorize(|| {
                work_resource_manager.update_non_fungible_data(
                    &validated_work.non_fungible_id(),
                    Work {
                        client: work_nft.client,
                        total_compensation: work_nft.total_compensation,
                        work_title: work_nft.work_title,
                        work_description: work_nft.work_description,
                        work_status: WorkStatus::InProgress,
                        skills_required: work_nft.skills_required,
                        contractor_requests: work_nft.contractor_requests,
                        contractor_assigned: Some(contractor),
                    },
                )
            });
        }

        /// Update work NFT with the contractor non fungible ID
        fn update_work_with_contractor_request(
            &self,
            work_resource_manager: &mut ResourceManager,
            validated_contractor: ValidatedProof,
            mut work: Work,
            work_id: NonFungibleId,
        ) {
            work.contractor_requests
                .insert(validated_contractor.non_fungible_id(), true);
            self.service_auth.authorize(|| {
                work_resource_manager.update_non_fungible_data(
                    &work_id,
                    Work {
                        client: work.client,
                        total_compensation: work.total_compensation,
                        work_description: work.work_description,
                        work_title: work.work_title,
                        work_status: work.work_status,
                        skills_required: work.skills_required,
                        contractor_requests: work.contractor_requests,
                        contractor_assigned: work.contractor_assigned,
                    },
                )
            });
        }

        /// Calls finalise work from the service component
        fn finalise_work(
            &self,
            service_component: &Component,
            validated_work: &ValidatedProof,
            work_nft: &Work,
            validated_contractor: ValidatedProof,
        ) {
            service_component.call::<()>(
                "finalise_work",
                args!(
                    self.work_resource,
                    validated_work.non_fungible_id(),
                    work_nft.total_compensation,
                    validated_contractor.non_fungible_id()
                ),
            );
        }

        // Calls to compensate the contractor from the service component
        fn compensate_contractor(
            &self,
            service_component: &Component,
            validated_client: &ValidatedProof,
            validated_contractor: &ValidatedProof,
            work_nft: &Work,
        ) {
            service_component.call::<()>(
                "compensate_contractor",
                args!(
                    validated_client.non_fungible_id(),
                    validated_contractor.non_fungible_id(),
                    work_nft.total_compensation
                ),
            );
        }

        fn update_work_to_finished(
            &self,
            work_resource_manager: &mut ResourceManager,
            validated_work: ValidatedProof,
            work_nft: Work,
        ) {
            work_resource_manager.update_non_fungible_data(
                &validated_work.non_fungible_id(),
                Work {
                    client: work_nft.client,
                    total_compensation: work_nft.total_compensation,
                    work_title: work_nft.work_title,
                    work_description: work_nft.work_description,
                    work_status: WorkStatus::Finished,
                    skills_required: work_nft.skills_required,
                    contractor_requests: work_nft.contractor_requests,
                    contractor_assigned: work_nft.contractor_assigned,
                },
            );
        }

        fn update_client_total_paid_out(
            &self,
            client_resource_manager: &mut ResourceManager,
            validated_client: ValidatedProof,
            client_nft: Client,
            work_nft: &Work,
        ) {
            client_resource_manager.update_non_fungible_data(
                &validated_client.non_fungible_id(),
                Client {
                    jobs_created: client_nft.jobs_created,
                    total_paid_out: client_nft.total_paid_out + work_nft.total_compensation,
                    disputed: client_nft.disputed,
                    jobs_paid_out: client_nft.jobs_paid_out + 1,
                },
            );
        }
    }
}
