use bevy::prelude::*;
use crate::game::{Board, PlayerModel, Player};
use bevy::color::palettes::css::*;

// Component to track grid position
#[derive(Component)]
struct GridSquare {
    row: usize,
    col: usize,
}

// Plugin to handle game initialization and grid interactions
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_gameboard)
            .add_systems(Startup, spawn_layout)
            .add_systems(Update, grid_button_interaction);
    }
}

// Initialize the game board and players
fn create_gameboard(mut commands: Commands) {
    let player_white_model = PlayerModel::new(crate::game::WHITE);
    let player_black_model = PlayerModel::new(crate::game::BLACK);
    let game_board: Board = Board::new(9);
    commands.spawn(player_white_model);
    commands.spawn(player_black_model);
    commands.spawn(game_board);
}

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
            background_color: DARK_SLATE_GRAY.into(), // Debug color
            ..default()
        })
        .with_children(|parent| {
            // Game board container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(600.0),  // Fixed size for testing
                        height: Val::Px(600.0), // Fixed size for testing
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(cols, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(rows, 1.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BLACK.into(),
                    ..default()
                })
                .with_children(|builder| {
                    // Create grid squares
                    for row in 0..rows {
                        for col in 0..cols {
                            spawn_grid_square(builder, row as usize, col as usize);
                        }
                    }
                });
        });
}

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

fn grid_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &GridSquare, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut board: Query<&mut Board>,
    mut white_player: Query<&mut PlayerModel, With<Player>>,
    mut black_player: Query<&mut PlayerModel, Without<Player>>,
) {
    for (interaction, grid_square, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("Square clicked: row={}, col={}", grid_square.row, grid_square.col);
                if let Ok(mut board) = board.get_single_mut() {
                    if let Ok(mut white_player) = white_player.get_single_mut() {
                        if let Ok(mut black_player) = black_player.get_single_mut() {
                            let row = grid_square.row;
                            let col = grid_square.col;
                            let placed = crate::game::place_stone(
                                &mut board,
                                &mut white_player,
                                &mut black_player,
                                row,
                                col,
                            );
                            
                            if placed {
                                *color = Color::rgb(0.35, 0.75, 0.35).into();
                                println!("Stone placed successfully");
                            } else {
                                println!("Failed to place stone");
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.75, 0.75, 0.75).into();
            }
            Interaction::None => {
                *color = Color::rgb(1.0, 1.0, 1.0).into();
            }
        }
    }
}