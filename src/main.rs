mod game;
mod grid;

use bevy::prelude::*;
use grid::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}