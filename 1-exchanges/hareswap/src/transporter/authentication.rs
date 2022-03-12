use k256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use scrypto::prelude::EcdsaPublicKey;

#[derive(Debug, Clone)]
pub enum VerifyError {
    ParsePublicKeyError,
    ParseSignatureError,
    SignatureVerificationError,
}

/// verify the signature is made by the public_key on the serialized bytes
///
/// NOTE: the choice of encryption algorithm is not particularly relevent for this prototype but the EcdsaPublicKey type from Scrypto was reused.
/// Scrypto does not yet have an implementation so this implementation may be incompatible with what is chosen to verify transactions in the future
/// but that does not matter for how signature validation is utilized in Hareswap.  The Hareswap signatures and algorithm are completely independent.
///
/// WARNING: the cryptographic libraries used here are not audited, and this code is also not auditted.  Additionally, the documentation for the k256
/// create and related crates leaves a lot to be desired.  This works but the choices may not be optimal for future on-ledger use
pub fn verify(public_key: &EcdsaPublicKey, serialized: &[u8], signature: &[u8]) -> Result<(), VerifyError> {
    let pub_bytes = public_key.to_vec();
    let verifying_key: VerifyingKey =
        VerifyingKey::from_sec1_bytes(&pub_bytes).map_err(|_| VerifyError::ParsePublicKeyError)?;

    let sig = match Signature::from_der(signature) {
        Ok(s) => s.normalize_s().unwrap_or(s),
        Err(_) => return Err(VerifyError::ParseSignatureError),
    };

    match verifying_key.verify(serialized, &sig) {
        Ok(_) => Ok(()), // GOOD!
        Err(_) => Err(VerifyError::SignatureVerificationError),
    }
}

/// verify the signature is made by the public_key on the serialized bytes and PANIC on failure
pub fn verify_or_panic(public_key: &EcdsaPublicKey, serialized: &[u8], signature: &[u8]) {
    match verify(public_key, serialized, signature) {
        Err(VerifyError::ParsePublicKeyError) => panic!("verify: failed to parse Sec1 public key"),
        Err(VerifyError::ParseSignatureError) => panic!("verify: failed to parse ASN.1 signature"),
        Err(VerifyError::SignatureVerificationError) => panic!("verify: signature verification failed"),
        Ok(_) => (),
    }
}

/// sign the serialized message using the private key.  This is meant to be rather opaque.  Of course it does work with verify.
///
/// WARNING: no error checking, PANICs when given a bad private key
pub fn sign(serialized: &[u8], private_key: &[u8]) -> Vec<u8> {
    let signer = SigningKey::from_bytes(private_key).unwrap();
    let signature: Signature = signer.sign(serialized);
    signature.to_vec()
}
