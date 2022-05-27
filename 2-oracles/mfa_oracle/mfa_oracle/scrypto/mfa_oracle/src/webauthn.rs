//! Server-side implemenation of webauthn.
//! Adapted from slauth example server for use on-ledger
//!
use serde_json::json;
use slauth::webauthn::{
    error::{CredentialError as CredE, Error::CredentialError},
    proto::{
        constants::WEBAUTHN_CHALLENGE_LENGTH,
        raw_message::CredentialPublicKey,
        web_message::{
            PublicKeyCredential, PublicKeyCredentialCreationOptions,
            PublicKeyCredentialRequestOptions,
        },
    },
    server::{
        CredentialCreationBuilder, CredentialCreationVerifier, CredentialRequestBuilder,
        CredentialRequestVerifier,
    },
};
use std::{collections::HashMap, str::FromStr};

use scrypto::prelude::{Encode, Decode, TypeId, Describe};
use sbor::{Decoder, Encoder, DecodeError, Type};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Cred {
    public_key: CredentialPublicKey,
    sign_count: u32,
}

impl TypeId for Cred {
    // TypeId
    #[inline(always)]
    fn type_id() -> u8 {
        <String as TypeId>::type_id()
    }
}

impl Encode for Cred {
    // Encode
    #[inline(always)]
    fn encode_value(&self, encoder: &mut Encoder) {
        let json_str = serde_json::to_string(self).unwrap();
        <String as Encode>::encode_value(&json_str, encoder)
    }
}

impl Decode for Cred {
    #[inline(always)]
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let json_str = <String as Decode>::decode_value(decoder)?;
        serde_json::from_str::<Cred>(&json_str).map_err(|_| DecodeError::InvalidCustomData(0))
    }
}

impl Describe for Cred {
    #[inline(always)]
    fn describe() -> Type {
        <String as Describe>::describe()
    }
}

pub type CredMap = HashMap<String, Cred>;

pub struct RPContext<'a> {
    creds: &'a mut CredMap,
    reg_contexts: HashMap<String, PublicKeyCredentialCreationOptions>,
    sign_contexts: HashMap<String, PublicKeyCredentialRequestOptions>,
}

impl<'a> RPContext<'a> {
    pub fn new(creds: &'a mut CredMap) -> Self {
        RPContext {
            creds: creds,
            reg_contexts: HashMap::new(), // ephemeral, not stored on ledger
            sign_contexts: HashMap::new(), // ephemeral, not stored on ledger
        }
    }
    pub fn start_register(&mut self, rp: String, user_id: String, user_name: String, user_display_name: String, challenge: String) {
        let builder = CredentialCreationBuilder::new()
            .challenge(challenge)
            .user(user_id.clone(), user_name, user_display_name, None)
            .rp(rp.clone(), None, Some(rp)) // the rp id is related to the origin see https://w3c.github.io/webauthn/#rp-id
            .build();

        match builder {
            Ok(pubkey) => {
                scrypto::debug!("WebAuthn new registration PublicKeyCredentialCreationOptions: {}", &json!({ "publicKey": pubkey }));
                self.reg_contexts.insert(user_id, pubkey.clone());
            }
            Err(e) => {
                panic!("Error creating PublicKeyCredentialCreationOptions to register user {} error: {:?}", user_id, e);
            }
        }
    }
    pub fn complete_register(&mut self, origin: &str, user_id: &str, response: &str) {
        let value = serde_json::from_str::<PublicKeyCredential>(response);
        if let Ok(cred) = value {
            scrypto::debug!("trying to complete registration with: {:?}", cred);
            if let Some(context) = self.reg_contexts.get(user_id) {
                let mut verifier =
                    CredentialCreationVerifier::new(cred.clone(), context.clone(), origin);
                let verify_result = verifier.verify();
                if let Ok(result) = verify_result {
                    scrypto::debug!("complete register for cred with id: {}", cred.id);
                    self.creds
                        .insert(cred.id, Cred { public_key: result.public_key, sign_count: result.sign_count });
                } else {
                    panic!("Credential verifiation failed for user: {} with error {:?}", user_id, verify_result.err());
                }
            } else {
                panic!("Registration user not found: {}", user_id)
            }
        } else {
            panic!("Bad PublicKeyCredential Format (could not deserialize JSON) for user: {}", user_id)
        }
    }

