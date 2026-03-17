use crate::movegen::*;
use crate::types::{Bitboard, Color};

const EMPTY: Bitboard = 0x0000000000000000;

pub fn get_single_push_targets(pawns: Bitboard, color: Color) -> Bitboard {
    match color {
        Color::White => return shift_north(pawns) & EMPTY,
        Color::Black => return shift_south(pawns) & EMPTY,
    }
}

pub fn get_double_push_targets(pawns: Bitboard, color: Color) -> Bitboard {
    match color {
        Color::White => {
            const RANK4: Bitboard = 0x00000000FF000000;

            return get_single_push_targets(pawns, color) & RANK4;
        }
        Color::Black => {
            const RANK5: Bitboard = 0x000000FF00000000;

            return get_single_push_targets(pawns, color) & RANK5;
        }
    }
}
