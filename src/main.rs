use crate::{board::Board, types::Color};

mod board;
mod movegen;
mod pawnmove;
mod types;
mod utils;

fn main() {
    let board: Board = Board::new();

    board.display_chess_board();

    board.display_piece_bitboard(Color::Black, types::Piece::Pawn);
}
