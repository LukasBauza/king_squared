use crate::movegen::{shift_north, shift_south};
use crate::types::{Bitboard, Color};

pub fn get_single_push_targets(pawns: Bitboard, color: Color, empty: Bitboard) -> Bitboard {
    match color {
        Color::White => return shift_north(pawns) & empty,
        Color::Black => return shift_south(pawns) & empty,
    }
}

pub fn get_double_push_targets(pawns: Bitboard, color: Color, empty: Bitboard) -> Bitboard {
    match color {
        Color::White => {
            const RANK4: Bitboard = 0x00000000FF000000;

            return (get_single_push_targets(pawns, color, empty) << 8) & empty & RANK4;
        }
        Color::Black => {
            const RANK5: Bitboard = 0x000000FF00000000;

            return (get_single_push_targets(pawns, color, empty) >> 8) & empty & RANK5;
        }
    }
}
