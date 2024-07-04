use crate::board::Color;

/**
 * 
 * There are traditionaly 181 stones in Go.
 */
pub(crate) struct Player {
    player_color: Color,
    stones_taken: u8,
}

impl Player {
    pub fn new(new_player_color: Color) -> Self {
        Player {
            player_color: new_player_color,
            stones_taken: 0
        }
    }

    //Add stones that have been taken
    pub fn add_stone_taken(&mut self, new_stones: u8) -> u8 {
        self.stones_taken = self.stones_taken + new_stones;
        return self.stones_taken;
    }
}