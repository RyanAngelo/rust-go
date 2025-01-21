use std::{
    collections::HashMap, fmt,
};

use bevy::prelude::Component;

pub const EMPTY: u8 = 0;
pub const WHITE: u8 = 1;
pub const BLACK: u8 = 2;
pub const WHITE_TERR: u8 = 3;
pub const BLACK_TERR: u8 = 4;

/**
 * Place a stone on the board and run through the accompanying logic
 * This includes checking to see if the opponent has had any chains captured.
 * Returns true if stone has been placed, false if it has not
 */
pub fn place_stone(
    board: &mut Board,
    player_model: &mut PlayerModel,
    opponent_model: &mut PlayerModel,
    row: usize,
    col: usize,
) -> bool {
    if board.board_state[row][col].player_color == 0 {
        println!("Placing {:?} at location {:?}/{:?}", player_model.player.player_color, row, col);
        board.update_board_color(row, col, player_model.player.player_color);
        let friends = get_adjacent(&mut board.board_state, board.board_size, row, col, player_model.player.player_color);
        let new_friend_chain = update_chain(&mut board.board_state, player_model, friends, row, col);
        player_model.add_player_chain(&Board::generate_id(row, col), new_friend_chain);
        //For the opponent
        let opponent_liberties = update_player_liberties(&mut board.board_state, board.board_size, opponent_model);
        opponent_model.set_player_liberties(opponent_liberties);
        let removed_chain_keys = check_for_conquered(opponent_model, &mut board.board_state);
        cleanup_captured(opponent_model, removed_chain_keys);
        return true;
    } else {
        return false;
    }
}

