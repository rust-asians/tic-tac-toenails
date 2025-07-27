use bevy::prelude::*;
use domain::mark::Mark as DomainMark;

#[derive(Component, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct Mark {
    pub id: u8,
}

impl DomainMark for Mark {
    fn value(&self) -> u8 {
        self.id
    }
}
