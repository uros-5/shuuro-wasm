use js_sys::Map;
pub use shuuro::shuuro12::{attacks12::Attacks12, bitboard12::BB12, square12::Square12};
use shuuro::Variant;
use wasm_bindgen::prelude::*;

use crate::position_container::PositionContainer;

#[wasm_bindgen]
pub struct ShuuroPosition {
    shuuro: PositionContainer,
}

#[wasm_bindgen]
impl ShuuroPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(variant: &str) -> Self {
        Self {
            shuuro: PositionContainer::new(Variant::from(&variant.to_string())),
        }
    }
    // Main functions.

    /// Change game variant
    #[wasm_bindgen]
    pub fn change_variant(&mut self, s: &str) {
        self.shuuro.change_variant(s);
    }

    /// Set hand for pocket.
    #[wasm_bindgen]
    pub fn set_hand(&mut self, s: &str) {
        self.shuuro.set_hand(s);
    }

    /// Set sfen.
    #[wasm_bindgen]
    pub fn set_sfen(&mut self, s: &str) {
        self.shuuro.set_sfen(s);
    }

    /// Get sfen for current position.
    #[wasm_bindgen]
    pub fn generate_sfen(&self) -> String {
        self.shuuro.generate_sfen()
    }

    /// Get side to move.
    #[wasm_bindgen]
    pub fn side_to_move(&self) -> String {
        self.shuuro.side_to_move()
    }

    /// All plinths on board.
    #[wasm_bindgen]
    pub fn map_plinths(&self) -> Map {
        self.shuuro.map_plinths()
    }

    /// All pieces on board.
    #[wasm_bindgen]
    pub fn map_pieces(&self) -> Map {
        self.shuuro.map_pieces()
    }

    /// Get piece count.
    #[wasm_bindgen]
    pub fn pieces_count(&self) -> usize {
        self.shuuro.pieces_count()
    }
    /// Get last move.
    #[wasm_bindgen]
    pub fn last_move(&self) -> String {
        self.shuuro.last_move()
    }

    /// Returns if side_to_move is in check.
    #[wasm_bindgen]
    pub fn is_check(&self) -> bool {
        self.shuuro.is_check()
    }

    // Deploy part

    /// Squares where piece can be placed.
    pub fn place_moves(&mut self, piece: char) -> Map {
        self.shuuro.place_moves(piece)
    }

    /// Count how many pieces are left in hand.
    #[wasm_bindgen]
    pub fn count_hand_pieces(&self) -> String {
        self.shuuro.count_hand_pieces()
    }

    /// Place piece on board.
    #[wasm_bindgen]
    pub fn place(&mut self, game_move: String) -> bool {
        self.shuuro.place(game_move)
    }

    /// FIGHT PART

    /// All legal moves for square.
    #[wasm_bindgen]
    pub fn legal_moves(&self, color: &str) -> Map {
        self.shuuro.legal_moves(color)
    }

    /// Get move from server and play
    #[wasm_bindgen]
    pub fn make_move(&mut self, game_move: String) -> String {
        self.shuuro.make_move(game_move)
    }
}

impl Default for ShuuroPosition {
    fn default() -> Self {
        Self::new("shuuro")
    }
}
