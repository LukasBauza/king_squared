use strum::IntoEnumIterator;

use crate::types::{Bitboard, Color, File, Occupancy, Piece, Rank, Square};

pub struct Board {
    pieces: [[Bitboard; 6]; 2],
    occupancies: [Bitboard; 4],

    side_to_move: bool,

    castling_rights: u8,
    en_passant_square: u8,
    half_move_clock: u8,
}

impl Board {
    fn set_chess_board(&mut self) {
        const BACK_RANK_PIECES: [Piece; 8] = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        for (file, piece) in BACK_RANK_PIECES.iter().enumerate() {
            self.set_piece(Color::White, piece, square);
        }
    }

    fn set_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.pieces[color as usize][piece as usize] |= 1u64 << square as u64;
    }

    fn update_occupancy(&mut self) {
        for color in Color::iter() {
            for piece in Piece::iter() {
                self.occupancies[Occupancy::White as usize] |=
                    self.pieces[color as usize][piece as usize];
            }
        }

        self.occupancies[Occupancy::Occupied as usize] |= self.occupancies
            [Occupancy::White as usize]
            | self.occupancies[Occupancy::Black as usize];

        self.occupancies[Occupancy::Empty as usize] =
            !self.occupancies[Occupancy::Occupied as usize];
    }
}
