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

    display_bitboard(move_gen::knight::get_knight_move_targets(
        board,
        Color::Black,
    ));
}
