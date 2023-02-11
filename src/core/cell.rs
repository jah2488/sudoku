use std::{
    collections::HashSet,
    fmt::{Debug, Formatter},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub value: u8,
    pub mutable: bool,
    pub neighbors: HashSet<Point>,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Cell {{ x: {}, y: {}, value: {}, neighbors: {:?} }}",
            self.x, self.y, self.value, self.neighbors
        )
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        return Point { x, y };
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}
