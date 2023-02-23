use js_sys::{Array, Map};
use serde::{Deserialize, Serialize};
use shuuro::{
    attacks::Attacks, bitboard::BitBoard, piece_type::PieceTypeIter, position::Position, Color,
    Move, Piece, Square, Variant,
};
use wasm_bindgen::JsValue;

use std::{
    hash::Hash,
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitOrAssign, Not},
};

#[derive(Clone, Copy)]
pub struct LocalPosition<S, B, A, P>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    P: Position<S, B, A>,

    for<'a> &'a B: BitOr<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> B: BitOrAssign<&'a S>,
{
    _s: PhantomData<S>,
    _b: PhantomData<B>,
    _a: PhantomData<A>,
    _p: PhantomData<P>,
    state: P,
}

impl<S, B, A, P> LocalPosition<S, B, A, P>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    P: Position<S, B, A>,

    for<'a> &'a B: BitOr<&'a B, Output = B>,
    for<'a> &'a B: BitAnd<&'a B, Output = B>,
    for<'a> &'a B: Not<Output = B>,
    for<'a> &'a B: BitOr<&'a S, Output = B>,
    for<'a> &'a B: BitAnd<&'a S, Output = B>,
    for<'a> B: BitOrAssign<&'a S>,
{
    pub fn new() -> Self {
        A::init();
        Self {
            _s: PhantomData,
            _b: PhantomData,
            _a: PhantomData,
            _p: PhantomData,
            state: P::new(),
        }
    }
    // Main functions.

    pub fn change_variant(&mut self, s: &str) {
        self.state.update_variant(Variant::from(&String::from(s)));
    }

    pub fn set_hand(&mut self, s: &str) {
        self.state.set_hand(s);
    }

    pub fn set_sfen(&mut self, s: &str) {
        if let Err(_e) = self.state.set_sfen(s) {}
    }

    pub fn generate_sfen(&self) -> String {
        self.state.generate_sfen()
    }

    pub fn side_to_move(&self) -> String {
        self.state.side_to_move().to_string()
    }

    pub fn map_plinths(&self) -> Map {
        let list = Map::new();
        let bb = self.state.player_bb(Color::NoColor);
        for i in bb {
            let example = PieceJS {
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

    pub fn map_pieces(&self) -> Map {
        let list = Map::new();
        let colors = [Color::White, Color::Black];
        for i in colors {
            let bb = self.state.player_bb(i);
            let color = self.get_color(&i.to_string());
            for sq in bb {
                let piece = self.state.piece_at(sq);
                if let Some(piece) = piece {
                    let sq = sq.to_string();
                    let p = PieceJS {
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

    pub fn pieces_count(&self) -> usize {
        let mut sum = self.state.player_bb(Color::Black).count();
        sum += self.state.player_bb(Color::White).count();
        sum
    }

    pub fn last_move(&self) -> String {
        self.state.get_sfen_history().last().unwrap().to_string()
    }

    pub fn is_check(&self) -> bool {
        self.state.in_check(self.state.side_to_move())
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

    pub fn place_moves(&mut self, piece: char) -> Map {
        let list = Map::new();
        if let Some(p) = Piece::from_sfen(piece) {
            let bb = self.state.empty_squares(p);
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

    pub fn count_hand_pieces(&self) -> String {
        let mut sum = String::from("");
        for color in Color::iter() {
            if color != Color::NoColor {
                let iterator = PieceTypeIter::default();
                for piece_type in iterator {
                    if !self.state.variant().can_buy(&piece_type) {
                        continue;
                    }
                    let piece = Piece { piece_type, color };
                    let counter = self.state.hand(piece);
                    for _i in 0..counter {
                        sum.push(piece.to_string().chars().last().unwrap());
                    }
                }
            }
        }
        sum
    }

    pub fn place(&mut self, game_move: String) -> bool {
        let m = Move::from_sfen(game_move.as_str());
        let past_length = self.state.get_sfen_history().len();
        #[allow(clippy::collapsible_match)]
        if let Some(m) = m {
            if let Move::Put { to, piece } = m {
                self.state.place(piece, to);
            }
        }
        let current_length = self.state.get_sfen_history().len();
        current_length > past_length
    }

    pub fn legal_moves(&self, color: &str) -> Map {
        let map = Map::new();
        let stm = self.state.side_to_move();
        if color == stm.to_string() {
            let l_m = self.state.legal_moves(&stm);
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

    pub fn make_move(&mut self, game_move: String) -> String {
        #[allow(clippy::collapsible_match)]
        if let Some(m) = Move::<S>::from_sfen(game_move.as_str()) {
            if let Move::Normal {
                from,
                to,
                promote: _,
            } = m
            {
                let res = self
                    .state
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

/// This represents piece.
#[derive(Serialize, Deserialize)]
pub struct PieceJS {
    pub role: String,
    pub color: String,
}
