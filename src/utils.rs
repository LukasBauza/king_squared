use strum::IntoEnumIterator;

use crate::types::{Bitboard, File, Rank, Square};

pub const fn get_square_index(rank: Rank, file: File) -> Square {
    let index = (rank as u8) * 8 + (file as u8);

    return Square::from_repr(index).expect("Invalid square index.");
}

pub fn display_bitboard(bitboard: Bitboard) {
    println!("\n  a b c d e f g h");
    for rank in Rank::iter().rev() {
        print!("{}", (rank as u8) + 1);
        for file in File::iter() {
            let mask = 1u64 << get_square_index(rank, file) as u64;

            print!("{}", if (bitboard & mask) != 0 { " 1" } else { " 0" });
        }
        println!();
    }
}
