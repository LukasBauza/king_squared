use crate::movegen::*;
use crate::types::Bitboard;

const EMPTY: Bitboard = 0x0000000000000000;

pub fn white_single_push_targets(pawns: Bitboard) -> Bitboard {
    return shift_north(pawns) & EMPTY;
}

pub fn white_double_push_targets(pawns: Bitboard) -> Bitboard {
    const RANK4: Bitboard = 0x00000000FF000000;
    let single_push = white_single_push_targets(pawns);

    return white_single_push_targets(single_push) & RANK4;
}

pub fn black_single_push_targets(pawns: Bitboard) -> Bitboard {
    return shift_south(pawns) & EMPTY;
}

pub fn black_double_push_targets(pawns: Bitboard) -> Bitboard {
    const RANK5: Bitboard = 0x000000FF00000000;
    let single_push = black_single_push_targets(pawns);

    return black_single_push_targets(single_push) & RANK5;
}
