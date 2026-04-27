use crate::{
    board::Board,
    types::{Bitboard, Occupancy, Square},
};
use strum::EnumCount;

struct MagicEntry {
    mask: Bitboard,
    magic_number: u64,
    index_bits: u8,
}

const ROOK_MAGICS: &[MagicEntry; Square::COUNT] = todo!();
const BISHOP_MAGICS: &[MagicEntry; Square::COUNT] = todo!();

const ROOK_MOVES: &[&[Bitboard]; Square::COUNT] = todo!();
const BISHOP_MOVES: &[&[Bitboard]; Square::COUNT] = todo!();

fn magic_index(entry: MagicEntry, board: Board) -> usize {
    let occupied = board.occupancies[Occupancy::Occupied as usize] & entry.mask;
    let hash = occupied.wrapping_mul(entry.magic_number);
    let index = (hash >> (64 - entry.index_bits)) as usize;

    return index;
}
