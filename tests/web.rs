//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use shuuro_wasm::position::ShuuroPosition;
use shuuro_wasm::selection::ShuuroShop;
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
fn standard_placement() {
    let mut pos = ShuuroPosition::new("standard");
    let sfen = "8/8/3L04/1L06/8/L01L05/8/8 w kqnKQR 0";
    pos.set_sfen(sfen);
    let m = pos.place_moves('K');
}

#[wasm_bindgen_test]
fn placement_hand() {
    let cases = [
        // (
        //     "shuuro",
        //     "5KQ5/57/57/7L04/2L09/5L0L05/57/L056/4L07/57/55L01/6k1Ln3 w 2qr6b2n15pQ3R4B3N9P 4",
        //     "qqrbbbbbbnnpppppppppppppppQRRRBBBBNNNPPPPPPPPP",
        // ),
        (
            "standard",
            "4K3/8/8/1L01L04/4L03/6L01/8/8 b RBNNNPPPPPPPPPPPPkqrbbnnp 1",
            "kqrbbnnpRBNNNPPPPPPPPPPPP",
        ),
    ];
    for case in cases {
        let mut pos = ShuuroPosition::new(case.0);
        pos.set_sfen(case.1);
        let hand = pos.count_hand_pieces();
        assert_eq!(hand, String::from(case.2));
    }
}
