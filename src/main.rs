mod game;

use game::Board;
use game::PlayerModel;
use bevy::prelude::*;

pub struct PlayerActionPlugin;
impl Plugin for PlayerActionPlugin {
    fn build(&self, app: &mut App) {
        //Add systems here
        app.add_systems(Startup, create_gameboard);
        app.add_systems(Startup, setup_system);

    }
}

fn create_gameboard(mut commands: Commands) {
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
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Create a UI root node with Flex direction and spacing
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Define the grid dimensions
            let rows = 3;
            let cols = 3;

            // Create rows
            for _ in 0..rows {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|row| {
                        // Create buttons for each row
                        for _ in 0..cols {
                            row.spawn(ButtonBundle {
                                style: Style {
                                    left: Val::Percent(10.),
                                    right: Val::Percent(10.),
                                    top: Val::Percent(15.),
                                    bottom: Val::Percent(15.),
                                    margin: UiRect::all(Val::Px(5.0)), // Spacing between buttons
                                    ..Default::default()
                                },
                                background_color: Color::rgb(0.25, 0.25, 0.75).into(), // Button color
                                ..Default::default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    text: Text::from_section(
                                        "Button",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..Default::default()
                                });
                            });
                        }
                    });
            }
        });
}