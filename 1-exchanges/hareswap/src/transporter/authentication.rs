use scrypto::prelude::EcdsaPublicKey;
use k256::{
    //EncodedPoint, ecdsa::recoverable::Signature as RSignature
    ecdsa::{recoverable::Signature as RSignature, Signature, SigningKey, VerifyingKey, signature::Verifier, signature::Signer},
    //ecdsa::recoverable::*,
};
//use k256::ecdsa::*;
//use sha2::Sha256;

// verify or panic
pub fn verify(public_key: &EcdsaPublicKey, serialized: &[u8], signature: &[u8]) {
    // assuming EcdsaPublicKey will continue to be a [u8;33] and we can store the bytes in sec1 format
    let pub_bytes = public_key.to_vec();
    let verifying_key: VerifyingKey = VerifyingKey::from_sec1_bytes(&pub_bytes).expect("verify: failed to parse verifying public key");

    // let sig = match Signature::from_der(signature) {
    //     Ok(s) => s,
    //     Err(_) => panic!("verify: failed to parse signature ASN.1"),
    // };
    //let rsignature:T <k256::ecdsa::Signature as TryFrom<&[u8]>>::try_from(signature);
    //let rsignature: <Signature as TryFrom<&u8>>::try_from(signature);
    //let rsignature: k256::ecdsa::recoverable::Signature = <k256::ecdsa::recoverable::Signature as TryFrom<&[u8]>>::try_from(signature).unwrap();
    let rsignature = RSignature::try_from(signature).unwrap();
    let sig = Signature::from(rsignature);

    match verifying_key.verify(serialized, &sig) {
        Ok(_) => (), // GOOD!
        Err(_) => panic!("verify: signature verification failed"),
    }
    // // verify with public key extraction
    // let sig = RSignature::try_from(signature).unwrap();
    // let prehash = Sha256::new().chain(serialized);
    // let pk = sig.recover_verify_key_from_digest(prehash).unwrap();
    // assert_eq!(&public_key.to_vec(), EncodedPoint::from(&pk).as_bytes(), "verify: signature verification failed, public key mismatch");
}
// use k256::{
//     ecdsa::{signature::Signer, recoverable, /*signature::RandomizedSigner*/},
// //    elliptic_curve::{Generate, rand_core::OsRng},
// //    SecretKey, PublicKey
// };

pub fn sign(serialized: &[u8], private_key: &[u8]) -> Vec<u8> {
    //let signer = Signer::new(&private_key).expect("private key invalid");
    //let signature: RSignature = signer.sign(serialized);
    let signer = SigningKey::from_bytes(&private_key).unwrap();
    let signature: Signature = signer.sign(serialized);
    signature.to_vec()
}