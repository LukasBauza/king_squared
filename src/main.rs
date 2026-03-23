mod board;
mod move_gen;
mod types;
mod utils;

use crate::{
    board::Board,
    types::{Color, Occupancy, Piece},
    utils::display_bitboard,
};

fn main() {
    let board: Board = Board::new();

    board.display_chess_board();

    display_bitboard(move_gen::pawn::get_double_push_targets(board, Color::White));
}
