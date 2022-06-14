use std::fmt;

use crate::Shape;

#[derive(Eq, PartialEq, Debug)]
pub struct Player {
    pub name: String,
    pub shape: Shape,
    pub score: u64,
}

impl Player {
    pub fn new(name: &str, shape: Shape) -> Self {
        Self {
            name: name.to_owned(),
            shape,
            score: 0,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Shape: {:?}, Score: {}", self.name, self.shape, self.score)
    }
}
