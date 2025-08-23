use crate::game::{self, Board, Player, PlayerModel};
use bevy::color::palettes::css::*;
use bevy::prelude::*;

pub const BOARD_SIZE: usize = 9;



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
    let game_board: Board = Board::new(BOARD_SIZE);
    commands.spawn((player_white_model, Player { player_color: game::WHITE, opponent_color: game::BLACK }));
    commands.spawn((player_black_model, Player { player_color: game::BLACK, opponent_color: game::WHITE }));
    commands.spawn(game_board);
}

/**
 * Creates the visual layout of the game board.
 * Spawns a camera and creates a grid of interactive squares representing the Go board.
 * The board is centered on screen with a dark background.
 */
fn spawn_layout(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let rows = BOARD_SIZE;
    let cols = BOARD_SIZE;

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: DARK_GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            // Header text
            parent.spawn(TextBundle::from_section(
                "GO",
                TextStyle {
                    font_size: 60.0,
                    color: WHITE.into(),
                    ..default()
                },
            ));

            // Turn indicator text
            parent.spawn((
                TextBundle::from_section(
                    "White's Turn",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TurnText,
            ));

            // Game board container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(600.0),
                        position_type: PositionType::Relative,
                        padding: UiRect::all(Val::Px(0.0)),
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.87, 0.68, 0.34).into(), // Wooden board color
                    ..default()
                })
                .with_children(|parent| {
                    // Grid lines
                    for i in 0..rows {
                        // Horizontal lines
                        parent.spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                right: Val::Px(0.0),
                                height: Val::Px(2.0),
                                top: Val::Percent(i as f32 * (100.0 / (rows - 1) as f32)),
                                ..default()
                            },
                            background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                            ..default()
                        });
                        
                        // Vertical lines
                        parent.spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                                width: Val::Px(2.0),
                                left: Val::Percent(i as f32 * (100.0 / (cols - 1) as f32)),
                                ..default()
                            },
                            background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                            ..default()
                        });
                    }

                    // Spawn intersection points
                    for row in 0..rows {
                        for col in 0..cols {
                            spawn_intersection(parent, row, col, rows, cols);
                        }
                    }
                });
        });
}

fn spawn_intersection(parent: &mut ChildBuilder, row: usize, col: usize, rows: usize, cols: usize) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(col as f32 * (100.0 / (cols - 1) as f32) - 2.0),
                top: Val::Percent(row as f32 * (100.0 / (rows - 1) as f32) - 2.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        GridSquare { row, col },
    ))
    .with_children(|parent| {
        // Stone visual (initially invisible)
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(28.0),
                    height: Val::Px(28.0),
                    border: UiRect::all(Val::Px(0.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                ..default()
            },
            StoneBackground,
        ));
    });
}

// Add this marker component at the top with other components
#[derive(Component)]
struct StoneBackground;

// Add this near other components
#[derive(Component)]
struct TurnText;

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
    interaction_query: Query<
        (&Interaction, &GridSquare, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    grid_squares: Query<(&GridSquare, Entity), With<Button>>,
    mut stone_query: Query<(&mut BackgroundColor, &Parent), With<StoneBackground>>,
    mut board: Query<&mut Board>,
    mut player_query: Query<&mut PlayerModel, With<Player>>,
    mut turn_text: Query<&mut Text, With<TurnText>>,
) {
    for (interaction, grid_square, button_entity) in interaction_query.iter() {
        if let Some(_stone_color) = stone_query.iter_mut()
            .find(|(_, parent)| parent.get() == button_entity) {
            if let Interaction::Pressed = *interaction {
                if let Ok(mut board) = board.get_single_mut() {
                    let mut current_player = None;
                    let mut opponent_player = None;

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
                            // Update all stones
                            for row in 0..board.board_size {
                                for col in 0..board.board_size {
                                    let color = board.board_state[row][col].get_player_color();
                                    if let Some(mut square_color) = stone_query.iter_mut()
                                        .find(|(_, parent)| {
                                            if let Ok((grid_square, _)) = grid_squares.get(parent.get()) {
                                                grid_square.row == row && grid_square.col == col
                                            } else {
                                                false
                                            }
                                        }) {
                                        match color {
                                            game::WHITE => *square_color.0 = Color::srgba(1.0, 1.0, 1.0, 1.0).into(),
                                            game::BLACK => *square_color.0 = Color::srgba(0.0, 0.0, 0.0, 1.0).into(),
                                            game::WHITE_TERR => *square_color.0 = Color::srgba(0.9, 0.9, 0.9, 0.8).into(),
                                            game::BLACK_TERR => *square_color.0 = Color::srgba(0.2, 0.2, 0.2, 0.8).into(),
                                            game::EMPTY => *square_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                                            _ => {}
                                        }
                                    }
                                }
                            }

                            // Update turn text
                            if let Ok(mut text) = turn_text.get_single_mut() {
                                if board.is_white_turn {
                                    text.sections[0].value = "White's Turn".to_string();
                                    text.sections[0].style.color = Color::WHITE;
                                } else {
                                    text.sections[0].value = "Black's Turn".to_string();
                                    text.sections[0].style.color = Color::BLACK;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}