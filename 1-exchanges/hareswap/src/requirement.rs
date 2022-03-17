//! Functionality to describe and compare expectations about an asset in a bucket
use std::cmp::Ordering;
use std::fmt;

use sbor::*;
use scrypto::prelude::*;

/// Describes the type and amount of Fungible or NonFungible assets in a Bucket (not including specific resource address)
/// Can be compared via ParitalOrd with a BucketRef (and thus with a Bucket) for easy correct checking of requirements
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub enum BucketContents {
    Fungible(Decimal),
    NonFungible(BTreeSet<NonFungibleKey>),
}

/// Implement Display in terms of Debug
impl fmt::Display for BucketContents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) }
}

impl PartialEq<BucketContents> for BucketRef {
    fn eq(&self, other: &BucketContents) -> bool {
        let bucket_type = self.resource_def().resource_type();

        match (bucket_type, other) {
            (ResourceType::Fungible { .. }, BucketContents::Fungible(amount)) => self.amount() == *amount,
            (ResourceType::NonFungible, BucketContents::NonFungible(keys)) => {
                // avoid copies by comparing sets of references.  Iterates over the sets more than strictly needed to make the code simplier
                let contents_keys: BTreeSet<&NonFungibleKey> = keys.iter().collect();
                let self_keys = self.get_non_fungible_keys();
                let self_keys: BTreeSet<&NonFungibleKey> = self_keys.iter().collect();
                self_keys == contents_keys
            }
            (_, _) => false,
        }
    }
}

impl PartialOrd<BucketContents> for BucketRef {
    fn partial_cmp(&self, other: &BucketContents) -> Option<Ordering> {
        trace!(
            "partial_cmp BucketRef to BucketContents: {:?} =?= {:?}",
            self.resource_def().resource_type(),
            other
        );
        let bucket_type = self.resource_def().resource_type();

        match (bucket_type, other) {
            (ResourceType::Fungible { .. }, BucketContents::Fungible(amount)) => {
                trace!(
                    "partial_cmp BucketRef to BucketContents: Fungible {:?} =?= {:?}",
                    self.amount(),
                    amount
                );
                Some(self.amount().cmp(amount))
            }
            (ResourceType::NonFungible, BucketContents::NonFungible(keys)) => {
                // avoid copies by comparing sets of references.  Iterates over the sets more than strictly needed to make the code simplier
                let contents_keys: BTreeSet<&NonFungibleKey> = keys.iter().collect();
                let self_keys = self.get_non_fungible_keys();
                let self_keys: BTreeSet<&NonFungibleKey> = self_keys.iter().collect();
                trace!(
                    "partial_cmp BucketRef to BucketContents: NonFungible {:?} =?= {:?}",
                    self_keys,
                    contents_keys
                );
                Some(self_keys.cmp(&contents_keys))
            }
            (_, _) => None,
        }
    }
}

/// Combines a BucketContents with a specific ResourceDef to create a "requirement" on a Bucket which is easliy checked
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub struct BucketRequirement {
    pub resource: ResourceDef,
    pub contents: BucketContents,
}

impl BucketRequirement {
    /// Check a BucketRef exactly matches the requirement
    pub fn check_ref(&self, bucket_ref: &BucketRef) -> bool {
        // same resource
        if self.resource != bucket_ref.resource_def() {
            return false;
        }
        // contents exactly match
        *bucket_ref == self.contents
    }

    /// Check a Bucket exactly matches the requirement
    pub fn check(&self, bucket: &Bucket) -> bool {
        bucket.authorize(|bucket_ref| {
            let r = self.check_ref(&bucket_ref);
            bucket_ref.drop(); // it does not auto drop so this is needed, the scrypto_statictypes BucketRefOf<T> has a nice Drop implementation to avoid these issues :)
            r
        })
    }

    /// Check a BucketRef contains at least as much as the requirement (in quantity of Fungible or subset of NonFungible) of the correct resource
    pub fn check_at_least_ref(&self, bucket_ref: &BucketRef) -> bool {
        // same resource
        if self.resource != bucket_ref.resource_def() {
            return false;
        }

        // bucket_ref holds at least the required contents (or more)
        *bucket_ref >= self.contents
    }

    /// Check a Bucket contains at least as much as the requirement (in quantity of Fungible or subset of NonFungible) of the correct resource
    pub fn check_at_least(&self, bucket: &Bucket) -> bool {
        bucket.authorize(|bucket_ref| {
            let r = self.check_at_least_ref(&bucket_ref);
            bucket_ref.drop(); // it does not auto drop so this is needed, the scrypto_statictypes BucketRefOf<T> has a nice Drop implementation to avoid these issues :)
            r
        })
    }
}

/// Possible errors when parsing a BucketContents from string
#[derive(Debug, Clone)]
pub enum ParseBucketContentsError {
    ParseFungibleError(ParseDecimalError),
    ParseNonFungibleError(ParseNonFungibleKeyError),
}

/// Implement Display in terms of Debug
impl fmt::Display for ParseBucketContentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) }
}

impl std::str::FromStr for BucketContents {
    type Err = ParseBucketContentsError;

    /// oversimplified string to BucketContents parsing
    ///
    /// A string with a "." will be parsed as a Decimal for the Funbible amount
    /// Otherwise, a single NonFungibleKey or comma-seperated list of keys to create the set of NonFungibles
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // treat the string as a set of NonFungibleKeys if there is a comma, otherwise Fungible
        let contents = if s.contains(".") {
            BucketContents::Fungible(Decimal::from_str(s).map_err(Self::Err::ParseFungibleError)?)
        } else {
            let set_result: Result<BTreeSet<NonFungibleKey>, _> =
                s.split(",").map(|s| NonFungibleKey::from_str(s)).collect();
            let set = set_result.map_err(Self::Err::ParseNonFungibleError)?;
            BucketContents::NonFungible(set)
        };
        Ok(contents)
    }
}

/// Implement TryFrom in terms of FromStr
impl TryFrom<&str> for BucketContents {
    type Error = ParseBucketContentsError;

    fn try_from(s: &str) -> Result<Self, Self::Error> { BucketContents::from_str(s) }
}
/// Implement in terms of TryFrom<&str>
impl TryFrom<String> for BucketContents {
    type Error = ParseBucketContentsError;

    fn try_from(s: String) -> Result<Self, Self::Error> { BucketContents::from_str(&s) }
}
