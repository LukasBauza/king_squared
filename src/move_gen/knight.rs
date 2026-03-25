use crate::Board;
use crate::move_gen::files_ranks::FILE_MASKS;
use crate::types::{Bitboard, Color, File, Occupancy, Piece};

pub fn get_move_targets(board: Board, color: Color) -> Bitboard {
    let knights = board.pieces[color as usize][Piece::Knight as usize];

    let moves: [Bitboard; 8] = [
        (knights << 17) & !FILE_MASKS[File::A as usize],
        (knights << 15) & !FILE_MASKS[File::H as usize],
        (knights << 10) & !FILE_MASKS[File::A as usize] & !FILE_MASKS[File::B as usize],
        (knights << 6) & !FILE_MASKS[File::G as usize] & !FILE_MASKS[File::H as usize],
        (knights >> 17) & !FILE_MASKS[File::H as usize],
        (knights >> 15) & !FILE_MASKS[File::A as usize],
        (knights >> 10) & !FILE_MASKS[File::G as usize] & !FILE_MASKS[File::H as usize],
        (knights >> 6) & !FILE_MASKS[File::A as usize] & !FILE_MASKS[File::B as usize],
    ];

    let mut all_moves = 0;

    for m in moves {
        all_moves |= m;
    }

    return all_moves & board.occupancies[Occupancy::Empty as usize];
}

pub fn get_attack_targets(board: Board, color: Color) -> Bitboard {
    return get_move_targets(board, color);
}
