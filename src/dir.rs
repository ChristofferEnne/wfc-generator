use std::{fmt, slice::Iter};
pub enum Direction {
  West,
  North,
  East,
  South
}

impl Direction {
  pub fn iterator() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 4] = [
      Direction::West,
      Direction::North,
      Direction::East,
      Direction::South
    ];
    DIRECTIONS.iter()
  }
}

impl fmt::Debug for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
