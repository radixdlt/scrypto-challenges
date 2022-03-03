use sbor::{Decoder, DecodeError, TypeId};

/// A data structure that can be decoded from a byte array using SBOR (but not automatically derived by blueprint! macro)
pub trait PrivateDecode: Sized + TypeId {
    #[inline]
    fn decode(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.check_type(Self::type_id())?;
        Self::decode_value(decoder)
    }

    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError>;
}
/// duplicate of decode_with_type with my private trait instead
pub fn private_decode_with_type<T: PrivateDecode>(buf: &[u8]) -> Result<T, DecodeError> {
    let mut dec = Decoder::with_type(buf);
    let v = T::decode(&mut dec)?;
    dec.check_end()?;
    Ok(v)
}
