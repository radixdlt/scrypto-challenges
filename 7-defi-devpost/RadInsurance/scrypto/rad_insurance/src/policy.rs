use crate::badge_data::*;
use crate::constants::*;
use crate::rad_insurance_badge_manager::{BadgeType, RadInsuranceBadgeManager};
use scrypto::prelude::*;

#[blueprint]
mod PolicyModule {

    pub struct Policy {
        id: u128,
        //name of the policy
        name: String,
        //description of the policy
        description: String,
        // resource address
        resource_address: ResourceAddress,
        // insured contribution percent rate
        insured_contribution_percent_rate: Decimal,
        // insurer reward percent rate
        insurer_reward_percent_rate: Decimal,
        // insured contribution vault
        insured_contribution_vault: Vault,
        // service fees
        service_fee: Decimal,
        // service fee vault
        service_vault: Vault,
        // radInsurance badge manager
        rad_insurance_badge_manager: RadInsuranceBadgeManager,
        // admin badge resource address
        admin_badge_resource_address: ResourceAddress,
        // insurers
        insurers: HashMap<NonFungibleLocalId, Vault>,
        // insureds
        insureds: HashMap<NonFungibleLocalId, Decimal>,
        // initial liquidity vault
        initial_liquidity: Vault,
        // total insurers amount
        total_insurers_amount: Decimal,
        // total insureds cover amount
        total_insureds_cover_amount: Decimal,
        // claims
        claims: HashSet<NonFungibleLocalId>,
        // accepted claims vaults
        claims_accepted_vaults: HashMap<NonFungibleLocalId, Vault>,
        // marketplace vaults
        marketplace_vaults: HashMap<NonFungibleLocalId, Vault>,
        // purchases vaults
        purchases_vaults : HashMap<NonFungibleLocalId, Vault>,
    }

    impl Policy {
          // This function instanciate the policy
        pub fn instanciate_policy(
            name: String,
            descrption: String,
            insurer_reward_percent_rate: Decimal,
            resource_address: ResourceAddress,
            service_fee: Decimal,
            admin_badge_resource_address: ResourceAddress,
            insurer_badge_resource_address: ResourceAddress,
            insured_badge_resource_address: ResourceAddress,
            insured_claim_badge_resource_address: ResourceAddress,
            insurer_market_list_resource_address: ResourceAddress,
            minter_badge: Bucket,
            initial_liquidity: Bucket,
        ) -> (u128, PolicyComponent) {
            let rad_insurance_badge_manager =
                RadInsuranceBadgeManager::instanciate_rad_insurance_badge_manager(
                    admin_badge_resource_address,
                    Some(insurer_badge_resource_address),
                    Some(insured_badge_resource_address),
                    Some(insured_claim_badge_resource_address),
                    Some(insurer_market_list_resource_address),
                    Some(minter_badge),
                );

            let id = Runtime::generate_uuid();
            // the policy component
            let component = Self {
                id: id,
                name: name,
                description: descrption,
                insurer_reward_percent_rate: insurer_reward_percent_rate,
                insured_contribution_percent_rate: insurer_reward_percent_rate,
                insured_contribution_vault: Vault::new(resource_address),
                insurers: HashMap::new(),
                service_vault: Vault::new(resource_address),
                admin_badge_resource_address: admin_badge_resource_address,
                service_fee: service_fee,
                resource_address: resource_address,
                initial_liquidity: Vault::with_bucket(initial_liquidity),
                rad_insurance_badge_manager: rad_insurance_badge_manager,
                insureds: HashMap::new(),
                total_insureds_cover_amount: Decimal::zero(),
                total_insurers_amount: Decimal::zero(),
                claims: HashSet::new(),
                claims_accepted_vaults: HashMap::new(),
                marketplace_vaults: HashMap::new(),
                purchases_vaults : HashMap::new()
            }
            .instantiate();

            return (id, component);
        }