    pub fn get_sign_request(&mut self, rp: String, user_id: String, challenge: String) -> String {
        let mut builder = CredentialRequestBuilder::new()
            .rp(rp)
            .challenge(challenge);
        for (cred, _) in self.creds.iter() {
            builder = builder.allow_credential(cred.clone());
        }
        match builder.build() {
            Ok(pubkey) => {
                self.sign_contexts.insert(user_id, pubkey.clone());
                let cred_request = json!({ "publicKey": pubkey });
                return cred_request.to_string();
            }
            Err(e) => {
                panic!("MFA Failure: Could not create sign request: {}", e)
            }
        }
    }

    pub fn check_sign_response(&mut self, user_id: &str, response: &str, origin: &str) {
        let value = serde_json::from_str::<PublicKeyCredential>(response);
        let result = if let Ok(cred) = value {
            if let Some(context) = self.sign_contexts.get(user_id) {
                scrypto::debug!("look for cred with id: {}", cred.id);
                if let Some(Cred {public_key, sign_count}) = self.creds.get(&cred.id) {
                    let mut verifier = CredentialRequestVerifier::new(
                        cred.clone(),
                        public_key.clone(),
                        context.clone(),
                        origin,
                        user_id,
                        *sign_count,
                    );
                    match verifier.verify() {
                        Ok(res) => Ok((public_key.clone(), res.sign_count)),

                        Err(e) => Err(e),
                    }
                } else {
                    Err(CredentialError(CredE::Other("Credential not found".to_string())))
                }
            } else {
                Err(CredentialError(CredE::Other("Context not found".to_string())))
            }
        } else {
            Err(CredentialError(CredE::Other(
                "Public key credential could not be parsed".to_string(),
            )))
        };

        match result {
            Ok((public_key, sign_count)) => {
                self.creds.insert(user_id.to_owned(), Cred { public_key, sign_count } );
            }

            Err(e) => {
                panic!("MFA Failure: could not validate signature: {}", e)
            }
        }
    }

}

#[cfg(feature = "testing")]
pub(crate) mod testing {
    use super::*;

    fn do_register() -> CredMap {
        let rp = "localhost".to_owned(); // needs to be domain of origin?
        let origin = "http://localhost:8080".to_owned();
        //let origin = "localhost".to_owned();
        let user_id = "0i99MIQfGVSKuWZVuZ4uoZnvO6ZiaYV8c6eWmHENDYM".to_owned(); //"ME";
        let user_name = "user_name".to_owned();
        let user_display_name = "user_displayName".to_owned();
        let challenge = "-sgIuRggalY2E-c-mLsqIuMwNI9lva06XonhGPVTdEE".to_owned();

        let response = r#"{"id":"0i99MIQfGVSKuWZVuZ4uoZnvO6ZiaYV8c6eWmHENDYM","response":{"attestationObject":"o2NmbXRmcGFja2VkZ2F0dFN0bXSjY2FsZyZjc2lnWEcwRQIgL2u+F31GemAIm+yubtYN4FcHOobBzI6gzci+mGsmpwoCIQC3T+D3NvUZdnS3NnW7QrjQEnWzIYSwvb/eyYSOYz2sEWN4NWOBWQHeMIIB2jCCAX2gAwIBAgIBATANBgkqhkiG9w0BAQsFADBgMQswCQYDVQQGEwJVUzERMA8GA1UECgwIQ2hyb21pdW0xIjAgBgNVBAsMGUF1dGhlbnRpY2F0b3IgQXR0ZXN0YXRpb24xGjAYBgNVBAMMEUJhdGNoIENlcnRpZmljYXRlMB4XDTE3MDcxNDAyNDAwMFoXDTQyMDUxNjIyNTM0MVowYDELMAkGA1UEBhMCVVMxETAPBgNVBAoMCENocm9taXVtMSIwIAYDVQQLDBlBdXRoZW50aWNhdG9yIEF0dGVzdGF0aW9uMRowGAYDVQQDDBFCYXRjaCBDZXJ0aWZpY2F0ZTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABI1hfmXJUI5kvMVnOsgqZ5naPBRGaCwljEY//99Y39L6Pmw3i1PXlcSk3/tBme3Xhi8jq68CA7S4kRugVpmU4QGjJTAjMBMGCysGAQQBguUcAgEBBAQDAgUgMAwGA1UdEwEB/wQCMAAwDQYJKoZIhvcNAQELBQADSAAwRQIgB5BNIIoB+FUjTezB8P7zcmT6vjb5ip3J+tD0PpXiN7cCIQCBr/1ysP8G325/xJfv6XYlsjOEXFkV4tZ1lFjt0izgiWhhdXRoRGF0YVikSZYN5YgOjGh0NBcPZHZgW4/krrmihjLHmVzzuoMdl2NFAAAAAQECAwQFBgcIAQIDBAUGBwgAINIvfTCEHxlUirlmVbmeLqGZ7zumYmmFfHOnlphxDQ2DpQECAyYgASFYIALBcmd1ptldpytKngB8vYy2PD0fx9OnoVn0RE2CmrWCIlggUaLtszoKSVN3ECQnX/pJbWaKboXSLaSRjGLd5HFOcrM=","clientDataJSON":"eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiLXNnSXVSZ2dhbFkyRS1jLW1Mc3FJdU13Tkk5bHZhMDZYb25oR1BWVGRFRSIsIm9yaWdpbiI6Imh0dHA6Ly9sb2NhbGhvc3Q6ODA4MCIsImNyb3NzT3JpZ2luIjpmYWxzZX0="}}"#;

        let mut credmap: CredMap = Default::default();
        let mut ctx = RPContext::new(&mut credmap);
        ctx.start_register(rp, user_id.clone(), user_name, user_display_name, challenge);
        ctx.complete_register(&origin, &user_id, &response);

        credmap
    }

