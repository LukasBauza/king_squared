mod board;
mod utils;

use crate::board::{Board, PieceType};

fn main() {
    let board = Board::new();

    board.display_piece_bitboard(PieceType::BlackBishops);
}