//Find surrounding adjacent neighbors of a color
//Only borrows
pub fn get_adjacent(
    board_state: &mut Vec<Vec<Intersection>>,
    board_size: usize,
    row: usize,
    col: usize,
    desired_color: u8,
) -> Vec<(usize, usize)> {
    //Check left, right, above and below for "friendly" intersections
    let mut friends: Vec<(usize, usize)> = Vec::new();
    let intersection: &Intersection = &board_state[row][col];
    //Check left friend
    if intersection.col != 0 {
        if board_state[intersection.row][intersection.col - 1].player_color
            == desired_color
        {
            friends.push((intersection.row, intersection.col-1));
        }
    }

    //Check right friend
    if intersection.col + 1 < board_size {
        if board_state[intersection.row][intersection.col + 1].player_color
            == desired_color
        {
            friends.push((intersection.row, intersection.col+1));
        }
    }

    //Check friend above
    if intersection.row != 0 {
        if board_state[intersection.row - 1][intersection.col].player_color
            == desired_color
        {
            friends.push((intersection.row - 1, intersection.col));
        }
    }

    //Check friend below
    if intersection.row + 1 < board_size {
        if board_state[intersection.row + 1][intersection.col].player_color
            == desired_color
        {
            friends.push((intersection.row + 1, intersection.col));
        }
    }
    return friends;
}


    /**
     * Call after placing a stone
     * Create a new chain based on the friends that were found around the newly placed stone
     * row and col represent the location of the newly placed stone
    **/
    pub fn update_chain(
        board_state: &mut Vec<Vec<Intersection>>,
        player_model: &mut PlayerModel,
        friends: Vec<(usize, usize)>,
        row: usize,
        col: usize,
    ) -> Vec<(usize, usize)> {
        //All of these friends needs to be made into a single chain
        //They will have the id of the newly placed stone
        let mut new_friend_chain: Vec<(usize, usize)> = vec![(row, col)];
        for friend in friends {
            let friend_chain_id: String = board_state[friend.0][friend.1].chain_id.clone();
            let friend_chain: Option<(String, Vec<(usize, usize)>)> = player_model.remove_player_chain(&friend_chain_id);
            player_model.remove_player_liberties(&friend_chain_id);
            if let Some((old_chain, vec)) = friend_chain {
                println!("Merging old chain {:?} with new chain id{:?}", old_chain, &friend_chain_id);
                for (friend_row, friend_col) in vec {
                    //All of the friends of this chain will now belong to the same chain with the id of the new stone
                    board_state[friend_row][friend_col].chain_id = board_state[row][col].chain_id.clone();
                    new_friend_chain.push((friend_row, friend_col));
                }
            }
        }
        return new_friend_chain;
    }

    /**
     * Check for liberties for all pieces potentially impacted by a stone placement
     * For each chain, get all associated liberties. If a chain doesn't have any liberties, it has been conquered.
     * This is done on a specific player's chains
     */
    pub fn update_player_liberties(
        board_state: &mut Vec<Vec<Intersection>>,
        board_size: usize,
        player_model: &mut PlayerModel) -> HashMap<String, Vec<(usize, usize)>> {
        let mut player_liberties: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
        for chain_key in player_model.player_chains.keys() {
            let mut liberties_for_chain: Vec<(usize, usize)> = Vec::new();
            println!("Checking chain {chain_key}");
            let chain = match player_model.player_chains.get(chain_key) {
                Some(chain_val) => chain_val,
                None => continue,
            };
            for chain_friend in chain {
                println!("Checking space {:?} for liberties", chain_friend);
                let row = chain_friend.0;
                let col = chain_friend.1;
                let adjacent_empty: Vec<(usize, usize)> = get_adjacent(board_state, board_size, row, col, EMPTY);
                for empty_space in adjacent_empty {
                    liberties_for_chain.push(empty_space);
                }
            }
            player_liberties.insert(chain_key.to_string(), liberties_for_chain);
        }            
        println!("Liberties {:?}", player_model.player_liberties);
        return player_liberties;
    }

    /**
     * Given a placement at position (row,col), check that the 
     * placement of the stone is not going to cause a self capture
     * return true if the placement would cause self capture
     * return false if the placement is valid
     */
    pub fn check_for_self_capture(board: &mut Board, row: usize, col: usize) -> bool {
        return false; //TODO
    }

    pub fn check_for_conquered(
        player_model: &mut PlayerModel, 
        board_state: &mut Vec<Vec<Intersection>>) 
    -> Vec<String> {
        let mut removed_chain_keys: Vec<String> = Vec::<String>::new();
        for (chain_key, liberties) in player_model.player_liberties.iter() {
            println!("Checking to see if chain id {chain_key} has any liberties...");
            if liberties.len() == 0 {
                println!("Chain id {chain_key} has no liberties. It has been eliminated.");
                let captured_chain = match player_model.player_chains.get(chain_key) {
                    Some(captured) => captured,
                    None => continue,
                };
                for captured_location in captured_chain {
                    if board_state[captured_location.0][captured_location.1].player_color == WHITE {
                        board_state[captured_location.0][captured_location.1].player_color = BLACK_TERR;
                    } else if board_state[captured_location.0][captured_location.1].player_color == BLACK {
                        board_state[captured_location.0][captured_location.1].player_color = WHITE_TERR;
                    }
                }
                removed_chain_keys.push(chain_key.to_string());
            }
        }
        
        println!("The following chains were removed {:?}", removed_chain_keys);
        //TODO: call update_prisoners as needed
        return removed_chain_keys;
    }
    
    /**
     * Cleanup the player models that have lose chains
     */
    pub fn cleanup_captured(player_model: &mut PlayerModel, removed_chain_keys: Vec<String>) {
        for chain_key in removed_chain_keys {
            player_model.remove_player_chain(&chain_key);
            player_model.remove_player_liberties(&chain_key);
        }
    }

#[derive(Debug)]
#[derive(Component)]
pub(crate) struct PlayerModel {
    player_chains: HashMap<String, Vec<(usize, usize)>>,
    player_liberties: HashMap<String, Vec<(usize, usize)>>,
    player: Player,
}

impl PlayerModel {
    pub fn new(player_color: u8) -> Self {
        let mut new_player_color=0;
        let mut opponent_color=0;
        if player_color == 1 {
            new_player_color = 1;
            opponent_color = 2;
        } else {
            new_player_color = 2;
            opponent_color = 1;
        }
        PlayerModel {
            player_chains: HashMap::new(),
            player_liberties: HashMap::new(),
            player: Player::new(new_player_color, opponent_color),
        }
    }

