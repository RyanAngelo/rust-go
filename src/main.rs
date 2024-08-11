mod game;

use game::Board;
use game::PlayerModel;
use bevy::prelude::*;

pub struct PlayerActionPlugin;
impl Plugin for PlayerActionPlugin {
    fn build(&self, app: &mut App) {
        //Add systems here
    }
}

fn create_commands(mut commands: Commands) {
    let player_white_model = PlayerModel::new(crate::game::WHITE);
    let player_black_model = PlayerModel::new(crate::game::BLACK);
    let game_board: Board = Board::new(9);
    commands.spawn(player_white_model);
    commands.spawn(player_black_model);
    commands.spawn(game_board);
}

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
fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerActionPlugin)
        .run();

}
