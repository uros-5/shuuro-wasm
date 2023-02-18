use js_sys::{Array, Map};
use serde::{Deserialize, Serialize};
use shuuro::attacks::Attacks;
use shuuro::shuuro12::position12::P12;
pub use shuuro::shuuro12::{attacks12::Attacks12, bitboard12::BB12, square12::Square12};
use shuuro::{self, piece_type::PieceTypeIter, Color, Move, Piece};
use shuuro::{position::*, Variant};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct ShuuroPosition {
    shuuro: P12<Square12, BB12<Square12>>,
}

#[wasm_bindgen]
impl ShuuroPosition {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Attacks12::init();
        ShuuroPosition {
            shuuro: P12::default(),
        }
    }
    // Main functions.

    /// Change game variant
    #[wasm_bindgen]
    pub fn change_variant(&mut self, s: &str) {
        self.shuuro.update_variant(Variant::from(&String::from(s)));
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
        for i in bb {
            let example = P {
                role: String::from("l-piece"),
                color: String::from("white"),
            };
            let sq = i.to_string();

            list.set(
                &JsValue::from_str(sq.as_str()),
                &serde_wasm_bindgen::to_value(&example).unwrap(), // &JsValue::from_serde(&example).unwrap(),
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
            for sq in bb {
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
                        &serde_wasm_bindgen::to_value(&p).unwrap(), // &JsValue::from_serde(&example).unwrap(),
                    );
                }
            }
        }

        list
    }

    /// Get piece count.
    #[wasm_bindgen]
    pub fn pieces_count(&self) -> usize {
        let mut sum = self.shuuro.player_bb(Color::Black).count();
        sum += self.shuuro.player_bb(Color::White).count();
        sum
    }
    /// Get last move.
    #[wasm_bindgen]
    pub fn last_move(&self) -> String {
        self.shuuro.get_sfen_history().last().unwrap().to_string()
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
                let iterator = PieceTypeIter::default();
                for piece_type in iterator {
                    if !self.shuuro.variant().can_buy(&piece_type) {
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
        let m = Move::from_sfen(game_move.as_str());
        let past_length = self.shuuro.get_sfen_history().len();
        #[allow(clippy::collapsible_match)]
        if let Some(m) = m {
            if let Move::Put { to, piece } = m {
                self.shuuro.place(piece, to);
            }
        }
        let current_length = self.shuuro.get_sfen_history().len();
        current_length > past_length
    }

    /// FIGHT PART

    /// All legal moves for square.
    #[wasm_bindgen]
    pub fn legal_moves(&self, color: &str) -> Map {
        let map = Map::new();
        let stm = self.shuuro.side_to_move();
        if color == stm.to_string() {
            let l_m = self.shuuro.legal_moves(&stm);
            for m in l_m {
                let piece = m.0.to_string();
                let moves = Array::new();
                for sq in m.1 {
                    let value = JsValue::from_str(&sq.to_string()[..]);
                    moves.push(&value);
                }
                let piece = JsValue::from_str(&piece);
                let moves = JsValue::from(moves);
                map.set(&piece, &moves);
            }
        }
        map
    }

    /// Get move from server and play
    #[wasm_bindgen]
    pub fn make_move(&mut self, game_move: String) -> String {
        #[allow(clippy::collapsible_match)]
        if let Some(m) = Move::<Square12>::from_sfen(game_move.as_str()) {
            if let Move::Normal {
                from,
                to,
                promote: _,
            } = m
            {
                let res = self
                    .shuuro
                    .play(from.to_string().as_str(), to.to_string().as_str());
                let res = match res {
                    Ok(i) => i.to_string(),
                    Err(_) => String::from("illegal_move"),
                };
                return res;
            }
        }
        String::from("")
    }
}

impl Default for ShuuroPosition {
    fn default() -> Self {
        Self::new()
    }
}

/// This represents piece.
#[derive(Serialize, Deserialize)]
pub struct P {
    pub role: String,
    pub color: String,
}
