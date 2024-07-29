use std::{
    collections::{HashMap, HashSet}, fmt,
};

pub const EMPTY: u8 = 0;
pub const WHITE: u8 = 1;
pub const BLACK: u8 = 2;
pub const WHITE_TERR: u8 = 3;
pub const BLACK_TERR: u8 = 4;

/**
 *
 * There are traditionaly 181 stones in Go.
 * Traditional Board size is 19x19
 * Also popular are 13x13 and 9x9
 *
 * End game scoring: A player's score is the number
 * of stones that the player has on the board, plus
 * the number of empty intersections surrounded by that player's stones.
 */

pub fn place_stone(
    board: &mut Board,
    player: &Player,
    row: usize,
    col: usize,
) -> bool {
    if board.board_state[row][col].player_color == 0 {
        println!("Placing {:?} stone at location {:?}", player.player_color, stringify!(row, col));
        board.board_state[row][col].player_color = player.player_color;
        let friends:Vec<(usize, usize)> = get_adjacent(board, row, col, player.player_color);
        update_chain(board, friends, row, col);
        check_for_liberties(board);
        check_for_conquered(board);
        return true;
    } else {
        return false;
    }
}

//Find surrounding adjacent neighbors of a color
//Only borrows
pub fn get_adjacent(
    board: &Board,
    row: usize,
    col: usize,
    desired_color: u8,
) -> Vec<(usize, usize)> {
    //Check left, right, above and below for "friendly" intersections
    let mut friends: Vec<(usize, usize)> = Vec::new();
    let intersection: &Intersection = &board.board_state[row][col];
    //Check left friend
    if intersection.col != 0 {
        if board.board_state[intersection.row][intersection.col - 1].player_color
            == desired_color
        {
            friends.push((intersection.row, intersection.col-1));
        }
    }

    //Check right friend
    if intersection.col + 1 < board.board_size {
        if board.board_state[intersection.row][intersection.col + 1].player_color
            == desired_color
        {
            friends.push((intersection.row, intersection.col+1));
        }
    }

    //Check friend above
    if intersection.row != 0 {
        if board.board_state[intersection.row - 1][intersection.col].player_color
            == desired_color
        {
            friends.push((intersection.row - 1, intersection.col));
        }
    }

    //Check friend below
    if intersection.row + 1 < board.board_size {
        if board.board_state[intersection.row + 1][intersection.col].player_color
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
        board: &mut Board,
        friends: Vec<(usize, usize)>,
        row: usize,
        col: usize,
    ) {
        //All of these friends needs to be made into a single chain
        //They will have the id of the newly placed stone
        let mut new_friend_chain: Vec<(usize, usize)> = vec![(row, col)];
        for friend in friends {
            let friend_intersection: &Intersection = &board.board_state[friend.0][friend.1];
            let friend_chain: Option<(String, Vec<(usize, usize)>)> = board.board_chains.remove_entry(&friend_intersection.chain_id);
            board.board_liberties.remove(&friend_intersection.chain_id);
            if let Some((old_chain, vec)) = friend_chain {
                println!("Merging old chain {:?}", old_chain);
                for (friend_row, friend_col) in vec {
                    //All of the friends of this chain will now belong to the same chain with the id of the new stone
                    board.board_state[friend_row][friend_col].chain_id = board.board_state[row][col].chain_id.clone();
                    new_friend_chain.push((friend_row, friend_col));
                }
            }
        }
        //Add to the board chains our new combined chain
        board.board_chains.insert(Board::generate_id(row, col), new_friend_chain);
        println!("Board Chains {:?}", board.board_chains);

    }

    /**
     * Check for liberties for all pieces potentially impacted by a stone placement
     * For each chain, get all associated liberties. If a chain doesn't have any liberties, it has been conquered.
     */
    pub fn check_for_liberties(board: &mut Board) { 
        for chain_key in board.board_chains.keys() {
            let mut liberties_for_chain: HashSet<(usize, usize)> = HashSet::new();
            board.board_liberties.insert(chain_key.clone(), Vec::<(usize, usize)>::new());
            println!("Checking chain {chain_key}");
            let chain = match board.board_chains.get(chain_key) {
                Some(chain_val) => chain_val,
                None => continue,
              };
            for chain_friend in chain {
                println!("Checking space {:?} for liberties", chain_friend);
                let row = chain_friend.0;
                let col = chain_friend.1;
                let adjacent_empty: Vec<(usize, usize)> = get_adjacent(board, row, col, EMPTY);
                for empty_space in adjacent_empty {
                    liberties_for_chain.insert(empty_space);
                }
            }
            board.board_liberties.insert(chain_key.clone(), liberties_for_chain.into_iter().collect());
        }
        println!("Board Liberties {:?}", board.board_liberties);
    }

    /**
     * Given a placement at position (row,col), check that the 
     * placement of the stone is not going to cause a self capture
     * return true if the placement would cause self capture
     * return false if the placement is valid
     */
    pub fn check_for_self_capture(board: &mut Board, row: usize, col: usize) -> bool {
        return false;
    }

    pub fn check_for_conquered(board: &mut Board) -> Vec<(usize, usize)> {
        let mut removed_stones: Vec<(usize, usize)> = Vec::<(usize, usize)>::new();
        for chain_key in board.board_liberties.keys() {
            println!("Checking to see if chain id {chain_key} has any liberties...");
            let chain = match board.board_liberties.get(chain_key) {
                Some(chain_val) => chain_val,
                None => continue,
            };
            if (chain.len() == 0) {
                println!("Chain id {chain_key} has no liberties. It has been eliminated.");
                let captured_chain = match board.board_chains.get(chain_key) {
                    Some(captured) => captured,
                    None => continue,
                };
                for captured_location in captured_chain {
                    let captured_intersection: &Intersection = &board.board_state[captured_location.0][captured_location.1];
                    if captured_intersection.player_color == WHITE {
                        board.white_captured = board.white_captured + 1;
                        board.board_state[captured_location.0][captured_location.1].player_color = BLACK_TERR;
                    } else if captured_intersection.player_color == BLACK {
                        board.black_captured = board.black_captured + 1;
                        board.board_state[captured_location.0][captured_location.1].player_color = WHITE_TERR;
                    }
                    removed_stones.push((board.board_state[captured_location.0][captured_location.1].row, board.board_state[captured_location.0][captured_location.1].col));
                }
            }
        }
        println!("The following stones were removed {:?}", removed_stones);
        //TODO: call update_prisoners as needed
        return removed_stones;
    }
 
/**
 * board_state represents the board using 2D vectors
 * 0 means onocuppied
 * 1 means white stone
 * 2 means black stone
 */
pub(crate) struct Board {
    pub board_size: usize,
    pub board_state: Vec<Vec<Intersection>>,
    pub board_chains: HashMap<String, Vec<(usize, usize)>>,
    pub board_liberties: HashMap<String, Vec<(usize, usize)>>,
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
            board_chains: HashMap::new(),
            board_liberties: HashMap::new(),
            white_captured: 0,
            black_captured: 0,
        }
    }
}

