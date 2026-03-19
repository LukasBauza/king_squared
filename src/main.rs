use crate::{
    board::Board,
    pawnmove::{get_attack_targets, get_double_push_targets, get_single_push_targets},
    types::{Color, Occupancy, Piece},
    utils::display_bitboard,
};

mod board;
mod movegen;
mod pawnmove;
mod types;
mod utils;

fn main() {
    let board: Board = Board::new();

    board.display_chess_board();

    // display_bitboard(get_double_push_targets(
    //     board.pieces[Color::White as usize][Piece::Pawn as usize],
    //     Color::White,
    //     board.occupancies[Occupancy::Empty as usize],
    // ));

    display_bitboard(get_attack_targets(
        board.pieces[Color::White as usize][Piece::Pawn as usize],
        Color::White,
    ));
}
