//Traditional Board size is 19x19
//Also popular are 13x13 and 9x9

/**
 * board_state represents the board using 2D vectors
 * 0 means onocuppied
 * 1 means white stone
 * 2 means black stone
 */
pub(crate) struct Board {
    pub board_size: usize,
    pub board_state: Vec<Vec<u8>>
}

impl Board {
    pub fn new(board_size: usize) -> Self {
        Board {
            board_size: board_size,
            board_state: vec![vec![0; board_size]; board_size],
        }
    }
}

pub(crate) enum Color {
    WHITE,
    BLACK
}