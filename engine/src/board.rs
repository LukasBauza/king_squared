use num_enum::TryFromPrimitive;

type Bitboard = u64;

#[rustfmt::skip]
enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[repr(usize)]
enum Color {
    White = 0,
    Black,
}

#[repr(usize)]
enum Piece {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[repr(usize)]
enum Occupancy {
    White = 0,
    Black,
    Occupied,
    Empty,
}

pub struct Board {
    pieces: [[Bitboard; 6]; 2],
    occupancies: [Bitboard; 4],

    side_to_move: bool,

    castling_rights: u8,
    en_passant_square: u8,
    half_move_clock: u8,
}

impl Board {
    fn set_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.pieces[color as usize][piece as usize] |= 1u64 << square as u64;

        if color == Color::White {
        } else {
        }
    }

    fn update_occupancy(&mut self, color: Color) {
        if color == Color::White {
            self.occupancies[Occupancy::White as usize];
        } else {
            self.occupancies[Occupancy::Black as usize];
        }

        match color {
            Color::White -> self.occupancies[Occupancy::White as usize];
        }
    }
}
