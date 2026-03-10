#[rustfmt::skip]
#[derive(Copy, Clone)]
pub(crate)enum PieceType {
    WhitePawns = 0, WhiteKnights = 1, WhiteBishops = 2, WhiteRooks = 3, WhiteQueens = 4, WhiteKing = 5,
    BlackPawns = 6, BlackKnights = 7, BlackBishops = 8, BlackRooks = 9, BlackQueens = 10, BlackKing = 11,

    // Adding these will save time in calculation, as you don't need to count each array individually.
    WhitePieces = 12,
    BlackPieces = 13,
}

pub(crate) struct Board {
    pieces: [u64; 14],
    empty: u64,
    occupied: u64,
}

impl Board {
    pub(crate) fn new() -> Self {
        let mut pieces = [0u64; 14];

        Self::set_rank_pieces([PieceType::WhitePawns; 8], 1, &mut pieces);
        Self::set_rank_pieces(
            [
                PieceType::WhiteRooks,
                PieceType::WhiteKnights,
                PieceType::WhiteBishops,
                PieceType::WhiteQueens,
                PieceType::WhiteKing,
                PieceType::WhiteBishops,
                PieceType::WhiteKnights,
                PieceType::WhiteRooks,
            ],
            0,
            &mut pieces,
        );

        Self::set_rank_pieces([PieceType::BlackPawns; 8], 6, &mut pieces);
        Self::set_rank_pieces(
            [
                PieceType::BlackRooks,
                PieceType::BlackKnights,
                PieceType::BlackBishops,
                PieceType::BlackQueens,
                PieceType::BlackKing,
                PieceType::BlackBishops,
                PieceType::BlackKnights,
                PieceType::BlackRooks,
            ],
            0,
            &mut pieces,
        );

        let occupied =
            pieces[PieceType::WhitePieces as usize] | pieces[PieceType::BlackPieces as usize];
        let empty = !occupied;

        Self {
            pieces,
            occupied,
            empty,
        }
    }

    fn set_rank_pieces(piece_type: [PieceType; 8], rank: u8, pieces: &mut [u64; 14]) {
        for (file, piece) in piece_type.iter().enumerate() {
            let sq = crate::utils::get_square_index(rank, file as u8);
            pieces[*piece as usize] |= 1u64 << sq;
        }
    }

    pub(crate) fn get_piece_bitboard(&self, piece: PieceType) -> u64 {
        return self.pieces[piece as usize];
    }

    fn get_empty_bitboard(&self) -> u64 {
        return self.empty;
    }

    fn get_occupied_bitboard(&self) -> u64 {
        return self.occupied;
    }
    
    // TODO: Create method to display the bitboard.
}
