//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wekolo_helper_rust;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

// #[wasm_bindgen_test]
// fn test_gallery() {
//     assert_eq!(1, wekolo_helper_rust::get_formatted_items());
// }
