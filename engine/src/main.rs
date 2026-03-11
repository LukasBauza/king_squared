mod board;
mod utils;

use crate::board::Board;

fn main() {
    let board = Board::new();

    //board.display_bitboard(board::BoardType::WhiteBishops);
    board.display_chess_board();
}
