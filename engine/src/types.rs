use strum_macros::EnumIter;

pub type Bitboard = u64;

pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eight,
}

pub struct Square {
    pub file: File,
    pub rank: Rank,
}

#[repr(usize)]
#[derive(Copy, Clone, EnumIter)]
pub enum Color {
    White = 0,
    Black,
}

#[repr(usize)]
#[derive(Copy, Clone, EnumIter)]
pub enum Piece {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[repr(usize)]
pub enum Occupancy {
    White = 0,
    Black,
    Occupied,
    Empty,
}
