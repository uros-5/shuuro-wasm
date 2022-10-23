mod utils;

use std::fmt::format;

use js_sys::{Array, Map, Uint8Array};
use serde::{Deserialize, Serialize};
use shuuro::piece_type::Variant;
use shuuro::{self, piece_type::PieceTypeIter, Color, Move, Piece, PieceType, Position};
use shuuro::{init, Square};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Class for ShuuroShop
#[wasm_bindgen]
pub struct ShuuroShop {
    shuuro: shuuro::Shop,
    variant: Variant,
}

#[wasm_bindgen]
impl ShuuroShop {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ShuuroShop {
            shuuro: shuuro::Shop::default(),
            variant: Variant::Normal,
        }
    }

    #[wasm_bindgen]
    pub fn change_variant(&mut self, variant: String) {
        let v = Variant::from(&variant);
        self.shuuro.change_variant();
        self.variant = v.clone();
    }

    /// Buying piece. game_move is in this format `+P`. Returns Uint8Array
    #[wasm_bindgen]
    pub fn buy(&mut self, game_move: String) -> Uint8Array {
        if let Some(game_move) = Move::from_sfen(game_move.as_str()) {
            match game_move {
                Move::Buy { piece } => {
                    self.shuuro.play(game_move);
                    return self.js_shop_items(&piece.color);
                }
                _ => (),
            }
        }
        Uint8Array::new_with_length(7)
    }

    /// Confirm players hand. s is color. It can be 'w' or 'b'.
    #[wasm_bindgen]
    pub fn confirm(&mut self, s: char) {
        let color = Color::from_char(s);
        match color {
            Some(c) => match c {
                Color::NoColor => (),
                _ => {
                    self.shuuro.confirm(c);
                }
            },
            None => (),
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
        match color {
            Some(c) => match c {
                Color::NoColor => {
                    return Uint8Array::new_with_length(7);
                }
                _ => {
                    return self.js_shop_items(&c);
                }
            },
            None => (),
        }
        Uint8Array::new_with_length(7)
    }

    fn js_shop_items(&self, color: &Color) -> Uint8Array {
        let array = Uint8Array::new_with_length(7);
        let mut current_state: [u8; 7] = [0, 0, 0, 0, 0, 0, 0];
        let iterator = PieceTypeIter::new();
        for i in iterator {
            if self.variant.wrong(i.index()) {
                continue;
            } else if i == PieceType::King {
                continue;
            } else if i == PieceType::Plinth {
                continue;
            }
            if i.index() != 0 || i.index() != 8 {
                let piece = Piece {
                    piece_type: i,
                    color: *color,
                };
                let index = self.js_shop_index(i.index());
                let current = self.shuuro.get(piece);
                current_state[index] = current;
                array.set_index(index as u32, current);
            }
        }
        array
    }

    fn js_shop_index(&self, index: usize) -> usize {
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
            Some(p) => {
                return self.shuuro.get(p);
            }
            None => return 0,
        }
    }

    /// Set hand for all players.
    #[wasm_bindgen]
    pub fn set_hand(&mut self, hand: &str) {
        self.shuuro.set_hand(&hand);
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

#[wasm_bindgen]
pub struct ShuuroPosition {
    shuuro: Position,
}

#[wasm_bindgen]
impl ShuuroPosition {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        init();
        ShuuroPosition {
            shuuro: Position::default(),
        }
    }
    // Main functions.

    /// Change game variant
    #[wasm_bindgen]
    pub fn change_variant(&mut self) {
        self.shuuro.update_variant();
    }

    /// Set hand for pocket.
    #[wasm_bindgen]
    pub fn set_hand(&mut self, s: &str) {
        self.shuuro.set_hand(s);
    }

    /// Set sfen.
    #[wasm_bindgen]
    pub fn set_sfen(&mut self, s: &str) {
        if let Err(_e) = self.shuuro.set_sfen(s) {}
    }

    /// Get sfen for current position.
    #[wasm_bindgen]
    pub fn generate_sfen(&self) -> String {
        self.shuuro.generate_sfen()
    }

    /// Get side to move.
    #[wasm_bindgen]
    pub fn side_to_move(&self) -> String {
        self.shuuro.side_to_move().to_string()
    }

    /// All plinths on board.
    #[wasm_bindgen]
    pub fn map_plinths(&self) -> Map {
        let list = Map::new();
        let bb = self.shuuro.player_bb(Color::NoColor);
        for i in bb.clone() {
            let example = P {
                role: String::from("l-piece"),
                color: String::from("white"),
            };
            let sq = i.to_string();

            list.set(
                &JsValue::from_str(sq.as_str()),
                &JsValue::from_serde(&example).unwrap(),
            );
        }

        list
    }

    /// All pieces on board.
    #[wasm_bindgen]
    pub fn map_pieces(&self) -> Map {
        let list = Map::new();
        let colors = [Color::White, Color::Black];
        for i in colors {
            let bb = self.shuuro.player_bb(i);
            let color = self.get_color(&i.to_string());
            for sq in bb.clone() {
                let piece = self.shuuro.piece_at(sq);
                if let Some(piece) = piece {
                    let sq = sq.to_string();
                    let p = P {
                        role: format!(
                            "{}-piece",
                            piece.to_string().as_str().to_lowercase().as_str()
                        ),
                        color: String::from(color),
                    };

                    list.set(
                        &JsValue::from_str(sq.as_str()),
                        &JsValue::from_serde(&p).unwrap(),
                    );
                }
            }
        }

        list
    }

    /// Get piece count.
    #[wasm_bindgen]
    pub fn pieces_count(&self) -> u32 {
        let mut sum = self.shuuro.player_bb(Color::Black).count();
        sum += self.shuuro.player_bb(Color::White).count();
        sum
    }
    /// Get last move.
    #[wasm_bindgen]
    pub fn last_move(&self) -> String {
        self.shuuro.get_sfen_history().last().unwrap().0.clone()
    }

    /// Returns if side_to_move is in check.
    #[wasm_bindgen]
    pub fn is_check(&self) -> bool {
        self.shuuro.in_check(self.shuuro.side_to_move())
    }

    fn get_color(&self, c: &String) -> &str {
        if c == "w" {
            return "white";
        } else if c == "b" {
            return "black";
        }
        "none"
    }

    // Deploy part

    /// Squares where piece can be placed.
    pub fn place_moves(&mut self, piece: char) -> Map {
        let list = Map::new();
        if let Some(p) = Piece::from_sfen(piece) {
            let bb = self.shuuro.empty_squares(p);
            let moves = Array::new();
            for i in bb {
                moves.push(&JsValue::from_str(i.to_string().as_str()));
            }
            let key = format!("{}@", piece.to_uppercase());
            let key = JsValue::from_str(key.as_str());
            let value = JsValue::from(moves);
            list.set(&key, &value);
        }
        list
    }

    /// Count how many pieces are left in hand.
    #[wasm_bindgen]
    pub fn count_hand_pieces(&self) -> String {
        let mut sum = String::from("");
        for color in Color::iter() {
            if color != Color::NoColor {
                let iterator = PieceTypeIter::new();
                for piece_type in iterator {
                    if self.shuuro.variant().wrong(piece_type.index()) {
                        continue;
                    }
                    let piece = Piece { piece_type, color };
                    let counter = self.shuuro.hand(piece);
                    for _i in 0..counter {
                        sum.push(piece.to_string().chars().last().unwrap());
                    }
                }
            }
        }
        sum
    }

    /// Place piece on board.
    #[wasm_bindgen]
    pub fn place(&mut self, game_move: String) -> bool {
        let m = Move::from_sfen(&game_move.as_str());
        let past_length = self.shuuro.get_sfen_history().len();
        if let Some(m) = m {
            match m {
                Move::Put { to, piece } => {
                    self.shuuro.place(piece, to);
                }
                _ => (),
            }
        }
        let current_length = self.shuuro.get_sfen_history().len();
        current_length > past_length
    }

    /// FIGHT PART

    /// All legal moves for square.
    #[wasm_bindgen]
    pub fn legal_moves(&self, sq: &str) -> Array {
        let moves = Array::new();
        if let Some(square) = Square::from_sfen(&String::from(sq)) {
            if let Some(piece) = self.shuuro.piece_at(square) {
                if piece.color == self.shuuro.side_to_move() {
                    let l_m = self.shuuro.legal_moves(&square);
                    for i in l_m {
                        let value = JsValue::from_str(&i.to_string()[..]);
                        moves.push(&value);
                    }
                }
            }
        }
        moves
    }

    /// Get move from server and play
    #[wasm_bindgen]
    pub fn make_move(&mut self, game_move: String) -> String {
        if let Some(m) = Move::from_sfen(&game_move.as_str()) {
            match m {
                Move::Normal {
                    from,
                    to,
                    promote: _,
                } => {
                    let res = self
                        .shuuro
                        .play(&from.to_string().as_str(), &to.to_string().as_str());
                    let res = match res {
                        Ok(i) => i.to_string(),
                        Err(_) => String::from("illegal_move"),
                    };
                    return res;
                }
                _ => (),
            }
        }
        String::from("")
    }
}

/// This represents piece.
#[derive(Serialize, Deserialize)]
pub struct P {
    pub role: String,
    pub color: String,
}
