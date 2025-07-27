use std::fmt::Debug;
use std::hash::Hash;

pub trait Position: Eq + Ord + Copy + Hash + Debug {
    fn x(&self) -> u8;
    fn y(&self) -> u8;
}
