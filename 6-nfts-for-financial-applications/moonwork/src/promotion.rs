use scrypto::prelude::*;

// Assuming an epoch interval is 30mins, 10 days expiry time
const PROMOTION_EXPIRATION_TIME: u64 = 480;

#[derive(Debug, NonFungibleData)]
pub struct PromotedContractor {
    pub expiry: u64,
    pub contractor_id: NonFungibleId,
}

blueprint! {
    struct PromotionService {
        contractor: ResourceAddress,
        service_auth: Vault,
        service_component: ComponentAddress,
        promoted_contractors: Vault,
        // because we burn, its unreliable to use total_supply + 1 for non fungible ids, so this
        // is another way of minting a unique id everytime
        latest_promoted_contractors_id: u64,
        completed_work_required: u8,
        accolades_required: u8,
        promoted_contractors_limit: u64,
        completed_work_resource: ResourceAddress,
        accolades_resource: ResourceAddress,
    }

    impl PromotionService {
        /// This is an example of how to then leverage and verify a contractor's credentials and
        /// work done by presenting WorkCompleted NFTs and ContractorAccolades NFTs. This can be
        /// further evolved by allow client's work to be promoted as well given some requirements
        /// such as: number of work finish must exceed 10, as an example
        pub fn create(
            contractor: ResourceAddress,
            service_badge: Bucket,
            service_component: ComponentAddress,
            completed_work_resource: ResourceAddress,
            accolades_resource: ResourceAddress,
            completed_work_required: u8,
            accolades_required: u8,
            promoted_contractors_limit: u64,
        ) -> ComponentAddress {
            let promoted_contractors = ResourceBuilder::new_non_fungible()
                .metadata("name", "Promoted Contractors")
                .mintable(rule!(require(service_badge.resource_address())), LOCKED)
                .burnable(rule!(require(service_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(service_badge.resource_address())),
                    LOCKED,
                )
                .no_initial_supply();

            let access_rules = AccessRules::new()
                // Ideally wanted to use access rules to enforce number of completed work and
                // accolades required
                // .method(
                //     "promote_contractor",
                //     rule!(
                //         require_n_of("completed_work_required", vec![completed_work])
                //             && require_n_of("accolades_required", vec![accolade])
                //     ),
                // )
                .method(
                    "update_contractor_promotion_requirements",
                    rule!(require(service_badge.resource_address())),
                )
                .method(
                    "remove_expired_promotions",
                    rule!(require(service_badge.resource_address())),
                )
                .default(AccessRule::AllowAll);

            let mut component = Self {
                contractor,
                service_auth: Vault::with_bucket(service_badge),
                service_component,
                promoted_contractors: Vault::new(promoted_contractors),
                completed_work_resource,
                accolades_resource,
                completed_work_required,
                accolades_required,
                promoted_contractors_limit,
                latest_promoted_contractors_id: 0,
            }
            .instantiate();

            component.add_access_check(access_rules);

            component.globalize()
        }

        /// Simple admin method which is used to tweak requirements for being able to promote
        /// a contractor.
        pub fn update_contractor_promotion_requirements(
            &mut self,
            new_completed_work: u8,
            accolades_required: u8,
        ) {
            self.completed_work_required = new_completed_work;
            self.accolades_required = accolades_required;
        }

        /// Its important to call this every so often, not really thought out. Perhaps while
        /// querying using the gateway API, checks if there has been a build up of expired
        /// promotions and must be cleaned up in an automated manner.
        pub fn remove_expired_promotions(&mut self) {
            let expired_promoted_contractors: Vec<NonFungibleId> = self
                .promoted_contractors
                .non_fungibles::<PromotedContractor>()
                .iter()
                .filter(|promoted_contractor| {
                    Runtime::current_epoch() > promoted_contractor.data().expiry
                })
                .map(|promoted_contractor| promoted_contractor.id())
                .collect();

            self.service_auth.authorize(|| {
                let resource_manager =
                    borrow_resource_manager!(self.promoted_contractors.resource_address());
                for expired_promotions in expired_promoted_contractors {
                    resource_manager.burn(
                        self.promoted_contractors
                            .take_non_fungible(&expired_promotions),
                    );
                }
            })
        }

        /// Contractors must present their work done and accolades collected so they have the
        /// priviledge to promote themselves. Ideally we should use access rules instead of
        /// explicit proof validations on a method level.
        ///
        /// # Panics:
        /// - if not enough work completed proofs are given
        /// - if not enough accolade proofs are given
        /// - if the limit for promotions has been reached
        /// - if contractor is already promoted
        pub fn promote_contractor(
            &mut self,
            work_completed: Proof,
            accolades: Proof,
            contractor: Proof,
        ) {
            let has_work_completed = self.validate_enough_work_completed(work_completed);
            let has_accolades = self.validate_enough_accolades_earned(accolades);

            assert!(
                has_work_completed.is_ok() && has_accolades.is_ok(),
                "not enough work"
            );

            let validated_contractor = self
                .validate_contractor(contractor)
                .expect("unauthorized access");

            let non_expired_promoted_contractors = self.get_non_expired_promoted_contractors();

            assert!(
                non_expired_promoted_contractors
                    .iter()
                    .find(|c| c.data().contractor_id == validated_contractor.non_fungible_id())
                    .is_none(),
                "already promoted"
            );

            let non_expired_promoted_count = non_expired_promoted_contractors.len();

            assert!(
                non_expired_promoted_count as u64 != self.promoted_contractors_limit,
                "promoted limit reached"
            );

            let promoted_contractor = self.mint_promotion_for_contractor(validated_contractor);

            self.promoted_contractors.put(promoted_contractor);
        }

        fn mint_promotion_for_contractor(
            &mut self,
            validated_contractor: ValidatedProof,
        ) -> Bucket {
            self.latest_promoted_contractors_id = self.latest_promoted_contractors_id + 1;

            let id = NonFungibleId::from_u64(self.latest_promoted_contractors_id);
            self.service_auth.authorize(|| {
                borrow_resource_manager!(self.promoted_contractors.resource_address())
                    .mint_non_fungible(
                        &id,
                        PromotedContractor {
                            expiry: Runtime::current_epoch() + PROMOTION_EXPIRATION_TIME,
                            contractor_id: validated_contractor.non_fungible_id(),
                        },
                    )
            })
        }

        fn get_non_expired_promoted_contractors(&mut self) -> Vec<NonFungible<PromotedContractor>> {
            let promoted_contractors = self
                .promoted_contractors
                .non_fungibles::<PromotedContractor>();
            let non_expired_promoted_contractors =
                promoted_contractors
                    .into_iter()
                    .filter(|promoted_contractor| {
                        Runtime::current_epoch() <= promoted_contractor.data().expiry
                    });
            non_expired_promoted_contractors.collect()
        }

        fn validate_contractor(
            &mut self,
            contractor: Proof,
        ) -> Result<ValidatedProof, (Proof, ProofValidationError)> {
            contractor.validate_proof(ProofValidationMode::ValidateContainsAmount(
                self.contractor,
                dec!(1),
            ))
        }

        fn validate_enough_accolades_earned(
            &mut self,
            accolades: Proof,
        ) -> Result<ValidatedProof, (Proof, ProofValidationError)> {
            accolades.validate_proof(ProofValidationMode::ValidateContainsAmount(
                self.accolades_resource,
                self.accolades_required.into(),
            ))
        }

        fn validate_enough_work_completed(
            &mut self,
            work_completed: Proof,
        ) -> Result<ValidatedProof, (Proof, ProofValidationError)> {
            work_completed.validate_proof(ProofValidationMode::ValidateContainsAmount(
                self.completed_work_resource,
                self.completed_work_required.into(),
            ))
        }
    }
}