    pub fn test_register() {
        do_register(); // ignore return
        // success if it doesn't panic
    }

    // TODO make register test that fails

    pub fn test_validation() {
        let mut credmap = do_register();
        let mut ctx = RPContext::new(&mut credmap);

        // same as in register
        let rp = "localhost".to_owned(); // needs to be domain of origin?
        let origin = "http://localhost:8080".to_owned();
        let user_id = "0i99MIQfGVSKuWZVuZ4uoZnvO6ZiaYV8c6eWmHENDYM".to_owned(); //"ME";

        // test data
        let challenge = "Placeholder";
        let b64_challenge = base64::encode(challenge);
        let challenge = b64_challenge.trim_end_matches("="); // challenge must be base64 encoded when stored in the ctx so it is compared properly to auth result but needs to be decoded for call to credential.get on the frontend (that could be a bug in webpki but probably not)
        let response = "{\"id\":\"0i99MIQfGVSKuWZVuZ4uoZnvO6ZiaYV8c6eWmHENDYM\",\"response\":{\"authenticatorData\":\"SZYN5YgOjGh0NBcPZHZgW4/krrmihjLHmVzzuoMdl2MFAAAAEQ==\",\"signature\":\"MEUCIQDqgcBDRgHx9trhh6dqdYLl5+/QHplYdeBAYs7tkUGBdAIgD3Pxq0zQ822riNF+vBfbD+DFwHNGLgL7xwE+XFeyJKs=\",\"userHandle\":null,\"clientDataJSON\":\"eyJ0eXBlIjoid2ViYXV0aG4uZ2V0IiwiY2hhbGxlbmdlIjoiVUd4aFkyVm9iMnhrWlhJIiwib3JpZ2luIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwIiwiY3Jvc3NPcmlnaW4iOmZhbHNlLCJvdGhlcl9rZXlzX2Nhbl9iZV9hZGRlZF9oZXJlIjoiZG8gbm90IGNvbXBhcmUgY2xpZW50RGF0YUpTT04gYWdhaW5zdCBhIHRlbXBsYXRlLiBTZWUgaHR0cHM6Ly9nb28uZ2wveWFiUGV4In0=\"}}";

        let _request = ctx.get_sign_request(rp, user_id.clone(), challenge.to_owned());
        ctx.check_sign_response(&user_id, response, &origin);
        // success if it doesn't panic
    }

}