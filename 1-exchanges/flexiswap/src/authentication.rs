use scrypto::prelude::EcdsaPublicKey;
use k256::{
    ecdsa::{Signature, VerifyingKey, signature::Verifier},
};

// verify or panic
pub fn verify(public_key: &EcdsaPublicKey, serialized: &[u8], signature: &[u8]) {
    // assuming EcdsaPublicKey will continue to be a [u8;33] and we can store the bytes in sec1 format
    let pub_bytes = public_key.to_vec();
    let verifying_key: VerifyingKey = VerifyingKey::from_sec1_bytes(&pub_bytes).expect("failed to parse verifying public key");

    let sig = match Signature::from_der(signature) {
        Ok(s) => s,
        Err(_) => panic!("failed to parse signature ASN.1"),
    };

    match verifying_key.verify(serialized, &sig) {
        Ok(_) => (), // GOOD!
        Err(_) => panic!("signature verify failed"),
    }
}