        // Allows to invest in an insurance policy
        //#Return
        // Returns a bucket that contains the insurer badge
        pub fn invest_as_insurer(
            &mut self,
            // bucket that contain invest amount
            invest_amount: Bucket,
            // bucket that contain services fees
            service_fee: Bucket,
        ) -> (Bucket, NonFungibleLocalId) {
            assert!(
                invest_amount.resource_address() == self.resource_address,
                "invalid policy resource address"
            );
            assert!(
                invest_amount.amount() > Decimal::zero(),
                "invest amout must be > 0"
            );
            assert!(
                service_fee.amount() == self.service_fee,
                "fee must be = {}",
                self.service_fee
            );
            assert!(service_fee.resource_address() == self.resource_address , "Invalid fee resource address") ; 

            // increment total insurers investissement amount
            self.increment_total_insurers_amount(invest_amount.amount());

            let now = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let insurer_nft_data = InsurerBadgeData {
                amount: invest_amount.amount(),
                last_reward_reclaim_date: now,
                date: now,
                reward_percent_rate: self.insurer_reward_percent_rate,
                policy_id: self.id,
            };

            // creating insurer badge
            let insurer_badge = self
                .rad_insurance_badge_manager
                .mint_new_non_fungible_badge(BadgeType::Insurer, insurer_nft_data);

            let insurer_badge_id = insurer_badge.non_fungible_local_id();

            // storing created badge id with the invest bucket
            self.insurers
                .insert(insurer_badge_id.clone(), Vault::with_bucket(invest_amount));

            // take fees for the team
            self.service_vault.put(service_fee);

            // return insurer_badge and his id to caller
            return (insurer_badge, insurer_badge_id);
        }

        // Allows to get list of rewards
        //#Return
        // Returns a map with rewards
        pub fn get_rewards(
            &self,
            ids: BTreeSet<NonFungibleLocalId>,
        ) -> HashMap<NonFungibleLocalId, (Decimal, i64)> {
            let mut rewards: HashMap<NonFungibleLocalId, (Decimal, i64)> = HashMap::new();
            for id in ids {
                rewards.insert(id.clone(), self.get_reward_by_badge_id(id));
            }
            return rewards;
        }

        // Allows to withdraw rewards
        //#Return
        // Returns a bucket that contains rewards
        pub fn rewards_withdrawal(
            &mut self,
            // insurer_badge_proof
            badge_ids: BTreeSet<NonFungibleLocalId>,
        ) -> Bucket {
            for badge_id in badge_ids.clone() {
                let insurer_vault = self.insurers.get(&badge_id);
                match insurer_vault {
                    Some(_) => {}
                    None => {
                        panic!("Error while validating policy");
                    }
                }
            }

            let rewards_amount = self.get_rewards(badge_ids);
            let mut rewards: Bucket = Bucket::new(self.resource_address);
            for reward in rewards_amount {
                let amount = reward.1 .0;
                let now = reward.1 .1;
                let badge_id = reward.0;
                if amount > Decimal::zero() {
                    let mut data: InsurerBadgeData = borrow_resource_manager!(self
                        .rad_insurance_badge_manager
                        .get_resource_address_by_badge_type(BadgeType::Insurer))
                    .get_non_fungible_data(&badge_id.clone());

                    if self.insured_contribution_vault.amount() >= amount {
                        rewards.put(self.insured_contribution_vault.take(amount));
                    } else {
                        // assume that initial_liquidity always contains reward amount when there are no insurers.
                        rewards.put(self.initial_liquidity.take(amount));
                    }
                    // update reclaim date
                    data.last_reward_reclaim_date = now;
                    self.rad_insurance_badge_manager.update_non_fungible_data(
                        BadgeType::Insurer,
                        data,
                        &badge_id,
                    );
                }
            }

            assert!(
                rewards.amount() > Decimal::zero(),
                "There are no rewards to claim"
            );

            return rewards;
        }

