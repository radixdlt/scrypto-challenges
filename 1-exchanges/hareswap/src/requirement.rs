use std::cmp::Ordering;

use scrypto::prelude::*;
use sbor::*;

#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub enum BucketContents {
    Fungible(Decimal),
    NonFungible(BTreeSet<NonFungibleKey>),
}

// Implement <BucketRef> == <BucketContents> comparisons
impl PartialEq<BucketContents> for BucketRef {
    fn eq(&self, other: &BucketContents) -> bool {
        let bucket_type = self.resource_def().resource_type();

        match (bucket_type, other) {
            (ResourceType::Fungible { .. }, BucketContents::Fungible(amount)) => { self.amount() == *amount },
            (ResourceType::NonFungible, BucketContents::NonFungible(keys)) => {
                // avoid copies by comparing sets of references.  Iterates over the sets more than strictly needed to make the code simplier
                let contents_keys: BTreeSet<&NonFungibleKey> = keys.iter().collect();
                let self_keys = self.get_non_fungible_keys();
                let self_keys: BTreeSet<&NonFungibleKey> = self_keys.iter().collect();
                self_keys == contents_keys
            },
            (_, _) => false,
        }
    }
}

impl PartialOrd<BucketContents> for BucketRef {
    fn partial_cmp(&self, other: &BucketContents) -> Option<Ordering> {
        let bucket_type = self.resource_def().resource_type();

        match (bucket_type, other) {
            (ResourceType::Fungible { .. }, BucketContents::Fungible(amount)) => { 
                Some(self.amount().cmp(amount))
            },
            (ResourceType::NonFungible, BucketContents::NonFungible(keys)) => {
                // avoid copies by comparing sets of references.  Iterates over the sets more than strictly needed to make the code simplier
                let contents_keys: BTreeSet<&NonFungibleKey> = keys.iter().collect();
                let self_keys = self.get_non_fungible_keys();
                let self_keys: BTreeSet<&NonFungibleKey> = self_keys.iter().collect();
                Some(self_keys.cmp(&contents_keys))
            },
            (_, _) => None,
        }
    }
}

#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub struct BucketRequirement {
    pub resource: ResourceDef,
    pub contents: BucketContents,
}

impl BucketRequirement {
    pub fn check_ref(&self, bucket_ref: &BucketRef) -> bool {
        // same resource
        if self.resource != bucket_ref.resource_def() {
            return false;
        }
        // contents exactly match
        bucket_ref == &self.contents
    }
    pub fn check(&self, bucket: &Bucket) -> bool {
        bucket.authorize(|bucket_ref| self.check_ref(&bucket_ref))
    }
    pub fn check_at_least_ref(&self, bucket_ref: &BucketRef) -> bool {
        // same resource
        if self.resource != bucket_ref.resource_def() {
            return false;
        }
        // bucket_ref holds at least the required contents (or more)
        bucket_ref >= &self.contents
    }
    pub fn check_at_least(&self, bucket: &Bucket) -> bool {
        bucket.authorize(|bucket_ref| self.check_at_least_ref(&bucket_ref))
    }
}