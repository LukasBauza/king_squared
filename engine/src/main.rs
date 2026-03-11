mod board;
mod utils;

use crate::board::Board;

fn main() {
    let board = Board::new();

    board.display_bitboard(board::BoardType::BlackBishops);
}