    pub fn get_player_color(&self) -> u8 {
        self.player.player_color
    }

    fn update_player_chains(&mut self, chain_key: &str, index: usize, new_value: (usize, usize)) {
        if let Some(vec) = self.player_chains.get_mut(chain_key) {
            if let Some(pair) = vec.get_mut(index) {
                *pair = new_value;
            }
        }
    }

    fn remove_player_chain(&mut self, chain_key: &str) -> Option<(String, Vec<(usize, usize)>)> {
        if let Some(vec) = self.player_chains.remove_entry(chain_key) {
            return Some(vec);
        }
        None
    }

    fn remove_player_chain_item(&mut self, chain_key: &str, index: usize) -> Option<(usize, usize)> {
        if let Some(vec) = self.player_chains.get_mut(chain_key) {
            if index < vec.len() {
                return Some(vec.remove(index));
            }
        }
        None
    }

    fn add_player_chain(&mut self, chain_key: &str, new_chain: Vec<(usize, usize)>) {
            self.player_chains.entry(chain_key.to_string()).or_insert_with(Vec::new)
            .extend(new_chain);
    }

    fn add_player_chain_item(&mut self, chain_key: &str, item: (usize, usize)) {
        self.player_chains.entry(chain_key.to_string()).or_insert_with(Vec::new)
        .push(item);
    }

    fn update_player_liberties(&mut self, chain_key: &str, index: usize, new_value: (usize, usize)) {
        if let Some(vec) = self.player_chains.get_mut(chain_key) {
            if let Some(pair) = vec.get_mut(index) {
                *pair = new_value;
            }
        }
    }


    fn remove_player_liberties(&mut self, chain_key: &str) -> Option<(String, Vec<(usize, usize)>)> {
        if let Some(vec) = self.player_liberties.remove_entry(chain_key) {
            return Some(vec);
        }
        None
    }

    fn remove_player_liberties_item(&mut self, chain_key: &str, index: usize) -> Option<(usize, usize)> {
            if let Some(vec) = self.player_liberties.get_mut(chain_key) {
                if index < vec.len() {
                    return Some(vec.remove(index));
                }
            }
        None
    }

    fn add_player_liberties(&mut self, chain_key: &str, new_liberties: Vec<(usize, usize)>) {
        self.player_liberties.entry(chain_key.to_string()).or_insert_with(Vec::new)
            .extend(new_liberties);
    }

    fn add_player_liberties_item(&mut self, chain_key: &str, item: (usize, usize)) {
        self.player_liberties.entry(chain_key.to_string()).or_insert_with(Vec::new)
            .push(item);
    }

    fn set_player_liberties(&mut self, updated_liberties: HashMap<String, Vec<(usize, usize)>>) {
        self.player_liberties = updated_liberties;
    }

}
/**
 * board_state represents the board using 2D vectors
 * 0 means onocuppied
 * 1 means white stone
 * 2 means black stone
 */
#[derive(Component)]
 pub(crate) struct Board {
    pub board_size: usize,
    pub board_state: Vec<Vec<Intersection>>,
    pub white_captured: u8,
    pub black_captured: u8,
}

impl Board {
    pub fn generate_id(row: usize, col: usize) -> String {
        return row.to_string() + "_" + &col.to_string();
    }

    /**board_connections
     * Build a board filled with empty intersections
     * All Intersections start with their chain being just themselves.
     */
    pub fn build_board_start(board_size: usize) -> Vec<Vec<Intersection>> {
        let mut b_rows = Vec::new();
        for row in 0..=board_size-1 {
            let mut b_columns = Vec::new();
            for column in 0..=board_size-1 {
                let next_intersection: Intersection = Intersection::new(row, column);
                b_columns.push(next_intersection);
            }
            b_rows.push(b_columns);
        }
        return b_rows;
    }

    pub fn new(board_size: usize) -> Self {
        Board {
            board_size: board_size,
            board_state: Self::build_board_start(board_size),
            white_captured: 0,
            black_captured: 0,
        }
    }

    pub fn update_board_color(&mut self, row: usize, col: usize, color: u8) {
        self.board_state[row][col].player_color = color;
    }

}

