use scrypto::prelude::*;
use hex;

mod webauthn;

type TxHash = String;
//type TxHash = Hash;

blueprint! {
    struct MFAOracle {
        registration_index: u128,
        authorized_transactions: HashSet<TxHash>,
        rp_creds: webauthn::CredMap,
        rp: String,
        origin: String,
    }

    impl MFAOracle {
        /// New LocalComponent with defaults
        fn new_local(rp: String, origin: String) -> LocalComponent {
            Self {
                registration_index: 0u128,
                authorized_transactions: Default::default(),
                rp_creds: Default::default(),
                rp,
                origin,
            }
            .instantiate()
        }

        /// new with AccessRule derived from Proof
        fn new_internal(rp: String, origin: String, owner: Proof) -> ComponentAddress {
            let nf_addresses: Vec<NonFungibleAddress> = owner.non_fungible_ids().into_iter().map(|id| NonFungibleAddress::new(owner.resource_address(), id)).collect();
            assert_ne!(nf_addresses.len(), 0, "Proof must contain at least 1 NonFungible to protect MFA registration");

            let auth = AccessRules::new()
            .method("register", rule!(require_all_of(nf_addresses)))
            .default(rule!(allow_all));

            Self::new_local(rp, origin)
            .add_access_check(auth)
            .globalize()
        }

        /// New for localhost
        pub fn new_localhost(owner: Proof) -> ComponentAddress {
            Self::new_internal("localhost".to_owned(), "http://localhost:8080".to_owned(), owner)
        }

        /// New with customizable relying party and origin
        pub fn new(rp: String, origin: String, owner: Proof) -> ComponentAddress {
            Self::new_internal(rp, origin, owner) // should be ok to pass through proof since this is not a new scrypto call frame
        }

        /// Register a new MFA device.  Proof is needed to allow to register, sidestepping the typical registration ceremony which provides no additional security.
        pub fn register(&mut self, user_id: String, user_name: String, user_display_name: String, registration_json: String) {
            let me: ComponentAddress = if let ScryptoActor::Component(addr) = Runtime::actor().actor() {
                addr
            } else {
                panic!("my component address not found"); // should not happen
            };

            debug!("Trying to register new MFA device {} for {} on component {} with arg {}", user_id, user_name, me, registration_json);

            let challenge = self.get_registration_challenge(self.rp.clone(), self.origin.clone());

            let mut ctx = webauthn::RPContext::new(&mut self.rp_creds);
            ctx.start_register(self.rp.clone(), user_id.clone(), user_name.clone(), user_display_name, challenge);
            ctx.complete_register(&self.origin, &user_id, &registration_json);

            info!("Registered new MFA device {} for {} on component {}", user_id, user_name, me);

            // registration succeeds if we get to here, increment the nonce
            self.registration_index += 1;
        }

        /// generate the challenge deterministically bound to rp, origin, component and use nonce to avoid replay
        /// you could use this as a read-only function if the frontend wanted to trust the gateway, though better not to
        pub fn get_registration_challenge(&self, rp: String, origin: String) -> String {
            let me: ComponentAddress = if let ScryptoActor::Component(addr) = Runtime::actor().actor() {
                addr
            } else {
                panic!("my component address not found"); // should not happen
            };
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            debug!("hash input: component_address (without 0x02 prefix): {:?}", me.0);
            hasher.update(me.0);
            debug!("hash input: rp: {}", rp);
            hasher.update(rp);
            debug!("hash input: origin: {}", origin);
            hasher.update(origin);
            debug!("hash input: registration_index: {:?}", self.registration_index.to_be_bytes());
            hasher.update(self.registration_index.to_le_bytes());
            let challenge = hasher.finalize();
            debug!("hash output: {:?}", challenge);
            // challenge must be base64 encoded when stored in the ctx so it is
            // compared properly to auth result but needs to be decoded for call
            // to credential.get on the frontend (that could be a bug in webpki)
            let b64_challenge = base64::encode_config(challenge, base64::URL_SAFE_NO_PAD);
            debug!("challenge base64: {:?}", b64_challenge);
            b64_challenge.to_owned()
        }

        /// generate the challenge deterministically based bound to rp, origin, component and current transaction hash
        /// you could use this as a read-only function if the frontend wanted to trust the gateway, though better not to
        pub fn get_auth_challenge(&self, rp: String, origin: String, txhash: String) -> String {
            let me: ComponentAddress = if let ScryptoActor::Component(addr) = Runtime::actor().actor() {
                addr
            } else {
                panic!("my component address not found"); // should not happen
            };
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(me.to_vec());
            hasher.update(rp);
            hasher.update(origin);
            hasher.update(hex::decode(txhash).expect("txhash was not formatted properly"));
            let challenge = hasher.finalize();
            // challenge must be base64 encoded when stored in the ctx so it is
            // compared properly to auth result but needs to be decoded for call
            // to credential.get on the frontend (that could be a bug in webpki)
            let b64_challenge = base64::encode_config(challenge, base64::URL_SAFE_NO_PAD);
            debug!("challenge base64: {:?}", b64_challenge);
            b64_challenge.to_owned()
        }

        /// authorize a previusly failed transaction by returning the webauthn authentication response by user_id with the challenge including a specific txhash
        pub fn authorize_transaction(&mut self, user_id: String, response: String, txhash: String) {
            if self.authorized_transactions.contains(&txhash) {
                panic!("Transaction already authorized: {}", txhash);
            }
            let rp = self.rp.clone();
            let origin = self.origin.clone();

            let challenge = self.get_auth_challenge(rp.clone(), origin.clone(), txhash.to_string());

            let mut ctx = webauthn::RPContext::new(&mut self.rp_creds);
            let _request = ctx.get_sign_request(rp, user_id.clone(), challenge);
            ctx.check_sign_response(&user_id, &response, &origin);

            // if we get here without panicing MFA has been validated
            info!("MFA device {} has authorized transaction: {}", user_id, txhash);

            self.authorized_transactions.insert(txhash);
        }

        /// Check that an MFA authentication has been recorded for this transaction
        pub fn check(&self) {
            let txhash = Runtime::transaction_hash().to_string();
            if !self.authorized_transactions.contains(&txhash) {
                panic!("MFA Needed for Transaction: {}", txhash);
            }
        }

        // TODO optional API in addition to "check" which returns a minted NFT as proof of
        // MFA authentication for use in more complex transactions
    }
}

#[cfg(feature = "testing")]
pub mod tester {
    use super::*;
    blueprint! {
        struct Tester {
        }

        impl Tester {

            pub fn test_register() {
                webauthn::testing::test_register()
            }

            pub fn test_validation() {
                webauthn::testing::test_validation()
            }
        }
    }
}
//pub use tester::Tester;