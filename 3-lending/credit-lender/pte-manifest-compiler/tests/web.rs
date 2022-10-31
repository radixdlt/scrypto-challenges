//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use pte_manifest_compiler::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_compile() {
    assert_eq!(
        compile("CLEAR_AUTH_ZONE;"),
        Ok(vec![16, 1, 0, 0, 0, 48, 17, 1, 0, 0, 0, 9, 0, 0, 0, 0])
    );
}

#[wasm_bindgen_test]
fn test_compile_with_nonce() {
    assert_eq!(
        compile_with_nonce("CLEAR_AUTH_ZONE;", 1),
        Ok(vec![
            16, 1, 0, 0, 0, 48, 17, 2, 0, 0, 0, 9, 0, 0, 0, 0, 20, 1, 0, 0, 0, 10, 1, 0, 0, 0, 0,
            0, 0, 0
        ])
    );
}
