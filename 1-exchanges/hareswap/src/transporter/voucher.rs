use scrypto::prelude::*;
use sbor::{Decode, Decoder, DecodeError, Describe, Encode, describe::Type, TypeId};

use super::decoder::*;

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
            mutable_data: self.mutable_data()
        }
    }
}
impl<T: NonFungibleData> IsPassThruNFD for T {}

impl NonFungibleData for PassThruNFD {
    /// Decodes `Self` from the serialized immutable and mutable parts.
    fn decode(immutable_data: &[u8], mutable_data: &[u8]) -> Result<Self, DecodeError> {
        Ok(PassThruNFD {
            immutable_data: immutable_data.into(),
            mutable_data: mutable_data.into()
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
        panic!("unimplemented");
    }

    /// Returns the schema of the mutable data.
    fn mutable_data_schema() -> Type {
        panic!("unimplemented");
    }
}

// make the Voucher not Decode-able so it can't be (accidentally) created other than from SealedVoucher (with sig check)

#[derive(PartialEq, Eq, Debug, TypeId, Describe, Encode)]
pub struct Voucher {
    pub resource_def: ResourceDef,
    pub key: Option<NonFungibleKey>,
    pub nfd: PassThruNFD
}

impl PrivateDecode for Voucher {
    // based on the derive Decode implementation  in sbor-derive:src/decode.rs
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let index = decoder.read_u8()?;
        if index != ::sbor::type_id::FIELDS_TYPE_NAMED {
            return Err(::sbor::DecodeError::InvalidIndex(index));
        }
        decoder.check_len(3)?;
        let resource_def  = ResourceDef::decode(decoder)?;
        let key  = Option::<NonFungibleKey>::decode(decoder)?;
        let nfd  = <PassThruNFD as sbor::Decode>::decode(decoder)?; // cannot derive Decode for Voucher because the decode method is implemented for both  NonFungibleData and Decode traits.  Disambiguate here
        decoder.check_end()?;
        Ok(Voucher {
            resource_def,
            key,
            nfd
        })
    }
}

impl Voucher {
    pub fn redeem(self, required_resource_def: &ResourceDef, required_key: Option<NonFungibleKey>, auth: BucketRef) -> Bucket {
        let Voucher {
            mut resource_def,
            key,
            nfd
        } = self;
        assert_eq!(resource_def, *required_resource_def, "Voucher::redeem: resource requirement not met");
        // test key against required key if both exist, otherwise use whichever is given.  panic if both are None
        let key = match required_key {
            None => {key.unwrap()}
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
}