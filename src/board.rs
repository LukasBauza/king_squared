use strum::IntoEnumIterator;

use crate::{
    types::{Bitboard, Color, File, Occupancy, Piece, Rank, Square},
    utils::get_square_index,
};

pub struct Board {
    pieces: [[Bitboard; 6]; 2],
    occupancies: [Bitboard; 4],

    side_to_move: Color,

    // TODO: These need to be changed, for better abstraction.
    castling_rights: u8,
    en_passant_square: u8,
    half_move_clock: u8,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            pieces: [[0; 6]; 2],
            occupancies: [0; 4],
            side_to_move: Color::White,
            castling_rights: 0xF,
            en_passant_square: 64,
            half_move_clock: 0,
        };

        const BACK_RANK: [Piece; 8] = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        board.set_pieces_on_rank(Color::White, BACK_RANK, Rank::First);
        board.set_pieces_on_rank(Color::White, [Piece::Pawn; 8], Rank::Second);
        board.set_pieces_on_rank(Color::Black, BACK_RANK, Rank::Eight);
        board.set_pieces_on_rank(Color::Black, [Piece::Pawn; 8], Rank::Seventh);
        board.update_occupancy();

        return board;
    }

    pub fn display_chess_board(&self) {
        println!("\n  a  b  c  d  e  f  g  h");

        for rank in Rank::iter().rev() {
            print!("{}", (rank as u8) + 1);

            for file in File::iter() {
                let square_index = get_square_index(rank, file);
                let bit_mask = 1u64 << square_index as u8;
                let mut square_content = " ..";

                for color in Color::iter() {
                    for piece in Piece::iter() {
                        if (self.pieces[color as usize][piece as usize] & bit_mask) == 0 {
                            continue;
                        }

                        square_content = match piece {
                            Piece::Pawn => "P",
                            Piece::Knight => "N",
                            Piece::Bishop => "B",
                            Piece::Rook => "R",
                            Piece::Queen => "Q",
                            Piece::King => "K",
                        };

                        match color {
                            Color::White => print!(" w{}", square_content),
                            Color::Black => print!(" b{}", square_content),
                        };
                    }
                }
                if square_content == " .." {
                    print!(" ..");
                }
            }
            println!();
        }
    }

    pub fn display_occupied_bitboard(&self, occupancy: Occupancy) {
        let bitboard = match occupancy {
            Occupancy::White => self.occupancies[Occupancy::White as usize],
            Occupancy::Black => self.occupancies[Occupancy::Black as usize],
            Occupancy::Occupied => self.occupancies[Occupancy::Occupied as usize],
            Occupancy::Empty => self.occupancies[Occupancy::Empty as usize],
        };

        self.display_bitboard(bitboard);
    }

    pub fn display_piece_bitboard(&self, color: Color, piece: Piece) {
        self.display_bitboard(self.pieces[color as usize][piece as usize]);
    }

    fn display_bitboard(&self, bitboard: Bitboard) {
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

    fn set_piece_on_square(&mut self, color: Color, piece: Piece, square: Square) {
        self.pieces[color as usize][piece as usize] |= 1u64 << square as u64;
    }

    // TODO: Is this needed?
    fn set_pieces_on_rank(&mut self, color: Color, pieces: [Piece; 8], rank: Rank) {
        for file in File::iter() {
            let square: Square = get_square_index(rank, file);

            self.set_piece_on_square(color, pieces[file as usize], square);
        }
    }

    fn update_occupancy(&mut self) {
        for piece in Piece::iter() {
            self.occupancies[Occupancy::White as usize] |=
                self.pieces[Color::White as usize][piece as usize];
            self.occupancies[Occupancy::Black as usize] |=
                self.pieces[Color::Black as usize][piece as usize];
        }

        self.occupancies[Occupancy::Occupied as usize] |= self.occupancies
            [Occupancy::White as usize]
            | self.occupancies[Occupancy::Black as usize];

        self.occupancies[Occupancy::Empty as usize] =
            !self.occupancies[Occupancy::Occupied as usize];
    }
}
