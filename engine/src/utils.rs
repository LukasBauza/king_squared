pub fn get_square_index(rank: u8, file: u8) -> u64 {
    return (rank * 8 + file) as u64;
}