#[derive(Debug)]
#[derive(Component)]
pub(crate) struct Intersection {
    player_color: u8,
    chain_id: String,
    row: usize,
    col: usize,
}

impl Intersection {
    pub fn new(row: usize, col: usize) -> Self {
        Intersection {
            player_color: EMPTY, //a new intersection is always an empty intersection
            chain_id: Board::generate_id(row, col),
            row: row,
            col: col,
        }
    }
}

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Intersection {{ player_color: {}, chain_id: {}, row: {}, col: {} }}", self.player_color, self.chain_id, self.row, self.col)
    }
}

#[derive(Debug)]
#[derive(Component)]
pub(crate) struct Player {
    pub(crate) player_color: u8,
    pub(crate) opponent_color: u8,
}

impl Player {
    pub fn new(new_player_color: u8, opponent_color: u8) -> Self {
        Player {
            player_color: new_player_color,
            opponent_color: opponent_color,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::game;
    use crate::game::Board;
    use crate::game::PlayerModel;
    use crate::game::BLACK_TERR;
    use crate::game::WHITE_TERR;

    #[test]
    fn test_place_stone() {
        let mut test_board: Board = Board::new(9);

        let mut black_player_model = PlayerModel::new(crate::game::BLACK);
        let mut white_player_model = PlayerModel::new(crate::game::WHITE);

        let result1 = game::place_stone(&mut test_board, &mut black_player_model, &mut white_player_model, 2, 2);
        assert_eq!(
            test_board.board_state[2][2].player_color,
            crate::game::BLACK
        );
        let result2 = game::place_stone(&mut test_board, &mut black_player_model, &mut white_player_model, 8, 8);
        let result3 = game::place_stone(&mut test_board, &mut black_player_model, &mut white_player_model, 8, 8);
        assert_eq!(
            test_board.board_state[8][8].player_color,
            crate::game::BLACK
        );
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, false);
    }

    #[test]
    fn test_check_for_chain() {
        let mut test_board: Board = Board::new(3);

        let mut black_player_model = PlayerModel::new(crate::game::BLACK);
        let mut white_player_model = PlayerModel::new(crate::game::WHITE);

        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 0, 0);
        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 0, 1);
        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 0, 2);
        assert_eq!(test_board.board_state[0][0].chain_id, test_board.board_state[0][1].chain_id);
        assert_eq!(test_board.board_state[0][1].chain_id, test_board.board_state[0][2].chain_id);
        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 1, 2);
        assert_eq!(test_board.board_state[1][2].chain_id, test_board.board_state[0][2].chain_id);
    }

    #[test]
    fn test_check_for_capture() {
        let mut test_board: Board = Board::new(3);

        let mut black_player_model = PlayerModel::new(crate::game::BLACK);
        let mut white_player_model = PlayerModel::new(crate::game::WHITE);

        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 0, 0);
        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 0, 1);
        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 0, 2);
        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 1, 0);
        game::place_stone(&mut test_board, &mut black_player_model, &mut white_player_model, 1, 1);
        game::place_stone(&mut test_board, &mut black_player_model, &mut white_player_model, 1, 2);
        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 2, 1);
        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 2, 2);
        assert_eq!(test_board.board_state[1][1].player_color, WHITE_TERR);
        assert_eq!(test_board.board_state[1][2].player_color, WHITE_TERR);
    }

    #[test]
    fn test_check_for_capture_corner() {
        let mut test_board: Board = Board::new(2);

        let mut black_player_model = PlayerModel::new(crate::game::BLACK);
        let mut white_player_model = PlayerModel::new(crate::game::WHITE);

        game::place_stone(&mut test_board, &mut white_player_model, &mut black_player_model, 0, 0);
        game::place_stone(&mut test_board, &mut black_player_model, &mut white_player_model, 1, 1);
        game::place_stone(&mut test_board, &mut black_player_model, &mut white_player_model, 0, 1);
        game::place_stone(&mut test_board, &mut black_player_model, &mut white_player_model, 1, 0);
        assert_eq!(test_board.board_state[0][0].player_color, BLACK_TERR);
    }

}
