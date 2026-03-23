use crate::types::Bitboard;

pub const FILE_MASKS: [Bitboard; 8] = [
    0x0101010101010101, // File A
    0x0202020202020202, // File B
    0x0404040404040404, // File C
    0x0808080808080808, // File D
    0x1010101010101010, // File E
    0x2020202020202020, // File F
    0x4040404040404040, // File G
    0x8080808080808080, // File H
];

pub const RANK_MASKS: [Bitboard; 8] = [
    0x0000_0000_0000_00FF, // Rank 0
    0x0000_0000_0000_FF00, // Rank 1
    0x0000_0000_00FF_0000, // Rank 2
    0x0000_0000_FF00_0000, // Rank 3
    0x0000_00FF_0000_0000, // Rank 4
    0x0000_FF00_0000_0000, // Rank 5
    0x00FF_0000_0000_0000, // Rank 6
    0xFF00_0000_0000_0000, // Rank 7
];
