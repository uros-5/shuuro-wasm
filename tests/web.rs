//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use shuuro_wasm::shop::ShuuroShop;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn fairy_shop() {
    let mut shop = ShuuroShop::new();
    shop.change_variant(String::from("shuuroFairy"));
    shop.buy(String::from("+G"));
    shop.buy(String::from("+G"));
    shop.buy(String::from("+G"));
    assert_eq!(shop.get_credit('w'), 660);
}