#[derive(Debug)]
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

pub(crate) struct Player {
    player_color: u8,
}

impl Player {
    pub fn new(new_player_color: u8) -> Self {
        Player {
            player_color: new_player_color,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::game;
    use crate::game::check_for_conquered;
    use crate::game::Board;
    use crate::game::Player;
    use crate::game::WHITE_TERR;

    #[test]
    fn test_place_stone() {
        let test_player: Player = Player::new(crate::game::BLACK);
        let mut test_board: Board = Board::new(9);
        let result1 = game::place_stone(&mut test_board, &test_player, 2, 2);
        assert_eq!(
            test_board.board_state[2][2].player_color,
            crate::game::BLACK
        );
        let result2 = game::place_stone(&mut test_board, &test_player, 8, 8);
        let result3 = game::place_stone(&mut test_board, &test_player, 8, 8);
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
        let test_white: Player = Player::new(crate::game::WHITE);
        let mut test_board: Board = Board::new(3);
        game::place_stone(&mut test_board, &test_white, 0, 0);
        game::place_stone(&mut test_board, &test_white, 0, 1);
        game::place_stone(&mut test_board, &test_white, 0, 2);
        assert_eq!(test_board.board_state[0][0].chain_id, test_board.board_state[0][1].chain_id);
        assert_eq!(test_board.board_state[0][1].chain_id, test_board.board_state[0][2].chain_id);
        game::place_stone(&mut test_board, &test_white, 1, 2);
        assert_eq!(test_board.board_state[1][2].chain_id, test_board.board_state[0][2].chain_id);
    }

    #[test]
    fn test_check_for_capture() {
        let mut test_white: Player = Player::new(crate::game::WHITE);
        let test_black: Player = Player::new(crate::game::BLACK);
        let mut test_board: Board = Board::new(3);
        game::place_stone(&mut test_board, &test_white, 0, 0);
        game::place_stone(&mut test_board, &test_white, 0, 1);
        game::place_stone(&mut test_board, &test_white, 0, 2);
        game::place_stone(&mut test_board, &test_white, 1, 0);
        game::place_stone(&mut test_board, &test_black, 1, 1);
        game::place_stone(&mut test_board, &test_black, 1, 2);
        let captured1: Vec<(usize, usize)> = check_for_conquered(&mut test_board); //Should return 0.
        assert_eq!(captured1.len(), 0);
        game::place_stone(&mut test_board, &test_white, 2, 1);
        game::place_stone(&mut test_board, &test_white, 2, 2);
        let captured2: Vec<(usize, usize)> = check_for_conquered(&mut test_board); //Should return 2
        assert_eq!(captured2.len(), 2);
        assert_eq!(test_board.board_state[captured2.first().unwrap().0][captured2.first().unwrap().1].player_color, WHITE_TERR);
    }
}
