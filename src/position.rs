use bevy::prelude::*;
use domain::position::Position as DomainPosition;

#[derive(Component, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl DomainPosition for Position {
    fn x(&self) -> u8 {
        self.x
    }

    fn y(&self) -> u8 {
        self.y
    }
}
