use hex;

use sbor::{describe::Type, Decode, DecodeError, Decoder, Describe, Encode, TypeId};
use scrypto::prelude::*;

use super::decoder::*;
use super::authentication::*;

/// The PassThruNFD implementation and related traits enables a NonFungibleData
/// property to 
#[derive(PartialEq, Eq, Debug, TypeId, Encode, Decode, Describe)]
pub struct PassThruNFD {
    immutable_data: Vec<u8>,
    mutable_data: Vec<u8>,
}

// note: can't implement From, this works just as well with a specific method name
pub trait IsPassThruNFD: NonFungibleData + Sized {
    fn as_passthru(&self) -> PassThruNFD {
        PassThruNFD {
            immutable_data: self.immutable_data(),
            mutable_data: self.mutable_data(),
        }
    }
}
impl<T: NonFungibleData> IsPassThruNFD for T {}

impl NonFungibleData for PassThruNFD {
    /// Decodes `Self` from the serialized immutable and mutable parts.
    fn decode(immutable_data: &[u8], mutable_data: &[u8]) -> Result<Self, DecodeError> {
        Ok(PassThruNFD {
            immutable_data: immutable_data.into(),
            mutable_data: mutable_data.into(),
        })
    }

    /// Returns the serialization of the immutable data part.
    fn immutable_data(&self) -> Vec<u8> {
        self.immutable_data.clone() // NOTE: could optimize to avoid this clone using RefCell<Option<...>> knowing this is only called from mint_non_fungible called from to_bucket which consumes the Voucher
    }

    /// Returns the serialization of the mutable data part.
    fn mutable_data(&self) -> Vec<u8> {
        self.mutable_data.clone() // NOTE: could optimize to avoid this clone using RefCell<Option<...>> knowing this is only called from mint_non_fungible called from to_bucket which consumes the Voucher
    }

    /// Returns the schema of the immutable data.
    fn immutable_data_schema() -> Type {
        panic!("unimplemented"); // not needed to decode a PassThruNFD
    }

    /// Returns the schema of the mutable data.
    fn mutable_data_schema() -> Type {
        panic!("unimplemented"); // not needed to decode a PassThruNFD
    }
}

// make the Voucher not Decode-able so it can't be (accidentally) created other than from SealedVoucher (with sig check)

#[derive(PartialEq, Eq, Debug, TypeId, Describe, Encode)]
pub struct Voucher {
    resource_def: ResourceDef,
    key: Option<NonFungibleKey>,
    nfd: PassThruNFD,
}

impl PrivateDecode for Voucher {
    // based on the derive Decode implementation  in sbor-derive:src/decode.rs
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let index = decoder.read_u8()?;
        if index != ::sbor::type_id::FIELDS_TYPE_NAMED {
            return Err(::sbor::DecodeError::InvalidIndex(index));
        }
        decoder.check_len(3)?;
        let resource_def = ResourceDef::decode(decoder)?;
        let key = Option::<NonFungibleKey>::decode(decoder)?;
        let nfd = <PassThruNFD as sbor::Decode>::decode(decoder)?; // cannot derive Decode for Voucher because the decode method is implemented for both  NonFungibleData and Decode traits.  Disambiguate here
        decoder.check_end()?;
        Ok(Voucher { resource_def, key, nfd })
    }
}

impl Voucher {
    pub fn redeem(
        self,
        required_resource_def: &ResourceDef,
        required_key: Option<NonFungibleKey>,
        auth: BucketRef,
    ) -> Bucket {
        let Voucher {
            mut resource_def,
            key,
            nfd,
        } = self;
        assert_eq!(
            resource_def, *required_resource_def,
            "Voucher::redeem: resource requirement not met"
        );
        // test key against required key if both exist, otherwise use whichever is given.  panic if both are None
        let key = match required_key {
            None => key.unwrap(),
            Some(required_key) => {
                if let Some(voucher_key) = key {
                    assert_eq!(voucher_key, required_key, "Voucher::redeem: key requirement not met");
                }
                required_key
            }
        };
        // finally mint
        resource_def.mint_non_fungible(&key, nfd, auth)
    }


    /// create a Voucher from any NonFungibleData with the the included metadata
    pub fn from_nfd<T: NonFungibleData>(resource_def: ResourceDef, key: Option<NonFungibleKey>, nfd: T) -> Voucher {
        Voucher {
            resource_def,
            key,
            nfd: nfd.as_passthru(), // calling as_passthru is the "trick" and is an implementation detail for how Vouchers work which can be ignored for users.
        }
    }

    /// create a SealedVoucher from this Voucher and an opaque signature
    pub fn to_sealed(&self, signature: Vec<u8>) -> SealedVoucher {
        SealedVoucher {
            serialized: scrypto_encode(self),
            signature
        }
    }
}

/// An opaque data structure which can (only) be converted back into a Voucher by validating the digital signature
#[derive(TypeId, Describe, Encode, Decode)]
pub struct SealedVoucher {
    serialized: Vec<u8>,
    signature: Vec<u8>,
}

impl SealedVoucher {
    /// Converts a SealedVoucher back to a Voucher by verifying the signature against public_key
    pub fn unseal(&self, public_key: &EcdsaPublicKey) -> Voucher {
        debug!("SealedVoucher::unseal: serialized: {}", hex::encode(&self.serialized));
        debug!("SealedVoucher::unseal:  signature: {}", hex::encode(&self.signature));
        verify_or_panic(public_key, &self.serialized, &self.signature); // NOTE: panics on failure
        private_decode_with_type(&self.serialized).unwrap()
    }
}
