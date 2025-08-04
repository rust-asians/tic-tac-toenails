mod dimension;
mod grid;
mod mark;
mod needs_redraw;
mod position;

use crate::grid::{GridSize, draw_grid};
use crate::mark::Mark;
use crate::position::Position;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_systems(Startup, (create_dummy_grid, setup))
        .insert_resource(GridSize {
            width: 3,
            height: 3,
        })
        .add_systems(Startup, draw_grid)
        .run();
}
