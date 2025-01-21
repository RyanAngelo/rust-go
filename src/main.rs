mod game;
mod grid;

use bevy::prelude::*;
use grid::GridPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GridPlugin)
        .run();
}