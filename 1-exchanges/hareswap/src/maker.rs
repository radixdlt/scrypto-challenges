use scrypto::prelude::*;
use sbor::*;

use super::transporter::blueprint::{Transporter, SealedVoucher};
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
    pub maker_request: BucketRequirement,
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
    use sbor::{TypeId, Encode, Decode, Describe};
    #[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
    pub struct SignedOrder {
        pub order: MatchedOrder,
        pub signature: Vec<u8>,
    }
}
use signed_order::SignedOrder;

#[derive(TypeId, Encode, Decode, Describe)]
pub struct AccountInfo {
    account: Component,
    account_auth: Bucket
}

blueprint! {
    struct Maker {
        verifying_key: EcdsaPublicKey,
        callback_auth: Vault,
        transporter: Transporter,
        order_def: ResourceDef,
        redeem_auth: Vault,
        account: CustodialAccount,
        account_auth: Vault,
    }

    impl Maker {
        pub fn instantiate(verifying_key: EcdsaPublicKey, callback_auth: Bucket, account: Component, account_auth: Bucket) -> Component {

            let redeem_auth = Vault::with_bucket(ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(1));

            let transporter: Transporter = Transporter::instantiate(verifying_key, redeem_auth.resource_def()).into();
            let order_def = transporter.resource_def();  // this wont change, save it here

            Self {
                verifying_key,
                callback_auth: Vault::with_bucket(callback_auth),
                transporter,
                order_def,
                redeem_auth,
                account: account.into(),
                account_auth: Vault::with_bucket(account_auth)
            }.instantiate()
        }

        pub fn default_swap(&mut self, from_taker: Bucket) -> Bucket {

            let maker_account_address: Address = Address::from_str("0").unwrap();
            let args = vec![scrypto_encode(&from_taker)];
            call_method(maker_account_address, "deposit", args);
            let args = vec![scrypto_encode(&from_taker)];
            let rtn = call_method(maker_account_address, "withdraw", args);
            scrypto_decode(&rtn).unwrap()
        }

        // example/default callback
        pub fn handle_order_default_callback(&mut self, matched_order: MatchedOrder, from_taker: Bucket, callback_auth: BucketRef) -> Bucket {
            let auth_requirement = BucketRequirement {
                resource: self.callback_auth.resource_def(),
                contents: BucketContents::Fungible(Decimal::one()) // assuming fungible otherwise use another callback
            };

            assert_eq!(auth_requirement.check_ref(&callback_auth), true, "callback_auth failed");

            // create full taker request to check from_taker Bucket
            let taker_request = BucketRequirement {
                resource: matched_order.partial_order.taker_resource,
                contents: matched_order.taker_contents
            };

            // check from_taker fills the order request.... this callback will just take everything the taker gives us even if they overpay
            assert_eq!(taker_request.check_at_least(&from_taker), true);

            // execute Account deposit
            self.account.deposit(from_taker);

            // execute Account withdrawl
            let withdraw_address = matched_order.partial_order.maker_request.resource.address();
            match matched_order.partial_order.maker_request.contents {
                BucketContents::Fungible(amount) => {
                    self.account_auth.authorize(|auth| self.account.withdraw(amount, withdraw_address, auth))
                },
                BucketContents::NonFungible(keys) => {
                    self.account_auth.authorize(|auth| self.account.withdraw_non_fungibles(keys, withdraw_address, auth))
                },
            }
        }


        // private, signature verification must happen first
        fn settle_order(&mut self, matched_order: MatchedOrder, from_taker: Bucket) -> /*fromMaker*/ Bucket {
            // just calling the callback with our auth.  It will verify and execute

            // tail call the callback with auth
            self.callback_auth.authorize(|callback_auth| {

                // add args provided by the taker and the auth
                let mut extra_args = vec![
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
            assert_eq!(order_tokens.resource_def(), self.order_def, "invalid order token");
            assert_eq!(order_tokens.amount(), Decimal::one(), "cannot execute multiple order tokens at once"); // TODO, another interface for multiple order tokens in the same call

            let orders: Vec<NonFungible<MatchedOrder>> = order_tokens.get_non_fungibles();
            let order = orders[0].data(); // made sure we have exactly 1

            let maker_bucket = self.settle_order(order, from_taker);
            // need to burn the order_token bucket now that it's settled
            order_tokens.burn();
            maker_bucket
        }

        // call this as the entrypoint if doing fancy things
        // effectively trading in taker_auth requirement for the order token, but don't need to execute right away
        pub fn tokenize_order(&mut self, signed_order: SignedOrder, taker_auth: BucketRef) -> Bucket {
            let SignedOrder {
                order,
                signature,
            } = signed_order;
            // check taker_auth matches the order before redeeming it.  (if it matches but the signature is bad it wont redeem properly anyway)
            // check binding to taker - stops frontrunning the (public) SignedOrder (along with using redeem_auth)
            assert_eq!(order.partial_order.taker_auth.check_at_least_ref(&taker_auth), true);

            let voucher = SealedVoucher {
                serialized: scrypto_encode(&order),
                signature
            };

            self.redeem_auth.authorize(|auth|
                self.transporter.redeem(voucher, None, auth) // panics on bad vouchers
            )
        }

        // call this as the entrypoint for the boring way to execute the SignedOrder
        pub fn execute_order(&mut self, signed_order: SignedOrder, from_taker: Bucket, taker_auth: BucketRef) -> Bucket {
            let orders = self.tokenize_order(signed_order, taker_auth);
            self.execute_order_token(orders, from_taker)
        }

    }
}