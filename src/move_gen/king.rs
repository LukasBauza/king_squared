use crate::{
    Board,
    move_gen::files_ranks::FILE_MASKS,
    types::{Bitboard, Color, File, Occupancy, Piece},
};

pub fn get_move_targets(board: Board, color: Color) -> Bitboard {
    let king = board.pieces[color as usize][Piece::King as usize];

    let moves: [Bitboard; 8] = [
        (king >> 9) & !FILE_MASKS[File::H as usize],
        (king >> 8),
        (king >> 7) & !FILE_MASKS[File::A as usize],
        (king >> 1) & !FILE_MASKS[File::H as usize],
        (king << 1) & !FILE_MASKS[File::A as usize],
        (king << 9) & !FILE_MASKS[File::H as usize],
        (king << 8),
        (king << 7) & !FILE_MASKS[File::A as usize],
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
