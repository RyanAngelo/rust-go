use crate::game::{self, Board, Player, PlayerModel};
use bevy::color::palettes::css::*;
use bevy::prelude::*;

/**
 * Component to track grid position
 */
#[derive(Component)]
pub struct GridSquare {
    row: usize,
    col: usize,
}

/**
 * Plugin to handle game initialization and grid interactions
 */
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_gameboard)
            .add_systems(Startup, spawn_layout)
            .add_systems(Update, grid_button_interaction);
    }
}

/**
 * Creates the initial game board and player models.
 * Spawns the board and both players (black and white) into the game world.
 */
fn create_gameboard(mut commands: Commands) {
    let player_white_model = PlayerModel::new(game::WHITE);
    let player_black_model = PlayerModel::new(game::BLACK);
    let game_board: Board = Board::new(9);
    commands.spawn(player_white_model);
    commands.spawn(player_black_model);
    commands.spawn(game_board);
}

/**
 * Creates the visual layout of the game board.
 * Spawns a camera and creates a grid of interactive squares representing the Go board.
 * The board is centered on screen with a dark background.
 */
fn spawn_layout(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    let rows = 9;
    let cols = 9;

    // Root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            // Game board container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(600.0),
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(cols, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(rows, 1.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: DARK_GREY.into(),
                    ..default()
                })
                .with_children(|builder| {
                    // Create grid squares
                    for row in 0..rows {
                        for col in 0..cols {
                            spawn_grid_square(builder, row.into(), col.into());
                        }
                    }
                });
        });
}

/**
 * Creates an individual square for the game board grid.
 * Each square is a button that can be interacted with to place stones.
 * 
 * @param builder - The ChildBuilder to spawn the square into
 * @param row - The row position of this square on the board
 * @param col - The column position of this square on the board
 */
fn spawn_grid_square(builder: &mut ChildBuilder, row: usize, col: usize) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: WHITE.into(),
                ..default()
            },
            GridSquare { row, col },
        ))
        .with_children(|parent| {
            // Square content
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(90.0),
                    height: Val::Percent(90.0),
                    ..default()
                },
                background_color: GRAY.into(),
                ..default()
            });
        });
}

/**
 * Handles all interaction with the game board squares.
 * This includes:
 * - Processing clicks to place stones
 * - Updating square colors based on the current player's turn
 * - Showing hover effects when moving over squares
 * - Enforcing game rules through the place_stone function
 * 
 * The color of placed stones will be:
 * - White for the white player
 * - Black for the black player
 */
fn grid_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &GridSquare, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut board: Query<&mut Board>,
    mut player_query: Query<&mut PlayerModel, With<Player>>,
) {
    for (interaction, grid_square, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!(
                    "Square clicked: row={}, col={}",
                    grid_square.row, grid_square.col
                );
                if let Ok(mut board) = board.get_single_mut() {
                    let mut current_player = None;
                    let mut opponent_player = None;

                    // Get the correct current player and opponent based on whose turn it is
                    for player_model in player_query.iter_mut() {
                        if (board.is_white_turn && player_model.get_player_color() == game::WHITE) ||
                           (!board.is_white_turn && player_model.get_player_color() == game::BLACK) {
                            current_player = Some(player_model);
                        } else {
                            opponent_player = Some(player_model);
                        }
                    }

                    if let (Some(mut current), Some(mut opponent)) = (current_player, opponent_player) {
                        let placed = game::place_stone(
                            &mut board,
                            &mut current,
                            &mut opponent,
                            grid_square.row,
                            grid_square.col,
                        );

                        if placed {
                            // Set color based on which player placed the stone
                            if current.get_player_color() == game::WHITE {
                                *color = Color::srgb(1.0, 1.0, 1.0).into(); // White stone
                            } else {
                                *color = Color::srgb(0.0, 0.0, 0.0).into(); // Black stone
                            }
                            println!("Stone placed successfully");
                        } else {
                            println!("Failed to place stone");
                        }
                    }
                }
            }
            Interaction::Hovered => {
                if let Ok(board) = board.get_single() {
                    // Show hover color based on current player's turn
                    if board.is_white_turn {
                        *color = Color::srgb(0.9, 0.9, 0.9).into(); // Light grey for white's turn
                    } else {
                        *color = Color::srgb(0.3, 0.3, 0.3).into(); // Dark grey for black's turn
                    }
                }
            }
            Interaction::None => {
                *color = Color::srgb(0.8, 0.8, 0.8).into(); // Default grid color
            }
        }
    }
}
