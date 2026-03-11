use num_enum::TryFromPrimitive;

#[rustfmt::skip]
#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub(crate)enum BoardType {
    WhitePawns = 0, WhiteKnights = 1, WhiteBishops = 2, WhiteRooks = 3, WhiteQueens = 4, WhiteKing = 5,
    BlackPawns = 6, BlackKnights = 7, BlackBishops = 8, BlackRooks = 9, BlackQueens = 10, BlackKing = 11,

    // Adding these will save time in calculation, as you don't need to count each array individually.
    WhitePieces = 12,
    BlackPieces = 13,
    Occupied = 14,
    Empty = 15
}

pub(crate) struct Board {
    bitboards: [u64; 16],
}

impl Board {
    pub(crate) fn new() -> Self {
        let mut bitboards = [0u64; 16];

        Self::set_rank_pieces([BoardType::WhitePawns; 8], 1, &mut bitboards);
        Self::set_rank_pieces(
            [
                BoardType::WhiteRooks,
                BoardType::WhiteKnights,
                BoardType::WhiteBishops,
                BoardType::WhiteQueens,
                BoardType::WhiteKing,
                BoardType::WhiteBishops,
                BoardType::WhiteKnights,
                BoardType::WhiteRooks,
            ],
            0,
            &mut bitboards,
        );

        Self::set_rank_pieces([BoardType::BlackPawns; 8], 6, &mut bitboards);
        Self::set_rank_pieces(
            [
                BoardType::BlackRooks,
                BoardType::BlackKnights,
                BoardType::BlackBishops,
                BoardType::BlackQueens,
                BoardType::BlackKing,
                BoardType::BlackBishops,
                BoardType::BlackKnights,
                BoardType::BlackRooks,
            ],
            7,
            &mut bitboards,
        );

        bitboards[BoardType::Occupied as usize] =
            bitboards[BoardType::WhitePieces as usize] | bitboards[BoardType::BlackPieces as usize];
        // FIX: For some reason this is not being updated properly.
        bitboards[BoardType::Empty as usize] = !bitboards[BoardType::Occupied as usize];

        Self { bitboards }
    }

    fn set_rank_pieces(piece_type: [BoardType; 8], rank: u8, bitboards: &mut [u64; 16]) {
        for (file, piece) in piece_type.iter().enumerate() {
            let sq = crate::utils::get_square_index(rank, file as u8);
            bitboards[*piece as usize] |= 1u64 << sq;
        }
    }

    pub(crate) fn display_bitboard(&self, board_type: BoardType) {
        let bitboard = self.bitboards[board_type as usize];

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = crate::utils::get_square_index(rank, file as u8);
                let bit = (bitboard >> square) & 1;

                print!("{}", if bit == 1 { "1" } else { "." })
            }
            println!();
        }
    }

    pub(crate) fn display_chess_board(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                for (index, bitboard) in self.bitboards.iter().enumerate() {
                    // TODO: It is not actually needed to check wether there is
                    // a bit, as every bit should be associated with one board.
                    let square = crate::utils::get_square_index(rank, file as u8);
                    let bit = (bitboard >> square) & 1;

                    let board_type = BoardType::try_from(index);

                    #[rustfmt::skip]
                    match board_type {
                        Ok(BoardType::WhitePawns) => if bit == 1 { print!("WP") },
                        Ok(BoardType::WhiteKnights) => if bit == 1 { print!("WN") },
                        Ok(BoardType::WhiteBishops) => if bit == 1 { print!("WB") },
                        Ok(BoardType::WhiteRooks) => if bit == 1 { print!("WR") },
                        Ok(BoardType::WhiteQueens) => if bit == 1 { print!("WQ") },
                        Ok(BoardType::WhiteKing) => if bit == 1 { print!("WK") },

                        Ok(BoardType::BlackPawns) => if bit == 1 { print!("BP") },
                        Ok(BoardType::BlackKnights) => if bit == 1 { print!("BN") },
                        Ok(BoardType::BlackBishops) => if bit == 1 { print!("BB") },
                        Ok(BoardType::BlackRooks) => if bit == 1 { print!("BR") },
                        Ok(BoardType::BlackQueens) => if bit == 1 { print!("BQ") },
                        Ok(BoardType::BlackKing) => if bit == 1 { print!("BK") },
                        Ok(BoardType::Empty) => if bit == 1 { print!(" ..") },
                        Ok(BoardType::Occupied)
                        | Ok(BoardType::WhitePieces)
                        | Ok(BoardType::BlackPieces) => continue,
                        Err(_) => print!("??"),
                    }
                }
            }
            println!();
        }
    }
}
