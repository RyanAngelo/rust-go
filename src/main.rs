mod game;

use game::Player;
use game::Board;
use game::PlayerModel;
use bevy::prelude::*;

pub struct PlayerActionPlugin;
impl Plugin for PlayerActionPlugin {
    fn build(&self, app: &mut App) {
        //Add systems here
    }
}

fn main() {

    let black_player: Player = Player::new(crate::game::BLACK, crate::game::WHITE);
    let mut black_player_model = PlayerModel::new(black_player);

    let white_player: Player = Player::new(crate::game::WHITE, crate::game::BLACK);
    let mut white_player_model = PlayerModel::new(white_player);

    let mut game_board: Board = Board::new(9);

    println!("{:?}", game_board.board_state);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerActionPlugin)
        .run();

}
