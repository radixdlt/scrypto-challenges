mod utils;

use radix_engine::model::Instruction;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn compile(manifest: &str) -> Result<Vec<u8>, String> {
    utils::set_panic_hook();
    let transaction = transaction_manifest::compile(manifest).map_err(|e| format!("{:?}", e))?;
    Ok(scrypto::buffer::scrypto_encode(&transaction))
}

#[wasm_bindgen]
pub fn compile_with_nonce(manifest: &str, nonce: u64) -> Result<Vec<u8>, String> {
    utils::set_panic_hook();
    let mut transaction =
        transaction_manifest::compile(manifest).map_err(|e| format!("{:?}", e))?;
    transaction.instructions.push(Instruction::Nonce { nonce });
    Ok(scrypto::buffer::scrypto_encode(&transaction))
}
