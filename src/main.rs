mod game;

use game::Player;
use game::Board;

fn main() {

    let mut player1: Player = Player::new(game::BLACK);
    let mut player2: Player = Player::new(game::WHITE);

    let mut game_board: Board = Board::new(9);

    println!("{:?}", game_board.board_state);

}
