use crate::board::Board;

mod board;
mod types;
mod utils;

fn main() {
    let board: Board = Board::new();

    board.display_chess_board();
}
