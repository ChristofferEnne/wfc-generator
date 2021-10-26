pub struct Tile {
  pub id: usize,
  pub name: String,
  pub filename: String,
  pub rotation: usize, // nr of extra rotation variants to create
  pub connectors: (usize, usize, usize, usize, usize, usize),
  pub flipped: (bool, bool, bool, bool)
}

impl Tile {
  pub fn new(
    id: usize,
    name: String,
    filename: String,
    rotation: usize,
    connectors: (usize, usize, usize, usize, usize, usize),
    flipped: (bool, bool, bool, bool)
  ) -> Self {
    Self {
      id,
      name,
      filename,
      rotation,
      connectors,
      flipped
    }
  }
}
