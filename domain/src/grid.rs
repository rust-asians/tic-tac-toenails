use std::fmt::Debug;
use std::hash::Hash;
use crate::mark::Mark;
use crate::position::Position;

pub trait GridSize: Eq + Ord + Copy + Hash + Debug {
    fn width(&self) -> u8;
    fn height(&self) -> u8;

    fn area(&self) -> u16 {
        (self.width() * self.height()) as u16
    }
}

pub trait Grid: Eq + Clone + Hash + Debug {
    fn get(position: &impl Position) -> Option<&impl Mark>;
}
