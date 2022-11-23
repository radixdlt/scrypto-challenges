use scrypto::prelude::*;

pub(crate) struct ResourceAddressWrapper(pub(crate) ResourceAddress);

impl From<&ResourceAddress> for ResourceAddressWrapper {
    fn from(resource_address: &ResourceAddress) -> Self {
        Self(*resource_address)
    }
}

impl From<&Bucket> for ResourceAddressWrapper {
    fn from(bucket: &Bucket) -> Self {
        Self(bucket.resource_address().clone())
    }
}

impl From<&Vault> for ResourceAddressWrapper {
    fn from(bucket: &Vault) -> Self {
        Self(bucket.resource_address().clone())
    }
}

impl From<&Proof> for ResourceAddressWrapper {
    fn from(bucket: &Proof) -> Self {
        Self(bucket.resource_address().clone())
    }
}

macro_rules! debug_log_resources {
    ( $( $resource:expr ),+ ) => {
        {
            use crate::utils::ResourceAddressWrapper;
            $(
                debug!("Resource: {}={}", stringify!($resource), ResourceAddressWrapper::from(&$resource).0);
            )+
        }
    };
}
pub(crate) use debug_log_resources;

macro_rules! assert_resource_eq {
    ( $actual:expr, $expected:expr ) => {
        {
            use crate::utils::ResourceAddressWrapper;
            let actual_resource = ResourceAddressWrapper::from(&$actual).0;
            let expected_resource = ResourceAddressWrapper::from(&$expected).0;

            if actual_resource != expected_resource {
                panic!("Invalid resource: expected {} ({}) but was {} ({})",
                    expected_resource, stringify!($expected),
                    actual_resource, stringify!($actual), 
                );
            }
        }
    };
}
pub(crate) use assert_resource_eq;

pub(crate)  fn debug_log_non_fungible(message: &str, non_fungible: &Bucket) {
    debug!(
        "{}: #{},{}",
        message,
        non_fungible.non_fungible_ids().into_iter().next().unwrap(),
        non_fungible.resource_address()
    );
}