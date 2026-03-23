use crate::Board;
use crate::move_gen::files_ranks::RANK_MASKS;
use crate::types::{Bitboard, Color, Occupancy, Piece, Rank};

pub fn get_single_push_targets(board: Board, color: Color) -> Bitboard {
    match color {
        Color::White => {
            return (board.pieces[color as usize][Piece::Pawn as usize] << 8)
                & board.occupancies[Occupancy::Empty as usize];
        }
        Color::Black => {
            return (board.pieces[color as usize][Piece::Pawn as usize] >> 8)
                & board.occupancies[Occupancy::Empty as usize];
        }
    }
}

pub fn get_double_push_targets(board: Board, color: Color) -> Bitboard {
    match color {
        Color::White => {
            return (board.pieces[color as usize][Piece::Pawn as usize] << 16)
                & board.occupancies[Occupancy::Empty as usize]
                & RANK_MASKS[Rank::Fourth as usize];
        }
        Color::Black => {
            return (board.pieces[color as usize][Piece::Pawn as usize] >> 16)
                & board.occupancies[Occupancy::Empty as usize]
                & RANK_MASKS[Rank::Fifth as usize];
        }
    }
}

pub fn get_attack_targets(pawns: Bitboard, color: Color, empty: Bitboard) -> Bitboard {
    match color {
        Color::White => {
            let north_east_shift = (pawns >> 7) & empty;
            let north_west_shift = (pawns >> 9) & empty;

            return north_east_shift | north_west_shift;
        }

        Color::Black => {
            let south_east_shift = (pawns << 9) & empty;
            let south_west_shift = (pawns << 7) & empty;

            return south_east_shift | south_west_shift;
        }
    }
}
