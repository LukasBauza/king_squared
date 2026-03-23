use crate::move_gen::files_ranks::FILE_MASKS;
use crate::types::{Bitboard, File};

pub fn get_knight_move_targets(knights: Bitboard, empty: Bitboard) -> Bitboard {
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

    return all_moves & empty;
}
