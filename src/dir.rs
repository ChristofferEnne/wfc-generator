use std::{fmt, slice::Iter};
pub enum Direction {
  West,
  North,
  East,
  South,
  Up,
  Down
}

impl Direction {
  pub fn iterator() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 6] = [
      Direction::West,
      Direction::North,
      Direction::East,
      Direction::South,
      Direction::Up,
      Direction::Down
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
