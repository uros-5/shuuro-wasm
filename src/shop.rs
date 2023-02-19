use js_sys::{Array, Uint8Array};

use shuuro::{
    piece_type::PieceTypeIter, shuuro12::square12::Square12, Color, Move, Piece, PieceType,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Class for ShuuroShop
/// Generics in Shop are not that important.
#[wasm_bindgen]
#[derive(Default)]
pub struct ShuuroShop {
    shuuro: shuuro::Shop<Square12>,
}

#[wasm_bindgen]
impl ShuuroShop {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ShuuroShop {
            shuuro: shuuro::Shop::default(),
        }
    }

    #[wasm_bindgen]
    pub fn change_variant(&mut self, variant: String) {
        self.shuuro.change_variant(&variant);
    }

    #[wasm_bindgen]
    pub fn get_variant(&self) -> String {
        self.shuuro.variant().to_string()
    }

    /// Buying piece. game_move is in this format `+P`. Returns Uint8Array
    #[wasm_bindgen]
    pub fn buy(&mut self, game_move: String) -> Uint8Array {
        if let Some(game_move) = Move::from_sfen(game_move.as_str()) {
            if let Move::Buy { piece } = game_move {
                self.shuuro.play(game_move);
                return self.js_shop_items(&piece.color);
            }
        }
        Uint8Array::new_with_length(9)
    }

    /// Confirm players hand. s is color. It can be 'w' or 'b'.
    #[wasm_bindgen]
    pub fn confirm(&mut self, s: char) {
        let color = Color::from_char(s);
        if let Some(c) = color {
            if let Color::NoColor = c {
            } else {
                self.shuuro.confirm(c);
            }
        }
    }

    /// Get credit for selected player.
    #[wasm_bindgen]
    pub fn get_credit(&self, s: char) -> i32 {
        let color = Color::from_char(s);
        match color {
            Some(c) => match c {
                Color::NoColor => 800,
                _ => self.shuuro.credit(c),
            },

            None => 800,
        }
    }

    /// Check if selected player if confirmed.
    #[wasm_bindgen]
    pub fn is_confirmed(&self, s: char) -> bool {
        let color = Color::from_char(s);
        match color {
            Some(c) => match c {
                Color::NoColor => false,
                _ => self.shuuro.is_confirmed(c),
            },

            None => false,
        }
    }

    /// Count all items for selected player.
    #[wasm_bindgen]
    pub fn shop_items(&self, s: char) -> Uint8Array {
        let color = Color::from_char(s);
        if let Some(c) = color {
            if let Color::NoColor = c {
                return Uint8Array::new_with_length(9);
            } else {
                return self.js_shop_items(&c);
            }
        }
        Uint8Array::new_with_length(8)
    }

    fn js_shop_items(&self, color: &Color) -> Uint8Array {
        let array = Uint8Array::new_with_length(9);
        let mut current_state: [u8; 9] = [1, 0, 0, 0, 0, 0, 0, 0, 0];
        let iterator = PieceTypeIter::default();
        for i in iterator {
            if !self.shuuro.variant().can_buy(&i) {
                continue;
            } else if i == PieceType::King {
                array.set_index(0, current_state[0]);
            } else if i == PieceType::Plinth {
                continue;
            }
            if i.index() != 0 || i.index() != 9 {
                let piece = Piece {
                    piece_type: i,
                    color: *color,
                };
                let index = i.index();
                let current = self.shuuro.get(piece);
                current_state[index] = current;
                array.set_index(index as u32, current);
            }
        }
        array
    }

    fn _js_shop_index(&self, index: usize) -> usize {
        match index {
            6 => 2,
            7 => 3,
            _ => index,
        }
    }

    /// Get counter for selected piece.
    #[wasm_bindgen]
    pub fn get_piece(&self, s: char) -> u8 {
        let piece = shuuro::Piece::from_sfen(s);
        match piece {
            Some(p) => self.shuuro.get(p),
            None => 0,
        }
    }

    /// Set hand for all players.
    #[wasm_bindgen]
    pub fn set_hand(&mut self, hand: &str) {
        self.shuuro.set_hand(hand);
    }

    /// All moves for player in sfen format: [{"+k", 0}, ...]
    #[wasm_bindgen]
    pub fn history(&self) -> Array {
        let ar = Array::new();
        let history = self.shuuro.get_sfen_history(&Color::NoColor);
        for m in history {
            let t = Array::new();
            t.push(&JsValue::from_str(m.0.as_str()));
            t.push(&JsValue::from(m.1));
            ar.push(&JsValue::from(t));
        }
        ar
    }
}
