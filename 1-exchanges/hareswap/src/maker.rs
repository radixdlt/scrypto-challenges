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

#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub struct SignedOrder {
    order: MatchedOrder,
    signature: Vec<u8>,
}

blueprint! {
    struct Maker {
        verifying_key: EcdsaPublicKey,
        callback_auth: Vault,
        //vaults: LazyMap<ResourceDef, Vault>,
    }

    impl Maker {
        pub fn instantiate(verifying_key: EcdsaPublicKey, callback_auth: Bucket) -> Component {
            // callback_auth may be empty if only used for validation not callback or if not needed
  //          let callback_auth_resource = callback_auth.resource_def();
            Self {
                verifying_key,
                callback_auth: Vault::with_bucket(callback_auth),
 //               callback_auth_resource,
            }.instantiate()
        }

        // #[auth(callback_auth_resource)]
        // pub fn default_swap(&mut self, from_taker: Bucket) -> Bucket {

        //     let maker_account_address: Address = Address::from_str("0").unwrap();
        //     let args = vec![scrypto_encode(&from_taker)];
        //     call_method(maker_account_address, "deposit", args);
        //     let args = vec![scrypto_encode(&from_taker)];
        //     let rtn = call_method(maker_account_address, "withdraw", args);
        //     scrypto_decode(&rtn).unwrap()
        // }


        // private, signature verification must happen first
        fn settle_order(&mut self, matched_order: MatchedOrder, from_taker: Bucket, taker_auth: BucketRef) -> /*fromMaker*/ Bucket {

            // check binding to taker - stops frontrunning the (public) SignedOrder
            assert_eq!(matched_order.order.taker_requirement.check_ref(&taker_auth), true);

            // create full taker request to check from_taker Bucket
            let taker_request = BucketRequest {
                resource: matched_order.order.taker_resource,
                contents: matched_order.taker_contents
            };

            // check from_taker matches order request
            assert_eq!(taker_request.check(&from_taker), true);

            // tail call the callback with auth
            self.callback_auth.authorize(|callback_auth| {

                // add args provided by the taker and the auth
                let mut extra_args = vec![
                    scrypto_encode(&from_taker), // taker tokens being sold
                    // scrypto_encode(&taker_auth), // auth to stop front running // don't think this needs to be passed on since we've verified it here
                    scrypto_encode(&callback_auth), // auth to (ultimately) enable release of maker tokens
                ];

                // execute the callback
                let result: Vec<u8> = match matched_order.maker_callback {
                    Callback::CallFunction {
                        package_address,
                        blueprint_name,
                        function,
                        mut args
                    } => {
                        args.append(&mut extra_args);
                        call_function(package_address, &blueprint_name, &function, args)
                    },
                    Callback::CallMethod {
                        component_address,
                        method,
                        mut args
                    } => {
                        args.append(&mut extra_args);
                        call_method(component_address, &method, args)
                    },
                };

                // return the result
                scrypto_decode(&result).unwrap()

            })
        }

        pub fn execute_order(&mut self, signed_order: SignedOrder, from_taker: Bucket, taker_auth: BucketRef) -> Bucket {
            let SignedOrder {
                order,
                signature,
            } = signed_order;
            verify(&self.verifying_key, &scrypto_encode(&order), &signature); // panics on failure
            self.settle_order(order, from_taker, taker_auth)
        }
    }
}
