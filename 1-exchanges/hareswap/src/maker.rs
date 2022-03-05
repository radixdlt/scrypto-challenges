use scrypto::prelude::*;
use sbor::*;

use super::transporter::authentication::verify;

#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub enum Callback {
    /// Calls a blueprint function.
    CallFunction {
        package_address: Address,
        blueprint_name: String,
        function: String,
        args: Vec<Vec<u8>>,
    },

    /// Calls a component method.
    CallMethod {
        component_address: Address,
        method: String,
        args: Vec<Vec<u8>>,
    },
}


#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub enum BucketRequestContents {
    Fungible(Decimal),
    NonFungible(Vec<NonFungibleKey>),
}

#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub struct BucketRequest {
    resource: ResourceDef,
    contents: BucketRequestContents,
}

impl BucketRequest {
    pub fn check_ref(&self, bucket_ref: &BucketRef) -> bool {
        if self.resource != bucket_ref.resource_def() {
            return false;
        }
        match &self.contents {
            BucketRequestContents::Fungible(amount) => { *amount == bucket_ref.amount() }, // TODO consider inequality or fuzzy bound instead
            BucketRequestContents::NonFungible(expected_keys) => {
                let actual_keys = bucket_ref.get_non_fungible_keys();
                *expected_keys == actual_keys // TODO handle vec order mismatch
            }
        }
    }
    pub fn check(&self, bucket: &Bucket) -> bool {
        bucket.authorize(|bucket_ref| self.check_ref(&bucket_ref))
    }
}

#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub struct PartialOrder {
    maker_request: BucketRequest,
    taker_resource: ResourceDef,
    taker_requirement: BucketRequest,
}

#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub struct MatchedOrder {
    order: PartialOrder,
    taker_contents: BucketRequestContents,
    maker_callback: Callback,
}

blueprint! {
    struct Maker {
        public_key: EcdsaPublicKey,
    }

    impl Maker {
        // private, signature verification must happen first
        fn settle_order(matched_order: MatchedOrder, from_taker: Bucket, taker_auth: BucketRef) -> /*fromMaker*/ Bucket {

            // check binding to taker
            assert_eq!(matched_order.order.taker_requirement.check_ref(&taker_auth), true);

            // create full taker request from each leg
            let taker_request = BucketRequest {
                resource: matched_order.order.taker_resource,
                contents: matched_order.taker_contents
            };

            // check from_taker matches order request
            assert_eq!(taker_request.check(&from_taker), true);

            // call callback .. or is this the callback?  this should be the callback which everyone uses the same one? but where to deposit?
            // pretend this is constant and calls the per-maker callback

            let result: Vec<u8> = match matched_order.maker_callback {
                Callback::CallFunction {
                    package_address,
                    blueprint_name,
                    function,
                    mut args
                } => {
                    args.push(scrypto_encode(&from_taker));
                    args.push(scrypto_encode(&taker_auth));
                    call_function(package_address, &blueprint_name, &function, args)
                },
                Callback::CallMethod {
                    component_address,
                    method,
                    mut args
                } => {
                    args.push(scrypto_encode(&from_taker));
                    args.push(scrypto_encode(&taker_auth));
                    call_method(component_address, &method, args)
                },
            };

            scrypto_decode(&result).unwrap()
        }

        pub fn execute_order(&self, order: MatchedOrder, signature: Vec<u8>, from_taker: Bucket, taker_auth: BucketRef) -> Bucket {
            verify(&self.public_key, &scrypto_encode(&order), &signature);
            Maker::settle_order(order, from_taker, taker_auth)
        }
    }
}
