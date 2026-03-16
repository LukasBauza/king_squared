use crate::types::{File, Rank, Square};

pub const fn get_square_index(rank: Rank, file: File) -> Square {
    let index = (rank as u8) * 8 + (file as u8);

    return Square::from_repr(index).expect("Invalid square index.");
}
