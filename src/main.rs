use crate::{
    board::Board,
    knightmove::get_knight_move_targets,
    pawnmove::{get_attack_targets, get_double_push_targets, get_single_push_targets},
    types::{Color, Occupancy, Piece},
    utils::display_bitboard,
};

mod board;
mod knightmove;
mod movegen;
mod pawnmove;
mod types;
mod utils;

fn main() {
    let board: Board = Board::new();

    board.display_chess_board();

    display_bitboard(get_knight_move_targets(
        board.pieces[Color::Black as usize][Piece::Knight as usize],
        board.occupancies[Occupancy::Empty as usize],
    ));
}
