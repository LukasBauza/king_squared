use crate::types::Bitboard;

const NOT_A_FILE: Bitboard = 0xfefefefefefefefe;
const NOT_H_FILE: Bitboard = 0x7f7f7f7f7f7f7f7f;

pub fn shift_north(bitboard: Bitboard) -> Bitboard {
    bitboard << 8
}

pub fn shift_south(bitboard: Bitboard) -> Bitboard {
    bitboard >> 8
}

pub fn shift_east(bitboard: Bitboard) -> Bitboard {
    (bitboard << 1) & NOT_A_FILE
}

pub fn shift_west(bitboard: Bitboard) -> Bitboard {
    (bitboard >> 1) & NOT_H_FILE
}

pub fn shift_north_east(bitboard: Bitboard) -> Bitboard {
    (bitboard << 9) & NOT_A_FILE
}

pub fn shift_north_west(bitboard: Bitboard) -> Bitboard {
    (bitboard << 7) & NOT_H_FILE
}

pub fn shift_south_east(bitboard: Bitboard) -> Bitboard {
    (bitboard >> 7) & NOT_H_FILE
}

pub fn shift_south_west(bitboard: Bitboard) -> Bitboard {
    (bitboard >> 9) & NOT_A_FILE
}

pub fn shift(bitboard: Bitboard, shifts: u8, right: bool) -> Bitboard {
    let offset = shifts % 8;
    let out: Bitboard;

    if right {
        out = bitboard << shifts;
        match offset {
            1 => out & NOT_A_FILE,
        };
    } else {
        out = bitboard >> shifts;
    }

    return out;
}
