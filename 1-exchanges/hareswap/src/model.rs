//! This module holds the "application layer" data types including `PartialOrder` `MatchedOrder` and `SignedOrder`
use sbor::{Decode, Describe, Encode, TypeId};
use scrypto::prelude::*;

use super::requirement::{BucketContents, BucketRequirement};

/// A description of a function or method call (optionally bound with arguments) exactly as in a normal transaction Instruction
///
/// This is included in the MatchedOrder to give the signer flexability in the on-ledger operations which are needed to fullfill
/// their side of the order.
///
/// NOTE: In this prototype, only the minimal calls are implemented, but one
/// could imagine an entire transaction interpreter blueprint that knows how to
/// do more than `call_function` and `call_method` so this could become more
/// than this subset of a Transaction as it exists now, and instead
/// represent an entire transaction (or at least more parts) all composed off-ledger
/// by the signer in a similar way to how the transaction manifest is created off-ledger.
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub enum Callback {
    /// Calls a blueprint function.
    CallFunction {
        package_address: Address,
        blueprint_name: String,
        function: String,
        args: Vec<Vec<u8>>,
    },

    /// Calls a component method.
    CallMethod {
        component_address: Address,
        method: String,
        args: Vec<Vec<u8>>,
    },
}

/// The main parts of the "request" in the request-for-quote (RFQ) coming from the sender
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
pub struct PartialOrder {
    /// description of the assets to be provided by the signer
    pub maker_requirement: BucketRequirement,
    /// the resource that will be provided by the sender (but not the amount, that's the "quote" we want)
    pub taker_resource: ResourceDef,
    /// description of the assets (badges) which the signer bake into the SignedOrder so that only the sender can execute the order (or get an order token)
    pub taker_auth: BucketRequirement,
}

/// the main to-be-signed parts of the response to the RFQ supplied by the signer
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe, NonFungibleData)]
pub struct MatchedOrder {
    /// The original PartialOrder in the RFQ
    pub partial_order: PartialOrder,
    /// The amount the signer has decided the taker needs to provide
    pub taker_contents: BucketContents,
    /// A Callback which decides how the the signer's assets are to be obtained to settle the order
    pub maker_callback: Callback,
}

/// mod signed_order contains only the SignedOrder struct
///
/// in a seperate module where we explicitly avoid importing NonFungibleData
/// This is to work around a conflict when deriving sbor::Decode.
/// Otherwise there are multiple `decode` functions on MatchedOrder
/// one for the sbor::Decode trait and one for the NonFungibleData trait:
///    multiple `decode` found
///
///    help: disambiguate the associated function for candidate #1: `<&mut sbor::Decoder<'_> as sbor::Decode>::`
///    help: disambiguate the associated function for candidate #2: `<&[u8] as scrypto::prelude::NonFungibleData>::`
mod signed_order {
    use super::MatchedOrder;
    use super::{NonFungibleKey, ResourceDef};
    use sbor::{Decode, Describe, Encode, TypeId};

    /// Combines a MatchedOrder with Voucher metadata and signature covering the
    /// Voucher which can be creatd from them
    #[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq, Describe)]
    pub struct SignedOrder {
        pub order: MatchedOrder,
        pub voucher_resource: ResourceDef,
        pub voucher_key: NonFungibleKey,
        pub signature: Vec<u8>,
    }
}
pub use signed_order::SignedOrder;
