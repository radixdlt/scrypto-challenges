// build a callback contract
use radix_engine::model::{Instruction, Transaction as RETx};
use sbor::describe::Type;
use sbor::{Decode, DecodeError, Decoder, Describe, Encode, Encoder, TypeId};
use scrypto::prelude::*;

#[derive(Describe)]
pub struct PassThru(Vec<u8>);

impl TypeId for PassThru {
    fn type_id() -> u8 {
        panic!("unimplemented");
    }
}
impl Encode for PassThru {
    #[inline]
    fn encode(&self, encoder: &mut Encoder) { encoder.write_slice(&self.0); }

    fn encode_value(&self, _encoder: &mut Encoder) {}
}
impl Decode for PassThru {
    #[inline]
    fn decode(_decoder: &mut Decoder) -> Result<Self, DecodeError> {
        panic!("unimplemented");
    }

    fn decode_value(_decoder: &mut Decoder) -> Result<Self, DecodeError> {
        panic!("unimplemented");
    }
}

#[derive(Encode, Decode, TypeId)]
pub struct Tx(RETx);

impl Describe for Tx {
    fn describe() -> Type {
        panic!("unimplemented");
        // Type::Struct {
        //     name: "Tx".to_owned(),
        //     fields: Fields::Unit
        // }
    }
}


#[derive(NonFungibleData)]
pub struct Transaction {
    tx: Tx,
}

blueprint! {
    struct Interpreter {}

    impl Interpreter {
        pub fn run(txs: Bucket) -> PassThru {
            assert_eq!(txs.amount(), Decimal::one(), "Interpreter::run: exactly 1 tx is supported");
            let txs = txs.get_non_fungibles::<Transaction>();
            for entry in &txs {
                let transaction = entry.data();
                let tx = transaction.tx.0;
                info!("tx is: {:?}", tx);
                assert_eq!(tx.instructions.len(), 1, "Interpreter::run: exactly 1 instruction is supported"); // FUTURE: handle all/most transaction instructions with emulated workbench
                for instruction in tx.instructions {
                    let result: Vec<u8> = match instruction {
                        Instruction::CallFunction {
                            package_address,
                            blueprint_name,
                            function,
                            args
                        } => {
                            call_function(package_address, &blueprint_name, &function, args)
                        },
                        Instruction::CallMethod {
                            component_address,
                            method,
                            args
                        } => {
                            call_method(component_address, &method, args)
                        },
                        _ => { panic!("unimplemented"); } // FUTURE: handle all/most transaction instructions with emulated workbench
                    };
                    return PassThru(result);
                }
            }
            panic!("unreachable");
        }
    }

}
