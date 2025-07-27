use std::fmt::Debug;
use std::hash::Hash;

pub trait Mark: Eq + Ord + Copy + Hash + Debug {
    fn value(&self) -> u8;
}
