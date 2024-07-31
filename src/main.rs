mod game;

use game::Player;
use game::Board;
use bevy::prelude::*;

pub struct PlayerActionPlugin;
impl Plugin for PlayerActionPlugin {
    fn build(&self, app: &mut App) {
        //Add systems here
    }
}

fn main() {

    let mut player1: Player = Player::new(game::BLACK);
    let mut player2: Player = Player::new(game::WHITE);

    let mut game_board: Board = Board::new(9);

    println!("{:?}", game_board.board_state);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerActionPlugin)
        .run();

}
