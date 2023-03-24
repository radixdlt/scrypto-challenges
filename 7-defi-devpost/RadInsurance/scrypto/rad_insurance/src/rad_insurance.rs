use crate::policy::{PolicyComponent, PolicyInfo};
use crate::rad_insurance_badge_manager::{BadgeType, RadInsuranceBadgeManager};
use enum_iterator::all;
use scrypto::prelude::*;
#[blueprint]
mod RadInsuranceModule {

    pub struct RadInsurance {
        // Store the insurance policies component address
        policies: HashMap<u128, ComponentAddress>,
        // Store the archived insurance policies component address
        archive_policies: HashMap<u128, ComponentAddress>,
        // Corresponds to the service fees paid when subscribing to an insurance policy and when investing in an insurance policy
        services_fees: Decimal,
        // Corresponds to the Resource address use by all policies.
        resource_address: ResourceAddress,
        // ResourceAddress of admin_badge : this admin_badge is return to user who instanciate the component
        admin_badge_resource_address: ResourceAddress,
        // Store all necessary resources address badges for the operation of this blueprint
        resource_address_by_badge_type: HashMap<u16, ResourceAddress>,
        // Corresponds to the plueprint badge manager
        rad_insurance_badge_manager: RadInsuranceBadgeManager,
    }

    //This blueprint outlines the features of decentralized insurance
    //We have decided to implement a simplified version as part of this challenge
    //The objective being to improve the implementation with a much more robust protocol (If you have extensive knowledge of insurance, we can join forces)
    //The simplified protocol is as follows:
    // - Insurers invest in an insurance policy and are rewarded thanks to the contributions made by the insured (the rate of reward of the insurers is equal to the rate of contributions of the insured).
    // - For N XRD invested, the insurance policy will cover N XRD at most.
    // - Insurers cannot withdraw their investment but can sell their investment on the market place of insurers in case of liquidity need.
    // - The service fees are fixed and are deducted during the investment of the insurers and the subscription of the insured.
    //This blueprint allows:
    //- An administrator to create an insurance policy (see the create_policy method).
    //- Anonymous users to invest liquidity in an insurance policy and thus become an insurer (invest_as_insurer method)
    //- Insurers to withdraw their rewards (rewards_withdrawal method)
    //- Anonymous users to subscribe to an insurance policy and thus become insured (subscribe_to_insurance_policy method) .
    //- Insured to report a claim (method: report_a_claim )
    //- The administrator to accept the claim report and allow the insured to withdraw the amount (make_claim_as_accepted)
    impl RadInsurance {
        // This function instanciate the RadInsurance
        pub fn instanciate_rad_insurance(
            //Corresponds to the service fees paid when subscribing to an insurance policy and when investing in an insurance policy
            services_fees: Decimal,
            // corresponds to the Resource address use by all policies.
            resource_address: ResourceAddress,
        ) -> (ComponentAddress, Bucket) {
            //create admin_badge bucket with one supply
            let admin_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Admin Badge")
                .metadata("description", "Give admin right")
                .mint_initial_supply(1);

            // Created all necessary resources address badges for the operation of this blueprint
            let rad_insurance_badge_manager =
                RadInsuranceBadgeManager::instanciate_rad_insurance_badge_manager(
                    admin_badge.resource_address(),
                    None,
                    None,
                    None,
                    None,
                    None,
                );

            let mut resource_address_by_badge_type = HashMap::new();
            for badge_type in all::<BadgeType>().collect::<Vec<_>>() {
                let resource_address = rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(badge_type.clone());
                resource_address_by_badge_type.insert(badge_type as u16, resource_address);
            }

            //Management of access rights to methods
            let access_rules = AccessRules::new()
                .method(
                    "create_policy",
                    rule!(require(admin_badge.resource_address())),
                    LOCKED,
                )
                .method(
                    "make_claim_as_refused",
                    rule!(require(admin_badge.resource_address())),
                    MUTABLE(rule!(require(admin_badge.resource_address()))),
                )
                .method(
                    "make_claim_as_accepted",
                    rule!(require(admin_badge.resource_address())),
                    MUTABLE(rule!(require(admin_badge.resource_address()))),
                )
                .method(
                    "get_rewards",
                    rule!(require(
                        rad_insurance_badge_manager
                            .get_resource_address_by_badge_type(BadgeType::Insurer)
                    )),
                    LOCKED,
                )
                .method(
                    "rewards_withdrawal",
                    rule!(require(
                        rad_insurance_badge_manager
                            .get_resource_address_by_badge_type(BadgeType::Insurer)
                    )),
                    LOCKED,
                )
                .method("list_on_marketplace", AccessRule::AllowAll, LOCKED)
                .method("delist_on_marketplace", rule!(require(
                    rad_insurance_badge_manager
                        .get_resource_address_by_badge_type(BadgeType::InsurerMarketListing)
                )), LOCKED)
                .method("withdrawal_sale_amount", rule!(require(
                    rad_insurance_badge_manager
                        .get_resource_address_by_badge_type(BadgeType::InsurerMarketListing)
                )), LOCKED)
                .method(
                    "report_a_claim",
                    rule!(require(
                        rad_insurance_badge_manager
                            .get_resource_address_by_badge_type(BadgeType::Insured)
                    )),
                    LOCKED,
                )
                .method(
                    "claim_withdraw",
                    rule!(require(
                        rad_insurance_badge_manager
                            .get_resource_address_by_badge_type(BadgeType::InsuredClaim)
                    )),
                    LOCKED,
                )
                .method("get_policies", AccessRule::AllowAll, LOCKED)
                .method("buy_on_marketplace", AccessRule::AllowAll, LOCKED)
                .method("invest_as_insurer", AccessRule::AllowAll, LOCKED)
                .method("get_policy_info", AccessRule::AllowAll, LOCKED)
                .method(
                    "subscribe_to_insurance_policy",
                    AccessRule::AllowAll,
                    LOCKED,
                )
                .method(
                    "recalculate_total_insureds_cover_amount",
                    AccessRule::AllowAll,
                    MUTABLE(rule!(require(admin_badge.resource_address()))),
                );

            // Instantiate and globalize instanciate_rad_insurance component and return it with the admin badge to caller
            let mut component = Self {
                policies: HashMap::new(),
                archive_policies: HashMap::new(),
                services_fees: services_fees,
                resource_address: resource_address,
                admin_badge_resource_address: admin_badge.resource_address(),
                resource_address_by_badge_type: resource_address_by_badge_type,
                rad_insurance_badge_manager: rad_insurance_badge_manager,
            }
            .instantiate();

            component.add_access_check(access_rules);

            return (component.globalize(), admin_badge);
        }

