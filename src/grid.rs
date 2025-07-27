use crate::mark::Mark;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    pub width: i8,
    pub height: i8,
}

#[derive(Component)]
pub enum GridCell {
    Marked(Mark),
    Empty,
}

impl GridCell {
    pub fn mark(&self) -> Option<&Mark> {
        match self {
            GridCell::Marked(mark) => Some(mark),
            GridCell::Empty => None,
        }
    }
}