        // Allows to withdraw rewards
        //* `cover_amount` The amount to be insured
        //* `deposit` The bucket that contains the deposit amount
        //* `service_fee` The bucket that contains services fees
        //#Return
        // Returns a bucket that contains insured badge and the end of coverage date
        pub fn subscribe_to_insurance_policy(
            &mut self,
            //
            cover_amount: Decimal,
            //
            deposit: Bucket,
            // bucket that contain services fees
            service_fee: Bucket,
        ) -> (Bucket, i64) {
            assert!(
                deposit.resource_address() == self.resource_address,
                "Invalid policy resource address provided"
            );
            assert!(
                deposit.amount() > Decimal::zero(),
                "deposit amount must be > 0"
            );
            assert!(
                self.can_cover_amount(cover_amount),
                "the maximum cover amount is {}",
                self.get_max_cover_amount()
            );
            assert!(
                service_fee.amount() == self.service_fee,
                "fee must be = {}",
                self.service_fee
            );

            // get the end date cover
            let coverage_end_date = self.get_cover_end_date_from_deposit_amount(
                cover_amount,
                deposit.amount(),
                self.insured_contribution_percent_rate,
            );
            
            // creating the insuredBadge
            let data = InsuredBadgeData {
                cover_amount: cover_amount,
                coverage_end_date: coverage_end_date,
                contribution_percent_rate: self.insured_contribution_percent_rate,
                current_claim_report: None,
                accepted_claims: Vec::new(),
                declined_claims: Vec::new(),
                policy_id: self.id,
            };

            let insured_badge = self
                .rad_insurance_badge_manager
                .mint_new_non_fungible_badge(BadgeType::Insured, data);

            let insured_badge_id = insured_badge.non_fungible_local_id();
            // stored insured badge id with cover amount
            self.insureds.insert(insured_badge_id, cover_amount);

            // increment insured cover amount
            self.increment_total_insureds_cover_amount(cover_amount);

            // put de deposit amount in vault
            self.insured_contribution_vault.put(deposit);

            // taking service fees
            self.service_vault.put(service_fee);

            return (insured_badge, coverage_end_date);
        }

        // Allows to find insurers badge Identifiers
        //* `badges_ids` badge identifier
        //#Return
        // Returns a found badge Ids
        pub fn find_assurers_badge_ids(
            &self,
            badges_ids: Vec<NonFungibleLocalId>,
        ) -> Vec<NonFungibleLocalId> {
            let mut result: Vec<NonFungibleLocalId> = Vec::new();

            for badge_id in badges_ids {
                match self.insurers.get(&badge_id.clone()) {
                    Some(_) => result.push(badge_id),
                    None => {}
                }
            }
            return result;
        }

        // Allows to compute total amount of insureds cover
        //* `policy_id` The Id of insurance policy
        pub fn recalculate_total_insureds_cover_amount(&mut self) {
            let mut result = Decimal::zero();

            for mut insured in self.insureds.iter() {
                let data: InsuredBadgeData = borrow_resource_manager!(self
                    .rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::Insured))
                .get_non_fungible_data(insured.0);

                let now = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
                let is_cover_expired = data.coverage_end_date <= now;
                if is_cover_expired {
                    insured.1 = &Decimal::zero();
                } else {
                    result += data.cover_amount;
                }
            }

            self.total_insureds_cover_amount = result;
        }

