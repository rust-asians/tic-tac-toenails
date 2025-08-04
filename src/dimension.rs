#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn all() -> [Axis; 2] {
        [Axis::X, Axis::Y]
    }

    pub fn reversed(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    pub fn get(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.width,
            Axis::Y => self.height,
        }
    }
}
