pub struct Tile {
  pub id: usize,
  pub name: String,
  pub filename: String,
  pub rotation: usize, // nr of extra rotation variants to create
  pub connectors: ((usize, bool), (usize, bool), (usize, bool), (usize, bool)) // socket, flipped
}

impl Tile {
  pub fn new(
    id: usize,
    name: String,
    filename: String,
    rotation: usize,
    connectors: ((usize, bool), (usize, bool), (usize, bool), (usize, bool))
  ) -> Self {
    Self {
      id,
      name,
      filename,
      rotation,
      connectors
    }
  }
}
