
pub const WHITE: u8 = 1;
pub const BLACK: u8 = 2;

/**
 *
 * There are traditionaly 181 stones in Go.
 * Traditional Board size is 19x19
 * Also popular are 13x13 and 9x9
 */

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

    pub fn place_stone(&mut self, player: &Player, row: usize, col: usize) -> bool {
        if self.board_state[row][col] == 0 {
            self.board_state[row][col] = player.player_color;
            return true;
        } else {
            return false;
        }
    }

    //Check the last placed location and see if it has created a capture
    //Returns the number of stones captured
    pub fn check_for_capture(&mut self, row: usize, col:usize) -> u8 {
        
        return 0;
    }
}

pub(crate) struct Player {
    player_color: u8,
    stones_taken: u8,
}

impl Player {
    pub fn new(new_player_color: u8) -> Self {
        Player {
            player_color: new_player_color,
            stones_taken: 0,
        }
    }

    //Add stones that have been taken from opponent
    pub fn add_stone_taken(&mut self, num_taken_stones: u8) -> u8 {
        self.stones_taken = self.stones_taken + num_taken_stones;
        return self.stones_taken;
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Player;
    use crate::game::Board;

    #[test]
    fn test_place_stone() {
        let test_player: Player = Player::new(crate::game::BLACK);
        let mut test_board: Board = Board::new(9);
        let result1 = test_board.place_stone(&test_player, 2, 2);
        assert_eq!(test_board.board_state[2][2], crate::game::BLACK);
        let result2 = test_board.place_stone(&test_player, 8, 8);
        let result3 = test_board.place_stone(&test_player, 8, 8);
        assert_eq!(test_board.board_state[8][8], crate::game::BLACK);
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, false);
    }

    #[test]
    fn test_add_stone_taken() {
        let mut test_player: Player = Player::new(crate::game::WHITE);
        test_player.add_stone_taken(5);
        assert_eq!(test_player.stones_taken, 5);
        test_player.add_stone_taken(3);
        assert_eq!(test_player.stones_taken, 8);
    }

    #[test]
    fn test_check_for_capture() {
        let mut test_player: Player = Player::new(crate::game::WHITE);
        let mut test_board: Board = Board::new(3);
        test_board.board_state[0][0] = 1;
        test_board.board_state[0][1] = 1;
        test_board.board_state[0][2] = 1;
        test_board.board_state[1][0] = 1;
        test_board.board_state[1][1] = 2;
        test_board.board_state[1][2] = 1;
        //test_board[2][1] getting acquired by white (1) will capture the black (2) at [1][1]
        test_board.place_stone(&test_player, 2, 1);
        let captured: u8 = test_board.check_for_capture(2, 1); //Should return 1.
        assert_eq!(captured, 1);
        test_player.add_stone_taken(captured);
        assert_eq!(test_player.stones_taken, 1);
    }

}
