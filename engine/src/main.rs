mod board;
mod utils;

use crate::board::{Board, PieceType};

fn main() {
    let board = Board::new();

    let white_pawns_bb = board.get_piece_bitboard(PieceType::WhitePawns);
    println!("White pawns bitboard: {:#064b}", white_pawns_bb);
}
