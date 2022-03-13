//! The [Maker] blueprint is the core blueprint for HareSwap.  It accepts SignedOrders and buckets and can execute or tokenize an order.
//! The signer is expected to instantiate one Maker Component, but is of course free to instantiate many, but only one is needed for all trades.
//! 
//! [Maker]: blueprint::Maker
use scrypto::prelude::*;

use super::account::SharedAccount;
use super::requirement::{BucketContents, BucketRequirement};
use super::transporter::blueprint::Transporter;
use super::transporter::voucher::Voucher;

// The HareSwap specific data types
use super::model::{Callback, MatchedOrder, SignedOrder};

blueprint! {
    /// Storage for a Maker component
    /// See Maker::instantiate for details
    struct Maker {
        verifying_key: EcdsaPublicKey,
        callback_auth: Vault,
        transporter: Transporter,
        order_def: ResourceDef,
        redeem_auth: Vault,
        account: SharedAccount,
        account_auth: Vault,
    }

    impl Maker {
        /// Creates a new Maker component
        ///
        /// Intended to be used by the HareSwap transaction signer to create the main component to handle on-ledger order submissions
        /// - verifying_key:  will be used to verify SignedOrders.
        /// - callback_auth: optional badge to use when executing Callback functions/methods that were "baked in" to the SignedOrder
        ///                  if not provided, authentication is generated internally.  NOTE:  Remember, internal does not mean private.
        ///                  A user could still read the resource address and make sure to check it in a custom Callback, that's a good thing.
        ///
        /// The next 2 arguments support the default swap implementation, ie. when the Callback points back to this component:
        /// - account: a component address supporting the SharedAccount deposit and withdraw interfaces (which are the same as the builtin Account)
        /// - account_auth: assets to use for auth against account when doing withdraw(...)
        pub fn instantiate(verifying_key: EcdsaPublicKey, callback_auth: Option<Bucket>, account: Component, account_auth: Bucket) -> Component {
            // change this redeem_auth to be a parameter
            let redeem_auth = Vault::with_bucket(ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(1));

            let transporter: Transporter = Transporter::instantiate(verifying_key, redeem_auth.resource_def()).into();
            let order_def = transporter.resource_def();  // this wont change, save it here

            info!("tokenized order resource address: {}", order_def.address());

            // default without explicit callback_auth is to expect the default callback with be used, so just generate our own internal use badge.
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
                account: account.into(), // convert the Component (address) passed in to a SharedAccount for use by the default callback.  If the callback and account interface mismatch, panic ensues.
                account_auth: Vault::with_bucket(account_auth)
            }.instantiate()
        }

        /// The default order settlement implementation
        ///
        /// Checks all the requirements and details of the MatchedOrder.
        /// Deposits the from_taker assets to self.account and uses the
        /// self.account_auth to withdraw the "from Maker" assets from the same
        /// SharedAccount, finally returning them.
        ///
        /// NOTE: this is a public function so it is reachable as a Callback,
        /// but it requires callback_auth to satisfy self.callback_auth The
        /// current implementation assumes a single NonFungible badge, matching
        /// the same limitations in SharedAccount.  More complex implementations
        /// are possible and can even be utilized without changes to HareSwap by
        /// using an alternate Callback.
        pub fn handle_order_default_callback(&mut self, matched_order: MatchedOrder, from_taker: Bucket, callback_auth: BucketRef) -> Bucket {
            let auth_requirement = BucketRequirement {
                resource: self.callback_auth.resource_def(),
                contents: BucketContents::Fungible(Decimal::one()) // assuming fungible otherwise use another callback
            };

            assert_eq!(auth_requirement.check_ref(&callback_auth), true, "handle_order_default_callback: callback_auth failed");

            // check the order has not expired  (in a world with oracles this would be based on some timestamp instead)
            let epoch = Context::current_epoch();
            assert!(epoch <= matched_order.deadline, "The order has expired.  Current epoch ({}) is past the order deadline ({})", epoch, matched_order.deadline);

            // create full taker requirement to check from_taker Bucket
            let taker_requirement = BucketRequirement {
                resource: matched_order.partial_order.taker_resource,
                contents: matched_order.taker_contents
            };

            // check from_taker fills the order request.... this callback will just take everything the taker gives us even if they overpay
            trace!("handle_order_default_callback: from_taker: {:?}", from_taker);
            assert_eq!(taker_requirement.check_at_least(&from_taker), true, "handle_order_default_callback: taker Bucket does not meet requirements");

            debug!("handle_order_default_callback: requirements passed, now deposit from_taker Bucket to Maker's account");

            // execute Account deposit
            self.account.deposit(from_taker);

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


        /// Settle the MatchedOrder by calling the signer's predetermined "Callback" functionality to ultimately return the "froMMaker" Bucket
        ///
        /// IMPORTANT: this method MUST be private.  It trusts the MatchedOrder and that is only possible because
        /// we can be sure it has been verified already.
        fn settle_order(&mut self, matched_order: MatchedOrder, from_taker: Bucket) -> /*fromMaker*/ Bucket {
            info!("settle_order: matched_order: {:?}", matched_order);

            // call the callback with auth returning it's result
            self.callback_auth.authorize(|callback_auth| {

                // add args provided by the taker and the auth
                let mut extra_args = vec![
                    scrypto_encode(&matched_order), // the order
                    scrypto_encode(&from_taker), // taker tokens being sold
                    scrypto_encode(&callback_auth), // auth to (ultimately) enable release of maker tokens
                ];

                // execute the callback (ie. handle_order_default_callback(...) or something custom)
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
                scrypto_decode(&result).expect("settle_order Callback must result in a Bucket")

            })
        }

        /// Execute the MatchedOrder represented by a NonFungible by sending in the "from Taker" Bucket and returning the "from Maker" Bucket.
        ///
        /// Ownership of the MatchedOrder NonFungible is synonymous with being
        /// allowed to execute the order (you of course also need to provide the
        /// from_taker assets)
        ///
        /// NOTE: this interface only supports a single order token.  Supporting
        /// multiple orders in the same call should be possible, but left as an
        /// exercise. :)
        pub fn execute_order_token(&mut self, order_tokens: Bucket, from_taker: Bucket) -> /* from_maker */ Bucket {
            assert_eq!(order_tokens.resource_def(), self.order_def, "execute_order_token: invalid order token");
            assert_eq!(order_tokens.amount(), Decimal::one(), "execute_order_token: cannot execute multiple order tokens at once"); // FUTURE: add another interface for multiple order tokens in the same call

            let orders: Vec<NonFungible<MatchedOrder>> = order_tokens.get_non_fungibles();
            let order = orders[0].data(); // already made sure we have exactly 1

            // finally settle the MatchedOrder, this is a non-public method call
            let maker_bucket = self.settle_order(order, from_taker);

            debug!("execute_order_token: settle_order completed. now burn order token");

            // burn by giving it back to the transporter (to burn the token and turn it back into a Voucher) and then just ignore the Voucher
            let _ = self.transporter.make(order_tokens);

            debug!("execute_order_token: returning maker_bucket: {:?}", maker_bucket);
            maker_bucket
        }

        /// Convert a SignedOrder into a NonFungible representing the same order.  ie. the entrypoint for doing "fancy things"
        ///
        /// Effectively allows the actor with taker_auth to "Transport" the
        /// off-ledger SignedOrder back on-ledger and do whatever they want with
        /// the tokenized order.  You can think of this as converting an
        /// authorization grant based on ownership of taker_auth to an
        /// authorization grant based on ownership of the non-fungible order
        /// token itself.  The token now includes both the authorization and the
        /// instruction all wrapped up together and can be handled freely in a
        /// truly asset oriented way.
        ///
        /// This is the ultimate in DeFi flexibility!
        /// Imaging reselling the order NonFungible to someone else, or using
        /// this order as a guarentee for further multiparty trades or as a way
        /// to haggle for a better deal with another counterparty.
        pub fn tokenize_order(&mut self, signed_order: SignedOrder, taker_auth: BucketRef) -> Bucket {
            let SignedOrder {
                order,
                voucher_resource,
                voucher_key,
                signature,
            } = signed_order;
            // check taker_auth matches the order before redeeming it.  (if it matches but the signature is bad it wont redeem properly anyway.  This stops frontrunning)
            assert_eq!(order.partial_order.taker_auth.check_at_least_ref(&taker_auth), true, "tokenize_order: taker_auth not accepted");

            // rebuild a voucher from the SignedOrder contents (ie. the MatchedOrder data and voucher metadata)
            let voucher = Voucher::from_nfd(voucher_resource, Some(voucher_key), order);

            // and then rebuild the sealed_voucher by serializing and including the signature
            let sealed_voucher = voucher.to_sealed(signature);

            // "transport" the MatchedOrder back into existance by redeeming the voucher.   Only this Maker is allowed to use this Transporter (thanks to redeem_auth)
            self.redeem_auth.authorize(|auth|
                self.transporter.redeem(sealed_voucher, None, auth) // panics on bad vouchers
            )
        }

        /// Execute the SignedOrder sending in the "from Taker" Bucket and returning the "from Maker" Bucket.  ie. the entrypoint for doing things the easy way
        ///
        /// Only the actor with taker_auth is allowed to execute the order to avoid frontrunning.
        /// Note using a real BucketRef for auth instead of forcing the originator of the PartialOrder to include
        /// some address or public key to be signed with the SignedOrder is more flexible and promotes composability
        pub fn execute_order(&mut self, signed_order: SignedOrder, from_taker: Bucket, taker_auth: BucketRef) -> Bucket {
            let orders = self.tokenize_order(signed_order, taker_auth);
            info!("execute_order: SignedOrder successfully tokenized: {:?}", orders);
            self.execute_order_token(orders, from_taker)
        }

    }
}
