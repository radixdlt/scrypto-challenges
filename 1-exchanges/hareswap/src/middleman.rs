//! An example for a "Middleman" component used in the "Tokenized Swap" demo.
//!
//! This advanced example has no error checking and is "single use" so
//! maybe not too realistic, but a good demo. Shows the power of
//! tokenizing an order and Maker's custom callbacks
use scrypto::prelude::*;

use super::account::SharedAccount;
use super::requirement::BucketContents;

// The HareSwap specific data types
use super::model::MatchedOrder;

use super::maker::Maker;

blueprint! {
    struct Middleman {
        callback_auth: ResourceDef,
        account: SharedAccount,
        commitment: Option<Vault>,
    }

    impl Middleman {
        /// create a Middleman with the account to deposit their cut of the sale
        pub fn instantiate(account: Component) -> (Component, Bucket) {
            let callback_auth_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(1);
            let callback_auth = callback_auth_bucket.resource_def();
            (Self {
                callback_auth,
                account: account.into(), // makes the code below more readable and assumes the "deposit" interface on the component
                commitment: None, // we don't know the resource def up front so can't create the Vault
            }.instantiate(), callback_auth_bucket)
        }

        /// save the order here and send everything else through to the account
        pub fn add_orders(&mut self, buckets: Vec<Bucket>) {
            // a bit of a hack but since we can't do auth on this method and still call it "WITH_ALL_RESOURCES"
            // going to check that one of the buckets actually matches the auth.  Works since we'll pass it through anyway
            // probably a better way to organize this middleman code, but it's just for a demo.
            //
            // this all stems from a limitation with transaction manifests that there's no way to say
            // "bucket from the last instruction".  And since we wont know the order token's resource in advance
            // (in the general case) the design for this relies on using "CALL_METHOD_WITH_ALL_RESOURCES" since you don't have
            // to specificy the resource address for that to work
            let mut seen_auth = false;
            let mut seen_order = false;

            // assume the NonFungible buckets are what we want (this is good enough for a demo)
            let mut deposit_buckets = vec![];
            for b in buckets.into_iter() {
                if matches!(b.resource_def().resource_type(), ResourceType::NonFungible{..}) {
                    // if this happens more than once it will panic since it will lose the swapped out vault
                    let _old_vault = self.commitment.replace(Vault::with_bucket(b));
                    seen_order = true;
                } else {
                    if b.resource_def() == self.callback_auth {
                        seen_auth = true;
                    }
                    deposit_buckets.push(b);
                }
            }
            assert!(seen_auth, "bad auth for add_orders");
            assert!(seen_order, "no order seen in any add_orders buckets");
            self.account.deposit_batch(deposit_buckets);
        }

        /// See the middleman demo: An order callback which uses a stored order to execute this order and takes a cut
        #[auth(callback_auth)]
        pub fn middleman_callback(&mut self, matched_order: MatchedOrder, from_taker: Bucket) -> Bucket {
            // look at the order and figure out how much we will owe, assumes we're acting as a middleman for sell orders
            let contents = matched_order.quote_contents;
            let owed_amount = match contents {
                BucketContents::Fungible(amount) => {
                    debug!("middleman_callback: will owe amount {:?}", amount);
                    amount
                },
                BucketContents::NonFungible(_keys) => {
                    panic!("this middleman only accepts fungible payments");
                },
            };

            // execute the tokenized order "commitment"
            let order_tokens = self.commitment.as_mut().unwrap().take_all();
            //self.commitment = None;
            let order: MatchedOrder = order_tokens.get_non_fungible_data(&order_tokens.get_non_fungible_key()); // assumes single order
            let maker: Maker = order.maker_address.into(); // so we can use the interface without marshalling the args ourselves
            let mut payment = maker.execute_order_token(order_tokens, from_taker);

            // take our cut of the payment and deposit into account
            let owed = payment.take(owed_amount);
            self.account.deposit(payment);

            // return amount owed to the seller
            owed
        }
    }
}