        //
        //#Arguments
        //* `amount` The amount of badge
        //* `admin_badge_bucket` The admin badge bucket
        //#Return
        // Returns a bucket that contains amount
        pub fn mint_minter_badge(&mut self, amount: Decimal, admin_badge_bucket: Bucket) -> Bucket {
            assert!(
                admin_badge_bucket.amount() > Decimal::zero(),
                "admin badge amount must be > 0"
            );
            assert!(
                admin_badge_bucket.resource_address() == self.admin_badge_resource_address,
                "Invalid badge provided"
            );
            let admin_badge_bucket = self
                .rad_insurance_badge_manager
                .mint_minter_badge(admin_badge_bucket, amount);
            return admin_badge_bucket;
        }

        // Get all insurance policies
        //#Return
        // Returns a vector of policyId
        pub fn get_policies(&self) -> Vec<u128> {
            let mut result = Vec::new();
            for policy in self.policies.iter() {
                result.push(*policy.0);
            }
            return result;
        }

        // Allows the creation of an insurance policy
        //#Return
        // Returns the Id of the insurance policy
        pub fn create_policy(
            &mut self,
            //name of the insurance policy
            name: String,
            // description of the insurance policy
            description: String,
            //Annual Insurer Reward Rate
            insurer_reward_percent_rate: Decimal,
            //Initial liquidity needed to pay investors before insureds subscriptions can
            initial_liquidity: Bucket,
        ) -> u128 {
            assert!(
                !initial_liquidity.is_empty(),
                "initial liquidity must be > 0"
            );
            assert!(
                initial_liquidity.resource_address() == self.resource_address,
                "bad initial liquidity resource address"
            );
            assert!(
                insurer_reward_percent_rate > Decimal::zero()
                    && insurer_reward_percent_rate <= Decimal::from("100"),
                "Insurer reward percent rate must be beween 1 and 100"
            );

            // instanciate the insurance policy blueprint
            let policy_instance = PolicyComponent::instanciate_policy(
                name,
                description,
                insurer_reward_percent_rate,
                self.resource_address,
                self.services_fees,
                self.admin_badge_resource_address,
                self.rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::Insurer),
                self.rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::Insured),
                self.rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::InsuredClaim),
                self.rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::InsurerMarketListing),
                self.rad_insurance_badge_manager
                    .get_minter_badge(Decimal::one()),
                initial_liquidity,
            );

            let policy_component_address = policy_instance.1.globalize();

            // Store the insurance policy component and his id
            self.policies
                .insert(policy_instance.0, policy_component_address);

            //DONT REMOVE USE FOR SHELL TRANSACTION MANISFEST
            Logger::debug(format!("policy_id: {}", policy_instance.0));

            // return the insurance policy id to the caller
            return policy_instance.0;
        }

        // Allows to invest in an insurance policy
        //#Return
        // Returns a bucket that contains the insurer badge
        pub fn invest_as_insurer(
            &mut self,
            // id of the insurance policy
            policy_id: u128,
            // bucket that contain invest amount
            invest_amount: Bucket,
            // bucket that contain services fees
            service_fee: Bucket,
        ) -> (Bucket, NonFungibleLocalId) {
            // get the policy insurance componentaddress
            let policy_component = self.get_policy_by_id(policy_id);
            // details in
            return borrow_component!(*policy_component).call::<(Bucket, NonFungibleLocalId)>(
                "invest_as_insurer",
                args!(invest_amount, service_fee),
            );
        }

        // Allows to get list of rewards
        //#Return
        // Returns a map with rewards
        pub fn get_rewards(
            &self,
            // Id of the insurance policy
            policy_id: u128,
            // Insurer badge proof
            insurer_badge_proof: Proof,
        ) -> HashMap<NonFungibleLocalId, (Decimal, i64)> {
            insurer_badge_proof
                .validate_resource_address(
                    self.rad_insurance_badge_manager
                        .get_resource_address_by_badge_type(BadgeType::Insurer),
                )
                .expect("Invalid insurer badge provided");
            assert!(
                insurer_badge_proof.amount() > Decimal::zero(),
                "insurer badge proof amount must be > 0"
            );
            let policy_component = self.get_policy_by_id(policy_id);
            return borrow_component!(*policy_component)
                .call::<HashMap<NonFungibleLocalId, (Decimal, i64)>>(
                    "get_rewards",
                    args!(insurer_badge_proof.non_fungible_local_ids()),
                );
            // return policy_component.get_rewards(insurer_badge_proof.non_fungible_local_ids());
        }


        // Allows to get the insurance policy informations
        //* `policy_id` The Id of insurance policy
        //#Return
        // Returns the insurance policy informations
        pub fn get_policy_info(&self, policy_id: u128) -> PolicyInfo {
            let policy_component = self.get_policy_by_id(policy_id);
            return borrow_component!(*policy_component)
                .call::<PolicyInfo>("get_policy_info", args!());
            // return policy_component.get_policy_info();
        }


        // Allows to withdraw rewards
        //* `policy_id` The Id of insurance policy
        //* `insurer_badge_proof` The insurer bdage proof
        //#Return
        // Returns a bucket that contains rewards
        pub fn rewards_withdrawal(
            &mut self,
            policy_id: u128,
            insurer_badge_proof: Proof,
        ) -> Bucket {
            insurer_badge_proof
                .validate_resource_address(
                    self.rad_insurance_badge_manager
                        .get_resource_address_by_badge_type(BadgeType::Insurer),
                )
                .expect("Invalid insurer badge provided");
            assert!(
                insurer_badge_proof.amount() > Decimal::zero(),
                " insurer badge proof amount must be > 0"
            );
            let policy_component = self.get_policy_by_id(policy_id);
            return borrow_component!(*policy_component).call::<Bucket>(
                "rewards_withdrawal",
                args!(insurer_badge_proof.non_fungible_local_ids()),
            );
        }

        // Allows to withdraw rewards
        //* `policy_id` The Id of insurance policy
        //* `cover_amount` The amount to be insured
        //* `deposit` The bucket that contains the deposit amount
        //* `service_fee` The bucket that contains services fees
        //#Return
        // Returns a bucket that contains insured badge and the end of coverage date
        pub fn subscribe_to_insurance_policy(
            &mut self,
            policy_id: u128,
            cover_amount: Decimal,
            deposit: Bucket,
            service_fee: Bucket,
        ) -> (Bucket, i64) {
            let policy_component = self.get_policy_by_id(policy_id);
            return borrow_component!(*policy_component).call::<(Bucket, i64)>(
                "subscribe_to_insurance_policy",
                args!(cover_amount, deposit, service_fee),
            );
        }

        // Allows to report a claim
        //* `policy_id` The Id of insurance policy
        //* `insured_badge_proof` The insured badge proof
        //* `claim_report` The claim report description
        //* `claim_amount` The claim amount
        //* `claim_date` The claim report date
        //#Return
        // Returns a bucket that contains a claim badge
        pub fn report_a_claim(
            &mut self,
            insured_badge_proof: Proof,
            policy_id: u128,
            claim_report: String,
            claim_amount: Decimal,
            claim_date: i64,
        ) -> Bucket {
            let policy_component = self.get_policy_by_id(policy_id);

            insured_badge_proof
                .validate_resource_address(
                    self.rad_insurance_badge_manager
                        .get_resource_address_by_badge_type(BadgeType::Insured),
                )
                .expect("Invalid insured badge provided");

            insured_badge_proof
                .validate_contains_amount(Decimal::one())
                .expect("insured badge proof amount must be = 1");

            let insured_badge_id = match insured_badge_proof.non_fungible_local_ids().first() {
                Some(badge_id) => badge_id.clone(),
                None => panic!(""),
            };

            return borrow_component!(*policy_component).call::<Bucket>(
                "report_a_claim",
                args!(insured_badge_id, claim_report, claim_amount, claim_date),
            );
        }

        // Allows to accept a claim
        //* `policy_id` The Id of insurance policy
        //* `claim_badge_id` The claim badge id
        pub fn make_claim_as_accepted(
            &mut self,
            policy_id: u128,
            claim_badge_id: NonFungibleLocalId,
        ) {
            let policy_component = self.get_policy_by_id(policy_id);
            borrow_component!(*policy_component)
                .call::<()>("make_claim_as_accepted", args!(claim_badge_id));
        }


        // Allows to refuse a claim
        //* `policy_id` The Id of insurance policy
        //* `claim_badge_id` The claim badge id
        pub fn make_claim_as_refused(
            &mut self,
            policy_id: u128,
            claim_badge_id: NonFungibleLocalId,
        ) {
            let policy_component = self.get_policy_by_id(policy_id);
            borrow_component!(*policy_component)
                .call::<()>("make_claim_as_refused", args!(claim_badge_id));
        }

        // Allows to claim a withdraw
        //* `policy_id` The Id of insurance policy
        //* `claim_badge_proof` The claim badge proof
        //#Return
        // Returns a bucket that contains a claim badge
        pub fn claim_withdraw(&mut self, policy_id: u128, claim_badge_proof: Proof) -> Bucket {
            let policy_component = self.get_policy_by_id(policy_id);
            let claim_badge_id = match claim_badge_proof.non_fungible_local_ids().first() {
                Some(badge_id) => badge_id.clone(),
                None => panic!(""),
            };
            return borrow_component!(*policy_component)
                .call::<Bucket>("claim_withdraw", args!(claim_badge_id));
        }

        // Allows to compute total amount of insureds cover
        //* `policy_id` The Id of insurance policy
        pub fn recalculate_total_insureds_cover_amount(&mut self, policy_id: u128) {
            let policy_component = self.get_policy_by_id(policy_id);
            borrow_component!(*policy_component)
                .call::<()>("recalculate_total_insureds_cover_amount", args!());
        }

       
        // Allows to list on the marketplace
        //* `policy_id` The Id of insurance policy
        //* `insurer_bucket_to_list` The insurer bucket to list
        //* `service_fee` The bucket that contains services fees
        //* `listing_amount` The listing amount
        //#Return
        // Returns a bucket that contains a listing badge
        pub fn list_on_marketplace(
            &self,
            policy_id: u128,
            insurer_bucket_to_list: Bucket,
            service_fee: Bucket,
            listing_amount: Decimal,
        ) -> (Bucket, Bucket) {
            assert!(
                self.rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::Insurer)
                    == insurer_bucket_to_list.resource_address(),
                "Invalid Bucket Resource Address provided"
            );
            let policy_component = self.get_policy_by_id(policy_id);
            return borrow_component!(*policy_component).call::<(Bucket, Bucket)>(
                "list_on_marketplace",
                args!(insurer_bucket_to_list, service_fee, listing_amount),
            );
        }

        // Allows to delist on the marketplace
        //* `policy_id` The Id of insurance policy
        //* `service_fee` The bucket that contains services fees
        //* `to_delist_proof` The proof
        //#Return
        // Returns a bucket that contains a delisting badge
        pub fn delist_on_marketplace(&self,  
                                    policy_id: u128,
                                     service_fee: Bucket,
                                     to_delist_proof: Proof) -> (Bucket, Bucket) {

            to_delist_proof.validate_resource_address(self.rad_insurance_badge_manager.get_resource_address_by_badge_type(BadgeType::InsurerMarketListing))
            .expect("Invalid resource address badge provided"); 
            
            let to_delist_id = match to_delist_proof.non_fungible_local_ids().first() {
                Some(badge_id) => badge_id.clone(),
                None => panic!(""),
            };

            let policy_component = self.get_policy_by_id(policy_id);                               
            return borrow_component!(*policy_component).call::<(Bucket, Bucket)>(
                "delist_on_marketplace",
                args!(service_fee, to_delist_id),
            );
        }

        // Allows to buy on the marketplace
        //* `policy_id` The Id of insurance policy
        //* `payment_amount` The payment amount
        //* `service_fee` The bucket that contains services fees
        //* `market_listing_id` The market listing id
        //#Return
        // Returns a bucket that contains a insurer market listing badge
        pub fn buy_on_marketplace(&mut self, 
            policy_id: u128,
            payment_amount : Bucket, 
            service_fee: Bucket,
            market_listing_id : NonFungibleLocalId) -> (Bucket,Bucket,Bucket){
            let policy_component = self.get_policy_by_id(policy_id); 
            return borrow_component!(*policy_component).call::<(Bucket, Bucket, Bucket)>(
                "buy_on_marketplace",
                args!(payment_amount, service_fee, market_listing_id),
            );

        }


        // Allows to withdraw sale amount
        //* `policy_id` The Id of insurance policy
        //* `market_listing_proof` The market listing proof
        //#Return
        // Returns a purchase bucket
        pub fn withdrawal_sale_amount(&self, 
                                            policy_id : u128, 
                                            market_listing_proof : Proof) -> Bucket {

            let policy_component = self.get_policy_by_id(policy_id);
            let market_listing_id = match market_listing_proof.non_fungible_local_ids().first() {
                Some(badge_id) => badge_id.clone(),
                None => panic!(""),
            };

            return borrow_component!(*policy_component)
                .call::<Bucket>("withdrawal_sale_amount", args!(market_listing_id));
        }

        // Allows to retrieve insurance policy component address
        //* `policy_id` The Id of insurance policy
        //#Return
        // Returns the insurance policy component address
        fn get_policy_by_id(&self, policy_id: u128) -> &ComponentAddress {
            match self.policies.get(&policy_id) {
                Some(policy_component) => {
                    return policy_component;
                }
                None => panic!("policy not found"),
            }
        }
    }
}
