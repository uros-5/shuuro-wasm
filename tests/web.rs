//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use shuuro_wasm::position::ShuuroPosition;
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

#[wasm_bindgen_test]
fn standardPlacement() {
    let mut pos = ShuuroPosition::new("standard");
    let sfen = "8/8/3L04/1L06/8/L01L05/8/8 w kqnKQR 0";
    pos.set_sfen(sfen);
    let m = pos.place_moves('K');
}
