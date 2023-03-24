use crate::badge_data::*;
use enum_iterator::Sequence;
use scrypto::prelude::*;
use std::any::type_name;
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe)]
pub struct RadInsuranceBadgeManager {
    minter_badge_vault: Vault,
    admin_badge_vault: Vault,
    resource_address_by_badge_type: KeyValueStore<BadgeType, ResourceAddress>,
    type_name_by_badge_type: KeyValueStore<BadgeType, String>,
}

impl RadInsuranceBadgeManager {
    // This function instanciate the rad insurance badge manager
    pub fn instanciate_rad_insurance_badge_manager(
        // admin badge resource address
        admin_badge_resource_address: ResourceAddress,
        // insurer badge resource address
        insurer_badge_resource_address: Option<ResourceAddress>,
        // insured badge resource address
        insured_badge_resource_address: Option<ResourceAddress>,
        // insured claim badge resource address
        insured_claim_badge_resource_address: Option<ResourceAddress>,
        // insurer market list resource address
        insurer_market_list_resource_address: Option<ResourceAddress>,
        // minter badge
        minter_badge: Option<Bucket>,
    ) -> RadInsuranceBadgeManager {
        let minter_badge = match minter_badge {
            Some(minter_badge) => {
                assert!(
                    minter_badge.amount() > Decimal::zero(),
                    "minter badge amount bust be > 0"
                );
                minter_badge
            }
            None => ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "minter badge")
                // .mintable(rule!(require(admin_badge_resource_address)), LOCKED)
                .mint_initial_supply(Decimal::from("100")),
        };

        let insurer_badge_resource_address = match insurer_badge_resource_address {
            Some(resource_address) => resource_address,
            None => ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Insurer badge")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(minter_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply(),
        };

        let insured_badge_resource_address = match insured_badge_resource_address {
            Some(resource_address) => resource_address,
            None => ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Insured badge")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(minter_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply(),
        };

        let insured_claim_badge_resource_address = match insured_claim_badge_resource_address {
            Some(resource_address) => resource_address,
            None => ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Insured claim badge")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(minter_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply(),
        };

        let insurer_market_list_resource_address = match insurer_market_list_resource_address {
            Some(resource_address) => resource_address,
            None => ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Insured market list")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(minter_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply(),
        };

        let resource_address_by_badge_type = KeyValueStore::new();
        resource_address_by_badge_type.insert(BadgeType::Insurer, insurer_badge_resource_address);
        resource_address_by_badge_type.insert(BadgeType::Insured, insured_badge_resource_address);
        resource_address_by_badge_type.insert(
            BadgeType::InsuredClaim,
            insured_claim_badge_resource_address,
        );
        resource_address_by_badge_type.insert(
            BadgeType::InsurerMarketListing,
            insurer_market_list_resource_address,
        );

        let type_name_by_badge_type = KeyValueStore::new();
        type_name_by_badge_type.insert(
            BadgeType::Insurer,
            type_name::<InsurerBadgeData>().to_owned(),
        );
        type_name_by_badge_type.insert(
            BadgeType::Insured,
            type_name::<InsuredBadgeData>().to_owned(),
        );
        type_name_by_badge_type.insert(
            BadgeType::InsuredClaim,
            type_name::<InsuredClaimBadgeData>().to_owned(),
        );
        type_name_by_badge_type.insert(
            BadgeType::InsurerMarketListing,
            type_name::<InsurerMarketListingData>().to_owned(),
        );

        return Self {
            minter_badge_vault: Vault::with_bucket(minter_badge),
            admin_badge_vault: Vault::new(admin_badge_resource_address),
            resource_address_by_badge_type: resource_address_by_badge_type,
            type_name_by_badge_type: type_name_by_badge_type,
        };
    }

    // Allows to get resource address by badge type
    //* `badge_type` The badge type
    //#Return
    // Returns the resource address 
    pub fn get_resource_address_by_badge_type(&self, badge_type: BadgeType) -> ResourceAddress {
        match self.resource_address_by_badge_type.get(&badge_type) {
            Some(resource_address) => *resource_address,
            None => {
                panic!("unsupported");
            }
        }
    }

    // Allows to mint a badge
    //* `admin_badge_bucket` Represent the admin badge bucket
    //* `amount` The amount
    //#Return
    // Returns a bucket
    pub fn mint_minter_badge(&mut self, admin_badge_bucket: Bucket, amount: Decimal) -> Bucket {
        assert!(
            admin_badge_bucket.resource_address() == self.admin_badge_vault.resource_address(),
            "Invalid badge provided"
        );
        self.admin_badge_vault.put(admin_badge_bucket);
        let minter_badge = self.admin_badge_vault.authorize(|| {
            borrow_resource_manager!(self.minter_badge_vault.resource_address()).mint(amount)
        });
        self.minter_badge_vault.put(minter_badge);
        return self.admin_badge_vault.take(Decimal::one());
    }

    // Allows to get minter badge
    //* `amount` The amount
    //#Return
    // Returns a bucket
    pub fn get_minter_badge(&mut self, amount: Decimal) -> Bucket {
        return self.minter_badge_vault.take(amount);
    }

    // Allows to mint a new non fungible badge
    //* `badge_to_mint` Represent the badge type
    //* `data` Represent a NonFungibleData
    //#Return
    // Returns a bucket
    pub fn mint_new_non_fungible_badge<T>(&self, badge_to_mint: BadgeType, data: T) -> Bucket
    where
        T: NonFungibleData,
    {
        Logger::debug(format!(
            "minter badge amount : {}",
            self.minter_badge_vault.amount()
        ));
        match self.resource_address_by_badge_type.get(&badge_to_mint) {
            Some(_)
                if self
                    .type_name_by_badge_type
                    .get(&badge_to_mint)
                    .unwrap()
                    .to_string()
                    != type_name::<T>().to_string() =>
            {
                panic!("Invalid data provided")
            }
            Some(resource_address) => self.minter_badge_vault.authorize(|| {
                borrow_resource_manager!(*resource_address).mint_uuid_non_fungible(data)
            }),
            None => panic!("unsupported badge type"),
        }
    }

    // Allows to update a new non fungible badge
    //* `badge_to_update` Represent the badge type to update
    //* `data` Represent a NonFungibleData
    //* `id` Represent a NonFungibleLocalId
    pub fn update_non_fungible_data<T>(
        &self,
        badge_to_update: BadgeType,
        data: T,
        id: &NonFungibleLocalId,
    ) where
        T: NonFungibleData,
    {
        match self.resource_address_by_badge_type.get(&badge_to_update) {
            Some(_)
                if self
                    .type_name_by_badge_type
                    .get(&badge_to_update)
                    .unwrap()
                    .to_string()
                    != type_name::<T>().to_string() =>
            {
                panic!("Invalid data provided")
            }
            Some(resource_address) => self.minter_badge_vault.authorize(|| {
                borrow_resource_manager!(*resource_address).update_non_fungible_data(id, data)
            }),
            None => panic!("unsupported badge type"),
        }
    }
}

#[derive(
    ScryptoCategorize, LegacyDescribe, Encode, Decode, Clone, Debug, Sequence, PartialEq, Hash, Eq,
)]
pub enum BadgeType {
    Insured,
    Insurer,
    InsuredClaim,
    InsurerMarketListing,
}
