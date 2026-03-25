use crate::Board;
use crate::move_gen::files_ranks::{FILE_MASKS, RANK_MASKS};
use crate::types::{Bitboard, Color, File, Occupancy, Piece, Rank};

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

pub fn get_attack_targets(board: Board, color: Color) -> Bitboard {
    match color {
        Color::White => {
            let north_east_shift = (board.pieces[color as usize][Piece::Pawn as usize] << 7)
                & !FILE_MASKS[File::H as usize];
            let north_west_shift = board.pieces[color as usize][Piece::Pawn as usize] << 9
                & !FILE_MASKS[File::A as usize];

            return north_east_shift | north_west_shift;
        }

        Color::Black => {
            let south_east_shift = (board.pieces[color as usize][Piece::Pawn as usize] >> 9)
                & !FILE_MASKS[File::H as usize];
            let south_west_shift = (board.pieces[color as usize][Piece::Pawn as usize] >> 7)
                & !FILE_MASKS[File::A as usize];

            return south_east_shift | south_west_shift;
        }
    }
}
