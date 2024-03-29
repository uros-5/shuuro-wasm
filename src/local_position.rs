extern crate console_error_panic_hook;
use js_sys::{Array, Map};
use serde::{Deserialize, Serialize};
use shuuro::{
    attacks::Attacks, bitboard::BitBoard, position::Position, Color, Move, Piece, Square, Variant,
};
use wasm_bindgen::JsValue;

use std::{hash::Hash, marker::PhantomData, panic};

#[derive(Clone, Copy)]
pub struct LocalPosition<S, B, A, P>
where
    S: Square + Hash,
    B: BitBoard<S>,
    A: Attacks<S, B>,
    P: Position<S, B, A>,
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

    pub fn start_credit(&self) -> i32 {
        self.state.variant().start_credit()
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
            if let Ok(piece) = serde_wasm_bindgen::to_value(&example) {
                list.set(&JsValue::from_str(sq.as_str()), &piece);
            }
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
                    let mut role = piece.to_string().to_lowercase();
                    role.push_str("-piece");
                    let p = PieceJS {
                        role,
                        color: String::from(color),
                    };
                    if let Ok(piece) = serde_wasm_bindgen::to_value(&p) {
                        list.set(&JsValue::from_str(sq.as_str()), &piece);
                    }
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
        let history = self.state.get_sfen_history();
        let m = history.last();
        if let Some(mv) = m {
            let mv = mv.to_string();
            {
                let _x = m;
            };
            mv
        } else {
            String::new()
        }
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
        let map = Map::new();
        if let Some(p) = Piece::from_sfen(piece) {
            let bb = self.state.empty_squares(p);
            let moves = Array::new();
            for i in bb {
                moves.push(&JsValue::from_str(&i.to_string()));
            }
            let mut piece = piece.to_uppercase().to_string();
            piece.push('@');
            let key = piece;
            let key = JsValue::from_str(&key);
            let value = JsValue::from(moves);
            map.set(&key, &value);
        }
        map
    }

    pub fn count_hand_pieces(&self) -> String {
        let w = self.state.get_hand(Color::White, true);
        let mut b = self.state.get_hand(Color::Black, true);
        b.push_str(&w);
        b
    }

    pub fn place(&mut self, game_move: String) -> bool {
        let m = Move::from_sfen(&game_move);
        let past_length = self.state.get_sfen_history().len();
        #[allow(clippy::collapsible_match)]
        if let Some(m) = m {
            if let Move::Put { to, piece, .. } = m {
                self.state.place(piece, to);
            }
        }
        let current_length = self.state.get_sfen_history().len();
        current_length > past_length
    }

    pub fn legal_moves(&self, color: &str) -> Map {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let map = Map::new();
        let stm = self.state.side_to_move();
        if color == stm.to_string() {
            let l_m = self.state.legal_moves(&stm);
            for m in l_m {
                let piece = m.0.to_string();
                let moves = Array::new();
                for sq in m.1 {
                    let value = JsValue::from_str(&sq.to_string());
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
            if let Move::Normal { from, to, .. } = m {
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

    pub fn last_move_data(&self) -> u8 {
        let last = self.state.move_history().last();
        let mut data = 0;
        if let Some(m) = last {
            let formatted = m.format();
            if formatted.contains('=') {
                data = 1;
            } else if formatted.contains('+') {
                data = 2;
            } else if formatted.contains('#') {
                data = 3
            }
        }
        data
    }
}

/// This represents piece.
#[derive(Serialize, Deserialize)]
pub struct PieceJS {
    pub role: String,
    pub color: String,
}
