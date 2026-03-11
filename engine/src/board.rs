#[rustfmt::skip]
#[derive(Copy, Clone)]
pub(crate)enum BoardType {
    WhitePawns = 0, WhiteKnights = 1, WhiteBishops = 2, WhiteRooks = 3, WhiteQueens = 4, WhiteKing = 5,
    BlackPawns = 6, BlackKnights = 7, BlackBishops = 8, BlackRooks = 9, BlackQueens = 10, BlackKing = 11,

    // Adding these will save time in calculation, as you don't need to count each array individually.
    WhitePieces = 12,
    BlackPieces = 13,
    Empty = 14,
    Occupied = 15
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
        let bit_board = self.bitboards[board_type as usize];

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = crate::utils::get_square_index(rank, file as u8);
                let bit = (bit_board >> square) & 1;

                print!("{}", if bit == 1 { "1" } else { "." })
            }
            println!();
        }
    }
}