        // Allows to report a claim (this method is call by Insured)
        //* `badge_id` The badge identifier
        //* `claim_report` The claim report description
        //* `claim_amount` The claim amount
        //* `claim_date` The claim report date
        //#Return
        // Returns a bucket that contains a claim badge
        pub fn report_a_claim(
            &mut self,
            badge_id: NonFungibleLocalId,
            claim_report: String,
            claim_amount: Decimal,
            claim_date: i64,
        ) -> Bucket {
            assert!(
                self.insureds.contains_key(&badge_id),
                "Insured not found in thie policy : {}",
                self.id
            );

            let mut insured_badge_data: InsuredBadgeData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::Insured))
            .get_non_fungible_data(&badge_id);

            assert!(
                insured_badge_data.current_claim_report == None,
                "A claim declaration already in progress"
            );

            let is_cover_expired = insured_badge_data.coverage_end_date < claim_date;
            assert!(
                !is_cover_expired,
                "Insurance coverage expired on date of loss"
            );

            let claim_data = InsuredClaimBadgeData {
                claim_amount: claim_amount,
                insured_badge_id: badge_id.to_owned(),
                claim_report: claim_report,
                state: ClaimState::Declared,
                policy_id: self.id,
            };

            // minting the claim badge
            let claim_badge = self
                .rad_insurance_badge_manager
                .mint_new_non_fungible_badge(BadgeType::InsuredClaim, claim_data);

            // stored the claim badge
            self.claims.insert(claim_badge.non_fungible_local_id());

            //set the claim badge id on insured_badge_data
            insured_badge_data.current_claim_report = Some(claim_badge.non_fungible_local_id());

            // updating insured_badge_data
            self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::Insured,
                insured_badge_data,
                &badge_id,
            );

            Logger::debug(format!(
                "claim_badge_id: {}",
                claim_badge.non_fungible_local_id()
            ));
            // return claim badge to caller
            return claim_badge;
        }

        // Allows to refuse a claim  (this method is call by admin)
        //* `claim_badge_id` The claim badge id
        pub fn make_claim_as_refused(&mut self, claim_badge_id: NonFungibleLocalId) {
            assert!(
                self.claims.contains(&claim_badge_id),
                "claim not found in the policy {}",
                self.name
            );

            let mut claim_badge: InsuredClaimBadgeData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::InsuredClaim))
            .get_non_fungible_data(&claim_badge_id);

            assert!(
                claim_badge.state == ClaimState::UnderInvestigation
                    || claim_badge.state == ClaimState::Declared,
                "This claim has been processed"
            );

            let mut insured_badge: InsuredBadgeData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::Insured))
            .get_non_fungible_data(&claim_badge.insured_badge_id);

            insured_badge.current_claim_report = None;
            self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::Insured,
                insured_badge,
                &claim_badge.insured_badge_id,
            );

            claim_badge.state = ClaimState::Refused;
            self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::InsuredClaim,
                claim_badge,
                &claim_badge_id,
            );
        }

        // Allows to accept a claim
        //* `claim_badge_id` The claim badge id
        pub fn make_claim_as_accepted(&mut self, claim_badge_id: NonFungibleLocalId) {
            assert!(
                self.claims.contains(&claim_badge_id),
                "claim not found in the policy {}",
                self.name
            );

            let mut claim_badge: InsuredClaimBadgeData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::InsuredClaim))
            .get_non_fungible_data(&claim_badge_id);

            assert!(
                claim_badge.state == ClaimState::UnderInvestigation
                    || claim_badge.state == ClaimState::Declared,
                "This claim has been processed"
            );

            let mut claim_amount_accepted_bucket = Bucket::new(self.resource_address);
            let percent_to_deduct =
                claim_badge.claim_amount * Decimal::from(100) / self.total_insurers_amount;
            let mut new_total_insurers_amount = Decimal::zero();

            for vault in self.insurers.values_mut() {
                let amount_to_deduct = vault.amount() * percent_to_deduct / Decimal::from(100);
                claim_amount_accepted_bucket.put(vault.take(amount_to_deduct));
                new_total_insurers_amount += vault.amount();
            }

            self.total_insurers_amount = new_total_insurers_amount;

            let mut insured_badge: InsuredBadgeData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::Insured))
            .get_non_fungible_data(&claim_badge.insured_badge_id);

            insured_badge.current_claim_report = None;
            insured_badge
                .accepted_claims
                .push(claim_badge.insured_badge_id.clone());
            self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::Insured,
                insured_badge,
                &claim_badge.insured_badge_id,
            );

            claim_badge.state = ClaimState::Accepted;
            self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::InsuredClaim,
                claim_badge,
                &claim_badge_id,
            );

            self.claims_accepted_vaults.insert(
                claim_badge_id,
                Vault::with_bucket(claim_amount_accepted_bucket),
            );
        }

        // Allows to claim a withdraw (this method is call by insured with claim badge)
        //* `claim_badge_proof` The claim badge proof
        //#Return
        // Returns a bucket that contains a claim badge
        pub fn claim_withdraw(&mut self, claim_badge_id: NonFungibleLocalId) -> Bucket {
            assert!(
                self.claims.contains(&claim_badge_id),
                "claim not found in the policy {}",
                self.name
            );

            assert!(
                self.claims_accepted_vaults.contains_key(&claim_badge_id),
                "Unauthorized withdrawal"
            );

            let mut claim_badge: InsuredClaimBadgeData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::InsuredClaim))
            .get_non_fungible_data(&claim_badge_id);

            assert!(
                claim_badge.state == ClaimState::Accepted,
                "Unauthorized withdrawal"
            );
            claim_badge.state = ClaimState::Collected;
            self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::InsuredClaim,
                claim_badge,
                &claim_badge_id,
            );

            return match self.claims_accepted_vaults.get_mut(&claim_badge_id) {
                Some(v) => v.take_all(),
                None => panic!("Unauthorized withdrawal"),
            };
        }

        // Get insurance policy informations
        //#Return
        // Returns the policy informations
        pub fn get_policy_info(&self) -> PolicyInfo {
            PolicyInfo {
                id: self.id,
                name: self.name.clone(),
                description: self.description.clone(),
                insured_contribution_percent_rate: self.insured_contribution_percent_rate,
                insurer_reward_percent_rate: self.insurer_reward_percent_rate,
                insured_badge_resource_address: self
                    .rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::Insured),
                insurer_badge_resource_address: self
                    .rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::Insurer),
                service_fee: self.service_fee,
                max_cover_amount: self.get_max_cover_amount(),
            }
        }

        // Allows to list on the marketplace
        //* `insurer_bucket_to_list` The insurer bucket to list
        //* `service_fee` The bucket that contains services fees
        //* `listing_amount` The listing amount
        //#Return
        // Returns a bucket that contains a listing badge
        pub fn list_on_marketplace(
            &mut self,
            insurer_bucket_to_list: Bucket,
            mut service_fee: Bucket,
            listing_amount: Decimal,
        ) -> (Bucket, Bucket) {
            assert!(
                self.rad_insurance_badge_manager
                    .get_resource_address_by_badge_type(BadgeType::Insurer)
                    == insurer_bucket_to_list.resource_address(),
                "Invalid Reousource Address provided"
            );

            assert!(service_fee.resource_address() == self.resource_address , "Invalid fee resource address") ; 

            assert!(
                insurer_bucket_to_list.amount() == Decimal::one(),
                "The amount must be 1"
            );

            assert!(service_fee.resource_address() == self.resource_address , "Invalid fee resource address") ; 
            assert!(
                service_fee.amount() >= self.service_fee,
                "The service fee must : {}",
                self.service_fee
            );

            let insurer_badge_id = insurer_bucket_to_list.non_fungible_local_id();
            let insurerMarketListingData = InsurerMarketListingData {
                insurer_badge_id: insurer_badge_id.clone(),
                listing_amount: listing_amount,
                listing_state: ListingState::Listed,
                policy_id: self.id,
            };

            let market_listing_badge = self
                .rad_insurance_badge_manager
                .mint_new_non_fungible_badge(
                    BadgeType::InsurerMarketListing,
                    insurerMarketListingData,
                );

            self.marketplace_vaults.insert(
                market_listing_badge.non_fungible_local_id(),
                Vault::with_bucket(insurer_bucket_to_list),
            );

            self.service_vault.put(service_fee.take(self.service_fee));

            Logger::debug(format!(
                "listing_badge_id: {}",
                market_listing_badge.non_fungible_local_id()
            ));

            return (market_listing_badge, service_fee);
        }

        // Allows to delist on the marketplace
        //* `service_fee` The bucket that contains services fees
        //* `to_delist_id` the listing id to delist
        //#Return
        // Returns a bucket that contains a delisting badge
        pub fn delist_on_marketplace(
            &mut self,
            mut service_fee: Bucket,
            to_delist_id: NonFungibleLocalId,
        ) -> (Bucket, Bucket) {
            assert!(
                service_fee.amount() >= self.service_fee,
                "The service fee must : {}",
                self.service_fee
            );

            assert!(service_fee.resource_address() == self.resource_address , "Invalid fee resource address") ; 

            assert!(
                self.marketplace_vaults.contains_key(&to_delist_id),
                "listing not found in the policy {}",
                self.name
            );

            let mut market_listing_data: InsurerMarketListingData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::InsurerMarketListing))
            .get_non_fungible_data(&to_delist_id);

            assert!(
                market_listing_data.listing_state == ListingState::Listed,
                "The listing cannot be delist"
            );

            market_listing_data.listing_state = ListingState::Delisted;
            self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::InsurerMarketListing,
                market_listing_data,
                &to_delist_id,
            );

            let insurer_badge_bucket = self
                .marketplace_vaults
                .get_mut(&to_delist_id)
                .unwrap()
                .take_all();

            self.service_vault.put(service_fee.take(self.service_fee));

            return (insurer_badge_bucket, service_fee);
        }


        // Allows to buy on the marketplace
        //* `payment_amount` The payment amount
        //* `service_fee` The bucket that contains services fees
        //* `market_listing_id` The market listing id
        //#Return
        // Returns a bucket that contains a insurer market listing badge
        pub fn buy_on_marketplace(&mut self, 
                                    mut payment_amount : Bucket, 
                                    mut service_fee: Bucket,
                                    market_listing_id : NonFungibleLocalId) -> (Bucket, Bucket, Bucket)
        {

            assert!(
                self.marketplace_vaults.contains_key(&market_listing_id),
                "listing not found in the policy {} {}",
                self.name,
                self.id
            );

            let mut market_listing_data: InsurerMarketListingData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::InsurerMarketListing))
            .get_non_fungible_data(&market_listing_id);

             assert!(market_listing_data.listing_state == ListingState::Listed); 
             assert!(payment_amount.amount() >= market_listing_data.listing_amount, "insufficient payment amount"); 
             
             market_listing_data.listing_state = ListingState::AcceptedAnOffer; 
             

             let insurer_badge_bucket = self.marketplace_vaults.get_mut(&market_listing_id).unwrap().take_all(); 
            
             self.purchases_vaults.insert(market_listing_id.clone(),
                                            Vault::with_bucket(payment_amount.take(market_listing_data.listing_amount))); 
             self.service_vault.put(service_fee.take(self.service_fee));

             self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::InsurerMarketListing,
                market_listing_data,
                &market_listing_id,
            );
             

            return (insurer_badge_bucket,service_fee,payment_amount); 
             
        }
        
        // Allows to withdraw sale amount
        //* `market_listing_id` The market listing id
        //#Return
        // Returns a purchase bucket
        pub fn withdrawal_sale_amount(&mut self, 
                                    market_listing_id : NonFungibleLocalId) -> Bucket {
            assert!(
                self.marketplace_vaults.contains_key(&market_listing_id),
                "listing not found in the policy {} {}",
                self.name,
                self.id
            );

            assert!(
                self.purchases_vaults.contains_key(&market_listing_id),
                "this listing have not been sale "
            );

            let mut market_listing_data: InsurerMarketListingData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::InsurerMarketListing))
            .get_non_fungible_data(&market_listing_id);

            assert!(market_listing_data.listing_state == ListingState::AcceptedAnOffer, "this listing have not been sale"); 
            market_listing_data.listing_state = ListingState::AmountHasBeenCollected; 

            self.rad_insurance_badge_manager.update_non_fungible_data(
                BadgeType::InsurerMarketListing,
                market_listing_data,
                &market_listing_id,
            );
            
            let  purchase_bucket = self
                                    .purchases_vaults
                                    .get_mut(&market_listing_id).unwrap().take_all(); 
            
            return purchase_bucket; 

        }

        // Allows to get rewards amount
        //* `invest_amount` The invest amount
        //* `reward_percent_rate` The reward percent rate
        //* `periode_in_secondes` The coverage period in seconds
        //#Return
        // Returns the amount of the rewards
        fn get_rewards_amount(
            &self,
            invest_amount: Decimal,
            reward_percent_rate: Decimal,
            periode_in_secondes: i64,
        ) -> Decimal {
            let annual_reward_amount = invest_amount * reward_percent_rate / Decimal::from(100);
            return Decimal::from(periode_in_secondes) * annual_reward_amount
                / NUMBER_OF_SECONDS_PER_YEAR;
        }

        // Checks whether the amount can be covered
        //* `amount` The amount to be covered
        //#Return
        // Returns `true` if the amount can be covered, `false' otherwise
        fn can_cover_amount(&self, amount: Decimal) -> bool {
            return self.get_max_cover_amount() >= amount;
        }

        // Allows to get the max cover amount
        //#Return
        // Returns the max cover amount
        fn get_max_cover_amount(&self) -> Decimal {
            return self.total_insurers_amount - self.total_insureds_cover_amount;
        }

        // Allows to increment total insurers amount
        //* `amount` The amount to be added
        fn increment_total_insurers_amount(&mut self, amount: Decimal) {
            self.total_insurers_amount += amount;
        }

        // Allows to increment total insurers cover amount
        //* `cover_amount` The amount to be added
        fn increment_total_insureds_cover_amount(&mut self, cover_amount: Decimal) {
            self.total_insureds_cover_amount += cover_amount;
        }

        // Allows to get end date of coverage from deposit amount
        //* `cover_amount` The cover amount 
        //* `deposit_amount` The deposit amount 
        //#Return
        // Returns end date of coverage in seconds
        fn get_cover_end_date_from_deposit_amount(
            &self,
            cover_amount: Decimal,
            deposit_amount: Decimal,
            contribution_percent_rate: Decimal,
        ) -> i64 {
            let result = deposit_amount * NUMBER_OF_SECONDS_PER_YEAR
                / (cover_amount * contribution_percent_rate / 100);
            let now = Clock::current_time(TimePrecision::Minute);
            Logger::debug(format!(
                "get_cover_end_date_from_deposit_amount result : {}",
                result
            ));
            let result_str = format!("{}", result);
            let cover_date = now.add_seconds(i64::from_str(result_str.as_str()).unwrap());
            return cover_date.unwrap().seconds_since_unix_epoch;
        }

        // Allows to get reward by badge id
        //* `badge_id` The cover amount 
        //#Return
        // Returns the reward amount associate to date of claim (now)
        fn get_reward_by_badge_id(&self, badge_id: NonFungibleLocalId) -> (Decimal, i64) {
            let now = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            let data: InsurerBadgeData = borrow_resource_manager!(self
                .rad_insurance_badge_manager
                .get_resource_address_by_badge_type(BadgeType::Insurer))
            .get_non_fungible_data(&badge_id);
            let periode_in_secondes = now - data.last_reward_reclaim_date;
            return (
                self.get_rewards_amount(data.amount, data.reward_percent_rate, periode_in_secondes),
                now,
            );
        }
    }
}

#[derive(
    ScryptoCategorize,
    ScryptoEncode,
    ScryptoDecode,
    LegacyDescribe,
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
)]
pub struct PolicyInfo {
    //
    id: u128,
    name: String,
    //description of the policy
    description: String,
    //
    service_fee: Decimal,
    //
    insured_contribution_percent_rate: Decimal,
    //
    insurer_reward_percent_rate: Decimal,
    //
    insurer_badge_resource_address: ResourceAddress,
    //
    insured_badge_resource_address: ResourceAddress,
    //
    max_cover_amount: Decimal,
}
