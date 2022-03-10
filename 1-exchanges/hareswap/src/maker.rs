use scrypto::prelude::*;
use sbor::*;

use super::transporter::blueprint::{Transporter, SealedVoucher};
use super::transporter::voucher::{Voucher, IsPassThruNFD};
use super::requirement::{BucketRequirement, BucketContents};
use super::account::*;

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
pub struct PartialOrder {
    pub maker_requirement: BucketRequirement,
    pub taker_resource: ResourceDef,
    pub taker_auth: BucketRequirement,
}

#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe, NonFungibleData)]
pub struct MatchedOrder {
    pub partial_order: PartialOrder,
    pub taker_contents: BucketContents,
    pub maker_callback: Callback,
}

// in a seperate module to deal with conflicting `decode` for sbor::Decode and NonFungibleData on MatchedOrder during derive
mod signed_order {
    use super::MatchedOrder;
    use super::{ResourceDef, NonFungibleKey};
    use sbor::{TypeId, Encode, Decode, Describe};
    #[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
    pub struct SignedOrder {
        pub order: MatchedOrder,
        pub voucher_resource: ResourceDef,
        pub voucher_key: NonFungibleKey,
        pub signature: Vec<u8>,
    }
}
pub use signed_order::SignedOrder;

blueprint! {
    struct Maker {
        verifying_key: EcdsaPublicKey,
        callback_auth: Vault,
        transporter: Transporter,
        order_def: ResourceDef,
        redeem_auth: Vault,
        account: CustodialAccount,
        account_component: Address,
        account_auth: Vault,
    }

    impl Maker {
        // when any args are types that cannot be handled by the transport manifest, use the "_raw" version
        // pub fn instantiate_raw(verifying_key: Vec<u8>, callback_auth: Option<Bucket>, account: Address, account_auth: Bucket) -> Component {
            // info!("here0");
            // Maker::instantiate(scrypto_decode(&verifying_key).unwrap(), callback_auth, account.into(), account_auth)
        // }

        pub fn instantiate(verifying_key: EcdsaPublicKey, callback_auth: Option<Bucket>, account: Component, account_auth: Bucket) -> Component {
            // change this redeem_auth to be a parameter
            let redeem_auth = Vault::with_bucket(ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(1));

            let transporter: Transporter = Transporter::instantiate(verifying_key, redeem_auth.resource_def()).into();
            let order_def = transporter.resource_def();  // this wont change, save it here

            info!("tokenized order resource address: {}", order_def.address());

            // default to expecting the default callback and so just make the callback_auth ourselves
            let callback_auth = if callback_auth.is_none() {
                Vault::with_bucket(ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(1))
            } else {
                Vault::with_bucket(callback_auth.unwrap())
            };

            Self {
                verifying_key,
                callback_auth,
                transporter,
                order_def,
                redeem_auth,
                account: account.clone().into(),
                account_component: account.address(),
                account_auth: Vault::with_bucket(account_auth)
            }.instantiate()
        }

        // example/default callback
        pub fn handle_order_default_callback(&mut self, matched_order: MatchedOrder, from_taker: Bucket, callback_auth: BucketRef) -> Bucket {
            let auth_requirement = BucketRequirement {
                resource: self.callback_auth.resource_def(),
                contents: BucketContents::Fungible(Decimal::one()) // assuming fungible otherwise use another callback
            };

            assert_eq!(auth_requirement.check_ref(&callback_auth), true, "handle_order_default_callback: callback_auth failed");

            // create full taker requirement to check from_taker Bucket
            let taker_requirement = BucketRequirement {
                resource: matched_order.partial_order.taker_resource,
                contents: matched_order.taker_contents
            };
            // match taker_requirement.contents {
            //     BucketContents::Fungible(ref mut amount) => *amount -= 1,
            //     _ => (),
            // };

            // let mut from_taker = from_taker;
            // let b0: Bucket = from_taker.take(1);

            // check from_taker fills the order request.... this callback will just take everything the taker gives us even if they overpay
            debug!("trying to check from_taker: {:?}", from_taker);
            // there's some bug where creating the bucketref makes the bucket unable to be depositted
            // from_taker.authorize(|from_taker_ref| {
            //     assert_eq!(taker_requirement.xxx_check_at_least_ref(&from_taker_ref), true, "handle_order_default_callback: taker Bucket does not meet requirements");
            //     from_taker_ref.drop(); // it does not auto drop so this is needed, the scrypto_statictypes BucketRefOf<T> has a nice Drop implementation to avoid these issues :)
            // });
            assert_eq!(taker_requirement.check_at_least(&from_taker), true, "handle_order_default_callback: taker Bucket does not meet requirements");
            //assert_eq!(taker_requirement.check_at_least_ref(from_taker.present()), true, "handle_order_default_callback: taker Bucket does not meet requirements");
            // let taker_actual = BucketRequirement {
            //     resource: from_taker.resource_def(),
            //     contents: BucketContents::Fungible(from_taker.amount()) // TODO handle NonFungible too
            // };
            // assert_eq!(taker_requirement.contents <= taker_actual.contents, "handle_order_default_callback: taker Bucket does not meet requirements");

            debug!("handle_order_default_callback: requirements passed, now deposit from_taker Bucket to Maker's account");

            // let mut from_taker = from_taker;
            // let b: Bucket = from_taker.take(1);

            // debug!("take 1 worked");

            // execute Account deposit
            self.account.deposit(from_taker);
            //call_method(self.account_component, "deposit", vec![scrypto_encode(&from_taker)]);

            // debug!("deposit worked");

            // self.account.deposit(b);
            // debug!("deposit worked again");
            // self.account.deposit(b0);
            // debug!("deposit worked again again");

            // execute Account withdrawl
            let withdraw_address = matched_order.partial_order.maker_requirement.resource.address();
            match matched_order.partial_order.maker_requirement.contents {
                BucketContents::Fungible(amount) => {
                    debug!("handle_order_default_callback: now withdraw from Maker's account {:?} and return", amount);
                    self.account_auth.authorize(|auth| self.account.withdraw(amount, withdraw_address, auth))
                },
                BucketContents::NonFungible(keys) => {
                    debug!("handle_order_default_callback: now withdraw from Maker's account {:?} and return", keys);
                    self.account_auth.authorize(|auth| self.account.withdraw_non_fungibles(keys, withdraw_address, auth))
                },
            }
        }


        // private, signature verification must happen first
        fn settle_order(&mut self, matched_order: MatchedOrder, from_taker: Bucket) -> /*fromMaker*/ Bucket {
            info!("settle_order: matched_order: {:?}", matched_order);
            // just calling the callback with our auth.  It will verify and execute

            // tail call the callback with auth
            self.callback_auth.authorize(|callback_auth| {

                // add args provided by the taker and the auth
                let mut extra_args = vec![
                    scrypto_encode(&matched_order), // the order
                    scrypto_encode(&from_taker), // taker tokens being sold
                    scrypto_encode(&callback_auth), // auth to (ultimately) enable release of maker tokens
                ];

                // execute the callback (ie. handle_order())
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

        // call this with the order token if doing fancy things
        pub fn execute_order_token(&mut self, order_tokens: Bucket, from_taker: Bucket) -> /* from_maker */ Bucket {
            assert_eq!(order_tokens.resource_def(), self.order_def, "execute_order_token: invalid order token");
            assert_eq!(order_tokens.amount(), Decimal::one(), "execute_order_token: cannot execute multiple order tokens at once"); // FUTURE: add another interface for multiple order tokens in the same call

            let orders: Vec<NonFungible<MatchedOrder>> = order_tokens.get_non_fungibles();
            let order = orders[0].data(); // made sure we have exactly 1

            let maker_bucket = self.settle_order(order, from_taker);

            debug!("execute_order_token: settle_order completed. now burn order token");
            // burn by giving it back to the transporter (to burn the token and turn it back into a Voucher) and then just ignore the Voucher
            let _ = self.transporter.make(order_tokens);

            debug!("execute_order_token: returning maker_bucket: {:?}", maker_bucket);
            maker_bucket
        }

        // call this as the entrypoint if doing fancy things
        // effectively trading in taker_auth requirement for the order token, but don't need to execute right away
        pub fn tokenize_order(&mut self, signed_order: SignedOrder, taker_auth: BucketRef) -> Bucket {
            let SignedOrder {
                order,
                voucher_resource,
                voucher_key,
                signature,
            } = signed_order;
            // check taker_auth matches the order before redeeming it.  (if it matches but the signature is bad it wont redeem properly anyway)
            // check binding to taker - stops frontrunning the (public) SignedOrder (along with using redeem_auth)
            assert_eq!(order.partial_order.taker_auth.check_at_least_ref(&taker_auth), true, "tokenize_order: taker_auth not accepted");

            let voucher = Voucher {
                resource_def: voucher_resource,
                key: Some(voucher_key),
                nfd: order.as_passthru(),
            };

            let sealed_voucher = SealedVoucher {
                serialized: scrypto_encode(&voucher),
                signature
            };

            self.redeem_auth.authorize(|auth|
                self.transporter.redeem(sealed_voucher, None, auth) // panics on bad vouchers
            )
        }

        // call this as the entrypoint for the boring way to execute the SignedOrder
        pub fn execute_order(&mut self, signed_order: SignedOrder, from_taker: Bucket, taker_auth: BucketRef) -> Bucket {
            let orders = self.tokenize_order(signed_order, taker_auth);
            info!("execute_order: SignedOrder successfully tokenized: {:?}", orders);
            self.execute_order_token(orders, from_taker)
        }

    }
}
