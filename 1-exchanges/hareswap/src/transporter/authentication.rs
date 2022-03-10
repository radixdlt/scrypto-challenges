use scrypto::prelude::EcdsaPublicKey;
use k256::{
    //EncodedPoint, ecdsa::recoverable::Signature as RSignature
    ecdsa::{Signature, SigningKey, VerifyingKey, signature::Verifier, signature::Signer},
    //ecdsa::recoverable::*,
};
//use k256::ecdsa::*;
//use sha2::Sha256;

pub fn verify(public_key: &EcdsaPublicKey, serialized: &[u8], signature: &[u8]) {
    let pub_bytes = public_key.to_vec();
    let verifying_key: VerifyingKey = VerifyingKey::from_sec1_bytes(&pub_bytes).expect("verify: failed to parse Sec1 public key");

    let sig = match Signature::from_der(&signature) {
        Ok(s) => s.normalize_s().unwrap_or(s),
        Err(_) => panic!("verify: failed to parse ASN.1 signature"),
    };

    match verifying_key.verify(serialized, &sig) {
        Ok(_) => (), // GOOD!
        Err(_) => panic!("verify: signature verification failed"),
    }
}


pub fn sign(serialized: &[u8], private_key: &[u8]) -> Vec<u8> {
    //let signer = Signer::new(&private_key).expect("private key invalid");
    //let signature: RSignature = signer.sign(serialized);
    let signer = SigningKey::from_bytes(&private_key).unwrap();
    let signature: Signature = signer.sign(serialized);
    signature.to_vec()
}