use crate::local_position::LocalPosition;
use js_sys::Map;
use shuuro::shuuro12::{
    attacks12::Attacks12, bitboard12::BB12, position12::P12, square12::Square12,
};
use shuuro::shuuro8::{attacks8::Attacks8, bitboard8::BB8, position8::P8, square8::Square8};
use shuuro::Variant;

macro_rules! local_position {
    // mutate
    ($self: ident, $method: ident, $param: expr, $mut: expr, $ret: expr) => {
        match $self.variant {
            Variant::Standard | Variant::StandardFairy => {
                $self.local8.as_mut().unwrap().$method($param)
            }
            _ => $self.local12.as_mut().unwrap().$method($param),
        }
    };
    ($self: ident, $method: ident, $param: expr, $mut: expr) => {
        match $self.variant {
            Variant::Standard | Variant::StandardFairy => {
                $self.local8.as_mut().unwrap().$method($param)
            }
            _ => $self.local12.as_mut().unwrap().$part($param),
        }
    };
    // reading
    ($self: ident, $method: ident) => {
        match $self.variant {
            Variant::Standard | Variant::StandardFairy => $self.local8.as_ref().unwrap().$method(),
            _ => $self.local12.as_ref().unwrap().$method(),
        }
    };

    ($self: ident, $method: ident, $param: expr) => {
        match $self.variant {
            Variant::Standard | Variant::StandardFairy => {
                $self.local8.as_ref().unwrap().$method($param)
            }
            _ => $self.local12.as_ref().unwrap().$method($param),
        }
    };
}

type Local8 = LocalPosition<
    Square8,
    BB8<Square8>,
    Attacks8<Square8, BB8<Square8>>,
    P8<Square8, BB8<Square8>>,
>;

type Local12 = LocalPosition<
    Square12,
    BB12<Square12>,
    Attacks12<Square12, BB12<Square12>>,
    P12<Square12, BB12<Square12>>,
>;

pub struct PositionContainer {
    local8: Option<Local8>,
    local12: Option<Local12>,
    variant: Variant,
}

impl PositionContainer {
    #[inline]
    pub fn new(variant: Variant) -> Self {
        match variant {
            Variant::Standard | Variant::StandardFairy => Self {
                local8: Some(Local8::new()),
                local12: None,
                variant,
            },
            _ => Self {
                local12: Some(Local12::new()),
                local8: None,
                variant,
            },
        }
    }

    #[inline]
    pub fn change_variant(&mut self, variant: &str) {
        local_position!(self, change_variant, variant, true, true);
    }

    #[inline]
    pub fn set_hand(&mut self, hand: &str) {
        local_position!(self, set_hand, hand, true, true);
    }

    #[inline]
    pub fn set_sfen(&mut self, s: &str) {
        local_position!(self, set_sfen, s, true, true);
    }

    #[inline]
    pub fn generate_sfen(&self) -> String {
        local_position!(self, generate_sfen)
    }

    #[inline]
    pub fn side_to_move(&self) -> String {
        local_position!(self, side_to_move)
    }

    #[inline]
    pub fn map_plinths(&self) -> Map {
        local_position!(self, map_plinths)
    }

    #[inline]
    pub fn map_pieces(&self) -> Map {
        local_position!(self, map_pieces)
    }

    #[inline]
    pub fn pieces_count(&self) -> usize {
        local_position!(self, pieces_count)
    }

    #[inline]
    pub fn last_move(&self) -> String {
        local_position!(self, last_move)
    }

    #[inline]
    pub fn is_check(&self) -> bool {
        local_position!(self, is_check)
    }

    #[inline]
    pub fn place_moves(&mut self, piece: char) -> Map {
        local_position!(self, place_moves, piece, false, false)
    }

    pub fn place(&mut self, game_move: String) -> bool {
        local_position!(self, place, game_move, false, false)
    }

    #[inline]
    pub fn count_hand_pieces(&self) -> String {
        local_position!(self, count_hand_pieces)
    }

    #[inline]
    pub fn legal_moves(&self, color: &str) -> Map {
        local_position!(self, legal_moves, color)
    }

    #[inline]
    pub fn make_move(&mut self, game_move: String) -> String {
        local_position!(self, make_move, game_move, false, false)
    }
}
